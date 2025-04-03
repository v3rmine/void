// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Query a single member of a project with ancestor collapsing.
#[derive(Debug, Builder, Clone)]
pub struct AllProjectMember<'a> {
    /// The project to query for membership.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The ID of the user.
    user: u64,
}

impl<'a> AllProjectMember<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> AllProjectMemberBuilder<'a> {
        AllProjectMemberBuilder::default()
    }
}

impl<'a> Endpoint for AllProjectMember<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/members/all/{}", self.project, self.user).into()
    }
}

#[cfg(test)]
mod tests {
    use crate::api::projects::members::{AllProjectMember, AllProjectMemberBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_user_are_needed() {
        let err = AllProjectMember::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, AllProjectMemberBuilderError, "project");
    }

    #[test]
    fn project_is_needed() {
        let err = AllProjectMember::builder().user(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, AllProjectMemberBuilderError, "project");
    }

    #[test]
    fn user_is_needed() {
        let err = AllProjectMember::builder().project(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, AllProjectMemberBuilderError, "user");
    }

    #[test]
    fn project_and_user_are_sufficient() {
        AllProjectMember::builder()
            .project(1)
            .user(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/members/all/1")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = AllProjectMember::builder()
            .project("simple/project")
            .user(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
