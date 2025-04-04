// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::{self, NameOrId};
use crate::api::endpoint_prelude::*;

/// Query for a specific commit in a project.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Commit<'a> {
    /// The project to get a commit from.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The commit to get.
    #[builder(setter(into))]
    commit: Cow<'a, str>,

    /// Include commit stats.
    #[builder(default)]
    stats: Option<bool>,
}

impl<'a> Commit<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> CommitBuilder<'a> {
        CommitBuilder::default()
    }
}

impl<'a> Endpoint for Commit<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/repository/commits/{}",
            self.project,
            common::path_escaped(&self.commit),
        )
        .into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();

        params.push_opt("stats", self.stats);

        params
    }
}

#[cfg(test)]
mod tests {
    use crate::api::projects::repository::commits::{Commit, CommitBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_commit_are_necessary() {
        let err = Commit::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, CommitBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = Commit::builder().commit("master").build().unwrap_err();
        crate::test::assert_missing_field!(err, CommitBuilderError, "project");
    }

    #[test]
    fn commit_is_necessary() {
        let err = Commit::builder().project(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, CommitBuilderError, "commit");
    }

    #[test]
    fn project_and_commit_are_sufficient() {
        Commit::builder()
            .project(1)
            .commit("master")
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder().endpoint("projects/simple%2Fproject/repository/commits/0000000000000000000000000000000000000000").build().unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Commit::builder()
            .project("simple/project")
            .commit("0000000000000000000000000000000000000000")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_stats() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/repository/commits/0000000000000000000000000000000000000000")
            .add_query_params(&[("stats", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Commit::builder()
            .project("simple/project")
            .commit("0000000000000000000000000000000000000000")
            .stats(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
