// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Query a single member of a project.
#[derive(Debug, Builder, Clone)]
pub struct ProjectMember<'a> {
    /// The project to query for membership.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The ID of the user.
    user: u64,
    // Whether to include ancestor users from enclosing Groups in the queried list of members.
    #[builder(private)]
    _include_ancestors: bool,
}

impl<'a> ProjectMember<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> ProjectMemberBuilder<'a> {
        ProjectMemberBuilder {
            _include_ancestors: Some(false),
            ..Default::default()
        }
    }

    /// Create an ancester-including builder for the endpoint.
    #[deprecated(note = "use `api/projects/members/AllProjectMember` instead")]
    pub fn all_builder() -> ProjectMemberBuilder<'a> {
        ProjectMemberBuilder {
            _include_ancestors: Some(true),
            ..Default::default()
        }
    }
}

impl<'a> Endpoint for ProjectMember<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        if self._include_ancestors {
            format!("projects/{}/members/all/{}", self.project, self.user).into()
        } else {
            format!("projects/{}/members/{}", self.project, self.user).into()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::api::projects::members::{ProjectMember, ProjectMemberBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_user_are_needed() {
        let err = ProjectMember::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, ProjectMemberBuilderError, "project");

        #[allow(deprecated)]
        let err = ProjectMember::all_builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, ProjectMemberBuilderError, "project");
    }

    #[test]
    fn project_is_needed() {
        let err = ProjectMember::builder().user(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, ProjectMemberBuilderError, "project");

        #[allow(deprecated)]
        let err = ProjectMember::all_builder().user(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, ProjectMemberBuilderError, "project");
    }

    #[test]
    fn user_is_needed() {
        let err = ProjectMember::builder().project(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, ProjectMemberBuilderError, "user");

        #[allow(deprecated)]
        let err = ProjectMember::all_builder().project(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, ProjectMemberBuilderError, "user");
    }

    #[test]
    fn project_and_user_are_sufficient() {
        ProjectMember::builder().project(1).user(1).build().unwrap();
        #[allow(deprecated)]
        ProjectMember::all_builder()
            .project(1)
            .user(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/members/1")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectMember::builder()
            .project("simple/project")
            .user(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_all() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/members/all/1")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        #[allow(deprecated)]
        let endpoint = ProjectMember::all_builder()
            .project("simple/project")
            .user(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
