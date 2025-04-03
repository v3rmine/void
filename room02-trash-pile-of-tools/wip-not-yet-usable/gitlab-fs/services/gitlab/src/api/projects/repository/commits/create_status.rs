// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::{self, NameOrId};
use crate::api::endpoint_prelude::*;
use crate::api::ParamValue;

/// The state a commit status may have.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommitStatusState {
    /// The status is expected, but has not started yet.
    Pending,
    /// The status is currently running.
    Running,
    /// The status completed successfully.
    Success,
    /// The status completed with an error.
    Failed,
    /// The status was canceled before it completed.
    Canceled,
}

impl CommitStatusState {
    fn as_str(self) -> &'static str {
        match self {
            CommitStatusState::Pending => "pending",
            CommitStatusState::Running => "running",
            CommitStatusState::Success => "success",
            CommitStatusState::Failed => "failed",
            CommitStatusState::Canceled => "canceled",
        }
    }
}

impl ParamValue<'static> for CommitStatusState {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Post a comment on a specific commit in a project.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct CreateCommitStatus<'a> {
    /// The project to get a commit from.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The commit to comment on.
    #[builder(setter(into))]
    commit: Cow<'a, str>,
    /// The state of the commit status.
    #[builder(setter(into))]
    state: CommitStatusState,

    /// The name of the status.
    #[builder(setter(into), default)]
    name: Option<Cow<'a, str>>,
    /// The name of the ref for the commit.
    #[builder(setter(into), default)]
    ref_: Option<Cow<'a, str>>,
    /// The URL to use for more details.
    #[builder(setter(into), default)]
    target_url: Option<Cow<'a, str>>,
    /// A description for the status.
    #[builder(setter(into), default)]
    description: Option<Cow<'a, str>>,
    /// The total code coverage (as a percentage).
    #[builder(default)]
    coverage: Option<f64>,
    /// The ID of the pipeline to use (in case it is ambiguous).
    #[builder(default)]
    pipeline_id: Option<u64>,
}

impl<'a> CreateCommitStatus<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> CreateCommitStatusBuilder<'a> {
        CreateCommitStatusBuilder::default()
    }
}

impl<'a> Endpoint for CreateCommitStatus<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            // XXX(gitlab): This is a really bad name for this endpoint. It's the only one that
            // exists in this namespace.
            // https://gitlab.com/gitlab-org/gitlab/-/issues/217412
            "projects/{}/statuses/{}",
            self.project,
            common::path_escaped(&self.commit),
        )
        .into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params
            .push("state", self.state)
            .push_opt("name", self.name.as_ref())
            .push_opt("ref", self.ref_.as_ref())
            .push_opt("target_url", self.target_url.as_ref())
            .push_opt("description", self.description.as_ref())
            .push_opt("coverage", self.coverage)
            .push_opt("pipeline_id", self.pipeline_id);

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::repository::commits::{
        CommitStatusState, CreateCommitStatus, CreateCommitStatusBuilderError,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn commit_status_state_as_str() {
        let items = &[
            (CommitStatusState::Pending, "pending"),
            (CommitStatusState::Running, "running"),
            (CommitStatusState::Success, "success"),
            (CommitStatusState::Failed, "failed"),
            (CommitStatusState::Canceled, "canceled"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn project_commit_and_state_are_necessary() {
        let err = CreateCommitStatus::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, CreateCommitStatusBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = CreateCommitStatus::builder()
            .commit("master")
            .state(CommitStatusState::Pending)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateCommitStatusBuilderError, "project");
    }

    #[test]
    fn commit_is_necessary() {
        let err = CreateCommitStatus::builder()
            .project(1)
            .state(CommitStatusState::Pending)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateCommitStatusBuilderError, "commit");
    }

    #[test]
    fn state_is_necessary() {
        let err = CreateCommitStatus::builder()
            .project(1)
            .commit("master")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateCommitStatusBuilderError, "state");
    }

    #[test]
    fn project_commit_and_state_are_sufficient() {
        CreateCommitStatus::builder()
            .project(1)
            .commit("master")
            .state(CommitStatusState::Pending)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/statuses/0000000000000000000000000000000000000000")
            .content_type("application/x-www-form-urlencoded")
            .body_str("state=pending")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateCommitStatus::builder()
            .project("simple/project")
            .commit("0000000000000000000000000000000000000000")
            .state(CommitStatusState::Pending)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_name() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/statuses/0000000000000000000000000000000000000000")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("state=pending", "&name=jobname"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateCommitStatus::builder()
            .project("simple/project")
            .commit("0000000000000000000000000000000000000000")
            .state(CommitStatusState::Pending)
            .name("jobname")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_ref() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/statuses/0000000000000000000000000000000000000000")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("state=pending", "&ref=refname"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateCommitStatus::builder()
            .project("simple/project")
            .commit("0000000000000000000000000000000000000000")
            .state(CommitStatusState::Pending)
            .ref_("refname")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_target_url() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/statuses/0000000000000000000000000000000000000000")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "state=pending",
                "&target_url=https%3A%2F%2Ftest.invalid%2Fpath%3Fsome%3Dfoo",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateCommitStatus::builder()
            .project("simple/project")
            .commit("0000000000000000000000000000000000000000")
            .state(CommitStatusState::Pending)
            .target_url("https://test.invalid/path?some=foo")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_description() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/statuses/0000000000000000000000000000000000000000")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("state=pending", "&description=description"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateCommitStatus::builder()
            .project("simple/project")
            .commit("0000000000000000000000000000000000000000")
            .state(CommitStatusState::Pending)
            .description("description")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_coverage() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/statuses/0000000000000000000000000000000000000000")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("state=pending", "&coverage=90"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateCommitStatus::builder()
            .project("simple/project")
            .commit("0000000000000000000000000000000000000000")
            .state(CommitStatusState::Pending)
            .coverage(90.)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_pipeline_id() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/statuses/0000000000000000000000000000000000000000")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("state=pending", "&pipeline_id=1"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateCommitStatus::builder()
            .project("simple/project")
            .commit("0000000000000000000000000000000000000000")
            .state(CommitStatusState::Pending)
            .pipeline_id(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
