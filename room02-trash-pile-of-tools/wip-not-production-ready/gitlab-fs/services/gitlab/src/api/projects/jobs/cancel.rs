// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Cancel a job.
#[derive(Debug, Builder, Clone)]
pub struct CancelJob<'a> {
    /// The project which owns the job.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The ID of the job.
    job: u64,
}

impl<'a> CancelJob<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> CancelJobBuilder<'a> {
        CancelJobBuilder::default()
    }
}

impl<'a> Endpoint for CancelJob<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/jobs/{}/cancel", self.project, self.job).into()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::jobs::{CancelJob, CancelJobBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_job_are_needed() {
        let err = CancelJob::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, CancelJobBuilderError, "project");
    }

    #[test]
    fn project_is_needed() {
        let err = CancelJob::builder().job(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, CancelJobBuilderError, "project");
    }

    #[test]
    fn job_is_needed() {
        let err = CancelJob::builder().project(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, CancelJobBuilderError, "job");
    }

    #[test]
    fn project_and_job_are_sufficient() {
        CancelJob::builder().project(1).job(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/jobs/1/cancel")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CancelJob::builder()
            .project("simple/project")
            .job(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
