use super::JoseType;
use crate::types::{IResult, ParseValue, parse_spaces_and_newlines};
use nom::bytes::complete::tag;
use nom::error::context;
use nom::multi::separated_list0;
use nom::sequence::{delimited, tuple};

#[non_exhaustive]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Table<'a, 'b> {
    inner: Vec<JoseType<'a, 'b>>,
}

impl<'a, 'b> From<Vec<JoseType<'a, 'b>>> for Table<'a, 'b> {
    fn from(from: Vec<JoseType<'a, 'b>>) -> Self {
        Self { inner: from }
    }
}

const TABLE_DELIM_BEGIN: &str = "DÉBUT";
const TABLE_DELIM_END: &str = "FIN";
const SEPARATOR: &str = " ; ";

impl<'a> ParseValue<'a, 'a> for Table<'a, 'a> {
    type Input = &'a str;
    fn parse(input: Self::Input) -> IResult<Self::Input, JoseType<'a, 'a>> {
        context(
            "nom parsing table",
            delimited(
                tuple((
                    tag(TABLE_DELIM_BEGIN),
                    parse_spaces_and_newlines,
                )),
                separated_list0(tag(SEPARATOR), JoseType::parse),
                tuple((
                    parse_spaces_and_newlines,
                    tag(TABLE_DELIM_END),
                )),
            ),
        )(input)
        .map(|(next_input, res)| (next_input, JoseType::Table(Table { inner: res })))
    }
}

#[cfg(test)]
mod tests {
    use super::Table;
    use crate::types::{string, JoseType, ParseValue};

    #[test]
    fn test_parse_table() {
        assert_eq!(
            Table::parse("DÉBUT « Seine-Maritime » ; « Eure » ; « Rhône » FIN").unwrap(),
            (
                "",
                JoseType::Table(Table::from(vec![
                    JoseType::String(string::String::from(" Seine-Maritime ")),
                    JoseType::String(string::String::from(" Eure ")),
                    JoseType::String(string::String::from(" Rhône ")),
                ]))
            )
        );
    }

    #[test]
    fn test_parse_table_error() {
        assert!(Table::parse("NOTTABLE").err().is_some());
    }
}
