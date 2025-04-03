use crate::{LocatedSpan, Span};

pub trait AsMietteOffsets {
    fn as_miette_offsets(&self) -> (miette::SourceOffset, miette::SourceOffset);
}

impl<'input, T: LocatedSpan> AsMietteOffsets for T
where
    <Self as LocatedSpan>::Content: Into<&'input str>,
{
    fn as_miette_offsets(&self) -> (miette::SourceOffset, miette::SourceOffset) {
        let (col, line) = self.position();
        let start = miette::SourceOffset::from_location(self.origin().into(), line, col);

        let (col, line) = self.position_after(self.len());
        let end = miette::SourceOffset::from_location(self.origin().into(), line, col);

        (start, end)
    }
}

pub trait ToMiette {
    fn to_miette_labeled_span<S: ToString>(&self, label: Option<S>) -> miette::LabeledSpan;
}

impl<T: LocatedSpan> ToMiette for T {
    fn to_miette_labeled_span<S: ToString>(&self, label: Option<S>) -> miette::LabeledSpan {
        miette::LabeledSpan::new(
            label.map(|label| label.to_string()),
            self.offset(),
            self.end_offset(),
        )
    }
}

impl<'input> From<Span<'input>> for miette::SourceSpan {
    fn from(src: Span<'input>) -> Self {
        let (start, end) = src.as_miette_offsets();
        miette::SourceSpan::new(start, end)
    }
}
