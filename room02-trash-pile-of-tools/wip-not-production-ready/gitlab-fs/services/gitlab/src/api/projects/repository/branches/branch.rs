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
pub struct Branch<'a> {
    /// The project to get a branch from.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The branch to get.
    #[builder(setter(into))]
    branch: Cow<'a, str>,
}

impl<'a> Branch<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> BranchBuilder<'a> {
        BranchBuilder::default()
    }
}

impl<'a> Endpoint for Branch<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/repository/branches/{}",
            self.project,
            common::path_escaped(&self.branch),
        )
        .into()
    }
}

#[cfg(test)]
mod tests {
    use crate::api::projects::repository::branches::{Branch, BranchBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_branch_are_necessary() {
        let err = Branch::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, BranchBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = Branch::builder().branch("master").build().unwrap_err();
        crate::test::assert_missing_field!(err, BranchBuilderError, "project");
    }

    #[test]
    fn branch_is_necessary() {
        let err = Branch::builder().project(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, BranchBuilderError, "branch");
    }

    #[test]
    fn project_and_branch_are_sufficient() {
        Branch::builder()
            .project(1)
            .branch("master")
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/repository/branches/special%2Fbranch")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Branch::builder()
            .project("simple/project")
            .branch("special/branch")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
