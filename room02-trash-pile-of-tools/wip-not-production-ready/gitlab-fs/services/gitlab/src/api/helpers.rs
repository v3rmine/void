// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Helper types for use in multiple endpoints.
//!
//! Some endpoints have common fields across various places in the API. This module should be used
//! to store common bits that are needed by multiple endpoints which share a common root "far
//! enough" away from their usage to make `super::` access inconvenient.

use std::borrow::Cow;

use crate::api::common::CommaSeparatedList;
use crate::api::ParamValue;

/// Keys note results may be ordered by.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NoteOrderBy {
    /// Sort by creation date.
    #[default]
    CreatedAt,
    /// Sort by last updated date.
    UpdatedAt,
}

impl NoteOrderBy {
    fn as_str(self) -> &'static str {
        match self {
            NoteOrderBy::CreatedAt => "created_at",
            NoteOrderBy::UpdatedAt => "updated_at",
        }
    }
}

impl ParamValue<'static> for NoteOrderBy {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Labels<'a> {
    Any,
    None,
    AllOf(CommaSeparatedList<Cow<'a, str>>),
}

impl<'a> Labels<'a> {
    fn as_str(&self) -> Cow<'static, str> {
        match self {
            Labels::Any => "Any".into(),
            Labels::None => "None".into(),
            Labels::AllOf(labels) => labels.to_string().into(),
        }
    }
}

impl<'a, 'b: 'a> ParamValue<'static> for &'b Labels<'a> {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str()
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Milestone<'a> {
    None,
    Any,
    Named(Cow<'a, str>),
}

impl<'a> Milestone<'a> {
    fn as_str(&self) -> &str {
        match self {
            Milestone::None => "None",
            Milestone::Any => "Any",
            Milestone::Named(name) => name.as_ref(),
        }
    }
}

impl<'a, 'b: 'a> ParamValue<'a> for &'b Milestone<'a> {
    fn as_value(&self) -> Cow<'a, str> {
        self.as_str().into()
    }
}

#[derive(Debug, Clone)]
pub(crate) enum ReactionEmoji<'a> {
    None,
    Any,
    Emoji(Cow<'a, str>),
}

impl<'a> ReactionEmoji<'a> {
    fn as_str(&self) -> &str {
        match self {
            ReactionEmoji::None => "None",
            ReactionEmoji::Any => "Any",
            ReactionEmoji::Emoji(name) => name.as_ref(),
        }
    }
}

impl<'a, 'b: 'a> ParamValue<'a> for &'b ReactionEmoji<'a> {
    fn as_value(&self) -> Cow<'a, str> {
        self.as_str().into()
    }
}

#[cfg(test)]
mod tests {
    use std::iter;

    use super::{Labels, Milestone, NoteOrderBy, ReactionEmoji};

    #[test]
    fn note_order_by_default() {
        assert_eq!(NoteOrderBy::default(), NoteOrderBy::CreatedAt);
    }

    #[test]
    fn note_order_by_as_str() {
        let items = &[
            (NoteOrderBy::CreatedAt, "created_at"),
            (NoteOrderBy::UpdatedAt, "updated_at"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn labels_as_str() {
        let one_user = iter::once("one".into()).collect();
        let two_users = ["one".into(), "two".into()].iter().cloned().collect();

        let items = &[
            (Labels::Any, "Any"),
            (Labels::None, "None"),
            (Labels::AllOf(one_user), "one"),
            (Labels::AllOf(two_users), "one,two"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn milestone_as_str() {
        let items = &[
            (Milestone::Any, "Any"),
            (Milestone::None, "None"),
            (Milestone::Named("milestone".into()), "milestone"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn reaction_emoji_as_str() {
        let items = &[
            (ReactionEmoji::None, "None"),
            (ReactionEmoji::Any, "Any"),
            (ReactionEmoji::Emoji("emoji".into()), "emoji"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }
}
