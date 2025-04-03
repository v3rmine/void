use crate::{
    nom_helpers::DIGIT_LIST, Counter, ErrorTrace, IntoNomErr, LocatedSpan, PResult, Parser,
    COUNTER_SEPARATOR_SYMBOL, COUNTER_SURROUNDING_SYMBOL_END, COUNTER_SURROUNDING_SYMBOL_START,
};
use nom::{
    bytes::complete,
    sequence::{delimited, separated_pair},
};

impl<'input> Parser<'input> for Counter {
    fn parse_text(i: Self::Input) -> PResult<'input, Self> {
        let (next, (current, total)) = delimited(
            complete::tag(COUNTER_SURROUNDING_SYMBOL_START),
            separated_pair(
                complete::is_a(DIGIT_LIST),
                complete::tag(COUNTER_SEPARATOR_SYMBOL),
                complete::is_a(DIGIT_LIST),
            ),
            complete::tag(COUNTER_SURROUNDING_SYMBOL_END),
        )(i)?;

        let counter = Counter {
            current: current
                .parse::<u32>()
                .map_err(ErrorTrace::into_nom_err(&next))?,
            total: total
                .parse::<u32>()
                .map_err(ErrorTrace::into_nom_err(&next))?,
        };

        Ok((next, counter))
    }
}

impl Counter {
    pub fn is_complete(&self) -> bool {
        self.current == self.total
    }
}

#[cfg(test)]
mod tests {
    use assay::assay;

    use crate::{Counter, Parser};

    #[assay]
    fn parse_counter() {
        let (_, counter) = Counter::parse_text("[1/3]".into())?;

        assert_eq!(counter.current, 1);
        assert_eq!(counter.total, 3);
    }
}
