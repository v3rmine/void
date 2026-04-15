// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Edit a new deploy key on project.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct EditDeployKey<'a> {
    /// The project to open the deploy key on.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The deploy key to edit.
    deploy_key: u64,
    /// The title of the deploy key
    #[builder(setter(into), default)]
    title: Option<Cow<'a, str>>,
    /// Can this deploy key push to the project repository
    #[builder(default)]
    can_push: Option<bool>,
}

impl<'a> EditDeployKey<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> EditDeployKeyBuilder<'a> {
        EditDeployKeyBuilder::default()
    }
}

impl<'a> Endpoint for EditDeployKey<'a> {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/deploy_keys/{}", self.project, self.deploy_key).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params
            .push_opt("title", self.title.as_ref())
            .push_opt("can_push", self.can_push);

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::deploy_keys::{EditDeployKey, EditDeployKeyBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_deploy_key_are_necessary() {
        let err = EditDeployKey::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, EditDeployKeyBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = EditDeployKey::builder().deploy_key(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, EditDeployKeyBuilderError, "project");
    }

    #[test]
    fn deploy_key_is_necessary() {
        let err = EditDeployKey::builder().project(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, EditDeployKeyBuilderError, "deploy_key");
    }

    #[test]
    fn project_and_deploy_key_are_sufficient() {
        EditDeployKey::builder()
            .project(1)
            .deploy_key(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/deploy_keys/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditDeployKey::builder()
            .project("simple/project")
            .deploy_key(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_title() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/deploy_keys/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("title=title")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditDeployKey::builder()
            .project("simple/project")
            .deploy_key(1)
            .title("title")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_can_push() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/deploy_keys/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("can_push=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditDeployKey::builder()
            .project("simple/project")
            .deploy_key(1)
            .can_push(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
