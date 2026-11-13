use crate::{
    error::{Error, Result},
    fingerprint, json,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File, OpenOptions, TryLockError},
    io::{Read, Write},
    path::Path,
};

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Receipt {
    schema: u8,
    tool: String,
    version: String,
    inputs_sha256: String,
    binary_sha256: String,
    os: String,
    arch: String,
}

fn digest(path: &Path) -> Result<String> {
    if !fs::symlink_metadata(path)?.is_file() {
        return Err(Error::Occupied);
    }
    let mut input = File::open(path)?;
    let mut hash = Sha256::new();
    let mut buffer = [0; 64 * 1024];
    loop {
        let count = input.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hash.update(&buffer[..count]);
    }
    Ok(fingerprint::hex(&hash.finalize()))
}

pub(crate) fn info() -> Result<Receipt> {
    Ok(Receipt {
        schema: 1,
        tool: env!("CARGO_PKG_NAME").into(),
        version: env!("CARGO_PKG_VERSION").into(),
        inputs_sha256: env!("NDS_SOURCE_INPUTS").into(),
        binary_sha256: digest(&std::env::current_exe()?)?,
        os: std::env::consts::OS.into(),
        arch: std::env::consts::ARCH.into(),
    })
}

fn directory(path: &Path) -> Result<()> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.is_dir() => Ok(()),
        Ok(_) => Err(Error::Occupied),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            fs::create_dir(path)?;
            Ok(())
        }
        Err(error) => Err(error.into()),
    }
}

fn atomic(path: &Path, mut input: impl Read, executable: bool) -> Result<()> {
    if fs::symlink_metadata(path).is_ok_and(|m| !m.is_file()) {
        return Err(Error::Occupied);
    }
    let mut staged = tempfile::NamedTempFile::new_in(path.parent().ok_or(Error::Occupied)?)?;
    std::io::copy(&mut input, &mut staged)?;
    staged.as_file().sync_all()?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        staged
            .as_file()
            .set_permissions(fs::Permissions::from_mode(if executable {
                0o755
            } else {
                0o644
            }))?;
    }
    #[cfg(not(unix))]
    let _ = executable;
    staged.persist(path).map_err(|e| Error::from(e.error))?;
    Ok(())
}

pub(crate) fn install(root: &Path) -> Result<()> {
    let tools = root.join(".tools");
    directory(&tools)?;
    let marker = tools.join("nds-source-install.lock");
    const OWNER: &[u8] = b"nds-source installer v1\n";
    // Publish a complete ownership marker. A crash must not leave an empty
    // marker that a later installation would have to overwrite or guess about.
    if !marker.try_exists()? {
        let mut staged = tempfile::NamedTempFile::new_in(&tools)?;
        staged.write_all(OWNER)?;
        staged.as_file().sync_all()?;
        if let Err(error) = staged.persist_noclobber(&marker)
            && error.error.kind() != std::io::ErrorKind::AlreadyExists
        {
            return Err(error.error.into());
        }
    }
    if !fs::symlink_metadata(&marker)?.is_file() {
        return Err(Error::Occupied);
    }
    let mut lock = OpenOptions::new().read(true).write(true).open(&marker)?;
    match lock.try_lock() {
        Ok(()) => {}
        Err(TryLockError::WouldBlock) => {
            return Err(Error::Contract(
                "another source-tool installation is active",
            ));
        }
        Err(TryLockError::Error(error)) => return Err(error.into()),
    }
    let mut owner = Vec::new();
    (&mut lock).take(64).read_to_end(&mut owner)?;
    if owner != OWNER {
        return Err(Error::Occupied);
    }
    let bin = tools.join("bin");
    directory(&bin)?;
    let destination = bin.join(format!("nds-source{}", std::env::consts::EXE_SUFFIX));
    let receipt_path = tools.join("nds-source.json");
    let desired = info()?;
    let observed = match fs::symlink_metadata(&destination) {
        Ok(_) => Some(digest(&destination)?),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
        Err(error) => return Err(error.into()),
    };
    if let Some(observed) = &observed
        && observed != &desired.binary_sha256
    {
        let previous: Receipt =
            serde_json::from_value(json::load(&receipt_path)?).map_err(|_| Error::Occupied)?;
        if previous.schema != 1
            || previous.tool != desired.tool
            || &previous.binary_sha256 != observed
        {
            return Err(Error::Occupied);
        }
    }
    if fs::symlink_metadata(&receipt_path).is_ok() {
        let previous: Receipt =
            serde_json::from_value(json::load(&receipt_path)?).map_err(|_| Error::Occupied)?;
        if previous.schema != 1 || previous.tool != desired.tool {
            return Err(Error::Occupied);
        }
    }
    // A lost reply after binary publication is reconciled by the exact desired
    // binary digest. Never overwrite an unrelated file to repair a receipt.
    if observed.as_ref() != Some(&desired.binary_sha256) {
        atomic(&destination, File::open(std::env::current_exe()?)?, true)?;
        if digest(&destination)? != desired.binary_sha256 {
            return Err(Error::Occupied);
        }
    }
    let receipt = serde_json::to_vec_pretty(&desired).map_err(|_| Error::Json)?;
    atomic(&receipt_path, receipt.as_slice(), false)?;
    println!("Source verifier installed.");
    Ok(())
}
