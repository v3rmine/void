// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::BTreeSet;

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;
use crate::api::ParamValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// States that project memberships may be in.
pub enum ProjectMemberState {
    /// Users awaiting acceptance.
    Awaiting,
    /// Users actively members of the project.
    Active,
}

impl ProjectMemberState {
    fn as_str(self) -> &'static str {
        match self {
            ProjectMemberState::Awaiting => "awaiting",
            ProjectMemberState::Active => "active",
        }
    }
}

impl ParamValue<'static> for ProjectMemberState {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Query a members of a project including parent group memberships.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct AllProjectMembers<'a> {
    /// The project to query for membership.
    #[builder(setter(into))]
    project: NameOrId<'a>,

    /// A search string to filter members by.
    #[builder(setter(into), default)]
    query: Option<Cow<'a, str>>,
    /// A search string to filter members by.
    #[builder(setter(name = "_user_ids"), default, private)]
    user_ids: BTreeSet<u64>,
    /// Filter results by member state.
    #[builder(default)]
    state: Option<ProjectMemberState>,
}

impl<'a> AllProjectMembers<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> AllProjectMembersBuilder<'a> {
        AllProjectMembersBuilder::default()
    }
}

impl<'a> AllProjectMembersBuilder<'a> {
    /// Filter results by the given user ID.
    pub fn user_id(&mut self, user_id: u64) -> &mut Self {
        self.user_ids
            .get_or_insert_with(BTreeSet::new)
            .insert(user_id);
        self
    }

    /// Filter results by the given user IDs.
    pub fn user_ids<I>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = u64>,
    {
        self.user_ids.get_or_insert_with(BTreeSet::new).extend(iter);
        self
    }
}

impl<'a> Endpoint for AllProjectMembers<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/members/all", self.project).into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();

        params
            .push_opt("query", self.query.as_ref())
            .extend(self.user_ids.iter().map(|&value| ("user_ids[]", value)))
            .push_opt("state", self.state);

        params
    }
}

impl<'a> Pageable for AllProjectMembers<'a> {}

#[cfg(test)]
mod tests {
    use crate::api::projects::members::{
        AllProjectMembers, AllProjectMembersBuilderError, ProjectMemberState,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_member_state_as_str() {
        let items = &[
            (ProjectMemberState::Awaiting, "awaiting"),
            (ProjectMemberState::Active, "active"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn project_is_needed() {
        let err = AllProjectMembers::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, AllProjectMembersBuilderError, "project");
    }

    #[test]
    fn project_is_sufficient() {
        AllProjectMembers::builder().project(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/members/all")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = AllProjectMembers::builder()
            .project("simple/project")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_query() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/members/all")
            .add_query_params(&[("query", "search")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = AllProjectMembers::builder()
            .project("simple/project")
            .query("search")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_user_ids() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/members/all")
            .add_query_params(&[("user_ids[]", "1"), ("user_ids[]", "2")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = AllProjectMembers::builder()
            .project("simple/project")
            .user_id(1)
            .user_ids([1, 2].iter().copied())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_state() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/members/all")
            .add_query_params(&[("state", "awaiting")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = AllProjectMembers::builder()
            .project("simple/project")
            .state(ProjectMemberState::Awaiting)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
