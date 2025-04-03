mod checkbox;
mod counter;
mod link;
mod simple_part_generator;
mod subscript;
mod superscript;

use crate::{
    generate_simple_separated,
    nom_helpers::{is_newline, is_spacing},
    Bold, Checkbox, Code, Counter, Italic, Link, LocatedSpan, PInput, PResult, Parser, PartsType,
    Strikethrough, Subscript, Superscript, Underline, Verbatim, BOLD_SURROUNDING_SYMBOL,
    CODE_SURROUNDING_SYMBOL, ITALIC_SURROUNDING_SYMBOL, STRIKETHROUGH_SURROUNDING_SYMBOL,
    UNDERLINE_SURROUNDING_SYMBOL, VERBATIM_SURROUNDING_SYMBOL,
};

use nom::{branch::alt, bytes::complete::take_till1, combinator::map, multi::fold_many1};

impl<'input> Parser<'input> for PartsType<'input> {
    fn parse_text(i: Self::Input) -> PResult<'input, Self> {
        alt((
            map(Superscript::parse_text, PartsType::Superscript),
            map(Subscript::parse_text, PartsType::Subscript),
            map(Bold::parse_text, PartsType::Bold),
            map(Italic::parse_text, PartsType::Italic),
            map(Strikethrough::parse_text, PartsType::StrikeThrough),
            map(Underline::parse_text, PartsType::Underline),
            map(Code::parse_text, PartsType::Code),
            map(Verbatim::parse_text, PartsType::Verbatim),
            map(Counter::parse_text, PartsType::Counter),
            map(Checkbox::parse_text, PartsType::Checkbox),
            map(Link::parse_text, PartsType::Link),
        ))(i)
    }
}

impl<'input> PartsType<'input> {
    pub fn parse_many_parts(i: PInput<'input>) -> PResult<'input, Vec<Self>> {
        fold_many1(
            |i: PInput<'input>| -> PResult<'input, PartsType<'input>> {
                let res = match PartsType::parse_text(i) {
                    Ok((next, part)) => Ok((next, part)),
                    Err(_) => map(
                        take_till1(|c| {
                            // We add '^' and '_' for subscripts and superscripts
                            // which can start inside a word
                            is_spacing(c) || c == '^' || c == '_'
                        }),
                        PartsType::Plain,
                    )(i),
                };

                dbg!(&res);
                res
            },
            Vec::new,
            |mut parts: Vec<_>, part| {
                if let (PartsType::Plain(part), Some(PartsType::Plain(prev_part))) =
                    (&part, parts.last_mut())
                {
                    *prev_part = prev_part.extend_with(part);
                } else {
                    parts.push(part);
                }

                parts
            },
        )(i)
    }
}

// Parsers
generate_simple_separated!(BOLD_SURROUNDING_SYMBOL, Bold);
generate_simple_separated!(ITALIC_SURROUNDING_SYMBOL, Italic);
generate_simple_separated!(STRIKETHROUGH_SURROUNDING_SYMBOL, Strikethrough);
generate_simple_separated!(UNDERLINE_SURROUNDING_SYMBOL, Underline);
generate_simple_separated!(CODE_SURROUNDING_SYMBOL, Code);
generate_simple_separated!(VERBATIM_SURROUNDING_SYMBOL, Verbatim);
pub use checkbox::*;
pub use counter::*;
pub use link::*;
pub use subscript::*;
pub use superscript::*;

#[cfg(test)]
mod tests {
    use assay::assay;

    use crate::{Bold, LocatedSpan, Parser, PartsType};

    #[assay]
    fn parse_bold() {
        let (next, parsed) = Bold::parse_text("*Hello boldy*".into())?;
        assert_eq!(parsed.0.fragment(), "Hello boldy");
        assert_eq!(next.fragment(), "");
    }

    #[assay]
    fn parse_many_parts() {
        let (_, res) = PartsType::parse_many_parts("*that is some* /fucking dope/ 1^{33}".into())?;
        dbg!(res);

        assert_eq!(1, 2);
    }
}
