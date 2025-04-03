use std::fmt;

use crate::ContextSpan;

impl<'input> PartialEq<&'input str> for ContextSpan<'input> {
    fn eq(&self, other: &&'input str) -> bool {
        &self.span == other
    }
}

impl<'input> PartialEq<ContextSpan<'input>> for &'input str {
    fn eq(&self, other: &ContextSpan<'input>) -> bool {
        self.eq(&other.span)
    }
}

impl<'input> fmt::Display for ContextSpan<'input> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.span.fmt(f)
    }
}
