// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::{self, NameOrId};
use crate::api::endpoint_prelude::*;

/// Query a protected tag of a project.
#[derive(Debug, Clone, Builder)]
pub struct ProtectedTag<'a> {
    /// The project to query for the protected tag.
    #[builder(setter(into))]
    project: NameOrId<'a>,

    /// The name of the tag or wildcard.
    #[builder(setter(into))]
    name: Cow<'a, str>,
}

impl<'a> ProtectedTag<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> ProtectedTagBuilder<'a> {
        ProtectedTagBuilder::default()
    }
}

impl<'a> Endpoint for ProtectedTag<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/protected_tags/{}",
            self.project,
            common::path_escaped(self.name.as_ref()),
        )
        .into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        QueryParams::default()
    }
}

#[cfg(test)]
mod tests {
    use crate::api::projects::protected_tags::{ProtectedTag, ProtectedTagBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_is_needed() {
        let err = ProtectedTag::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, ProtectedTagBuilderError, "project");
    }

    #[test]
    fn name_is_needed() {
        let err = ProtectedTag::builder()
            .project("project_name")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, ProtectedTagBuilderError, "name");
    }

    #[test]
    fn project_and_name_is_sufficient() {
        ProtectedTag::builder()
            .project(1)
            .name("1.0")
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/group%2Fproject/protected_tags/master")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProtectedTag::builder()
            .project("group/project")
            .name("master")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
