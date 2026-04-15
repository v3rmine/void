// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Query for approvals on a merge request within a project.
#[derive(Debug, Builder, Clone)]
pub struct MergeRequestApprovals<'a> {
    /// The project to query for the merge request.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The ID of the merge request.
    merge_request: u64,
}

impl<'a> MergeRequestApprovals<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> MergeRequestApprovalsBuilder<'a> {
        MergeRequestApprovalsBuilder::default()
    }
}

impl<'a> Endpoint for MergeRequestApprovals<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/merge_requests/{}/approvals",
            self.project, self.merge_request,
        )
        .into()
    }
}

#[cfg(test)]
mod tests {
    use crate::api::projects::merge_requests::approvals::{
        MergeRequestApprovals, MergeRequestApprovalsBuilderError,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_merge_request_are_necessary() {
        let err = MergeRequestApprovals::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, MergeRequestApprovalsBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = MergeRequestApprovals::builder()
            .merge_request(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, MergeRequestApprovalsBuilderError, "project");
    }

    #[test]
    fn merge_request_is_necessary() {
        let err = MergeRequestApprovals::builder()
            .project(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, MergeRequestApprovalsBuilderError, "merge_request");
    }

    #[test]
    fn project_and_merge_request_are_sufficient() {
        MergeRequestApprovals::builder()
            .project(1)
            .merge_request(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests/1/approvals")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequestApprovals::builder()
            .project("simple/project")
            .merge_request(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
