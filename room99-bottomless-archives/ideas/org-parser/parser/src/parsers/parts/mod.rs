mod checkbox;
mod counter;
mod link;
mod simple_part_generator;
mod subscript;
mod superscript;

use crate::{
    generate_simple_separated, Bold, Checkbox, Code, Counter, Italic, Link, PInput, PResult,
    ParseTextToAst, PartsType, Strikethrough, Subscript, Superscript, Underline, Verbatim,
    BOLD_SURROUNDING_SYMBOL, CODE_SURROUNDING_SYMBOL, ITALIC_SURROUNDING_SYMBOL,
    STRIKETHROUGH_SURROUNDING_SYMBOL, UNDERLINE_SURROUNDING_SYMBOL, VERBATIM_SURROUNDING_SYMBOL,
};

use located_span::LocatedSpan;
use nom::{
    branch::alt,
    bytes::complete::{take_till1, take_while1},
    combinator::map,
    multi::fold_many1,
};
use parser_helpers::is_spacing;

impl<'input> ParseTextToAst<'input> for PartsType<'input> {
    fn parse_text_to_ast(i: Self::Input) -> PResult<'input, Self> {
        alt((
            map(Superscript::parse_text_to_ast, PartsType::Superscript),
            map(Subscript::parse_text_to_ast, PartsType::Subscript),
            map(Bold::parse_text_to_ast, PartsType::Bold),
            map(Italic::parse_text_to_ast, PartsType::Italic),
            map(Strikethrough::parse_text_to_ast, PartsType::StrikeThrough),
            map(Underline::parse_text_to_ast, PartsType::Underline),
            map(Code::parse_text_to_ast, PartsType::Code),
            map(Verbatim::parse_text_to_ast, PartsType::Verbatim),
            map(Counter::parse_text_to_ast, PartsType::Counter),
            map(Checkbox::parse_text_to_ast, PartsType::Checkbox),
            map(Link::parse_text_to_ast, PartsType::Link),
        ))(i)
    }
}

impl<'input> PartsType<'input> {
    pub fn parse_many_parts(i: PInput<'input>) -> PResult<'input, Vec<Self>> {
        fold_many1(
            alt((
                PartsType::parse_text_to_ast,
                // We add '^' and '_' for subscripts and superscripts
                // which can start inside a word
                map(
                    take_till1(|c| is_spacing(c) || c == '^' || c == '_'),
                    PartsType::Plain,
                ),
                map(take_while1(is_spacing), PartsType::Plain),
            )),
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

    use crate::{Bold, ParseTextToAst, PartsType};
    use located_span::LocatedSpan;

    #[assay]
    fn parse_bold() {
        let (next, parsed) = Bold::parse_text_to_ast("*Hello boldy*".into())?;
        assert_eq!(parsed.0.fragment(), "Hello boldy");
        assert_eq!(next.fragment(), "");
    }

    #[assay]
    fn parse_many_parts() {
        //let res = PartsType::parse_many_parts("*that is some* /fucking dope/ 1^{33}".into())?;
        let res = PartsType::parse_many_parts("  tt  ".into());
        dbg!(res);

        assert_eq!(1, 2);
    }
}
