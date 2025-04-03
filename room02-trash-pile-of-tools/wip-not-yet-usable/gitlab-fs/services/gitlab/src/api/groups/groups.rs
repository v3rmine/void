// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::{BTreeMap, BTreeSet};

use derive_builder::Builder;

use crate::api::common::{AccessLevel, SortOrder};
use crate::api::endpoint_prelude::*;
use crate::api::ParamValue;

/// Keys group results may be ordered by.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GroupOrderBy {
    /// Order by the name of the group.
    #[default]
    Name,
    /// Order by the full path of the group.
    Path,
    /// Order by the group ID.
    Id,
    /// Order by similarity (only relevant for searches).
    Similarity,
}

impl GroupOrderBy {
    fn use_keyset_pagination(self) -> bool {
        self == GroupOrderBy::Name
    }

    /// The ordering as a query parameter.
    fn as_str(self) -> &'static str {
        match self {
            GroupOrderBy::Name => "name",
            GroupOrderBy::Path => "path",
            GroupOrderBy::Id => "id",
            GroupOrderBy::Similarity => "similarity",
        }
    }
}

impl ParamValue<'static> for GroupOrderBy {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Query for groups on an instance.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Groups<'a> {
    /// Search for groups using a query string.
    ///
    /// The search query will be escaped automatically.
    #[builder(setter(into), default)]
    search: Option<Cow<'a, str>>,

    /// Skip groups with the given IDs.
    #[builder(setter(name = "_skip_groups"), default, private)]
    skip_groups: BTreeSet<u64>,
    /// Show all groups with access.
    ///
    /// Note that the default for this endpoint differs based on whether the API caller has
    /// administrator privileges or not.
    #[builder(default)]
    all_available: Option<bool>,
    /// Filter owned by those owned by the API caller.
    #[builder(default)]
    owned: Option<bool>,
    /// Filter groups by those where the API caller has a minimum access level.
    #[builder(default)]
    min_access_level: Option<AccessLevel>,
    /// Only return top-level groups.
    #[builder(default)]
    top_level_only: Option<bool>,

    /// Include project statistics in the results.
    #[builder(default)]
    statistics: Option<bool>,
    /// Include custom attributes in the response.
    #[builder(default)]
    with_custom_attributes: Option<bool>,
    /// Search for projects with a given custom attribute set.
    #[builder(setter(name = "_custom_attributes"), default, private)]
    custom_attributes: BTreeMap<Cow<'a, str>, Cow<'a, str>>,

    /// Order results by a given key.
    #[builder(default)]
    order_by: Option<GroupOrderBy>,
    /// The sort order for return results.
    #[builder(default)]
    sort: Option<SortOrder>,
}

impl<'a> Groups<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> GroupsBuilder<'a> {
        GroupsBuilder::default()
    }
}

impl<'a> GroupsBuilder<'a> {
    /// Skip the given group ID.
    pub fn skip_group(&mut self, group: u64) -> &mut Self {
        self.skip_groups
            .get_or_insert_with(BTreeSet::new)
            .insert(group);
        self
    }

    /// Skip the given group IDs.
    pub fn skip_groups<I>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = u64>,
    {
        self.skip_groups
            .get_or_insert_with(BTreeSet::new)
            .extend(iter);
        self
    }

    /// Add a custom attribute search parameter.
    pub fn custom_attribute<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: Into<Cow<'a, str>>,
        V: Into<Cow<'a, str>>,
    {
        self.custom_attributes
            .get_or_insert_with(BTreeMap::new)
            .insert(key.into(), value.into());
        self
    }

    /// Add multiple custom attribute search parameters.
    pub fn custom_attributes<I, K, V>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = (K, V)>,
        K: Into<Cow<'a, str>>,
        V: Into<Cow<'a, str>>,
    {
        self.custom_attributes
            .get_or_insert_with(BTreeMap::new)
            .extend(iter.map(|(k, v)| (k.into(), v.into())));
        self
    }
}

impl<'a> Endpoint for Groups<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "groups".into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();

        params
            .push_opt("search", self.search.as_ref())
            .extend(
                self.skip_groups
                    .iter()
                    .map(|&value| ("skip_groups[]", value)),
            )
            .push_opt("all_available", self.all_available)
            .push_opt("owned", self.owned)
            .push_opt(
                "min_access_level",
                self.min_access_level.map(|level| level.as_u64()),
            )
            .push_opt("top_level_only", self.top_level_only)
            .push_opt("statistics", self.statistics)
            .push_opt("with_custom_attributes", self.with_custom_attributes)
            .extend(
                self.custom_attributes
                    .iter()
                    .map(|(key, value)| (format!("custom_attributes[{}]", key), value)),
            )
            .push_opt("order_by", self.order_by)
            .push_opt("sort", self.sort);

        params
    }
}

impl<'a> Pageable for Groups<'a> {
    fn use_keyset_pagination(&self) -> bool {
        self.order_by
            .map_or(false, |order_by| order_by.use_keyset_pagination())
            && self.sort.map_or(true, |sort| sort == SortOrder::Ascending)
    }
}

#[cfg(test)]
mod tests {
    use crate::api::common::{AccessLevel, SortOrder};
    use crate::api::groups::{GroupOrderBy, Groups};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn order_by_default() {
        assert_eq!(GroupOrderBy::default(), GroupOrderBy::Name);
    }

    #[test]
    fn order_by_as_str() {
        let items = &[
            (GroupOrderBy::Name, "name"),
            (GroupOrderBy::Path, "path"),
            (GroupOrderBy::Id, "id"),
            (GroupOrderBy::Similarity, "similarity"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn defaults_are_sufficient() {
        Groups::builder().build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder().endpoint("groups").build().unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Groups::builder().build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_search() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups")
            .add_query_params(&[("search", "query")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Groups::builder().search("query").build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_skip_groups() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups")
            .add_query_params(&[("skip_groups[]", "1"), ("skip_groups[]", "2")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Groups::builder()
            .skip_group(1)
            .skip_groups([1, 2].iter().copied())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_all_available() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups")
            .add_query_params(&[("all_available", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Groups::builder().all_available(true).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_owned() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups")
            .add_query_params(&[("owned", "false")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Groups::builder().owned(false).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_min_access_level() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups")
            .add_query_params(&[("min_access_level", "30")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Groups::builder()
            .min_access_level(AccessLevel::Developer)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_top_level_only() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups")
            .add_query_params(&[("top_level_only", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Groups::builder().top_level_only(true).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_statistics() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups")
            .add_query_params(&[("statistics", "false")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Groups::builder().statistics(false).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_with_custom_attributes() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups")
            .add_query_params(&[("with_custom_attributes", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Groups::builder()
            .with_custom_attributes(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_custom_attributes() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups")
            .add_query_params(&[
                ("custom_attributes[key]", "value"),
                ("custom_attributes[key2]", "value"),
                ("custom_attributes[key3]", "value&value"),
            ])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Groups::builder()
            .custom_attribute("key", "value")
            .custom_attributes([("key2", "value"), ("key3", "value&value")].iter().cloned())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_order_by() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups")
            .add_query_params(&[("order_by", "path")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Groups::builder()
            .order_by(GroupOrderBy::Path)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_sort() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups")
            .add_query_params(&[("sort", "asc")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Groups::builder()
            .sort(SortOrder::Ascending)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
