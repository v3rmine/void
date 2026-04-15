use std::str;

use derive_builder::Builder;

use crate::api::common::{self, NameOrId};
use crate::api::endpoint_prelude::*;

/// Create a new file in a project.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct DeleteFile<'a> {
    /// The project to create a file within.
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
    /// The commit message to use.
    #[builder(setter(into))]
    commit_message: Cow<'a, str>,

    /// Where to start the branch from (if it doesn't already exist).
    #[builder(setter(into), default)]
    start_branch: Option<Cow<'a, str>>,
    /// The email of the author for the new commit.
    #[builder(setter(into), default)]
    author_email: Option<Cow<'a, str>>,
    /// The name of the author for the new commit.
    #[builder(setter(into), default)]
    author_name: Option<Cow<'a, str>>,
    /// The last commit ID the path was modified in.
    ///
    /// Used to detect conflicts.
    #[builder(setter(into), default)]
    last_commit_id: Option<Cow<'a, str>>,
}

impl<'a> DeleteFile<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> DeleteFileBuilder<'a> {
        DeleteFileBuilder::default()
    }
}

impl<'a> Endpoint for DeleteFile<'a> {
    fn method(&self) -> Method {
        Method::DELETE
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
            .push_opt("last_commit_id", self.last_commit_id.as_ref());

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::repository::files::{DeleteFile, DeleteFileBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn all_parameters_are_needed() {
        let err = DeleteFile::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, DeleteFileBuilderError, "project");
    }

    #[test]
    fn project_is_required() {
        let err = DeleteFile::builder()
            .file_path("new/file")
            .branch("master")
            .commit_message("commit message")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, DeleteFileBuilderError, "project");
    }

    #[test]
    fn file_path_is_required() {
        let err = DeleteFile::builder()
            .project(1)
            .branch("master")
            .commit_message("commit message")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, DeleteFileBuilderError, "file_path");
    }

    #[test]
    fn branch_is_required() {
        let err = DeleteFile::builder()
            .project(1)
            .file_path("new/file")
            .commit_message("commit message")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, DeleteFileBuilderError, "branch");
    }

    #[test]
    fn commit_message_is_required() {
        let err = DeleteFile::builder()
            .project(1)
            .file_path("new/file")
            .branch("master")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, DeleteFileBuilderError, "commit_message");
    }

    #[test]
    fn sufficient_parameters() {
        DeleteFile::builder()
            .project(1)
            .file_path("new/file")
            .branch("master")
            .commit_message("commit message")
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::DELETE)
            .endpoint("projects/simple%2Fproject/repository/files/path%2Fto%2Ffile")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("branch=branch", "&commit_message=commit+message"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = DeleteFile::builder()
            .project("simple/project")
            .file_path("path/to/file")
            .branch("branch")
            .commit_message("commit message")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_start_branch() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::DELETE)
            .endpoint("projects/simple%2Fproject/repository/files/path%2Fto%2Ffile")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "branch=branch",
                "&commit_message=commit+message",
                "&start_branch=master",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = DeleteFile::builder()
            .project("simple/project")
            .file_path("path/to/file")
            .branch("branch")
            .commit_message("commit message")
            .start_branch("master")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_author_email() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::DELETE)
            .endpoint("projects/simple%2Fproject/repository/files/path%2Fto%2Ffile")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "branch=branch",
                "&commit_message=commit+message",
                "&author_email=author%40email.invalid",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = DeleteFile::builder()
            .project("simple/project")
            .file_path("path/to/file")
            .branch("branch")
            .commit_message("commit message")
            .author_email("author@email.invalid")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_author_name() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::DELETE)
            .endpoint("projects/simple%2Fproject/repository/files/path%2Fto%2Ffile")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "branch=branch",
                "&commit_message=commit+message",
                "&author_name=Arthur+Developer",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = DeleteFile::builder()
            .project("simple/project")
            .file_path("path/to/file")
            .branch("branch")
            .commit_message("commit message")
            .author_name("Arthur Developer")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_last_commit_id() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::DELETE)
            .endpoint("projects/simple%2Fproject/repository/files/path%2Fto%2Ffile")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "branch=branch",
                "&commit_message=commit+message",
                "&last_commit_id=deadbeefdeadbeefdeadbeefdeadbeefdeadbeef",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = DeleteFile::builder()
            .project("simple/project")
            .file_path("path/to/file")
            .branch("branch")
            .commit_message("commit message")
            .last_commit_id("deadbeefdeadbeefdeadbeefdeadbeefdeadbeef")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
