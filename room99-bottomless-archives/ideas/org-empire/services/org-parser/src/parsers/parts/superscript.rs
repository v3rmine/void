use nom::{
    bytes::complete::{self, take_until},
    combinator::map,
    sequence::{delimited, preceded},
};

use crate::{
    PResult, Parser, Superscript, SUPERSCRIPT_START_SYMBOL, SUPERSCRIPT_SURROUNDING_SYMBOL_END,
    SUPERSCRIPT_SURROUNDING_SYMBOL_START,
};

impl<'input> Parser<'input> for Superscript<'input> {
    fn parse_text(i: Self::Input) -> PResult<'input, Self> {
        map(
            preceded(
                complete::tag(SUPERSCRIPT_START_SYMBOL),
                delimited(
                    complete::tag(SUPERSCRIPT_SURROUNDING_SYMBOL_START),
                    take_until(SUPERSCRIPT_SURROUNDING_SYMBOL_END),
                    complete::tag(SUPERSCRIPT_SURROUNDING_SYMBOL_END),
                ),
            ),
            Superscript,
        )(i)
    }
}

#[cfg(test)]
mod tests {
    use assay::assay;

    use crate::{Parser, Superscript};

    #[assay]
    fn parse_superscript() {
        let (_, script) = Superscript::parse_text("^{I'm sup}".into())?;
        assert_eq!(script.0, "I'm sup");
    }
}
