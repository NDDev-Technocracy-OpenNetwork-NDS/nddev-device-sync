use sha2::{Digest, Sha256};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

// Runtime catalog and Git selections are data, not verifier code. Schemas are
// included so an old binary cannot claim acceptance under changed contracts.
const FILES: &[&str] = &[
    "automation/Cargo.toml",
    "automation/Cargo.lock",
    "automation/build.rs",
    "automation/rust-toolchain.toml",
    "automation/deny.toml",
    "automation/.config/nextest.toml",
    "contracts/module-catalog.schema.json",
    "contracts/module-manifest.json",
    "contracts/app-release.schema.json",
];

pub(crate) fn inputs(root: &Path) -> io::Result<Vec<PathBuf>> {
    let mut paths: Vec<PathBuf> = FILES.iter().map(PathBuf::from).collect();
    collect(root, Path::new("automation/src"), &mut paths)?;
    collect(root, Path::new("automation/tests"), &mut paths)?;
    paths.sort();
    Ok(paths)
}

fn collect(root: &Path, relative: &Path, paths: &mut Vec<PathBuf>) -> io::Result<()> {
    if !fs::symlink_metadata(root.join(relative))?.is_dir() {
        return Err(io::ErrorKind::InvalidData.into());
    }
    for entry in fs::read_dir(root.join(relative))? {
        let entry = entry?;
        let path = relative.join(entry.file_name());
        let kind = entry.file_type()?;
        if kind.is_dir() {
            collect(root, &path, paths)?;
        } else if kind.is_file() && path.extension().is_some_and(|v| v == "rs") {
            paths.push(path);
        } else if !kind.is_file() {
            return Err(io::ErrorKind::InvalidData.into());
        }
    }
    Ok(())
}

pub(crate) fn calculate(root: &Path) -> io::Result<String> {
    let mut hash = Sha256::new();
    for relative in inputs(root)? {
        let path = root.join(&relative);
        if !fs::symlink_metadata(&path)?.is_file() {
            return Err(io::ErrorKind::InvalidData.into());
        }
        let name = relative.to_string_lossy().replace('\\', "/");
        let bytes = fs::read(path)?;
        hash.update((name.len() as u64).to_le_bytes());
        hash.update(name.as_bytes());
        hash.update((bytes.len() as u64).to_le_bytes());
        hash.update(bytes);
    }
    Ok(hex(&hash.finalize()))
}

pub(crate) fn hex(bytes: &[u8]) -> String {
    use std::fmt::Write;
    let mut result = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        write!(&mut result, "{byte:02x}").unwrap();
    }
    result
}
