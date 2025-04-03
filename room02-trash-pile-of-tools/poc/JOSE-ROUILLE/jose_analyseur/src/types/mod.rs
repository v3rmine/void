use crate::errors::VerboseJoseError;
use nom::branch::alt;
use nom::bytes::complete::take_while;
use nom::error::context;

mod bool;
mod escape;
mod integer;
mod null;
mod object;
mod string;
mod table;

#[non_exhaustive]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum JoseType<'a, 'b> {
    Table(table::Table<'a, 'b>),
    Bool(bool::Bool),
    String(string::String<'a>),
    Null(null::Null<'a>),
    Integer(integer::Integer<'a>),
    Escape(escape::Escape),
    Object(object::Object<'a, 'b>),
}

pub type IResult<T, U> = nom::IResult<T, U, VerboseJoseError>;

#[allow(clippy::result_unit_err)]
pub fn parse_spaces_and_newlines(input: &str) -> IResult<&str, ()> {
    context(
        "nom parsing whitespaces",
        take_while(|c: char| c.is_whitespace() || c == '\n'),
    )(input)
    .map(|(next_input, _)| (next_input, ()))
}

pub trait ParseValue<'a, 'b> {
    type Input;
    fn parse(input: Self::Input) -> IResult<Self::Input, JoseType<'a, 'b>>;
}

impl<'a> ParseValue<'a, 'a> for JoseType<'a, 'a> {
    type Input = &'a str;
    fn parse(input: Self::Input) -> IResult<Self::Input, JoseType<'a, 'a>> {
        context(
            "nom parsing jose",
            alt((
                bool::Bool::parse,
                null::Null::parse,
                escape::Escape::parse,
                string::String::parse,
                table::Table::parse,
                object::Object::parse,
                integer::Integer::parse
            )),
        )(input)
    }
}

impl<'a, 'b> From<&'a str> for JoseType<'a, 'b> {
    fn from(from: &'a str) -> Self {
        JoseType::String(string::String::from(from))
    }
}

impl<'a, 'b> From<bool> for JoseType<'a, 'b> {
    fn from(from: bool) -> Self {
        JoseType::Bool(bool::Bool::from(from))
    }
}

impl<'a, 'b> From<Vec<JoseType<'a, 'b>>> for JoseType<'a, 'b> {
    fn from(from: Vec<JoseType<'a, 'b>>) -> Self {
        JoseType::Table(table::Table::from(from))
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{JoseType, ParseValue, object};
    use std::borrow::Cow;

    #[test]
    fn test_parse_jose() {
        assert_eq!(
            JoseType::parse("OBJET Masculin
    — « Confinement » : Vrai ;
    — « vaccins » : huit millions quatre mille neuf cent cinquante-huit ;
    — « restrictions » :
        OBJET Féminin
            — « Écoles ouvertes » : Faux ;
            — « départements confinés » :
                DÉBUT « Seine-Maritime » ; « Eure » ; « Rhône » FIN.
        TEJBO ;
    — « motivation » : nulle.
TEJBO").unwrap(),
            ("", JoseType::Object(object::Object::from((
                object::Gender::Masculine,
                vec![
                    (Cow::from(" Confinement "),  (JoseType::Bool(true.into()), false)),
                    (Cow::from(" vaccins "),  (JoseType::Integer("huit millions quatre mille neuf cent cinquante-huit ".into()), false)),
                    (Cow::from(" restrictions "), (JoseType::Object(object::Object::from((
                        object::Gender::Feminine,
                        vec![
                            (Cow::from(" départements confinés "),  (JoseType::Table(vec![
                                " Seine-Maritime ".into(),
                                " Eure ".into(),
                                " Rhône ".into(),
                            ].into()), true)),
                            (Cow::from(" Écoles ouvertes "),  (JoseType::Bool(false.into()), false)),
                        ].into_iter().collect()
                    ))), false)),
                    (Cow::from(" motivation "), (JoseType::Null("nulle".into()), true)),
                ].into_iter().collect()
            ))))
        );
    }

    #[test]
    fn test_parse_jose_error() {
        assert!(JoseType::parse("NOTJOSÉ").err().is_some());
    }
}
