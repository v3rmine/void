use crate::types::{IResult, JoseType, ParseValue};
use nom::bytes::complete::take_while1;
use nom::error::context;
use std::borrow::Cow;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Integer<'a> {
    inner: Cow<'a, str>,
}

impl<'a> From<&'a str> for Integer<'a> {
    fn from(from: &'a str) -> Self {
        Self {
            inner: Cow::from(from),
        }
    }
}

impl<'a, 'b> ParseValue<'a, 'b> for Integer<'b> {
    type Input = &'a str;
    fn parse(input: Self::Input) -> IResult<Self::Input, JoseType<'a, 'b>> {
        context(
            "nom parsing integer value",
            take_while1(|c: char| (c.is_alphabetic() && c.is_lowercase()) || c.is_whitespace() || c == '-'),
        )(input)
        .map(|(next_input, res)| (next_input, JoseType::Integer(Integer::from(res))))
    }
}

#[cfg(test)]
mod tests {
    use super::Integer;
    use crate::types::{JoseType, ParseValue};

    #[test]
    fn test_parse_string() {
        assert_eq!(
            Integer::parse("huit millions quatre mille neuf cent cinquante-huit").unwrap(),
            (
                "",
                JoseType::Integer(Integer::from(
                    "huit millions quatre mille neuf cent cinquante-huit"
                ))
            )
        );
    }

    #[test]
    fn test_parse_string_error() {
        assert!(Integer::parse("&*&%*#").err().is_some());
    }
}
