// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::BTreeSet;
use std::iter;

use derive_builder::Builder;

use crate::api::common::{CommaSeparatedList, NameOrId};
use crate::api::endpoint_prelude::*;

#[derive(Debug, Clone)]
pub(crate) enum Assignee {
    Unassigned,
    Id(u64),
    Ids(BTreeSet<u64>),
}

impl Assignee {
    pub(crate) fn add_params<'a>(&'a self, params: &mut FormParams<'a>) {
        match self {
            Assignee::Unassigned => {
                params.push("assignee_ids", "0");
            },
            Assignee::Id(id) => {
                params.push("assignee_id", *id);
            },
            Assignee::Ids(ids) => {
                params.extend(ids.iter().map(|&id| ("assignee_ids[]", id)));
            },
        }
    }
}

/// Parameters for setting the reviewer(s) of a merge request.
#[derive(Debug, Clone)]
pub(crate) enum Reviewer {
    /// Unset all reviewers.
    Unassigned,
    /// A set of reviewers.
    Ids(BTreeSet<u64>),
}

impl Reviewer {
    pub(crate) fn add_params<'a>(&'a self, params: &mut FormParams<'a>) {
        match self {
            Reviewer::Unassigned => {
                params.push("reviewer_ids", "0");
            },
            Reviewer::Ids(ids) => {
                params.extend(ids.iter().map(|&id| ("reviewer_ids[]", id)));
            },
        }
    }
}

/// Create a new merge request on project.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct CreateMergeRequest<'a> {
    /// The project to open the merge requset *from*.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The name of the source branch for the merge request.
    #[builder(setter(into))]
    source_branch: Cow<'a, str>,
    /// The name of the target branch for the merge request.
    #[builder(setter(into))]
    target_branch: Cow<'a, str>,
    /// The title for the merge request.
    #[builder(setter(into))]
    title: Cow<'a, str>,

    /// The assignee of the merge request.
    #[builder(setter(name = "_assignee"), default, private)]
    assignee: Option<Assignee>,
    #[builder(setter(name = "_reviewer"), default, private)]
    reviewer: Option<Reviewer>,
    /// The description of the merge request.
    #[builder(setter(into), default)]
    description: Option<Cow<'a, str>>,
    /// The ID of the target project for the merge request.
    #[builder(default)]
    target_project_id: Option<u64>,
    /// Labels to add to the merge request.
    #[builder(setter(name = "_labels"), default, private)]
    labels: Option<CommaSeparatedList<Cow<'a, str>>>,
    /// The ID of the milestone to add the merge request to.
    #[builder(default)]
    milestone_id: Option<u64>,
    /// How many approvals are required before merging will be allowed.
    ///
    /// Note that this must be more than the project limit (if present).
    #[builder(default)]
    approvals_before_merge: Option<u64>,
    /// Whether to remove the source branch once merged or not.
    #[builder(default)]
    remove_source_branch: Option<bool>,
    /// Whether to allow collaboration with maintainers of the target project or not.
    #[builder(default)]
    allow_collaboration: Option<bool>,
    /// Whether to squash the branch when merging or not.
    #[builder(default)]
    squash: Option<bool>,

    /// Whether to allow collaboration with maintainers of the target project or not.
    #[deprecated(note = "use `allow_collaboration` instead")]
    #[builder(default)]
    allow_maintainer_to_push: Option<bool>,
}

impl<'a> CreateMergeRequest<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> CreateMergeRequestBuilder<'a> {
        CreateMergeRequestBuilder::default()
    }
}

impl<'a> CreateMergeRequestBuilder<'a> {
    /// Filter unassigned merge requests.
    pub fn unassigned(&mut self) -> &mut Self {
        self.assignee = Some(Some(Assignee::Unassigned));
        self
    }

    /// Filter merge requests assigned to a user (by ID).
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

    /// Filter merge requests assigned to a users (by ID).
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

    /// Add a label.
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

    /// Add multiple labels.
    pub fn labels<I, L>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = L>,
        L: Into<Cow<'a, str>>,
    {
        self.labels
            .get_or_insert(None)
            .get_or_insert_with(CommaSeparatedList::new)
            .extend(iter.map(Into::into));
        self
    }
}

impl<'a> Endpoint for CreateMergeRequest<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/merge_requests", self.project).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params
            .push("source_branch", self.source_branch.as_ref())
            .push("target_branch", self.target_branch.as_ref())
            .push("title", self.title.as_ref())
            .push_opt("description", self.description.as_ref())
            .push_opt("target_project_id", self.target_project_id)
            .push_opt("milestone_id", self.milestone_id)
            .push_opt("labels", self.labels.as_ref())
            .push_opt("approvals_before_merge", self.approvals_before_merge)
            .push_opt("remove_source_branch", self.remove_source_branch)
            .push_opt("allow_collaboration", self.allow_collaboration)
            .push_opt("squash", self.squash);

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
        CreateMergeRequest, CreateMergeRequestBuilderError,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_source_branch_target_branch_and_title_are_necessary() {
        let err = CreateMergeRequest::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, CreateMergeRequestBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = CreateMergeRequest::builder()
            .source_branch("source")
            .target_branch("target")
            .title("title")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateMergeRequestBuilderError, "project");
    }

    #[test]
    fn source_branch_is_necessary() {
        let err = CreateMergeRequest::builder()
            .project(1)
            .target_branch("target")
            .title("title")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateMergeRequestBuilderError, "source_branch");
    }

    #[test]
    fn target_branch_is_necessary() {
        let err = CreateMergeRequest::builder()
            .project(1)
            .source_branch("source")
            .title("title")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateMergeRequestBuilderError, "target_branch");
    }

    #[test]
    fn title_is_necessary() {
        let err = CreateMergeRequest::builder()
            .project(1)
            .source_branch("source")
            .target_branch("target")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateMergeRequestBuilderError, "title");
    }

    #[test]
    fn project_source_branch_target_branch_and_title_are_sufficient() {
        CreateMergeRequest::builder()
            .project(1)
            .source_branch("source")
            .target_branch("target")
            .title("title")
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/merge_requests")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "source_branch=source%2Fbranch",
                "&target_branch=target%2Fbranch",
                "&title=title",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateMergeRequest::builder()
            .project("simple/project")
            .source_branch("source/branch")
            .target_branch("target/branch")
            .title("title")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_unassigned() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/merge_requests")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "source_branch=source%2Fbranch",
                "&target_branch=target%2Fbranch",
                "&title=title",
                "&assignee_ids=0",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateMergeRequest::builder()
            .project("simple/project")
            .source_branch("source/branch")
            .target_branch("target/branch")
            .title("title")
            .unassigned()
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_assignee() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/merge_requests")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "source_branch=source%2Fbranch",
                "&target_branch=target%2Fbranch",
                "&title=title",
                "&assignee_id=1",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateMergeRequest::builder()
            .project("simple/project")
            .source_branch("source/branch")
            .target_branch("target/branch")
            .title("title")
            .assignee(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_assignees() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/merge_requests")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "source_branch=source%2Fbranch",
                "&target_branch=target%2Fbranch",
                "&title=title",
                "&assignee_ids%5B%5D=1",
                "&assignee_ids%5B%5D=2",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateMergeRequest::builder()
            .project("simple/project")
            .source_branch("source/branch")
            .target_branch("target/branch")
            .title("title")
            .assignee(1)
            .assignees([1, 2].iter().copied())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_unreviewed() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/merge_requests")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "source_branch=source%2Fbranch",
                "&target_branch=target%2Fbranch",
                "&title=title",
                "&reviewer_ids=0",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateMergeRequest::builder()
            .project("simple/project")
            .source_branch("source/branch")
            .target_branch("target/branch")
            .title("title")
            .without_reviewer()
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_reviewer() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/merge_requests")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "source_branch=source%2Fbranch",
                "&target_branch=target%2Fbranch",
                "&title=title",
                "&reviewer_ids%5B%5D=1",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateMergeRequest::builder()
            .project("simple/project")
            .source_branch("source/branch")
            .target_branch("target/branch")
            .title("title")
            .reviewer(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_reviewers() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/merge_requests")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "source_branch=source%2Fbranch",
                "&target_branch=target%2Fbranch",
                "&title=title",
                "&reviewer_ids%5B%5D=1",
                "&reviewer_ids%5B%5D=2",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateMergeRequest::builder()
            .project("simple/project")
            .source_branch("source/branch")
            .target_branch("target/branch")
            .title("title")
            .reviewer(1)
            .reviewers([1, 2].iter().copied())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_description() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/merge_requests")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "source_branch=source%2Fbranch",
                "&target_branch=target%2Fbranch",
                "&title=title",
                "&description=description",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateMergeRequest::builder()
            .project("simple/project")
            .source_branch("source/branch")
            .target_branch("target/branch")
            .title("title")
            .description("description")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_target_project_id() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/merge_requests")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "source_branch=source%2Fbranch",
                "&target_branch=target%2Fbranch",
                "&title=title",
                "&target_project_id=1",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateMergeRequest::builder()
            .project("simple/project")
            .source_branch("source/branch")
            .target_branch("target/branch")
            .title("title")
            .target_project_id(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_labels() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/merge_requests")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "source_branch=source%2Fbranch",
                "&target_branch=target%2Fbranch",
                "&title=title",
                "&labels=label%2Clabel1%2Clabel2",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateMergeRequest::builder()
            .project("simple/project")
            .source_branch("source/branch")
            .target_branch("target/branch")
            .title("title")
            .label("label")
            .labels(["label1", "label2"].iter().cloned())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_milestone_id() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/merge_requests")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "source_branch=source%2Fbranch",
                "&target_branch=target%2Fbranch",
                "&title=title",
                "&milestone_id=1",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateMergeRequest::builder()
            .project("simple/project")
            .source_branch("source/branch")
            .target_branch("target/branch")
            .title("title")
            .milestone_id(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_approvals_before_merge() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/merge_requests")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "source_branch=source%2Fbranch",
                "&target_branch=target%2Fbranch",
                "&title=title",
                "&approvals_before_merge=2",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateMergeRequest::builder()
            .project("simple/project")
            .source_branch("source/branch")
            .target_branch("target/branch")
            .title("title")
            .approvals_before_merge(2)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_remove_source_branch() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/merge_requests")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "source_branch=source%2Fbranch",
                "&target_branch=target%2Fbranch",
                "&title=title",
                "&remove_source_branch=true",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateMergeRequest::builder()
            .project("simple/project")
            .source_branch("source/branch")
            .target_branch("target/branch")
            .title("title")
            .remove_source_branch(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_allow_collaboration() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/merge_requests")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "source_branch=source%2Fbranch",
                "&target_branch=target%2Fbranch",
                "&title=title",
                "&allow_collaboration=true",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateMergeRequest::builder()
            .project("simple/project")
            .source_branch("source/branch")
            .target_branch("target/branch")
            .title("title")
            .allow_collaboration(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_squash() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/merge_requests")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "source_branch=source%2Fbranch",
                "&target_branch=target%2Fbranch",
                "&title=title",
                "&squash=false",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateMergeRequest::builder()
            .project("simple/project")
            .source_branch("source/branch")
            .target_branch("target/branch")
            .title("title")
            .squash(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    #[allow(deprecated)]
    fn endpoint_allow_maintainer_to_push() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/merge_requests")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "source_branch=source%2Fbranch",
                "&target_branch=target%2Fbranch",
                "&title=title",
                "&allow_maintainer_to_push=true",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateMergeRequest::builder()
            .project("simple/project")
            .source_branch("source/branch")
            .target_branch("target/branch")
            .title("title")
            .allow_maintainer_to_push(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
