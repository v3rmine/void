// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Query for tags of a registry repository within a project.
#[derive(Debug, Builder, Clone)]
pub struct RepositoryTags<'a> {
    /// The project to query.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The repository id to query.
    repository_id: u64,
}

impl<'a> RepositoryTags<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> RepositoryTagsBuilder<'a> {
        RepositoryTagsBuilder::default()
    }
}

impl<'a> Endpoint for RepositoryTags<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/registry/repositories/{}/tags",
            self.project, self.repository_id,
        )
        .into()
    }
}

#[cfg(test)]
mod tests {
    use crate::api::projects::registry::{RepositoryTags, RepositoryTagsBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_repository_are_necessary() {
        let err = RepositoryTags::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, RepositoryTagsBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = RepositoryTags::builder()
            .repository_id(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, RepositoryTagsBuilderError, "project");
    }

    #[test]
    fn repository_is_necessary() {
        let err = RepositoryTags::builder().project(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, RepositoryTagsBuilderError, "repository_id");
    }

    #[test]
    fn project_and_repository_are_sufficient() {
        RepositoryTags::builder()
            .project(1)
            .repository_id(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/registry/repositories/1/tags")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = RepositoryTags::builder()
            .project("simple/project")
            .repository_id(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
