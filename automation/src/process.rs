use crate::error::{Error, Result};
use std::{ffi::OsStr, path::Path, process::Stdio, time::Duration};
use tokio::{
    io::{AsyncRead, AsyncReadExt},
    process::Command,
};

async fn bounded(reader: impl AsyncRead + Unpin, maximum: u64) -> Result<Vec<u8>> {
    let mut output = Vec::new();
    reader.take(maximum + 1).read_to_end(&mut output).await?;
    if output.len() as u64 > maximum {
        return Err(Error::GitBound);
    }
    Ok(output)
}

/// Only native read-only Git operations use this capture path. Recipe execution
/// inherits its streams and retains the component's own operation contract.
pub(crate) async fn git(
    root: &Path,
    args: impl IntoIterator<Item = impl AsRef<OsStr>>,
) -> Result<String> {
    let mut child = Command::new("git")
        .args(args)
        .current_dir(root)
        .env("GIT_OPTIONAL_LOCKS", "0")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()?;
    let stdout = child.stdout.take().ok_or(Error::Git)?;
    let stderr = child.stderr.take().ok_or(Error::Git)?;
    let captured = tokio::time::timeout(Duration::from_secs(30), async {
        let (stdout, _, status) = tokio::try_join!(
            bounded(stdout, 8 * 1024 * 1024),
            bounded(stderr, 64 * 1024),
            async { child.wait().await.map_err(Error::from) }
        )?;
        if !status.success() {
            return Err(Error::Git);
        }
        String::from_utf8(stdout).map_err(|_| Error::Git)
    })
    .await;
    match captured {
        Ok(Ok(output)) => Ok(output),
        other => {
            // Reap this exact owned child; do not signal by a discovered PID.
            if child.try_wait()?.is_none() {
                child.kill().await?;
            }
            match other {
                Ok(Err(error)) => Err(error),
                _ => Err(Error::GitBound),
            }
        }
    }
}
