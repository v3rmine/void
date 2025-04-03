use nom::{
    bytes::complete::{self, take_until1},
    combinator::{map, opt},
    sequence::{delimited, tuple},
};

use crate::{
    Link, PResult, ParseTextToAst, LINK_DESCRIPTION_SURROUNDING_SYMBOL_START,
    LINK_DESCRITPION_SURROUNDING_SYMBOL_END, LINK_SURROUNDING_SYMBOL_END,
    LINK_SURROUNDING_SYMBOL_START, LINK_TARGET_SURROUNDING_SYMBOL_END,
    LINK_TARGET_SURROUNDING_SYMBOL_START,
};

impl<'input> ParseTextToAst<'input> for Link<'input> {
    fn parse_text_to_ast(i: Self::Input) -> PResult<'input, Self> {
        delimited(
            complete::tag(LINK_SURROUNDING_SYMBOL_START),
            map(
                tuple((
                    delimited(
                        complete::tag(LINK_TARGET_SURROUNDING_SYMBOL_START),
                        take_until1(LINK_TARGET_SURROUNDING_SYMBOL_END),
                        complete::tag(LINK_TARGET_SURROUNDING_SYMBOL_END),
                    ),
                    opt(delimited(
                        complete::tag(LINK_DESCRIPTION_SURROUNDING_SYMBOL_START),
                        take_until1(LINK_DESCRITPION_SURROUNDING_SYMBOL_END),
                        complete::tag(LINK_DESCRITPION_SURROUNDING_SYMBOL_END),
                    )),
                )),
                |(target, desc)| Link { target, desc },
            ),
            complete::tag(LINK_SURROUNDING_SYMBOL_END),
        )(i)
    }
}

impl<'input> Link<'input> {
    pub fn is_website_link(&self) -> bool {
        self.target.starts_with("http://") || self.target.starts_with("https://")
    }
}

#[cfg(test)]
mod tests {
    use assay::assay;

    use crate::{Link, ParseTextToAst};

    #[assay]
    fn parse_web_link() {
        let (_, parsed) = Link::parse_text_to_ast("[[https://example.org/]]".into())?;
        assert_eq!(parsed.target, "https://example.org/");
        assert_eq!(parsed.desc, None);
        assert!(parsed.is_website_link());
    }
}
