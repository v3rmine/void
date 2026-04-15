// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::{self, NameOrId};
use crate::api::endpoint_prelude::*;

/// Delete a tag from a registry repository within a project.
#[derive(Debug, Builder, Clone)]
pub struct DeleteRepositoryTag<'a> {
    /// The project to delete from.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The repository id to delete from.
    repository_id: u64,
    /// The tag name to delete.
    #[builder(setter(into))]
    tag_name: Cow<'a, str>,
}

impl<'a> DeleteRepositoryTag<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> DeleteRepositoryTagBuilder<'a> {
        DeleteRepositoryTagBuilder::default()
    }
}

impl<'a> Endpoint for DeleteRepositoryTag<'a> {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/registry/repositories/{}/tags/{}",
            self.project,
            self.repository_id,
            common::path_escaped(&self.tag_name),
        )
        .into()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::registry::{DeleteRepositoryTag, DeleteRepositoryTagBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn parameters_are_necessary() {
        let err = DeleteRepositoryTag::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, DeleteRepositoryTagBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = DeleteRepositoryTag::builder()
            .repository_id(1)
            .tag_name("latest")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, DeleteRepositoryTagBuilderError, "project");
    }

    #[test]
    fn repository_is_necessary() {
        let err = DeleteRepositoryTag::builder()
            .project(1)
            .tag_name("latest")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, DeleteRepositoryTagBuilderError, "repository_id");
    }

    #[test]
    fn tag_name_is_necessary() {
        let err = DeleteRepositoryTag::builder()
            .project(1)
            .repository_id(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, DeleteRepositoryTagBuilderError, "tag_name");
    }

    #[test]
    fn all_parameters_are_sufficient() {
        DeleteRepositoryTag::builder()
            .project(1)
            .repository_id(1)
            .tag_name("latest")
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::DELETE)
            .endpoint("projects/simple%2Fproject/registry/repositories/1/tags/latest")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = DeleteRepositoryTag::builder()
            .project("simple/project")
            .repository_id(1)
            .tag_name("latest")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
