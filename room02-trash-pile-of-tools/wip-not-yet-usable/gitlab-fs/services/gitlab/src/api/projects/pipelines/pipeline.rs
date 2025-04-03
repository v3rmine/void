// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Query a single pipeline on a project.
#[derive(Debug, Builder, Clone)]
pub struct Pipeline<'a> {
    /// The project to query for pipeline.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The ID of the pipeline.
    pipeline: u64,
}

impl<'a> Pipeline<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> PipelineBuilder<'a> {
        PipelineBuilder::default()
    }
}

impl<'a> Endpoint for Pipeline<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/pipelines/{}", self.project, self.pipeline).into()
    }
}

#[cfg(test)]
mod tests {
    use crate::api::projects::pipelines::{Pipeline, PipelineBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_pipeline_are_needed() {
        let err = Pipeline::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, PipelineBuilderError, "project");
    }

    #[test]
    fn project_is_needed() {
        let err = Pipeline::builder().pipeline(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, PipelineBuilderError, "project");
    }

    #[test]
    fn pipeline_is_needed() {
        let err = Pipeline::builder().project(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, PipelineBuilderError, "pipeline");
    }

    #[test]
    fn project_and_pipeline_are_sufficient() {
        Pipeline::builder().project(1).pipeline(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/pipelines/1")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Pipeline::builder()
            .project("simple/project")
            .pipeline(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
