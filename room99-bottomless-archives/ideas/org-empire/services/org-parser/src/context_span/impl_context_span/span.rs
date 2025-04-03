use std::str::FromStr;

use crate::{ContextSpan, LocatedSpan};

impl<'input> LocatedSpan for ContextSpan<'input> {
    type Content = &'input str;

    fn origin(&self) -> Self::Content {
        self.span.origin()
    }

    fn fragment(&self) -> Self::Content {
        self.span.fragment()
    }

    fn offset(&self) -> usize {
        self.span.offset()
    }

    fn end_offset(&self) -> usize {
        self.span.end_offset()
    }

    /// @TODO test offset
    /// **Does not update col and line**
    fn set_offset(&self, offset: usize) -> Self {
        Self {
            span: self.span.set_offset(offset),
            ..self.clone()
        }
    }

    /// @TODO test offset
    /// **Does not update col and line**
    fn set_end_offset(&self, offset: usize) -> Self {
        Self {
            span: self.span.set_end_offset(offset),
            ..self.clone()
        }
    }

    fn len(&self) -> usize {
        self.span.len()
    }

    /// @TODO test offset
    /// **Does not update col and line**
    fn set_len(&self, len: usize) -> Self {
        Self {
            span: self.span.set_len(len),
            ..self.clone()
        }
    }

    fn is_empty(&self) -> bool {
        self.span.is_empty()
    }

    fn position(&self) -> (usize, usize) {
        self.span.position()
    }

    fn position_after(&self, count: usize) -> (usize, usize) {
        self.span.position_after(count)
    }

    fn set_position(&self, col: usize, line: usize) -> Self {
        Self {
            span: self.span.set_position(col, line),
            ..self.clone()
        }
    }

    fn recompute_position_till_offset(&self) -> Self {
        Self {
            span: self.span.recompute_position_till_offset(),
            ..self.clone()
        }
    }

    fn parse<T: FromStr>(&self) -> Result<T, <T as FromStr>::Err> {
        self.fragment().parse::<T>()
    }

    fn extend_with<T: LocatedSpan>(&self, other: &T) -> Self {
        Self {
            span: self.span.extend_with(other),
            ..self.clone()
        }
    }
}
