// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Remove a user from a project.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct RemoveProjectMember<'a> {
    /// The project to remove the user from.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The user to remove from the project.
    user: u64,
    /// Unassign from any issues or merge requests inside a given project.
    #[builder(default)]
    unassign_issuables: Option<bool>,
}

impl<'a> RemoveProjectMember<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> RemoveProjectMemberBuilder<'a> {
        RemoveProjectMemberBuilder::default()
    }
}

impl<'a> Endpoint for RemoveProjectMember<'a> {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/members/{}", self.project, self.user).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params.push_opt("unassign_issuables", self.unassign_issuables);

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::members::{RemoveProjectMember, RemoveProjectMemberBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn all_parameters_are_needed() {
        let err = RemoveProjectMember::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, RemoveProjectMemberBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = RemoveProjectMember::builder().user(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, RemoveProjectMemberBuilderError, "project");
    }

    #[test]
    fn user_is_necessary() {
        let err = RemoveProjectMember::builder()
            .project(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, RemoveProjectMemberBuilderError, "user");
    }

    #[test]
    fn sufficient_parameters() {
        RemoveProjectMember::builder()
            .project("project")
            .user(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::DELETE)
            .endpoint("projects/project%2Fsubproject/members/1")
            .content_type("application/x-www-form-urlencoded")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = RemoveProjectMember::builder()
            .project("project/subproject")
            .user(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_unassign_issuables() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::DELETE)
            .endpoint("projects/project%2Fsubproject/members/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("unassign_issuables=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = RemoveProjectMember::builder()
            .project("project/subproject")
            .user(1)
            .unassign_issuables(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
