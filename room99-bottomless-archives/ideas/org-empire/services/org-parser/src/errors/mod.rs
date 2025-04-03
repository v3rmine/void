mod std_convertion;
pub use std_convertion::*;

mod span;
pub use span::*;

mod context_span;
pub use context_span::*;

use thiserror::Error as ErrorDerive;

use crate::PInput;

pub type PResult<'input, T, Input = PInput<'input>> =
    Result<(Input, T), nom::Err<ErrorTrace<Input>>>;

#[derive(Debug, Clone, PartialEq, Eq, ErrorDerive)]
#[error("{0:?}")]
pub struct ErrorTrace<Input>(pub Vec<Error<Input>>);

#[derive(Debug, Clone, PartialEq, Eq, ErrorDerive)]
pub enum Error<Input> {
    #[error("[{kind:?}] {context}: {input}")]
    WithContext {
        input: Input,
        context: String,
        kind: ErrorKind,
    },
    #[error("{kind:?}: {input}")]
    WithoutContext { input: Input, kind: ErrorKind },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    Context,
    Nom(nom::error::ErrorKind),
}

// Convertion between errors
impl From<nom::error::ErrorKind> for ErrorKind {
    fn from(src: nom::error::ErrorKind) -> Self {
        Self::Nom(src)
    }
}
