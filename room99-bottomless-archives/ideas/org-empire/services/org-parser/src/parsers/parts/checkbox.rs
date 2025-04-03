use crate::{
    Checkbox, PResult, Parser, CHECKBOX_CHECKED_SYMBOL, CHECKBOX_SURROUNDING_SYMBOL_END,
    CHECKBOX_SURROUNDING_SYMBOL_START, CHECKBOX_UNCHECKED_SYMBOL,
};
use nom::{branch::alt, bytes::complete, combinator::map, sequence::delimited};

impl<'input> Parser<'input> for Checkbox {
    fn parse_text(i: Self::Input) -> PResult<'input, Self> {
        delimited(
            complete::tag(CHECKBOX_SURROUNDING_SYMBOL_START),
            alt((
                map(complete::tag_no_case(CHECKBOX_CHECKED_SYMBOL), |_| {
                    Checkbox(true)
                }),
                map(complete::tag_no_case(CHECKBOX_UNCHECKED_SYMBOL), |_| {
                    Checkbox(false)
                }),
            )),
            complete::tag(CHECKBOX_SURROUNDING_SYMBOL_END),
        )(i)
    }
}

#[cfg(test)]
mod tests {
    use assay::assay;

    use crate::{Checkbox, Parser};

    #[assay]
    fn parse_checkbox() {
        let (_, checked_checkbox) = Checkbox::parse_text("[x]".into())?;
        let (_, empty_checkbox) = Checkbox::parse_text("[ ]".into())?;

        assert_eq!(checked_checkbox.0, true);
        assert_eq!(empty_checkbox.0, false);
    }
}
