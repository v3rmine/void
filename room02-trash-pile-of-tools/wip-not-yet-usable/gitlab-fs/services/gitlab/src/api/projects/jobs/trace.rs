// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Query for a job within a project.
#[derive(Debug, Builder, Clone)]
pub struct JobTrace<'a> {
    /// The project to query for the job.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The ID of the job.
    job: u64,
}

impl<'a> JobTrace<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> JobTraceBuilder<'a> {
        JobTraceBuilder::default()
    }
}

impl<'a> Endpoint for JobTrace<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/jobs/{}/trace", self.project, self.job).into()
    }
}

#[cfg(test)]
mod tests {
    use crate::api::projects::jobs::{JobTrace, JobTraceBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_job_are_needed() {
        let err = JobTrace::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, JobTraceBuilderError, "project");
    }

    #[test]
    fn project_is_needed() {
        let err = JobTrace::builder().job(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, JobTraceBuilderError, "project");
    }

    #[test]
    fn job_is_needed() {
        let err = JobTrace::builder().project(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, JobTraceBuilderError, "job");
    }

    #[test]
    fn project_and_job_are_sufficient() {
        JobTrace::builder().project(1).job(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/jobs/1/trace")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = JobTrace::builder()
            .project("simple/project")
            .job(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
