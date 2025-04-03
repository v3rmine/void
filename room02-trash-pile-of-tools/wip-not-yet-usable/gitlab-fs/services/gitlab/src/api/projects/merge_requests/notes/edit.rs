// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Edit a merge request note on a project.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct EditMergeRequestNote<'a> {
    /// The project to add the merge request to.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The internal IID of the merge request.
    merge_request: u64,
    /// The ID of the note.
    note: u64,

    /// The content of the note.
    #[builder(setter(into))]
    body: Cow<'a, str>,
    /// The SHA of the head of the merge request.
    ///
    /// Required when using the `/merge` command to verify that the intended commit is being
    /// merged.
    #[builder(setter(into), default)]
    merge_request_diff_sha: Option<Cow<'a, str>>,
}

impl<'a> EditMergeRequestNote<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> EditMergeRequestNoteBuilder<'a> {
        EditMergeRequestNoteBuilder::default()
    }
}

impl<'a> Endpoint for EditMergeRequestNote<'a> {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/merge_requests/{}/notes/{}",
            self.project, self.merge_request, self.note,
        )
        .into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params.push("body", self.body.as_ref()).push_opt(
            "merge_request_diff_sha",
            self.merge_request_diff_sha.as_ref(),
        );

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::merge_requests::notes::{
        EditMergeRequestNote, EditMergeRequestNoteBuilderError,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_merge_request_note_and_body_are_necessary() {
        let err = EditMergeRequestNote::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, EditMergeRequestNoteBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = EditMergeRequestNote::builder()
            .merge_request(1)
            .note(1)
            .body("body")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, EditMergeRequestNoteBuilderError, "project");
    }

    #[test]
    fn merge_request_is_necessary() {
        let err = EditMergeRequestNote::builder()
            .project(1)
            .note(1)
            .body("body")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, EditMergeRequestNoteBuilderError, "merge_request");
    }

    #[test]
    fn note_is_necessary() {
        let err = EditMergeRequestNote::builder()
            .project(1)
            .merge_request(1)
            .body("body")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, EditMergeRequestNoteBuilderError, "note");
    }

    #[test]
    fn body_is_necessary() {
        let err = EditMergeRequestNote::builder()
            .project(1)
            .merge_request(1)
            .note(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, EditMergeRequestNoteBuilderError, "body");
    }

    #[test]
    fn project_merge_request_note_and_body_are_sufficient() {
        EditMergeRequestNote::builder()
            .project(1)
            .merge_request(1)
            .note(1)
            .body("body")
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/merge_requests/1/notes/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("body=body")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditMergeRequestNote::builder()
            .project("simple/project")
            .merge_request(1)
            .note(1)
            .body("body")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_merge_request_diff_sha() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/merge_requests/1/notes/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "body=body",
                "&merge_request_diff_sha=deadbeefdeadbeefdeadbeefdeadbeefdeadbeef",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditMergeRequestNote::builder()
            .project("simple/project")
            .merge_request(1)
            .note(1)
            .body("body")
            .merge_request_diff_sha("deadbeefdeadbeefdeadbeefdeadbeefdeadbeef")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
