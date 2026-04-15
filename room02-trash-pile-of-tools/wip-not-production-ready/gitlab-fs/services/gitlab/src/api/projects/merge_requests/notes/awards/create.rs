// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Create a new award on a note on a merge request on a project.
#[derive(Debug, Builder, Clone)]
pub struct CreateMergeRequestNoteAward<'a> {
    /// The project the merge request belongs to.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The merge request to add the note to.
    merge_request: u64,
    /// The ID of the note.
    note: u64,
    /// The award to give to the note (without colons).
    #[builder(setter(into))]
    name: Cow<'a, str>,
}

impl<'a> CreateMergeRequestNoteAward<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> CreateMergeRequestNoteAwardBuilder<'a> {
        CreateMergeRequestNoteAwardBuilder::default()
    }
}

impl<'a> Endpoint for CreateMergeRequestNoteAward<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/merge_requests/{}/notes/{}/award_emoji",
            self.project, self.merge_request, self.note,
        )
        .into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params.push("name", self.name.as_ref());

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::merge_requests::notes::awards::{
        CreateMergeRequestNoteAward, CreateMergeRequestNoteAwardBuilderError,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_merge_request_note_and_name_are_necessary() {
        let err = CreateMergeRequestNoteAward::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, CreateMergeRequestNoteAwardBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = CreateMergeRequestNoteAward::builder()
            .merge_request(1)
            .note(1)
            .name("award")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateMergeRequestNoteAwardBuilderError, "project");
    }

    #[test]
    fn merge_request_is_necessary() {
        let err = CreateMergeRequestNoteAward::builder()
            .project(1)
            .note(1)
            .name("award")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(
            err,
            CreateMergeRequestNoteAwardBuilderError,
            "merge_request",
        );
    }

    #[test]
    fn note_is_necessary() {
        let err = CreateMergeRequestNoteAward::builder()
            .project(1)
            .merge_request(1)
            .name("award")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateMergeRequestNoteAwardBuilderError, "note");
    }

    #[test]
    fn name_is_necessary() {
        let err = CreateMergeRequestNoteAward::builder()
            .project(1)
            .merge_request(1)
            .note(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateMergeRequestNoteAwardBuilderError, "name");
    }

    #[test]
    fn project_merge_request_note_and_name_are_sufficient() {
        CreateMergeRequestNoteAward::builder()
            .project(1)
            .merge_request(1)
            .note(1)
            .name("award")
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/merge_requests/1/notes/1/award_emoji")
            .content_type("application/x-www-form-urlencoded")
            .body_str("name=emoji")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateMergeRequestNoteAward::builder()
            .project("simple/project")
            .merge_request(1)
            .note(1)
            .name("emoji")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
