// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::{NameOrId, SortOrder};
use crate::api::endpoint_prelude::*;
use crate::api::helpers::NoteOrderBy;

/// Query for notes on an issue within a project.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct IssueNotes<'a> {
    /// The project to query for the issue.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The ID of the issue.
    issue: u64,

    /// Order results by a given key.
    #[builder(default)]
    order_by: Option<NoteOrderBy>,
    /// The sort order for return results.
    #[builder(default)]
    sort: Option<SortOrder>,
}

impl<'a> IssueNotes<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> IssueNotesBuilder<'a> {
        IssueNotesBuilder::default()
    }
}

impl<'a> Endpoint for IssueNotes<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/issues/{}/notes", self.project, self.issue).into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();

        params
            .push_opt("order_by", self.order_by)
            .push_opt("sort", self.sort);

        params
    }
}

impl<'a> Pageable for IssueNotes<'a> {}

#[cfg(test)]
mod tests {
    use crate::api::common::SortOrder;
    use crate::api::projects::issues::notes::{IssueNotes, IssueNotesBuilderError, NoteOrderBy};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_issue_are_necessary() {
        let err = IssueNotes::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, IssueNotesBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = IssueNotes::builder().issue(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, IssueNotesBuilderError, "project");
    }

    #[test]
    fn issue_is_necessary() {
        let err = IssueNotes::builder().project(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, IssueNotesBuilderError, "issue");
    }

    #[test]
    fn project_and_issue_are_sufficient() {
        IssueNotes::builder().project(1).issue(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/issues/1/notes")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = IssueNotes::builder()
            .project("simple/project")
            .issue(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_order_by() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/issues/1/notes")
            .add_query_params(&[("order_by", "created_at")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = IssueNotes::builder()
            .project("simple/project")
            .issue(1)
            .order_by(NoteOrderBy::CreatedAt)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_sort() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/issues/1/notes")
            .add_query_params(&[("sort", "desc")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = IssueNotes::builder()
            .project("simple/project")
            .issue(1)
            .sort(SortOrder::Descending)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
