// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::endpoint_prelude::*;

/// Query a user by ID.
#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct User {
    /// The ID of the user.
    user: u64,

    #[builder(default)]
    /// Provide custom attributes as well.
    with_custom_attributes: Option<bool>,
}

impl User {
    /// Create a builder for the endpoint.
    pub fn builder() -> UserBuilder {
        UserBuilder::default()
    }
}

impl Endpoint for User {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("users/{}", self.user).into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();

        params.push_opt("with_custom_attributes", self.with_custom_attributes);

        params
    }
}

#[cfg(test)]
mod tests {
    use crate::api::users::{User, UserBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn user_is_needed() {
        let err = User::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, UserBuilderError, "user");
    }

    #[test]
    fn user_is_sufficient() {
        User::builder().user(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder().endpoint("users/1").build().unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = User::builder().user(1).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_with_custom_attributes() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("users/1")
            .add_query_params(&[("with_custom_attributes", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = User::builder()
            .user(1)
            .with_custom_attributes(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
