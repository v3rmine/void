// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Web hook structures
//!
//! These hooks are received from Gitlab when registered as a web hook within a project.
//!
//! Gitlab does not have consistent structures for its hooks, so they often change from
//! version to version.

use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use log::error;
use serde::de::{Error, Unexpected};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{self, Value};

use crate::types::{
    IssueId, IssueInternalId, IssueState, JobId, MergeRequestId, MergeRequestInternalId,
    MergeRequestState, MergeStatus, MilestoneId, NoteId, NoteType, NoteableId, ObjectId,
    PipelineId, ProjectId, RunnerId, SnippetId, StatusState, UserId,
};

/// A wrapper struct for dates in web hooks.
///
/// Gitlab does not use a standard date format for dates in web hooks. This structure supports
/// deserializing the formats that have been observed.
#[derive(Debug, Clone, Copy)]
pub struct HookDate(DateTime<Utc>);

impl Serialize for HookDate {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HookDate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val = String::deserialize(deserializer)?;

        Utc.datetime_from_str(&val, "%Y-%m-%d %H:%M:%S UTC")
            .or_else(|_| DateTime::parse_from_rfc3339(&val).map(|dt| dt.with_timezone(&Utc)))
            .or_else(|_| {
                DateTime::parse_from_str(&val, "%Y-%m-%d %H:%M:%S %z")
                    .map(|dt| dt.with_timezone(&Utc))
            })
            .map_err(|err| {
                D::Error::invalid_value(
                    Unexpected::Other("hook date"),
                    &format!("Unsupported format: {} {:?}", val, err).as_str(),
                )
            })
            .map(HookDate)
    }
}

impl AsRef<DateTime<Utc>> for HookDate {
    fn as_ref(&self) -> &DateTime<Utc> {
        &self.0
    }
}

/// Project information exposed in hooks.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectHookAttrs {
    /// The display name of the project.
    pub name: String,
    /// The description of the project.
    pub description: Option<String>,
    /// The URL for the project's homepage.
    pub web_url: String,
    /// The URL to the project avatar.
    pub avatar_url: Option<String>,
    /// The URL to clone the repository over SSH.
    pub git_ssh_url: String,
    /// The URL to clone the repository over HTTPS.
    pub git_http_url: String,
    /// The namespace the project lives in.
    pub namespace: String,
    /// Integral value for the project's visibility.
    pub visibility_level: u64,
    /// The path to the project's repository with its namespace.
    pub path_with_namespace: String,
    /// The default branch for the project.
    pub default_branch: Option<String>,
    homepage: String,
    http_url: String,
    ssh_url: String,
    url: String,
}

/// Wiki project information exposed in hooks.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectWikiHookAttrs {
    /// The URL for the project's homepage.
    pub web_url: String,
    /// The URL to clone the repository over SSH.
    pub git_ssh_url: String,
    /// The URL to clone the repository over HTTPS.
    pub git_http_url: String,
    /// The path to the project's repository with its namespace.
    pub path_with_namespace: String,
    /// The default branch for the project.
    pub default_branch: String,
}

/// User information exposed in hooks.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserHookAttrs {
    /// The name of the user.
    pub name: String,
    /// The handle of the user.
    pub username: String,
    /// The URL to the avatar of the user.
    pub avatar_url: Option<String>,
}

/// The identity of a user exposed through a hook.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HookCommitIdentity {
    /// The name of the author or committer.
    pub name: String,
    /// The email address of the author or committer.
    pub email: String,
}

/// Commit information exposed in hooks.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommitHookAttrs {
    /// The commit's ID.
    pub id: ObjectId,
    /// The commit message.
    pub message: String,
    pub timestamp: DateTime<Utc>,
    /// The URL of the commit.
    pub url: String,
    /// The author of the commit.
    pub author: HookCommitIdentity,
    pub added: Option<Vec<String>>,
    pub modified: Option<Vec<String>>,
    pub removed: Option<Vec<String>>,
}

/// A push hook.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PushHook {
    /// The event which occurred.
    pub object_kind: String,
    /// XXX(gitlab): Bug in Gitlab; it should not send this.
    event_name: String,
    /// The old object ID of the ref before the push.
    pub before: ObjectId,
    /// The new object ID of the ref after the push.
    pub after: ObjectId,
    #[serde(rename = "ref")]
    /// The name of the reference which has been pushed.
    pub ref_: String,
    /// The new object ID of the ref after the push.
    pub checkout_sha: Option<ObjectId>,
    /// The message for the push (used for annotated tags).
    pub message: Option<String>,
    /// The ID of the user who pushed.
    pub user_id: UserId,
    /// The name of the user who pushed.
    pub user_name: String,
    /// The username of the user who pushed.
    pub user_username: String,
    /// The email address of the user who pushed.
    pub user_email: Option<String>,
    /// The URL of the user's avatar.
    pub user_avatar: Option<String>,
    /// The ID of the project pushed to.
    pub project_id: ProjectId,
    /// Attributes of the project.
    pub project: ProjectHookAttrs,
    /// The commits pushed to the repository.
    ///
    /// Limited to 20 commits.
    pub commits: Vec<CommitHookAttrs>, // limited to 20 commits
    /// The total number of commits pushed.
    pub total_commits_count: u64,
    repository: Value,
}

/// Actions which may occur on an issue.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum IssueAction {
    /// The issue was updated.
    #[serde(rename = "update")]
    Update,
    /// The issue was opened.
    #[serde(rename = "open")]
    Open,
    /// The issue was closed.
    #[serde(rename = "close")]
    Close,
    /// The issue was reopened.
    #[serde(rename = "reopen")]
    Reopen,
}

/// Issue information exposed in hooks.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssueHookAttrs {
    /// The ID of the issue.
    pub id: IssueId,
    /// The title of the issue.
    pub title: String,
    /// The ID of the assignee of the issue.
    pub assignee_id: Option<UserId>,
    /// The ID of the author of the issue.
    pub author_id: UserId,
    /// The ID of the project.
    pub project_id: ProjectId,
    /// When the issue was created.
    pub created_at: HookDate,
    /// When the issue was last updated.
    pub updated_at: HookDate,
    /// When the issue was deleted.
    pub deleted_at: Option<HookDate>,
    /// When the issue was closed.
    pub closed_at: Option<HookDate>,
    /// When the issue is due.
    pub due_date: Option<NaiveDate>,
    /// The ID of the user which last updated the issue.
    pub updated_by_id: Option<UserId>,
    pub moved_to_id: Option<Value>, // ???
    /// The branch name for the issue.
    pub branch_name: Option<String>,
    /// The description of the issue.
    pub description: Option<String>,
    /// The ID of the milestone of the issue.
    pub milestone_id: Option<MilestoneId>,
    /// The state of the issue.
    pub state: IssueState,
    /// The user-visible ID of the issue.
    pub iid: IssueInternalId,
    /// Whether the issue is confidential or not.
    pub confidential: bool,
    /// The time estimate, in seconds.
    pub time_estimate: u64,
    /// The total time spent, in seconds.
    pub total_time_spent: u64,
    /// The time estimate, as a human-readable string.
    pub human_time_estimate: Option<String>,
    /// The total time spent, as a human-readable string.
    pub human_total_time_spent: Option<String>,

    // It seems that notes miss these properties?
    /// The URL of the issue.
    pub url: Option<String>,
    /// The type of action which caused the hook.
    pub action: Option<IssueAction>,
}

/// An issue hook.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssueHook {
    /// The event which occurred.
    pub object_kind: String,
    /// The user which triggered the hook.
    pub user: UserHookAttrs,
    /// The project the hook was created for.
    pub project: ProjectHookAttrs,
    /// Attributes of the issue.
    pub object_attributes: IssueHookAttrs,
    /// The assignee of the issue.
    pub assignee: Option<UserHookAttrs>,
}

/// Actions which may occur on a merge request.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum MergeRequestAction {
    /// The merge request was updated.
    #[serde(rename = "update")]
    Update,
    /// The merge request was opened.
    #[serde(rename = "open")]
    Open,
    /// The merge request was closed.
    #[serde(rename = "close")]
    Close,
    /// The merge request was reopened.
    #[serde(rename = "reopen")]
    Reopen,
    /// The merge request was approved.
    #[serde(rename = "approved")]
    Approved,
    /// A merge request approval was revoked.
    #[serde(rename = "unapproved")]
    Unapproved,
    /// The merge request was merged.
    #[serde(rename = "merge")]
    Merge,
}

/// Merge parameters for a merge request.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MergeRequestParams {
    force_remove_source_branch: Option<Value>, // sigh
}

impl MergeRequestParams {
    /// Whether the author of the merge request indicated that the source branch should be deleted
    /// upon merge or not.
    // https://gitlab.com/gitlab-org/gitlab-ce/issues/20880
    pub fn force_remove_source_branch(&self) -> bool {
        self.force_remove_source_branch
            .as_ref()
            .map_or(false, |val| {
                if let Some(as_str) = val.as_str() {
                    as_str == "1"
                } else if let Some(as_bool) = val.as_bool() {
                    as_bool
                } else {
                    error!(
                        target: "gitlab",
                        "unknown value for force_remove_source_branch: {}",
                        val,
                    );
                    false
                }
            })
    }
}

/// Merge request information exposed in hooks.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MergeRequestHookAttrs {
    /// The source project of the merge request.
    ///
    /// If this is `None`, the source repository has been deleted.
    pub source: Option<ProjectHookAttrs>,
    /// The target project of the merge request.
    pub target: ProjectHookAttrs,
    pub last_commit: Option<CommitHookAttrs>,
    /// Whether the merge request is a work-in-progress or not.
    pub work_in_progress: bool,
    /// The object ID of the merge commit which is currently being handled.
    pub in_progress_merge_commit_sha: Option<ObjectId>,

    /// The ID of the merge request.
    pub id: MergeRequestId,
    /// The target branch of the merge request.
    pub target_branch: String,
    /// The ID of the target project.
    pub target_project_id: ProjectId,
    /// The source branch of the merge request.
    pub source_branch: String,
    /// The ID of the source project.
    pub source_project_id: Option<ProjectId>,
    /// The ID of the author of the merge request.
    pub author_id: UserId,
    /// The ID of the assignee of the merge request.
    pub assignee_id: Option<UserId>,
    /// The title of the merge request.
    pub title: String,
    /// When the merge request was created.
    pub created_at: HookDate,
    /// When the merge request was last updated.
    pub updated_at: HookDate,
    /// When the merge request was deleted.
    pub deleted_at: Option<HookDate>,
    /// When the merge request was locked.
    pub locked_at: Option<HookDate>,
    /// The ID of the user which last updated the merge request.
    pub updated_by_id: Option<UserId>,
    /// The object ID of the commit which merged the merge request.
    pub merge_commit_sha: Option<ObjectId>,
    pub merge_error: Option<Value>, // String?
    /// The parameters for merging the merge request.
    pub merge_params: MergeRequestParams,
    /// The user which merged the merge request.
    pub merge_user_id: Option<UserId>,
    /// Whether the merge request will be merged once all builds succeed or not.
    pub merge_when_pipeline_succeeds: bool,
    // st_commits
    // st_diffs
    /// The milestone of the merge request.
    pub milestone_id: Option<MilestoneId>,
    pub oldrev: Option<ObjectId>,
    /// The state of the merge request.
    pub state: MergeRequestState,
    /// The merge status of the merge request.
    pub merge_status: MergeStatus,
    /// The user-visible ID of the merge request.
    pub iid: MergeRequestInternalId,
    /// The description of the merge request.
    pub description: Option<String>,

    /// The newest pipeline, if any
    pub head_pipeline_id: Option<PipelineId>,

    // It seems that notes miss these properties?
    /// The URL of the merge request.
    pub url: Option<String>,
    /// The type of action which caused the hook.
    pub action: Option<MergeRequestAction>,
    pub time_estimate: u64,
    lock_version: Option<u64>,
}

/// A merge request hook.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MergeRequestHook {
    /// The event which occurred.
    pub object_kind: String,
    /// The user which triggered the hook.
    pub user: UserHookAttrs,
    /// The project the hook was created for.
    pub project: ProjectHookAttrs,
    /// Attributes of the merge request.
    pub object_attributes: MergeRequestHookAttrs,
    /// The assignee of the merge request.
    pub assignee: Option<UserHookAttrs>,
    repository: Value,

    /// Details about the changes on the MR
    pub changes: Option<MergeRequestChanges>,
}

/// MR Changes
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MergeRequestChanges {
    pub updated_at: Option<MergeRequestChange<String>>,
    pub merge_status: Option<MergeRequestChange<String>>,
    pub merge_commit_sha: Option<MergeRequestChange<Option<String>>>,
}
/// A MR change
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MergeRequestChange<T> {
    pub previous: T,
    pub current: T,
}

/// The type of a snippet.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnippetType {
    /// A project-owned snippet.
    #[serde(rename = "ProjectSnippet")]
    Project,
    /// A user-owned snippet.
    #[serde(rename = "PersonalSnippet")]
    Personal,
}

/// Snippet information exposed in hooks.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SnippetHookAttrs {
    /// The title of the snippet.
    pub title: String,
    /// The content of the snippet.
    pub content: String,
    /// The author of the snippet.
    pub author_id: UserId,
    /// The project the snippet belongs to.
    pub project_id: Option<ProjectId>,
    /// When the snippet was created.
    pub created_at: HookDate,
    /// When the snippet was last updated.
    pub updated_at: HookDate,
    /// The name of the snippet.
    pub file_name: String,
    #[serde(rename = "type")]
    /// The type of the snippet.
    pub type_: SnippetType,
    /// The visibility of the snippet.
    pub visibility_level: u64,
}

/// Actions which may occur on a wiki page.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum WikiPageAction {
    /// A wiki page was created.
    #[serde(rename = "create")]
    Create,
    /// A wiki page was updated.
    #[serde(rename = "update")]
    Update,
}

/// Wiki information exposed in hooks.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WikiPageHookAttrs {
    /// The title of the wiki page.
    pub title: String,
    /// The content of the wiki page.
    pub content: String,
    pub format: String,
    pub message: String,
    /// The slug of the wiki page.
    pub slug: String,

    /// The URL of the wiki page.
    pub url: String,
    /// The type of action which caused the hook.
    pub action: WikiPageAction,
}

/// Diff information exposed in hooks.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiffHookAttrs {
    pub diff: String,
    /// The path on the new side of the diff.
    pub new_path: String,
    /// The path on the old side of the diff.
    pub old_path: String,
    /// The mode on the old side of the diff.
    // TODO: Create a mode type.
    pub a_mode: String,
    /// The mode on the new side of the diff.
    pub b_mode: String,
    /// Whether the diff indicates the addition of a file.
    pub new_file: bool,
    /// Whether the diff indicates the rename of a file.
    pub renamed_file: bool,
    /// Whether the diff indicates the deletion of a file.
    pub deleted_file: bool,
    pub too_large: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
// FIXME(gitlab#21467): This can apparently be a string sometimes.
// https://gitlab.com/gitlab-org/gitlab-ce/issues/21467
pub struct PositionHookAttrs {
    pub base_sha: ObjectId,
    pub head_sha: ObjectId,
    pub start_sha: ObjectId,
    pub old_line: Option<u64>,
    /// The path on the old side of the diff.
    pub old_path: String,
    pub new_line: Option<u64>,
    /// The path on the new side of the diff.
    pub new_path: String,
}

/// Note (comment) information exposed in hooks.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NoteHookAttrs {
    /// The ID of the note.
    pub id: NoteId,
    /// The content of the note.
    pub note: String,
    /// The type of entity the note is attached to.
    pub noteable_type: NoteType,
    // pub original_position: Option<PositionHookAttrs>,
    original_position: Value,
    // pub position: Option<PositionHookAttrs>,
    position: Value,
    /// The author of the note.
    pub author_id: UserId,
    /// When the note was created.
    pub created_at: HookDate,
    /// When the note was last updated.
    pub updated_at: HookDate,
    /// The ID of the user who last updated the note.
    pub updated_by_id: Option<UserId>,
    /// When the note was marked as resolved.
    pub resolved_at: Option<HookDate>,
    /// The ID of the user who marked the note as resolved.
    pub resolved_by_id: Option<UserId>,
    /// The ID of the project.
    pub project_id: ProjectId,
    /// The URL of an attachment to the note.
    pub attachment: Option<String>,
    pub line_code: Option<String>, // XXX: This is some internal format.
    pub commit_id: Option<ObjectId>, // XXX(8.11): apparently can be an empty string?
    pub discussion_id: ObjectId,
    pub original_discussion_id: Option<ObjectId>,
    noteable_id: Value, // Keep as JSON because its type depends on what `noteable_type` is.
    /// Whether the note was created by a user or in response to an external action.
    pub system: bool,
    pub st_diff: Option<DiffHookAttrs>,
    /// The URL of the note.
    pub url: String,

    // XXX: What is this field?
    #[serde(rename = "type")]
    pub type_: Option<String>,
    // XXX: Seems to have been removed?
    // pub is_award: bool,
}

impl NoteHookAttrs {
    /// The ID of the object the note is for.
    pub fn noteable_id(&self) -> Option<NoteableId> {
        match self.noteable_type {
            NoteType::Commit => self
                .noteable_id
                .as_str()
                .map(|id| NoteableId::Commit(ObjectId::new(id))),
            NoteType::Issue => self
                .noteable_id
                .as_u64()
                .map(|id| NoteableId::Issue(IssueId::new(id))),
            NoteType::MergeRequest => self
                .noteable_id
                .as_u64()
                .map(|id| NoteableId::MergeRequest(MergeRequestId::new(id))),
            NoteType::Snippet => self
                .noteable_id
                .as_u64()
                .map(|id| NoteableId::Snippet(SnippetId::new(id))),
        }
    }
}

/// A note hook.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NoteHook {
    /// The event which occurred.
    pub object_kind: String,
    /// The user who triggered the hook.
    pub user: UserHookAttrs,
    /// The ID of the project the note belongs to.
    pub project_id: ProjectId,
    /// The project the note belongs to.
    pub project: ProjectHookAttrs,
    /// The attributes on the note itself.
    pub object_attributes: NoteHookAttrs,
    /// The commit the note is associated with (for commit notes).
    pub commit: Option<CommitHookAttrs>,
    /// The issue the note is associated with (for issue notes).
    pub issue: Option<IssueHookAttrs>,
    /// The merge request the note is associated with (for merge request notes).
    pub merge_request: Option<MergeRequestHookAttrs>,
    /// The snippet the note is associated with (for snippet notes).
    pub snippet: Option<SnippetHookAttrs>,
    repository: Value,
}

/// Build user information exposed in hooks.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BuildUserHookAttrs {
    /// The ID of the user.
    pub id: Option<UserId>,
    /// The user's name.
    pub name: Option<String>,
    /// The user's email address.
    pub email: Option<String>,
}

/// Build commit information exposed in hooks.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BuildCommitHookAttrs {
    pub id: String,
    /// The object ID of the commit.
    pub sha: ObjectId,
    /// The full commit message.
    pub message: String,
    /// The commit's author's name.
    pub author_name: String,
    /// The commit's author's email address.
    pub author_email: String,
    pub status: String,
    pub duration: u64,
    /// When the build started.
    pub started_at: Option<HookDate>,
    /// When the build completed.
    pub finished_at: Option<HookDate>,
}

/// Project information exposed in build hooks.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BuildProjectHookAttrs {
    /// The display name of the project.
    pub name: String,
    /// The description of the project.
    pub description: Option<String>,
    /// The URL for the project's homepage.
    pub homepage: String,
    /// The URL to clone the repository over HTTPS.
    pub git_http_url: String,
    /// The URL to clone the repository over SSH.
    pub git_ssh_url: String,
    /// Integral value for the project's visibility.
    pub visibility_level: u64,
}

/// A build hook.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BuildHook {
    /// The event which occurred.
    pub object_kind: String,
    #[serde(rename = "ref")]
    /// The name of the reference that was tested.
    pub ref_: String,
    pub tag: String,
    pub before_sha: String,
    /// The object ID that was built.
    pub sha: String,
    /// The ID of the build.
    pub build_id: JobId,
    /// The name of the build.
    pub build_name: String,
    pub build_stage: String,
    /// When the build started.
    pub build_started_at: Option<HookDate>,
    /// When the build completed.
    pub build_finished_at: Option<HookDate>,
    pub build_duration: Option<u64>,
    /// Whether the build is allowed to fail.
    pub build_allow_failure: bool,
    /// The ID of the project.
    pub project_id: ProjectId,
    /// The user which owns the build.
    pub user: BuildUserHookAttrs,
    /// The commit which was built.
    pub commit: BuildCommitHookAttrs,
    /// The repository the build is for.
    pub repository: BuildProjectHookAttrs,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PipelineVariable {
    /// Environment variable key
    pub key: String,
    /// Environment variable value
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PipelineHookAttrs {
    pub id: PipelineId,
    /// The object ID that was tested.
    pub sha: ObjectId,
    #[serde(rename = "ref")]
    /// The name of the reference that was tested.
    pub ref_: Option<String>,
    /// The status of the pipeline.
    pub status: StatusState,
    pub before_sha: String,
    /// Was this pipeline triggered by a tag.
    pub tag: bool,
    /// When the pipeline was created.
    pub created_at: HookDate,
    /// When the pipeline completed.
    pub finished_at: Option<HookDate>,
    /// Duration of pipeline in seconds.
    pub duration: Option<u64>,
    /// What triggered the pipeline.
    pub source: String,
    /// The stages of the pipeline.
    pub stages: Vec<String>,
    /// Environment variables manually set by the user starting the pipeline.
    pub variables: Vec<PipelineVariable>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PipelineBuildRunner {
    /// The runner id.
    pub id: RunnerId,
    /// The runner description
    pub description: String,
    /// Whether the runner is active.
    pub active: bool,
    /// Whether the runner is shared.
    pub is_shared: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PipelineMergeRequestAttrs {
    pub id: MergeRequestId,
    pub iid: MergeRequestInternalId,
    /// The title of the merge request.
    pub title: String,
    /// The target branch of the merge request.
    pub target_branch: String,
    /// The ID of the target project.
    pub target_project_id: ProjectId,
    /// The source branch of the merge request.
    pub source_branch: String,
    /// The ID of the source project.
    pub source_project_id: Option<ProjectId>,
    pub state: MergeRequestState,
    pub merge_status: MergeStatus,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PipelineProjectAttrs {
    pub id: ProjectId,
    /// The display name of the project.
    pub name: String,
    /// The description of the project.
    pub description: Option<String>,
    /// The URL for the project's homepage.
    pub web_url: String,
    /// The URL to the project avatar.
    pub avatar_url: Option<String>,
    /// The URL to clone the repository over SSH.
    pub git_ssh_url: String,
    /// The URL to clone the repository over HTTPS.
    pub git_http_url: String,
    /// The namespace the project lives in.
    pub namespace: String,
    /// Integral value for the project's visibility.
    pub visibility_level: u64,
    /// The path to the project's repository with its namespace.
    pub path_with_namespace: String,
    /// The default branch for the project.
    pub default_branch: Option<String>,
    /// The path to the ci config file.
    pub ci_config_path: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PipelineHook {
    /// The event which occured.
    pub object_kind: String,
    /// The pipeline.
    pub object_attributes: PipelineHookAttrs,
    /// The merge request this pipeline is running for.
    pub merge_request: Option<PipelineMergeRequestAttrs>,
    /// The user that started the the pipeline.
    pub user: UserHookAttrs,
    /// The project this pipeline is running in.
    pub project: PipelineProjectAttrs,
    /// The commit this pipeline is running for
    pub commit: Option<CommitHookAttrs>,
}

/// A wiki page hook.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WikiPageHook {
    /// The event which occurred.
    pub object_kind: String,
    /// The user who caused the hook.
    pub user: UserHookAttrs,
    /// The project the wiki belongs to.
    pub project: ProjectHookAttrs,
    /// The wiki the page belongs to.
    pub wiki: ProjectWikiHookAttrs,
    /// The wiki page.
    pub object_attributes: WikiPageHookAttrs,
}

/// A deserializable structure for all Gitlab web hooks.
#[derive(Debug, Clone)]
pub enum WebHook {
    /// A push hook.
    Push(Box<PushHook>),
    /// An issue hook.
    Issue(Box<IssueHook>),
    /// A merge request hook.
    MergeRequest(Box<MergeRequestHook>),
    /// A note hook.
    Note(Box<NoteHook>),
    /// A build hook.
    Build(Box<BuildHook>),
    /// A pipeline hook.
    Pipeline(Box<PipelineHook>),
    /// A wiki page hook.
    WikiPage(Box<WikiPageHook>),
}

impl<'de> Deserialize<'de> for WebHook {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val = <Value as Deserialize>::deserialize(deserializer)?;

        let object_kind = match val.pointer("/object_kind") {
            Some(Value::String(kind)) => kind,
            Some(_) => {
                return Err(D::Error::invalid_type(
                    Unexpected::Other("JSON value"),
                    &"a string",
                ));
            },
            None => {
                return Err(D::Error::missing_field("object_kind"));
            },
        };

        let hook_res = match object_kind.as_ref() {
            "push" | "tag_push" => {
                serde_json::from_value(val).map(|hook| WebHook::Push(Box::new(hook)))
            },

            "issue" => serde_json::from_value(val).map(|hook| WebHook::Issue(Box::new(hook))),

            "merge_request" => {
                serde_json::from_value(val).map(|hook| WebHook::MergeRequest(Box::new(hook)))
            },

            "note" => serde_json::from_value(val).map(|hook| WebHook::Note(Box::new(hook))),

            "build" => serde_json::from_value(val).map(|hook| WebHook::Build(Box::new(hook))),

            "pipeline" => serde_json::from_value(val).map(|hook| WebHook::Pipeline(Box::new(hook))),

            _ => {
                return Err(D::Error::invalid_value(
                    Unexpected::Other("object kind"),
                    &format!("unrecognized webhook object kind: {}", object_kind).as_str(),
                ));
            },
        };

        hook_res.map_err(|err| {
            D::Error::invalid_value(
                Unexpected::Other("web hook"),
                &format!("{:?}", err).as_str(),
            )
        })
    }
}
