// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Query for merge requests closing an issue
#[derive(Debug, Builder, Clone)]
pub struct MergeRequestsClosing<'a> {
    /// The project to of the merge request.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The ID of the issue
    issue: u64,
}

impl<'a> MergeRequestsClosing<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> MergeRequestsClosingBuilder<'a> {
        MergeRequestsClosingBuilder::default()
    }
}

impl<'a> Endpoint for MergeRequestsClosing<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/issues/{}/closed_by", self.project, self.issue).into()
    }
}

impl<'a> Pageable for MergeRequestsClosing<'a> {}

#[cfg(test)]
mod tests {
    use crate::api::projects::issues::{MergeRequestsClosing, MergeRequestsClosingBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_issue_are_needed() {
        let err = MergeRequestsClosing::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, MergeRequestsClosingBuilderError, "project");
    }

    #[test]
    fn project_is_needed() {
        let err = MergeRequestsClosing::builder()
            .issue(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, MergeRequestsClosingBuilderError, "project");
    }

    #[test]
    fn issue_is_needed() {
        let err = MergeRequestsClosing::builder()
            .project(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, MergeRequestsClosingBuilderError, "issue");
    }

    #[test]
    fn project_and_issue_are_sufficient() {
        MergeRequestsClosing::builder()
            .project(1)
            .issue(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/issues/1/closed_by")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequestsClosing::builder()
            .project("simple/project")
            .issue(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
