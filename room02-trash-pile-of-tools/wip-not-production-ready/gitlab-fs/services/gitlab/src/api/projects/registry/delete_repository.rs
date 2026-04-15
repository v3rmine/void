// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Delete a registry repository within a project.
#[derive(Debug, Builder, Clone)]
pub struct DeleteRepository<'a> {
    /// The project to delete from.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The repository id to delete.
    repository_id: u64,
}

impl<'a> DeleteRepository<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> DeleteRepositoryBuilder<'a> {
        DeleteRepositoryBuilder::default()
    }
}

impl<'a> Endpoint for DeleteRepository<'a> {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/registry/repositories/{}",
            self.project, self.repository_id,
        )
        .into()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::registry::{DeleteRepository, DeleteRepositoryBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_repository_are_necessary() {
        let err = DeleteRepository::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, DeleteRepositoryBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = DeleteRepository::builder()
            .repository_id(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, DeleteRepositoryBuilderError, "project");
    }

    #[test]
    fn repository_is_necessary() {
        let err = DeleteRepository::builder().project(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, DeleteRepositoryBuilderError, "repository_id");
    }

    #[test]
    fn project_and_repository_are_sufficient() {
        DeleteRepository::builder()
            .project(1)
            .repository_id(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::DELETE)
            .endpoint("projects/simple%2Fproject/registry/repositories/1")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = DeleteRepository::builder()
            .project("simple/project")
            .repository_id(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
