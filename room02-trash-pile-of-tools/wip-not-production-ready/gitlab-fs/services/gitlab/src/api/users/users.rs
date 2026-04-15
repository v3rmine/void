// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::BTreeMap;

use chrono::{DateTime, Utc};
use derive_builder::Builder;

use crate::api::common::{EnableState, SortOrder};
use crate::api::endpoint_prelude::*;
use crate::api::ParamValue;

/// Keys user results may be ordered by.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UserOrderBy {
    /// Order by the user ID.
    #[default]
    Id,
    /// Order by the user display name.
    Name,
    /// Order by the username.
    Username,
    /// Order by the creation date of the user.
    CreatedAt,
    /// Order by the last updated date of the project.
    UpdatedAt,
}

impl UserOrderBy {
    /// The ordering as a query parameter.
    fn as_str(self) -> &'static str {
        match self {
            UserOrderBy::Id => "id",
            UserOrderBy::Name => "name",
            UserOrderBy::Username => "username",
            UserOrderBy::CreatedAt => "created_at",
            UserOrderBy::UpdatedAt => "updated_at",
        }
    }
}

impl ParamValue<'static> for UserOrderBy {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Representation of a user provided by an external service.
#[derive(Debug, Clone, PartialEq, Eq, Builder)]
pub struct ExternalProvider<'a> {
    /// The UID of the user on the service.
    #[builder(setter(into))]
    pub uid: Cow<'a, str>,
    /// The name of the service.
    #[builder(setter(into))]
    pub name: Cow<'a, str>,
}

impl<'a> ExternalProvider<'a> {
    /// Create a builder for the external provider.
    pub fn builder() -> ExternalProviderBuilder<'a> {
        ExternalProviderBuilder::default()
    }
}

impl<'a> ExternalProviderBuilder<'a> {
    /// Deprecated compatibility method to set UID.
    #[deprecated(note = "use `uid` instead")]
    pub fn id(&mut self, id: u64) -> &mut ExternalProviderBuilder<'a> {
        self.uid(id.to_string())
    }
}

/// Query for users on an instance.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Users<'a> {
    /// Search for users using a query string.
    ///
    /// The search query will be escaped automatically.
    #[builder(setter(into), default)]
    search: Option<Cow<'a, str>>,

    /// Get a user with a given username.
    #[builder(setter(into), default)]
    username: Option<Cow<'a, str>>,
    /// Return only active users.
    #[builder(default)]
    active: Option<()>,
    /// Return only blocked users.
    #[builder(default)]
    blocked: Option<()>,

    /// Search for a user with a given external provider identity.
    #[builder(default)]
    external_provider: Option<ExternalProvider<'a>>,
    /// Whether to return external users or not.
    #[builder(default)]
    external: Option<()>,

    /// Return users created before a given date.
    #[builder(default)]
    created_before: Option<DateTime<Utc>>,
    /// Return users created after a given date.
    #[builder(default)]
    created_after: Option<DateTime<Utc>>,

    /// Search for users with a given custom attribute set.
    #[builder(setter(name = "_custom_attributes"), default, private)]
    custom_attributes: BTreeMap<Cow<'a, str>, Cow<'a, str>>,
    /// Search for users with custom attributes.
    #[builder(default)]
    with_custom_attributes: Option<bool>,

    /// Order results by a given key.
    #[builder(default)]
    order_by: Option<UserOrderBy>,
    /// The sort order for return results.
    #[builder(default)]
    sort: Option<SortOrder>,
    /// Return users with a two-factor enabled or not.
    #[builder(setter(into), default)]
    two_factor: Option<EnableState>,
    /// If set to `true`, filter out users without any projects.
    #[builder(default)]
    without_projects: Option<bool>,
    /// If set to `true`, filter out bot users for projects.
    #[builder(default)]
    without_project_bots: Option<bool>,
    /// Exclude internal users.
    ///
    /// These are generally Service Desk users or other GitLab-managed users.
    #[builder(default)]
    exclude_internal: Option<bool>,
    /// Filter uses based on administrator status.
    #[builder(default)]
    admins: Option<bool>,
    /// If provided, only return users created by the given SAML provider ID.
    #[builder(default)]
    saml_provider_id: Option<u64>,
}

impl<'a> Users<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> UsersBuilder<'a> {
        UsersBuilder::default()
    }
}

impl<'a> UsersBuilder<'a> {
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

impl<'a> Endpoint for Users<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "users".into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();

        params
            .push_opt("search", self.search.as_ref())
            .push_opt("username", self.username.as_ref())
            .push_opt("active", self.active.map(|()| true))
            .push_opt("blocked", self.blocked.map(|()| true))
            .push_opt("external", self.external.map(|()| true))
            .push_opt("created_before", self.created_before)
            .push_opt("created_after", self.created_after)
            .extend(
                self.custom_attributes
                    .iter()
                    .map(|(key, value)| (format!("custom_attributes[{}]", key), value)),
            )
            .push_opt("with_custom_attributes", self.with_custom_attributes)
            .push_opt("order_by", self.order_by)
            .push_opt("sort", self.sort)
            .push_opt("two_factor", self.two_factor)
            .push_opt("without_projects", self.without_projects)
            .push_opt("without_project_bots", self.without_project_bots)
            .push_opt("exclude_internal", self.exclude_internal)
            .push_opt("admins", self.admins)
            .push_opt("saml_provider_id", self.saml_provider_id);

        if let Some(value) = self.external_provider.as_ref() {
            params
                .push("extern_uid", &value.uid)
                .push("provider", &value.name);
        }

        params
    }
}

impl<'a> Pageable for Users<'a> {}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    use crate::api::common::{EnableState, SortOrder};
    use crate::api::users::{ExternalProvider, ExternalProviderBuilderError, UserOrderBy, Users};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn order_by_default() {
        assert_eq!(UserOrderBy::default(), UserOrderBy::Id);
    }

    #[test]
    fn order_by_as_str() {
        let items = &[
            (UserOrderBy::Id, "id"),
            (UserOrderBy::Name, "name"),
            (UserOrderBy::Username, "username"),
            (UserOrderBy::CreatedAt, "created_at"),
            (UserOrderBy::UpdatedAt, "updated_at"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn external_provider_uid_and_name_are_necessary() {
        let err = ExternalProvider::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, ExternalProviderBuilderError, "uid");
    }

    #[test]
    fn external_provider_uid_is_necessary() {
        let err = ExternalProvider::builder()
            .name("name")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, ExternalProviderBuilderError, "uid");
    }

    #[test]
    fn external_provider_name_is_necessary() {
        let err = ExternalProvider::builder().uid("1").build().unwrap_err();
        crate::test::assert_missing_field!(err, ExternalProviderBuilderError, "name");
    }

    #[test]
    fn external_provider_uid_and_name_are_sufficient() {
        ExternalProvider::builder()
            .uid("1")
            .name("name")
            .build()
            .unwrap();
    }

    #[test]
    fn defaults_are_sufficient() {
        Users::builder().build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder().endpoint("users").build().unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Users::builder().build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_search() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("users")
            .add_query_params(&[("search", "special/query")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Users::builder().search("special/query").build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_username() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("users")
            .add_query_params(&[("username", "user")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Users::builder().username("user").build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_active() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("users")
            .add_query_params(&[("active", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Users::builder().active(()).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_blocked() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("users")
            .add_query_params(&[("blocked", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Users::builder().blocked(()).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_external_provider() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("users")
            .add_query_params(&[("extern_uid", "1"), ("provider", "provider")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Users::builder()
            .external_provider(ExternalProvider {
                uid: "1".into(),
                name: "provider".into(),
            })
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_external() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("users")
            .add_query_params(&[("external", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Users::builder().external(()).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_created_before() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("users")
            .add_query_params(&[("created_before", "2020-01-01T00:00:00Z")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Users::builder()
            .created_before(Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_created_after() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("users")
            .add_query_params(&[("created_after", "2020-01-01T00:00:00Z")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Users::builder()
            .created_after(Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_custom_attributes() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("users")
            .add_query_params(&[
                ("custom_attributes[key]", "value"),
                ("custom_attributes[key2]", "value"),
                ("custom_attributes[key3]", "value&value"),
            ])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Users::builder()
            .custom_attribute("key", "value")
            .custom_attributes([("key2", "value"), ("key3", "value&value")].iter().cloned())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_with_custom_attributes() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("users")
            .add_query_params(&[("with_custom_attributes", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Users::builder()
            .with_custom_attributes(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_order_by() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("users")
            .add_query_params(&[("order_by", "id")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Users::builder().order_by(UserOrderBy::Id).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_sort() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("users")
            .add_query_params(&[("sort", "desc")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Users::builder()
            .sort(SortOrder::Descending)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_two_factor() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("users")
            .add_query_params(&[("two_factor", "disabled")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Users::builder()
            .two_factor(EnableState::Disabled)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_without_projects() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("users")
            .add_query_params(&[("without_projects", "false")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Users::builder().without_projects(false).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_without_project_bots() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("users")
            .add_query_params(&[("without_project_bots", "false")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Users::builder()
            .without_project_bots(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_exclude_internal() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("users")
            .add_query_params(&[("exclude_internal", "false")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Users::builder().exclude_internal(false).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_admins() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("users")
            .add_query_params(&[("admins", "false")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Users::builder().admins(false).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_saml_provider_id() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("users")
            .add_query_params(&[("saml_provider_id", "2")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Users::builder().saml_provider_id(2).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
