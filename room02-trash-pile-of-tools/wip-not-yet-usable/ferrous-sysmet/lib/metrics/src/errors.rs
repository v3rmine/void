use thiserror::Error;

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to get stat from psutil because: {0}")]
    Psutil(#[from] psutil::Error),
    // SemVer
    #[cfg(feature = "database")]
    #[error("SemVer failed")]
    SemVer(#[from] semver::Error),
    // Ciborium
    #[cfg(feature = "database")]
    #[error("Failed to convert data from cbor")]
    CborDeserialize(#[from] ciborium::de::Error<std::io::Error>),
    #[cfg(feature = "database")]
    #[error("Failed to convert data to cbor")]
    CborSerialize(#[from] ciborium::ser::Error<std::io::Error>),
    // Database (file management)
    #[cfg(feature = "database")]
    #[error("Provided path is invalid: {0}")]
    InvalidPath(std::convert::Infallible),
    #[cfg(feature = "database")]
    #[error("Failed to open file: {0}")]
    FailedToOpenFile(std::io::Error),
    #[cfg(feature = "database")]
    #[error("Failed to get file metadata: {0}")]
    FailedToGetFileMetadata(std::io::Error),
    #[cfg(feature = "database")]
    #[error("Failed to write to file: {0}")]
    FailedToWriteFile(std::io::Error),
    #[cfg(feature = "database")]
    #[error("Failed to set file cursor: {0}")]
    FailedToSetFileCursor(std::io::Error),
    #[cfg(feature = "database")]
    #[error("Failed to remove file: {0}")]
    FailedToRemoveFile(std::io::Error),
    #[cfg(feature = "database")]
    #[error("Timeout while trying to lock {0:?}")]
    LockFileTimeout(std::path::PathBuf),
    // Chrono
    #[error("Oldest date is too big to big calculated")]
    OldestDateOverflow,
}
