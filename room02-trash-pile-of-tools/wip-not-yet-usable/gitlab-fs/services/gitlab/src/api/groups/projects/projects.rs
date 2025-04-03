// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::{AccessLevel, NameOrId, SortOrder, VisibilityLevel};
use crate::api::endpoint_prelude::*;
use crate::api::ParamValue;

/// Keys project results may be ordered by.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GroupProjectsOrderBy {
    /// Order by the user ID.
    Id,
    /// Order by the user display name.
    Name,
    /// Order by the path.
    Path,
    /// Order by the creation date of the project.
    #[default]
    CreatedAt,
    /// Order by the last updated date of the project.
    UpdatedAt,
    /// Order by a similarity score based on the search.
    Similarity,
    /// Order by the last activity date of the project.
    LastActivityAt,
}

impl GroupProjectsOrderBy {
    /// The ordering as a query parameter.
    fn as_str(self) -> &'static str {
        match self {
            GroupProjectsOrderBy::Id => "id",
            GroupProjectsOrderBy::Name => "name",
            GroupProjectsOrderBy::Path => "path",
            GroupProjectsOrderBy::CreatedAt => "created_at",
            GroupProjectsOrderBy::UpdatedAt => "updated_at",
            GroupProjectsOrderBy::Similarity => "similarity",
            GroupProjectsOrderBy::LastActivityAt => "last_activity_at",
        }
    }
}

impl ParamValue<'static> for GroupProjectsOrderBy {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Query projects of a group.
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct GroupProjects<'a> {
    /// The group to query for projects.
    #[builder(setter(into))]
    group: NameOrId<'a>,

    /// Limit by archived status.
    #[builder(default)]
    archived: Option<bool>,
    /// Limit by visibility public, internal, or private
    #[builder(default)]
    visibility: Option<VisibilityLevel>,

    /// Return projects ordered by keys.
    #[builder(default)]
    order_by: Option<GroupProjectsOrderBy>,
    /// Return projects sorted in asc or desc order.
    #[builder(default)]
    sort: Option<SortOrder>,
    /// Search for projects using a query string.
    ///
    /// The search query will be escaped automatically.
    #[builder(setter(into), default)]
    search: Option<Cow<'a, str>>,

    /// Return only the ID, URL, name, and path of each project.
    #[builder(default)]
    simple: Option<bool>,
    /// Limit by projects owned by the current user.
    #[builder(default)]
    owned: Option<bool>,
    /// Limit by projects starred by the current user.
    #[builder(default)]
    starred: Option<bool>,
    /// Limit by projects with issues feature enabled.
    #[builder(default)]
    with_issues_enabled: Option<bool>,
    /// Limit by projects with merge requests feature enabled.
    #[builder(default)]
    with_merge_requests_enabled: Option<bool>,
    /// Include projects shared to this group.
    #[builder(default)]
    with_shared: Option<bool>,
    /// Include projects in subgroups of this group.
    #[builder(default)]
    include_subgroups: Option<bool>,
    /// Limit to projects where current user has at least this access level.
    #[builder(default)]
    min_access_level: Option<AccessLevel>,
    /// Include custom attributes in response (admins only).
    #[builder(default)]
    with_custom_attributes: Option<bool>,
    /// Return only projects that have security reports artifacts present in any of their builds.
    #[builder(default)]
    with_security_reports: Option<bool>,
}

impl<'a> GroupProjects<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> GroupProjectsBuilder<'a> {
        GroupProjectsBuilder::default()
    }
}

impl<'a> Endpoint for GroupProjects<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("groups/{}/projects", self.group).into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();

        params
            .push_opt("archived", self.archived)
            .push_opt("visibility", self.visibility)
            .push_opt("order_by", self.order_by)
            .push_opt("sort", self.sort)
            .push_opt("search", self.search.as_ref())
            .push_opt("simple", self.simple)
            .push_opt("owned", self.owned)
            .push_opt("starred", self.starred)
            .push_opt("with_issues_enabled", self.with_issues_enabled)
            .push_opt(
                "with_merge_requests_enabled",
                self.with_merge_requests_enabled,
            )
            .push_opt("with_shared", self.with_shared)
            .push_opt("include_subgroups", self.include_subgroups)
            .push_opt(
                "min_access_level",
                self.min_access_level.map(AccessLevel::as_u64),
            )
            .push_opt("with_custom_attributes", self.with_custom_attributes)
            .push_opt("with_security_reports", self.with_security_reports);

        params
    }
}

impl<'a> Pageable for GroupProjects<'a> {}

#[cfg(test)]
mod tests {
    use crate::api::common::{AccessLevel, SortOrder, VisibilityLevel};
    use crate::api::groups::projects::{
        GroupProjects, GroupProjectsBuilderError, GroupProjectsOrderBy,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn order_by_default() {
        assert_eq!(
            GroupProjectsOrderBy::default(),
            GroupProjectsOrderBy::CreatedAt
        );
    }

    #[test]
    fn order_by_as_str() {
        let items = &[
            (GroupProjectsOrderBy::Id, "id"),
            (GroupProjectsOrderBy::Name, "name"),
            (GroupProjectsOrderBy::Path, "path"),
            (GroupProjectsOrderBy::CreatedAt, "created_at"),
            (GroupProjectsOrderBy::UpdatedAt, "updated_at"),
            (GroupProjectsOrderBy::Similarity, "similarity"),
            (GroupProjectsOrderBy::LastActivityAt, "last_activity_at"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn group_is_needed() {
        let err = GroupProjects::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, GroupProjectsBuilderError, "group");
    }

    #[test]
    fn group_is_sufficient() {
        GroupProjects::builder().group(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/group%2Fsubgroup/projects")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GroupProjects::builder()
            .group("group/subgroup")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_archived() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/group%2Fsubgroup/projects")
            .add_query_params(&[("archived", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GroupProjects::builder()
            .group("group/subgroup")
            .archived(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_visibility() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/group%2Fsubgroup/projects")
            .add_query_params(&[("visibility", "private")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GroupProjects::builder()
            .group("group/subgroup")
            .visibility(VisibilityLevel::Private)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_order_by() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/group%2Fsubgroup/projects")
            .add_query_params(&[("order_by", "id")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GroupProjects::builder()
            .group("group/subgroup")
            .order_by(GroupProjectsOrderBy::Id)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_sort() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/group%2Fsubgroup/projects")
            .add_query_params(&[("sort", "asc")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GroupProjects::builder()
            .group("group/subgroup")
            .sort(SortOrder::Ascending)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_search() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/group%2Fsubgroup/projects")
            .add_query_params(&[("search", "name")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GroupProjects::builder()
            .group("group/subgroup")
            .search("name")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_simple() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/group%2Fsubgroup/projects")
            .add_query_params(&[("simple", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GroupProjects::builder()
            .group("group/subgroup")
            .simple(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_owned() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/group%2Fsubgroup/projects")
            .add_query_params(&[("owned", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GroupProjects::builder()
            .group("group/subgroup")
            .owned(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_starred() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/group%2Fsubgroup/projects")
            .add_query_params(&[("starred", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GroupProjects::builder()
            .group("group/subgroup")
            .starred(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_with_issues_enabled() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/group%2Fsubgroup/projects")
            .add_query_params(&[("with_issues_enabled", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GroupProjects::builder()
            .group("group/subgroup")
            .with_issues_enabled(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_with_merge_requests_enabled() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/group%2Fsubgroup/projects")
            .add_query_params(&[("with_merge_requests_enabled", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GroupProjects::builder()
            .group("group/subgroup")
            .with_merge_requests_enabled(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_with_shared() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/group%2Fsubgroup/projects")
            .add_query_params(&[("with_shared", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GroupProjects::builder()
            .group("group/subgroup")
            .with_shared(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_include_subgroups() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/group%2Fsubgroup/projects")
            .add_query_params(&[("include_subgroups", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GroupProjects::builder()
            .group("group/subgroup")
            .include_subgroups(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_min_access_level() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/group%2Fsubgroup/projects")
            .add_query_params(&[("min_access_level", "30")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GroupProjects::builder()
            .group("group/subgroup")
            .min_access_level(AccessLevel::Developer)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_with_custom_attributes() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/group%2Fsubgroup/projects")
            .add_query_params(&[("with_custom_attributes", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GroupProjects::builder()
            .group("group/subgroup")
            .with_custom_attributes(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_with_security_reports() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/group%2Fsubgroup/projects")
            .add_query_params(&[("with_security_reports", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GroupProjects::builder()
            .group("group/subgroup")
            .with_security_reports(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
