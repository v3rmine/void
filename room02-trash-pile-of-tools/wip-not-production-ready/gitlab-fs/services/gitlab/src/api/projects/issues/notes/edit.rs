// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Edit an issue note on a project.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct EditIssueNote<'a> {
    /// The project to add the issue to.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The internal IID of the issue.
    issue: u64,
    /// The ID of the note.
    note: u64,

    /// The content of the note.
    #[builder(setter(into))]
    body: Cow<'a, str>,
    /// The confidential flag of the note.
    #[builder(default)]
    confidential: Option<bool>,
}

impl<'a> EditIssueNote<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> EditIssueNoteBuilder<'a> {
        EditIssueNoteBuilder::default()
    }
}

impl<'a> Endpoint for EditIssueNote<'a> {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/issues/{}/notes/{}",
            self.project, self.issue, self.note,
        )
        .into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params
            .push("body", self.body.as_ref())
            .push_opt("confidential", self.confidential);

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::issues::notes::{EditIssueNote, EditIssueNoteBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_issue_note_and_body_are_necessary() {
        let err = EditIssueNote::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, EditIssueNoteBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = EditIssueNote::builder()
            .issue(1)
            .note(1)
            .body("body")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, EditIssueNoteBuilderError, "project");
    }

    #[test]
    fn issue_is_necessary() {
        let err = EditIssueNote::builder()
            .project(1)
            .note(1)
            .body("body")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, EditIssueNoteBuilderError, "issue");
    }

    #[test]
    fn note_is_necessary() {
        let err = EditIssueNote::builder()
            .project(1)
            .issue(1)
            .body("body")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, EditIssueNoteBuilderError, "note");
    }

    #[test]
    fn body_is_necessary() {
        let err = EditIssueNote::builder()
            .project(1)
            .issue(1)
            .note(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, EditIssueNoteBuilderError, "body");
    }

    #[test]
    fn project_issue_note_and_body_are_sufficient() {
        EditIssueNote::builder()
            .project(1)
            .issue(1)
            .note(1)
            .body("body")
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/issues/1/notes/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("body=body")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditIssueNote::builder()
            .project("simple/project")
            .issue(1)
            .note(1)
            .body("body")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_confidential() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/issues/1/notes/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("body=body&confidential=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditIssueNote::builder()
            .project("simple/project")
            .issue(1)
            .note(1)
            .body("body")
            .confidential(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
