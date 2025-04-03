// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::BTreeSet;

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Query a members of a project.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct ProjectMembers<'a> {
    /// The project to query for membership.
    #[builder(setter(into))]
    project: NameOrId<'a>,

    /// A search string to filter members by.
    #[builder(setter(into), default)]
    query: Option<Cow<'a, str>>,
    /// A search string to filter members by.
    #[builder(setter(name = "_user_ids"), default, private)]
    user_ids: BTreeSet<u64>,
    // Whether to include ancestor users from enclosing Groups in the queried list of members.
    #[builder(private)]
    _include_ancestors: bool,
}

impl<'a> ProjectMembers<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> ProjectMembersBuilder<'a> {
        let mut builder = ProjectMembersBuilder::default();
        builder._include_ancestors(false);
        builder
    }

    /// Create a builder for the endpoint that includes ancestor groups.
    #[deprecated(note = "use `api/projects/members/AllProjectMembers` instead")]
    pub fn all_builder() -> ProjectMembersBuilder<'a> {
        let mut builder = ProjectMembersBuilder::default();
        builder._include_ancestors(true);
        builder
    }
}

impl<'a> ProjectMembersBuilder<'a> {
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

impl<'a> Endpoint for ProjectMembers<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        if self._include_ancestors {
            format!("projects/{}/members/all", self.project).into()
        } else {
            format!("projects/{}/members", self.project).into()
        }
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();

        params
            .push_opt("query", self.query.as_ref())
            .extend(self.user_ids.iter().map(|&value| ("user_ids[]", value)));

        params
    }
}

impl<'a> Pageable for ProjectMembers<'a> {}

#[cfg(test)]
mod tests {
    use crate::api::projects::members::{ProjectMembers, ProjectMembersBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_is_needed() {
        let err = ProjectMembers::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, ProjectMembersBuilderError, "project");

        #[allow(deprecated)]
        let err = ProjectMembers::all_builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, ProjectMembersBuilderError, "project");
    }

    #[test]
    fn project_is_sufficient() {
        ProjectMembers::builder().project(1).build().unwrap();

        #[allow(deprecated)]
        ProjectMembers::all_builder().project(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/members")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectMembers::builder()
            .project("simple/project")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_all() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/members/all")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        #[allow(deprecated)]
        let endpoint = ProjectMembers::all_builder()
            .project("simple/project")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_query() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/members")
            .add_query_params(&[("query", "search")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectMembers::builder()
            .project("simple/project")
            .query("search")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_user_ids() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/members")
            .add_query_params(&[("user_ids[]", "1"), ("user_ids[]", "2")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectMembers::builder()
            .project("simple/project")
            .user_id(1)
            .user_ids([1, 2].iter().copied())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
