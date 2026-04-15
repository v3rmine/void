// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Query for resource label events for a merge request.
#[derive(Debug, Builder, Clone)]
pub struct MergeRequestResourceLabelEvents<'a> {
    /// The project to query for the merge request.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The ID of the merge request.
    merge_request: u64,
}

impl<'a> MergeRequestResourceLabelEvents<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> MergeRequestResourceLabelEventsBuilder<'a> {
        MergeRequestResourceLabelEventsBuilder::default()
    }
}

impl<'a> Endpoint for MergeRequestResourceLabelEvents<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/merge_requests/{}/resource_label_events",
            self.project, self.merge_request,
        )
        .into()
    }
}

impl<'a> Pageable for MergeRequestResourceLabelEvents<'a> {}

#[cfg(test)]
mod tests {
    use crate::api::projects::merge_requests::{
        MergeRequestResourceLabelEvents, MergeRequestResourceLabelEventsBuilderError,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_merge_request_are_needed() {
        let err = MergeRequestResourceLabelEvents::builder()
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(
            err,
            MergeRequestResourceLabelEventsBuilderError,
            "project",
        );
    }

    #[test]
    fn project_is_needed() {
        let err = MergeRequestResourceLabelEvents::builder()
            .merge_request(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(
            err,
            MergeRequestResourceLabelEventsBuilderError,
            "project",
        );
    }

    #[test]
    fn merge_request_is_needed() {
        let err = MergeRequestResourceLabelEvents::builder()
            .project(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(
            err,
            MergeRequestResourceLabelEventsBuilderError,
            "merge_request",
        );
    }

    #[test]
    fn project_and_merge_request_are_sufficient() {
        MergeRequestResourceLabelEvents::builder()
            .project(1)
            .merge_request(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests/1/resource_label_events")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequestResourceLabelEvents::builder()
            .project("simple/project")
            .merge_request(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
