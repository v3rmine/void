// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::iter;

use derive_builder::Builder;

use crate::api::common::{CommaSeparatedList, NameOrId};
use crate::api::endpoint_prelude::*;
use crate::api::projects::merge_requests::create::{Assignee, Reviewer};
use crate::api::ParamValue;

#[derive(Debug, Clone)]
enum MergeRequestLabels<'a> {
    Unlabeled,
    Labeled(CommaSeparatedList<Cow<'a, str>>),
}

impl<'a, 'b: 'a> ParamValue<'a> for &'b MergeRequestLabels<'a> {
    fn as_value(&self) -> Cow<'a, str> {
        match self {
            MergeRequestLabels::Unlabeled => "".into(),
            MergeRequestLabels::Labeled(labels) => labels.to_string().into(),
        }
    }
}

/// States an issue may be set to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MergeRequestStateEvent {
    /// Close the issue.
    Close,
    /// Reopen a closed issue.
    Reopen,
}

impl MergeRequestStateEvent {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            MergeRequestStateEvent::Close => "close",
            MergeRequestStateEvent::Reopen => "reopen",
        }
    }
}

impl ParamValue<'static> for MergeRequestStateEvent {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Edit a new merge request on project.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct EditMergeRequest<'a> {
    /// The project to open the merge requset *from*.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The merge request to edit.
    merge_request: u64,

    /// The name of the target branch for the merge request.
    #[builder(setter(into), default)]
    target_branch: Option<Cow<'a, str>>,
    /// The title for the merge request.
    #[builder(setter(into), default)]
    title: Option<Cow<'a, str>>,
    /// The assignee of the merge request.
    #[builder(setter(name = "_assignee"), default, private)]
    assignee: Option<Assignee>,
    #[builder(setter(name = "_reviewer"), default, private)]
    reviewer: Option<Reviewer>,
    /// The ID of the milestone to add the merge request to.
    #[builder(default)]
    milestone_id: Option<u64>,
    /// Labels to add to the merge request.
    #[builder(setter(name = "_labels"), default, private)]
    labels: Option<MergeRequestLabels<'a>>,
    #[builder(setter(name = "_add_labels"), default, private)]
    add_labels: Option<CommaSeparatedList<Cow<'a, str>>>,
    #[builder(setter(name = "_remove_labels"), default, private)]
    remove_labels: Option<CommaSeparatedList<Cow<'a, str>>>,
    /// The description of the merge request.
    #[builder(setter(into), default)]
    description: Option<Cow<'a, str>>,
    /// Change the state of the merge request.
    #[builder(default)]
    state_event: Option<MergeRequestStateEvent>,
    /// Whether to remove the source branch once merged or not.
    #[builder(default)]
    remove_source_branch: Option<bool>,
    /// Whether to squash the branch when merging or not.
    #[builder(default)]
    squash: Option<bool>,
    /// Whether to lock discussion or not..
    #[builder(default)]
    discussion_locked: Option<bool>,
    /// Whether to allow collaboration with maintainers of the target project or not.
    #[builder(default)]
    allow_collaboration: Option<bool>,

    /// Whether to allow collaboration with maintainers of the target project or not.
    #[deprecated(note = "use `allow_collaboration` instead")]
    #[builder(default)]
    allow_maintainer_to_push: Option<bool>,
}

impl<'a> EditMergeRequest<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> EditMergeRequestBuilder<'a> {
        EditMergeRequestBuilder::default()
    }
}

impl<'a> EditMergeRequestBuilder<'a> {
    /// Unset assignee(s) of merge request
    pub fn unassigned(&mut self) -> &mut Self {
        self.assignee = Some(Some(Assignee::Unassigned));
        self
    }

    /// Assign merge request to a user (by ID).
    pub fn assignee(&mut self, assignee: u64) -> &mut Self {
        let assignee = match self.assignee.take() {
            Some(Some(Assignee::Ids(mut set))) => {
                set.insert(assignee);
                Assignee::Ids(set)
            },
            Some(Some(Assignee::Id(old_id))) => {
                let set = [old_id, assignee].iter().copied().collect();
                Assignee::Ids(set)
            },
            _ => Assignee::Id(assignee),
        };
        self.assignee = Some(Some(assignee));
        self
    }

    /// Assign merge request to users (by ID).
    pub fn assignees<I>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = u64>,
    {
        let assignee = match self.assignee.take() {
            Some(Some(Assignee::Ids(mut set))) => {
                set.extend(iter);
                Assignee::Ids(set)
            },
            Some(Some(Assignee::Id(old_id))) => {
                let set = iter.chain(iter::once(old_id)).collect();
                Assignee::Ids(set)
            },
            _ => Assignee::Ids(iter.collect()),
        };
        self.assignee = Some(Some(assignee));
        self
    }

    /// Filter merge requests without a reviewer.
    pub fn without_reviewer(&mut self) -> &mut Self {
        self.reviewer = Some(Some(Reviewer::Unassigned));
        self
    }

    /// Filter merge requests reviewed by a user (by ID).
    pub fn reviewer(&mut self, reviewer: u64) -> &mut Self {
        let reviewer = match self.reviewer.take() {
            Some(Some(Reviewer::Ids(mut set))) => {
                set.insert(reviewer);
                Reviewer::Ids(set)
            },
            _ => Reviewer::Ids(iter::once(reviewer).collect()),
        };
        self.reviewer = Some(Some(reviewer));
        self
    }

    /// Filter merge requests reviewed by users (by ID).
    pub fn reviewers<I>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = u64>,
    {
        let reviewer = match self.reviewer.take() {
            Some(Some(Reviewer::Ids(mut set))) => {
                set.extend(iter);
                Reviewer::Ids(set)
            },
            _ => Reviewer::Ids(iter.collect()),
        };
        self.reviewer = Some(Some(reviewer));
        self
    }

    /// Remove all labels from the issue.
    #[deprecated(note = "use `clear_labels` instead")]
    pub fn remove_labels(&mut self) -> &mut Self {
        self.clear_labels()
    }

    /// Remove all labels from the issue.
    pub fn clear_labels(&mut self) -> &mut Self {
        self.labels = Some(Some(MergeRequestLabels::Unlabeled));
        self
    }

    /// Add a label
    pub fn label<L>(&mut self, label: L) -> &mut Self
    where
        L: Into<Cow<'a, str>>,
    {
        let label = label.into();
        let labels = if let Some(Some(MergeRequestLabels::Labeled(mut set))) = self.labels.take() {
            set.push(label);
            set
        } else {
            iter::once(label).collect()
        };
        self.labels = Some(Some(MergeRequestLabels::Labeled(labels)));
        self
    }

    /// Add multiple labels
    pub fn labels<I, L>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = L>,
        L: Into<Cow<'a, str>>,
    {
        let iter = iter.map(Into::into);
        let labels = if let Some(Some(MergeRequestLabels::Labeled(mut set))) = self.labels.take() {
            set.extend(iter);
            set
        } else {
            iter.collect()
        };
        self.labels = Some(Some(MergeRequestLabels::Labeled(labels)));
        self
    }

    /// Add a label to the merge request.
    ///
    /// This is an incremental addition to the existing set of labels on the merge request.
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

    /// Remove a label from the merge request.
    ///
    /// This is an incremental remove form the existing set of labels on the merge request.
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

impl<'a> Endpoint for EditMergeRequest<'a> {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/merge_requests/{}",
            self.project, self.merge_request,
        )
        .into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params
            .push_opt("target_branch", self.target_branch.as_ref())
            .push_opt("title", self.title.as_ref())
            .push_opt("milestone_id", self.milestone_id)
            .push_opt("labels", self.labels.as_ref())
            .push_opt("add_labels", self.add_labels.as_ref())
            .push_opt("remove_labels", self.remove_labels.as_ref())
            .push_opt("description", self.description.as_ref())
            .push_opt("state_event", self.state_event)
            .push_opt("remove_source_branch", self.remove_source_branch)
            .push_opt("squash", self.squash)
            .push_opt("discussion_locked", self.discussion_locked)
            .push_opt("allow_collaboration", self.allow_collaboration);

        if let Some(assignee) = self.assignee.as_ref() {
            assignee.add_params(&mut params);
        }
        if let Some(reviewer) = self.reviewer.as_ref() {
            reviewer.add_params(&mut params);
        }

        #[allow(deprecated)]
        {
            params.push_opt("allow_maintainer_to_push", self.allow_maintainer_to_push);
        }

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::merge_requests::{
        EditMergeRequest, EditMergeRequestBuilderError, MergeRequestStateEvent,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn merge_request_state_event_as_str() {
        let items = &[
            (MergeRequestStateEvent::Close, "close"),
            (MergeRequestStateEvent::Reopen, "reopen"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn project_and_merge_request_are_necessary() {
        let err = EditMergeRequest::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, EditMergeRequestBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = EditMergeRequest::builder()
            .merge_request(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, EditMergeRequestBuilderError, "project");
    }

    #[test]
    fn merge_request_is_necessary() {
        let err = EditMergeRequest::builder().project(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, EditMergeRequestBuilderError, "merge_request");
    }

    #[test]
    fn project_and_merge_request_are_sufficient() {
        EditMergeRequest::builder()
            .project(1)
            .merge_request(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/merge_requests/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditMergeRequest::builder()
            .project("simple/project")
            .merge_request(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_target_branch() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/merge_requests/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("target_branch=target%2Fbranch")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditMergeRequest::builder()
            .project("simple/project")
            .merge_request(1)
            .target_branch("target/branch")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_title() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/merge_requests/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("title=title")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditMergeRequest::builder()
            .project("simple/project")
            .merge_request(1)
            .title("title")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_unassigned() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/merge_requests/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("assignee_ids=0")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditMergeRequest::builder()
            .project("simple/project")
            .merge_request(1)
            .unassigned()
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_assignee() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/merge_requests/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("assignee_id=1")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditMergeRequest::builder()
            .project("simple/project")
            .merge_request(1)
            .assignee(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_assignees() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/merge_requests/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("assignee_ids%5B%5D=1", "&assignee_ids%5B%5D=2"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditMergeRequest::builder()
            .project("simple/project")
            .merge_request(1)
            .assignee(1)
            .assignees([1, 2].iter().copied())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_unreviewed() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/merge_requests/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("reviewer_ids=0")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditMergeRequest::builder()
            .project("simple/project")
            .merge_request(1)
            .without_reviewer()
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_reviewer() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/merge_requests/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("reviewer_ids%5B%5D=1")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditMergeRequest::builder()
            .project("simple/project")
            .merge_request(1)
            .reviewer(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_reviewers() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/merge_requests/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("reviewer_ids%5B%5D=1", "&reviewer_ids%5B%5D=2"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditMergeRequest::builder()
            .project("simple/project")
            .merge_request(1)
            .reviewer(1)
            .reviewers([1, 2].iter().copied())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_milestone_id() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/merge_requests/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("milestone_id=1")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditMergeRequest::builder()
            .project("simple/project")
            .merge_request(1)
            .milestone_id(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_labels() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/merge_requests/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("labels=label%2Clabel1%2Clabel2")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditMergeRequest::builder()
            .project("simple/project")
            .merge_request(1)
            .label("label")
            .labels(["label1", "label2"].iter().cloned())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    #[allow(deprecated)]
    fn endpoint_labels_remove() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/merge_requests/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("labels=")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditMergeRequest::builder()
            .project("simple/project")
            .merge_request(1)
            .remove_labels()
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_labels_clear() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/merge_requests/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("labels=")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditMergeRequest::builder()
            .project("simple/project")
            .merge_request(1)
            .clear_labels()
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_add_labels() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/merge_requests/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("add_labels=one%2Ctwo")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditMergeRequest::builder()
            .project("simple/project")
            .merge_request(1)
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
            .endpoint("projects/simple%2Fproject/merge_requests/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("remove_labels=one%2Ctwo")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditMergeRequest::builder()
            .project("simple/project")
            .merge_request(1)
            .remove_label("one")
            .remove_label("two")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_description() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/merge_requests/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("description=description")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditMergeRequest::builder()
            .project("simple/project")
            .merge_request(1)
            .description("description")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_state_event() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/merge_requests/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("state_event=reopen")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditMergeRequest::builder()
            .project("simple/project")
            .merge_request(1)
            .state_event(MergeRequestStateEvent::Reopen)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_remove_source_branch() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/merge_requests/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("remove_source_branch=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditMergeRequest::builder()
            .project("simple/project")
            .merge_request(1)
            .remove_source_branch(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_squash() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/merge_requests/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("squash=false")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditMergeRequest::builder()
            .project("simple/project")
            .merge_request(1)
            .squash(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_discussion_locked() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/merge_requests/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("discussion_locked=false")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditMergeRequest::builder()
            .project("simple/project")
            .merge_request(1)
            .discussion_locked(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_allow_collaboration() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/merge_requests/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("allow_collaboration=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditMergeRequest::builder()
            .project("simple/project")
            .merge_request(1)
            .allow_collaboration(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    #[allow(deprecated)]
    fn endpoint_allow_maintainer_to_push() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/merge_requests/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("allow_maintainer_to_push=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditMergeRequest::builder()
            .project("simple/project")
            .merge_request(1)
            .allow_maintainer_to_push(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
