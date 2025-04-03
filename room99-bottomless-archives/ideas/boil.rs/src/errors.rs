use std::path::PathBuf;
use thiserror::Error;

pub type StandardResult<T> = std::result::Result<T, BoilrError>;

#[derive(Error, Debug)]
pub enum BoilrError {
    #[error("Error cannot format {0:?}")]
    FormatDisplayError(#[from] std::fmt::Error),
    #[error("Error cannot read from {path:?}")]
    ReadError {
        source: std::io::Error,
        path: PathBuf,
    },
    #[error("Error cannot write to {path:?}")]
    WriteError {
        source: std::io::Error,
        path: PathBuf,
    },
    #[error("Error cannot copy from {from_path:?} {to_path:?}")]
    CopyError {
        source: Box<BoilrError>,
        from_path: PathBuf,
        to_path: PathBuf,
    },
    #[error("Error cannot delete {path:?}")]
    DeleteError {
        source: std::io::Error,
        path: PathBuf,
    },
    #[error("Error while deserializing from {path:?}")]
    TomlDeserializeError {
        source: toml::de::Error,
        path: PathBuf,
    },
    #[error("Error while serializing at {path:?}")]
    TomlSerializeError {
        source: toml::ser::Error,
        path: PathBuf,
    },
    #[error("Error while displaying on terminal")]
    TerminalError { source: std::io::Error },
    #[error("Error while parsing files using Tera")]
    TeraTemplateError(#[from] tera::Error),
    #[error("Error while parsing directories")]
    WalkDirError(#[from] walkdir::Error),
    #[error("Internal path stripping error")]
    StripPrefixError(#[from] std::path::StripPrefixError),
    #[error("Cannot convert to String")]
    StrError,
    #[error("Cannot find home dir")]
    HomeDirNotFoundError,
    #[error("Cannot access current directory")]
    AccessCurrentDirError,
    #[error("Error arg not found in clap args")]
    ArgNotFoundError,
    #[error("{path:?} is not a directory")]
    NotADirectoryError { path: PathBuf },
    #[error("Generic error")]
    UnspecifiedError(Option<String>),
}
