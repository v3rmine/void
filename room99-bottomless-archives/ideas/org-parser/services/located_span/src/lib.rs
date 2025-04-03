use std::str::FromStr;

pub trait LocatedSpan {
    type Content;

    fn origin(&self) -> Self::Content;
    fn fragment(&self) -> Self::Content;
    fn offset(&self) -> usize;
    fn end_offset(&self) -> usize;
    fn set_offset(&self, offset: usize) -> Self;
    fn set_end_offset(&self, offset: usize) -> Self;
    fn position(&self) -> (usize, usize);
    fn position_after(&self, count: usize) -> (usize, usize);
    fn set_position(&self, col: usize, line: usize) -> Self;
    fn recompute_position_till_offset(&self) -> Self;
    fn len(&self) -> usize;
    fn set_len(&self, len: usize) -> Self;
    fn is_empty(&self) -> bool;
    fn parse<T: FromStr>(&self) -> Result<T, <T as FromStr>::Err>;
    fn extend_with<T: LocatedSpan>(&self, other: &T) -> Self;
}
