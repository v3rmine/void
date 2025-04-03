// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Project merge request approvals API endpoint.
//!
//! This endpoint is used for querying project merge request approvals.
//! See <https://docs.gitlab.com/ee/api/merge_request_approvals.html#merge-request-level-mr-approvals>
//!
//! # Example
//! ```rust,no_run
//! use serde::{Serialize, Deserialize};
//! use gitlab::*;
//! use gitlab::api::Query;
//! use chrono::{DateTime, Utc};
//!
//! /// A merge request with approvals.
//! #[derive(Serialize, Deserialize, Debug, Clone)]
//! pub struct MergeRequestApprovals {
//!     /// The ID of the merge request.
//!     pub id: MergeRequestId,
//!     /// The user-visible ID of the merge request.
//!     pub iid: MergeRequestInternalId,
//!     /// The ID of the project.
//!     pub project_id: ProjectId,
//!     /// The title of the merge request.
//!     pub title: String,
//!     /// The description of the merge request.
//!     pub description: Option<String>,
//!     /// The state of the merge request.
//!     pub state: MergeRequestState,
//!     /// When the merge request was created.
//!     pub created_at: DateTime<Utc>,
//!     /// When the merge request was last updated.
//!     pub updated_at: DateTime<Utc>,
//!     /// The status of the merge request.
//!     pub merge_status: MergeStatus,
//!     /// The total number of approvals required before the merge request can be merged.
//!     pub approvals_required: u64,
//!     /// The number of remaining approvals required before the merge request can be merged.
//!     pub approvals_left: u64,
//!     /// The users that approved the merge request.
//!     pub approved_by: Vec<UserBasic>,
//! }
//! // Create the client.
//! let client = Gitlab::new("gitlab.com", "private-token").unwrap();
//! // Create the endpoint for the merge request 34 in project 12.
//! let endpoint = api::projects::merge_requests::approvals::MergeRequestApprovals::builder()
//!     .project(12)
//!     .merge_request(34)
//!     .build()
//!     .unwrap();
//! // Get the merge request with approvals.
//! let approvals: MergeRequestApprovals = endpoint.query(&client).unwrap();
//! ```

mod approvals;

pub use self::approvals::MergeRequestApprovals;
pub use self::approvals::MergeRequestApprovalsBuilder;
pub use self::approvals::MergeRequestApprovalsBuilderError;
