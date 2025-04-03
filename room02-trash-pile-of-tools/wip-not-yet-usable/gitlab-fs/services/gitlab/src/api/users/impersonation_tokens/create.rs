// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::HashSet;

use chrono::NaiveDate;
use derive_builder::Builder;

use crate::api::endpoint_prelude::*;
use crate::api::ParamValue;

/// Scopes for impersonation tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImpersonationTokenScope {
    /// Access the API and perform git reads and writes.
    Api,
    /// Access to read the user information.
    ReadUser,
}

impl ImpersonationTokenScope {
    /// The scope as a query parameter.
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            ImpersonationTokenScope::Api => "api",
            ImpersonationTokenScope::ReadUser => "read_user",
        }
    }
}

impl ParamValue<'static> for ImpersonationTokenScope {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Create a new impersonation token for a user.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct CreateImpersonationToken<'a> {
    /// The user to create an impersonation token for.
    user: u64,
    /// The name of the impersonation token.
    #[builder(setter(into))]
    name: Cow<'a, str>,
    /// The scopes to allow the token to access.
    #[builder(setter(name = "_scopes"), private)]
    scopes: HashSet<ImpersonationTokenScope>,

    /// When the token expires.
    #[builder(default)]
    expires_at: Option<NaiveDate>,
}

impl<'a> CreateImpersonationToken<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> CreateImpersonationTokenBuilder<'a> {
        CreateImpersonationTokenBuilder::default()
    }
}

impl<'a> CreateImpersonationTokenBuilder<'a> {
    /// Add a scope for the token.
    pub fn scope(&mut self, scope: ImpersonationTokenScope) -> &mut Self {
        self.scopes.get_or_insert_with(HashSet::new).insert(scope);
        self
    }

    /// Add scopes for the token.
    pub fn scopes<I>(&mut self, scopes: I) -> &mut Self
    where
        I: Iterator<Item = ImpersonationTokenScope>,
    {
        self.scopes.get_or_insert_with(HashSet::new).extend(scopes);
        self
    }
}

impl<'a> Endpoint for CreateImpersonationToken<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("users/{}/impersonation_tokens", self.user).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params
            .push("name", &self.name)
            .push_opt("expires_at", self.expires_at);

        params.extend(self.scopes.iter().map(|&value| ("scopes[]", value)));

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use http::Method;

    use crate::api::users::impersonation_tokens::{
        CreateImpersonationToken, CreateImpersonationTokenBuilderError, ImpersonationTokenScope,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn impersonation_token_scope_as_str() {
        let items = &[
            (ImpersonationTokenScope::Api, "api"),
            (ImpersonationTokenScope::ReadUser, "read_user"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn user_name_and_scopes_are_necessary() {
        let err = CreateImpersonationToken::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, CreateImpersonationTokenBuilderError, "user");
    }

    #[test]
    fn user_is_necessary() {
        let err = CreateImpersonationToken::builder()
            .name("name")
            .scope(ImpersonationTokenScope::Api)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateImpersonationTokenBuilderError, "user");
    }

    #[test]
    fn name_is_necessary() {
        let err = CreateImpersonationToken::builder()
            .user(1)
            .scope(ImpersonationTokenScope::Api)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateImpersonationTokenBuilderError, "name");
    }

    #[test]
    fn scopes_is_necessary() {
        let err = CreateImpersonationToken::builder()
            .user(1)
            .name("name")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateImpersonationTokenBuilderError, "scopes");
    }

    #[test]
    fn user_name_and_scopes_are_sufficient() {
        CreateImpersonationToken::builder()
            .user(1)
            .name("name")
            .scope(ImpersonationTokenScope::ReadUser)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("users/1/impersonation_tokens")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&scopes%5B%5D=api"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateImpersonationToken::builder()
            .user(1)
            .name("name")
            .scopes([ImpersonationTokenScope::Api].iter().cloned())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_expires_at() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("users/1/impersonation_tokens")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "name=name",
                "&expires_at=2022-01-01",
                "&scopes%5B%5D=api",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateImpersonationToken::builder()
            .user(1)
            .name("name")
            .scope(ImpersonationTokenScope::Api)
            .expires_at(NaiveDate::from_ymd_opt(2022, 1, 1).unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
