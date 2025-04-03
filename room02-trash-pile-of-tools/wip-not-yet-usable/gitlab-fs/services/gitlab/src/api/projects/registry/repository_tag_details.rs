// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::{self, NameOrId};
use crate::api::endpoint_prelude::*;

/// Query for tag details of a registry repository within a project.
#[derive(Debug, Builder, Clone)]
pub struct RepositoryTagDetails<'a> {
    /// The project to query.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The repository id to query.
    repository_id: u64,
    /// The tag name to query.
    #[builder(setter(into))]
    tag_name: Cow<'a, str>,
}

impl<'a> RepositoryTagDetails<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> RepositoryTagDetailsBuilder<'a> {
        RepositoryTagDetailsBuilder::default()
    }
}

impl<'a> Endpoint for RepositoryTagDetails<'a> {
    fn method(&self) -> Method {
        Method::GET
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
    use crate::api::projects::registry::{RepositoryTagDetails, RepositoryTagDetailsBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn parameters_are_necessary() {
        let err = RepositoryTagDetails::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, RepositoryTagDetailsBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = RepositoryTagDetails::builder()
            .repository_id(1)
            .tag_name("latest")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, RepositoryTagDetailsBuilderError, "project");
    }

    #[test]
    fn repository_is_necessary() {
        let err = RepositoryTagDetails::builder()
            .project(1)
            .tag_name("latest")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, RepositoryTagDetailsBuilderError, "repository_id");
    }

    #[test]
    fn tag_name_is_necessary() {
        let err = RepositoryTagDetails::builder()
            .project(1)
            .repository_id(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, RepositoryTagDetailsBuilderError, "tag_name");
    }

    #[test]
    fn all_parameters_are_sufficient() {
        RepositoryTagDetails::builder()
            .project(1)
            .repository_id(1)
            .tag_name("latest")
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/registry/repositories/1/tags/latest")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = RepositoryTagDetails::builder()
            .project("simple/project")
            .repository_id(1)
            .tag_name("latest")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
