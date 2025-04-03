use nom::{
    bytes::complete::{self, take_until},
    combinator::map,
    sequence::{delimited, preceded},
};

use crate::{
    PResult, Parser, Subscript, SUBSCRIPT_START_SYMBOL, SUBSCRIPT_SURROUNDING_SYMBOL_END,
    SUBSCRIPT_SURROUNDING_SYMBOL_START,
};

impl<'input> Parser<'input> for Subscript<'input> {
    fn parse_text(i: Self::Input) -> PResult<'input, Self> {
        map(
            preceded(
                complete::tag(SUBSCRIPT_START_SYMBOL),
                delimited(
                    complete::tag(SUBSCRIPT_SURROUNDING_SYMBOL_START),
                    take_until(SUBSCRIPT_SURROUNDING_SYMBOL_END),
                    complete::tag(SUBSCRIPT_SURROUNDING_SYMBOL_END),
                ),
            ),
            Subscript,
        )(i)
    }
}

#[cfg(test)]
mod tests {
    use assay::assay;

    use crate::{Parser, Subscript};

    #[assay]
    fn parse_subscript() {
        let (_, script) = Subscript::parse_text("_{I'm sub}".into())?;
        assert_eq!(script.0, "I'm sub");
    }
}
