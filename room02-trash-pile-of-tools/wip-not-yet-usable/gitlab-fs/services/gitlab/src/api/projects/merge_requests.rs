// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Project merge request API endpoints.
//!
//! These endpoints are used for querying projects merge requests.

pub mod approval_rules;
pub mod approval_state;
pub mod approvals;
mod approve;
pub mod awards;
mod changes;
mod commits;
mod create;
pub mod discussions;
mod edit;
mod issues_closed_by;
mod merge;
mod merge_request;
mod merge_requests;
pub mod notes;
pub mod pipelines;
mod rebase;
mod resource_label_events;
mod unapprove;

pub use self::approve::ApproveMergeRequest;
pub use self::approve::ApproveMergeRequestBuilder;
pub use self::approve::ApproveMergeRequestBuilderError;

pub use self::create::CreateMergeRequest;
pub use self::create::CreateMergeRequestBuilder;
pub use self::create::CreateMergeRequestBuilderError;

pub use self::edit::EditMergeRequest;
pub use self::edit::EditMergeRequestBuilder;
pub use self::edit::EditMergeRequestBuilderError;
pub use self::edit::MergeRequestStateEvent;

pub use self::issues_closed_by::IssuesClosedBy;
pub use self::issues_closed_by::IssuesClosedByBuilder;
pub use self::issues_closed_by::IssuesClosedByBuilderError;

pub use self::merge::MergeMergeRequest;
pub use self::merge::MergeMergeRequestBuilder;
pub use self::merge::MergeMergeRequestBuilderError;

pub use self::merge_request::MergeRequest;
pub use self::merge_request::MergeRequestBuilder;
pub use self::merge_request::MergeRequestBuilderError;

pub use self::commits::MergeRequestCommits;
pub use self::commits::MergeRequestCommitsBuilder;
pub use self::commits::MergeRequestCommitsBuilderError;

#[deprecated(note = "use `pipelines::MergeRequestPipelines` instead")]
pub use self::pipelines::MergeRequestPipelines;
#[deprecated(note = "use `pipelines::MergeRequestPipelinesBuilder` instead")]
pub use self::pipelines::MergeRequestPipelinesBuilder;
#[deprecated(note = "use `pipelines::MergeRequestPipelinesBuilderError` instead")]
pub use self::pipelines::MergeRequestPipelinesBuilderError;

pub use self::changes::MergeRequestChanges;
pub use self::changes::MergeRequestChangesBuilder;
pub use self::changes::MergeRequestChangesBuilderError;

pub use self::merge_requests::MergeRequestOrderBy;
pub use self::merge_requests::MergeRequestScope;
#[allow(deprecated)]
pub use self::merge_requests::MergeRequestSearchScope;
pub use self::merge_requests::MergeRequestState;
pub use self::merge_requests::MergeRequestView;
pub use self::merge_requests::MergeRequests;
pub use self::merge_requests::MergeRequestsBuilder;
pub use self::merge_requests::MergeRequestsBuilderError;

pub use self::rebase::RebaseMergeRequest;
pub use self::rebase::RebaseMergeRequestBuilder;
pub use self::rebase::RebaseMergeRequestBuilderError;

pub use self::resource_label_events::MergeRequestResourceLabelEvents;
pub use self::resource_label_events::MergeRequestResourceLabelEventsBuilder;
pub use self::resource_label_events::MergeRequestResourceLabelEventsBuilderError;

pub use self::unapprove::UnapproveMergeRequest;
pub use self::unapprove::UnapproveMergeRequestBuilder;
pub use self::unapprove::UnapproveMergeRequestBuilderError;
