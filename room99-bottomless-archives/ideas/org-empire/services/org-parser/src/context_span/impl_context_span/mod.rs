mod convertion;
pub use convertion::*;

mod span;
pub use span::*;

use crate::{ContextSpan, LocatedSpan, Span};

impl<'input> ContextSpan<'input> {
    pub fn new(origin: &'input str) -> Self {
        Self {
            span: Span::new(origin),
            contexts: Vec::new(),
        }
    }

    fn from_located_span<T: LocatedSpan>(src: T) -> Self {
        Self {
            span: Span::from_located_span(src),
            contexts: Vec::new(),
        }
    }
}

impl<'input> From<&'input str> for ContextSpan<'input> {
    fn from(src: &'input str) -> Self {
        Self::new(src)
    }
}
