// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::BTreeSet;
use std::iter;

use chrono::{DateTime, Utc};
use derive_builder::Builder;

use crate::api::common::{NameOrId, SortOrder, YesNo};
use crate::api::endpoint_prelude::*;
use crate::api::helpers::{Labels, Milestone, ReactionEmoji};
use crate::api::ParamValue;

/// Filters for merge request states.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MergeRequestState {
    /// Filter merge requests that are open.
    Opened,
    /// Filter merge requests that are closed.
    Closed,
    /// Filter merge requests that are locked.
    Locked,
    /// Filter merge requests that are merged.
    Merged,
}

impl MergeRequestState {
    fn as_str(self) -> &'static str {
        match self {
            MergeRequestState::Opened => "opened",
            MergeRequestState::Closed => "closed",
            MergeRequestState::Locked => "locked",
            MergeRequestState::Merged => "merged",
        }
    }
}

impl ParamValue<'static> for MergeRequestState {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Parameters for a merge request view.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MergeRequestView {
    /// Return just the IID, URL, title, description, and basic state information.
    Simple,
}

impl MergeRequestView {
    fn as_str(self) -> &'static str {
        match self {
            MergeRequestView::Simple => "simple",
        }
    }
}

impl ParamValue<'static> for MergeRequestView {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Filter merge requests by a scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MergeRequestScope {
    /// Filter merge requests created by the API caller.
    CreatedByMe,
    /// Filter merge requests assigned to the API caller.
    AssignedToMe,
    /// Return all merge requests.
    All,
}

impl MergeRequestScope {
    fn as_str(self) -> &'static str {
        match self {
            MergeRequestScope::CreatedByMe => "created_by_me",
            MergeRequestScope::AssignedToMe => "assigned_to_me",
            MergeRequestScope::All => "all",
        }
    }
}

impl ParamValue<'static> for MergeRequestScope {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

#[derive(Debug, Clone)]
enum Assignee {
    Assigned,
    Unassigned,
    Id(u64),
}

impl Assignee {
    fn add_params<'a>(&'a self, params: &mut QueryParams<'a>) {
        match self {
            Assignee::Assigned => {
                params.push("assignee_id", "Any");
            },
            Assignee::Unassigned => {
                params.push("assignee_id", "None");
            },
            Assignee::Id(id) => {
                params.push("assignee_id", *id);
            },
        }
    }
}

/// The scope to apply search query terms to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[deprecated(note = "not actually supported by GitLab")]
pub enum MergeRequestSearchScope {
    /// Search within titles.
    Title,
    /// Search within descriptions.
    Description,
}

#[allow(deprecated)]
impl MergeRequestSearchScope {
    fn as_str(self) -> &'static str {
        match self {
            MergeRequestSearchScope::Title => "title",
            MergeRequestSearchScope::Description => "description",
        }
    }
}

#[allow(deprecated)]
impl ParamValue<'static> for MergeRequestSearchScope {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Keys merge request results may be ordered by.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MergeRequestOrderBy {
    /// Sort by creation date.
    #[default]
    CreatedAt,
    /// Sort by last updated date.
    UpdatedAt,
    /// Sort by title.
    Title,
}

impl MergeRequestOrderBy {
    fn as_str(self) -> &'static str {
        match self {
            MergeRequestOrderBy::CreatedAt => "created_at",
            MergeRequestOrderBy::UpdatedAt => "updated_at",
            MergeRequestOrderBy::Title => "title",
        }
    }
}

impl ParamValue<'static> for MergeRequestOrderBy {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

#[derive(Debug, Clone)]
enum ApproverIds {
    Any,
    None,
    AllOf(BTreeSet<u64>),
}

impl ApproverIds {
    fn add_params<'a>(&'a self, params: &mut QueryParams<'a>) {
        match self {
            ApproverIds::Any => {
                params.push("approver_ids", "Any");
            },
            ApproverIds::None => {
                params.push("approver_ids", "None");
            },
            ApproverIds::AllOf(ids) => {
                params.extend(ids.iter().map(|&id| ("approver_ids[]", id)));
            },
        }
    }
}

#[derive(Debug, Clone)]
enum ApprovedByIds {
    Any,
    None,
    AllOf(BTreeSet<u64>),
}

impl ApprovedByIds {
    fn add_params<'a>(&'a self, params: &mut QueryParams<'a>) {
        match self {
            ApprovedByIds::Any => {
                params.push("approved_by_ids", "Any");
            },
            ApprovedByIds::None => {
                params.push("approved_by_ids", "None");
            },
            ApprovedByIds::AllOf(ids) => {
                params.extend(ids.iter().map(|&id| ("approved_by_ids[]", id)));
            },
        }
    }
}

/// Query for merge requests within a project.
///
/// TODO: Negation (not) filters are not yet supported.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct MergeRequests<'a> {
    /// The project to query for merge requests.
    #[builder(setter(into))]
    project: NameOrId<'a>,

    /// Filter merge requests with specific internal IDs.
    #[builder(setter(name = "_iids"), default, private)]
    iids: BTreeSet<u64>,
    /// Filter merge requests based on state.
    #[builder(default)]
    state: Option<MergeRequestState>,
    /// Filter merge requests with a milestone title.
    #[builder(setter(name = "_milestone"), default, private)]
    milestone: Option<Milestone<'a>>,
    /// The view of the merge request.
    ///
    /// This field can restrict the amount of data returned.
    #[builder(default)]
    view: Option<MergeRequestView>,
    /// Filter merge requests based on labels.
    #[builder(setter(name = "_labels"), default, private)]
    labels: Option<Labels<'a>>,
    /// Include label details in the result.
    #[builder(default)]
    with_labels_details: Option<bool>,
    /// Request that the merge status field be updated.
    #[builder(default)]
    with_merge_status_recheck: Option<bool>,
    /// Filter merge requests created after a point in time.
    #[builder(default)]
    created_after: Option<DateTime<Utc>>,
    /// Filter merge requests created before a point in time.
    #[builder(default)]
    created_before: Option<DateTime<Utc>>,
    /// Filter merge requests last updated after a point in time.
    #[builder(default)]
    updated_after: Option<DateTime<Utc>>,
    /// Filter merge requests last updated before a point in time.
    #[builder(default)]
    updated_before: Option<DateTime<Utc>>,
    /// Filter merge requests within a scope.
    #[builder(default)]
    scope: Option<MergeRequestScope>,
    /// Filter merge requests by author.
    #[builder(setter(into), default)]
    author: Option<NameOrId<'a>>,
    /// Filter merge requests by assignees.
    #[builder(setter(name = "_assignee"), default, private)]
    assignee: Option<Assignee>,
    /// Filter merge requests by approvers.
    #[builder(setter(name = "_approver_ids"), default, private)]
    approver_ids: Option<ApproverIds>,
    /// Filter merge requests by approvals.
    #[builder(setter(name = "_approved_by_ids"), default, private)]
    approved_by_ids: Option<ApprovedByIds>,
    /// Filter merge requests by reviewers.
    #[builder(setter(into), default)]
    reviewer: Option<NameOrId<'a>>,
    /// Filter merge requests by the API caller's reactions.
    #[builder(setter(name = "_my_reaction_emoji"), default, private)]
    my_reaction_emoji: Option<ReactionEmoji<'a>>,
    /// Filter merge requests by source branch.
    #[builder(setter(into), default)]
    source_branch: Option<Cow<'a, str>>,
    /// Filter merge requests by target branch.
    #[builder(setter(into), default)]
    target_branch: Option<Cow<'a, str>>,
    /// Filter merge requests by WIP state
    #[builder(setter(into), default)]
    wip: Option<YesNo>,
    /// Filter merge requests by deployed environment status.
    #[builder(setter(into), default)]
    environment: Option<Cow<'a, str>>,
    /// Filter merge requests by those deployed after a point in time.
    #[builder(default)]
    deployed_after: Option<DateTime<Utc>>,
    /// Filter merge requests by those deployed before a point in time.
    #[builder(default)]
    deployed_before: Option<DateTime<Utc>>,

    // TODO: support `not`
    /// Filter merge requests with a search query.
    #[builder(setter(into), default)]
    search: Option<Cow<'a, str>>,

    /// Order results by a given key.
    #[builder(default)]
    order_by: Option<MergeRequestOrderBy>,
    /// The sort order for return results.
    #[builder(default)]
    sort: Option<SortOrder>,
}

impl<'a> MergeRequests<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> MergeRequestsBuilder<'a> {
        MergeRequestsBuilder::default()
    }
}

impl<'a> MergeRequestsBuilder<'a> {
    /// Return a merge request with an internal ID.
    pub fn iid(&mut self, iid: u64) -> &mut Self {
        self.iids.get_or_insert_with(BTreeSet::new).insert(iid);
        self
    }

    /// Return merge requests with one of a set of internal IDs.
    pub fn iids<I>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = u64>,
    {
        self.iids.get_or_insert_with(BTreeSet::new).extend(iter);
        self
    }

    /// Filter unlabeled merge requests.
    pub fn unlabeled(&mut self) -> &mut Self {
        self.labels = Some(Some(Labels::None));
        self
    }

    /// Filter merge requests with any label.
    pub fn with_any_label(&mut self) -> &mut Self {
        self.labels = Some(Some(Labels::Any));
        self
    }

    /// Filter merge requests with a given label.
    pub fn label<L>(&mut self, label: L) -> &mut Self
    where
        L: Into<Cow<'a, str>>,
    {
        let label = label.into();
        let labels = if let Some(Some(Labels::AllOf(mut set))) = self.labels.take() {
            set.push(label);
            set
        } else {
            iter::once(label).collect()
        };
        self.labels = Some(Some(Labels::AllOf(labels)));
        self
    }

    /// Filter merge requests with all of the given labels.
    pub fn labels<I, L>(&mut self, iter: I) -> &mut Self
    where
        I: IntoIterator<Item = L>,
        L: Into<Cow<'a, str>>,
    {
        let iter = iter.into_iter().map(Into::into);
        let labels = if let Some(Some(Labels::AllOf(mut set))) = self.labels.take() {
            set.extend(iter);
            set
        } else {
            iter.collect()
        };
        self.labels = Some(Some(Labels::AllOf(labels)));
        self
    }

    /// Filter merge requests without a milestone.
    pub fn without_milestone(&mut self) -> &mut Self {
        self.milestone = Some(Some(Milestone::None));
        self
    }

    /// Filter merge requests with any milestone.
    pub fn any_milestone(&mut self) -> &mut Self {
        self.milestone = Some(Some(Milestone::Any));
        self
    }

    /// Filter merge requests with a given milestone.
    pub fn milestone<M>(&mut self, milestone: M) -> &mut Self
    where
        M: Into<Cow<'a, str>>,
    {
        self.milestone = Some(Some(Milestone::Named(milestone.into())));
        self
    }

    /// Filter unassigned merge requests.
    pub fn unassigned(&mut self) -> &mut Self {
        self.assignee = Some(Some(Assignee::Unassigned));
        self
    }

    /// Filter assigned merge requests.
    pub fn assigned(&mut self) -> &mut Self {
        self.assignee = Some(Some(Assignee::Assigned));
        self
    }

    /// Filter merge requests assigned to a user (by ID).
    pub fn assignee_id(&mut self, assignee: u64) -> &mut Self {
        self.assignee = Some(Some(Assignee::Id(assignee)));
        self
    }

    /// Filter merge requests which have no approvers.
    pub fn no_approvers(&mut self) -> &mut Self {
        self.approver_ids = Some(Some(ApproverIds::None));
        self
    }

    /// Filter merge requests which have any approver(s).
    pub fn any_approvers(&mut self) -> &mut Self {
        self.approver_ids = Some(Some(ApproverIds::Any));
        self
    }

    /// Filter merge requests with a specified approver (by ID).
    pub fn approver_id(&mut self, approver: u64) -> &mut Self {
        let approver_ids = if let Some(Some(ApproverIds::AllOf(mut set))) = self.approver_ids.take()
        {
            set.insert(approver);
            set
        } else {
            [approver].iter().copied().collect()
        };
        self.approver_ids = Some(Some(ApproverIds::AllOf(approver_ids)));
        self
    }

    /// Filter merge requests with specified approver (by ID).
    pub fn approver_ids<I>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = u64>,
    {
        let approver_ids = if let Some(Some(ApproverIds::AllOf(mut set))) = self.approver_ids.take()
        {
            set.extend(iter);
            set
        } else {
            iter.collect()
        };
        self.approver_ids = Some(Some(ApproverIds::AllOf(approver_ids)));
        self
    }

    /// Filter merge requests without approvals.
    pub fn no_approvals(&mut self) -> &mut Self {
        self.approved_by_ids = Some(Some(ApprovedByIds::None));
        self
    }

    /// Filter merge requests with any approvals.
    pub fn any_approvals(&mut self) -> &mut Self {
        self.approved_by_ids = Some(Some(ApprovedByIds::Any));
        self
    }

    /// Filter merge requests approved by a specific user (by ID).
    pub fn approved_by_id(&mut self, approved_by: u64) -> &mut Self {
        let approved_by_ids =
            if let Some(Some(ApprovedByIds::AllOf(mut set))) = self.approved_by_ids.take() {
                set.insert(approved_by);
                set
            } else {
                [approved_by].iter().copied().collect()
            };
        self.approved_by_ids = Some(Some(ApprovedByIds::AllOf(approved_by_ids)));
        self
    }

    /// Filter merge requests approved by a specific set of users (by ID).
    pub fn approved_by_ids<I>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = u64>,
    {
        let approved_by_ids =
            if let Some(Some(ApprovedByIds::AllOf(mut set))) = self.approved_by_ids.take() {
                set.extend(iter);
                set
            } else {
                iter.collect()
            };
        self.approved_by_ids = Some(Some(ApprovedByIds::AllOf(approved_by_ids)));
        self
    }

    /// Filter merge requests without a reaction by the API caller.
    pub fn no_reaction(&mut self) -> &mut Self {
        self.my_reaction_emoji = Some(Some(ReactionEmoji::None));
        self
    }

    /// Filter merge requests with any reaction by the API caller.
    pub fn any_reaction(&mut self) -> &mut Self {
        self.my_reaction_emoji = Some(Some(ReactionEmoji::Any));
        self
    }

    /// Filter merge requests with a specific reaction by the API caller.
    pub fn my_reaction<E>(&mut self, emoji: E) -> &mut Self
    where
        E: Into<Cow<'a, str>>,
    {
        self.my_reaction_emoji = Some(Some(ReactionEmoji::Emoji(emoji.into())));
        self
    }

    /// The scopes to look for search query within.
    #[deprecated(note = "not actually supported by GitLab")]
    #[allow(deprecated)]
    pub fn search_in(&mut self, _: MergeRequestSearchScope) -> &mut Self {
        self
    }
}

impl<'a> Endpoint for MergeRequests<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/merge_requests", self.project).into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();

        params
            .extend(self.iids.iter().map(|&value| ("iids[]", value)))
            .push_opt("state", self.state)
            .push_opt("milestone", self.milestone.as_ref())
            .push_opt("view", self.view)
            .push_opt("labels", self.labels.as_ref())
            .push_opt("with_labels_details", self.with_labels_details)
            .push_opt("with_merge_status_recheck", self.with_merge_status_recheck)
            .push_opt("created_after", self.created_after)
            .push_opt("created_before", self.created_before)
            .push_opt("updated_after", self.updated_after)
            .push_opt("updated_before", self.updated_before)
            .push_opt("scope", self.scope)
            .push_opt("my_reaction_emoji", self.my_reaction_emoji.as_ref())
            .push_opt("source_branch", self.source_branch.as_ref())
            .push_opt("target_branch", self.target_branch.as_ref())
            .push_opt("search", self.search.as_ref())
            .push_opt("wip", self.wip)
            .push_opt("environment", self.environment.as_ref())
            .push_opt("deployed_after", self.deployed_after)
            .push_opt("deployed_before", self.deployed_before)
            .push_opt("order_by", self.order_by)
            .push_opt("sort", self.sort);

        if let Some(author) = self.author.as_ref() {
            match author {
                NameOrId::Name(name) => {
                    params.push("author_username", name);
                },
                NameOrId::Id(id) => {
                    params.push("author_id", *id);
                },
            }
        }
        if let Some(assignee) = self.assignee.as_ref() {
            assignee.add_params(&mut params);
        }
        if let Some(approver_ids) = self.approver_ids.as_ref() {
            approver_ids.add_params(&mut params);
        }
        if let Some(approved_by_ids) = self.approved_by_ids.as_ref() {
            approved_by_ids.add_params(&mut params);
        }
        if let Some(reviewer) = self.reviewer.as_ref() {
            match reviewer {
                NameOrId::Name(name) => {
                    params.push("reviewer_username", name);
                },
                NameOrId::Id(id) => {
                    params.push("reviewer_id", *id);
                },
            }
        }

        params
    }
}

impl<'a> Pageable for MergeRequests<'a> {}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    use crate::api::common::{SortOrder, YesNo};
    use crate::api::projects::merge_requests::{
        MergeRequestOrderBy, MergeRequestScope, MergeRequestState, MergeRequestView, MergeRequests,
        MergeRequestsBuilderError,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn merge_request_state_as_str() {
        let items = &[
            (MergeRequestState::Opened, "opened"),
            (MergeRequestState::Closed, "closed"),
            (MergeRequestState::Locked, "locked"),
            (MergeRequestState::Merged, "merged"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn merge_request_view_as_str() {
        let items = &[(MergeRequestView::Simple, "simple")];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn merge_request_scope_as_str() {
        let items = &[
            (MergeRequestScope::CreatedByMe, "created_by_me"),
            (MergeRequestScope::AssignedToMe, "assigned_to_me"),
            (MergeRequestScope::All, "all"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn merge_request_order_by_default() {
        assert_eq!(
            MergeRequestOrderBy::default(),
            MergeRequestOrderBy::CreatedAt,
        );
    }

    #[test]
    fn merge_request_order_by_as_str() {
        let items = &[
            (MergeRequestOrderBy::CreatedAt, "created_at"),
            (MergeRequestOrderBy::UpdatedAt, "updated_at"),
            (MergeRequestOrderBy::Title, "title"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn project_is_needed() {
        let err = MergeRequests::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, MergeRequestsBuilderError, "project");
    }

    #[test]
    fn project_is_sufficient() {
        MergeRequests::builder().project(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequests::builder()
            .project("simple/project")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_iids() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests")
            .add_query_params(&[("iids[]", "1"), ("iids[]", "2")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequests::builder()
            .project("simple/project")
            .iid(1)
            .iids([1, 2].iter().copied())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_state() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests")
            .add_query_params(&[("state", "locked")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequests::builder()
            .project("simple/project")
            .state(MergeRequestState::Locked)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_milestone_none() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests")
            .add_query_params(&[("milestone", "None")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequests::builder()
            .project("simple/project")
            .without_milestone()
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_milestone_any() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests")
            .add_query_params(&[("milestone", "Any")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequests::builder()
            .project("simple/project")
            .any_milestone()
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_milestone() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests")
            .add_query_params(&[("milestone", "1.0")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequests::builder()
            .project("simple/project")
            .milestone("1.0")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_view() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests")
            .add_query_params(&[("view", "simple")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequests::builder()
            .project("simple/project")
            .view(MergeRequestView::Simple)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_labels() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests")
            .add_query_params(&[("labels", "label,label1,label2")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequests::builder()
            .project("simple/project")
            .label("label")
            .labels(["label1", "label2"].iter().cloned())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_labels_unlabeled() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests")
            .add_query_params(&[("labels", "None")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequests::builder()
            .project("simple/project")
            .unlabeled()
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_labels_any() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests")
            .add_query_params(&[("labels", "Any")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequests::builder()
            .project("simple/project")
            .with_any_label()
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_with_labels_details() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests")
            .add_query_params(&[("with_labels_details", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequests::builder()
            .project("simple/project")
            .with_labels_details(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_with_merge_status_recheck() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests")
            .add_query_params(&[("with_merge_status_recheck", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequests::builder()
            .project("simple/project")
            .with_merge_status_recheck(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_created_after() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests")
            .add_query_params(&[("created_after", "2020-01-01T00:00:00Z")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequests::builder()
            .project("simple/project")
            .created_after(Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_created_before() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests")
            .add_query_params(&[("created_before", "2020-01-01T00:00:00Z")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequests::builder()
            .project("simple/project")
            .created_before(Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_updated_after() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests")
            .add_query_params(&[("updated_after", "2020-01-01T00:00:00Z")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequests::builder()
            .project("simple/project")
            .updated_after(Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_updated_before() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests")
            .add_query_params(&[("updated_before", "2020-01-01T00:00:00Z")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequests::builder()
            .project("simple/project")
            .updated_before(Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_scope() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests")
            .add_query_params(&[("scope", "all")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequests::builder()
            .project("simple/project")
            .scope(MergeRequestScope::All)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_author() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests")
            .add_query_params(&[("author_id", "1")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequests::builder()
            .project("simple/project")
            .author(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_author_name() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests")
            .add_query_params(&[("author_username", "name")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequests::builder()
            .project("simple/project")
            .author("name")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_assignee_unassigned() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests")
            .add_query_params(&[("assignee_id", "None")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequests::builder()
            .project("simple/project")
            .unassigned()
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_assignee_assigned() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests")
            .add_query_params(&[("assignee_id", "Any")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequests::builder()
            .project("simple/project")
            .assigned()
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_assignee_id() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests")
            .add_query_params(&[("assignee_id", "1")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequests::builder()
            .project("simple/project")
            .assignee_id(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_approvers_none() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests")
            .add_query_params(&[("approver_ids", "None")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequests::builder()
            .project("simple/project")
            .no_approvers()
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_approvers_any() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests")
            .add_query_params(&[("approver_ids", "Any")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequests::builder()
            .project("simple/project")
            .any_approvers()
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_approver_ids() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests")
            .add_query_params(&[("approver_ids[]", "1"), ("approver_ids[]", "2")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequests::builder()
            .project("simple/project")
            .approver_id(1)
            .approver_ids([1, 2].iter().copied())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_approvals_none() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests")
            .add_query_params(&[("approved_by_ids", "None")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequests::builder()
            .project("simple/project")
            .no_approvals()
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_approvals_any() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests")
            .add_query_params(&[("approved_by_ids", "Any")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequests::builder()
            .project("simple/project")
            .any_approvals()
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_approved_by() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests")
            .add_query_params(&[("approved_by_ids[]", "1"), ("approved_by_ids[]", "2")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequests::builder()
            .project("simple/project")
            .approved_by_id(1)
            .approved_by_ids([1, 2].iter().copied())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_reviewer() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests")
            .add_query_params(&[("reviewer_id", "1")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequests::builder()
            .project("simple/project")
            .reviewer(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_reviewer_name() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests")
            .add_query_params(&[("reviewer_username", "name")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequests::builder()
            .project("simple/project")
            .reviewer("name")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_my_reaction_emoji() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests")
            .add_query_params(&[("my_reaction_emoji", "tada")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequests::builder()
            .project("simple/project")
            .my_reaction("tada")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_my_reaction_emoji_no_reaction() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests")
            .add_query_params(&[("my_reaction_emoji", "None")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequests::builder()
            .project("simple/project")
            .no_reaction()
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_my_reaction_emoji_any_reaction() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests")
            .add_query_params(&[("my_reaction_emoji", "Any")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequests::builder()
            .project("simple/project")
            .any_reaction()
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_target_branch() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests")
            .add_query_params(&[("target_branch", "target/branch")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequests::builder()
            .project("simple/project")
            .target_branch("target/branch")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_wip() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests")
            .add_query_params(&[("wip", "yes")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequests::builder()
            .project("simple/project")
            .wip(YesNo::Yes)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_environment() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests")
            .add_query_params(&[("environment", "env")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequests::builder()
            .project("simple/project")
            .environment("env")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_deployed_before() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests")
            .add_query_params(&[("deployed_before", "2020-01-01T00:00:00Z")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequests::builder()
            .project("simple/project")
            .deployed_before(Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_deployed_after() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests")
            .add_query_params(&[("deployed_after", "2020-01-01T00:00:00Z")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequests::builder()
            .project("simple/project")
            .deployed_after(Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_search() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests")
            .add_query_params(&[("search", "query")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequests::builder()
            .project("simple/project")
            .search("query")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_order_by() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests")
            .add_query_params(&[("order_by", "created_at")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequests::builder()
            .project("simple/project")
            .order_by(MergeRequestOrderBy::CreatedAt)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_sort() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests")
            .add_query_params(&[("sort", "desc")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequests::builder()
            .project("simple/project")
            .sort(SortOrder::Descending)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
