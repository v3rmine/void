// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those s.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Approve a merge request.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct ApproveMergeRequest<'a> {
    /// The project with the merge request.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The ID of the merge request.
    merge_request: u64,

    /// Git commit SHA to approve. If set, this must match the head of the branch being approved or
    /// the approval will fail.
    #[builder(setter(into), default)]
    sha: Option<Cow<'a, str>>,
    /// Approver's password (required if `Require user password to approve` is enabled in project
    /// settings). Note: no special encryption is applied to this field. TLS/HTTPS for
    /// on-the-wire encryption is assumed.
    #[builder(setter(into), default)]
    approval_password: Option<Cow<'a, str>>,
}

impl<'a> ApproveMergeRequest<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> ApproveMergeRequestBuilder<'a> {
        ApproveMergeRequestBuilder::default()
    }
}

impl<'a> Endpoint for ApproveMergeRequest<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/merge_requests/{}/approve",
            self.project, self.merge_request,
        )
        .into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params
            .push_opt("sha", self.sha.as_ref())
            .push_opt("approval_password", self.approval_password.as_ref());

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::merge_requests::{
        ApproveMergeRequest, ApproveMergeRequestBuilderError,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_merge_request_are_needed() {
        let err = ApproveMergeRequest::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, ApproveMergeRequestBuilderError, "project");
    }

    #[test]
    fn project_is_needed() {
        let err = ApproveMergeRequest::builder()
            .merge_request(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, ApproveMergeRequestBuilderError, "project");
    }

    #[test]
    fn merge_request_is_needed() {
        let err = ApproveMergeRequest::builder()
            .project(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, ApproveMergeRequestBuilderError, "merge_request");
    }

    #[test]
    fn project_and_merge_request_are_sufficient() {
        ApproveMergeRequest::builder()
            .project(1)
            .merge_request(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/merge_requests/1/approve")
            .content_type("application/x-www-form-urlencoded")
            .body_str("")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ApproveMergeRequest::builder()
            .project("simple/project")
            .merge_request(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_approval_password() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/merge_requests/1/approve")
            .content_type("application/x-www-form-urlencoded")
            .body_str("approval_password=blahblahblah")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ApproveMergeRequest::builder()
            .project("simple/project")
            .merge_request(1)
            .approval_password("blahblahblah")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_sha() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/merge_requests/1/approve")
            .content_type("application/x-www-form-urlencoded")
            .body_str("sha=blahblahblah")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ApproveMergeRequest::builder()
            .project("simple/project")
            .merge_request(1)
            .sha("blahblahblah")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
