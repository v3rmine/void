// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Query for registry repositories within a project.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Repositories<'a> {
    /// The project to query for repositories.
    #[builder(setter(into))]
    project: NameOrId<'a>,

    /// Include tags for each repository in response.
    #[builder(default)]
    tags: Option<bool>,
    /// Include tags_count for each repository in response.
    #[builder(default)]
    tags_count: Option<bool>,
}

impl<'a> Repositories<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> RepositoriesBuilder<'a> {
        RepositoriesBuilder::default()
    }
}

impl<'a> Endpoint for Repositories<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/registry/repositories", self.project).into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();

        params
            .push_opt("tags", self.tags)
            .push_opt("tags_count", self.tags_count);

        params
    }
}

impl<'a> Pageable for Repositories<'a> {}

#[cfg(test)]
mod tests {
    use crate::api::projects::registry::{Repositories, RepositoriesBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_is_necessary() {
        let err = Repositories::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, RepositoriesBuilderError, "project");
    }

    #[test]
    fn project_is_sufficient() {
        Repositories::builder().project(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/registry/repositories")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Repositories::builder()
            .project("simple/project")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_tags() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/registry/repositories")
            .add_query_params(&[("tags", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Repositories::builder()
            .project("simple/project")
            .tags(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_tags_count() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/registry/repositories")
            .add_query_params(&[("tags_count", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Repositories::builder()
            .project("simple/project")
            .tags_count(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
