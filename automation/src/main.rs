mod assembly;
mod contracts;
mod error;
mod fingerprint;
mod install;
mod json;
mod process;

use error::{Error, Result};
use std::{ffi::OsString, path::PathBuf, process::ExitCode};

const HELP: &str = "nds-source [--root DIRECTORY] check [--app-release FILE] [--development-http] [DESCRIPTOR ...]\nnds-source [--root DIRECTORY] component ID [JUST ARGUMENTS ...]\nnds-source [--root DIRECTORY] install-tool\nnds-source source-info";

enum Action {
    Check {
        descriptors: Vec<PathBuf>,
        application: Option<PathBuf>,
        development_http: bool,
    },
    Component {
        id: String,
        arguments: Vec<OsString>,
    },
    Install,
    Info,
    Help,
}

fn parse() -> Result<(PathBuf, Action)> {
    let mut args = std::env::args_os().skip(1).peekable();
    let root = if args.peek().is_some_and(|v| v == "--root") {
        args.next();
        PathBuf::from(args.next().ok_or(Error::Usage)?)
    } else {
        std::env::current_dir()?
    };
    let command = args.next().ok_or(Error::Usage)?;
    let action = match command.to_str() {
        Some("--help" | "-h") => Action::Help,
        Some("source-info") if args.peek().is_none() => Action::Info,
        Some("install-tool") if args.peek().is_none() => Action::Install,
        Some("component") => Action::Component {
            id: args
                .next()
                .and_then(|v| v.into_string().ok())
                .ok_or(Error::Usage)?,
            arguments: args.collect(),
        },
        Some("check") => {
            let mut descriptors = Vec::new();
            let mut application = None;
            let mut development_http = false;
            let mut positional = false;
            while let Some(arg) = args.next() {
                if positional {
                    descriptors.push(PathBuf::from(arg));
                    continue;
                }
                match arg.to_str() {
                    Some("--") => positional = true,
                    Some("--app-release") if application.is_none() => {
                        application = Some(PathBuf::from(args.next().ok_or(Error::Usage)?))
                    }
                    Some("--development-http") => development_http = true,
                    Some("--help" | "-h") => return Ok((root, Action::Help)),
                    Some(value) if value.starts_with('-') => return Err(Error::Usage),
                    _ => descriptors.push(PathBuf::from(arg)),
                }
            }
            Action::Check {
                descriptors,
                application,
                development_http,
            }
        }
        _ => return Err(Error::Usage),
    };
    Ok((root, action))
}

async fn run() -> Result<i32> {
    let (root, action) = parse()?;
    match action {
        Action::Help => {
            println!("{HELP}");
            return Ok(0);
        }
        Action::Info => {
            println!(
                "{}",
                serde_json::to_string(&install::info()?).map_err(|_| Error::Json)?
            );
            return Ok(0);
        }
        _ => {}
    }
    let root = std::fs::canonicalize(root)?;
    if fingerprint::calculate(&root)? != env!("NDS_SOURCE_INPUTS") {
        return Err(Error::Stale);
    }
    if matches!(action, Action::Install) {
        install::install(&root)?;
        return Ok(0);
    }
    let contracts = contracts::Contracts::open(&root)?;
    let catalog = contracts.catalog(json::load(&root.join("contracts/module-catalog.json"))?)?;
    let assembly = assembly::Assembly::read(&root, &catalog).await?;
    match action {
        Action::Check {
            descriptors,
            application,
            development_http,
        } => {
            if let Some(path) = application {
                contracts.application(json::load(&path)?, &catalog, development_http)?;
            }
            for path in &descriptors {
                contracts.descriptor(json::load(path)?)?;
            }
            println!(
                "Catalog and schemas valid; {} runtime descriptors checked.",
                descriptors.len()
            );
            Ok(0)
        }
        Action::Component { id, arguments } => {
            let child = assembly.select(&root, &catalog, &id).await?;
            assembly::dispatch(&root, &child, arguments).await
        }
        _ => unreachable!(),
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
    match run().await {
        Ok(status) => ExitCode::from(u8::try_from(status).unwrap_or(1)),
        Err(error) => {
            eprintln!("NDS source: {error}");
            ExitCode::FAILURE
        }
    }
}
