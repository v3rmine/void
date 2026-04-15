// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Delete a label within a project.
#[derive(Debug, Clone, Builder)]
pub struct DeleteLabel<'a> {
    /// The project to delete a label within.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The ID or title of the label.
    #[builder(setter(into))]
    label: NameOrId<'a>,
}

impl<'a> DeleteLabel<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> DeleteLabelBuilder<'a> {
        DeleteLabelBuilder::default()
    }
}

impl<'a> Endpoint for DeleteLabel<'a> {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/labels/{}", self.project, self.label).into()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::labels::{DeleteLabel, DeleteLabelBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_label_are_necessary() {
        let err = DeleteLabel::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, DeleteLabelBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = DeleteLabel::builder().label("label").build().unwrap_err();
        crate::test::assert_missing_field!(err, DeleteLabelBuilderError, "project");
    }

    #[test]
    fn label_is_necessary() {
        let err = DeleteLabel::builder().project(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, DeleteLabelBuilderError, "label");
    }

    #[test]
    fn project_and_label_are_sufficient() {
        DeleteLabel::builder()
            .project(1)
            .label("label")
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::DELETE)
            .endpoint("projects/simple%2Fproject/labels/simple%2Flabel")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = DeleteLabel::builder()
            .project("simple/project")
            .label("simple/label")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
