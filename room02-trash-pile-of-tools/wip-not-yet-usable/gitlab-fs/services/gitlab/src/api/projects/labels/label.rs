// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Query for a label within a project.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Label<'a> {
    /// The project to query for the label.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The ID or title of the label.
    #[builder(setter(into))]
    label: NameOrId<'a>,

    /// Include ancestor groups.
    ///
    /// Defaults to `true`.
    #[builder(default)]
    include_ancestor_groups: Option<bool>,
}

impl<'a> Label<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> LabelBuilder<'a> {
        LabelBuilder::default()
    }
}

impl<'a> Endpoint for Label<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/labels/{}", self.project, self.label).into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();

        params.push_opt("include_ancestor_groups", self.include_ancestor_groups);

        params
    }
}

#[cfg(test)]
mod tests {
    use crate::api::projects::labels::{Label, LabelBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_label_are_needed() {
        let err = Label::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, LabelBuilderError, "project");
    }

    #[test]
    fn project_is_needed() {
        let err = Label::builder().label(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, LabelBuilderError, "project");
    }

    #[test]
    fn label_is_needed() {
        let err = Label::builder().project(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, LabelBuilderError, "label");
    }

    #[test]
    fn project_and_label_are_sufficient() {
        Label::builder().project(1).label(1).build().unwrap();
    }

    #[test]
    fn label_by_id() {
        Label::builder().project(1).label(1).build().unwrap();
    }

    #[test]
    fn label_by_name() {
        Label::builder()
            .project(1)
            .label("label_name")
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/labels/1")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Label::builder()
            .project("simple/project")
            .label(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_include_ancestor_groups() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/labels/1")
            .add_query_params(&[("include_ancestor_groups", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Label::builder()
            .project("simple/project")
            .label(1)
            .include_ancestor_groups(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_escapes_label_name() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/labels/simple%2Flabel")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Label::builder()
            .project("simple/project")
            .label("simple/label")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
