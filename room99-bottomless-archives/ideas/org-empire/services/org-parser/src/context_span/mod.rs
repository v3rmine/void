mod impl_context_span;
mod impl_nom;

use crate::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextSpan<'input> {
    pub span: Span<'input>,
    pub contexts: Vec<(String, Span<'input>)>,
}

impl<'input> ContextSpan<'input> {
    pub fn add_context<T: ToString>(&self, context: T) -> Self {
        let mut contexts = self.contexts.clone();
        contexts.push((context.to_string(), self.span));

        Self {
            span: self.span,
            contexts,
        }
    }
}
