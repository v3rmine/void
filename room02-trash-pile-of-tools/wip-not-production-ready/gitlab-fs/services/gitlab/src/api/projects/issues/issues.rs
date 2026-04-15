// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Filters for issue states.
pub type IssueState = crate::api::issues::IssueState;

/// Filter issues by a scope.
pub type IssueScope = crate::api::issues::IssueScope;

/// Filter values for issue iteration values.
pub type IssueIteration<'a> = crate::api::issues::IssueIteration<'a>;

/// Filter issues by weight.
pub type IssueWeight = crate::api::issues::IssueWeight;

/// The scope to apply search query terms to.
pub type IssueSearchScope = crate::api::issues::IssueSearchScope;

/// Filter values for due dates.
pub type IssueDueDateFilter = crate::api::issues::IssueDueDateFilter;

/// Keys issue results may be ordered by.
pub type IssueOrderBy = crate::api::issues::IssueOrderBy;

/// Query for issues within a project.
///
/// TODO: Negation (not) filters are not yet supported.
pub type Issues<'a> = crate::api::issues::ProjectIssues<'a>;
/// Builder for [`Issues`].
pub type IssuesBuilder<'a> = crate::api::issues::ProjectIssuesBuilder<'a>;
/// Errors for issue builders.
pub type IssuesBuilderError = crate::api::issues::ProjectIssuesBuilderError;
