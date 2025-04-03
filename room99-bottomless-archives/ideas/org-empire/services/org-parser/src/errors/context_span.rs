use nom::error::{ContextError, ParseError};

use crate::{ContextSpan, Error, ErrorKind, ErrorTrace};

// Simple error
impl<'input, Input: Into<ContextSpan<'input>>> ParseError<Input> for Error<ContextSpan<'input>> {
    fn from_error_kind(input: Input, kind: nom::error::ErrorKind) -> Self {
        Self::WithoutContext {
            input: input.into(),
            kind: kind.into(),
        }
    }

    fn append(input: Input, kind: nom::error::ErrorKind, _other: Self) -> Self {
        Self::from_error_kind(input, kind)
    }
}

impl<'input, Input: Into<ContextSpan<'input>>> ContextError<Input> for Error<ContextSpan<'input>> {
    fn add_context(input: Input, ctx: &'static str, _other: Self) -> Self {
        Self::WithContext {
            input: input.into(),
            context: ctx.to_string(),
            kind: ErrorKind::Context,
        }
    }
}

// Verbose error
impl<'input, Input: Into<ContextSpan<'input>>> ParseError<Input>
    for ErrorTrace<ContextSpan<'input>>
{
    fn from_error_kind(input: Input, kind: nom::error::ErrorKind) -> Self {
        Self([Error::from_error_kind(input, kind)].into())
    }

    fn append(input: Input, kind: nom::error::ErrorKind, other: Self) -> Self {
        let mut errors = other.0;
        errors.push(Error::from_error_kind(input, kind));

        Self(errors)
    }
}

impl<'input, Input: Into<ContextSpan<'input>>> ContextError<Input>
    for ErrorTrace<ContextSpan<'input>>
{
    fn add_context(input: Input, ctx: &'static str, other: Self) -> Self {
        let mut errors = other.0;
        errors.push(Error::WithContext {
            input: input.into(),
            context: ctx.to_string(),
            kind: ErrorKind::Context,
        });

        Self(errors)
    }
}
