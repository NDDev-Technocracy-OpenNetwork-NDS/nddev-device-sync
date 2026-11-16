#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("invalid command; use nds-source --help")]
    Usage,
    #[error("source I/O failed ({0:?})")]
    Io(std::io::ErrorKind),
    #[error("invalid or oversized JSON input")]
    Json,
    #[error("source contract refused: {0}")]
    Contract(&'static str),
    #[error("native Git metadata could not be read")]
    Git,
    #[error("native Git metadata exceeded its time or output bound")]
    GitBound,
    #[error("source verifier is stale; prepare a verified tool on an execution host")]
    Stale,
    #[error("tool installation contains an unrelated or changed file")]
    Occupied,
}
impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error.kind())
    }
}
pub(crate) type Result<T> = std::result::Result<T, Error>;
