// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Query protected branches of a project.
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct ProtectedBranches<'a> {
    /// The project to query for protected branches.
    #[builder(setter(into))]
    project: NameOrId<'a>,

    /// Name or part of the name of protected branches to be searched for.
    ///
    /// The search query will be escaped automatically.
    #[builder(setter(into), default)]
    search: Option<Cow<'a, str>>,
}

impl<'a> ProtectedBranches<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> ProtectedBranchesBuilder<'a> {
        ProtectedBranchesBuilder::default()
    }
}

impl<'a> Endpoint for ProtectedBranches<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/protected_branches", self.project).into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();

        params.push_opt("search", self.search.as_ref());

        params
    }
}

impl<'a> Pageable for ProtectedBranches<'a> {}

#[cfg(test)]
mod tests {
    use crate::api::projects::protected_branches::{
        ProtectedBranches, ProtectedBranchesBuilderError,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_is_needed() {
        let err = ProtectedBranches::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, ProtectedBranchesBuilderError, "project");
    }

    #[test]
    fn project_is_sufficient() {
        ProtectedBranches::builder().project(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/group%2Fproject/protected_branches")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProtectedBranches::builder()
            .project("group/project")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_search() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/group%2Fproject/protected_branches")
            .add_query_params(&[("search", "name")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProtectedBranches::builder()
            .project("group/project")
            .search("name")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
