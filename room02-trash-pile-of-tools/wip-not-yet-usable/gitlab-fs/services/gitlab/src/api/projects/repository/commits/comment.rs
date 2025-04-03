// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::{self, NameOrId};
use crate::api::endpoint_prelude::*;
use crate::api::ParamValue;

/// Line types within a diff.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineType {
    /// A line added in the diff.
    New,
    /// A line removed in the diff.
    Old,
}

impl LineType {
    fn as_str(self) -> &'static str {
        match self {
            LineType::New => "new",
            LineType::Old => "old",
        }
    }
}

impl ParamValue<'static> for LineType {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Post a comment on a specific commit in a project.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct CommentOnCommit<'a> {
    /// The project to get a commit from.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The commit to comment on.
    #[builder(setter(into))]
    commit: Cow<'a, str>,
    /// The text of the comment.
    #[builder(setter(into))]
    note: Cow<'a, str>,

    /// The path to comment on.
    #[builder(setter(into), default)]
    path: Option<Cow<'a, str>>,
    /// The line within the path to comment on.
    #[builder(default)]
    line: Option<u64>,
    /// Set the line type to comment on.
    #[builder(default)]
    line_type: Option<LineType>,
}

impl<'a> CommentOnCommit<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> CommentOnCommitBuilder<'a> {
        CommentOnCommitBuilder::default()
    }
}

impl<'a> Endpoint for CommentOnCommit<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/repository/commits/{}/comments",
            self.project,
            common::path_escaped(&self.commit),
        )
        .into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params
            .push("note", &self.note)
            .push_opt("path", self.path.as_ref())
            .push_opt("line", self.line)
            .push_opt("line_type", self.line_type);

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::repository::commits::{
        CommentOnCommit, CommentOnCommitBuilderError, LineType,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn line_type_as_str() {
        let items = &[(LineType::New, "new"), (LineType::Old, "old")];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn project_commit_and_note_are_necessary() {
        let err = CommentOnCommit::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, CommentOnCommitBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = CommentOnCommit::builder()
            .commit("master")
            .note("note")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CommentOnCommitBuilderError, "project");
    }

    #[test]
    fn commit_is_necessary() {
        let err = CommentOnCommit::builder()
            .project(1)
            .note("note")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CommentOnCommitBuilderError, "commit");
    }

    #[test]
    fn note_is_necessary() {
        let err = CommentOnCommit::builder()
            .project(1)
            .commit("master")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CommentOnCommitBuilderError, "note");
    }

    #[test]
    fn project_commit_and_note_are_sufficient() {
        CommentOnCommit::builder()
            .project(1)
            .commit("master")
            .note("note")
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/repository/commits/0000000000000000000000000000000000000000/comments")
            .content_type("application/x-www-form-urlencoded")
            .body_str("note=comment+content")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CommentOnCommit::builder()
            .project("simple/project")
            .commit("0000000000000000000000000000000000000000")
            .note("comment content")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_path() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/repository/commits/0000000000000000000000000000000000000000/comments")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "note=comment+content",
                "&path=path%2Fto%2Ffile",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CommentOnCommit::builder()
            .project("simple/project")
            .commit("0000000000000000000000000000000000000000")
            .note("comment content")
            .path("path/to/file")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_line() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/repository/commits/0000000000000000000000000000000000000000/comments")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "note=comment+content",
                "&line=1",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CommentOnCommit::builder()
            .project("simple/project")
            .commit("0000000000000000000000000000000000000000")
            .note("comment content")
            .line(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_line_type() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/repository/commits/0000000000000000000000000000000000000000/comments")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "note=comment+content",
                "&line_type=new",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CommentOnCommit::builder()
            .project("simple/project")
            .commit("0000000000000000000000000000000000000000")
            .note("comment content")
            .line_type(LineType::New)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
