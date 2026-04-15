// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::{NameOrId, SortOrder};
use crate::api::endpoint_prelude::*;
use crate::api::helpers::NoteOrderBy;

/// Query for notes on an merge request within a project.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct MergeRequestNotes<'a> {
    /// The project to query for the merge request.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The ID of the merge request.
    merge_request: u64,

    /// Order results by a given key.
    #[builder(default)]
    order_by: Option<NoteOrderBy>,
    /// The sort order for return results.
    #[builder(default)]
    sort: Option<SortOrder>,
}

impl<'a> MergeRequestNotes<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> MergeRequestNotesBuilder<'a> {
        MergeRequestNotesBuilder::default()
    }
}

impl<'a> Endpoint for MergeRequestNotes<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/merge_requests/{}/notes",
            self.project, self.merge_request,
        )
        .into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();

        params
            .push_opt("order_by", self.order_by)
            .push_opt("sort", self.sort);

        params
    }
}

impl<'a> Pageable for MergeRequestNotes<'a> {}

#[cfg(test)]
mod tests {
    use crate::api::common::SortOrder;
    use crate::api::projects::merge_requests::notes::{
        MergeRequestNotes, MergeRequestNotesBuilderError, NoteOrderBy,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_merge_request_are_necessary() {
        let err = MergeRequestNotes::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, MergeRequestNotesBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = MergeRequestNotes::builder()
            .merge_request(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, MergeRequestNotesBuilderError, "project");
    }

    #[test]
    fn merge_request_is_necessary() {
        let err = MergeRequestNotes::builder().project(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, MergeRequestNotesBuilderError, "merge_request");
    }

    #[test]
    fn project_and_merge_request_are_sufficient() {
        MergeRequestNotes::builder()
            .project(1)
            .merge_request(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests/1/notes")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequestNotes::builder()
            .project("simple/project")
            .merge_request(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_order_by() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests/1/notes")
            .add_query_params(&[("order_by", "created_at")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequestNotes::builder()
            .project("simple/project")
            .merge_request(1)
            .order_by(NoteOrderBy::CreatedAt)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_sort() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests/1/notes")
            .add_query_params(&[("sort", "desc")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequestNotes::builder()
            .project("simple/project")
            .merge_request(1)
            .sort(SortOrder::Descending)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
