// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::endpoint_prelude::*;

/// Query information about the API calling user.
#[derive(Debug, Clone, Copy, Builder)]
pub struct CurrentUser {}

impl CurrentUser {
    /// Create a builder for the endpoint.
    pub fn builder() -> CurrentUserBuilder {
        CurrentUserBuilder::default()
    }
}

impl Endpoint for CurrentUser {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "user".into()
    }
}

#[cfg(test)]
mod tests {
    use crate::api::users::CurrentUser;
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn defaults_are_sufficient() {
        CurrentUser::builder().build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder().endpoint("user").build().unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CurrentUser::builder().build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
