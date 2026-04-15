// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Project issue API endpoints.
//!
//! These endpoints are used for querying projects issues.

mod create;
mod edit;
mod issue;
mod issues;
mod merge_requests_closing;
pub mod notes;
mod resource_label_events;

pub use self::create::CreateIssue;
pub use self::create::CreateIssueBuilder;
pub use self::create::CreateIssueBuilderError;

pub use self::edit::EditIssue;
pub use self::edit::EditIssueBuilder;
pub use self::edit::EditIssueBuilderError;
pub use self::edit::IssueStateEvent;

pub use self::issue::Issue;
pub use self::issue::IssueBuilder;
pub use self::issue::IssueBuilderError;

pub use self::issues::IssueDueDateFilter;
pub use self::issues::IssueIteration;
pub use self::issues::IssueOrderBy;
pub use self::issues::IssueScope;
pub use self::issues::IssueSearchScope;
pub use self::issues::IssueState;
pub use self::issues::IssueWeight;
pub use self::issues::Issues;
pub use self::issues::IssuesBuilder;
pub use self::issues::IssuesBuilderError;

pub use self::merge_requests_closing::MergeRequestsClosing;
pub use self::merge_requests_closing::MergeRequestsClosingBuilder;
pub use self::merge_requests_closing::MergeRequestsClosingBuilderError;

pub use self::resource_label_events::IssueResourceLabelEvents;
pub use self::resource_label_events::IssueResourceLabelEventsBuilder;
pub use self::resource_label_events::IssueResourceLabelEventsBuilderError;
