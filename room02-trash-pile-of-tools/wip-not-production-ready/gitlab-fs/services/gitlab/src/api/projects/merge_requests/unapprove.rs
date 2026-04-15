// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those s.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Unapprove a merge request.
#[derive(Debug, Builder, Clone)]
pub struct UnapproveMergeRequest<'a> {
    /// The project with the merge request.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The ID of the merge request.
    merge_request: u64,
}

impl<'a> UnapproveMergeRequest<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> UnapproveMergeRequestBuilder<'a> {
        UnapproveMergeRequestBuilder::default()
    }
}

impl<'a> Endpoint for UnapproveMergeRequest<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/merge_requests/{}/unapprove",
            self.project, self.merge_request,
        )
        .into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let params = FormParams::default();

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::merge_requests::{
        UnapproveMergeRequest, UnapproveMergeRequestBuilderError,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_merge_request_are_needed() {
        let err = UnapproveMergeRequest::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, UnapproveMergeRequestBuilderError, "project");
    }

    #[test]
    fn project_is_needed() {
        let err = UnapproveMergeRequest::builder()
            .merge_request(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, UnapproveMergeRequestBuilderError, "project");
    }

    #[test]
    fn merge_request_is_needed() {
        let err = UnapproveMergeRequest::builder()
            .project(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, UnapproveMergeRequestBuilderError, "merge_request");
    }

    #[test]
    fn project_and_merge_request_are_sufficient() {
        UnapproveMergeRequest::builder()
            .project(1)
            .merge_request(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/merge_requests/1/unapprove")
            .content_type("application/x-www-form-urlencoded")
            .body_str("")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = UnapproveMergeRequest::builder()
            .project("simple/project")
            .merge_request(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
