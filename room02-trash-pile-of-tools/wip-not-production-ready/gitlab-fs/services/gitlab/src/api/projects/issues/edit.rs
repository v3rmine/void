// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::BTreeSet;
use std::iter;

use chrono::{DateTime, NaiveDate, Utc};
use derive_builder::Builder;

use crate::api::common::{CommaSeparatedList, NameOrId};
use crate::api::endpoint_prelude::*;
use crate::api::issues::IssueType;
use crate::api::ParamValue;

/// States an issue may be set to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IssueStateEvent {
    /// Close the issue.
    Close,
    /// Reopen a closed issue.
    Reopen,
}

impl IssueStateEvent {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            IssueStateEvent::Close => "close",
            IssueStateEvent::Reopen => "reopen",
        }
    }
}

impl ParamValue<'static> for IssueStateEvent {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

#[derive(Debug, Clone)]
enum IssueAssignees {
    Unassigned,
    Assignees(BTreeSet<u64>),
}

#[derive(Debug, Clone)]
enum IssueLabels<'a> {
    Unlabeled,
    Labeled(CommaSeparatedList<Cow<'a, str>>),
}

impl<'a, 'b: 'a> ParamValue<'a> for &'b IssueLabels<'a> {
    fn as_value(&self) -> Cow<'a, str> {
        match self {
            IssueLabels::Unlabeled => "".into(),
            IssueLabels::Labeled(labels) => labels.to_string().into(),
        }
    }
}

/// Create a new issue on a project.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct EditIssue<'a> {
    /// The project to add the issue to.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The internal IID of the issue.
    issue: u64,

    /// The title of the new issue.
    #[builder(setter(into), default)]
    title: Option<Cow<'a, str>>,
    /// The description of the issue.
    #[builder(setter(into), default)]
    description: Option<Cow<'a, str>>,

    /// Assignees for the issue.
    #[builder(setter(name = "_assignee_ids"), default, private)]
    assignee_ids: Option<IssueAssignees>,
    /// The ID of a milestone to add the issue to.
    #[builder(default)]
    milestone_id: Option<u64>,
    /// Labels to set on the issue.
    #[builder(setter(name = "_labels"), default, private)]
    labels: Option<IssueLabels<'a>>,
    #[builder(setter(name = "_add_labels"), default, private)]
    add_labels: Option<CommaSeparatedList<Cow<'a, str>>>,
    #[builder(setter(name = "_remove_labels"), default, private)]
    remove_labels: Option<CommaSeparatedList<Cow<'a, str>>>,
    /// Change the state of the issue.
    #[builder(default)]
    state_event: Option<IssueStateEvent>,
    /// Set the last-updated time for the issue.
    #[builder(default)]
    updated_at: Option<DateTime<Utc>>,
    /// Set the due date for the issue.
    #[builder(default)]
    due_date: Option<NaiveDate>,
    /// Set the weight of the issue.
    #[builder(default)]
    weight: Option<u64>,
    /// Set whether discussion of the issue should be locked or not.
    #[builder(default)]
    discussion_locked: Option<bool>,
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

    /// Updates an issue to be confidential
    #[builder(default)]
    confidential: Option<bool>,
}

impl<'a> EditIssue<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> EditIssueBuilder<'a> {
        EditIssueBuilder::default()
    }
}

impl<'a> EditIssueBuilder<'a> {
    /// Set the issue ID.
    #[deprecated(note = "use `issue` instead")]
    pub fn issue_iid(&mut self, issue_iid: u64) -> &mut Self {
        self.issue = Some(issue_iid);
        self
    }

    /// Unassign the issue.
    pub fn unassign(&mut self) -> &mut Self {
        self.assignee_ids = Some(Some(IssueAssignees::Unassigned));
        self
    }

    /// Assign the issue to a user.
    pub fn assignee_id(&mut self, assignee: u64) -> &mut Self {
        let assignees =
            if let Some(Some(IssueAssignees::Assignees(mut set))) = self.assignee_ids.take() {
                set.insert(assignee);
                set
            } else {
                let mut set = BTreeSet::new();
                set.insert(assignee);
                set
            };
        self.assignee_ids = Some(Some(IssueAssignees::Assignees(assignees)));
        self
    }

    /// Assigne the issue to a set of users.
    pub fn assignee_ids<I>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = u64>,
    {
        let assignees =
            if let Some(Some(IssueAssignees::Assignees(mut set))) = self.assignee_ids.take() {
                set.extend(iter);
                set
            } else {
                iter.collect()
            };
        self.assignee_ids = Some(Some(IssueAssignees::Assignees(assignees)));
        self
    }

    /// Remove all labels from the issue.
    #[deprecated(note = "use `clear_labels` instead")]
    pub fn remove_labels(&mut self) -> &mut Self {
        self.clear_labels()
    }

    /// Remove all labels from the issue.
    pub fn clear_labels(&mut self) -> &mut Self {
        self.labels = Some(Some(IssueLabels::Unlabeled));
        self
    }

    /// Add a label to the issue.
    ///
    /// Note that the list of labels sent will replace the set on the instance. This only adds it
    /// to the list of labels to add to the set before sending it to the instance.
    ///
    /// See: `add_label`.
    pub fn label<L>(&mut self, label: L) -> &mut Self
    where
        L: Into<Cow<'a, str>>,
    {
        let label = label.into();
        let labels = if let Some(Some(IssueLabels::Labeled(mut set))) = self.labels.take() {
            set.push(label);
            set
        } else {
            iter::once(label).collect()
        };
        self.labels = Some(Some(IssueLabels::Labeled(labels)));
        self
    }

    /// Add a set of labels to the issue.
    ///
    /// Note that the list of labels sent will replace the set on the instance. This only adds it
    /// to the list of labels to add to the set before sending it to the instance.
    ///
    /// See: `add_label`.
    pub fn labels<I, L>(&mut self, iter: I) -> &mut Self
    where
        I: IntoIterator<Item = L>,
        L: Into<Cow<'a, str>>,
    {
        let iter = iter.into_iter().map(Into::into);
        let labels = if let Some(Some(IssueLabels::Labeled(mut set))) = self.labels.take() {
            set.extend(iter);
            set
        } else {
            iter.collect()
        };
        self.labels = Some(Some(IssueLabels::Labeled(labels)));
        self
    }

    /// Add a label to the issue.
    ///
    /// This is an incremental addition to the existing set of labels on the issue.
    pub fn add_label<L>(&mut self, label: L) -> &mut Self
    where
        L: Into<Cow<'a, str>>,
    {
        self.add_labels
            .get_or_insert(None)
            .get_or_insert_with(CommaSeparatedList::new)
            .push(label.into());
        self
    }

    /// Remove a label from the issue.
    ///
    /// This is an incremental remove form the existing set of labels on the issue.
    pub fn remove_label<L>(&mut self, label: L) -> &mut Self
    where
        L: Into<Cow<'a, str>>,
    {
        self.remove_labels
            .get_or_insert(None)
            .get_or_insert_with(CommaSeparatedList::new)
            .push(label.into());
        self
    }
}

impl<'a> Endpoint for EditIssue<'a> {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/issues/{}", self.project, self.issue).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params
            .push_opt("title", self.title.as_ref())
            .push_opt("description", self.description.as_ref())
            .push_opt("milestone_id", self.milestone_id)
            .push_opt("labels", self.labels.as_ref())
            .push_opt("add_labels", self.add_labels.as_ref())
            .push_opt("remove_labels", self.remove_labels.as_ref())
            .push_opt("state_event", self.state_event)
            .push_opt("updated_at", self.updated_at)
            .push_opt("due_date", self.due_date)
            .push_opt("weight", self.weight)
            .push_opt("discussion_locked", self.discussion_locked)
            .push_opt("epic_id", self.epic_id)
            .push_opt("issue_type", self.issue_type)
            .push_opt("confidential", self.confidential);

        if let Some(assignees) = self.assignee_ids.as_ref() {
            match assignees {
                IssueAssignees::Unassigned => {
                    params.push("assignee_ids[]", "0");
                },
                IssueAssignees::Assignees(ids) => {
                    params.extend(ids.iter().map(|&value| ("assignee_ids[]", value)));
                },
            }
        }

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
    use crate::api::projects::issues::{EditIssue, EditIssueBuilderError, IssueStateEvent};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn issue_state_event_as_str() {
        let items = &[
            (IssueStateEvent::Close, "close"),
            (IssueStateEvent::Reopen, "reopen"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn project_and_issue_are_necessary() {
        let err = EditIssue::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, EditIssueBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = EditIssue::builder().issue(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, EditIssueBuilderError, "project");
    }

    #[test]
    fn issue_is_necessary() {
        let err = EditIssue::builder().project(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, EditIssueBuilderError, "issue");
    }

    #[test]
    fn project_and_issue_are_sufficient() {
        EditIssue::builder().project(1).issue(1).build().unwrap();
    }

    #[test]
    #[allow(deprecated)]
    fn project_and_issue_iid_are_sufficient() {
        EditIssue::builder()
            .project(1)
            .issue_iid(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/issues/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditIssue::builder()
            .project("simple/project")
            .issue(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_title() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/issues/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("title=title")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditIssue::builder()
            .project("simple/project")
            .issue(1)
            .title("title")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_description() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/issues/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("description=description")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditIssue::builder()
            .project("simple/project")
            .issue(1)
            .description("description")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_assignee_ids() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/issues/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("assignee_ids%5B%5D=1", "&assignee_ids%5B%5D=2"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditIssue::builder()
            .project("simple/project")
            .issue(1)
            .assignee_id(1)
            .assignee_ids([1, 2].iter().copied())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_assignee_ids_unassign() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/issues/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("assignee_ids%5B%5D=0")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditIssue::builder()
            .project("simple/project")
            .issue(1)
            .unassign()
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_milestone_id() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/issues/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("milestone_id=1")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditIssue::builder()
            .project("simple/project")
            .issue(1)
            .milestone_id(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_labels() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/issues/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("labels=label")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditIssue::builder()
            .project("simple/project")
            .issue(1)
            .label("label")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_labels_multiple() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/issues/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("labels=label1%2Clabel2")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditIssue::builder()
            .project("simple/project")
            .issue(1)
            .labels(["label1", "label2"].iter().copied())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    #[allow(deprecated)]
    fn endpoint_labels_remove() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/issues/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("labels=")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditIssue::builder()
            .project("simple/project")
            .issue(1)
            .remove_labels()
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_labels_clear() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/issues/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("labels=")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditIssue::builder()
            .project("simple/project")
            .issue(1)
            .clear_labels()
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_add_labels() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/issues/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("add_labels=one%2Ctwo")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditIssue::builder()
            .project("simple/project")
            .issue(1)
            .add_label("one")
            .add_label("two")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_remove_labels() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/issues/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("remove_labels=one%2Ctwo")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditIssue::builder()
            .project("simple/project")
            .issue(1)
            .remove_label("one")
            .remove_label("two")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_state_event() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/issues/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("state_event=close")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditIssue::builder()
            .project("simple/project")
            .issue(1)
            .state_event(IssueStateEvent::Close)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_updated_at() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/issues/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("updated_at=2020-01-01T00%3A00%3A00Z")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditIssue::builder()
            .project("simple/project")
            .issue(1)
            .updated_at(Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_due_date() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/issues/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("due_date=2020-01-01")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditIssue::builder()
            .project("simple/project")
            .issue(1)
            .due_date(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_weight() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/issues/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("weight=1")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditIssue::builder()
            .project("simple/project")
            .issue(1)
            .weight(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_discussion_locked() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/issues/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("discussion_locked=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditIssue::builder()
            .project("simple/project")
            .issue(1)
            .discussion_locked(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_epic_id() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/issues/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("epic_id=1")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditIssue::builder()
            .project("simple/project")
            .issue(1)
            .epic_id(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_issue_type() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/issues/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("issue_type=incident")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditIssue::builder()
            .project("simple/project")
            .issue(1)
            .issue_type(IssueType::Incident)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    #[allow(deprecated)]
    fn endpoint_epic_iid() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/issues/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("epic_iid=1")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditIssue::builder()
            .project("simple/project")
            .issue(1)
            .epic_iid(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_confidential() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/issues/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("confidential=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditIssue::builder()
            .project("simple/project")
            .issue(1)
            .confidential(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
