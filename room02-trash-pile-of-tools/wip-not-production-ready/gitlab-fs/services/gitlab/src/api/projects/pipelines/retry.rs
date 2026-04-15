// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Retry a pipeline.
///
/// Note that this only restarts failed jobs.
#[derive(Debug, Builder, Clone)]
pub struct RetryPipeline<'a> {
    /// The project of the pipelines.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The ID of the pipeline.
    pipeline: u64,
}

impl<'a> RetryPipeline<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> RetryPipelineBuilder<'a> {
        RetryPipelineBuilder::default()
    }
}

impl<'a> Endpoint for RetryPipeline<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/pipelines/{}/retry",
            self.project, self.pipeline,
        )
        .into()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::pipelines::{RetryPipeline, RetryPipelineBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_pipeline_are_needed() {
        let err = RetryPipeline::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, RetryPipelineBuilderError, "project");
    }

    #[test]
    fn project_is_needed() {
        let err = RetryPipeline::builder().pipeline(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, RetryPipelineBuilderError, "project");
    }

    #[test]
    fn pipeline_is_needed() {
        let err = RetryPipeline::builder().project(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, RetryPipelineBuilderError, "pipeline");
    }

    #[test]
    fn project_and_pipeline_are_sufficient() {
        RetryPipeline::builder()
            .project(1)
            .pipeline(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/pipelines/1/retry")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = RetryPipeline::builder()
            .project("simple/project")
            .pipeline(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
