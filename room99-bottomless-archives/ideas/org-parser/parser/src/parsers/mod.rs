use nom::bytes::complete::tag;

use crate::{PInput, PResult};

pub mod blocks;
pub mod lines;
pub mod parts;

/// @TODO remove it: Require `#![feature(associated_type_defaults)]`
pub trait ParseTextToAst<'input, InputType = PInput<'input>, OutputType: Sized = Self>
where
    InputType: nom::InputTake,
    InputType: nom::InputLength,
{
    type Input = InputType;

    fn parse_text_to_ast(i: Self::Input) -> PResult<'input, OutputType>;
}

impl<'input> ParseTextToAst<'input> for &'input str {
    fn parse_text_to_ast(i: Self::Input) -> PResult<'input, Self> {
        let (i, _res) = tag("foobar")(i)?;

        Ok((i, ""))
    }
}
