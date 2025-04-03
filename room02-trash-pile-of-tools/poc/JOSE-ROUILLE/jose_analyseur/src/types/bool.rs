use crate::errors::JoseError;
use crate::types::{IResult, JoseType, ParseValue};
use nom::bytes::complete::take_while1;
use nom::error::context;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Bool {
    Vrai,
    Faux,
}

impl From<bool> for Bool {
    fn from(from: bool) -> Self {
        if from {
            Self::Vrai
        } else {
            Self::Faux
        }
    }
}

impl<'a, 'b> ParseValue<'a, 'b> for Bool {
    type Input = &'a str;
    fn parse(input: Self::Input) -> IResult<Self::Input, JoseType<'a, 'b>> {
        context(
            "nom parsing boolean",
            take_while1(|c: char| c.is_alphabetic()),
        )(input)
        .and_then(|(next_input, res)| match res {
            "Vrai" => Ok((next_input, JoseType::Bool(Self::Vrai))),
            "Faux" => Ok((next_input, JoseType::Bool(Self::Faux))),
            _ => Err(JoseError::NotABool.into()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Bool;
    use crate::types::{JoseType, ParseValue};

    #[test]
    fn test_parse_bool() {
        assert_eq!(
            Bool::parse("Vrai").unwrap(),
            ("", JoseType::Bool(Bool::Vrai))
        );
        assert_eq!(
            Bool::parse("Faux").unwrap(),
            ("", JoseType::Bool(Bool::Faux))
        );
    }

    #[test]
    fn test_parse_bool_error() {
        assert!(Bool::parse("NOTBOOL").err().is_some());
    }
}
