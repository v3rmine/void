// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Query for discussions on an merge request within a project.
#[derive(Debug, Builder, Clone)]
pub struct MergeRequestDiscussions<'a> {
    /// The project to query for the merge request.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The ID of the merge request.
    merge_request: u64,
}

impl<'a> MergeRequestDiscussions<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> MergeRequestDiscussionsBuilder<'a> {
        MergeRequestDiscussionsBuilder::default()
    }
}

impl<'a> Endpoint for MergeRequestDiscussions<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/merge_requests/{}/discussions",
            self.project, self.merge_request,
        )
        .into()
    }
}

impl<'a> Pageable for MergeRequestDiscussions<'a> {}

#[cfg(test)]
mod tests {
    use crate::api::projects::merge_requests::discussions::{
        MergeRequestDiscussions, MergeRequestDiscussionsBuilderError,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_merge_request_are_necessary() {
        let err = MergeRequestDiscussions::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, MergeRequestDiscussionsBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = MergeRequestDiscussions::builder()
            .merge_request(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, MergeRequestDiscussionsBuilderError, "project");
    }

    #[test]
    fn merge_request_is_necessary() {
        let err = MergeRequestDiscussions::builder()
            .project(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(
            err,
            MergeRequestDiscussionsBuilderError,
            "merge_request",
        );
    }

    #[test]
    fn project_and_merge_request_are_sufficient() {
        MergeRequestDiscussions::builder()
            .project(1)
            .merge_request(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests/1/discussions")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequestDiscussions::builder()
            .project("simple/project")
            .merge_request(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
