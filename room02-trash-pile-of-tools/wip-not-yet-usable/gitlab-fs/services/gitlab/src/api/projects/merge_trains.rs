// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Project merge train API endpoint.
//!
//! These endpoints are used for querying projects' merge trains.

use derive_builder::Builder;

use crate::api::common::{NameOrId, SortOrder};
use crate::api::{endpoint_prelude::*, ParamValue};

/// Filter merge train entries by a scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MergeTrainsScope {
    /// Still in progress.
    Active,
    /// Has been merged.
    Complete,
}

impl MergeTrainsScope {
    fn as_str(self) -> &'static str {
        match self {
            MergeTrainsScope::Active => "active",
            MergeTrainsScope::Complete => "complete",
        }
    }
}

impl ParamValue<'static> for MergeTrainsScope {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Get the list of merge trains for project.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct MergeTrains<'a> {
    /// The project with the merge request.
    #[builder(setter(into))]
    project: NameOrId<'a>,

    /// Filter merge trains within a scope.
    #[builder(default)]
    scope: Option<MergeTrainsScope>,

    /// The sort order for return results.
    #[builder(default)]
    sort: Option<SortOrder>,
}

impl<'a> MergeTrains<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> MergeTrainsBuilder<'a> {
        MergeTrainsBuilder::default()
    }
}

impl<'a> Endpoint for MergeTrains<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/merge_trains", self.project).into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();

        params
            .push_opt("scope", self.scope)
            .push_opt("sort", self.sort);

        params
    }
}

impl<'a> Pageable for MergeTrains<'a> {}

#[cfg(test)]
mod tests {
    use crate::api::projects::merge_trains::{
        MergeTrains, MergeTrainsBuilderError, MergeTrainsScope,
    };

    use crate::api::common::SortOrder;
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_is_needed() {
        let err = MergeTrains::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, MergeTrainsBuilderError, "project");
    }

    #[test]
    fn project_is_sufficient() {
        MergeTrains::builder().project(1).build().unwrap();
    }

    #[test]
    fn endpoint_sort() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_trains")
            .add_query_params(&[("sort", "desc")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeTrains::builder()
            .project("simple/project")
            .sort(SortOrder::Descending)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn merge_train_scope_as_str() {
        let items = &[
            (MergeTrainsScope::Active, "active"),
            (MergeTrainsScope::Complete, "complete"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn endpoint_scope() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_trains")
            .add_query_params(&[("scope", "active")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeTrains::builder()
            .project("simple/project")
            .scope(MergeTrainsScope::Active)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
