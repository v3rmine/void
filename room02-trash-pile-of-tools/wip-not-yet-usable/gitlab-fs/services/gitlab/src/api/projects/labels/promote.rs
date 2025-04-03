// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Promote a label within a project to a group label.
#[derive(Debug, Clone, Builder)]
pub struct PromoteLabel<'a> {
    /// The project to promote a label within.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The ID or title of the label.
    #[builder(setter(into))]
    label: NameOrId<'a>,
}

impl<'a> PromoteLabel<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> PromoteLabelBuilder<'a> {
        PromoteLabelBuilder::default()
    }
}

impl<'a> Endpoint for PromoteLabel<'a> {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/labels/{}/promote", self.project, self.label).into()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::labels::{PromoteLabel, PromoteLabelBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_label_are_necessary() {
        let err = PromoteLabel::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, PromoteLabelBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = PromoteLabel::builder().label("label").build().unwrap_err();
        crate::test::assert_missing_field!(err, PromoteLabelBuilderError, "project");
    }

    #[test]
    fn label_is_necessary() {
        let err = PromoteLabel::builder().project(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, PromoteLabelBuilderError, "label");
    }

    #[test]
    fn project_and_label_are_sufficient() {
        PromoteLabel::builder()
            .project(1)
            .label("label")
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/labels/simple%2Flabel/promote")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = PromoteLabel::builder()
            .project("simple/project")
            .label("simple/label")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
