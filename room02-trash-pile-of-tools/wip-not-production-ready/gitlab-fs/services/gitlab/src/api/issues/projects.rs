// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::{borrow::Cow, collections::BTreeSet, iter};

use chrono::{DateTime, Utc};
use derive_builder::Builder;
use http::Method;

use crate::api::{
    common::{CommaSeparatedList, NameOrId, SortOrder},
    helpers::{Labels, Milestone, ReactionEmoji},
    issues::IssueType,
    Endpoint, Pageable, QueryParams,
};

use super::{
    Assignee, IssueDueDateFilter, IssueIteration, IssueOrderBy, IssueScope, IssueSearchScope,
    IssueState, IssueWeight,
};

/// Query for issues within a project.
///
/// TODO: Negation (not) filters are not yet supported.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct ProjectIssues<'a> {
    /// The project to query for issues.
    #[builder(setter(into))]
    project: NameOrId<'a>,

    /// Filter issues with specific internal IDs.
    #[builder(setter(name = "_iids"), default, private)]
    iids: BTreeSet<u64>,
    /// Filter issues based on state.
    #[builder(default)]
    state: Option<IssueState>,
    /// Filter issues based on labels.
    #[builder(setter(name = "_labels"), default, private)]
    labels: Option<Labels<'a>>,
    /// Include label details in the result.
    #[builder(default)]
    with_labels_details: Option<bool>,
    /// Filter by the iteration.
    #[builder(default)]
    iteration: Option<IssueIteration<'a>>,
    /// Filter issues with a milestone.
    #[builder(setter(name = "_milestone"), default, private)]
    milestone: Option<Milestone<'a>>,
    /// Filter issues within a scope.
    #[builder(default)]
    scope: Option<IssueScope>,
    /// Filter issues by author.
    #[builder(setter(into), default)]
    author: Option<NameOrId<'a>>,
    /// Filter issues by assignees.
    #[builder(setter(name = "_assignee"), default, private)]
    assignee: Option<Assignee<'a>>,
    /// Filter issues by the API caller's reactions.
    #[builder(setter(name = "_my_reaction_emoji"), default, private)]
    my_reaction_emoji: Option<ReactionEmoji<'a>>,
    /// Filter issues by weight.
    #[builder(default)]
    weight: Option<IssueWeight>,

    /// Filter issues with a search query.
    #[builder(setter(into), default)]
    search: Option<Cow<'a, str>>,
    #[builder(setter(name = "_search_in"), default, private)]
    search_in: Option<CommaSeparatedList<IssueSearchScope>>,
    /// Filter issues created after a point in time.
    #[builder(default)]
    created_after: Option<DateTime<Utc>>,
    /// Filter issues created before a point in time.
    #[builder(default)]
    created_before: Option<DateTime<Utc>>,
    /// Filter issues last updated after a point in time.
    #[builder(default)]
    updated_after: Option<DateTime<Utc>>,
    /// Filter issues last updated before a point in time.
    #[builder(default)]
    updated_before: Option<DateTime<Utc>>,
    /// Filter issues by confidentiality.
    #[builder(default)]
    confidential: Option<bool>,
    /// Filter issues by due date.
    #[builder(default)]
    due_date: Option<IssueDueDateFilter>,
    /// Filter by epic ID.
    #[builder(default)]
    epic_id: Option<u64>,
    /// Filter by issue type.
    #[builder(default)]
    issue_type: Option<IssueType>,

    // TODO: How best to support this parameter?
    // not
    /// Order results by a given key.
    #[builder(default)]
    order_by: Option<IssueOrderBy>,
    /// The sort order for return results.
    #[builder(default)]
    sort: Option<SortOrder>,
}

impl<'a> ProjectIssues<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> ProjectIssuesBuilder<'a> {
        ProjectIssuesBuilder::default()
    }
}

impl<'a> ProjectIssuesBuilder<'a> {
    /// Return an issue with an internal ID.
    pub fn iid(&mut self, iid: u64) -> &mut Self {
        self.iids.get_or_insert_with(BTreeSet::new).insert(iid);
        self
    }

    /// Return issues with one of a set of internal IDs.
    pub fn iids<I>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = u64>,
    {
        self.iids.get_or_insert_with(BTreeSet::new).extend(iter);
        self
    }

    /// Filter unlabeled issues.
    pub fn unlabeled(&mut self) -> &mut Self {
        self.labels = Some(Some(Labels::None));
        self
    }

    /// Filter issues with any label.
    pub fn with_any_label(&mut self) -> &mut Self {
        self.labels = Some(Some(Labels::Any));
        self
    }

    /// Filter issues with a given label.
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

    /// Filter issues with all of the given labels.
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

    /// Filter issues without a milestone.
    pub fn without_milestone(&mut self) -> &mut Self {
        self.milestone = Some(Some(Milestone::None));
        self
    }

    /// Filter issues with any milestone.
    pub fn any_milestone(&mut self) -> &mut Self {
        self.milestone = Some(Some(Milestone::Any));
        self
    }

    /// Filter issues with a given milestone.
    pub fn milestone<M>(&mut self, milestone: M) -> &mut Self
    where
        M: Into<Cow<'a, str>>,
    {
        self.milestone = Some(Some(Milestone::Named(milestone.into())));
        self
    }

    /// Filter unassigned issues.
    pub fn unassigned(&mut self) -> &mut Self {
        self.assignee = Some(Some(Assignee::Unassigned));
        self
    }

    /// Filter assigned issues.
    pub fn assigned(&mut self) -> &mut Self {
        self.assignee = Some(Some(Assignee::Assigned));
        self
    }

    /// Filter issues assigned to a user (by ID).
    pub fn assignee_id(&mut self, assignee: u64) -> &mut Self {
        self.assignee = Some(Some(Assignee::Id(assignee)));
        self
    }

    /// Filter issues assigned to a users (by username).
    pub fn assignee<A>(&mut self, assignee: A) -> &mut Self
    where
        A: Into<Cow<'a, str>>,
    {
        let assignee = assignee.into();
        let assignees = if let Some(Some(Assignee::Usernames(mut set))) = self.assignee.take() {
            set.insert(assignee);
            set
        } else {
            let mut set = BTreeSet::new();
            set.insert(assignee);
            set
        };
        self.assignee = Some(Some(Assignee::Usernames(assignees)));
        self
    }

    /// Filter issues assigned to a set of users.
    pub fn assignees<I, A>(&mut self, iter: I) -> &mut Self
    where
        I: IntoIterator<Item = A>,
        A: Into<Cow<'a, str>>,
    {
        let iter = iter.into_iter().map(Into::into);
        let assignees = if let Some(Some(Assignee::Usernames(mut set))) = self.assignee.take() {
            set.extend(iter);
            set
        } else {
            iter.collect()
        };
        self.assignee = Some(Some(Assignee::Usernames(assignees)));
        self
    }

    /// Filter issues without a reaction by the API caller.
    pub fn no_reaction(&mut self) -> &mut Self {
        self.my_reaction_emoji = Some(Some(ReactionEmoji::None));
        self
    }

    /// Filter issues with any reaction by the API caller.
    pub fn any_reaction(&mut self) -> &mut Self {
        self.my_reaction_emoji = Some(Some(ReactionEmoji::Any));
        self
    }

    /// Filter issues with a specific reaction by the API caller.
    pub fn my_reaction<E>(&mut self, emoji: E) -> &mut Self
    where
        E: Into<Cow<'a, str>>,
    {
        self.my_reaction_emoji = Some(Some(ReactionEmoji::Emoji(emoji.into())));
        self
    }

    /// The scopes to look for search query within.
    pub fn search_in(&mut self, scope: IssueSearchScope) -> &mut Self {
        self.search_in
            .get_or_insert(None)
            .get_or_insert_with(CommaSeparatedList::new)
            .push(scope);
        self
    }
}

impl<'a> Endpoint for ProjectIssues<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/issues", self.project).into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();

        params
            .extend(self.iids.iter().map(|&value| ("iids[]", value)))
            .push_opt("state", self.state)
            .push_opt("labels", self.labels.as_ref())
            .push_opt("with_labels_details", self.with_labels_details)
            .push_opt("milestone", self.milestone.as_ref())
            .push_opt("scope", self.scope)
            .push_opt("my_reaction_emoji", self.my_reaction_emoji.as_ref())
            .push_opt("weight", self.weight)
            .push_opt("search", self.search.as_ref())
            .push_opt("in", self.search_in.as_ref())
            .push_opt("created_after", self.created_after)
            .push_opt("created_before", self.created_before)
            .push_opt("updated_after", self.updated_after)
            .push_opt("updated_before", self.updated_before)
            .push_opt("confidential", self.confidential)
            .push_opt("due_date", self.due_date)
            .push_opt("epic_id", self.epic_id)
            .push_opt("issue_type", self.issue_type)
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
        if let Some(iteration) = self.iteration.as_ref() {
            iteration.add_params(&mut params);
        }
        if let Some(assignee) = self.assignee.as_ref() {
            assignee.add_params(&mut params);
        }

        params
    }
}

impl<'a> Pageable for ProjectIssues<'a> {}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    use crate::api::common::SortOrder;
    use crate::api::issues::{
        projects::ProjectIssues, projects::ProjectIssuesBuilderError, IssueDueDateFilter,
        IssueIteration, IssueOrderBy, IssueScope, IssueSearchScope, IssueState, IssueType,
        IssueWeight,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_is_needed() {
        let err = ProjectIssues::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, ProjectIssuesBuilderError, "project");
    }

    #[test]
    fn project_is_sufficient() {
        ProjectIssues::builder().project(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/issues")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectIssues::builder()
            .project("simple/project")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_iids() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/issues")
            .add_query_params(&[("iids[]", "1"), ("iids[]", "2")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectIssues::builder()
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
            .endpoint("projects/simple%2Fproject/issues")
            .add_query_params(&[("state", "closed")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectIssues::builder()
            .project("simple/project")
            .state(IssueState::Closed)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_labels() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/issues")
            .add_query_params(&[("labels", "label,label1,label2")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectIssues::builder()
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
            .endpoint("projects/simple%2Fproject/issues")
            .add_query_params(&[("labels", "None")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectIssues::builder()
            .project("simple/project")
            .unlabeled()
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_labels_any() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/issues")
            .add_query_params(&[("labels", "Any")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectIssues::builder()
            .project("simple/project")
            .with_any_label()
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_with_labels_details() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/issues")
            .add_query_params(&[("with_labels_details", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectIssues::builder()
            .project("simple/project")
            .with_labels_details(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_iteration_none() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/issues")
            .add_query_params(&[("iteration_id", "None")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectIssues::builder()
            .project("simple/project")
            .iteration(IssueIteration::None)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_iteration_any() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/issues")
            .add_query_params(&[("iteration_id", "Any")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectIssues::builder()
            .project("simple/project")
            .iteration(IssueIteration::Any)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_iteration_id() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/issues")
            .add_query_params(&[("iteration_id", "1")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectIssues::builder()
            .project("simple/project")
            .iteration(IssueIteration::Id(1))
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_iteration_title() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/issues")
            .add_query_params(&[("iteration_title", "title")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectIssues::builder()
            .project("simple/project")
            .iteration(IssueIteration::Title("title".into()))
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_milestone() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/issues")
            .add_query_params(&[("milestone", "1.0")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectIssues::builder()
            .project("simple/project")
            .milestone("1.0")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_scope() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/issues")
            .add_query_params(&[("scope", "all")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectIssues::builder()
            .project("simple/project")
            .scope(IssueScope::All)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_author_id() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/issues")
            .add_query_params(&[("author_id", "1")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectIssues::builder()
            .project("simple/project")
            .author(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_author_name() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/issues")
            .add_query_params(&[("author_username", "name")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectIssues::builder()
            .project("simple/project")
            .author("name")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_assignee_unassigned() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/issues")
            .add_query_params(&[("assignee_id", "None")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectIssues::builder()
            .project("simple/project")
            .unassigned()
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_assignee_assigned() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/issues")
            .add_query_params(&[("assignee_id", "Any")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectIssues::builder()
            .project("simple/project")
            .assigned()
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_assignee_id() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/issues")
            .add_query_params(&[("assignee_id", "1")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectIssues::builder()
            .project("simple/project")
            .assignee_id(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_assignee_user() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/issues")
            .add_query_params(&[
                ("assignee_username[]", "name1"),
                ("assignee_username[]", "name2"),
            ])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectIssues::builder()
            .project("simple/project")
            .assignee("name1")
            .assignees(["name1", "name2"].iter().copied())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_my_reaction_emoji() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/issues")
            .add_query_params(&[("my_reaction_emoji", "tada")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectIssues::builder()
            .project("simple/project")
            .my_reaction("tada")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_my_reaction_emoji_no_reaction() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/issues")
            .add_query_params(&[("my_reaction_emoji", "None")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectIssues::builder()
            .project("simple/project")
            .no_reaction()
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_my_reaction_emoji_any_reaction() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/issues")
            .add_query_params(&[("my_reaction_emoji", "Any")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectIssues::builder()
            .project("simple/project")
            .any_reaction()
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_weight() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/issues")
            .add_query_params(&[("weight", "Any")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectIssues::builder()
            .project("simple/project")
            .weight(IssueWeight::Any)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_search() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/issues")
            .add_query_params(&[("search", "query")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectIssues::builder()
            .project("simple/project")
            .search("query")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_search_in() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/issues")
            .add_query_params(&[("in", "title,description")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectIssues::builder()
            .project("simple/project")
            .search_in(IssueSearchScope::Title)
            .search_in(IssueSearchScope::Description)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_created_after() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/issues")
            .add_query_params(&[("created_after", "2020-01-01T00:00:00Z")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectIssues::builder()
            .project("simple/project")
            .created_after(Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_created_before() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/issues")
            .add_query_params(&[("created_before", "2020-01-01T00:00:00Z")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectIssues::builder()
            .project("simple/project")
            .created_before(Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_updated_after() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/issues")
            .add_query_params(&[("updated_after", "2020-01-01T00:00:00Z")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectIssues::builder()
            .project("simple/project")
            .updated_after(Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_updated_before() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/issues")
            .add_query_params(&[("updated_before", "2020-01-01T00:00:00Z")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectIssues::builder()
            .project("simple/project")
            .updated_before(Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_confidential() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/issues")
            .add_query_params(&[("confidential", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectIssues::builder()
            .project("simple/project")
            .confidential(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_due_date() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/issues")
            .add_query_params(&[("due_date", "week")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectIssues::builder()
            .project("simple/project")
            .due_date(IssueDueDateFilter::ThisWeek)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_epic_id() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/issues")
            .add_query_params(&[("epic_id", "1")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectIssues::builder()
            .project("simple/project")
            .epic_id(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_issue_type() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/issues")
            .add_query_params(&[("issue_type", "incident")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectIssues::builder()
            .project("simple/project")
            .issue_type(IssueType::Incident)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_order_by() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/issues")
            .add_query_params(&[("order_by", "due_date")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectIssues::builder()
            .project("simple/project")
            .order_by(IssueOrderBy::DueDate)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_sort() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/issues")
            .add_query_params(&[("sort", "desc")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectIssues::builder()
            .project("simple/project")
            .sort(SortOrder::Descending)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
