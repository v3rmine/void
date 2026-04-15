// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Query for webhooks within a project.
#[derive(Debug, Builder, Clone)]
pub struct Hooks<'a> {
    /// The project to query for webhooks.
    #[builder(setter(into))]
    project: NameOrId<'a>,
}

impl<'a> Hooks<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> HooksBuilder<'a> {
        HooksBuilder::default()
    }
}

impl<'a> Endpoint for Hooks<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/hooks", self.project).into()
    }
}

impl<'a> Pageable for Hooks<'a> {}

#[cfg(test)]
mod tests {
    use crate::api::projects::hooks::{Hooks, HooksBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_is_needed() {
        let err = Hooks::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, HooksBuilderError, "project");
    }

    #[test]
    fn project_is_sufficient() {
        Hooks::builder().project(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/hooks")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Hooks::builder().project("simple/project").build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
