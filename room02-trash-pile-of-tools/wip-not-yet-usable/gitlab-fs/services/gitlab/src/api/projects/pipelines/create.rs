// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// The type of a pipeline variable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PipelineVariableType {
    /// An environment variable.
    ///
    /// The value of the variable is available as the value of the named environment variable.
    #[default]
    EnvVar,
    /// A file variable.
    ///
    /// The value of the variable is available in a file given by the value of the named
    /// environment variable.
    File,
}

impl PipelineVariableType {
    /// The variable type query parameter.
    fn as_str(self) -> &'static str {
        match self {
            PipelineVariableType::EnvVar => "env_var",
            PipelineVariableType::File => "file",
        }
    }
}

/// A pipeline variable.
#[derive(Debug, Clone, Builder)]
pub struct PipelineVariable<'a> {
    /// The name of the pipeline variable.
    #[builder(setter(into))]
    pub key: Cow<'a, str>,
    /// The value of the pipeline variable.
    #[builder(setter(into))]
    pub value: Cow<'a, str>,
    /// The way the variable should be exposed to pipeline jobs.
    #[builder(default)]
    pub variable_type: PipelineVariableType,
}

impl<'a> PipelineVariable<'a> {
    /// Create a builder for the pipeline variable.
    pub fn builder() -> PipelineVariableBuilder<'a> {
        PipelineVariableBuilder::default()
    }
}

/// Create a new pipeline on a project.
#[derive(Debug, Builder, Clone)]
pub struct CreatePipeline<'a> {
    /// The project to create the pipeline within.
    #[builder(setter(into))]
    project: NameOrId<'a>,

    /// The ref to create the pipeline for.
    #[builder(setter(into))]
    ref_: Cow<'a, str>,

    /// Search for users with a given custom attribute set.
    #[builder(setter(name = "_variables"), default, private)]
    variables: Vec<PipelineVariable<'a>>,
}

impl<'a> CreatePipeline<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> CreatePipelineBuilder<'a> {
        CreatePipelineBuilder::default()
    }
}

impl<'a> CreatePipelineBuilder<'a> {
    /// Add a variable.
    pub fn variable(&mut self, variable: PipelineVariable<'a>) -> &mut Self {
        self.variables.get_or_insert_with(Vec::new).push(variable);
        self
    }

    /// Add multiple variables.
    pub fn variables<I, V>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = V>,
        V: Into<PipelineVariable<'a>>,
    {
        self.variables
            .get_or_insert_with(Vec::new)
            .extend(iter.map(Into::into));
        self
    }
}

impl<'a> Endpoint for CreatePipeline<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/pipeline", self.project).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params.push("ref", &self.ref_);

        self.variables.iter().for_each(|variable| {
            params.extend(
                [
                    ("variables[][key]", variable.key.as_ref()),
                    ("variables[][value]", variable.value.as_ref()),
                    (
                        "variables[][variable_type]",
                        variable.variable_type.as_str(),
                    ),
                ]
                .iter()
                .cloned(),
            );
        });

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::pipelines::{
        CreatePipeline, CreatePipelineBuilderError, PipelineVariable, PipelineVariableBuilderError,
        PipelineVariableType,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn pipeline_variable_type_default() {
        assert_eq!(
            PipelineVariableType::default(),
            PipelineVariableType::EnvVar,
        );
    }

    #[test]
    fn pipeline_variable_type_as_str() {
        let items = &[
            (PipelineVariableType::EnvVar, "env_var"),
            (PipelineVariableType::File, "file"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn pipeline_variable_key_and_value_are_necessary() {
        let err = PipelineVariable::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, PipelineVariableBuilderError, "key");
    }

    #[test]
    fn pipeline_variable_key_is_necessary() {
        let err = PipelineVariable::builder()
            .value("value")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, PipelineVariableBuilderError, "key");
    }

    #[test]
    fn pipeline_variable_value_is_necessary() {
        let err = PipelineVariable::builder().key("key").build().unwrap_err();
        crate::test::assert_missing_field!(err, PipelineVariableBuilderError, "value");
    }

    #[test]
    fn pipeline_variable_key_and_value_are_sufficient() {
        PipelineVariable::builder()
            .key("key")
            .value("value")
            .build()
            .unwrap();
    }

    #[test]
    fn project_and_ref_are_needed() {
        let err = CreatePipeline::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, CreatePipelineBuilderError, "project");
    }

    #[test]
    fn project_is_needed() {
        let err = CreatePipeline::builder()
            .ref_("testref")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreatePipelineBuilderError, "project");
    }

    #[test]
    fn ref_is_needed() {
        let err = CreatePipeline::builder().project(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, CreatePipelineBuilderError, "ref_");
    }

    #[test]
    fn project_and_ref_are_sufficient() {
        CreatePipeline::builder()
            .project(1)
            .ref_("testref")
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/pipeline")
            .content_type("application/x-www-form-urlencoded")
            .body_str("ref=master")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreatePipeline::builder()
            .project("simple/project")
            .ref_("master")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_pipeline_variables() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/1/pipeline")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "ref=master",
                "&variables%5B%5D%5Bkey%5D=key",
                "&variables%5B%5D%5Bvalue%5D=value",
                "&variables%5B%5D%5Bvariable_type%5D=env_var",
                "&variables%5B%5D%5Bkey%5D=file",
                "&variables%5B%5D%5Bvalue%5D=contents",
                "&variables%5B%5D%5Bvariable_type%5D=file",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreatePipeline::builder()
            .project(1)
            .ref_("master")
            .variable(
                PipelineVariable::builder()
                    .key("key")
                    .value("value")
                    .build()
                    .unwrap(),
            )
            .variables(
                [PipelineVariable::builder()
                    .key("file")
                    .value("contents")
                    .variable_type(PipelineVariableType::File)
                    .build()
                    .unwrap()]
                .iter()
                .cloned(),
            )
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
