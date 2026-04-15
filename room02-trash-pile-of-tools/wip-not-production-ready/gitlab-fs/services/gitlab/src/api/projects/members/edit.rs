// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use chrono::NaiveDate;
use derive_builder::Builder;

use crate::api::common::{AccessLevel, NameOrId};
use crate::api::endpoint_prelude::*;

/// Edit a member of a project.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct EditProjectMember<'a> {
    /// The project to add the user to.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The user to add to the project.
    user: u64,
    /// The access level for the user in the project.
    access_level: AccessLevel,

    /// When the user's access expires.
    #[builder(default)]
    expires_at: Option<NaiveDate>,
}

impl<'a> EditProjectMember<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> EditProjectMemberBuilder<'a> {
        EditProjectMemberBuilder::default()
    }
}

impl<'a> Endpoint for EditProjectMember<'a> {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/members/{}", self.project, self.user).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params
            .push("user_id", self.user)
            .push("access_level", self.access_level.as_u64())
            .push_opt("expires_at", self.expires_at);

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use http::Method;

    use crate::api::common::AccessLevel;
    use crate::api::projects::members::{EditProjectMember, EditProjectMemberBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn all_parameters_are_needed() {
        let err = EditProjectMember::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, EditProjectMemberBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = EditProjectMember::builder()
            .user(1)
            .access_level(AccessLevel::Developer)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, EditProjectMemberBuilderError, "project");
    }

    #[test]
    fn user_is_necessary() {
        let err = EditProjectMember::builder()
            .project(1)
            .access_level(AccessLevel::Developer)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, EditProjectMemberBuilderError, "user");
    }

    #[test]
    fn access_level_is_necessary() {
        let err = EditProjectMember::builder()
            .project(1)
            .user(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, EditProjectMemberBuilderError, "access_level");
    }

    #[test]
    fn sufficient_parameters() {
        EditProjectMember::builder()
            .project("project")
            .user(1)
            .access_level(AccessLevel::Developer)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/project%2Fsubproject/members/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("user_id=1", "&access_level=30"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProjectMember::builder()
            .project("project/subproject")
            .user(1)
            .access_level(AccessLevel::Developer)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_expires_at() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/project%2Fsubproject/members/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "user_id=1",
                "&access_level=30",
                "&expires_at=2020-01-01",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProjectMember::builder()
            .project("project/subproject")
            .user(1)
            .access_level(AccessLevel::Developer)
            .expires_at(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
