// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::HashSet;

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;
use crate::api::ParamValue;

/// Scopes for jobs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JobScope {
    /// Created, but blocked on dependencies or triggers.
    Created,
    /// Ready to run, but have not been claimed by a runner.
    Pending,
    /// Currently running.
    Running,
    /// Failed jobs.
    Failed,
    /// Successful jobs.
    Success,
    /// Canceled jobs.
    Canceled,
    /// Skipped jobs.
    Skipped,
    /// Awaiting manual triggering.
    Manual,
}

impl JobScope {
    /// The scope as a query parameter.
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            JobScope::Created => "created",
            JobScope::Pending => "pending",
            JobScope::Running => "running",
            JobScope::Failed => "failed",
            JobScope::Success => "success",
            JobScope::Canceled => "canceled",
            JobScope::Skipped => "skipped",
            JobScope::Manual => "manual",
        }
    }
}

impl ParamValue<'static> for JobScope {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Query for jobs within a project.
#[derive(Debug, Builder, Clone)]
pub struct Jobs<'a> {
    /// The project to query for jobs.
    #[builder(setter(into))]
    project: NameOrId<'a>,

    /// The scopes to filter jobs by.
    #[builder(setter(name = "_scopes"), default, private)]
    scopes: HashSet<JobScope>,
}

impl<'a> Jobs<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> JobsBuilder<'a> {
        JobsBuilder::default()
    }
}

impl<'a> JobsBuilder<'a> {
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

impl<'a> Endpoint for Jobs<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/jobs", self.project).into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();

        params.extend(self.scopes.iter().map(|&value| ("scope[]", value)));

        params
    }
}

impl<'a> Pageable for Jobs<'a> {}

#[cfg(test)]
mod tests {
    use crate::api::projects::jobs::{JobScope, Jobs, JobsBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn job_scope_as_str() {
        let items = &[
            (JobScope::Created, "created"),
            (JobScope::Pending, "pending"),
            (JobScope::Running, "running"),
            (JobScope::Failed, "failed"),
            (JobScope::Success, "success"),
            (JobScope::Canceled, "canceled"),
            (JobScope::Skipped, "skipped"),
            (JobScope::Manual, "manual"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn project_is_needed() {
        let err = Jobs::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, JobsBuilderError, "project");
    }

    #[test]
    fn project_is_sufficient() {
        Jobs::builder().project(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/jobs")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Jobs::builder().project("simple/project").build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_scopes() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/1/jobs")
            .add_query_params(&[("scope[]", "created"), ("scope[]", "success")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Jobs::builder()
            .project(1)
            .scope(JobScope::Created)
            .scopes([JobScope::Created, JobScope::Success].iter().cloned())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
