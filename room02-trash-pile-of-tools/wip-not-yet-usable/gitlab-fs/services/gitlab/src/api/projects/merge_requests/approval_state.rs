// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Project merge request approval rules state API endpoints.
//!
//! These endpoints are used for querying project merge request approval rules state.
//!
//! # Example
//!
//! ```rust,no_run
//! use serde::Deserialize;
//! use gitlab::Gitlab;
//! use gitlab::api::{self, Query};
//! use gitlab::api::projects::merge_requests::approval_state::MergeRequestApprovalState;
//! use gitlab::types::UserBasic;
//!
//! // This enum describes approval rule types.
//! #[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
//! enum RuleType {
//!     // The approval rule gets this type when you press "Add approval rule" in settings.
//!     #[serde(rename = "regular")]
//!     Regular,
//!     // This is the default approval rule when you create a new project.
//!     #[serde(rename = "any_approver")]
//!     AnyApprover,
//!     // This is the approval rule found in Gitlab's code, looks like
//!     // it's a dummy value against null dereferencing. Nevertheless, better have it than not.
//!     #[serde(rename = "fallback")]
//!     Fallback,
//! }
//!
//! // Approval rule as returned by Gitlab REST API.
//! // This is only a partial representation of the full approval rule in a response, for the sake of example.
//! #[derive(Debug, Deserialize)]
//! struct ApprovalRule {
//!     name: String,
//!     rule_type: RuleType,
//!     eligible_approvers: Vec<UserBasic>,
//!     users: Vec<UserBasic>,
//!     approvals_required: u32,
//!     contains_hidden_groups: bool,
//!     approved: bool,
//! }
//!
//! // Approval state as returned by Gitlab REST API.
//! // See https://docs.gitlab.com/ee/api/merge_request_approvals.html#get-the-approval-state-of-merge-requests.
//! #[derive(Debug, Deserialize)]
//! struct ApprovalState {
//!     approval_rules_overwritten: bool,
//!     rules: Vec<ApprovalRule>,
//! }
//!
//! // Create the client.
//! let client = Gitlab::new("gitlab.com", "private-token").unwrap();
//! // Create the endpoint for the merge request 34 in project 12.
//! let endpoint = MergeRequestApprovalState::builder()
//!     .project(12)
//!     .merge_request(34)
//!     .build()
//!     .unwrap();
//! // Get the approval rules state for the merge request.
//! let approvals: ApprovalState = endpoint.query(&client).unwrap();
//! ```

mod approval_state;

pub use self::approval_state::MergeRequestApprovalState;
pub use self::approval_state::MergeRequestApprovalStateBuilder;
pub use self::approval_state::MergeRequestApprovalStateBuilderError;
