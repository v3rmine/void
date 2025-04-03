use nom::error::{ContextError, ErrorKind, ParseError, VerboseErrorKind};
use thiserror::Error;

#[derive(Error, Debug, Clone, Eq, PartialEq)]
pub enum JoseError {
    #[error("Not a JOSÉ Type")]
    NotAJoseType,
    #[error("Not a boolean")]
    NotABool,
    #[error("Not a gender")]
    NotAGender,
    #[error("Not a null")]
    NotANull,
    #[error("Not escaped")]
    NotEscaped,
    #[error("Not a string")]
    NotAString,
    #[error("Not an integer")]
    NotAnInteger,
    #[error("Not a table")]
    NotATable,
    #[error("Not an object")]
    NotAnObject,
    #[error("Not a key / value pair")]
    NotAKv,
    #[error("Generic JOSÉ parser error")]
    GenericParser(String),
}

impl<'a> From<JoseError> for nom::Err<VerboseJoseError> {
    fn from(from: JoseError) -> Self {
        #[allow(clippy::match_single_binding)]
        match from {
            _ => Self::Error(VerboseJoseError {
                errors: vec![(from, VerboseErrorKind::Context("JOSÉ initial error"))],
            }),
        }
    }
}

impl<'a> From<&'a str> for JoseError {
    fn from(from: &'a str) -> Self {
        Self::GenericParser(from.to_owned())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct VerboseJoseError {
    pub errors: Vec<(JoseError, VerboseErrorKind)>,
}

pub trait AppendToVerboseError {
    fn push(self, err: JoseError) -> Self;
}
impl AppendToVerboseError for nom::Err<VerboseJoseError> {
    fn push(self, err: JoseError) -> Self {
        self.map(|e| VerboseJoseError {
            errors: [
                e.errors,
                vec![(err, VerboseErrorKind::Context("JOSÉ appended error")); 1],
            ]
            .concat(),
        })
    }
}

impl<T> ParseError<T> for VerboseJoseError
where
    T: Into<JoseError>,
{
    fn from_error_kind(input: T, kind: ErrorKind) -> Self {
        Self {
            errors: vec![(input.into(), VerboseErrorKind::Nom(kind)); 1],
        }
    }

    fn append(input: T, kind: ErrorKind, other: Self) -> Self {
        Self {
            errors: [
                other.errors,
                vec![(input.into(), VerboseErrorKind::Nom(kind)); 1],
            ]
            .concat(),
        }
    }

    fn from_char(input: T, c: char) -> Self {
        Self {
            errors: vec![(input.into(), VerboseErrorKind::Char(c)); 1],
        }
    }

    fn or(self, other: Self) -> Self {
        Self {
            errors: [self.errors, other.errors].concat(),
        }
    }
}

impl<T> ContextError<T> for VerboseJoseError
where
    T: Into<JoseError>,
{
    fn add_context(input: T, ctx: &'static str, other: Self) -> Self {
        Self {
            errors: [
                other.errors,
                vec![(input.into(), VerboseErrorKind::Context(ctx)); 1],
            ]
            .concat(),
        }
    }
}
