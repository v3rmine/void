// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Create a new deploy key on project.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct CreateDeployKey<'a> {
    /// The project to add the deploy key to
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The title of the deploy key
    #[builder(setter(into))]
    title: Cow<'a, str>,
    /// The key value as found in an openssh public key file or authorized_key file
    #[builder(setter(into))]
    key: Cow<'a, str>,
    /// Can this deploy key push to the project repository
    #[builder(default)]
    can_push: Option<bool>,
}

impl<'a> CreateDeployKey<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> CreateDeployKeyBuilder<'a> {
        CreateDeployKeyBuilder::default()
    }
}

impl<'a> Endpoint for CreateDeployKey<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/deploy_keys", self.project).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params
            .push("title", self.title.as_ref())
            .push("key", self.key.as_ref())
            .push_opt("can_push", self.can_push);

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::deploy_keys::{CreateDeployKey, CreateDeployKeyBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_source_branch_target_branch_and_title_are_necessary() {
        let err = CreateDeployKey::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, CreateDeployKeyBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = CreateDeployKey::builder()
            .title("title")
            .key("ssh-rsa ABC")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateDeployKeyBuilderError, "project");
    }

    #[test]
    fn title_is_necessary() {
        let err = CreateDeployKey::builder()
            .project(1)
            .key("ssh-rsa ABC")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateDeployKeyBuilderError, "title");
    }

    #[test]
    fn key_is_necessary() {
        let err = CreateDeployKey::builder()
            .project(1)
            .title("title")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateDeployKeyBuilderError, "key");
    }

    #[test]
    fn project_title_and_key_are_sufficient() {
        CreateDeployKey::builder()
            .project(1)
            .title("title")
            .key("ssh-rsa ABC")
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/deploy_keys")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("title=title", "&key=ssh-rsa+ABC"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateDeployKey::builder()
            .project("simple/project")
            .title("title")
            .key("ssh-rsa ABC")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_can_push() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/deploy_keys")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("title=title", "&key=ssh-rsa+ABC", "&can_push=true"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateDeployKey::builder()
            .project("simple/project")
            .title("title")
            .key("ssh-rsa ABC")
            .can_push(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
