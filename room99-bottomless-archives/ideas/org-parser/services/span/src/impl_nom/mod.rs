use located_span::LocatedSpan;
use nom::{
    error::ParseError, AsBytes, Compare, CompareResult, FindSubstring, FindToken, InputIter,
    InputLength, InputTake, InputTakeAtPosition,
};

use crate::Span;

impl<'input> InputIter for Span<'input>
where
    Self: LocatedSpan,
    <Self as LocatedSpan>::Content: InputIter,
{
    type Item = <<Self as LocatedSpan>::Content as InputIter>::Item;

    type Iter = <<Self as LocatedSpan>::Content as InputIter>::Iter;

    type IterElem = <<Self as LocatedSpan>::Content as InputIter>::IterElem;

    fn iter_indices(&self) -> Self::Iter {
        self.fragment().iter_indices()
    }

    fn iter_elements(&self) -> Self::IterElem {
        self.fragment().iter_elements()
    }

    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.fragment().position(predicate)
    }

    fn slice_index(&self, count: usize) -> Result<usize, nom::Needed> {
        self.fragment().slice_index(count)
    }
}

impl<'input> InputLength for Span<'input>
where
    Self: LocatedSpan,
    <Self as LocatedSpan>::Content: InputLength,
{
    fn input_len(&self) -> usize {
        self.fragment().input_len()
    }
}

impl<'input> AsBytes for Span<'input> {
    fn as_bytes(&self) -> &'input [u8] {
        self.fragment().as_bytes()
    }
}

impl<'input> Compare<&'input str> for Span<'input>
where
    Self: LocatedSpan,
    <Self as LocatedSpan>::Content: Compare<&'input str>,
{
    fn compare(&self, t: &'input str) -> CompareResult {
        self.fragment().compare(t)
    }

    fn compare_no_case(&self, t: &'input str) -> CompareResult {
        self.fragment().compare_no_case(t)
    }
}

impl<'input> FindSubstring<&'input str> for Span<'input>
where
    Self: LocatedSpan,
    <Self as LocatedSpan>::Content: FindSubstring<&'input str>,
{
    fn find_substring(&self, substr: &'input str) -> Option<usize> {
        self.fragment().find_substring(substr)
    }
}

impl<'input> FindToken<char> for Span<'input>
where
    Self: LocatedSpan,
    <Self as LocatedSpan>::Content: FindToken<char>,
{
    fn find_token(&self, token: char) -> bool {
        self.fragment().find_token(token)
    }
}

impl<'input> FindToken<u8> for Span<'input>
where
    Self: LocatedSpan,
    <Self as LocatedSpan>::Content: FindToken<u8>,
{
    fn find_token(&self, token: u8) -> bool {
        self.fragment().find_token(token)
    }
}

impl<'input> InputTake for Span<'input>
where
    Self: LocatedSpan,
    <Self as LocatedSpan>::Content: InputTake,
    <Self as LocatedSpan>::Content: Into<&'input str>,
{
    fn take(&self, count: usize) -> Self {
        let (col, line) = self.position_after(count);

        Self {
            fragment: self.fragment().take(count).into(),
            offset: self.offset() + count,
            col,
            line,
            ..*self
        }
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        let (before, after) = self.fragment.take_split(count);
        let before_len = before.len();
        let (after_col, after_line) = self.position();

        let x = (
            Self {
                fragment: before,
                ..*self
            },
            Self {
                fragment: after,
                offset: self.offset + before_len,
                col: after_col,
                line: after_line,
                ..*self
            },
        );
        dbg!(&x);
        x
    }
}

impl<'input> InputTakeAtPosition for Span<'input>
where
    Self: LocatedSpan + InputTake + InputLength,
    <Self as LocatedSpan>::Content: InputTakeAtPosition,
{
    type Item = char;

    fn split_at_position<P, E: ParseError<Self>>(&self, predicate: P) -> nom::IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        dbg!("split_at_position");
        match self.fragment.position(predicate) {
            Some(n) => Ok(self.take_split(n)),
            None => Err(nom::Err::Incomplete(nom::Needed::new(1))),
        }
    }

    fn split_at_position1<P, E: ParseError<Self>>(
        &self,
        predicate: P,
        e: nom::error::ErrorKind,
    ) -> nom::IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        dbg!("split_at_position1");
        match self.fragment.position(predicate) {
            Some(0) => Err(nom::Err::Error(E::from_error_kind(*self, e))),
            Some(n) => Ok(self.take_split(n)),
            None => Err(nom::Err::Incomplete(nom::Needed::new(1))),
        }
    }

    fn split_at_position_complete<P, E: ParseError<Self>>(
        &self,
        predicate: P,
    ) -> nom::IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        dbg!("split_at_position_complete");
        match self.split_at_position(predicate) {
            Err(nom::Err::Incomplete(_)) => Ok(self.take_split(self.input_len())),
            res => res,
        }
    }

    fn split_at_position1_complete<P, E: ParseError<Self>>(
        &self,
        predicate: P,
        e: nom::error::ErrorKind,
    ) -> nom::IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        dbg!("split_at_position1");
        match self.fragment.position(predicate) {
            Some(0) => Err(nom::Err::Error(E::from_error_kind(*self, e))),
            Some(n) => Ok(self.take_split(n)),
            None => {
                if self.fragment.input_len() == 0 {
                    Err(nom::Err::Error(E::from_error_kind(*self, e)))
                } else {
                    Ok(self.take_split(self.input_len()))
                }
            }
        }
    }
}
