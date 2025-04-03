use crate::{Header, ParseTextToAst};

impl<'input> ParseTextToAst<'input> for Header<'input> {
    fn parse_text_to_ast(i: Self::Input) -> crate::PResult<'input, Self> {
        // let x = map(
        //     tuple((
        //         map(take_until1(LINE_HEADER_SYMBOL), |level: PInput<'input>| {
        //             level.len()
        //         }),
        //         take_until1(i),
        //     )),
        //     |s| s,
        // )(i);
        todo!()
    }
}
