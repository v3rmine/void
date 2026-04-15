// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Query for a deploy key on a project.
#[derive(Debug, Builder, Clone)]
pub struct DeployKey<'a> {
    /// The project with the merge requset.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The ID of the deploy key.
    deploy_key: u64,
}

impl<'a> DeployKey<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> DeployKeyBuilder<'a> {
        DeployKeyBuilder::default()
    }
}

impl<'a> Endpoint for DeployKey<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/deploy_keys/{}", self.project, self.deploy_key).into()
    }
}

#[cfg(test)]
mod tests {
    use crate::api::projects::deploy_keys::{DeployKey, DeployKeyBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_deploy_key_are_needed() {
        let err = DeployKey::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, DeployKeyBuilderError, "project");
    }

    #[test]
    fn project_is_needed() {
        let err = DeployKey::builder().deploy_key(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, DeployKeyBuilderError, "project");
    }

    #[test]
    fn deploy_key_is_needed() {
        let err = DeployKey::builder().project(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, DeployKeyBuilderError, "deploy_key");
    }

    #[test]
    fn project_and_deploy_key_are_sufficient() {
        DeployKey::builder()
            .project(1)
            .deploy_key(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/deploy_keys/1")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = DeployKey::builder()
            .project("simple/project")
            .deploy_key(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
