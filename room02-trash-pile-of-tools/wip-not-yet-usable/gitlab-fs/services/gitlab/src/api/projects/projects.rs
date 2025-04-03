// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::BTreeMap;

use chrono::{DateTime, Utc};
use derive_builder::Builder;

use crate::api::common::{AccessLevel, CommaSeparatedList, SortOrder, VisibilityLevel};
use crate::api::endpoint_prelude::*;
use crate::api::ParamValue;

/// Keys project results may be ordered by.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProjectOrderBy {
    /// Order by the project ID.
    Id,
    /// Order by the name of the project.
    Name,
    /// Order by the full path of the project.
    Path,
    /// Order by the creation date of the project.
    #[default]
    CreatedAt,
    /// Order by the last updated date of the project.
    UpdatedAt,
    /// Order by the last activity date of the project.
    LastActivityAt,
    /// Order by similarity (only relevant for searches).
    Similarity,
    /// Order by repository size.
    RepositorySize,
    /// Order by storage size.
    StorageSize,
    /// Order by packages size.
    PackagesSize,
    /// Order by wiki size.
    WikiSize,
}

impl ProjectOrderBy {
    fn use_keyset_pagination(self) -> bool {
        self == ProjectOrderBy::Id
    }

    /// The ordering as a query parameter.
    fn as_str(self) -> &'static str {
        match self {
            ProjectOrderBy::Id => "id",
            ProjectOrderBy::Name => "name",
            ProjectOrderBy::Path => "path",
            ProjectOrderBy::CreatedAt => "created_at",
            ProjectOrderBy::UpdatedAt => "updated_at",
            ProjectOrderBy::LastActivityAt => "last_activity_at",
            ProjectOrderBy::Similarity => "similarity",
            ProjectOrderBy::RepositorySize => "repository_size",
            ProjectOrderBy::StorageSize => "storage_size",
            ProjectOrderBy::PackagesSize => "packages_size",
            ProjectOrderBy::WikiSize => "wiki_size",
        }
    }
}

impl ParamValue<'static> for ProjectOrderBy {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Query for projects on an instance.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Projects<'a> {
    /// Search for projects using a query string.
    ///
    /// The search query will be escaped automatically.
    #[builder(setter(into), default)]
    search: Option<Cow<'a, str>>,

    /// Filter projects by its archived state.
    #[builder(default)]
    archived: Option<bool>,
    /// Filter projects by its visibility.
    #[builder(default)]
    visibility: Option<VisibilityLevel>,
    /// Search  ancestor namespaces when matching filters.
    ///
    /// Defaults to `false`.
    #[builder(default)]
    search_namespaces: Option<bool>,
    /// Return only simple fields for search results.
    #[builder(default)]
    simple: Option<bool>,
    /// Filter projects by those owned by the API caller.
    #[builder(default)]
    owned: Option<bool>,
    /// Filter projects by those the API caller is a member of.
    #[builder(default)]
    membership: Option<bool>,
    /// Filter projects by those the API caller has starred.
    #[builder(default)]
    starred: Option<bool>,
    /// Include project statistics in the results.
    #[builder(default)]
    statistics: Option<bool>,

    /// Filter projects by whether issues are enabled.
    #[builder(default)]
    with_issues_enabled: Option<bool>,
    /// Filter projects by whether merge requests are enabled.
    #[builder(default)]
    with_merge_requests_enabled: Option<bool>,
    /// Filter projects by programming language.
    #[builder(setter(into), default)]
    with_programming_language: Option<Cow<'a, str>>,
    /// Filter projects by those with a failing wiki checksum.
    #[builder(default)]
    wiki_checksum_failed: Option<bool>,
    /// Filter projects by those with a failing repository checksum.
    #[builder(default)]
    repository_checksum_failed: Option<bool>,
    /// Filter projects by those where the API caller has a minimum access level.
    #[builder(default)]
    min_access_level: Option<AccessLevel>,

    /// Search for projects with a given custom attribute set.
    #[builder(setter(name = "_custom_attributes"), default, private)]
    custom_attributes: BTreeMap<Cow<'a, str>, Cow<'a, str>>,
    /// Search for projects with custom attributes.
    #[builder(default)]
    with_custom_attributes: Option<bool>,
    /// Filter projects based on import status.
    #[builder(default)]
    imported: Option<bool>,
    /// Filter projects to those with all indicated topics.
    #[builder(setter(name = "_topic"), default, private)]
    topic: Option<CommaSeparatedList<Cow<'a, str>>>,
    /// Filter projects to those with the given topic ID.
    #[builder(default)]
    topic_id: Option<u64>,

    /// Filter projects by those with at least this ID.
    #[builder(default)]
    id_after: Option<u64>,
    /// Filter projects by those with at most this ID.
    #[builder(default)]
    id_before: Option<u64>,
    /// Filter projects by those with activity after this date.
    #[builder(default)]
    last_activity_after: Option<DateTime<Utc>>,
    /// Filter projects by those without activity before this date.
    #[builder(default)]
    last_activity_before: Option<DateTime<Utc>>,
    /// Filter projects by which storage backend the repository is on.
    ///
    /// Available to administrators only.
    #[builder(setter(into), default)]
    repository_storage: Option<Cow<'a, str>>,

    /// Order results by a given key.
    #[builder(default)]
    order_by: Option<ProjectOrderBy>,
    /// The sort order for return results.
    #[builder(default)]
    sort: Option<SortOrder>,
}

impl<'a> Projects<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> ProjectsBuilder<'a> {
        ProjectsBuilder::default()
    }
}

impl<'a> ProjectsBuilder<'a> {
    /// Add a custom attribute search parameter.
    pub fn topic<T>(&mut self, topic: T) -> &mut Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.topic
            .get_or_insert(None)
            .get_or_insert_with(CommaSeparatedList::new)
            .push(topic.into());
        self
    }

    /// Add multiple custom attribute search parameters.
    pub fn topics<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.topic
            .get_or_insert(None)
            .get_or_insert_with(CommaSeparatedList::new)
            .extend(iter.map(Into::into));
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

impl<'a> Endpoint for Projects<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "projects".into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();

        params
            .push_opt("search", self.search.as_ref())
            .push_opt("archived", self.archived)
            .push_opt("visibility", self.visibility)
            .push_opt("search_namespaces", self.search_namespaces)
            .push_opt("simple", self.simple)
            .push_opt("owned", self.owned)
            .push_opt("membership", self.membership)
            .push_opt("starred", self.starred)
            .push_opt("statistics", self.statistics)
            .push_opt("with_issues_enabled", self.with_issues_enabled)
            .push_opt(
                "with_merge_requests_enabled",
                self.with_merge_requests_enabled,
            )
            .push_opt(
                "with_programming_language",
                self.with_programming_language.as_ref(),
            )
            .push_opt("wiki_checksum_failed", self.wiki_checksum_failed)
            .push_opt(
                "repository_checksum_failed",
                self.repository_checksum_failed,
            )
            .push_opt(
                "min_access_level",
                self.min_access_level.map(|level| level.as_u64()),
            )
            .push_opt("id_after", self.id_after)
            .push_opt("id_before", self.id_before)
            .push_opt("last_activity_after", self.last_activity_after)
            .push_opt("last_activity_before", self.last_activity_before)
            .push_opt("repository_storage", self.repository_storage.as_ref())
            .extend(
                self.custom_attributes
                    .iter()
                    .map(|(key, value)| (format!("custom_attributes[{}]", key), value)),
            )
            .push_opt("with_custom_attributes", self.with_custom_attributes)
            .push_opt("imported", self.imported)
            .push_opt("topic", self.topic.as_ref())
            .push_opt("topic_id", self.topic_id)
            .push_opt("order_by", self.order_by)
            .push_opt("sort", self.sort);

        params
    }
}

impl<'a> Pageable for Projects<'a> {
    fn use_keyset_pagination(&self) -> bool {
        self.order_by
            .map_or(false, |order_by| order_by.use_keyset_pagination())
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    use crate::api::common::{AccessLevel, SortOrder, VisibilityLevel};
    use crate::api::projects::{ProjectOrderBy, Projects};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn order_by_default() {
        assert_eq!(ProjectOrderBy::default(), ProjectOrderBy::CreatedAt);
    }

    #[test]
    fn order_by_as_str() {
        let items = &[
            (ProjectOrderBy::Id, "id"),
            (ProjectOrderBy::Name, "name"),
            (ProjectOrderBy::Path, "path"),
            (ProjectOrderBy::CreatedAt, "created_at"),
            (ProjectOrderBy::UpdatedAt, "updated_at"),
            (ProjectOrderBy::LastActivityAt, "last_activity_at"),
            (ProjectOrderBy::Similarity, "similarity"),
            (ProjectOrderBy::RepositorySize, "repository_size"),
            (ProjectOrderBy::StorageSize, "storage_size"),
            (ProjectOrderBy::PackagesSize, "packages_size"),
            (ProjectOrderBy::WikiSize, "wiki_size"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn defaults_are_sufficient() {
        Projects::builder().build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder().endpoint("projects").build().unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Projects::builder().build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_search() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects")
            .add_query_params(&[("search", "special/query")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Projects::builder().search("special/query").build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_archived() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects")
            .add_query_params(&[("archived", "false")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Projects::builder().archived(false).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_visibility() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects")
            .add_query_params(&[("visibility", "private")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Projects::builder()
            .visibility(VisibilityLevel::Private)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_search_namespaces() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects")
            .add_query_params(&[("search_namespaces", "false")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Projects::builder()
            .search_namespaces(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_simple() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects")
            .add_query_params(&[("simple", "false")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Projects::builder().simple(false).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_owned() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects")
            .add_query_params(&[("owned", "false")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Projects::builder().owned(false).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_membership() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects")
            .add_query_params(&[("membership", "false")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Projects::builder().membership(false).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_starred() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects")
            .add_query_params(&[("starred", "false")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Projects::builder().starred(false).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_statistics() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects")
            .add_query_params(&[("statistics", "false")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Projects::builder().statistics(false).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_with_issues_enabled() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects")
            .add_query_params(&[("with_issues_enabled", "false")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Projects::builder()
            .with_issues_enabled(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_with_merge_requests_enabled() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects")
            .add_query_params(&[("with_merge_requests_enabled", "false")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Projects::builder()
            .with_merge_requests_enabled(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_with_programming_language() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects")
            .add_query_params(&[("with_programming_language", "rust")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Projects::builder()
            .with_programming_language("rust")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_wiki_checksum_failed() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects")
            .add_query_params(&[("wiki_checksum_failed", "false")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Projects::builder()
            .wiki_checksum_failed(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_repository_checksum_failed() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects")
            .add_query_params(&[("repository_checksum_failed", "false")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Projects::builder()
            .repository_checksum_failed(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_min_access_level() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects")
            .add_query_params(&[("min_access_level", "10")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Projects::builder()
            .min_access_level(AccessLevel::Guest)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_custom_attributes() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects")
            .add_query_params(&[
                ("custom_attributes[key]", "value"),
                ("custom_attributes[key2]", "value"),
                ("custom_attributes[key3]", "value&value"),
            ])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Projects::builder()
            .custom_attribute("key", "value")
            .custom_attributes([("key2", "value"), ("key3", "value&value")].iter().cloned())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_with_custom_attributes() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects")
            .add_query_params(&[("with_custom_attributes", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Projects::builder()
            .with_custom_attributes(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_imported() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects")
            .add_query_params(&[("imported", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Projects::builder().imported(true).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_topic() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects")
            .add_query_params(&[("topic", "a,b,c")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Projects::builder()
            .topic("a")
            .topics(["b", "c"].iter().copied())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_topic_id() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects")
            .add_query_params(&[("topic_id", "4")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Projects::builder().topic_id(4).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_id_before() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects")
            .add_query_params(&[("id_before", "100")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Projects::builder().id_before(100).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_id_after() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects")
            .add_query_params(&[("id_after", "100")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Projects::builder().id_after(100).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_last_activity_before() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects")
            .add_query_params(&[("last_activity_before", "2020-01-01T00:00:00Z")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Projects::builder()
            .last_activity_before(Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_repository_storage() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects")
            .add_query_params(&[("repository_storage", "default")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Projects::builder()
            .repository_storage("default")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_last_activity_after() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects")
            .add_query_params(&[("last_activity_after", "2020-01-01T00:00:00Z")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Projects::builder()
            .last_activity_after(Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_order_by() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects")
            .add_query_params(&[("order_by", "id")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Projects::builder()
            .order_by(ProjectOrderBy::Id)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_sort() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects")
            .add_query_params(&[("sort", "desc")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Projects::builder()
            .sort(SortOrder::Descending)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
