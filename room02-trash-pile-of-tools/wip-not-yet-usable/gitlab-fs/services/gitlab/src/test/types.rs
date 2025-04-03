// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::{fs::File, ops::Deref};

use chrono::{DateTime, NaiveDate, Utc};
use serde::de::DeserializeOwned;
use serde_json::{from_reader, json};

use crate::types::*;

fn check_user_brad_king(user: &UserBasic) {
    assert_eq!(user.username, "brad.king");
    assert_eq!(user.web_url, "https://gitlab.kitware.com/brad.king");
    assert_eq!(user.name, "Brad King");
    assert_eq!(user.state, UserState::Active);
    assert_eq!(
        user.avatar_url.as_ref().unwrap(),
        "https://secure.gravatar.com/avatar/0617392a2f9fd505720d0c42cefc1a10?s=80&d=identicon",
    );
    assert_eq!(user.id, UserId::new(10));
}

fn check_user_ben_boeckel(user: &UserBasic) {
    assert_eq!(user.username, "ben.boeckel");
    assert_eq!(user.web_url, "https://gitlab.kitware.com/ben.boeckel");
    assert_eq!(user.name, "Ben Boeckel");
    assert_eq!(user.state, UserState::Active);
    assert_eq!(
        user.avatar_url.as_ref().unwrap(),
        "https://secure.gravatar.com/avatar/2f5f7e99190174edb5a2f66b8653b0b2?s=80&d=identicon",
    );
    assert_eq!(user.id, UserId::new(13));
}

fn check_user_kwrobot(user: &UserBasic) {
    assert_eq!(user.username, "kwrobot");
    assert_eq!(user.web_url, "https://gitlab.kitware.com/kwrobot");
    assert_eq!(user.name, "Kitware Robot");
    assert_eq!(user.state, UserState::Active);
    assert_eq!(
        user.avatar_url.as_ref().unwrap(),
        "https://gitlab.kitware.com/uploads/-/system/user/avatar/11/avatar.png",
    );
    assert_eq!(user.id, UserId::new(11));
}

fn check_user_buildbot(user: &UserBasic) {
    assert_eq!(
        user.avatar_url.as_ref().unwrap(),
        "https://gitlab.kitware.com/uploads/-/system/user/avatar/35/buildbot-logo.png",
    );
    assert_eq!(user.id, UserId::new(35));
    assert_eq!(user.name, "buildbot");
    assert_eq!(user.username, "buildbot");
    assert_eq!(user.state, UserState::Active);
    assert_eq!(user.web_url, "https://gitlab.kitware.com/buildbot");
}

fn check_namespace_utils(namespace: &Namespace) {
    assert_eq!(namespace.name, "Utils");
    assert_eq!(namespace.path, "utils");
    assert_eq!(namespace.id(), NamespaceId::Group(GroupId::new(498)));
    assert_eq!(namespace.kind, NamespaceKind::Group);
    assert_eq!(namespace.full_path, "utils");
    assert_eq!(namespace.avatar_url, None);
    assert_eq!(namespace.web_url, "https://gitlab.kitware.com/groups/utils");
    assert_eq!(namespace.members_count_with_descendants, None);
}

fn check_empty_time_stats(time_stats: &IssuableTimeStats) {
    assert_eq!(time_stats.time_estimate, 0);
    assert_eq!(time_stats.total_time_spent, 0);
    assert_eq!(time_stats.human_time_estimate, None);
    assert_eq!(time_stats.human_total_time_spent, None);
}

fn datetime(ymd: (i32, u32, u32), time: (u32, u32, u32, u32)) -> DateTime<Utc> {
    NaiveDate::from_ymd_opt(ymd.0, ymd.1, ymd.2)
        .unwrap()
        .and_hms_milli_opt(time.0, time.1, time.2, time.3)
        .unwrap()
        .and_local_timezone(Utc)
        .unwrap()
}

fn read_test_file<T: DeserializeOwned>(name: &str) -> T {
    let fin = File::open(format!(
        concat!(env!("CARGO_MANIFEST_DIR"), "/data/{}.json"),
        name,
    ))
    .unwrap();

    from_reader::<File, T>(fin).unwrap()
}

#[test]
fn test_read_award_emoji() {
    let award_emoji: AwardEmoji = read_test_file("award_emoji");

    assert_eq!(award_emoji.id, AwardId::new(335));
    assert_eq!(award_emoji.name, "8ball");
    check_user_ben_boeckel(&award_emoji.user);
    assert_eq!(
        award_emoji.created_at,
        datetime((2016, 12, 7), (16, 23, 46, 742)),
    );
    assert_eq!(
        award_emoji.updated_at,
        datetime((2016, 12, 7), (16, 23, 46, 742)),
    );
    assert_eq!(
        award_emoji.awardable_id(),
        AwardableId::Note(NoteId::new(177_359)),
    );
    assert_eq!(award_emoji.awardable_type, AwardableType::Note);
}

#[test]
fn test_read_commit_note() {
    let commit_note: CommitNote = read_test_file("commit_note");

    assert_eq!(commit_note.note, "Example commit note for data fetching.");
    assert_eq!(commit_note.path, None);
    assert_eq!(commit_note.line, None);
    assert_eq!(commit_note.line_type, None);
    check_user_ben_boeckel(&commit_note.author);
    assert_eq!(
        commit_note.created_at,
        datetime((2016, 12, 7), (16, 28, 33, 966)),
    );
}

#[test]
fn test_read_commit_status() {
    let commit_status: CommitStatus = read_test_file("commit_status");

    assert_eq!(commit_status.id, CommitStatusId::new(931_434));
    assert_eq!(
        commit_status.sha,
        ObjectId::new("de4ac3cf96cb8a0893be22b03f5171d934f9d392"),
    );
    assert_eq!(commit_status.ref_.as_ref().unwrap(), "master");
    assert_eq!(commit_status.status, StatusState::Success);
    assert_eq!(commit_status.name, "rust-gitlab-megas-linux-debug");
    assert_eq!(
        commit_status.target_url.as_ref().unwrap(),
        "https://buildbot.kitware.com/builders/rust-gitlab-megas-linux-debug/builds/41",
    );
    assert_eq!(commit_status.description.as_ref().unwrap(), "expected");
    assert_eq!(
        commit_status.created_at,
        datetime((2016, 11, 8), (14, 35, 32, 627)),
    );
    assert_eq!(commit_status.started_at, None);
    assert_eq!(
        commit_status.finished_at.unwrap(),
        datetime((2016, 11, 8), (14, 35, 32, 629)),
    );
    assert!(!commit_status.allow_failure);
    check_user_buildbot(&commit_status.author);
    assert_eq!(commit_status.coverage, None);
}

#[test]
fn test_read_issue() {
    let issue: Issue = read_test_file("issue");

    assert_eq!(issue.id, IssueId::new(69328));
    assert_eq!(issue.iid, IssueInternalId::new(6));
    assert_eq!(issue.project_id, ProjectId::new(855));
    assert_eq!(issue.title, "fix documentation warnings");
    assert_eq!(issue.description, Some(String::new()));
    assert_eq!(issue.state, IssueState::Closed);
    assert_eq!(
        issue.created_at,
        datetime((2016, 10, 30), (18, 54, 28, 954)),
    );
    assert_eq!(issue.updated_at, datetime((2019, 7, 17), (13, 53, 48, 869)));
    assert_eq!(issue.closed_at, None);
    assert!(issue.closed_by.is_none());
    itertools::assert_equal(&issue.labels, &["area:doc"]);
    assert!(issue.milestone.is_none());
    check_user_ben_boeckel(&issue.author);
    let assignee = issue.assignee.as_ref().unwrap();
    check_user_ben_boeckel(assignee);
    let assignees = issue.assignees.as_ref().unwrap();
    assert_eq!(assignees.len(), 1);
    let assignee = &assignees[0];
    check_user_ben_boeckel(assignee);
    assert_eq!(issue.subscribed, Some(true));
    assert_eq!(issue.user_notes_count, 0);
    assert_eq!(issue.merge_requests_count, 1);
    assert_eq!(issue.upvotes, 0);
    assert_eq!(issue.downvotes, 0);
    assert_eq!(issue.due_date, None);
    assert_eq!(issue.has_tasks, Some(false));
    assert!(!issue.confidential);
    assert_eq!(issue.discussion_locked, None);
    assert_eq!(
        issue.web_url,
        "https://gitlab.kitware.com/utils/rust-gitlab/-/issues/6",
    );
    assert!(issue.has_links());
}

#[test]
fn test_read_issue_reference() {
    let issue_reference: IssueReference = read_test_file("issue_reference");

    let issue = if let IssueReference::Internal(issue) = issue_reference {
        issue
    } else {
        panic!("expected to have an internal issue reference");
    };

    assert_eq!(issue.id, IssueId::new(69075));
    assert_eq!(issue.iid, IssueInternalId::new(5));
    assert_eq!(issue.project_id, ProjectId::new(855));
    assert_eq!(issue.title, "Add project hook APIs");
    assert_eq!(
        issue.description.as_ref().unwrap(),
        "The workflow currently requires that the robot be able to register itself as a webhook \
         for new projects. An API needs added for this.\n\nCc: @brad.king",
    );
    assert_eq!(issue.state, IssueState::Closed);
    assert_eq!(issue.created_at, datetime((2016, 10, 4), (18, 59, 37, 178)));
    assert_eq!(issue.updated_at, datetime((2017, 7, 7), (6, 31, 5, 370)));
    assert_eq!(issue.closed_at, None);
    assert!(issue.closed_by.is_none());
    assert!(issue.labels.is_empty());
    assert!(issue.milestone.is_none());
    check_user_ben_boeckel(&issue.author);
    let assignee = issue.assignee.as_ref().unwrap();
    check_user_ben_boeckel(assignee);
    assert_eq!(issue.subscribed, None);
    check_empty_time_stats(&issue.time_stats);
    assert_eq!(issue.user_notes_count, 0);
    assert_eq!(issue.merge_requests_count, 1);
    assert_eq!(issue.upvotes, 0);
    assert_eq!(issue.downvotes, 0);
    assert_eq!(issue.due_date, None);
    assert!(!issue.confidential);
    assert_eq!(issue.discussion_locked, None);
    assert_eq!(
        issue.web_url,
        "https://gitlab.kitware.com/utils/rust-gitlab/-/issues/5",
    );
}

#[test]
fn test_read_member() {
    let member: Member = read_test_file("member");

    assert_eq!(member.username, "brad.king");
    assert_eq!(member.name, "Brad King");
    assert_eq!(member.id, UserId::new(10));
    assert_eq!(member.state, UserState::Active);
    assert_eq!(
        member.avatar_url.as_ref().unwrap(),
        "https://secure.gravatar.com/avatar/0617392a2f9fd505720d0c42cefc1a10?s=80&d=identicon",
    );
    assert_eq!(member.web_url, "https://gitlab.kitware.com/brad.king");
    assert_eq!(member.access_level, 50);
    assert_eq!(member.expires_at, None);
}

fn check_merge_request_a(merge_request: &MergeRequest) {
    assert_eq!(merge_request.id, MergeRequestId::new(20215));
    assert_eq!(merge_request.iid, MergeRequestInternalId::new(35));
    assert_eq!(merge_request.project_id, ProjectId::new(855));
    assert_eq!(merge_request.title, "gitlab: expose hook addition API");
    assert_eq!(merge_request.description.as_ref().unwrap(), "Fixes #5.");
    assert_eq!(merge_request.state, MergeRequestState::Merged);
    assert_eq!(
        merge_request.created_at,
        datetime((2016, 10, 4), (19, 56, 43, 276)),
    );
    assert_eq!(
        merge_request.updated_at,
        datetime((2021, 1, 7), (19, 18, 28, 558)),
    );
    assert_eq!(
        merge_request.merged_at.unwrap(),
        datetime((2016, 10, 4), (20, 18, 57, 914)),
    );
    assert!(merge_request.merged_by.is_none());
    assert!(merge_request.closed_by.is_none());
    assert_eq!(merge_request.target_branch, "master");
    assert_eq!(merge_request.source_branch, "add_hook-api");
    assert_eq!(merge_request.upvotes, 0);
    assert_eq!(merge_request.downvotes, 0);
    check_user_ben_boeckel(&merge_request.author);
    let assignee = merge_request.assignee.as_ref().unwrap();
    check_user_brad_king(assignee);
    let assignees = merge_request.assignees.as_ref().unwrap();
    assert_eq!(assignees.len(), 1);
    let assignee = &assignees[0];
    check_user_brad_king(assignee);
    let reviewers = merge_request.reviewers.as_ref().unwrap();
    assert_eq!(reviewers.len(), 1);
    let reviewer = &reviewers[0];
    check_user_brad_king(reviewer);
    assert_eq!(merge_request.source_project_id, Some(ProjectId::new(856)));
    assert_eq!(merge_request.target_project_id, ProjectId::new(855));
    assert!(merge_request.labels.is_empty());
}

fn check_merge_request_b(merge_request: &MergeRequest) {
    let check_sha = |oid: Option<&ObjectId>, value| {
        assert_eq!(oid.unwrap().value(), value);
    };

    assert!(!merge_request.work_in_progress);
    assert_eq!(merge_request.allow_collaboration, None);
    assert_eq!(merge_request.allow_maintainer_to_push, None);
    assert!(merge_request.milestone.is_none());
    assert!(!merge_request.squash);
    assert!(!merge_request.merge_when_pipeline_succeeds);
    assert_eq!(merge_request.merge_status, MergeStatus::CanBeMerged);
    check_sha(
        merge_request.sha.as_ref(),
        "04e94ae667024a62a90179f395bfdc2b35f3efd2",
    );
    let diff_refs = merge_request.diff_refs.as_ref().unwrap();
    check_sha(
        diff_refs.base_sha.as_ref(),
        "981262b03fc0149c1677ca51ea47b570e30d6a90",
    );
    check_sha(
        diff_refs.head_sha.as_ref(),
        "04e94ae667024a62a90179f395bfdc2b35f3efd2",
    );
    check_sha(
        diff_refs.start_sha.as_ref(),
        "981262b03fc0149c1677ca51ea47b570e30d6a90",
    );
    assert_eq!(merge_request.merge_error, None);
    assert_eq!(merge_request.rebase_in_progress, None);
    assert_eq!(merge_request.merge_commit_sha, None);
    assert_eq!(merge_request.subscribed, Some(true));
    check_empty_time_stats(&merge_request.time_stats);
    assert!(merge_request.blocking_discussions_resolved);
    assert_eq!(merge_request.changes_count.as_ref().unwrap(), "3");
    assert_eq!(merge_request.user_notes_count, 10);
    assert_eq!(merge_request.discussion_locked, None);
    assert_eq!(merge_request.should_remove_source_branch, None);
    assert_eq!(merge_request.force_remove_source_branch, Some(true));
    assert!(!merge_request.has_conflicts);
    assert!(merge_request.user.as_ref().unwrap().can_merge);
    assert_eq!(
        merge_request.web_url,
        "https://gitlab.kitware.com/utils/rust-gitlab/-/merge_requests/35",
    );
}

fn check_merge_request_basic(merge_request: &MergeRequestBasic) {
    assert_eq!(merge_request.id, MergeRequestId::new(200));
    assert_eq!(merge_request.iid, MergeRequestInternalId::new(5));
    assert_eq!(merge_request.project_id, ProjectId::new(15402121));
    assert_eq!(merge_request.title, "MR title");
    assert_eq!(
        merge_request.description,
        Some("The description of the MR".to_string()),
    );
    assert_eq!(merge_request.state, MergeRequestState::Merged);
    assert_eq!(
        merge_request.created_at,
        datetime((2021, 12, 14), (3, 23, 55, 176))
    );
    assert_eq!(
        merge_request.updated_at,
        datetime((2021, 12, 15), (19, 26, 45, 484))
    );
    assert_eq!(
        merge_request.web_url,
        "https://gitlab.com/organization/group/project/-/merge_requests/5",
    );
}

#[test]
fn test_read_merge_request() {
    let merge_request: MergeRequest = read_test_file("merge_request");

    // Split for clippy's complexity checks.
    check_merge_request_a(&merge_request);
    check_merge_request_b(&merge_request);
}

#[test]
fn test_read_merge_request_basic() {
    let merge_request: MergeRequestBasic = read_test_file("merge_request_basic");

    check_merge_request_basic(&merge_request);
}

#[test]
fn test_read_merge_train() {
    let merge_train: MergeTrain = read_test_file("merge_train");

    check_merge_request_basic(&merge_train.merge_request);
    check_user_kwrobot(&merge_train.user);

    assert_eq!(merge_train.target_branch, "main");
    assert_eq!(merge_train.status, MergeTrainState::Merged);
    assert_eq!(
        merge_train.merged_at,
        Some(datetime((2021, 12, 15), (19, 26, 46, 991)))
    );
    assert_eq!(
        merge_train.created_at,
        datetime((2021, 12, 14), (3, 23, 55, 176))
    );
    assert_eq!(
        merge_train.updated_at,
        datetime((2021, 12, 15), (19, 26, 45, 484))
    );
}

#[test]
fn test_read_note() {
    let note: Note = read_test_file("note");

    assert_eq!(note.id, NoteId::new(880_929));
    assert_eq!(note.body, "requested review from @brad.king");
    assert_eq!(note.attachment, None);
    check_user_brad_king(&note.author);
    assert_eq!(note.created_at, datetime((2021, 1, 7), (19, 18, 28, 633)));
    assert_eq!(note.updated_at, datetime((2021, 1, 7), (19, 18, 28, 637)));
    assert!(!note.resolvable);
    assert_eq!(note.resolved, None);
    assert!(note.resolved_by.is_none());
    assert!(note.system);
    assert_eq!(
        note.noteable_id().unwrap(),
        NoteableId::MergeRequest(MergeRequestId::new(20215)),
    );
    assert_eq!(
        note.noteable_iid().unwrap(),
        NoteableInternalId::MergeRequest(MergeRequestInternalId::new(35)),
    );
    assert_eq!(note.noteable_type, NoteType::MergeRequest);
}

#[test]
fn test_read_singlenote_discussion() {
    let discussions: Vec<Discussion> = read_test_file("discussion");
    let discussion = discussions
        .iter()
        .find(|x| x.id.value() == "18ea341cb10e952889e277836ba638c6b17ff26c")
        .unwrap();
    assert!(discussion.individual_note);
    assert_eq!(discussion.notes.len(), 1);
    let note = discussion.notes.get(0).unwrap();
    assert!(!note.resolvable);
    assert!(note.position.is_none());
    assert_eq!(note.note_type, None);
}

#[test]
fn test_read_nocode_discussion() {
    let discussions: Vec<Discussion> = read_test_file("discussion");
    let discussion = discussions
        .iter()
        .find(|x| x.id.value() == "a4d5505b3556eaa45edbe567af7aebc1760dedd7")
        .unwrap();
    assert!(!discussion.individual_note);
    assert_eq!(discussion.notes.len(), 3);
    let question = discussion.notes.get(0).unwrap();
    let comment = discussion.notes.get(1).unwrap();
    assert!(question.resolvable);
    assert!(comment.resolvable);

    assert_eq!(question.resolved, Some(true));

    assert!(question.position.is_none());
    assert!(comment.position.is_none());

    assert_eq!(question.id, NoteId::new(607_911));
    assert_eq!(comment.id, NoteId::new(607_912));

    assert_eq!(question.note_type, Some(DiscussionNoteType::DiscussionNote));
    assert_eq!(comment.note_type, Some(DiscussionNoteType::DiscussionNote));
}

#[test]
fn test_read_code_discussion() {
    let discussions: Vec<Discussion> = read_test_file("discussion");
    let discussion = discussions
        .into_iter()
        .find(|x| x.id.value() == "9f4998b2308728b95cff52af97019479e1269183")
        .unwrap();
    assert!(!discussion.individual_note);
    let note = discussion.notes.get(0).unwrap();
    assert!(note.resolvable);
    assert_eq!(note.resolved, Some(true));
    check_user_brad_king(&note.author);
    assert_eq!(note.id, NoteId::new(619_272));
    assert_eq!(note.note_type, Some(DiscussionNoteType::DiffNote));
    let position = note.position.as_ref().unwrap();
    assert_eq!(position.position_type, NotePositionType::Text);
    assert_eq!(
        position.head_sha.value(),
        "04e94ae667024a62a90179f395bfdc2b35f3efd2",
    );
    assert_eq!(position.new_line, Some(156));
    assert_eq!(position.new_path, "src/gitlab.rs");
}

fn check_project_a(project: &Project) {
    assert_eq!(project.id, ProjectId::new(855));
    assert_eq!(
        project.description.as_ref().unwrap(),
        "Rust library for communicating with a Gitlab instance.",
    );
    assert_eq!(project.default_branch.as_ref().unwrap(), "master");
    assert!(project.tag_list.is_empty());
    assert!(!project.archived);
    assert!(!project.empty_repo);
    assert_eq!(project.visibility, VisibilityLevel::Public);
    assert_eq!(
        project.ssh_url_to_repo,
        "git@gitlab.kitware.com:utils/rust-gitlab.git",
    );
    assert_eq!(
        project.http_url_to_repo,
        "https://gitlab.kitware.com/utils/rust-gitlab.git",
    );
    assert_eq!(
        project.web_url,
        "https://gitlab.kitware.com/utils/rust-gitlab",
    );
    assert_eq!(
        project.readme_url.as_ref().unwrap(),
        "https://gitlab.kitware.com/utils/rust-gitlab/-/blob/master/README.md",
    );
    assert!(project.owner.is_none());
    assert_eq!(project.name, "rust-gitlab");
    assert_eq!(project.name_with_namespace, "Utils / rust-gitlab");
    assert_eq!(project.path, "rust-gitlab");
    assert_eq!(project.path_with_namespace, "utils/rust-gitlab");
    assert_eq!(project.container_registry_enabled, Some(false));
    assert_eq!(
        project.created_at,
        datetime((2016, 6, 29), (17, 35, 12, 495)),
    );
    assert_eq!(
        project.last_activity_at,
        datetime((2022, 8, 22), (12, 56, 48, 154)),
    );
}

fn check_project_b(project: &Project) {
    assert_eq!(project.import_error, None);
    assert!(project.shared_runners_enabled);
    assert!(!project.lfs_enabled);
    assert_eq!(project.creator_id, UserId::new(13));
    check_namespace_utils(&project.namespace);
    assert!(project.forked_from_project.is_none());
    assert_eq!(project.avatar_url, None);
    assert_eq!(project.ci_config_path, None);
    assert_eq!(project.star_count, 16);
    assert_eq!(project.forks_count, 59);
    assert_eq!(project.open_issues_count, Some(25));
    assert!(project.public_jobs);
    assert!(project.shared_with_groups.is_empty());
    assert_eq!(project.only_allow_merge_if_pipeline_succeeds, Some(false));
    assert_eq!(
        project.only_allow_merge_if_all_discussions_are_resolved,
        None,
    );
    assert_eq!(project.remove_source_branch_after_merge, None);
    assert_eq!(project.printing_merge_request_link_enabled, Some(true));
    assert!(!project.request_access_enabled);
    assert_eq!(project.resolve_outdated_diff_discussions, None);

    assert!(project.jobs_enabled);
    assert!(project.issues_enabled);
    assert!(project.merge_requests_enabled);
    assert!(!project.snippets_enabled);
    assert!(project.wiki_enabled);
}

fn check_project_c(project: &Project) {
    assert_eq!(project.builds_access_level, FeatureVisibilityLevel::Enabled);
    assert_eq!(project.issues_access_level, FeatureVisibilityLevel::Enabled);
    assert_eq!(
        project.merge_requests_access_level,
        FeatureVisibilityLevel::Enabled,
    );
    assert_eq!(
        project.repository_access_level,
        FeatureVisibilityLevel::Enabled,
    );
    assert_eq!(
        project.snippets_access_level,
        FeatureVisibilityLevel::Disabled,
    );
    assert_eq!(project.wiki_access_level, FeatureVisibilityLevel::Enabled);

    assert_eq!(project.merge_method.as_ref().unwrap(), "merge");
    let permissions = project.permissions.as_ref().unwrap();
    let group_access = permissions.group_access.as_ref().unwrap();
    assert_eq!(group_access.access_level, 50);
    assert_eq!(group_access.notification_level, Some(3));
    assert!(permissions.project_access.is_none());
    assert!(project.has_links());
}

#[test]
fn test_read_project() {
    let project: Project = read_test_file("project");

    // Split for clippy's complexity checks.
    check_project_a(&project);
    check_project_b(&project);
    check_project_c(&project);
}

#[test]
fn test_read_project_hook() {
    let project_hook: ProjectHook = read_test_file("project_hook");

    assert_eq!(project_hook.id, HookId::new(1262));
    assert_eq!(project_hook.url, "http://kwrobot02:8082/gitlab.kitware.com");
    assert_eq!(
        project_hook.created_at,
        datetime((2016, 12, 16), (16, 37, 24, 589)),
    );
    assert!(project_hook.push_events);
    assert_eq!(project_hook.push_events_branch_filter, None);
    assert!(project_hook.tag_push_events);
    assert!(project_hook.issues_events);
    assert_eq!(project_hook.confidential_issues_events, Some(true));
    assert!(project_hook.merge_requests_events);
    assert!(project_hook.note_events);
    assert_eq!(project_hook.confidential_note_events, Some(true));
    assert!(!project_hook.repository_update_events);
    assert!(project_hook.enable_ssl_verification);
    assert!(project_hook.job_events);
    assert!(project_hook.pipeline_events);
    assert!(project_hook.wiki_page_events);
}

#[test]
fn test_read_repo_branch() {
    let repo_branch: RepoBranch = read_test_file("repo_branch");

    assert_eq!(repo_branch.name, "master");
    let commit = repo_branch.commit.as_ref().unwrap();
    assert_eq!(commit.author_email, "brad.king@kitware.com");
    assert_eq!(commit.author_name, "Brad King");
    assert_eq!(
        commit.authored_date,
        datetime((2018, 7, 12), (12, 50, 24, 0)),
    );
    assert_eq!(
        commit.committed_date,
        datetime((2018, 7, 12), (12, 50, 24, 0)),
    );
    assert_eq!(commit.created_at, datetime((2018, 7, 12), (12, 50, 24, 0)));
    assert_eq!(commit.committer_email, "brad.king@kitware.com");
    assert_eq!(commit.committer_name, "Brad King");
    assert_eq!(
        commit.id,
        ObjectId::new("e59db4b129b29df220ecec6119ed2130207a0397"),
    );
    assert_eq!(commit.short_id, ObjectId::new("e59db4b1"));
    assert_eq!(commit.title, "cargo: prep for 0.1100.1");
    assert_eq!(commit.message, "cargo: prep for 0.1100.1\n");
    itertools::assert_equal(
        commit.parent_ids.as_ref().unwrap(),
        &[ObjectId::new("5c81cc05661dcbb5fd923cca093920816c21ef7e")],
    );
    assert_eq!(repo_branch.merged, Some(false));
    assert_eq!(repo_branch.protected, Some(true));
    assert_eq!(repo_branch.developers_can_push, Some(false));
    assert_eq!(repo_branch.developers_can_merge, Some(false));
    assert_eq!(repo_branch.can_push, Some(true));
    assert_eq!(repo_branch.default, Some(true));
}

#[test]
fn test_read_repo_commit_detail() {
    let repo_commit_detail: RepoCommitDetail = read_test_file("repo_commit_detail");

    assert_eq!(
        repo_commit_detail.id,
        ObjectId::new("de4ac3cf96cb8a0893be22b03f5171d934f9d392"),
    );
    assert_eq!(repo_commit_detail.short_id, ObjectId::new("de4ac3cf"));
    assert_eq!(repo_commit_detail.title, "Merge topic 'mr-awards'");
    assert_eq!(repo_commit_detail.author_name, "Brad King");
    assert_eq!(repo_commit_detail.author_email, "brad.king@kitware.com");
    assert_eq!(repo_commit_detail.committer_name, "Kitware Robot");
    assert_eq!(repo_commit_detail.committer_email, "kwrobot@kitware.com");
    assert_eq!(
        repo_commit_detail.created_at,
        datetime((2016, 11, 8), (14, 30, 13, 0)),
    );
    assert_eq!(
        repo_commit_detail.message,
        "Merge topic 'mr-awards'\n\na222c553 gitlab: add a method for MR award \
         queries\n\nAcked-by: Kitware Robot <kwrobot@kitware.com>\nReviewed-by: Brad King \
         <brad.king@kitware.com>\nMerge-request: !46\n",
    );
    itertools::assert_equal(
        &repo_commit_detail.parent_ids,
        &[
            ObjectId::new("559f5f4a2bfe1f48e9e95afa09c029deb655cf7d"),
            ObjectId::new("a222c5539569cda6999b8069f1e51a5202c30711"),
        ],
    );
    assert_eq!(
        repo_commit_detail.committed_date,
        datetime((2016, 11, 8), (14, 30, 13, 0)),
    );
    assert_eq!(
        repo_commit_detail.authored_date,
        datetime((2016, 11, 8), (14, 30, 13, 0)),
    );
    let stats = repo_commit_detail.stats.as_ref().unwrap();
    assert_eq!(stats.additions, 8);
    assert_eq!(stats.deletions, 0);
    assert_eq!(stats.total, 8);
    let last_pipeline = repo_commit_detail.last_pipeline.as_ref().unwrap();
    assert_eq!(last_pipeline.id, PipelineId::new(34289));
    assert_eq!(last_pipeline.project_id, ProjectId::new(855));
    assert_eq!(last_pipeline.ref_.as_ref().unwrap(), "master");
    assert_eq!(
        last_pipeline.sha,
        ObjectId::new("de4ac3cf96cb8a0893be22b03f5171d934f9d392"),
    );
    assert_eq!(last_pipeline.status, StatusState::Success);
    assert_eq!(
        last_pipeline.created_at.unwrap(),
        datetime((2016, 11, 8), (14, 30, 16, 81)),
    );
    assert_eq!(
        last_pipeline.updated_at.unwrap(),
        datetime((2016, 11, 8), (14, 35, 32, 670)),
    );
    assert_eq!(
        last_pipeline.web_url,
        "https://gitlab.kitware.com/utils/rust-gitlab/-/pipelines/34289",
    );
    assert_eq!(repo_commit_detail.project_id, ProjectId::new(855));
}

#[test]
fn test_read_user() {
    let user: User = read_test_file("user");

    check_user_kwrobot(&user.clone().into());
    assert_eq!(
        user.created_at.unwrap(),
        datetime((2015, 2, 26), (15, 58, 34, 670)),
    );
    assert_eq!(user.is_admin, None);
    assert_eq!(user.highest_role, Some(AccessLevel::Owner));
    assert_eq!(user.bio, Some(String::new()));
    assert_eq!(user.private_profile, Some(false));
    assert_eq!(user.location, Some(String::new()));
    assert_eq!(user.public_email, Some(String::new()));
    assert_eq!(user.skype, "");
    assert_eq!(user.linkedin, "");
    assert_eq!(user.twitter, "");
    assert_eq!(user.website_url, "");
    assert_eq!(user.organization, Some(String::new()));
}

#[test]
fn test_read_user_public() {
    let user_public: UserPublic = read_test_file("user_public");

    check_user_kwrobot(&user_public.clone().into());
    assert_eq!(
        user_public.created_at.unwrap(),
        datetime((2015, 2, 26), (15, 58, 34, 670)),
    );
    assert_eq!(user_public.is_admin, Some(true));
    assert_eq!(user_public.bio, Some(String::new()));
    assert_eq!(user_public.private_profile, Some(false));
    assert_eq!(user_public.location, Some(String::new()));
    assert_eq!(user_public.public_email, Some(String::new()));
    assert_eq!(user_public.skype, "");
    assert_eq!(user_public.linkedin, "");
    assert_eq!(user_public.twitter, "");
    assert_eq!(user_public.website_url, "");
    assert_eq!(user_public.organization, Some(String::new()));
    assert_eq!(
        user_public.last_sign_in_at.unwrap(),
        datetime((2021, 12, 21), (13, 22, 1, 657)),
    );
    assert_eq!(
        user_public.last_activity_on.unwrap(),
        NaiveDate::from_ymd_opt(2022, 1, 6).unwrap(),
    );
    assert_eq!(
        user_public.confirmed_at.unwrap(),
        datetime((2015, 2, 26), (15, 58, 34, 660)),
    );
    assert_eq!(user_public.email, "kwrobot@kitware.com");
    assert_eq!(user_public.theme_id, None);
    assert_eq!(user_public.color_scheme_id, ColorSchemeId::new(4));
    assert_eq!(user_public.projects_limit, 50);
    assert_eq!(
        user_public.current_sign_in_at.unwrap(),
        datetime((2022, 1, 5), (15, 45, 8, 402)),
    );
    assert!(user_public.identities.is_empty());
    assert!(user_public.can_create_group);
    assert!(user_public.can_create_project);
    assert!(user_public.two_factor_enabled);
    assert!(!user_public.external);
}

#[test]
fn test_read_resource_label_events() {
    let event: ResourceLabelEvent = read_test_file("resource_label_event");

    assert_eq!(event.id, LabelEventId::new(10945));
    check_user_brad_king(&event.user);

    match &event.event_target() {
        Some(ResourceLabelEventTarget::Issue(id)) if id.value() == 69328 => {
            // this is the expected value
        },
        x => panic!("Unexpected resource_target: {:?}", x),
    }

    let label = event.label.unwrap();
    assert_eq!(label.id, LabelId::new(1720));
    assert_eq!(label.name, "area:doc");
    assert_eq!(label.color, LabelColor::from_rgb(0x58, 0x43, 0xAD));
    assert_eq!(label.description.as_ref().unwrap(), "Documentation issues");
}

#[test]
fn test_read_pipelines() {
    let pipeline_basic: PipelineBasic = read_test_file("pipeline_basic");

    assert_eq!(pipeline_basic.id, PipelineId::new(262_233));
    assert_eq!(pipeline_basic.project_id, ProjectId::new(855));
    assert_eq!(pipeline_basic.status, StatusState::Success);
    assert_eq!(pipeline_basic.ref_.as_ref().unwrap(), "master");
    assert_eq!(
        pipeline_basic.sha,
        ObjectId::new("f08c301293bf8267cd01f0892a89db8dba4f8cf6"),
    );
    assert_eq!(
        pipeline_basic.created_at.unwrap(),
        datetime((2022, 1, 6), (4, 3, 38, 142)),
    );
    assert_eq!(
        pipeline_basic.updated_at.unwrap(),
        datetime((2022, 1, 6), (4, 29, 7, 763)),
    );
    assert_eq!(
        pipeline_basic.web_url,
        "https://gitlab.kitware.com/utils/rust-gitlab/-/pipelines/262233",
    );
}

#[test]
fn test_read_pipeline() {
    let pipeline: Pipeline = read_test_file("pipeline");

    assert_eq!(pipeline.id, PipelineId::new(145_400));
    assert_eq!(pipeline.project_id, ProjectId::new(855));
    assert_eq!(pipeline.status, StatusState::Success);
    assert_eq!(pipeline.ref_.as_ref().unwrap(), "master");
    assert_eq!(
        pipeline.sha,
        ObjectId::new("7134adce4522c399cdab16e128b0a1af15b93f14"),
    );
    assert_eq!(
        pipeline.before_sha,
        Some(ObjectId::new("0000000000000000000000000000000000000000"))
    );
    assert!(!pipeline.tag);
    assert_eq!(pipeline.yaml_errors, None);
    assert_eq!(
        pipeline.created_at.unwrap(),
        datetime((2019, 9, 3), (18, 9, 47, 178)),
    );
    assert_eq!(
        pipeline.updated_at.unwrap(),
        datetime((2019, 9, 3), (18, 15, 47, 18)),
    );
    assert_eq!(
        pipeline.started_at.unwrap(),
        datetime((2019, 9, 3), (18, 9, 51, 465)),
    );
    assert_eq!(
        pipeline.finished_at.unwrap(),
        datetime((2019, 9, 3), (18, 15, 47, 13)),
    );
    assert_eq!(pipeline.committed_at, None);
    assert_eq!(pipeline.duration, Some(0));
    assert_eq!(pipeline.coverage, None);
    assert_eq!(
        pipeline.web_url,
        "https://gitlab.kitware.com/utils/rust-gitlab/-/pipelines/145400",
    );

    // nested user
    check_user_buildbot(&pipeline.user);

    // nested detailed status
    assert_eq!(
        pipeline.detailed_status,
        json!({
            "details_path": "/utils/rust-gitlab/-/pipelines/145400",
            "favicon": "/assets/ci_favicons/favicon_status_success-8451333011eee8ce9f2ab25dc487fe24a8758c694827a582f17f42b0a90446a2.png",
            "group": "success",
            "has_details": true,
            "icon": "status_success",
            "illustration": null,
            "label": "passed",
            "text": "passed",
            "tooltip": "passed",
        }),
    );
}

#[test]
fn test_read_pipeline_variables() {
    let var: PipelineVariable = read_test_file("pipeline_variable");

    assert_eq!(var.key, "RUN_NIGHTLY_BUILD");
    assert_eq!(var.variable_type, PipelineVariableType::EnvVar);
    assert_eq!(var.value, "true");
}

#[test]
fn test_read_group() {
    let group: Group = read_test_file("group");

    assert_eq!(group.id, GroupId::new(498));
    assert_eq!(group.name, "Utils");
    assert_eq!(group.path, "utils");
    assert_eq!(group.description.as_ref().unwrap(), "");
    assert_eq!(group.visibility, VisibilityLevel::Public);
    assert!(group.lfs_enabled);
    assert_eq!(group.avatar_url, None);
    assert_eq!(group.web_url, "https://gitlab.kitware.com/groups/utils");
    assert!(!group.request_access_enabled);
    assert_eq!(group.full_name, "Utils");
    assert_eq!(group.full_path, "utils");
    assert_eq!(group.parent_id, None);
    assert!(group.statistics.is_none());
}

fn check_commit_add_job_commands(commit: &RepoCommit) {
    assert_eq!(
        commit.id,
        ObjectId::new("0028f47612b928d94e5e1a4329f3e74d6fdd7032")
    );
    assert_eq!(commit.short_id, ObjectId::new("0028f476"));
    assert_eq!(commit.title, "Merge topic \'add-job-commands\'");
    itertools::assert_equal(
        commit.parent_ids.as_ref().unwrap(),
        &[
            ObjectId::new("ddb2c675b0b28bdb792b94d6ab0dc0c98a912374"),
            ObjectId::new("31fb1336aeaaaa0b22edda1a0938cb933ee575e4"),
        ],
    );
    assert_eq!(commit.author_name, "Ben Boeckel");
    assert_eq!(commit.author_email, "ben.boeckel@kitware.com");
    assert_eq!(
        commit.authored_date,
        datetime((2020, 4, 8), (17, 28, 40, 0))
    );
    assert_eq!(commit.committer_name, "Kitware Robot");
    assert_eq!(commit.committer_email, "kwrobot@kitware.com");
    assert_eq!(
        commit.committed_date,
        datetime((2020, 4, 8), (17, 28, 52, 0))
    );
    assert_eq!(commit.created_at, datetime((2020, 4, 8), (17, 28, 52, 0)));
    assert_eq!(
        commit.message,
        "Merge topic \'add-job-commands\'\n\
         \n\
         31fb133 add jobs apis\n\
         \n\
         Acked-by: Kitware Robot <kwrobot@kitware.com>\n\
         Acked-by: Ben Boeckel <ben.boeckel@kitware.com>\n\
         Merge-request: !213\n"
    );
}

#[derive(Clone, Debug, PartialEq)]
struct JobArtifactRef<'a> {
    file_type: &'a str,
    file_format: Option<&'a str>,
    filename: &'a str,
    size: u64,
}

impl<'a> JobArtifactRef<'a> {
    fn from(artifact: &'a JobArtifact) -> Self {
        let JobArtifact {
            file_type,
            file_format,
            filename,
            size,
        } = artifact;
        JobArtifactRef {
            file_type,
            file_format: file_format.as_ref().map(Deref::deref),
            filename,
            size: *size,
        }
    }
}

fn check_job_artifacts(artifacts: &[JobArtifact], expected: &[JobArtifactRef<'_>]) {
    itertools::assert_equal(
        artifacts.iter().map(JobArtifactRef::from),
        expected.iter().cloned(),
    );
}

#[test]
fn test_read_pending_job() {
    let jobs: Vec<Job> = read_test_file("job");
    let job: Job = jobs
        .into_iter()
        .find(|job| job.id == JobId::new(4_895_233))
        .unwrap();

    assert_eq!(job.status, StatusState::Pending);
    assert_eq!(job.stage, "test");
    assert_eq!(job.name, "test:cargo-nightly-no-default-features");
    assert_eq!(job.ref_.unwrap(), "master");
    assert!(!job.tag);
    assert_eq!(job.coverage, None);
    assert_eq!(job.created_at, datetime((2020, 4, 13), (4, 19, 46, 327)));
    assert_eq!(job.started_at, None);
    assert_eq!(job.finished_at, None);
    check_user_buildbot(&job.user.unwrap().into());
    assert!(job.artifacts_file.is_none());
    check_commit_add_job_commands(&job.commit);
    assert!(job.runner.is_none());
    assert_eq!(job.pipeline.id, PipelineId::new(168_478));
    assert_eq!(job.pipeline.ref_.unwrap(), "master");
    assert_eq!(
        job.pipeline.sha,
        ObjectId::new("0028f47612b928d94e5e1a4329f3e74d6fdd7032")
    );
    assert_eq!(job.pipeline.status, StatusState::Running);
    assert_eq!(
        job.pipeline.created_at,
        Some(datetime((2020, 4, 13), (4, 19, 45, 398)))
    );
    assert_eq!(
        job.pipeline.updated_at,
        Some(datetime((2020, 4, 13), (4, 36, 12, 508)))
    );
    assert_eq!(
        job.pipeline.web_url,
        "https://gitlab.kitware.com/utils/rust-gitlab/-/pipelines/168478"
    );
    assert!(!job.allow_failure);
    assert_eq!(job.duration, None);
    check_job_artifacts(&job.artifacts, &[]);
    assert_eq!(job.artifacts_expire_at, None);
    assert_eq!(
        job.web_url,
        "https://gitlab.kitware.com/utils/rust-gitlab/-/jobs/4895233"
    );
}

#[allow(clippy::cognitive_complexity)]
#[test]
fn test_read_success_job() {
    let jobs: Vec<Job> = read_test_file("job");
    let job: Job = jobs
        .into_iter()
        .find(|job| job.id == JobId::new(4_895_231))
        .unwrap();

    assert_eq!(job.status, StatusState::Success);
    assert_eq!(job.stage, "test");
    assert_eq!(job.name, "test:cargo-tarpaulin");
    assert_eq!(job.ref_.unwrap(), "master");
    assert!(!job.tag);
    assert_eq!(job.coverage, Some(36.43));
    assert_eq!(job.created_at, datetime((2020, 4, 13), (4, 19, 46, 268)));
    assert_eq!(
        job.started_at,
        Some(datetime((2020, 4, 13), (4, 29, 56, 752)))
    );
    assert_eq!(
        job.finished_at,
        Some(datetime((2020, 4, 13), (4, 30, 57, 772)))
    );
    check_user_buildbot(&job.user.unwrap().into());
    let artifacts_file = job.artifacts_file.unwrap();
    assert_eq!(artifacts_file.filename, "artifacts.zip");
    assert_eq!(artifacts_file.size, 76517);
    check_commit_add_job_commands(&job.commit);
    let runner = job.runner.unwrap();
    assert_eq!(runner.id, RunnerId::new(156));
    assert_eq!(runner.description.unwrap(), "minmus.priv-x11");
    assert!(runner.active);
    assert!(runner.is_shared);
    assert_eq!(runner.name.unwrap(), "gitlab-runner");
    assert_eq!(job.pipeline.id, PipelineId::new(168_478));
    assert_eq!(job.pipeline.ref_.unwrap(), "master");
    assert_eq!(
        job.pipeline.sha,
        ObjectId::new("0028f47612b928d94e5e1a4329f3e74d6fdd7032")
    );
    assert_eq!(job.pipeline.status, StatusState::Running);
    assert_eq!(
        job.pipeline.created_at,
        Some(datetime((2020, 4, 13), (4, 19, 45, 398)))
    );
    assert_eq!(
        job.pipeline.updated_at,
        Some(datetime((2020, 4, 13), (4, 36, 12, 508)))
    );
    assert_eq!(
        job.pipeline.web_url,
        "https://gitlab.kitware.com/utils/rust-gitlab/-/pipelines/168478"
    );
    assert!(!job.allow_failure);
    assert_eq!(job.duration, Some(61.019_904));
    check_job_artifacts(
        &job.artifacts,
        &[
            JobArtifactRef {
                file_type: "archive",
                file_format: Some("zip"),
                filename: "artifacts.zip",
                size: 76517,
            },
            JobArtifactRef {
                file_type: "metadata",
                file_format: Some("gzip"),
                filename: "metadata.gz",
                size: 166,
            },
            JobArtifactRef {
                file_type: "trace",
                file_format: None,
                filename: "job.log",
                size: 12238,
            },
        ],
    );
    assert_eq!(
        job.artifacts_expire_at,
        Some(datetime((2020, 4, 20), (4, 30, 57, 46)))
    );
    assert_eq!(
        job.web_url,
        "https://gitlab.kitware.com/utils/rust-gitlab/-/jobs/4895231"
    );
}

#[allow(clippy::cognitive_complexity)]
#[test]
fn test_read_running_job() {
    let jobs: Vec<Job> = read_test_file("job");
    let job: Job = jobs
        .into_iter()
        .find(|job| job.id == JobId::new(4_895_232))
        .unwrap();

    assert_eq!(job.status, StatusState::Running);
    assert_eq!(job.stage, "test");
    assert_eq!(job.name, "test:cargo-nightly");
    assert_eq!(job.ref_.unwrap(), "master");
    assert!(!job.tag);
    assert_eq!(job.coverage, None);
    assert_eq!(job.created_at, datetime((2020, 4, 13), (4, 19, 46, 302)));
    assert_eq!(
        job.started_at,
        Some(datetime((2020, 4, 13), (4, 33, 2, 536)))
    );
    assert_eq!(job.finished_at, None);
    check_user_buildbot(&job.user.unwrap().into());
    assert!(job.artifacts_file.is_none());
    check_commit_add_job_commands(&job.commit);
    let runner = job.runner.unwrap();
    assert_eq!(runner.id, RunnerId::new(160));
    assert_eq!(runner.description.unwrap(), "abeth.cuda-rt.large");
    assert!(runner.active);
    assert!(runner.is_shared);
    assert_eq!(runner.name.unwrap(), "gitlab-runner");
    assert_eq!(job.pipeline.id, PipelineId::new(168_478));
    assert_eq!(job.pipeline.project_id, ProjectId::new(855));
    assert_eq!(job.pipeline.ref_.unwrap(), "master");
    assert_eq!(
        job.pipeline.sha,
        ObjectId::new("0028f47612b928d94e5e1a4329f3e74d6fdd7032")
    );
    assert_eq!(job.pipeline.status, StatusState::Running);
    assert_eq!(
        job.pipeline.created_at,
        Some(datetime((2020, 4, 13), (4, 19, 45, 398)))
    );
    assert_eq!(
        job.pipeline.updated_at,
        Some(datetime((2020, 4, 13), (4, 36, 12, 508)))
    );
    assert_eq!(
        job.pipeline.web_url,
        "https://gitlab.kitware.com/utils/rust-gitlab/-/pipelines/168478"
    );
    assert!(!job.allow_failure);
    assert_eq!(job.duration, Some(49.516_176));
    check_job_artifacts(&job.artifacts, &[]);
    assert_eq!(job.artifacts_expire_at, None);
    assert_eq!(
        job.web_url,
        "https://gitlab.kitware.com/utils/rust-gitlab/-/jobs/4895232"
    );
}
