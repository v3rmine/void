#[macro_export]
macro_rules! generate_simple_separated {
    ($separator:expr, $type_name:ident) => {
        impl<'input> ParseTextToAst<'input> for $type_name<'input> {
            fn parse_text_to_ast(i: Self::Input) -> crate::PResult<'input, Self> {
                nom::combinator::map(
                    nom::sequence::delimited(
                        nom::bytes::complete::tag($separator),
                        nom::bytes::complete::take_until($separator),
                        nom::bytes::complete::tag($separator),
                    ),
                    $type_name,
                )(i)
            }
        }
    };
}
