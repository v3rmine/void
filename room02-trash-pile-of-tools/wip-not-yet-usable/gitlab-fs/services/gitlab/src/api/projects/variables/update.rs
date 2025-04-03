// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::{self, NameOrId};
use crate::api::endpoint_prelude::*;
use crate::api::projects::variables::{ProjectVariableFilter, ProjectVariableType};

/// Edit a variable of a project.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct UpdateProjectVariable<'a> {
    /// The project to edit the variable on.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The name of the variable.
    #[builder(setter(into))]
    key: Cow<'a, str>,
    /// The value of the variable.
    #[builder(setter(into))]
    value: Cow<'a, str>,
    /// The type of the variable.
    #[builder(default)]
    variable_type: Option<ProjectVariableType>,
    /// Whether the variable is protected.
    #[builder(default)]
    protected: Option<bool>,
    /// Whether the variable is masked.
    #[builder(default)]
    masked: Option<bool>,
    /// The environment scope of the variable.
    #[builder(setter(into), default)]
    environment_scope: Option<Cow<'a, str>>,
    /// Filters to apply to the returned variables.
    #[builder(default)]
    filter: Option<ProjectVariableFilter<'a>>,
}

impl<'a> UpdateProjectVariable<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> UpdateProjectVariableBuilder<'a> {
        UpdateProjectVariableBuilder::default()
    }
}

impl<'a> Endpoint for UpdateProjectVariable<'a> {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/variables/{}",
            self.project,
            common::path_escaped(&self.key),
        )
        .into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params
            .push("value", &self.value)
            .push_opt("variable_type", self.variable_type)
            .push_opt("protected", self.protected)
            .push_opt("masked", self.masked)
            .push_opt("environment_scope", self.environment_scope.as_ref());

        if let Some(filter) = self.filter.as_ref() {
            filter.add_query(&mut params);
        }

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::variables::update::{
        ProjectVariableFilter, ProjectVariableType, UpdateProjectVariable,
        UpdateProjectVariableBuilderError,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn all_parameters_are_needed() {
        let err = UpdateProjectVariable::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, UpdateProjectVariableBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = UpdateProjectVariable::builder()
            .key("testkey")
            .value("testvalue")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, UpdateProjectVariableBuilderError, "project");
    }

    #[test]
    fn key_is_necessary() {
        let err = UpdateProjectVariable::builder()
            .project(1)
            .value("testvalue")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, UpdateProjectVariableBuilderError, "key");
    }

    #[test]
    fn value_level_is_necessary() {
        let err = UpdateProjectVariable::builder()
            .project(1)
            .key("testkey")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, UpdateProjectVariableBuilderError, "value");
    }

    #[test]
    fn sufficient_parameters() {
        UpdateProjectVariable::builder()
            .project(1)
            .key("testkey")
            .value("testvalue")
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/variables/testkey")
            .content_type("application/x-www-form-urlencoded")
            .body_str("value=testvalue")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = UpdateProjectVariable::builder()
            .project("simple/project")
            .key("testkey")
            .value("testvalue")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_variable_type() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/variables/testkey")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("value=testvalue", "&variable_type=file"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = UpdateProjectVariable::builder()
            .project("simple/project")
            .key("testkey")
            .value("testvalue")
            .variable_type(ProjectVariableType::File)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_protected() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/variables/testkey")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("value=testvalue", "&protected=true"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = UpdateProjectVariable::builder()
            .project("simple/project")
            .key("testkey")
            .value("testvalue")
            .protected(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_masked() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/variables/testkey")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("value=testvalue", "&masked=true"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = UpdateProjectVariable::builder()
            .project("simple/project")
            .key("testkey")
            .value("testvalue")
            .masked(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_environment_scope() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/variables/testkey")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("value=testvalue", "&environment_scope=*"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = UpdateProjectVariable::builder()
            .project("simple/project")
            .key("testkey")
            .value("testvalue")
            .environment_scope("*")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_filter() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/variables/testkey")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "value=testvalue",
                "&filter%5Benvironment_scope%5D=production",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = UpdateProjectVariable::builder()
            .project("simple/project")
            .key("testkey")
            .value("testvalue")
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
