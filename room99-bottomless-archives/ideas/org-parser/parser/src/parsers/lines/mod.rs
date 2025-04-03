use nom::{branch::alt, bytes::complete::take_till1, combinator::map, multi::many1};
use parser_helpers::is_newline;

use crate::{Header, LinesType, ParseTextToAst, PartsType};

mod header;

impl<'input> ParseTextToAst<'input> for LinesType<'input> {
    fn parse_text_to_ast(i: Self::Input) -> crate::PResult<'input, Self> {
        let (next, line) = take_till1(is_newline)(i)?;

        let (_, res) = alt((
            map(Header::parse_text_to_ast, LinesType::Header),
            map(many1(PartsType::parse_text_to_ast), LinesType::Plain),
        ))(line)?;

        Ok((next, res))
    }
}
