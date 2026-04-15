// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Get the tree of a given path.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Tree<'a> {
    /// The ID or URL-encoded path of the project.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The path inside repository. Used to get content of subdirectories.
    #[builder(setter(into), default)]
    path: Option<Cow<'a, str>>,
    /// The name of a repository branch or tag or, if not given, the default branch.
    #[builder(setter(into), default)]
    ref_: Option<Cow<'a, str>>,
    /// Boolean value used to get a recursive tree (false by default).
    #[builder(default)]
    recursive: Option<bool>,
}

impl<'a> Tree<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> TreeBuilder<'a> {
        TreeBuilder::default()
    }
}

impl<'a> Endpoint for Tree<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/repository/tree", self.project).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params
            .push_opt("path", self.path.as_ref())
            .push_opt("ref", self.ref_.as_ref())
            .push_opt("recursive", self.recursive);

        params.into_body()
    }
}

impl<'a> Pageable for Tree<'a> {
    fn use_keyset_pagination(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::repository::tree::{Tree, TreeBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_is_necessary() {
        let err = Tree::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, TreeBuilderError, "project");
    }

    #[test]
    fn project_is_sufficient() {
        Tree::builder().project(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::GET)
            .endpoint("projects/simple%2Fproject/repository/tree")
            .content_type("application/x-www-form-urlencoded")
            .body_str("")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Tree::builder().project("simple/project").build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_path() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::GET)
            .endpoint("projects/simple%2Fproject/repository/tree")
            .content_type("application/x-www-form-urlencoded")
            .body_str("path=path%2Fto%2Ffile")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Tree::builder()
            .project("simple/project")
            .path("path/to/file")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_ref() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::GET)
            .endpoint("projects/simple%2Fproject/repository/tree")
            .content_type("application/x-www-form-urlencoded")
            .body_str("ref=123")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Tree::builder()
            .project("simple/project")
            .ref_("123")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_recursive() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::GET)
            .endpoint("projects/simple%2Fproject/repository/tree")
            .content_type("application/x-www-form-urlencoded")
            .body_str("recursive=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Tree::builder()
            .project("simple/project")
            .recursive(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
