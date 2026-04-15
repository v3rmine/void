// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::endpoint_prelude::*;

/// Query for deploy keys.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct DeployKeys {
    /// If `true`, list only public deploy keys.
    #[builder(default)]
    public: Option<bool>,
}

impl DeployKeys {
    /// Create a builder for the endpoint.
    pub fn builder() -> DeployKeysBuilder {
        DeployKeysBuilder::default()
    }
}

impl Endpoint for DeployKeys {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "deploy_keys".into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();

        params.push_opt("public", self.public);

        params
    }
}

impl Pageable for DeployKeys {}

#[cfg(test)]
mod tests {
    use crate::api::deploy_keys::DeployKeys;
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("deploy_keys")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = DeployKeys::builder().build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_public() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("deploy_keys")
            .add_query_params(&[("public", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = DeployKeys::builder().public(true).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
