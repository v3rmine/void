// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use chrono::NaiveDate;
use derive_builder::Builder;
use itertools::Itertools;

use crate::api::common::{AccessLevel, CommaSeparatedList, NameOrId};
use crate::api::endpoint_prelude::*;
use crate::api::ParamValue;

/// Tasks users may be assigned upon addition to a project.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectInviteTasksToBeDone {
    /// Request to focus on CI tasks.
    Ci,
    /// Request to focus on coding tasks.
    Code,
    /// Request to focus on issues.
    Issues,
}

impl ProjectInviteTasksToBeDone {
    /// The tasks as a query parameter.
    fn as_str(self) -> &'static str {
        match self {
            ProjectInviteTasksToBeDone::Ci => "ci",
            ProjectInviteTasksToBeDone::Code => "code",
            ProjectInviteTasksToBeDone::Issues => "issues",
        }
    }
}

impl ParamValue<'static> for ProjectInviteTasksToBeDone {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Add a user as a member of a project.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct AddProjectMember<'a> {
    /// The project to add the user to.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The user to add to the project.
    #[builder(setter(name = "_user"), private)]
    user_ids: CommaSeparatedList<u64>,
    /// The access level for the user in the project.
    access_level: AccessLevel,

    /// When the user's access expires.
    #[builder(default)]
    expires_at: Option<NaiveDate>,
    /// The source of the invitation.
    #[builder(setter(into), default)]
    invite_source: Option<Cow<'a, str>>,
    /// Tasks the inviter wants the member to focus on.
    ///
    /// Requires `tasks_project_id`.
    #[builder(setter(name = "_tasks_to_be_done"), default, private)]
    tasks_to_be_done: Vec<ProjectInviteTasksToBeDone>,
    /// The project ID in which to create task issues.
    ///
    /// Requires `tasks_to_be_done`.
    #[builder(default)]
    tasks_project_id: Option<u64>,
}

impl<'a> AddProjectMember<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> AddProjectMemberBuilder<'a> {
        AddProjectMemberBuilder::default()
    }
}

impl<'a> AddProjectMemberBuilder<'a> {
    /// The user to add (by ID).
    pub fn user(&mut self, user: u64) -> &mut Self {
        self.user_ids
            .get_or_insert_with(CommaSeparatedList::new)
            .push(user);
        self
    }

    /// Add a set of users (by ID).
    pub fn users<I>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = u64>,
    {
        self.user_ids
            .get_or_insert_with(CommaSeparatedList::new)
            .extend(iter);
        self
    }

    /// Focus on the given task.
    pub fn task_to_be_done(&mut self, task: ProjectInviteTasksToBeDone) -> &mut Self {
        self.tasks_to_be_done
            .get_or_insert_with(Vec::new)
            .push(task);
        self
    }

    /// Focus on the given tasks.
    pub fn tasks_to_be_done<I>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = ProjectInviteTasksToBeDone>,
    {
        self.tasks_to_be_done
            .get_or_insert_with(Vec::new)
            .extend(iter);
        self
    }
}

impl<'a> Endpoint for AddProjectMember<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/members", self.project).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params
            .push("user_id", &self.user_ids)
            .push("access_level", self.access_level.as_u64())
            .push_opt("expires_at", self.expires_at)
            .push_opt("invite_source", self.invite_source.as_ref())
            .extend(
                self.tasks_to_be_done
                    .iter()
                    .map(|task| task.as_str())
                    .unique()
                    .map(|value| ("tasks_to_be_done[]", value)),
            )
            .push_opt("tasks_project_id", self.tasks_project_id);

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use http::Method;

    use crate::api::common::AccessLevel;
    use crate::api::projects::members::{
        AddProjectMember, AddProjectMemberBuilderError, ProjectInviteTasksToBeDone,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_invite_tasks_as_str() {
        let items = &[
            (ProjectInviteTasksToBeDone::Ci, "ci"),
            (ProjectInviteTasksToBeDone::Code, "code"),
            (ProjectInviteTasksToBeDone::Issues, "issues"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn all_parameters_are_needed() {
        let err = AddProjectMember::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, AddProjectMemberBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = AddProjectMember::builder()
            .user(1)
            .access_level(AccessLevel::Developer)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, AddProjectMemberBuilderError, "project");
    }

    #[test]
    fn user_is_necessary() {
        let err = AddProjectMember::builder()
            .project(1)
            .access_level(AccessLevel::Developer)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, AddProjectMemberBuilderError, "user_ids");
    }

    #[test]
    fn access_level_is_necessary() {
        let err = AddProjectMember::builder()
            .project(1)
            .user(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, AddProjectMemberBuilderError, "access_level");
    }

    #[test]
    fn sufficient_parameters() {
        AddProjectMember::builder()
            .project("project")
            .user(1)
            .access_level(AccessLevel::Developer)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/members")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("user_id=1", "&access_level=30"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = AddProjectMember::builder()
            .project("simple/project")
            .user(1)
            .access_level(AccessLevel::Developer)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_user_multiple() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/members")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "user_id=1%2C2",
                "&access_level=30",
                "&expires_at=2020-01-01",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = AddProjectMember::builder()
            .project("simple/project")
            .user(1)
            .user(2)
            .access_level(AccessLevel::Developer)
            .expires_at(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_expires_at() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/members")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "user_id=1",
                "&access_level=30",
                "&expires_at=2020-01-01",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = AddProjectMember::builder()
            .project("simple/project")
            .user(1)
            .access_level(AccessLevel::Developer)
            .expires_at(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_invite_source() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/members")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "user_id=1",
                "&access_level=30",
                "&invite_source=tuesday",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = AddProjectMember::builder()
            .project("simple/project")
            .user(1)
            .access_level(AccessLevel::Developer)
            .invite_source("tuesday")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_tasks_to_be_done() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/members")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "user_id=1",
                "&access_level=30",
                "&tasks_to_be_done%5B%5D=ci",
                "&tasks_to_be_done%5B%5D=code",
                "&tasks_project_id=1",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = AddProjectMember::builder()
            .project("simple/project")
            .user(1)
            .access_level(AccessLevel::Developer)
            .task_to_be_done(ProjectInviteTasksToBeDone::Ci)
            .tasks_to_be_done([ProjectInviteTasksToBeDone::Code].iter().copied())
            .tasks_project_id(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
