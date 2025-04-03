use nom::{branch::alt, bytes::complete::take_till1, combinator::map, multi::many1};

use crate::{nom_helpers::is_newline, Header, LinesType, Parser, PartsType};

mod header;

impl<'input> Parser<'input> for LinesType<'input> {
    fn parse_text(i: Self::Input) -> crate::PResult<'input, Self> {
        let (next, line) = take_till1(is_newline)(i)?;

        let (_, res) = alt((
            map(Header::parse_text, LinesType::Header),
            map(many1(PartsType::parse_text), LinesType::Plain),
        ))(line)?;

        Ok((next, res))
    }
}
