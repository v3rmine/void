use std::{fmt, ops::Deref};

use crate::Span;

impl<'input> PartialEq<&'input str> for Span<'input> {
    fn eq(&self, other: &&'input str) -> bool {
        &self.fragment == other
    }
}

impl<'input> PartialEq<Span<'input>> for &'input str {
    fn eq(&self, other: &Span<'input>) -> bool {
        self == &other.fragment
    }
}

impl<'input> fmt::Display for Span<'input> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.fragment)
    }
}

impl<'input> Deref for Span<'input> {
    type Target = &'input str;

    fn deref(&self) -> &Self::Target {
        &self.fragment
    }
}
