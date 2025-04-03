use thiserror::Error;

#[derive(Debug, Error)]
pub enum Code2dParserError {
    #[allow(dead_code)]
    #[error("Unspecified error thrown")]
    Unknown,
}

pub type Code2dParserResult<T> = Result<T, Code2dParserError>;
