use nom::{bytes::complete::take_until1, combinator::map, sequence::tuple};

use crate::{Header, PInput, Parser, LINE_HEADER_SYMBOL};

impl<'input> Parser<'input> for Header<'input> {
    fn parse_text(i: Self::Input) -> crate::PResult<'input, Self> {
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
