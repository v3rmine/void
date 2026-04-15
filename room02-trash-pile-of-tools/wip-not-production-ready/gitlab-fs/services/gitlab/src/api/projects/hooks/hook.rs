// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Query for a webhook within a project.
#[derive(Debug, Builder, Clone)]
pub struct Hook<'a> {
    /// The project to query for webhooks.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The ID of the hook.
    hook: u64,
}

impl<'a> Hook<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> HookBuilder<'a> {
        HookBuilder::default()
    }
}

impl<'a> Endpoint for Hook<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/hooks/{}", self.project, self.hook).into()
    }
}

#[cfg(test)]
mod tests {
    use crate::api::projects::hooks::{Hook, HookBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_hook_are_needed() {
        let err = Hook::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, HookBuilderError, "project");
    }

    #[test]
    fn project_is_needed() {
        let err = Hook::builder().hook(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, HookBuilderError, "project");
    }

    #[test]
    fn hook_is_needed() {
        let err = Hook::builder().project(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, HookBuilderError, "hook");
    }

    #[test]
    fn project_and_hook_are_sufficient() {
        Hook::builder().project(1).hook(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/hooks/1")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Hook::builder()
            .project("simple/project")
            .hook(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
