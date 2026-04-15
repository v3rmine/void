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
pub struct Job<'a> {
    /// The project to query for the job.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The ID of the job.
    job: u64,
}

impl<'a> Job<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> JobBuilder<'a> {
        JobBuilder::default()
    }
}

impl<'a> Endpoint for Job<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/jobs/{}", self.project, self.job).into()
    }
}

#[cfg(test)]
mod tests {
    use crate::api::projects::jobs::{Job, JobBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_job_are_needed() {
        let err = Job::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, JobBuilderError, "project");
    }

    #[test]
    fn project_is_needed() {
        let err = Job::builder().job(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, JobBuilderError, "project");
    }

    #[test]
    fn job_is_needed() {
        let err = Job::builder().project(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, JobBuilderError, "job");
    }

    #[test]
    fn project_and_job_are_sufficient() {
        Job::builder().project(1).job(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/1/jobs/1")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Job::builder().project(1).job(1).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
