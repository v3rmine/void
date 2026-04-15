// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::{self, NameOrId};
use crate::api::endpoint_prelude::*;

/// Filter parameters.
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct ProjectVariableFilter<'a> {
    /// Filter based on the environment scope.
    #[builder(setter(into), default)]
    environment_scope: Option<Cow<'a, str>>,
}

impl<'a> ProjectVariableFilter<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> ProjectVariableFilterBuilder<'a> {
        ProjectVariableFilterBuilder::default()
    }

    pub(crate) fn add_query<'b>(&'b self, params: &mut FormParams<'b>) {
        params.push_opt("filter[environment_scope]", self.environment_scope.as_ref());
    }
}

/// Get the variable from a project.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct ProjectVariable<'a> {
    /// The project to get the variable from.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The name of the variable.
    #[builder(setter(into))]
    key: Cow<'a, str>,
    /// Filter
    #[builder(default)]
    filter: Option<ProjectVariableFilter<'a>>,
}

impl<'a> ProjectVariable<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> ProjectVariableBuilder<'a> {
        ProjectVariableBuilder::default()
    }
}

impl<'a> Endpoint for ProjectVariable<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/variables/{}",
            self.project,
            common::path_escaped(self.key.as_ref()),
        )
        .into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        if let Some(filter) = self.filter.as_ref() {
            filter.add_query(&mut params);
        }

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::variables::variable::{
        ProjectVariable, ProjectVariableBuilderError, ProjectVariableFilter,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn all_parameters_are_needed() {
        let err = ProjectVariable::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, ProjectVariableBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = ProjectVariable::builder()
            .key("testkey")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, ProjectVariableBuilderError, "project");
    }

    #[test]
    fn key_is_necessary() {
        let err = ProjectVariable::builder().project(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, ProjectVariableBuilderError, "key");
    }

    #[test]
    fn sufficient_parameters() {
        ProjectVariable::builder()
            .project(1)
            .key("testkey")
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::GET)
            .endpoint("projects/simple%2Fproject/variables/testkey%2F")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(""))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectVariable::builder()
            .project("simple/project")
            .key("testkey/")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_filter() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::GET)
            .endpoint("projects/simple%2Fproject/variables/testkey%2F")
            .content_type("application/x-www-form-urlencoded")
            .body_str("filter%5Benvironment_scope%5D=production")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectVariable::builder()
            .project("simple/project")
            .key("testkey/")
            .filter(
                ProjectVariableFilter::builder()
                    .environment_scope("production")
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
