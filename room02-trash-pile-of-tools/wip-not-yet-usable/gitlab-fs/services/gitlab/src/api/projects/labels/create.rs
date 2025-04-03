// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Create a label within a project.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct CreateLabel<'a> {
    /// The project to create a label within.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The name of the label.
    #[builder(setter(into))]
    name: Cow<'a, str>,
    /// The color of the label.
    ///
    /// CSS and RGB colors in `#RRGGBB` format are supported.
    ///
    /// TODO: Use a specific structure for this.
    #[builder(setter(into))]
    color: Cow<'a, str>,

    /// The description of the label.
    #[builder(setter(into), default)]
    description: Option<Cow<'a, str>>,
    /// The priority of the label.
    #[builder(default)]
    priority: Option<u64>,
}

impl<'a> CreateLabel<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> CreateLabelBuilder<'a> {
        CreateLabelBuilder::default()
    }
}

impl<'a> Endpoint for CreateLabel<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/labels", self.project).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params
            .push("name", &self.name)
            .push("color", &self.color)
            .push_opt("description", self.description.as_ref())
            .push_opt("priority", self.priority);

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::labels::{CreateLabel, CreateLabelBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_name_and_color_are_necessary() {
        let err = CreateLabel::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, CreateLabelBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = CreateLabel::builder()
            .name("label")
            .color("#f100fe")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateLabelBuilderError, "project");
    }

    #[test]
    fn name_is_necessary() {
        let err = CreateLabel::builder()
            .project(1)
            .color("#f100fe")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateLabelBuilderError, "name");
    }

    #[test]
    fn color_is_necessary() {
        let err = CreateLabel::builder()
            .project(1)
            .name("label")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateLabelBuilderError, "color");
    }

    #[test]
    fn project_name_and_color_are_sufficient() {
        CreateLabel::builder()
            .project(1)
            .name("label")
            .color("#f100fe")
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/labels")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=label", "&color=%23ffffff"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateLabel::builder()
            .project("simple/project")
            .name("label")
            .color("#ffffff")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_description() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/labels")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "name=label",
                "&color=%23ffffff",
                "&description=description",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateLabel::builder()
            .project("simple/project")
            .name("label")
            .color("#ffffff")
            .description("description")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_priority() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/labels")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=label", "&color=%23ffffff", "&priority=1"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateLabel::builder()
            .project("simple/project")
            .name("label")
            .color("#ffffff")
            .priority(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
