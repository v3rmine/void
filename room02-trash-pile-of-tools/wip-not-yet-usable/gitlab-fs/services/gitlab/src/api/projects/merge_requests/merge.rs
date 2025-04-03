// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Merge a merge request.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct MergeMergeRequest<'a> {
    /// The project with the merge request.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The ID of the merge request.
    merge_request: u64,

    /// Commit message to use on the merge commit.
    #[builder(setter(into), default)]
    merge_commit_message: Option<Cow<'a, str>>,
    /// Commit message to use on the squash commit.
    #[builder(setter(into), default)]
    squash_commit_message: Option<Cow<'a, str>>,
    /// Squash source branch commits into a single merge commit?
    #[builder(default)]
    squash: Option<bool>,
    /// Remove source branch on successful merge?
    #[builder(default)]
    should_remove_source_branch: Option<bool>,
    /// Merge when pipeline succeeds?
    #[builder(default)]
    merge_when_pipeline_succeeds: Option<bool>,
    /// Git commit SHA to merge. If set, this must match the head of the branch being merged or the
    /// merge will fail.
    #[builder(setter(into), default)]
    sha: Option<Cow<'a, str>>,
}

impl<'a> MergeMergeRequest<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> MergeMergeRequestBuilder<'a> {
        MergeMergeRequestBuilder::default()
    }
}

impl<'a> Endpoint for MergeMergeRequest<'a> {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/merge_requests/{}/merge",
            self.project, self.merge_request,
        )
        .into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params
            .push_opt("merge_commit_message", self.merge_commit_message.as_ref())
            .push_opt("squash_commit_message", self.squash_commit_message.as_ref())
            .push_opt("squash", self.squash)
            .push_opt(
                "should_remove_source_branch",
                self.should_remove_source_branch,
            )
            .push_opt(
                "merge_when_pipeline_succeeds",
                self.merge_when_pipeline_succeeds,
            )
            .push_opt("sha", self.sha.as_ref());

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::merge_requests::{MergeMergeRequest, MergeMergeRequestBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_merge_request_are_needed() {
        let err = MergeMergeRequest::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, MergeMergeRequestBuilderError, "project");
    }

    #[test]
    fn project_is_needed() {
        let err = MergeMergeRequest::builder()
            .merge_request(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, MergeMergeRequestBuilderError, "project");
    }

    #[test]
    fn merge_request_is_needed() {
        let err = MergeMergeRequest::builder().project(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, MergeMergeRequestBuilderError, "merge_request");
    }

    #[test]
    fn project_and_merge_request_are_sufficient() {
        MergeMergeRequest::builder()
            .project(1)
            .merge_request(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/merge_requests/1/merge")
            .content_type("application/x-www-form-urlencoded")
            .body_str("")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeMergeRequest::builder()
            .project("simple/project")
            .merge_request(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_merge_commit_message() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/merge_requests/1/merge")
            .content_type("application/x-www-form-urlencoded")
            .body_str("merge_commit_message=blahblahblah")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeMergeRequest::builder()
            .project("simple/project")
            .merge_request(1)
            .merge_commit_message("blahblahblah")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_squash_commit_message() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/merge_requests/1/merge")
            .content_type("application/x-www-form-urlencoded")
            .body_str("squash_commit_message=blahblahblah")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeMergeRequest::builder()
            .project("simple/project")
            .merge_request(1)
            .squash_commit_message("blahblahblah")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_merge_when_pipeline_succeeds() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/merge_requests/1/merge")
            .content_type("application/x-www-form-urlencoded")
            .body_str("merge_when_pipeline_succeeds=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeMergeRequest::builder()
            .project("simple/project")
            .merge_request(1)
            .merge_when_pipeline_succeeds(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_should_remove_source_branch() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/merge_requests/1/merge")
            .content_type("application/x-www-form-urlencoded")
            .body_str("should_remove_source_branch=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeMergeRequest::builder()
            .project("simple/project")
            .merge_request(1)
            .should_remove_source_branch(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_squash() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/merge_requests/1/merge")
            .content_type("application/x-www-form-urlencoded")
            .body_str("squash=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeMergeRequest::builder()
            .project("simple/project")
            .merge_request(1)
            .squash(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_sha() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/merge_requests/1/merge")
            .content_type("application/x-www-form-urlencoded")
            .body_str("sha=blahblahblah")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeMergeRequest::builder()
            .project("simple/project")
            .merge_request(1)
            .sha("blahblahblah")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
