// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::HashSet;

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;
use crate::api::projects::jobs::JobScope;

/// Query for jobs within a pipeline.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct PipelineJobs<'a> {
    /// The project to query for the pipeline.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The ID of the pipeline.
    pipeline: u64,

    /// The scopes to filter jobs by.
    #[builder(setter(name = "_scopes"), default, private)]
    scopes: HashSet<JobScope>,
    /// Include retried jobs in the response.
    #[builder(default)]
    include_retried: Option<bool>,
}

impl<'a> PipelineJobs<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> PipelineJobsBuilder<'a> {
        PipelineJobsBuilder::default()
    }
}

impl<'a> PipelineJobsBuilder<'a> {
    /// Filter jobs by a scope.
    pub fn scope(&mut self, scope: JobScope) -> &mut Self {
        self.scopes.get_or_insert_with(HashSet::new).insert(scope);
        self
    }

    /// Filter jobs by a set of scopes.
    pub fn scopes<I>(&mut self, scopes: I) -> &mut Self
    where
        I: Iterator<Item = JobScope>,
    {
        self.scopes.get_or_insert_with(HashSet::new).extend(scopes);
        self
    }
}

impl<'a> Endpoint for PipelineJobs<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/pipelines/{}/jobs", self.project, self.pipeline).into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();

        params
            .extend(self.scopes.iter().map(|&value| ("scope[]", value)))
            .push_opt("include_retried", self.include_retried);

        params
    }
}

impl<'a> Pageable for PipelineJobs<'a> {}

#[cfg(test)]
mod tests {
    use crate::api::projects::jobs::JobScope;
    use crate::api::projects::pipelines::{PipelineJobs, PipelineJobsBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_pipeline_are_needed() {
        let err = PipelineJobs::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, PipelineJobsBuilderError, "project");
    }

    #[test]
    fn project_is_needed() {
        let err = PipelineJobs::builder().pipeline(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, PipelineJobsBuilderError, "project");
    }

    #[test]
    fn pipeline_is_needed() {
        let err = PipelineJobs::builder().project(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, PipelineJobsBuilderError, "pipeline");
    }

    #[test]
    fn project_and_pipeline_are_sufficient() {
        PipelineJobs::builder()
            .project(1)
            .pipeline(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/pipelines/1/jobs")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = PipelineJobs::builder()
            .project("simple/project")
            .pipeline(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_scopes() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/1/pipelines/1/jobs")
            .add_query_params(&[("scope[]", "created"), ("scope[]", "success")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = PipelineJobs::builder()
            .project(1)
            .pipeline(1)
            .scope(JobScope::Created)
            .scopes([JobScope::Created, JobScope::Success].iter().cloned())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_include_retried() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/1/pipelines/1/jobs")
            .add_query_params(&[("include_retried", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = PipelineJobs::builder()
            .project(1)
            .pipeline(1)
            .include_retried(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
