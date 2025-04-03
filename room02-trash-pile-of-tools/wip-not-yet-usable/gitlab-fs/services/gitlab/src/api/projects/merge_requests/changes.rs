// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those s.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Shows information of a merge request including its files and changes.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct MergeRequestChanges<'a> {
    /// The project with the merge request.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The ID of the merge request.
    merge_request: u64,

    /// Retrieve changes diffs via Gitaly
    #[builder(default)]
    access_raw_diffs: Option<bool>,
}

impl<'a> MergeRequestChanges<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> MergeRequestChangesBuilder<'a> {
        MergeRequestChangesBuilder::default()
    }
}

impl<'a> Endpoint for MergeRequestChanges<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/merge_requests/{}/changes",
            self.project, self.merge_request,
        )
        .into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params.push_opt("access_raw_diffs", self.access_raw_diffs);

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::merge_requests::{
        MergeRequestChanges, MergeRequestChangesBuilderError,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_merge_request_are_needed() {
        let err = MergeRequestChanges::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, MergeRequestChangesBuilderError, "project");
    }

    #[test]
    fn project_is_needed() {
        let err = MergeRequestChanges::builder()
            .merge_request(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, MergeRequestChangesBuilderError, "project");
    }

    #[test]
    fn merge_request_is_needed() {
        let err = MergeRequestChanges::builder()
            .project(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, MergeRequestChangesBuilderError, "merge_request");
    }

    #[test]
    fn project_and_merge_request_are_sufficient() {
        MergeRequestChanges::builder()
            .project(1)
            .merge_request(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::GET)
            .endpoint("projects/simple%2Fproject/merge_requests/1/changes")
            .content_type("application/x-www-form-urlencoded")
            .body_str("")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequestChanges::builder()
            .project("simple/project")
            .merge_request(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_access_raw_diffs() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::GET)
            .endpoint("projects/simple%2Fproject/merge_requests/1/changes")
            .content_type("application/x-www-form-urlencoded")
            .body_str("access_raw_diffs=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequestChanges::builder()
            .project("simple/project")
            .merge_request(1)
            .access_raw_diffs(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
