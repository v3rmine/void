// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Get a list of merge requests related to the specified commit.
#[derive(Debug, Builder, Clone)]
pub struct MergeRequests<'a> {
    /// The project to get commits from.
    #[builder(setter(into))]
    project: NameOrId<'a>,

    /// The ref to get commits from.
    ///
    /// The commit SHA.
    #[builder(setter(into))]
    sha: Cow<'a, str>,
}

impl<'a> MergeRequests<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> MergeRequestsBuilder<'a> {
        MergeRequestsBuilder::default()
    }
}

impl<'a> Endpoint for MergeRequests<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/repository/commits/{}/merge_requests",
            self.project, self.sha,
        )
        .into()
    }
}

impl<'a> Pageable for MergeRequests<'a> {}

#[cfg(test)]
mod tests {

    use crate::api::projects::repository::commits::merge_requests::{
        MergeRequests, MergeRequestsBuilderError,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_sha_is_necessary() {
        let err = MergeRequests::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, MergeRequestsBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = MergeRequests::builder().sha("123").build().unwrap_err();
        crate::test::assert_missing_field!(err, MergeRequestsBuilderError, "project");
    }

    #[test]
    fn sha_is_necessary() {
        let err = MergeRequests::builder().project(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, MergeRequestsBuilderError, "sha");
    }

    #[test]
    fn project_is_sufficient() {
        MergeRequests::builder()
            .project(1)
            .sha("123")
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/repository/commits/123/merge_requests")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequests::builder()
            .project("simple/project")
            .sha("123")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
