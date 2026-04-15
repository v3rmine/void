// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::BTreeSet;

use chrono::{DateTime, NaiveDate, Utc};
use derive_builder::Builder;

use crate::api::common::{CommaSeparatedList, NameOrId};
use crate::api::endpoint_prelude::*;
use crate::api::issues::IssueType;

/// Create a new issue on a project.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct CreateIssue<'a> {
    /// The project to add the issue to.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The title of the new issue.
    ///
    /// Note: this is technically optional if `merge_request_to_resolve_discussions_of` is given,
    /// but to avoid more complicated shenanigans around choosing one or the other, this is always
    /// marked as required. Instead, if `title` is explictly empty and
    /// `merge_request_to_resolve_discussions_of` is given, `title` will not be sent allowing
    /// GitLab to generate the default title.
    #[builder(setter(into))]
    title: Cow<'a, str>,

    /// The internal ID of the issue.
    ///
    /// Requires administrator or owner permissions.
    #[builder(default)]
    iid: Option<u64>,
    /// The description of the new issue.
    #[builder(setter(into), default)]
    description: Option<Cow<'a, str>>,
    /// Whether the issue is confidential or not.
    #[builder(default)]
    confidential: Option<bool>,
    /// Assignees for the issue.
    #[builder(setter(name = "_assignee_ids"), default, private)]
    assignee_ids: BTreeSet<u64>,
    /// The ID of the milestone for the issue.
    #[builder(default)]
    milestone_id: Option<u64>,
    /// Labels to add to the issue.
    #[builder(setter(name = "_labels"), default, private)]
    labels: Option<CommaSeparatedList<Cow<'a, str>>>,
    /// The creation date of the issue.
    ///
    /// Requires administrator or owner permissions.
    #[builder(default)]
    created_at: Option<DateTime<Utc>>,
    /// The due date for the issue.
    #[builder(default)]
    due_date: Option<NaiveDate>,
    /// The ID of a merge request for which to resolve the discussions.
    ///
    /// Resolves all open discussions unless `discussion_to_resolve` is also passed.
    #[builder(default)]
    merge_request_to_resolve_discussions_of: Option<u64>,
    /// The ID of the discussion to resolve.
    #[builder(setter(into), default)]
    discussion_to_resolve: Option<Cow<'a, str>>,
    /// The weight of the issue.
    #[builder(default)]
    weight: Option<u64>,
    /// The ID of the epic to add the issue to.
    #[builder(default)]
    epic_id: Option<u64>,
    /// The type of issue.
    #[builder(default)]
    issue_type: Option<IssueType>,

    /// The internal ID of the epic to add the issue to.
    #[deprecated(note = "use `epic_id` instead")]
    #[builder(default)]
    epic_iid: Option<u64>,
}

impl<'a> CreateIssue<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> CreateIssueBuilder<'a> {
        CreateIssueBuilder::default()
    }
}

impl<'a> CreateIssueBuilder<'a> {
    /// Assign the issue to a user.
    pub fn assignee_id(&mut self, assignee: u64) -> &mut Self {
        self.assignee_ids
            .get_or_insert_with(BTreeSet::new)
            .insert(assignee);
        self
    }

    /// Assign the issue to a set of users.
    pub fn assignee_ids<I>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = u64>,
    {
        self.assignee_ids
            .get_or_insert_with(BTreeSet::new)
            .extend(iter);
        self
    }

    /// Add a label to the issue.
    pub fn label<L>(&mut self, label: L) -> &mut Self
    where
        L: Into<Cow<'a, str>>,
    {
        self.labels
            .get_or_insert(None)
            .get_or_insert_with(CommaSeparatedList::new)
            .push(label.into());
        self
    }

    /// Add a set of labels to the issue.
    pub fn labels<I, L>(&mut self, iter: I) -> &mut Self
    where
        I: IntoIterator<Item = L>,
        L: Into<Cow<'a, str>>,
    {
        self.labels
            .get_or_insert(None)
            .get_or_insert_with(CommaSeparatedList::new)
            .extend(iter.into_iter().map(Into::into));
        self
    }
}

impl<'a> Endpoint for CreateIssue<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/issues", self.project).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        if !self.title.is_empty() || self.merge_request_to_resolve_discussions_of.is_none() {
            params.push("title", &self.title);
        }

        params
            .push_opt("iid", self.iid)
            .push_opt("description", self.description.as_ref())
            .push_opt("confidential", self.confidential)
            .extend(
                self.assignee_ids
                    .iter()
                    .map(|&value| ("assignee_ids[]", value)),
            )
            .push_opt("milestone_id", self.milestone_id)
            .push_opt("labels", self.labels.as_ref())
            .push_opt("created_at", self.created_at)
            .push_opt("due_date", self.due_date)
            .push_opt(
                "merge_request_to_resolve_discussions_of",
                self.merge_request_to_resolve_discussions_of,
            )
            .push_opt("discussion_to_resolve", self.discussion_to_resolve.as_ref())
            .push_opt("weight", self.weight)
            .push_opt("epic_id", self.epic_id)
            .push_opt("issue_type", self.issue_type);

        #[allow(deprecated)]
        {
            params.push_opt("epic_iid", self.epic_iid);
        }

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, TimeZone, Utc};
    use http::Method;

    use crate::api::issues::IssueType;
    use crate::api::projects::issues::{CreateIssue, CreateIssueBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_title_are_necessary() {
        let err = CreateIssue::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, CreateIssueBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = CreateIssue::builder().title("title").build().unwrap_err();
        crate::test::assert_missing_field!(err, CreateIssueBuilderError, "project");
    }

    #[test]
    fn title_is_necessary() {
        let err = CreateIssue::builder().project(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, CreateIssueBuilderError, "title");
    }

    #[test]
    fn project_and_title_are_sufficient() {
        CreateIssue::builder()
            .project(1)
            .title("title")
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/issues")
            .content_type("application/x-www-form-urlencoded")
            .body_str("title=title+of+issue")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateIssue::builder()
            .project("simple/project")
            .title("title of issue")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_iid() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/issues")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("title=title", "&iid=1"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateIssue::builder()
            .project("simple/project")
            .title("title")
            .iid(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_description() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/issues")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("title=title", "&description=description"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateIssue::builder()
            .project("simple/project")
            .title("title")
            .description("description")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_confidential() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/issues")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("title=title", "&confidential=true"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateIssue::builder()
            .project("simple/project")
            .title("title")
            .confidential(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_assignee_ids() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/issues")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "title=title",
                "&assignee_ids%5B%5D=1",
                "&assignee_ids%5B%5D=2",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateIssue::builder()
            .project("simple/project")
            .title("title")
            .assignee_id(1)
            .assignee_ids([1, 2].iter().copied())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_milestone_id() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/issues")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("title=title", "&milestone_id=1"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateIssue::builder()
            .project("simple/project")
            .title("title")
            .milestone_id(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_labels() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/issues")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("title=title", "&labels=label"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateIssue::builder()
            .project("simple/project")
            .title("title")
            .label("label")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_labels_multiple() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/issues")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("title=title", "&labels=label1%2Clabel2"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateIssue::builder()
            .project("simple/project")
            .title("title")
            .labels(["label1", "label2"].iter().copied())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_created_at() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/issues")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "title=title",
                "&created_at=2020-01-01T00%3A00%3A00Z",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateIssue::builder()
            .project("simple/project")
            .title("title")
            .created_at(Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_due_date() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/issues")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("title=title", "&due_date=2020-01-01"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateIssue::builder()
            .project("simple/project")
            .title("title")
            .due_date(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_merge_request_to_resolve_discussions_of() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/issues")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "title=title",
                "&merge_request_to_resolve_discussions_of=1",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateIssue::builder()
            .project("simple/project")
            .title("title")
            .merge_request_to_resolve_discussions_of(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_merge_request_to_resolve_discussions_of_no_title() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/issues")
            .content_type("application/x-www-form-urlencoded")
            .body_str("merge_request_to_resolve_discussions_of=1")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateIssue::builder()
            .project("simple/project")
            .title("") // This should trigger logic to not send the parameter.
            .merge_request_to_resolve_discussions_of(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_discussion_to_resolve() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/issues")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("title=title", "&discussion_to_resolve=deadbeef"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateIssue::builder()
            .project("simple/project")
            .title("title")
            .discussion_to_resolve("deadbeef")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_weight() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/issues")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("title=title", "&weight=1"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateIssue::builder()
            .project("simple/project")
            .title("title")
            .weight(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_epic_id() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/issues")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("title=title", "&epic_id=1"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateIssue::builder()
            .project("simple/project")
            .title("title")
            .epic_id(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_issue_type() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/issues")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("title=title", "&issue_type=test_case"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateIssue::builder()
            .project("simple/project")
            .title("title")
            .issue_type(IssueType::TestCase)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    #[allow(deprecated)]
    fn endpoint_epic_iid() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/issues")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("title=title", "&epic_iid=1"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateIssue::builder()
            .project("simple/project")
            .title("title")
            .epic_iid(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
