// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Query for deploy keys within a project.
///
#[derive(Debug, Builder, Clone)]
pub struct DeployKeys<'a> {
    /// The project to query for deploy keys.
    #[builder(setter(into))]
    project: NameOrId<'a>,
}

impl<'a> DeployKeys<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> DeployKeysBuilder<'a> {
        DeployKeysBuilder::default()
    }
}

impl<'a> Endpoint for DeployKeys<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/deploy_keys", self.project).into()
    }
}

impl<'a> Pageable for DeployKeys<'a> {}

#[cfg(test)]
mod tests {
    use crate::api::projects::deploy_keys::{DeployKeys, DeployKeysBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_is_needed() {
        let err = DeployKeys::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, DeployKeysBuilderError, "project");
    }

    #[test]
    fn project_is_sufficient() {
        DeployKeys::builder().project(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/deploy_keys")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = DeployKeys::builder()
            .project("simple/project")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
