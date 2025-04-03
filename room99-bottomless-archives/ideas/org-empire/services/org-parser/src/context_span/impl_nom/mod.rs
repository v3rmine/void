use nom::{
    error::ParseError, AsBytes, Compare, CompareResult, FindSubstring, FindToken, InputIter,
    InputLength, InputTake, InputTakeAtPosition,
};

use crate::{ContextSpan, LocatedSpan};

impl<'input> InputIter for ContextSpan<'input>
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

impl<'input> InputLength for ContextSpan<'input>
where
    Self: LocatedSpan,
    <Self as LocatedSpan>::Content: InputLength,
{
    fn input_len(&self) -> usize {
        self.fragment().input_len()
    }
}

impl<'input> AsBytes for ContextSpan<'input> {
    fn as_bytes(&self) -> &[u8] {
        self.fragment().as_bytes()
    }
}

impl<'input> Compare<&'input str> for ContextSpan<'input>
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

impl<'input> FindSubstring<&'input str> for ContextSpan<'input>
where
    Self: LocatedSpan,
    <Self as LocatedSpan>::Content: FindSubstring<&'input str>,
{
    fn find_substring(&self, substr: &'input str) -> Option<usize> {
        self.fragment().find_substring(substr)
    }
}

impl<'input> FindToken<char> for ContextSpan<'input>
where
    Self: LocatedSpan,
    <Self as LocatedSpan>::Content: FindToken<char>,
{
    fn find_token(&self, token: char) -> bool {
        self.fragment().find_token(token)
    }
}

impl<'input> FindToken<u8> for ContextSpan<'input>
where
    Self: LocatedSpan,
    <Self as LocatedSpan>::Content: FindToken<u8>,
{
    fn find_token(&self, token: u8) -> bool {
        self.fragment().find_token(token)
    }
}

impl<'input> InputTake for ContextSpan<'input>
where
    Self: LocatedSpan,
    <Self as LocatedSpan>::Content: InputTake,
    <Self as LocatedSpan>::Content: Into<&'input str>,
{
    fn take(&self, count: usize) -> Self {
        Self {
            span: self.span.take(count),
            ..self.clone()
        }
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        let (before, after) = self.span.take_split(count);
        (
            Self {
                span: before,
                ..self.clone()
            },
            Self {
                span: after,
                ..self.clone()
            },
        )
    }
}

impl<'input> InputTakeAtPosition for ContextSpan<'input>
where
    Self: LocatedSpan + InputTake + InputLength,
    <Self as LocatedSpan>::Content: InputTakeAtPosition,
{
    type Item = char;

    fn split_at_position<P, E: ParseError<Self>>(&self, predicate: P) -> nom::IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.span.fragment.position(predicate) {
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
        match self.span.fragment.position(predicate) {
            Some(0) => Err(nom::Err::Error(E::from_error_kind(self.clone(), e))),
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
        match self.span.fragment.position(predicate) {
            Some(0) => Err(nom::Err::Error(E::from_error_kind(self.clone(), e))),
            Some(n) => Ok(self.take_split(n)),
            None => {
                if self.span.fragment.input_len() == 0 {
                    Err(nom::Err::Error(E::from_error_kind(self.clone(), e)))
                } else {
                    Ok(self.take_split(self.input_len()))
                }
            }
        }
    }
}
