// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Query for awards on a note on a merge request within a project.
#[derive(Debug, Builder, Clone)]
pub struct MergeRequestNoteAwards<'a> {
    /// The project to query for the merge request.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The ID of the merge request.
    merge_request: u64,
    /// The ID of the note.
    note: u64,
}

impl<'a> MergeRequestNoteAwards<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> MergeRequestNoteAwardsBuilder<'a> {
        MergeRequestNoteAwardsBuilder::default()
    }
}

impl<'a> Endpoint for MergeRequestNoteAwards<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/merge_requests/{}/notes/{}/award_emoji",
            self.project, self.merge_request, self.note,
        )
        .into()
    }
}

impl<'a> Pageable for MergeRequestNoteAwards<'a> {}

#[cfg(test)]
mod tests {
    use crate::api::projects::merge_requests::notes::awards::{
        MergeRequestNoteAwards, MergeRequestNoteAwardsBuilderError,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_merge_request_and_note_are_necessary() {
        let err = MergeRequestNoteAwards::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, MergeRequestNoteAwardsBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = MergeRequestNoteAwards::builder()
            .merge_request(1)
            .note(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, MergeRequestNoteAwardsBuilderError, "project");
    }

    #[test]
    fn merge_request_is_necessary() {
        let err = MergeRequestNoteAwards::builder()
            .project(1)
            .note(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(
            err,
            MergeRequestNoteAwardsBuilderError,
            "merge_request",
        );
    }

    #[test]
    fn note_is_necessary() {
        let err = MergeRequestNoteAwards::builder()
            .project(1)
            .merge_request(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, MergeRequestNoteAwardsBuilderError, "note");
    }

    #[test]
    fn project_merge_request_and_note_are_sufficient() {
        MergeRequestNoteAwards::builder()
            .project(1)
            .merge_request(1)
            .note(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests/1/notes/1/award_emoji")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequestNoteAwards::builder()
            .project("simple/project")
            .merge_request(1)
            .note(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
