// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Cancel a pipeline.
#[derive(Debug, Builder, Clone)]
pub struct CancelPipeline<'a> {
    /// The project to query for the pipeline.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The ID of the pipeline.
    pipeline: u64,
}

impl<'a> CancelPipeline<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> CancelPipelineBuilder<'a> {
        CancelPipelineBuilder::default()
    }
}

impl<'a> Endpoint for CancelPipeline<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/pipelines/{}/cancel",
            self.project, self.pipeline,
        )
        .into()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::pipelines::{CancelPipeline, CancelPipelineBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_pipeline_are_needed() {
        let err = CancelPipeline::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, CancelPipelineBuilderError, "project");
    }

    #[test]
    fn project_is_needed() {
        let err = CancelPipeline::builder().pipeline(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, CancelPipelineBuilderError, "project");
    }

    #[test]
    fn pipeline_is_needed() {
        let err = CancelPipeline::builder().project(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, CancelPipelineBuilderError, "pipeline");
    }

    #[test]
    fn project_and_pipeline_are_sufficient() {
        CancelPipeline::builder()
            .project(1)
            .pipeline(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/pipelines/1/cancel")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CancelPipeline::builder()
            .project("simple/project")
            .pipeline(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
