use std::num::ParseIntError;

use crate::{Error, ErrorKind, ErrorTrace};

pub trait IntoNomErr<'input, FromError> {
    type Input;

    fn into_nom_err(
        input: &'input Self::Input,
    ) -> Box<dyn Fn(ParseIntError) -> nom::Err<Self> + 'input>
    where
        Self: Sized,
        Self::Input: Clone;
}

impl<'input, InputType> IntoNomErr<'input, ParseIntError> for ErrorTrace<InputType> {
    type Input = InputType;

    fn into_nom_err(
        input: &'input Self::Input,
    ) -> Box<dyn Fn(ParseIntError) -> nom::Err<Self> + 'input>
    where
        Self: Sized,
        Self::Input: Clone,
    {
        Box::new(|err: ParseIntError| {
            nom::Err::Error(ErrorTrace(vec![Error::WithContext {
                context: err.to_string(),
                input: input.clone(),
                kind: ErrorKind::Context,
            }]))
        })
    }
}
