// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Query for a specific project on an instance.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Project<'a> {
    /// The project to get.
    #[builder(setter(into))]
    project: NameOrId<'a>,

    /// Include project statistics in the results.
    #[builder(default)]
    statistics: Option<bool>,
    /// Include project license information in the results.
    #[builder(default)]
    license: Option<bool>,
    /// Search for projects with custom attributes.
    #[builder(default)]
    with_custom_attributes: Option<bool>,
}

impl<'a> Project<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> ProjectBuilder<'a> {
        ProjectBuilder::default()
    }
}

impl<'a> Endpoint for Project<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}", self.project).into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();

        params
            .push_opt("statistics", self.statistics)
            .push_opt("license", self.license)
            .push_opt("with_custom_attributes", self.with_custom_attributes);

        params
    }
}

#[cfg(test)]
mod tests {
    use crate::api::projects::{Project, ProjectBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_is_necessary() {
        let err = Project::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, ProjectBuilderError, "project");
    }

    #[test]
    fn project_is_sufficient() {
        Project::builder().project(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Project::builder()
            .project("simple/project")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_statistics() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject")
            .add_query_params(&[("statistics", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Project::builder()
            .project("simple/project")
            .statistics(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_license() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject")
            .add_query_params(&[("license", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Project::builder()
            .project("simple/project")
            .license(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_with_custom_attributes() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject")
            .add_query_params(&[("with_custom_attributes", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Project::builder()
            .project("simple/project")
            .with_custom_attributes(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
