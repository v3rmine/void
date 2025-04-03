// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::{self, NameOrId};
use crate::api::endpoint_prelude::*;

/// Query a protected branch of a project.
#[derive(Debug, Clone, Builder)]
pub struct ProtectedBranch<'a> {
    /// The project to query for the protected branch.
    #[builder(setter(into))]
    project: NameOrId<'a>,

    /// The name of the branch or wildcard.
    #[builder(setter(into))]
    name: Cow<'a, str>,
}

impl<'a> ProtectedBranch<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> ProtectedBranchBuilder<'a> {
        ProtectedBranchBuilder::default()
    }
}

impl<'a> Endpoint for ProtectedBranch<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/protected_branches/{}",
            self.project,
            common::path_escaped(&self.name)
        )
        .into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        QueryParams::default()
    }
}

#[cfg(test)]
mod tests {
    use crate::api::projects::protected_branches::{ProtectedBranch, ProtectedBranchBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_is_needed() {
        let err = ProtectedBranch::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, ProtectedBranchBuilderError, "project");
    }

    #[test]
    fn name_is_needed() {
        let err = ProtectedBranch::builder()
            .project("project_name")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, ProtectedBranchBuilderError, "name");
    }

    #[test]
    fn project_and_name_are_sufficient() {
        ProtectedBranch::builder()
            .project(1)
            .name("master")
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/group%2Fproject/protected_branches/master")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProtectedBranch::builder()
            .project("group/project")
            .name("master")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
