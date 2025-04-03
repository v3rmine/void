use crate::errors::JoseError;
use crate::types::IResult;
use nom::bytes::complete::take_while1;
use nom::error::context;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Gender {
    Masculine,
    Feminine,
}

impl Gender {
    pub(crate) fn parse(input: &str) -> IResult<&str, Self> {
        context(
            "nom parsing gender",
            take_while1(|c: char| c.is_alphabetic()),
        )(input)
        .and_then(|(next_input, res)| match res {
            "Féminin" => Ok((next_input, Self::Feminine)),
            "Masculin" => Ok((next_input, Self::Masculine)),
            _ => Err(JoseError::NotAGender.into()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Gender;

    #[test]
    fn test_parse_gender() {
        assert_eq!(Gender::parse("Féminin").unwrap(), ("", Gender::Feminine));
        assert_eq!(Gender::parse("Masculin").unwrap(), ("", Gender::Masculine));
    }

    #[test]
    fn test_parse_gender_error() {
        assert!(Gender::parse("NOTGENDER").err().is_some());
        assert!(Gender::parse("69420").err().is_some());
    }
}
