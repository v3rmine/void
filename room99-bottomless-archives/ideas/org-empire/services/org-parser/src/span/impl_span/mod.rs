mod convertion;

pub use convertion::*;

mod span;
pub use span::*;

use crate::{LocatedSpan, Span};

impl<'input> Span<'input> {
    pub fn new(origin: &'input str) -> Self {
        Self {
            origin,
            fragment: origin,
            offset: 0,
            col: 1,
            line: 1,
        }
    }

    pub fn from_located_span<T>(src: T) -> Self
    where
        T: LocatedSpan,
        <T as LocatedSpan>::Content: 'input,
    {
        let (col, line) = src.position();

        Self {
            origin: src.origin(),
            col,
            line,
            fragment: src.fragment(),
            offset: src.offset(),
        }
    }
}

impl<'input> From<&'input str> for Span<'input> {
    fn from(src: &'input str) -> Self {
        Self::new(src)
    }
}
