// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::endpoint_prelude::*;
use crate::api::ParamValue;

/// States of impersonation tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImpersonationTokenState {
    /// All states.
    All,
    /// Filter to return only active tokens.
    Active,
    /// Filter to return only inactive tokens.
    Inactive,
}

impl ImpersonationTokenState {
    /// The scope as a query parameter.
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            ImpersonationTokenState::All => "all",
            ImpersonationTokenState::Active => "active",
            ImpersonationTokenState::Inactive => "inactive",
        }
    }
}

impl ParamValue<'static> for ImpersonationTokenState {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Get impersonation tokens of a user.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct ImpersonationTokens {
    /// The user to list impersonation tokens for.
    user: u64,
    /// Filter based on state.
    #[builder(default)]
    state: Option<ImpersonationTokenState>,
}

impl ImpersonationTokens {
    /// Create a builder for the endpoint.
    pub fn builder() -> ImpersonationTokensBuilder {
        ImpersonationTokensBuilder::default()
    }
}

impl Endpoint for ImpersonationTokens {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("users/{}/impersonation_tokens", self.user).into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();

        params.push_opt("state", self.state);

        params
    }
}

impl Pageable for ImpersonationTokens {}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::users::impersonation_tokens::{
        ImpersonationTokenState, ImpersonationTokens, ImpersonationTokensBuilderError,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn impersonation_token_state_as_str() {
        let items = &[
            (ImpersonationTokenState::All, "all"),
            (ImpersonationTokenState::Active, "active"),
            (ImpersonationTokenState::Inactive, "inactive"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn user_is_necessary() {
        let err = ImpersonationTokens::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, ImpersonationTokensBuilderError, "user");
    }

    #[test]
    fn user_is_sufficient() {
        ImpersonationTokens::builder().user(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::GET)
            .endpoint("users/1/impersonation_tokens")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ImpersonationTokens::builder().user(1).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_state() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::GET)
            .endpoint("users/1/impersonation_tokens")
            .add_query_params(&[("state", "active")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ImpersonationTokens::builder()
            .user(1)
            .state(ImpersonationTokenState::Active)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
