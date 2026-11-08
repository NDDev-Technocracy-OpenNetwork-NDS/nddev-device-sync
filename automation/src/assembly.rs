use crate::{
    contracts::Catalog,
    error::{Error, Result},
    process,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
};

pub(crate) struct Assembly {
    links: BTreeMap<String, String>,
}

impl Assembly {
    pub(crate) async fn read(root: &Path, catalog: &Catalog) -> Result<Self> {
        if !fs::symlink_metadata(root.join(".gitmodules"))?.is_file() {
            return Err(Error::Contract(
                "assembly requires a regular .gitmodules file",
            ));
        }
        let text = process::git(
            root,
            [
                "config",
                "--no-includes",
                "--file",
                ".gitmodules",
                "--null",
                "--list",
            ],
        )
        .await?;
        let mut config = BTreeMap::new();
        for entry in text.split_terminator('\0') {
            let (key, value) = entry.split_once('\n').ok_or(Error::Git)?;
            if config.insert(key.to_owned(), value.to_owned()).is_some() {
                return Err(Error::Contract("duplicate Git submodule field"));
            }
        }
        let mut expected = BTreeMap::new();
        for item in &catalog.components {
            expected.insert(format!("submodule.{}.path", item.path), item.path.clone());
            expected.insert(
                format!("submodule.{}.url", item.path),
                format!(
                    "https://github.com/{}/{}.git",
                    catalog.organization, item.repository
                ),
            );
        }
        if config != expected {
            return Err(Error::Contract(
                "Git submodules differ from the public catalog",
            ));
        }
        let index = process::git(root, ["ls-files", "--stage", "-z"]).await?;
        let mut links = BTreeMap::new();
        for entry in index.split_terminator('\0') {
            let (metadata, location) = entry.split_once('\t').ok_or(Error::Git)?;
            let mut fields = metadata.split_whitespace();
            let (Some(mode), Some(commit), Some(stage), None) =
                (fields.next(), fields.next(), fields.next(), fields.next())
            else {
                return Err(Error::Git);
            };
            if stage != "0" {
                return Err(Error::Contract(
                    "assembly index contains an unresolved merge",
                ));
            }
            if mode == "160000" {
                if commit.len() != 40
                    || !commit
                        .bytes()
                        .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
                    || commit.bytes().all(|b| b == b'0')
                {
                    return Err(Error::Contract("submodule has no exact Git commit"));
                }
                if links
                    .insert(location.to_owned(), commit.to_owned())
                    .is_some()
                {
                    return Err(Error::Contract("duplicate indexed Gitlink"));
                }
            } else if location.starts_with("modules/") || location.starts_with("tools/") {
                return Err(Error::Contract(
                    "component implementation must remain in its own repository",
                ));
            }
        }
        let expected: BTreeSet<_> = catalog.components.iter().map(|c| c.path.as_str()).collect();
        let indexed: BTreeSet<_> = links.keys().map(String::as_str).collect();
        if indexed != expected {
            return Err(Error::Contract(
                "indexed Gitlinks differ from the public catalog",
            ));
        }
        Ok(Self { links })
    }

    pub(crate) async fn select(
        &self,
        root: &Path,
        catalog: &Catalog,
        identity: &str,
    ) -> Result<PathBuf> {
        let item = catalog
            .components
            .iter()
            .find(|v| v.id == identity)
            .ok_or(Error::Contract("select a public catalog component"))?;
        let child = root.join(&item.path);
        for path in [&child, child.parent().ok_or(Error::Git)?] {
            if fs::symlink_metadata(path)?.file_type().is_symlink() {
                return Err(Error::Contract("component path is redirected"));
            }
        }
        if !child.join(".git").exists() {
            return Err(Error::Contract(
                "initialize the selected public submodule with just checkout",
            ));
        }
        let top = process::git(&child, ["rev-parse", "--show-toplevel"]).await?;
        if fs::canonicalize(top.trim())? != fs::canonicalize(&child)? {
            return Err(Error::Contract("component does not own its Git checkout"));
        }
        let remote = process::git(&child, ["remote", "get-url", "origin"]).await?;
        let remote = remote.trim().strip_suffix(".git").unwrap_or(remote.trim());
        let remote = remote
            .strip_prefix("https://")
            .unwrap_or(remote)
            .replace("git@github.com:", "github.com/");
        if !remote.eq_ignore_ascii_case(&format!(
            "github.com/{}/{}",
            catalog.organization, item.repository
        )) {
            return Err(Error::Contract(
                "component provider identity differs from the catalog",
            ));
        }
        if self.links.get(&item.path).map(String::as_str)
            != Some(process::git(&child, ["rev-parse", "HEAD"]).await?.trim())
        {
            return Err(Error::Contract(
                "component HEAD differs from the selected Gitlink",
            ));
        }
        if !child.join("justfile").is_file() {
            return Err(Error::Contract(
                "this source-only component has no just recipes",
            ));
        }
        Ok(child)
    }
}

pub(crate) async fn dispatch(root: &Path, child: &Path, args: Vec<OsString>) -> Result<i32> {
    let status = tokio::process::Command::new("just")
        .arg("--working-directory")
        .arg(child)
        .arg("--justfile")
        .arg(child.join("justfile"))
        .args(args)
        .current_dir(root)
        .kill_on_drop(true)
        .status()
        .await?;
    Ok(status.code().unwrap_or(1))
}
