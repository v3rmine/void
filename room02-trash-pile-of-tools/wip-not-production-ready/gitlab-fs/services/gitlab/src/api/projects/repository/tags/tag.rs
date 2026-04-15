// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::{self, NameOrId};
use crate::api::endpoint_prelude::*;

/// Query for a specific branch in a project.
#[derive(Debug, Builder, Clone)]
pub struct Tag<'a> {
    /// The project to get a atg from.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The tag to get.
    #[builder(setter(into))]
    tag_name: Cow<'a, str>,
}

impl<'a> Tag<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> TagBuilder<'a> {
        TagBuilder::default()
    }
}

impl<'a> Endpoint for Tag<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/repository/tags/{}",
            self.project,
            common::path_escaped(&self.tag_name),
        )
        .into()
    }
}

#[cfg(test)]
mod tests {
    use crate::api::projects::repository::tags::tag::{Tag, TagBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_tag_are_necessary() {
        let err = Tag::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, TagBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = Tag::builder().tag_name("master").build().unwrap_err();
        crate::test::assert_missing_field!(err, TagBuilderError, "project");
    }

    #[test]
    fn tag_is_necessary() {
        let err = Tag::builder().project(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, TagBuilderError, "tag_name");
    }

    #[test]
    fn project_and_branch_are_sufficient() {
        Tag::builder().project(1).tag_name("a-tag").build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/repository/tags/a-tag")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Tag::builder()
            .project("simple/project")
            .tag_name("a-tag")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
