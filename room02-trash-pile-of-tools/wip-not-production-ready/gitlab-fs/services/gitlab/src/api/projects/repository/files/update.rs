// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::str;

use derive_builder::Builder;
use log::warn;

use crate::api::common::{self, NameOrId};
use crate::api::endpoint_prelude::*;
use crate::api::projects::repository::files::Encoding;

/// Update a file in a project.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct UpdateFile<'a> {
    /// The project to update a file within.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The path to the file in the repository.
    ///
    /// This is automatically escaped as needed.
    #[builder(setter(into))]
    file_path: Cow<'a, str>,
    /// The branch to use for the new commit.
    #[builder(setter(into))]
    branch: Cow<'a, str>,
    /// The content of the new file.
    ///
    /// This will automatically be encoded according to the `encoding` parameter.
    #[builder(setter(into))]
    content: Cow<'a, [u8]>,
    /// The commit message to use.
    #[builder(setter(into))]
    commit_message: Cow<'a, str>,

    /// Where to start the branch from (if it doesn't already exist).
    #[builder(setter(into), default)]
    start_branch: Option<Cow<'a, str>>,
    /// The encoding to use for the content.
    ///
    /// Note that if `text` is requested and `content` contains non-UTF-8 content, a warning will
    /// be generated and a binary-safe encoding used instead.
    #[builder(default)]
    encoding: Option<Encoding>,
    /// The email of the author for the new commit.
    #[builder(setter(into), default)]
    author_email: Option<Cow<'a, str>>,
    /// The name of the author for the new commit.
    #[builder(setter(into), default)]
    author_name: Option<Cow<'a, str>>,
    /// Whether the file is executable or not.
    #[builder(default)]
    execute_filemode: Option<bool>,
    /// The last commit ID the path was modified in.
    ///
    /// Used to detect conflicts.
    #[builder(setter(into), default)]
    last_commit_id: Option<Cow<'a, str>>,
}

impl<'a> UpdateFile<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> UpdateFileBuilder<'a> {
        UpdateFileBuilder::default()
    }
}

const SAFE_ENCODING: Encoding = Encoding::Base64;

impl<'a> Endpoint for UpdateFile<'a> {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/repository/files/{}",
            self.project,
            common::path_escaped(&self.file_path),
        )
        .into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params
            .push("branch", &self.branch)
            .push("commit_message", &self.commit_message)
            .push_opt("start_branch", self.start_branch.as_ref())
            .push_opt("author_email", self.author_email.as_ref())
            .push_opt("author_name", self.author_name.as_ref())
            .push_opt("execute_filemode", self.execute_filemode)
            .push_opt("last_commit_id", self.last_commit_id.as_ref());

        let content = str::from_utf8(&self.content);
        let needs_encoding = content.is_err();
        let encoding = self.encoding.unwrap_or_default();
        let actual_encoding = if needs_encoding && !encoding.is_binary_safe() {
            warn!(
                "forcing the encoding to {} due to utf-8 unsafe content",
                SAFE_ENCODING.as_str(),
            );
            SAFE_ENCODING
        } else {
            encoding
        };
        params.push(
            "content",
            actual_encoding.encode(content.ok(), &self.content),
        );

        if let Some(value) = self
            .encoding
            // Use the actual encoding.
            .map(|_| actual_encoding)
            // Force the encoding if we're not using the default.
            .or_else(|| {
                if actual_encoding != Encoding::default() {
                    Some(actual_encoding)
                } else {
                    None
                }
            })
        {
            params.push("encoding", value);
        }

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::repository::files::{Encoding, UpdateFile, UpdateFileBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn all_parameters_are_needed() {
        let err = UpdateFile::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, UpdateFileBuilderError, "project");
    }

    #[test]
    fn project_is_required() {
        let err = UpdateFile::builder()
            .file_path("new/file")
            .branch("master")
            .commit_message("commit message")
            .content(&b"contents"[..])
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, UpdateFileBuilderError, "project");
    }

    #[test]
    fn file_path_is_required() {
        let err = UpdateFile::builder()
            .project(1)
            .branch("master")
            .commit_message("commit message")
            .content(&b"contents"[..])
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, UpdateFileBuilderError, "file_path");
    }

    #[test]
    fn branch_is_required() {
        let err = UpdateFile::builder()
            .project(1)
            .file_path("new/file")
            .commit_message("commit message")
            .content(&b"contents"[..])
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, UpdateFileBuilderError, "branch");
    }

    #[test]
    fn commit_message_is_required() {
        let err = UpdateFile::builder()
            .project(1)
            .file_path("new/file")
            .branch("master")
            .content(&b"contents"[..])
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, UpdateFileBuilderError, "commit_message");
    }

    #[test]
    fn content_is_required() {
        let err = UpdateFile::builder()
            .project(1)
            .file_path("new/file")
            .branch("master")
            .commit_message("commit message")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, UpdateFileBuilderError, "content");
    }

    #[test]
    fn sufficient_parameters() {
        UpdateFile::builder()
            .project(1)
            .file_path("new/file")
            .branch("master")
            .commit_message("commit message")
            .content(&b"contents"[..])
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/repository/files/path%2Fto%2Ffile")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "branch=branch",
                "&commit_message=commit+message",
                "&content=file+contents",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = UpdateFile::builder()
            .project("simple/project")
            .file_path("path/to/file")
            .branch("branch")
            .content(&b"file contents"[..])
            .commit_message("commit message")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_start_branch() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/repository/files/path%2Fto%2Ffile")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "branch=branch",
                "&commit_message=commit+message",
                "&start_branch=master",
                "&content=file+contents",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = UpdateFile::builder()
            .project("simple/project")
            .file_path("path/to/file")
            .branch("branch")
            .content(&b"file contents"[..])
            .commit_message("commit message")
            .start_branch("master")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_encoding() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/repository/files/path%2Fto%2Ffile")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "branch=branch",
                "&commit_message=commit+message",
                "&content=ZmlsZSBjb250ZW50cw%3D%3D",
                "&encoding=base64",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = UpdateFile::builder()
            .project("simple/project")
            .file_path("path/to/file")
            .branch("branch")
            .content(&b"file contents"[..])
            .commit_message("commit message")
            .encoding(Encoding::Base64)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_encoding_upgrade() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/repository/files/path%2Fto%2Ffile")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "branch=branch",
                "&commit_message=commit+message",
                "&content=%2Fw%3D%3D",
                "&encoding=base64",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = UpdateFile::builder()
            .project("simple/project")
            .file_path("path/to/file")
            .branch("branch")
            .content(&b"\xff"[..])
            .commit_message("commit message")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_author_email() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/repository/files/path%2Fto%2Ffile")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "branch=branch",
                "&commit_message=commit+message",
                "&author_email=author%40email.invalid",
                "&content=file+contents",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = UpdateFile::builder()
            .project("simple/project")
            .file_path("path/to/file")
            .branch("branch")
            .content(&b"file contents"[..])
            .commit_message("commit message")
            .author_email("author@email.invalid")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_author_name() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/repository/files/path%2Fto%2Ffile")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "branch=branch",
                "&commit_message=commit+message",
                "&author_name=Arthur+Developer",
                "&content=file+contents",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = UpdateFile::builder()
            .project("simple/project")
            .file_path("path/to/file")
            .branch("branch")
            .content(&b"file contents"[..])
            .commit_message("commit message")
            .author_name("Arthur Developer")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_execute_filemode() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/repository/files/path%2Fto%2Ffile")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "branch=branch",
                "&commit_message=commit+message",
                "&execute_filemode=true",
                "&content=file+contents",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = UpdateFile::builder()
            .project("simple/project")
            .file_path("path/to/file")
            .branch("branch")
            .content(&b"file contents"[..])
            .commit_message("commit message")
            .execute_filemode(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_last_commit_id() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/repository/files/path%2Fto%2Ffile")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "branch=branch",
                "&commit_message=commit+message",
                "&last_commit_id=deadbeefdeadbeefdeadbeefdeadbeefdeadbeef",
                "&content=file+contents",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = UpdateFile::builder()
            .project("simple/project")
            .file_path("path/to/file")
            .branch("branch")
            .content(&b"file contents"[..])
            .commit_message("commit message")
            .last_commit_id("deadbeefdeadbeefdeadbeefdeadbeefdeadbeef")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
