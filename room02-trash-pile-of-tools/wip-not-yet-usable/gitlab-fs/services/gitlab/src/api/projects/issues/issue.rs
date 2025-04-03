// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Query for a issue within a project.
#[derive(Debug, Builder, Clone)]
pub struct Issue<'a> {
    /// The project to query for the issue.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The ID of the issue.
    issue: u64,
}

impl<'a> Issue<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> IssueBuilder<'a> {
        IssueBuilder::default()
    }
}

impl<'a> Endpoint for Issue<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/issues/{}", self.project, self.issue).into()
    }
}

#[cfg(test)]
mod tests {
    use crate::api::projects::issues::{Issue, IssueBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_issue_are_needed() {
        let err = Issue::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, IssueBuilderError, "project");
    }

    #[test]
    fn project_is_needed() {
        let err = Issue::builder().issue(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, IssueBuilderError, "project");
    }

    #[test]
    fn issue_is_needed() {
        let err = Issue::builder().project(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, IssueBuilderError, "issue");
    }

    #[test]
    fn project_and_issue_are_sufficient() {
        Issue::builder().project(1).issue(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/issues/1")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Issue::builder()
            .project("simple/project")
            .issue(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
