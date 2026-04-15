// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Query for resource label events for an issue.
#[derive(Debug, Builder, Clone)]
pub struct IssueResourceLabelEvents<'a> {
    /// The project to query for the issue.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The ID of the issue.
    issue: u64,
}

impl<'a> IssueResourceLabelEvents<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> IssueResourceLabelEventsBuilder<'a> {
        IssueResourceLabelEventsBuilder::default()
    }
}

impl<'a> Endpoint for IssueResourceLabelEvents<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/issues/{}/resource_label_events",
            self.project, self.issue,
        )
        .into()
    }
}

impl<'a> Pageable for IssueResourceLabelEvents<'a> {}

#[cfg(test)]
mod tests {
    use crate::api::projects::issues::{
        IssueResourceLabelEvents, IssueResourceLabelEventsBuilderError,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_issue_are_needed() {
        let err = IssueResourceLabelEvents::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, IssueResourceLabelEventsBuilderError, "project");
    }

    #[test]
    fn project_is_needed() {
        let err = IssueResourceLabelEvents::builder()
            .issue(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, IssueResourceLabelEventsBuilderError, "project");
    }

    #[test]
    fn issue_is_needed() {
        let err = IssueResourceLabelEvents::builder()
            .project(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, IssueResourceLabelEventsBuilderError, "issue");
    }

    #[test]
    fn project_and_issue_are_sufficient() {
        IssueResourceLabelEvents::builder()
            .project(1)
            .issue(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/issues/1/resource_label_events")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = IssueResourceLabelEvents::builder()
            .project("simple/project")
            .issue(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
