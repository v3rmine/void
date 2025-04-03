// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Create a tag on a project.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct CreateTag<'a> {
    /// The project to create a tag on.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The name of the new tag.
    #[builder(setter(into))]
    tag_name: Cow<'a, str>,
    /// The ref to create the tag from.
    #[builder(setter(into))]
    ref_: Cow<'a, str>,
    /// Creates an annotated tag, if present.
    #[builder(setter(into), default)]
    message: Option<Cow<'a, str>>,
}

impl<'a> CreateTag<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> CreateTagBuilder<'a> {
        CreateTagBuilder::default()
    }
}

impl<'a> Endpoint for CreateTag<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/repository/tags", self.project).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params
            .push("tag_name", &self.tag_name)
            .push("ref", &self.ref_)
            .push_opt("message", self.message.as_ref());

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::repository::tags::{CreateTag, CreateTagBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_is_necessary() {
        let err = CreateTag::builder()
            .tag_name("tag")
            .ref_("ref")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateTagBuilderError, "project");
    }

    #[test]
    fn tag_name_is_necessary() {
        let err = CreateTag::builder()
            .project(1)
            .ref_("ref")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateTagBuilderError, "tag_name");
    }

    #[test]
    fn ref_is_necessary() {
        let err = CreateTag::builder()
            .project(1)
            .tag_name("a-tag-name")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateTagBuilderError, "ref_");
    }

    #[test]
    fn project_tag_name_and_ref_is_sufficient() {
        CreateTag::builder()
            .project(1)
            .tag_name("a-tag-name")
            .ref_("0000000000000000000000000000000000000000")
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/repository/tags")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "tag_name=a-tag",
                "&ref=0000000000000000000000000000000000000000",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateTag::builder()
            .project("simple/project")
            .tag_name("a-tag")
            .ref_("0000000000000000000000000000000000000000")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_message() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/repository/tags")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "tag_name=a-tag",
                "&ref=0000000000000000000000000000000000000000",
                "&message=Hi+there",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateTag::builder()
            .project("simple/project")
            .tag_name("a-tag")
            .ref_("0000000000000000000000000000000000000000")
            .message("Hi there")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
