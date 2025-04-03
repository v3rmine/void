// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// A job variable for a manual job.
#[derive(Debug, Builder, Clone)]
pub struct JobVariableAttribute<'a> {
    /// The name of the variable.
    #[builder(setter(into))]
    key: Cow<'a, str>,
    /// The value of the variable.
    #[builder(setter(into))]
    value: Cow<'a, str>,
}

impl<'a> JobVariableAttribute<'a> {
    /// Create a builder for a job variable.
    pub fn builder() -> JobVariableAttributeBuilder<'a> {
        JobVariableAttributeBuilder::default()
    }

    fn add_params<'b>(&'b self, params: &mut FormParams<'b>) {
        params.push("job_variables_attributes[][key]", self.key.as_ref());
        params.push("job_variables_attributes[][value]", self.value.as_ref());
    }
}

/// Play a job.
#[derive(Debug, Builder, Clone)]
pub struct PlayJob<'a> {
    /// The project which owns the job.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The ID of the job.
    job: u64,

    /// Variables to set for the job.
    #[builder(setter(name = "_job_variables_attributes"), default, private)]
    job_variables_attributes: Vec<JobVariableAttribute<'a>>,
}

impl<'a> PlayJob<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> PlayJobBuilder<'a> {
        PlayJobBuilder::default()
    }
}

impl<'a> PlayJobBuilder<'a> {
    /// Add a job variable to the job.
    pub fn job_variables_attribute(&mut self, attr: JobVariableAttribute<'a>) -> &mut Self {
        self.job_variables_attributes
            .get_or_insert_with(Vec::new)
            .push(attr);
        self
    }

    /// Add job variables to the job.
    pub fn job_variables_attributes<I>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = JobVariableAttribute<'a>>,
    {
        self.job_variables_attributes
            .get_or_insert_with(Vec::new)
            .extend(iter);
        self
    }
}

impl<'a> Endpoint for PlayJob<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/jobs/{}/play", self.project, self.job).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        self.job_variables_attributes
            .iter()
            .for_each(|value| value.add_params(&mut params));

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::jobs::{
        JobVariableAttribute, JobVariableAttributeBuilderError, PlayJob, PlayJobBuilderError,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn key_and_value_are_needed() {
        let err = JobVariableAttribute::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, JobVariableAttributeBuilderError, "key");
    }

    #[test]
    fn key_is_needed() {
        let err = JobVariableAttribute::builder()
            .value("value")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, JobVariableAttributeBuilderError, "key");
    }

    #[test]
    fn value_is_needed() {
        let err = JobVariableAttribute::builder()
            .key("key")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, JobVariableAttributeBuilderError, "value");
    }

    #[test]
    fn key_and_value_are_sufficient() {
        JobVariableAttribute::builder()
            .key("key")
            .value("value")
            .build()
            .unwrap();
    }

    #[test]
    fn project_and_job_are_needed() {
        let err = PlayJob::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, PlayJobBuilderError, "project");
    }

    #[test]
    fn project_is_needed() {
        let err = PlayJob::builder().job(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, PlayJobBuilderError, "project");
    }

    #[test]
    fn job_is_needed() {
        let err = PlayJob::builder().project(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, PlayJobBuilderError, "job");
    }

    #[test]
    fn project_and_job_are_sufficient() {
        PlayJob::builder().project(1).job(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/jobs/1/play")
            .content_type("application/x-www-form-urlencoded")
            .body_str("")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = PlayJob::builder()
            .project("simple/project")
            .job(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_job_variable_attributes() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/jobs/1/play")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "job_variables_attributes%5B%5D%5Bkey%5D=var1",
                "&job_variables_attributes%5B%5D%5Bvalue%5D=value1",
                "&job_variables_attributes%5B%5D%5Bkey%5D=var2",
                "&job_variables_attributes%5B%5D%5Bvalue%5D=value2",
                "&job_variables_attributes%5B%5D%5Bkey%5D=var3",
                "&job_variables_attributes%5B%5D%5Bvalue%5D=value3",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = PlayJob::builder()
            .project("simple/project")
            .job(1)
            .job_variables_attribute(
                JobVariableAttribute::builder()
                    .key("var1")
                    .value("value1")
                    .build()
                    .unwrap(),
            )
            .job_variables_attributes(
                [
                    JobVariableAttribute::builder()
                        .key("var2")
                        .value("value2")
                        .build()
                        .unwrap(),
                    JobVariableAttribute::builder()
                        .key("var3")
                        .value("value3")
                        .build()
                        .unwrap(),
                ]
                .iter()
                .cloned(),
            )
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
