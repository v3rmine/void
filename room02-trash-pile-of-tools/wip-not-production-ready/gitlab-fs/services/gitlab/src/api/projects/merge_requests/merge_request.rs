// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Query for a merge request on a project.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct MergeRequest<'a> {
    /// The project with the merge request.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The ID of the merge request.
    merge_request: u64,

    /// If true, the title and description will be returned as HTML.
    #[builder(default)]
    render_html: Option<bool>,
    /// Include the number of commits behind the target branch.
    #[builder(default)]
    include_diverged_commits_count: Option<bool>,
    /// Include whether a rebase is in progress or not.
    #[builder(default)]
    include_rebase_in_progress: Option<bool>,
}

impl<'a> MergeRequest<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> MergeRequestBuilder<'a> {
        MergeRequestBuilder::default()
    }
}

impl<'a> Endpoint for MergeRequest<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/merge_requests/{}",
            self.project, self.merge_request,
        )
        .into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();

        params
            .push_opt("render_html", self.render_html)
            .push_opt(
                "include_diverged_commits_count",
                self.include_diverged_commits_count,
            )
            .push_opt(
                "include_rebase_in_progress",
                self.include_rebase_in_progress,
            );

        params
    }
}

#[cfg(test)]
mod tests {
    use crate::api::projects::merge_requests::{MergeRequest, MergeRequestBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_merge_request_are_needed() {
        let err = MergeRequest::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, MergeRequestBuilderError, "project");
    }

    #[test]
    fn project_is_needed() {
        let err = MergeRequest::builder()
            .merge_request(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, MergeRequestBuilderError, "project");
    }

    #[test]
    fn merge_request_is_needed() {
        let err = MergeRequest::builder().project(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, MergeRequestBuilderError, "merge_request");
    }

    #[test]
    fn project_and_merge_request_are_sufficient() {
        MergeRequest::builder()
            .project(1)
            .merge_request(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests/1")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequest::builder()
            .project("simple/project")
            .merge_request(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_render_html() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests/1")
            .add_query_params(&[("render_html", "false")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequest::builder()
            .project("simple/project")
            .merge_request(1)
            .render_html(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_include_diverged_commits_count() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests/1")
            .add_query_params(&[("include_diverged_commits_count", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequest::builder()
            .project("simple/project")
            .merge_request(1)
            .include_diverged_commits_count(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_include_rebase_in_progress() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests/1")
            .add_query_params(&[("include_rebase_in_progress", "false")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequest::builder()
            .project("simple/project")
            .merge_request(1)
            .include_rebase_in_progress(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
