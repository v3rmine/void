// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Vaild archive formats for getting the archive of a repository.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ArchiveFormat {
    /// `.tar.gz` format.
    TarGz,
    /// `.tar.bz2` format.
    TarBz2,
    /// `.tbz` format.
    Tbz,
    /// `.tbz2` format.
    Tbz2,
    /// `.tb2` format.
    Tb2,
    /// `.bz2` format.
    Bz2,
    /// `.tar` format.
    Tar,
    /// `.zip` format.
    Zip,
}

/// Get the archive of a repository.
///
/// Note: This endpoint returns raw data, so [`crate::api::raw`] is recommended to avoid the normal
/// JSON parsing present in the typical endpoint handling.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Archive<'a> {
    /// The ID or URL-encoded path of the project.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The archive format to request. Defaults to tar.gz if unspecified.
    #[builder(default)]
    format: Option<ArchiveFormat>,
    /// The commit SHA to get. Defaults to the tip of the default branch if unspecified.
    #[builder(setter(into), default)]
    sha: Option<Cow<'a, str>>,
    /// The subpath of the repository to download.
    #[builder(setter(into), default)]
    path: Option<Cow<'a, str>>,
}

impl<'a> Archive<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> ArchiveBuilder<'a> {
        ArchiveBuilder::default()
    }
}

impl<'a> Endpoint for Archive<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        let archive_format = self.format.map_or("", |f| f.as_str());
        format!(
            "projects/{}/repository/archive{}",
            self.project, archive_format,
        )
        .into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();

        params
            .push_opt("sha", self.sha.as_ref())
            .push_opt("path", self.path.as_ref());

        params
    }
}

impl ArchiveFormat {
    /// Get the corresponding file extension for this archive format.
    ///
    /// The extension includes a leading dot.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::TarGz => ".tar.gz",
            Self::TarBz2 => ".tar.bz2",
            Self::Tbz => ".tbz",
            Self::Tbz2 => ".tbz2",
            Self::Tb2 => ".tb2",
            Self::Bz2 => ".bz2",
            Self::Tar => ".tar",
            Self::Zip => ".zip",
        }
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::repository::archive::{Archive, ArchiveBuilderError, ArchiveFormat};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn archive_format_as_str() {
        let items = &[
            (ArchiveFormat::TarGz, ".tar.gz"),
            (ArchiveFormat::TarBz2, ".tar.bz2"),
            (ArchiveFormat::Tbz, ".tbz"),
            (ArchiveFormat::Tbz2, ".tbz2"),
            (ArchiveFormat::Tb2, ".tb2"),
            (ArchiveFormat::Bz2, ".bz2"),
            (ArchiveFormat::Tar, ".tar"),
            (ArchiveFormat::Zip, ".zip"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn project_is_necessary() {
        let err = Archive::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, ArchiveBuilderError, "project");
    }

    #[test]
    fn project_is_sufficient() {
        Archive::builder().project(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::GET)
            .endpoint("projects/simple%2Fproject/repository/archive")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Archive::builder()
            .project("simple/project")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_format() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::GET)
            .endpoint("projects/simple%2Fproject/repository/archive.tar.gz")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Archive::builder()
            .project("simple/project")
            .format(ArchiveFormat::TarGz)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_sha() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::GET)
            .endpoint("projects/simple%2Fproject/repository/archive")
            .add_query_params(&[("sha", "0123456789abcdef0123456789abcdef01234567")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Archive::builder()
            .project("simple/project")
            .sha("0123456789abcdef0123456789abcdef01234567")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_path() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::GET)
            .endpoint("projects/simple%2Fproject/repository/archive")
            .add_query_params(&[("path", "some/sub/path")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Archive::builder()
            .project("simple/project")
            .path("some/sub/path")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
