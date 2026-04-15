// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Unshare a project with a group.
#[derive(Debug, Builder, Clone)]
pub struct UnshareProject<'a> {
    /// The project to add the user to.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The group to remove from the project.
    group_id: u64,
}

impl<'a> UnshareProject<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> UnshareProjectBuilder<'a> {
        UnshareProjectBuilder::default()
    }
}

impl<'a> Endpoint for UnshareProject<'a> {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/share/{}", self.project, self.group_id).into()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::{UnshareProject, UnshareProjectBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn all_parameters_are_needed() {
        let err = UnshareProject::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, UnshareProjectBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = UnshareProject::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, UnshareProjectBuilderError, "project");
    }

    #[test]
    fn sufficient_parameters() {
        let err = UnshareProject::builder()
            .project("project")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, UnshareProjectBuilderError, "group_id");
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::DELETE)
            .endpoint("projects/project%2Fsubproject/share/1")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = UnshareProject::builder()
            .project("project/subproject")
            .group_id(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
