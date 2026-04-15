// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Enable a new deploy key on project.
#[derive(Debug, Builder, Clone)]
pub struct EnableDeployKey<'a> {
    /// The project to enable the deploy key on.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The deploy key to enable.
    deploy_key: u64,
}

impl<'a> EnableDeployKey<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> EnableDeployKeyBuilder<'a> {
        EnableDeployKeyBuilder::default()
    }
}

impl<'a> Endpoint for EnableDeployKey<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/deploy_keys/{}/enable",
            self.project, self.deploy_key,
        )
        .into()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::deploy_keys::{EnableDeployKey, EnableDeployKeyBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_deploy_key_are_necessary() {
        let err = EnableDeployKey::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, EnableDeployKeyBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = EnableDeployKey::builder()
            .deploy_key(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, EnableDeployKeyBuilderError, "project");
    }

    #[test]
    fn deploy_key_is_necessary() {
        let err = EnableDeployKey::builder().project(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, EnableDeployKeyBuilderError, "deploy_key");
    }

    #[test]
    fn project_and_deploy_key_are_sufficient() {
        EnableDeployKey::builder()
            .project(1)
            .deploy_key(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/deploy_keys/1/enable")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EnableDeployKey::builder()
            .project("simple/project")
            .deploy_key(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
