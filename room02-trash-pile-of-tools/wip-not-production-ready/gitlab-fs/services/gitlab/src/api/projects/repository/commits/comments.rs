// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::{self, NameOrId};
use crate::api::endpoint_prelude::*;

/// Query for comments on a specific commit in a project.
#[derive(Debug, Builder, Clone)]
pub struct CommitComments<'a> {
    /// The project to get a commit from.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The commit to get comments from.
    #[builder(setter(into))]
    commit: Cow<'a, str>,
}

impl<'a> CommitComments<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> CommitCommentsBuilder<'a> {
        CommitCommentsBuilder::default()
    }
}

impl<'a> Endpoint for CommitComments<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/repository/commits/{}/comments",
            self.project,
            common::path_escaped(&self.commit),
        )
        .into()
    }
}

impl<'a> Pageable for CommitComments<'a> {}

#[cfg(test)]
mod tests {
    use crate::api::projects::repository::commits::{CommitComments, CommitCommentsBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_commit_are_necessary() {
        let err = CommitComments::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, CommitCommentsBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = CommitComments::builder()
            .commit("master")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CommitCommentsBuilderError, "project");
    }

    #[test]
    fn commit_is_necessary() {
        let err = CommitComments::builder().project(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, CommitCommentsBuilderError, "commit");
    }

    #[test]
    fn project_and_commit_are_sufficient() {
        CommitComments::builder()
            .project(1)
            .commit("master")
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder().endpoint("projects/simple%2Fproject/repository/commits/0000000000000000000000000000000000000000/comments").build().unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CommitComments::builder()
            .project("simple/project")
            .commit("0000000000000000000000000000000000000000")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
