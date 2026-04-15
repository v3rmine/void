// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::{self, NameOrId};
use crate::api::endpoint_prelude::*;

/// Query for statuses on a specific commit in a project.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct CommitStatuses<'a> {
    /// The project to get a commit from.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The commit to get statuses from.
    #[builder(setter(into))]
    commit: Cow<'a, str>,

    /// The branch or tag to check.
    #[builder(setter(into), default)]
    ref_: Option<Cow<'a, str>>,
    /// Filter by build stage.
    #[builder(setter(into), default)]
    stage: Option<Cow<'a, str>>,
    /// Filter by job name.
    #[builder(setter(into), default)]
    name: Option<Cow<'a, str>>,
    /// Return all statuses, not just the latest.
    #[builder(default)]
    all: Option<bool>,
}

impl<'a> CommitStatuses<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> CommitStatusesBuilder<'a> {
        CommitStatusesBuilder::default()
    }
}

impl<'a> Endpoint for CommitStatuses<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/repository/commits/{}/statuses",
            self.project,
            common::path_escaped(&self.commit),
        )
        .into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();

        params
            .push_opt("ref", self.ref_.as_ref())
            .push_opt("stage", self.stage.as_ref())
            .push_opt("name", self.name.as_ref())
            .push_opt("all", self.all);

        params
    }
}

impl<'a> Pageable for CommitStatuses<'a> {}

#[cfg(test)]
mod tests {
    use crate::api::projects::repository::commits::{CommitStatuses, CommitStatusesBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_commit_are_necessary() {
        let err = CommitStatuses::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, CommitStatusesBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = CommitStatuses::builder()
            .commit("master")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CommitStatusesBuilderError, "project");
    }

    #[test]
    fn commit_is_necessary() {
        let err = CommitStatuses::builder().project(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, CommitStatusesBuilderError, "commit");
    }

    #[test]
    fn project_and_commit_are_sufficient() {
        CommitStatuses::builder()
            .project(1)
            .commit("master")
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/repository/commits/0000000000000000000000000000000000000000/statuses")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CommitStatuses::builder()
            .project("simple/project")
            .commit("0000000000000000000000000000000000000000")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_ref() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/repository/commits/0000000000000000000000000000000000000000/statuses")
            .add_query_params(&[("ref", "master")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CommitStatuses::builder()
            .project("simple/project")
            .commit("0000000000000000000000000000000000000000")
            .ref_("master")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_stage() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/repository/commits/0000000000000000000000000000000000000000/statuses")
            .add_query_params(&[("stage", "stage")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CommitStatuses::builder()
            .project("simple/project")
            .commit("0000000000000000000000000000000000000000")
            .stage("stage")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_name() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/repository/commits/0000000000000000000000000000000000000000/statuses")
            .add_query_params(&[("name", "jobname")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CommitStatuses::builder()
            .project("simple/project")
            .commit("0000000000000000000000000000000000000000")
            .name("jobname")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_all() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/repository/commits/0000000000000000000000000000000000000000/statuses")
            .add_query_params(&[("all", "false")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CommitStatuses::builder()
            .project("simple/project")
            .commit("0000000000000000000000000000000000000000")
            .all(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
