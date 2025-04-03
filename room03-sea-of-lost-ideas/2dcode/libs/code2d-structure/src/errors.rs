use thiserror::Error;

#[derive(Debug, Error)]
pub enum Code2dStructureError {
    #[allow(dead_code)]
    #[error("Unspecified error thrown")]
    Unknown,
}

pub type Code2dStructureResult<T> = Result<T, Code2dStructureError>;
