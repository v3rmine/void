// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::endpoint_prelude::*;

/// Get a single impersonation token of a user.
#[derive(Debug, Builder, Clone)]
pub struct ImpersonationToken {
    /// The user to create an impersonation token for.
    user: u64,
    /// The name of the impersonation token.
    token_id: u64,
}

impl ImpersonationToken {
    /// Create a builder for the endpoint.
    pub fn builder() -> ImpersonationTokenBuilder {
        ImpersonationTokenBuilder::default()
    }
}

impl Endpoint for ImpersonationToken {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("users/{}/impersonation_tokens/{}", self.user, self.token_id).into()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::users::impersonation_tokens::{
        ImpersonationToken, ImpersonationTokenBuilderError,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn user_and_token_id_are_necessary() {
        let err = ImpersonationToken::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, ImpersonationTokenBuilderError, "user");
    }

    #[test]
    fn user_is_necessary() {
        let err = ImpersonationToken::builder()
            .token_id(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, ImpersonationTokenBuilderError, "user");
    }

    #[test]
    fn token_id_is_necessary() {
        let err = ImpersonationToken::builder().user(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, ImpersonationTokenBuilderError, "token_id");
    }

    #[test]
    fn user_and_token_id_are_sufficient() {
        ImpersonationToken::builder()
            .user(1)
            .token_id(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::GET)
            .endpoint("users/1/impersonation_tokens/1")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ImpersonationToken::builder()
            .user(1)
            .token_id(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
