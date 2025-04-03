// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Query protected tags of a project.
#[derive(Debug, Clone, Builder)]
pub struct ProtectedTags<'a> {
    /// The project to query for protected tags.
    #[builder(setter(into))]
    project: NameOrId<'a>,
}

impl<'a> ProtectedTags<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> ProtectedTagsBuilder<'a> {
        ProtectedTagsBuilder::default()
    }
}

impl<'a> Endpoint for ProtectedTags<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/protected_tags", self.project).into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        QueryParams::default()
    }
}

#[cfg(test)]
mod tests {
    use crate::api::projects::protected_tags::{ProtectedTags, ProtectedTagsBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_is_needed() {
        let err = ProtectedTags::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, ProtectedTagsBuilderError, "project");
    }

    #[test]
    fn project_is_sufficient() {
        ProtectedTags::builder().project(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/group%2Fproject/protected_tags")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProtectedTags::builder()
            .project("group/project")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
