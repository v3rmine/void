use located_span::LocatedSpan;
use parser_helpers::end_position_in_str;
use std::str::FromStr;

use crate::Span;

impl<'input> LocatedSpan for Span<'input> {
    type Content = &'input str;

    fn origin(&self) -> Self::Content {
        self.origin
    }

    fn fragment(&self) -> Self::Content {
        self.fragment
    }

    fn offset(&self) -> usize {
        self.offset
    }

    fn end_offset(&self) -> usize {
        self.offset + self.fragment.len()
    }

    /// @TODO test offset
    /// **Does not update col and line**
    fn set_offset(&self, offset: usize) -> Self {
        Self {
            fragment: &self.origin[offset..],
            offset,
            ..*self
        }
    }

    /// @TODO test offset
    /// **Does not update col and line**
    fn set_end_offset(&self, offset: usize) -> Self {
        Self {
            fragment: &self.origin[self.offset..=offset],
            ..*self
        }
    }

    fn len(&self) -> usize {
        self.fragment.len()
    }

    /// @TODO test offset
    /// **Does not update col and line**
    fn set_len(&self, len: usize) -> Self {
        Self {
            fragment: &self.fragment[..len],
            ..*self
        }
    }

    fn is_empty(&self) -> bool {
        self.fragment.is_empty()
    }

    fn position(&self) -> (usize, usize) {
        (self.col, self.line)
    }

    fn position_after(&self, count: usize) -> (usize, usize) {
        end_position_in_str(&self.fragment[..count], Some((self.col, self.line)))
    }

    fn set_position(&self, col: usize, line: usize) -> Self {
        Self { col, line, ..*self }
    }

    fn recompute_position_till_offset(&self) -> Self {
        let (col, line) = end_position_in_str(&self.origin[..self.offset], None);

        Self { col, line, ..*self }
    }

    fn parse<T: FromStr>(&self) -> Result<T, <T as FromStr>::Err> {
        self.fragment().parse::<T>()
    }

    fn extend_with<T: LocatedSpan>(&self, other: &T) -> Self {
        let (col, line) = other.position();
        Self {
            fragment: &self.origin()[self.offset()..other.end_offset()],
            col,
            line,
            ..*self
        }
    }
}
