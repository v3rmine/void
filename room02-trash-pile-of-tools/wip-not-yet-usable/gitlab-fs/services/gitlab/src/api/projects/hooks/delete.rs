// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Edit an existing webhook for a project.
#[derive(Debug, Builder, Clone)]
pub struct DeleteHook<'a> {
    /// The project to edit a webhook within.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The ID of the hook to edit.
    hook_id: u64,
}

impl<'a> DeleteHook<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> DeleteHookBuilder<'a> {
        DeleteHookBuilder::default()
    }
}

impl<'a> Endpoint for DeleteHook<'a> {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/hooks/{}", self.project, self.hook_id).into()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::hooks::{DeleteHook, DeleteHookBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_hook_id_are_necessary() {
        let err = DeleteHook::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, DeleteHookBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = DeleteHook::builder().hook_id(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, DeleteHookBuilderError, "project");
    }

    #[test]
    fn hook_id_is_necessary() {
        let err = DeleteHook::builder()
            .project("project")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, DeleteHookBuilderError, "hook_id");
    }

    #[test]
    fn project_and_hook_id_are_sufficient() {
        DeleteHook::builder()
            .project("project")
            .hook_id(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::DELETE)
            .endpoint("projects/simple%2Fproject/hooks/1")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = DeleteHook::builder()
            .project("simple/project")
            .hook_id(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
