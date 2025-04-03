mod impl_miette;
mod impl_nom;
mod impl_span;

pub use impl_miette::*;
pub use impl_nom::*;
pub use impl_span::*;

use crate::nom_helpers::is_newline;

/// <https://github.com/Geal/nom/blob/main/doc/custom_input_types.md>
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Span<'input> {
    pub origin: &'input str,
    pub fragment: &'input str,

    pub offset: usize,
    // Start at 1
    pub col: usize,
    // Start at 1
    pub line: usize,
}

pub fn end_position_in_str(src: &str, previous_position: Option<(usize, usize)>) -> (usize, usize) {
    src.chars().fold(
        previous_position.unwrap_or((1_usize, 1_usize)),
        |(col, line), c| {
            if is_newline(c) {
                (1, line + 1)
            } else {
                (col + 1, line)
            }
        },
    )
}

#[cfg(test)]
mod tests {
    use assay::assay;

    use crate::{LocatedSpan, Span};

    #[assay]
    fn span_till_offset() {
        assert_eq!(
            Span {
                fragment: "parser!",
                line: 1,
                col: 7,
                offset: 6,
                origin: "Hello parser!",
            },
            Span::new("Hello parser!")
                .set_offset(6)
                .recompute_position_till_offset()
        );
    }

    #[assay]
    fn span_set_end_offset() {
        assert_eq!(
            Span {
                fragment: "ob",
                line: 1,
                col: 1,
                offset: 2,
                origin: "foobar",
            },
            Span::new("foobar").set_offset(2).set_end_offset(3)
        );
    }

    #[assay]
    fn span_set_len() {
        assert_eq!(
            Span {
                fragment: "ob",
                line: 1,
                col: 1,
                offset: 2,
                origin: "foobar",
            },
            Span::new("foobar").set_offset(2).set_len(2)
        );
    }
}
