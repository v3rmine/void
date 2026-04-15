// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::HashSet;

use derive_builder::Builder;

use crate::api::common::{AccessLevel, NameOrId, SortOrder};
use crate::api::endpoint_prelude::*;
use crate::api::ParamValue;

/// Keys subgroup results may be ordered by.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GroupSubgroupsOrderBy {
    /// Order by the user ID.
    Id,
    /// Order by the user display name.
    #[default]
    Name,
    /// Order by the path.
    Path,
}

impl GroupSubgroupsOrderBy {
    /// The ordering as a query parameter.
    fn as_str(self) -> &'static str {
        match self {
            GroupSubgroupsOrderBy::Id => "id",
            GroupSubgroupsOrderBy::Name => "name",
            GroupSubgroupsOrderBy::Path => "path",
        }
    }
}

impl ParamValue<'static> for GroupSubgroupsOrderBy {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Query subgroups of a group.
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct GroupSubgroups<'a> {
    /// The group to query for subgroups.
    #[builder(setter(into))]
    group: NameOrId<'a>,

    /// Skip the group IDs passed
    #[builder(setter(name = "_skip_groups"), default, private)]
    skip_groups: HashSet<u64>,

    /// Show all the groups you have access to (defaults to false
    /// for authenticated users, true for admin);
    /// Attributes owned and min_access_level have precedence.
    #[builder(default)]
    all_available: Option<bool>,

    /// Search for subgroup using a query string.
    ///
    /// The search query will be escaped automatically.
    #[builder(setter(into), default)]
    search: Option<Cow<'a, str>>,
    /// Return subgroups ordered by keys.
    #[builder(default)]
    order_by: Option<GroupSubgroupsOrderBy>,
    /// Return subgroups sorted in asc or desc order.
    #[builder(default)]
    sort: Option<SortOrder>,

    /// Include group statistics (admins only).
    #[builder(default)]
    statistics: Option<bool>,
    /// Include custom attributes in response (admins only).
    #[builder(default)]
    with_custom_attributes: Option<bool>,
    /// Limit by subgroups owned by the current user.
    #[builder(default)]
    owned: Option<bool>,
    /// Limit to subgroups where current user has at least this access level.
    #[builder(default)]
    min_access_level: Option<AccessLevel>,
}

impl<'a> GroupSubgroups<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> GroupSubgroupsBuilder<'a> {
        GroupSubgroupsBuilder::default()
    }
}

impl<'a> GroupSubgroupsBuilder<'a> {
    /// Filter results by skipping the given group ID.
    pub fn skip_group(&mut self, group_id: u64) -> &mut Self {
        self.skip_groups
            .get_or_insert_with(HashSet::new)
            .insert(group_id);
        self
    }

    /// Filter results by skipping the given group IDs.
    pub fn skip_groups<I>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = u64>,
    {
        self.skip_groups
            .get_or_insert_with(HashSet::new)
            .extend(iter);
        self
    }
}

impl<'a> Endpoint for GroupSubgroups<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("groups/{}/subgroups", self.group).into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();

        params
            .push_opt("all_available", self.all_available)
            .push_opt("search", self.search.as_ref())
            .push_opt("order_by", self.order_by)
            .push_opt("sort", self.sort)
            .push_opt("statistics", self.statistics)
            .push_opt("with_custom_attributes", self.with_custom_attributes)
            .push_opt("owned", self.owned)
            .push_opt(
                "min_access_level",
                self.min_access_level.map(AccessLevel::as_u64),
            )
            .extend(
                self.skip_groups
                    .iter()
                    .map(|&value| ("skip_groups[]", value)),
            );

        params
    }
}

impl<'a> Pageable for GroupSubgroups<'a> {}

#[cfg(test)]
mod tests {
    use crate::api::common::{AccessLevel, SortOrder};
    use crate::api::groups::subgroups::{
        GroupSubgroups, GroupSubgroupsBuilderError, GroupSubgroupsOrderBy,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn order_by_default() {
        assert_eq!(
            GroupSubgroupsOrderBy::default(),
            GroupSubgroupsOrderBy::Name
        );
    }

    #[test]
    fn order_by_as_str() {
        let items = &[
            (GroupSubgroupsOrderBy::Id, "id"),
            (GroupSubgroupsOrderBy::Name, "name"),
            (GroupSubgroupsOrderBy::Path, "path"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn group_is_needed() {
        let err = GroupSubgroups::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, GroupSubgroupsBuilderError, "group");
    }

    #[test]
    fn group_is_sufficient() {
        GroupSubgroups::builder().group(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/group%2Fsubgroup/subgroups")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GroupSubgroups::builder()
            .group("group/subgroup")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_skip_groups() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/group%2Fsubgroup/subgroups")
            .add_query_params(&[("skip_groups[]", "42")])
            .add_query_params(&[("skip_groups[]", "1337")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GroupSubgroups::builder()
            .group("group/subgroup")
            .skip_groups(vec![42, 1337].into_iter())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_all_available() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/group%2Fsubgroup/subgroups")
            .add_query_params(&[("all_available", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GroupSubgroups::builder()
            .group("group/subgroup")
            .all_available(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_search() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/group%2Fsubgroup/subgroups")
            .add_query_params(&[("search", "name")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GroupSubgroups::builder()
            .group("group/subgroup")
            .search("name")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_order_by() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/group%2Fsubgroup/subgroups")
            .add_query_params(&[("order_by", "id")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GroupSubgroups::builder()
            .group("group/subgroup")
            .order_by(GroupSubgroupsOrderBy::Id)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_sort() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/group%2Fsubgroup/subgroups")
            .add_query_params(&[("sort", "asc")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GroupSubgroups::builder()
            .group("group/subgroup")
            .sort(SortOrder::Ascending)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_statistics() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/group%2Fsubgroup/subgroups")
            .add_query_params(&[("statistics", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GroupSubgroups::builder()
            .group("group/subgroup")
            .statistics(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_with_custom_attributes() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/group%2Fsubgroup/subgroups")
            .add_query_params(&[("with_custom_attributes", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GroupSubgroups::builder()
            .group("group/subgroup")
            .with_custom_attributes(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_owned() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/group%2Fsubgroup/subgroups")
            .add_query_params(&[("owned", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GroupSubgroups::builder()
            .group("group/subgroup")
            .owned(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_min_access_level() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/group%2Fsubgroup/subgroups")
            .add_query_params(&[("min_access_level", "30")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GroupSubgroups::builder()
            .group("group/subgroup")
            .min_access_level(AccessLevel::Developer)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
