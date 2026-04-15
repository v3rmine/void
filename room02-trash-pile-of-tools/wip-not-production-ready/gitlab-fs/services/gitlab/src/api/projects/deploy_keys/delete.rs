// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Delete a deploy key from a project.
#[derive(Debug, Builder, Clone)]
pub struct DeleteDeployKey<'a> {
    /// The project to delete the deploy key from.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The deploy key to delete.
    deploy_key: u64,
}

impl<'a> DeleteDeployKey<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> DeleteDeployKeyBuilder<'a> {
        DeleteDeployKeyBuilder::default()
    }
}

impl<'a> Endpoint for DeleteDeployKey<'a> {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/deploy_keys/{}", self.project, self.deploy_key).into()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::deploy_keys::{DeleteDeployKey, DeleteDeployKeyBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_deploy_key_are_necessary() {
        let err = DeleteDeployKey::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, DeleteDeployKeyBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = DeleteDeployKey::builder()
            .deploy_key(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, DeleteDeployKeyBuilderError, "project");
    }

    #[test]
    fn deploy_key_is_necessary() {
        let err = DeleteDeployKey::builder().project(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, DeleteDeployKeyBuilderError, "deploy_key");
    }

    #[test]
    fn project_and_deploy_key_are_sufficient() {
        DeleteDeployKey::builder()
            .project(1)
            .deploy_key(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::DELETE)
            .endpoint("projects/simple%2Fproject/deploy_keys/1")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = DeleteDeployKey::builder()
            .project("simple/project")
            .deploy_key(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
