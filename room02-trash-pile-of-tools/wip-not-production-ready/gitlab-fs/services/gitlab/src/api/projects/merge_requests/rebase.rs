// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those s.

//! This module implements the server-side rebasing endpoint, which is only relevant if you've set
//! your repository's merge model to __fast forward__ merging.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Rebase a merge request.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct RebaseMergeRequest<'a> {
    /// The project with the merge request.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The ID of the merge request.
    merge_request: u64,

    /// Skip CI on rebase?
    #[builder(default)]
    skip_ci: Option<bool>,
}

impl<'a> RebaseMergeRequest<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> RebaseMergeRequestBuilder<'a> {
        RebaseMergeRequestBuilder::default()
    }
}

impl<'a> Endpoint for RebaseMergeRequest<'a> {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/merge_requests/{}/rebase",
            self.project, self.merge_request,
        )
        .into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params.push_opt("skip_ci", self.skip_ci);
        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::merge_requests::{
        RebaseMergeRequest, RebaseMergeRequestBuilderError,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_merge_request_are_needed() {
        let err = RebaseMergeRequest::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, RebaseMergeRequestBuilderError, "project");
    }

    #[test]
    fn project_is_needed() {
        let err = RebaseMergeRequest::builder()
            .merge_request(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, RebaseMergeRequestBuilderError, "project");
    }

    #[test]
    fn merge_request_is_needed() {
        let err = RebaseMergeRequest::builder()
            .project(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, RebaseMergeRequestBuilderError, "merge_request");
    }

    #[test]
    fn project_and_merge_request_are_sufficient() {
        RebaseMergeRequest::builder()
            .project(1)
            .merge_request(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/merge_requests/1/rebase")
            .content_type("application/x-www-form-urlencoded")
            .body_str("")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = RebaseMergeRequest::builder()
            .project("simple/project")
            .merge_request(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_skip_ci() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/merge_requests/1/rebase")
            .content_type("application/x-www-form-urlencoded")
            .body_str("skip_ci=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = RebaseMergeRequest::builder()
            .project("simple/project")
            .merge_request(1)
            .skip_ci(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
