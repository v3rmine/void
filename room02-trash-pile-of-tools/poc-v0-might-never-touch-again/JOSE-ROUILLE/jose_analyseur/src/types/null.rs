use crate::types::{IResult, JoseType, ParseValue};
use nom::bytes::complete::{tag, take_while};
use nom::error::context;
use nom::sequence::tuple;
use std::borrow::Cow;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Null<'a> {
    inner: Cow<'a, str>,
}

impl<'a> From<&'a str> for Null<'a> {
    fn from(from: &'a str) -> Self {
        Self {
            inner: Cow::from(from),
        }
    }
}

impl<'a> From<String> for Null<'a> {
    fn from(from: String) -> Self {
        Self {
            inner: Cow::from(from),
        }
    }
}

impl<'a, 'b> ParseValue<'a, 'b> for Null<'b> {
    type Input = &'a str;
    fn parse(input: Self::Input) -> IResult<Self::Input, JoseType<'a, 'b>> {
        context(
            "nom parsing null value",
            tuple((tag("nul"), take_while(|c: char| c.is_alphabetic()))),
        )(input)
        .map(|(next_input, res)| {
            (
                next_input,
                JoseType::Null(Null::from([res.0, res.1].concat())),
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Null;
    use crate::types::{JoseType, ParseValue};

    #[test]
    fn test_parse_null() {
        assert_eq!(
            Null::parse("nul").unwrap(),
            ("", JoseType::Null(Null::from("nul")))
        );
        assert_eq!(
            Null::parse("nulle").unwrap(),
            ("", JoseType::Null(Null::from("nulle")))
        );
    }

    #[test]
    fn test_parse_null_error() {
        assert!(Null::parse("NOTNULL").err().is_some());
    }
}
