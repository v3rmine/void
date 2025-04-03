// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Query for approval rules of a merge request.
/// See https://docs.gitlab.com/ee/api/merge_request_approvals.html#get-merge-request-level-rules
#[derive(Debug, Builder, Clone)]
pub struct MergeRequestApprovalRules<'a> {
    /// The project to query for approval rules.
    #[builder(setter(into))]
    project: NameOrId<'a>,

    /// The internal ID of the merge request.
    merge_request: u64,
}

impl<'a> MergeRequestApprovalRules<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> MergeRequestApprovalRulesBuilder<'a> {
        MergeRequestApprovalRulesBuilder::default()
    }
}

impl<'a> Endpoint for MergeRequestApprovalRules<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/merge_requests/{}/approval_rules",
            self.project, self.merge_request,
        )
        .into()
    }
}

impl<'a> Pageable for MergeRequestApprovalRules<'a> {}

#[cfg(test)]
mod tests {
    use crate::api::projects::merge_requests::approval_rules::{
        MergeRequestApprovalRules, MergeRequestApprovalRulesBuilderError,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_merge_request_are_needed() {
        let err = MergeRequestApprovalRules::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, MergeRequestApprovalRulesBuilderError, "project");
    }

    #[test]
    fn project_is_needed() {
        let err = MergeRequestApprovalRules::builder()
            .merge_request(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, MergeRequestApprovalRulesBuilderError, "project");
    }

    #[test]
    fn merge_request_is_needed() {
        let err = MergeRequestApprovalRules::builder()
            .project(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(
            err,
            MergeRequestApprovalRulesBuilderError,
            "merge_request",
        );
    }

    #[test]
    fn project_and_merge_request_are_sufficient() {
        MergeRequestApprovalRules::builder()
            .project(1)
            .merge_request(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests/1/approval_rules")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequestApprovalRules::builder()
            .project("simple/project")
            .merge_request(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
