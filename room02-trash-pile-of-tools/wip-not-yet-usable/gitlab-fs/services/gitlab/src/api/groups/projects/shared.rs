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
pub enum SharedGroupProjectsOrderBy {
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
    /// Order by the last activity date of the project.
    LastActivityAt,
}

impl SharedGroupProjectsOrderBy {
    /// The ordering as a query parameter.
    fn as_str(self) -> &'static str {
        match self {
            SharedGroupProjectsOrderBy::Id => "id",
            SharedGroupProjectsOrderBy::Name => "name",
            SharedGroupProjectsOrderBy::Path => "path",
            SharedGroupProjectsOrderBy::CreatedAt => "created_at",
            SharedGroupProjectsOrderBy::UpdatedAt => "updated_at",
            SharedGroupProjectsOrderBy::LastActivityAt => "last_activity_at",
        }
    }
}

impl ParamValue<'static> for SharedGroupProjectsOrderBy {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Query projects of a group.
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct SharedGroupProjects<'a> {
    /// The ID or URL-encoded path of the group owned by the authenticated user.
    #[builder(setter(into))]
    id: NameOrId<'a>,

    /// Limit by archived status.
    #[builder(default)]
    archived: Option<bool>,
    /// Limit by visibility public, internal, or private
    #[builder(default)]
    visibility: Option<VisibilityLevel>,

    /// Return projects ordered by keys.
    #[builder(default)]
    order_by: Option<SharedGroupProjectsOrderBy>,
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
    /// Limit by projects starred by the current user.
    #[builder(default)]
    starred: Option<bool>,
    /// Limit by projects with issues feature enabled.
    #[builder(default)]
    with_issues_enabled: Option<bool>,
    /// Limit by projects with merge requests feature enabled.
    #[builder(default)]
    with_merge_requests_enabled: Option<bool>,
    /// Limit to projects where current user has at least this access level.
    #[builder(default)]
    min_access_level: Option<AccessLevel>,
    /// Include custom attributes in response (admins only).
    #[builder(default)]
    with_custom_attributes: Option<bool>,
}

impl<'a> SharedGroupProjects<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> SharedGroupProjectsBuilder<'a> {
        SharedGroupProjectsBuilder::default()
    }
}

impl<'a> Endpoint for SharedGroupProjects<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("groups/{}/projects/shared", self.id).into()
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
            .push_opt("starred", self.starred)
            .push_opt("with_issues_enabled", self.with_issues_enabled)
            .push_opt(
                "with_merge_requests_enabled",
                self.with_merge_requests_enabled,
            )
            .push_opt(
                "min_access_level",
                self.min_access_level.map(AccessLevel::as_u64),
            )
            .push_opt("with_custom_attributes", self.with_custom_attributes);

        params
    }
}

impl<'a> Pageable for SharedGroupProjects<'a> {}

#[cfg(test)]
mod tests {
    use crate::api::common::{AccessLevel, SortOrder, VisibilityLevel};
    use crate::api::groups::projects::{
        SharedGroupProjects, SharedGroupProjectsBuilderError, SharedGroupProjectsOrderBy,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn order_by_default() {
        assert_eq!(
            SharedGroupProjectsOrderBy::default(),
            SharedGroupProjectsOrderBy::CreatedAt,
        );
    }

    #[test]
    fn order_by_as_str() {
        let items = &[
            (SharedGroupProjectsOrderBy::Id, "id"),
            (SharedGroupProjectsOrderBy::Name, "name"),
            (SharedGroupProjectsOrderBy::Path, "path"),
            (SharedGroupProjectsOrderBy::CreatedAt, "created_at"),
            (SharedGroupProjectsOrderBy::UpdatedAt, "updated_at"),
            (
                SharedGroupProjectsOrderBy::LastActivityAt,
                "last_activity_at",
            ),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn group_is_needed() {
        let err = SharedGroupProjects::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, SharedGroupProjectsBuilderError, "id");
    }

    #[test]
    fn group_is_sufficient() {
        SharedGroupProjects::builder().id(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/group%2Fsubgroup/projects/shared")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = SharedGroupProjects::builder()
            .id("group/subgroup")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_archived() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/group%2Fsubgroup/projects/shared")
            .add_query_params(&[("archived", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = SharedGroupProjects::builder()
            .id("group/subgroup")
            .archived(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_visibility() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/group%2Fsubgroup/projects/shared")
            .add_query_params(&[("visibility", "private")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = SharedGroupProjects::builder()
            .id("group/subgroup")
            .visibility(VisibilityLevel::Private)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_order_by() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/group%2Fsubgroup/projects/shared")
            .add_query_params(&[("order_by", "id")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = SharedGroupProjects::builder()
            .id("group/subgroup")
            .order_by(SharedGroupProjectsOrderBy::Id)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_sort() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/group%2Fsubgroup/projects/shared")
            .add_query_params(&[("sort", "asc")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = SharedGroupProjects::builder()
            .id("group/subgroup")
            .sort(SortOrder::Ascending)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_search() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/group%2Fsubgroup/projects/shared")
            .add_query_params(&[("search", "name")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = SharedGroupProjects::builder()
            .id("group/subgroup")
            .search("name")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_simple() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/group%2Fsubgroup/projects/shared")
            .add_query_params(&[("simple", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = SharedGroupProjects::builder()
            .id("group/subgroup")
            .simple(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_starred() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/group%2Fsubgroup/projects/shared")
            .add_query_params(&[("starred", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = SharedGroupProjects::builder()
            .id("group/subgroup")
            .starred(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_with_issues_enabled() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/group%2Fsubgroup/projects/shared")
            .add_query_params(&[("with_issues_enabled", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = SharedGroupProjects::builder()
            .id("group/subgroup")
            .with_issues_enabled(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_with_merge_requests_enabled() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/group%2Fsubgroup/projects/shared")
            .add_query_params(&[("with_merge_requests_enabled", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = SharedGroupProjects::builder()
            .id("group/subgroup")
            .with_merge_requests_enabled(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_min_access_level() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/group%2Fsubgroup/projects/shared")
            .add_query_params(&[("min_access_level", "30")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = SharedGroupProjects::builder()
            .id("group/subgroup")
            .min_access_level(AccessLevel::Developer)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_with_custom_attributes() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/group%2Fsubgroup/projects/shared")
            .add_query_params(&[("with_custom_attributes", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = SharedGroupProjects::builder()
            .id("group/subgroup")
            .with_custom_attributes(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
