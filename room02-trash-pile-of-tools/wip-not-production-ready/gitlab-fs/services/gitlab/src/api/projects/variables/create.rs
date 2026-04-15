// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;
use crate::api::ParamValue;

/// The type of a project variable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectVariableType {
    /// An environment variable.
    ///
    /// The value of the variable is available as the value of the named environment variable.
    EnvVar,
    /// A file variable.
    ///
    /// The value of the variable is available in a file given by the value of the named
    /// environment variable.
    File,
}

impl ProjectVariableType {
    /// The variable type query parameter.
    fn as_str(self) -> &'static str {
        match self {
            ProjectVariableType::EnvVar => "env_var",
            ProjectVariableType::File => "file",
        }
    }
}

impl ParamValue<'static> for ProjectVariableType {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Add a variable to a project.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct CreateProjectVariable<'a> {
    /// The project to add the variable to.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The key of the variable
    #[builder(setter(into))]
    key: Cow<'a, str>,
    /// The value of a variable
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
}

impl<'a> CreateProjectVariable<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> CreateProjectVariableBuilder<'a> {
        CreateProjectVariableBuilder::default()
    }
}

impl<'a> Endpoint for CreateProjectVariable<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/variables", self.project).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params
            .push("key", &self.key)
            .push("value", &self.value)
            .push_opt("variable_type", self.variable_type)
            .push_opt("protected", self.protected)
            .push_opt("masked", self.masked)
            .push_opt("environment_scope", self.environment_scope.as_ref());

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::variables::create::{
        CreateProjectVariable, CreateProjectVariableBuilderError, ProjectVariableType,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_variable_type_as_str() {
        let items = &[
            (ProjectVariableType::EnvVar, "env_var"),
            (ProjectVariableType::File, "file"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn all_parameters_are_needed() {
        let err = CreateProjectVariable::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, CreateProjectVariableBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = CreateProjectVariable::builder()
            .key("testkey")
            .value("testvalue")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateProjectVariableBuilderError, "project");
    }

    #[test]
    fn key_is_necessary() {
        let err = CreateProjectVariable::builder()
            .project(1)
            .value("testvalue")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateProjectVariableBuilderError, "key");
    }

    #[test]
    fn value_level_is_necessary() {
        let err = CreateProjectVariable::builder()
            .project(1)
            .key("testkey")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateProjectVariableBuilderError, "value");
    }

    #[test]
    fn sufficient_parameters() {
        CreateProjectVariable::builder()
            .project(1)
            .key("testkey")
            .value("testvalue")
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/variables")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("key=testkey", "&value=testvalue"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProjectVariable::builder()
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
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/variables")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "key=testkey",
                "&value=testvalue",
                "&variable_type=file"
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProjectVariable::builder()
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
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/variables")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "key=testkey",
                "&value=testvalue",
                "&protected=true"
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProjectVariable::builder()
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
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/variables")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("key=testkey", "&value=testvalue", "&masked=true"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProjectVariable::builder()
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
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/variables")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "key=testkey",
                "&value=testvalue",
                "&environment_scope=*"
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProjectVariable::builder()
            .project("simple/project")
            .key("testkey")
            .value("testvalue")
            .environment_scope("*")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
