# Endpoint status

This document categorizes the APIs as they pertain to this crate.

Last synced: 13.9.0-pre (29cc646679b927158b3315dbc6c5af39524fd6ec)

# Implemented

These API endpoints have been implemented.

  * `GET    /deploy_keys` `deploy_keys/deploy_keys.rs`
  * `GET    /groups` `groups/groups.rs`
  * `POST   /groups` `groups/create.rs`
  * `PUT    /groups/:group` `groups/edit.rs`
  * `GET    /groups/:group` `groups/group.rs`
  * `GET    /groups/:group/hooks` `groups/hooks/hooks.rs`
  * `POST   /groups/:group/hooks` `groups/hooks/create.rs`
  * `GET    /groups/:group/hooks/:hook` `groups/hooks/hook.rs`
  * `PUT    /groups/:group/hooks/:hook` `groups/hooks/edit.rs`
  * `DELETE /groups/:group/hooks/:hook` `groups/hooks/delete.rs`
  * `GET    /groups/:group/members` `groups/members/members.rs`
  * `POST   /groups/:group/members` `groups/members/add.rs`
  * `GET    /groups/:group/members/:member` `groups/members/member.rs`
  * `DELETE /groups/:group/members/:member` `groups/members/remove.rs`
  * `PUT    /groups/:group/members/:member` `groups/members/edit.rs`
  * `POST   /groups/:group/milestones` `groups/milestones/create.rs`
  * `GET    /groups/:group/projects` `groups/projects/projects.rs`
  * `GET    /groups/:group/projects/shared` `groups/projects/shared.rs`
  * `POST   /groups/:group/share` `groups/share.rs`
  * `DELETE /groups/:group/share/:group2` `groups/unshare.rs`
  * `GET    /groups/:group/subgroups` `groups/subgroups/subgroups.rs`
  * `GET    /projects` `projects/projects.rs`
  * `POST   /projects` `projects/create.rs`
  * `GET    /projects/:project` `projects/project.rs`
  * `PUT    /projects/:project` `projects/edit.rs`
  * `GET    /projects/:project/deploy_keys` `projects/deploy_keys/deploy_keys.rs`
  * `GET    /projects/:project/deploy_keys/:deploy_key` `projects/deploy_keys/deploy_key.rs`
  * `POST   /projects/:project/deploy_keys` `projects/deploy_keys/create.rs`
  * `PUT    /projects/:project/deploy_keys/:deploy_key` `projects/deploy_keys/edit.rs`
  * `DELETE /projects/:project/deploy_keys/:deploy_key` `projects/deploy_keys/delete.rs`
  * `POST   /projects/:project/deploy_keys/:deploy_key/enable` `projects/deploy_keys/enable.rs`
  * `GET    /projects/:project/environments` `projects/environments/environments.rs`
  * `GET    /projects/:project/environments/:environment` `projects/environments/environment.rs`
  * `GET    /projects/:project/hooks` `projects/hooks/hooks.rs`
  * `POST   /projects/:project/hooks` `projects/hooks/create.rs`
  * `GET    /projects/:project/hooks/:hook` `projects/hooks/hook.rs`
  * `PUT    /projects/:project/hooks/:hook` `projects/hooks/edit.rs`
  * `DELETE /projects/:project/hooks/:hook` `projects/hooks/delete.rs`
  * `GET    /projects/:project/issues` `projects/issues/issues.rs`
    - Missing support for `not`.
  * `POST   /projects/:project/issues` `projects/issues/create.rs`
  * `GET    /projects/:project/issues/:issue` `projects/issues/issue.rs`
  * `PUT    /projects/:project/issues/:issue` `projects/issues/edit.rs`
  * `GET    /projects/:project/issues/:issue/closed_by` `projects/issues/merge_requests_closing.rs`
  * `GET    /projects/:project/issues/:issue/notes` `projects/issues/notes/notes.rs`
  * `POST   /projects/:project/issues/:issue/notes` `projects/issues/notes/create.rs`
  * `PUT    /projects/:project/issues/:issue/notes/:note` `projects/issues/notes/edit.rs`
  * `GET    /projects/:project/issues/:issue/resource_label_events` `projects/issues/resource_label_events.rs`
  * `GET    /projects/:project/jobs` `projects/jobs/jobs.rs`
  * `GET    /projects/:project/jobs/:job` `projects/jobs/job.rs`
  * `POST   /projects/:project/jobs/:job/cancel` `projects/jobs/cancel.rs`
  * `POST   /projects/:project/jobs/:job/erase` `projects/jobs/erase.rs`
  * `POST   /projects/:project/jobs/:job/retry` `projects/jobs/retry.rs`
  * `POST   /projects/:project/jobs/:job/play` `projects/jobs/play.rs`
  * `GET    /projects/:project/jobs/:job/trace` `projects/jobs/trace.rs`
  * `GET    /projects/:project/labels` `projects/labels/labels.rs`
  * `POST   /projects/:project/labels` `projects/labels/create.rs`
  * `GET    /projects/:project/labels/:label` `projects/labels/label.rs`
  * `DELETE /projects/:project/labels/:label` `projects/labels/delete.rs`
  * `PUT    /projects/:project/labels/:label/promote` `projects/labels/promote.rs`
    Arguably, this should be `POST /projects/:project/labels/:label/promote`.
    https://gitlab.com/gitlab-org/gitlab/-/issues/219324#note_382305638
  * `GET    /projects/:project/members` `projects/members/members.rs`
  * `POST   /projects/:project/members` `projects/members/add.rs`
  * `GET    /projects/:project/members/:member` `projects/members/member.rs`
  * `PUT    /projects/:project/members/:member` `projects/members/edit.rs`
  * `DELETE /projects/:project/members/:member` `projects/members/remove.rs`
  * `GET    /projects/:project/members/all` `projects/members/members.rs`
  * `GET    /projects/:project/members/all/:member` `projects/members/member.rs`
  * `GET    /projects/:project/merge_requests` `projects/merge_requests/merge_requests.rs`
    - Missing support for `not`.
  * `POST   /projects/:project/merge_requests` `projects/merge_requests/create.rs`
  * `GET    /projects/:project/merge_requests/:merge_request` `projects/merge_requests/merge_request.rs`
  * `PUT    /projects/:project/merge_requests/:merge_request` `projects/merge_requests/edit.rs`
  * `GET    /projects/:project/merge_requests/:merge_request/approval_rules` `projects/merge_requests/approval_rules/approval_rules.rs`
  * `GET    /projects/:project/merge_requests/:merge_request/approval_state` `projects/merge_requests/approval_state/approval_state.rs`
  * `GET    /projects/:project/merge_requests/:merge_request/approvals` `projects/merge_requests/approvals/approvals.rs`
  * `POST   /projects/:project/merge_requests/:merge_request/approve` `projects/merge_requests/approve.rs`
  * `GET    /projects/:project/merge_requests/:merge_request/award_emoji` `projects/merge_requests/awards/awards.rs`
  * `GET    /projects/:project/merge_requests/:merge_request/changes` `projects/merge_requests/changes.rs`
  * `GET    /projects/:project/merge_requests/:merge_request/closes_issues` `projects/merge_requests/issues_closed_by.rs`
  * `GET    /projects/:project/merge_requests/:merge_request/commits` `projects/merge_requests/commits.rs`
  * `GET    /projects/:project/merge_requests/:merge_request/discussions` `projects/merge_requests/discussions/discussions.rs`
  * `POST   /projects/:project/merge_requests/:merge_request/discussions` `projects/merge_requests/discussions/create.rs`
  * `PUT    /projects/:project/merge_requests/:merge_request/merge` `projects/merge_requests/merge.rs`
    This should be a `POST` action.
    https://gitlab.com/gitlab-org/gitlab/-/issues/219324
  * `GET    /projects/:project/merge_requests/:merge_request/notes` `projects/merge_requests/notes/notes.rs`
  * `POST   /projects/:project/merge_requests/:merge_request/notes` `projects/merge_requests/notes/create.rs`
  * `PUT    /projects/:project/merge_requests/:merge_request/notes/:note` `projects/merge_requests/notes/edit.rs`
  * `GET    /projects/:project/merge_requests/:merge_request/notes/:note/award_emoji` `projects/merge_requests/notes/awards/awards.rs`
  * `POST   /projects/:project/merge_requests/:merge_request/notes/:note/award_emoji` `projects/merge_requests/notes/awards/create.rs`
  * `GET    /projects/:project/merge_requests/:merge_request/pipelines` `projects/merge_requests/pipelines/pipelines.rs`
  * `POST   /projects/:project/merge_requests/:merge_request/pipelines` `projects/merge_requests/pipelines/create.rs`
  * `PUT    /projects/:project/merge_requests/:merge_request/rebase` `projects/merge_requests/rebase.rs`
    This should be a `POST` action.
    https://gitlab.com/gitlab-org/gitlab/-/issues/219324
  * `GET    /projects/:project/merge_requests/:merge_request/resource_label_events` `projects/merge_requests/resource_label_events.rs`
  * `POST   /projects/:project/merge_requests/:merge_request/unapprove` `projects/merge_requests/unapprove.rs`
  * `GET    /projects/:project/merge_trains` `projects/merge_trains.rs`
  * `POST   /projects/:project/milestones` `projects/milestones/create.rs`
  * `GET    /projects/:project/pipelines` `projects/pipelines/pipelines.rs`
  * `POST   /projects/:project/pipelines` `projects/pipelines/create.rs`
  * `GET    /projects/:project/pipelines/:pipeline` `projects/pipelines/pipeline.rs`
  * `DELETE /projects/:project/pipelines/:pipeline` `projects/pipelines/delete.rs`
  * `POST   /projects/:project/pipelines/:pipeline/cancel` `projects/pipelines/cancel.rs`
  * `GET    /projects/:project/pipelines/:pipeline/jobs` `projects/pipelines/jobs.rs`
  * `POST   /projects/:project/pipelines/:pipeline/retry` `projects/pipelines/retry.rs`
  * `GET    /projects/:project/pipelines/:pipeline/variables` `projects/pipelines/variables.rs`
  * `GET    /projects/:project/protected_branches` `projects/protected_branches/protected_branches.rs`
  * `POST   /projects/:project/protected_branches` `projects/protected_branches/protect.rs`
  * `DELETE /projects/:project/protected_branches/*branch` `projects/protected_branches/unprotect.rs`
  * `GET    /projects/:project/protected_branches/:branch` `projects/protected_branches/protected_branch.rs`
  * `GET    /projects/:project/protected_tags` `projects/protected_tags/protected_tags.rs`
  * `POST   /projects/:project/protected_tags` `projects/protected_tags/protect.rs`
  * `GET    /projects/:project/protected_tags/:name` `projects/protected_tags/protected_tag.rs`
  * `DELETE /projects/:project/protected_tags/:name` `projects/protected_tags/unprotect.rs`
  * `GET    /projects/:project/registry/repositories` `projects/registry/repositories.rs`
  * `DELETE /projects/:project/registry/repositories/:repository` `projects/registry/delete_repository.rs`
  * `GET    /projects/:project/registry/repositories/:repository/tags` `projects/registry/repository_tags.rs`
  * `GET    /projects/:project/registry/repositories/:repository/tags/:tag_name` `projects/registry/repository_tag_details.rs`
  * `DELETE /projects/:project/registry/repositories/:repository/tags/:tag_name` `projects/registry/delete_repository_tag.rs`
  * `GET    /projects/:project/releases` `projects/releases/releases.rs`
  * `GET    /projects/:project/repository/archive[.format]` `projects/repository/archive.rs`
  * `GET    /projects/:project/repository/branches` `projects/repository/branches/branches.rs`
  * `POST   /projects/:project/repository/branches` `projects/repository/branches/create.rs`
  * `GET    /projects/:project/repository/branches/:branch` `projects/repository/branches/branch.rs`
  * `GET    /projects/:project/repository/commits` `projects/repository/commits/commits.rs`
  * `GET    /projects/:project/repository/commits/:sha` `projects/repository/commits/commit.rs`
  * `GET    /projects/:project/repository/commits/:sha/comments` `projects/repository/commits/comments.rs`
  * `POST   /projects/:project/repository/commits/:sha/comments` `projects/repository/commits/comment.rs`
  * `GET    /projects/:project/repository/commits/:sha/merge_requests` `projects/repository/commits/merge_requests.rs`
  * `GET    /projects/:project/repository/commits/:sha/statuses` `projects/repository/commits/statuses.rs`
  * `POST   /projects/:project/repository/files/*file_path` `projects/repository/files/create.rs`
  * `PUT    /projects/:project/repository/files/*file_path` `projects/repository/files/update.rs`
  * `DELETE /projects/:project/repository/files/*file_path` `projects/repository/files/delete.rs`
  * `GET    /projects/:project/repository/files/*file_path/raw` `projects/repository/files/file_raw.rs`
  * `GET    /projects/:project/repository/tags` `projects/repository/tags/tags.rs`
  * `POST   /projects/:project/repository/tags` `projects/repository/tags/create.rs`
  * `GET    /projects/:project/repository/tags/:tag` `projects/repository/tags/tag.rs`
  * `GET    /projects/:project/repository/tree` `projects/repository/tree.rs`
  * `POST   /projects/:project/share` `projects/share.rs`
  * `DELETE /projects/:project/share/:group` `projects/unshare.rs`
  * `POST   /projects/:project/statuses/:sha` `projects/repository/commits/create_status.rs`
    Arguably, this should be `POST /projects/:project/repository/commits/:sha/statuses`.
    https://gitlab.com/gitlab-org/gitlab/-/issues/217412
  * `POST   /projects/:project/variables` `projects/variables/create.rs`
  * `GET    /projects/:project/variables/:key` `projects/variables/variable.rs`
  * `PUT    /projects/:project/variables/:key` `projects/variables/update.rs`
  * `GET    /user` `users/current_user.rs`
  * `GET    /users` `users/users.rs`
  * `GET    /users/:user` `users/user.rs`
  * `GET    /users/:user/impersonation_tokens` `users/impersonation_tokens/impersonation_tokens.rs`
  * `POST   /users/:user/impersonation_tokens` `users/impersonation_tokens/create.rs`
  * `GET    /users/:user/impersonation_tokens/:impersonation_token` `users/impersonation_tokens/impersonation_token.rs`
  * `DELETE /users/:user/impersonation_tokens/:impersonation_token` `users/impersonation_tokens/delete.rs`

# Todo

This section contains the list of API endpoints which are not yet implemented
in this crate. Contributions welcome!

## Specific endpoints

These endpoints are documented on a page that have other endpoints already
implemented above. This is split out into a separate list for convenience
instead of having to search the page for missing endpoints.

  * `GET    /issues` https://gitlab.kitware.com/help/api/issues.md#list-issues
  * `GET    /issues/:issue` https://gitlab.kitware.com/help/api/issues.md#single-issue
  * `DELETE /groups/:group` https://gitlab.kitware.com/help/api/groups.md#remove-group
  * `GET    /groups/:group/avatar` https://gitlab.kitware.com/help/api/groups.md#download-a-group-avatar
  * `GET    /groups/:group/billable_members` https://gitlab.kitware.com/help/api/groups.md#list-all-billable-members-of-a-group
  * `GET    /groups/:group/billable_members:user/memberships` https://gitlab.kitware.com/help/api/members.md#list-memberships-for-a-billable-member-of-a-group
  * `DELETE /groups/:group/billable_members:user` https://gitlab.kitware.com/help/api/members.md#remove-a-billable-member-from-a-group
  * `PUT    /groups/:group/billable_members:user/state` https://gitlab.kitware.com/help/api/members.md#change-membership-state-of-a-user-in-a-group
  * `GET    /groups/:group/descendant_groups` https://gitlab.kitware.com/help/api/groups.md#list-a-groups-descendant-groups
  * `GET    /groups/:group/epics/:epic/discussions` https://gitlab.kitware.com/help/api/discussions.md#list-group-epic-discussion-items
  * `POST   /groups/:group/epics/:epic/discussions` https://gitlab.kitware.com/help/api/discussions.md#create-new-epic-thread
  * `GET    /groups/:group/epics/:epic/discussions/:discussion` https://gitlab.kitware.com/help/api/discussions.md#get-single-epic-discussion-item
  * `POST   /groups/:group/epics/:epic/discussions/:discussion/notes` https://gitlab.kitware.com/help/api/discussions.md#add-note-to-existing-epic-thread
  * `PUT    /groups/:group/epics/:epic/discussions/:discussion/notes/:note` https://gitlab.kitware.com/help/api/discussions.md#modify-existing-epic-thread-note
  * `DELETE /groups/:group/epics/:epic/discussions/:discussion/notes/:note` https://gitlab.kitware.com/help/api/discussions.md#delete-an-epic-thread-note
  * `GET    /groups/:group/epics/:epic/notes` https://gitlab.kitware.com/help/api/notes.md#list-all-epic-notes
  * `POST   /groups/:group/epics/:epic/notes` https://gitlab.kitware.com/help/api/notes.md#create-new-epic-note
  * `GET    /groups/:group/epics/:epic/notes/:note` https://gitlab.kitware.com/help/api/notes.md#get-single-epic-note
  * `PUT    /groups/:group/epics/:epic/notes/:note` https://gitlab.kitware.com/help/api/notes.md#modify-existing-epic-note
  * `DELETE /groups/:group/epics/:epic/notes/:note` https://gitlab.kitware.com/help/api/notes.md#delete-an-epic-note
  * `GET    /groups/:group/epics/:epic/resource_label_events` https://gitlab.kitware.com/help/api/resource_label_events.md#list-group-epic-label-events
  * `GET    /groups/:group/epics/:epic/resource_label_events/:event` https://gitlab.kitware.com/help/api/resource_label_events.md#get-single-epic-label-event
  * `GET    /groups/:group/issues` https://gitlab.kitware.com/help/api/issues.md#list-group-issues
  * `GET    /groups/:group/ldap_group_links` https://gitlab.kitware.com/help/api/groups.md#list-ldap-group-links-starter
  * `POST   /groups/:group/ldap_group_links` https://gitlab.kitware.com/help/api/groups.md#add-ldap-group-link-with-cn-or-filter-starter
  * `DELETE /groups/:group/ldap_group_links` https://gitlab.kitware.com/help/api/groups.md#delete-ldap-group-link-with-cn-or-filter-starter
  * `DELETE /groups/:group/ldap_group_links/:cn` https://gitlab.kitware.com/help/api/groups.md#delete-ldap-group-link-starter (deprecated)
  * `DELETE /groups/:group/ldap_group_links/:provider/:cn` https://gitlab.kitware.com/help/api/groups.md#delete-ldap-group-link-starter (deprecated)
  * `POST   /groups/:group/ldap_sync` https://gitlab.kitware.com/help/api/groups.md#sync-group-with-ldap-starter
  * `GET    /groups/:group/members/all` https://gitlab.kitware.com/help/api/members.md#list-all-members-of-a-group-or-project-including-inherited-members
  * `GET    /groups/:group/members/all/:member` https://gitlab.kitware.com/help/api/members.md#get-a-member-of-a-group-or-project-including-inherited-members
  * `POST   /groups/:group/members/:member/approve` https://gitlab.kitware.com/help/api/members.md#approve-a-member-for-a-group
  * `POST   /groups/:group/members/:member/approve_all` https://gitlab.kitware.com/help/api/members.md#approve-all-pending-members-for-a-group
  * `POST   /groups/:group/members/:member/override` https://gitlab.kitware.com/help/api/members.md#set-override-flag-for-a-member-from-a-group
  * `DELETE /groups/:group/members/:member/override` https://gitlab.kitware.com/help/api/members.md#remove-override-for-a-member-from-a-group
  * `GET    /groups/:group/merge_requests` https://gitlab.kitware.com/help/api/merge_requests.md#list-group-merge-requests
  * `GET    /groups/:group/milestones` https://gitlab.kitware.com/help/api/group_milestones.md#list-group-milestones
  * `GET    /groups/:group/milestones/:milestone` https://gitlab.kitware.com/help/api/group_milestones.md#get-single-milestone
  * `PUT    /groups/:group/milestones/:milestone` https://gitlab.kitware.com/help/api/group_milestones.md#edit-milestone
  * `DELETE /groups/:group/milestones/:milestone` https://gitlab.kitware.com/help/api/group_milestones.md#delete-group-milestone
  * `GET    /groups/:group/milestones/:milestone/burndown_events` https://gitlab.kitware.com/help/api/group_milestones.md#get-all-burndown-chart-events-for-a-single-milestone-starter
  * `GET    /groups/:group/milestones/:milestone/issues` https://gitlab.kitware.com/help/api/group_milestones.md#get-all-issues-assigned-to-a-single-milestone
  * `GET    /groups/:group/milestones/:milestone/merge_requests` https://gitlab.kitware.com/help/api/group_milestones.md#get-all-merge-requests-assigned-to-a-single-milestone
  * `GET    /groups/:group/pending_members` https://gitlab.kitware.com/help/api/members.md#list-pending-members-of-a-group-and-its-subgroups-and-projects
  * `POST   /groups/:group/projects/:project` https://gitlab.kitware.com/help/api/groups.md#transfer-project-to-group
  * `GET    /groups/:group/provisioned_users` https://gitlab.kitware.com/help/api/groups.md#list-provisioned-users
  * `GET    /groups/:group/push_rule` https://gitlab.kitware.com/help/api/groups.md#get-group-push-rules
  * `POST   /groups/:group/push_rule` https://gitlab.kitware.com/help/api/groups.md#add-group-push-rule
  * `PUT    /groups/:group/push_rule` https://gitlab.kitware.com/help/api/groups.md#edit-group-push-rule
  * `DELETE /groups/:group/push_rule` https://gitlab.kitware.com/help/api/groups.md#delete-group-push-rule
  * `GET    /groups/:group/registry/repositories` https://gitlab.kitware.com/help/api/container_registry.md#within-a-group
  * `POST   /groups/:group/restore` https://gitlab.kitware.com/help/api/groups.md#restore-group-marked-for-deletion-premium
  * `POST   /groups/:group/transfer` https://gitlab.kitware.com/help/api/groups.md#transfer-a-group-to-a-new-parent-group-turn-a-subgroup-to-a-top-level-group
  * `GET    /job` https://gitlab.kitware.com/help/api/jobs.md#get-job-tokens-job
  * `GET    /job/allowed_agents` https://gitlab.kitware.com/help/api/jobs.md#get-gitlab-agent-by-ci_job_token
  * `GET    /merge_requests` https://gitlab.kitware.com/help/api/merge_requests.md#list-merge-requests
  * `DELETE /projects/:project` https://gitlab.kitware.com/help/api/projects.md#delete-project
  * `POST   /projects/:project/archive` https://gitlab.kitware.com/help/api/projects.md#archive-a-project
  * `GET    /projects/:project/approvals` https://gitlab.kitware.com/help/api/merge_request_approvals.md#get-configuration
  * `POST   /projects/:project/approvals` https://gitlab.kitware.com/help/api/merge_request_approvals.md#change-configuration
  * `GET    /projects/:project/approval_rules` https://gitlab.kitware.com/help/api/merge_request_approvals.md#get-project-level-rules
  * `POST   /projects/:project/approval_rules` https://gitlab.kitware.com/help/api/merge_request_approvals.md#create-project-level-rule
  * `GET    /projects/:project/approval_rules/:approval_rule` https://gitlab.kitware.com/help/api/merge_request_approvals.md#get-a-single-project-level-rule
  * `PUT    /projects/:project/approval_rules/:approval_rule` https://gitlab.kitware.com/help/api/merge_request_approvals.md#update-project-level-rule
  * `DELETE /projects/:project/approval_rules/:approval_rule` https://gitlab.kitware.com/help/api/merge_request_approvals.md#delete-project-level-rule
  * `GET    /projects/:project/commits/:sha/discussions` https://gitlab.kitware.com/help/api/discussions.md#list-project-commit-discussion-items
    Arguably, this (and its related endpoints) should be `GET
    /projects/:project/repository/commits/:sha/discussions`.
    https://gitlab.com/gitlab-org/gitlab/-/issues/219321
  * `POST   /projects/:project/commits/:sha/discussions` https://gitlab.kitware.com/help/api/discussions.md#create-new-commit-thread
  * `GET    /projects/:project/commits/:sha/discussions/:discussion` https://gitlab.kitware.com/help/api/discussions.md#get-single-commit-discussion-item
  * `POST   /projects/:project/commits/:sha/discussions/:discussion/notes` https://gitlab.kitware.com/help/api/discussions.md#add-note-to-existing-commit-thread
  * `PUT    /projects/:project/commits/:sha/discussions/:discussion/notes/:note` https://gitlab.kitware.com/help/api/discussions.md#modify-an-existing-commit-thread-note
  * `DELETE /projects/:project/commits/:sha/discussions/:discussion/notes/:note` https://gitlab.kitware.com/help/api/discussions.md#delete-a-commit-thread-note
  * `POST   /projects/:project/environments` https://gitlab.kitware.com/help/api/environments.md#create-a-new-environment
  * `DELETE /projects/:project/environments/review_apps` https://gitlab.kitware.com/help/api/environments.md#delete-multiple-stopped-review-apps
  * `PUT    /projects/:project/environments/:environment` https://gitlab.kitware.com/help/api/environments.md#edit-an-existing-environment
  * `DELETE /projects/:project/environments/:environment` https://gitlab.kitware.com/help/api/environments.md#delete-an-environment
  * `POST   /projects/:project/environments/:environment/stop` https://gitlab.kitware.com/help/api/environments.md#stop-an-environment
  * `GET    /projects/:project/epics/:epic/notes` https://gitlab.kitware.com/help/api/notes.md#list-all-epic-notes
  * `GET    /projects/:project/epics/:epic/notes/:note` https://gitlab.kitware.com/help/api/notes.md#get-single-epic-note
  * `POST   /projects/:project/epics/:epic/notes` https://gitlab.kitware.com/help/api/notes.md#create-new-epic-note
  * `PUT    /projects/:project/epics/:epic/notes/:note` https://gitlab.kitware.com/help/api/notes.md#modify-existing-epic-note
  * `DELETE /projects/:project/epics/:epic/notes/:note` https://gitlab.kitware.com/help/api/notes.md#delete-an-epic-note
  * `POST   /projects/:project/fork` https://gitlab.kitware.com/help/api/projects.md#fork-project
  * `DELETE /projects/:project/fork` https://gitlab.kitware.com/help/api/projects.md#delete-an-existing-forked-from-relationship
  * `POST   /projects/:project/fork/:from` https://gitlab.kitware.com/help/api/projects.md#create-a-forked-fromto-relation-between-existing-projects
  * `GET    /projects/:project/forks` https://gitlab.kitware.com/help/api/projects.md#list-forks-of-a-project
  * `GET    /projects/:project/groups` https://gitlab.kitware.com/help/api/projects.md#list-projects-groups
  * `POST   /projects/:project/housekeeping` https://gitlab.kitware.com/help/api/projects.md#start-the-housekeeping-task-for-a-project
  * `POST   /projects/:project/import_project_members/:project2` https://gitlab.kitware.com/help/api/projects.md#import-project-members
  * `DELETE /projects/:project/issues/:issue` https://gitlab.kitware.com/help/api/issues.md#delete-an-issue
  * `POST   /projects/:project/issues/:issue/add_spent_time` https://gitlab.kitware.com/help/api/issues.md#add-spent-time-for-an-issue
  * `GET    /projects/:project/issues/:issue/award_emoji` https://gitlab.kitware.com/help/api/award_emoji.md#list-an-awardables-award-emoji
  * `POST   /projects/:project/issues/:issue/award_emoji` https://gitlab.kitware.com/help/api/award_emoji.md#award-a-new-emoji
  * `GET    /projects/:project/issues/:issue/award_emoji/:award` https://gitlab.kitware.com/help/api/award_emoji.md#get-single-award-emoji
  * `DELETE /projects/:project/issues/:issue/award_emoji/:award` https://gitlab.kitware.com/help/api/award_emoji.md#delete-an-award-emoji
  * `POST   /projects/:project/issues/:issue/clone` https://gitlab.kitware.com/help/api/issues.md#clone-an-issue
  * `GET    /projects/:project/issues/:issue/discussions` https://gitlab.kitware.com/help/api/discussions.md#list-project-issue-discussion-items
  * `POST   /projects/:project/issues/:issue/discussions` https://gitlab.kitware.com/help/api/discussions.md#create-new-issue-thread
  * `GET    /projects/:project/issues/:issue/discussions/:discussion` https://gitlab.kitware.com/help/api/discussions.md#get-single-issue-discussion-item
  * `POST   /projects/:project/issues/:issue/discussions/:discussion/notes` https://gitlab.kitware.com/help/api/discussions.md#add-note-to-existing-issue-thread
  * `PUT    /projects/:project/issues/:issue/discussions/:discussion/notes/:note` https://gitlab.kitware.com/help/api/discussions.md#modify-existing-issue-thread-note
  * `DELETE /projects/:project/issues/:issue/discussions/:discussion/notes/:note` https://gitlab.kitware.com/help/api/discussions.md#delete-an-issue-thread-note
  * `GET    /projects/:project/issues/:issue/notes/:note` https://gitlab.kitware.com/help/api/notes.md#get-single-issue-note
  * `DELETE /projects/:project/issues/:issue/notes/:note` https://gitlab.kitware.com/help/api/notes.md#delete-an-issue-note
  * `GET    /projects/:project/issues/:issue/notes/:note/award_emoji` https://gitlab.kitware.com/help/api/award_emoji.md#list-a-comments-award-emoji
  * `POST   /projects/:project/issues/:issue/notes/:note/award_emoji` https://gitlab.kitware.com/help/api/award_emoji.md#award-a-new-emoji-on-a-comment
  * `GET    /projects/:project/issues/:issue/notes/:note/award_emoji/:award` https://gitlab.kitware.com/help/api/award_emoji.md#get-an-award-emoji-for-a-comment
  * `DELETE /projects/:project/issues/:issue/notes/:note/award_emoji/:award` https://gitlab.kitware.com/help/api/award_emoji.md#delete-an-award-emoji-from-a-comment
  * `GET    /projects/:project/issues/:issue/metric_images` https://gitlab.kitware.com/help/api/issues.md#list-metric-images
  * `POST   /projects/:project/issues/:issue/metric_images` https://gitlab.kitware.com/help/api/issues.md#upload-metric-image
  * `PUT    /projects/:project/issues/:issue/metric_images/:metric_image` https://gitlab.kitware.com/help/api/issues.md#update-metric-image
  * `DELETE /projects/:project/issues/:issue/metric_images/:metric_image` https://gitlab.kitware.com/help/api/issues.md#delete-metric-image
  * `POST   /projects/:project/issues/:issue/move` https://gitlab.kitware.com/help/api/issues.md#move-an-issue
  * `GET    /projects/:project/issues/:issue/participants` https://gitlab.kitware.com/help/api/issues.md#participants-on-issues
  * `GET    /projects/:project/issues/:issue/related_merge_requests` https://gitlab.kitware.com/help/api/issues.md#list-merge-requests-related-to-issue
  * `PUT    /projects/:project/issues/:issue/reorder` https://gitlab.kitware.com/help/api/issues.md#reorder-an-issue
    Arguably this should be a `POST` endpoint
  * `POST   /projects/:project/issues/:issue/reset_spent_time` https://gitlab.kitware.com/help/api/issues.md#reset-spent-time-for-an-issue
  * `POST   /projects/:project/issues/:issue/reset_time_estimate` https://gitlab.kitware.com/help/api/issues.md#reset-the-time-estimate-for-an-issue
  * `GET    /projects/:project/issues/:issue/resource_label_events/:event` https://gitlab.kitware.com/help/api/resource_label_events.md#get-single-issue-label-event
  * `POST   /projects/:project/issues/:issue/subscribe` https://gitlab.kitware.com/help/api/issues.md#subscribe-to-an-issue
  * `POST   /projects/:project/issues/:issue/time_estimate` https://gitlab.kitware.com/help/api/issues.md#set-a-time-estimate-for-an-issue
  * `GET    /projects/:project/issues/:issue/time_stats` https://gitlab.kitware.com/help/api/issues.md#get-time-tracking-stats
  * `POST   /projects/:project/issues/:issue/todo` https://gitlab.kitware.com/help/api/issues.md#create-a-to-do-item
  * `POST   /projects/:project/issues/:issue/unsubscribe` https://gitlab.kitware.com/help/api/issues.md#unsubscribe-from-an-issue
  * `GET    /projects/:project/issues/:issue/user_agent_detail` https://gitlab.kitware.com/help/api/issues.md#get-user-agent-details
  * `PUT    /projects/:project/labels/:label` https://gitlab.kitware.com/help/api/labels.md#edit-an-existing-label
  * `POST   /projects/:project/labels/:label/subscribe` https://gitlab.kitware.com/help/api/labels.md#subscribe-to-a-label
  * `POST   /projects/:project/labels/:label/unsubscribe` https://gitlab.kitware.com/help/api/labels.md#unsubscribe-from-a-label
  * `GET    /projects/:project/languages` https://gitlab.kitware.com/help/api/projects.md#languages
  * `DELETE /projects/:project/merge_requests/:merge_request` https://gitlab.kitware.com/help/api/merge_requests.md#delete-a-merge-request
  * `POST   /projects/:project/merge_requests/:merge_request/add_spent_time` https://gitlab.kitware.com/help/api/merge_requests.md#add-spent-time-for-a-merge-request
  * `POST   /projects/:project/merge_requests/:merge_request/approvals` https://gitlab.kitware.com/help/api/merge_request_approvals.md#change-approval-configuration
  * `POST   /projects/:project/merge_requests/:merge_request/approval_rules` https://gitlab.kitware.com/help/api/merge_request_approvals.md#get-merge-request-level-rules
  * `GET    /projects/:project/merge_requests/:merge_request/approval_rules/:approval_rule` https://gitlab.kitware.com/help/api/merge_request_approvals.md#get-a-single-merge-request-level-rule
  * `PUT    /projects/:project/merge_requests/:merge_request/approval_rules/:approval_rule` https://gitlab.kitware.com/help/api/merge_request_approvals.md#update-merge-request-level-rule
  * `DELETE /projects/:project/merge_requests/:merge_request/approval_rules/:approval_rule` https://gitlab.kitware.com/help/api/merge_request_approvals.md#delete-merge-request-level-rule
  * `POST   /projects/:project/merge_requests/:merge_request/award_emoji` https://gitlab.kitware.com/help/api/award_emoji.md#award-a-new-emoji
  * `GET    /projects/:project/merge_requests/:merge_request/award_emoji/:award` https://gitlab.kitware.com/help/api/award_emoji.md#get-single-award-emoji
  * `DELETE /projects/:project/merge_requests/:merge_request/award_emoji/:award` https://gitlab.kitware.com/help/api/award_emoji.md#delete-an-award-emoji
  * `POST   /projects/:project/merge_requests/:merge_request/cancel_merge_when_pipeline_succeeds` https://gitlab.kitware.com/help/api/merge_requests.md#cancel-merge-when-pipeline-succeeds
  * `GET    /projects/:project/merge_requests/:merge_request/discussions/:discussion` https://gitlab.kitware.com/help/api/discussions.md#get-single-merge-request-discussion-item
  * `PUT    /projects/:project/merge_requests/:merge_request/discussions/:discussion` https://gitlab.kitware.com/help/api/discussions.md#resolve-a-merge-request-thread
  * `POST   /projects/:project/merge_requests/:merge_request/discussions/:discussion/notes` https://gitlab.kitware.com/help/api/discussions.md#add-note-to-existing-merge-request-thread
  * `PUT    /projects/:project/merge_requests/:merge_request/discussions/:discussion/notes/:note` https://gitlab.kitware.com/help/api/discussions.md#modify-an-existing-merge-request-thread-note
  * `DELETE /projects/:project/merge_requests/:merge_request/discussions/:discussion/notes/:note` https://gitlab.kitware.com/help/api/discussions.md#delete-a-merge-request-thread-note
  * `GET    /projects/:project/merge_requests/:merge_request/merge_ref` https://gitlab.kitware.com/help/api/merge_requests.md#merge-to-default-merge-ref-path
    This should probably be a `POST` event?
  * `GET    /projects/:project/merge_requests/:merge_request/notes/:note` https://gitlab.kitware.com/help/api/notes.md#get-single-merge-request-note
  * `DELETE /projects/:project/merge_requests/:merge_request/notes/:note` https://gitlab.kitware.com/help/api/notes.md#delete-a-merge-request-note
  * `GET    /projects/:project/merge_requests/:merge_request/notes/:note/award_emoji/:award` https://gitlab.kitware.com/help/api/award_emoji.md#get-an-award-emoji-for-a-comment
  * `DELETE /projects/:project/merge_requests/:merge_request/notes/:note/award_emoji/:award` https://gitlab.kitware.com/help/api/award_emoji.md#delete-an-award-emoji-from-a-comment
  * `GET    /projects/:project/merge_requests/:merge_request/participants` https://gitlab.kitware.com/help/api/merge_requests.md#get-single-mr-participants
  * `POST   /projects/:project/merge_requests/:merge_request/reset_spent_time` https://gitlab.kitware.com/help/api/merge_requests.md#reset-spent-time-for-a-merge-request
  * `POST   /projects/:project/merge_requests/:merge_request/reset_time_estimate` https://gitlab.kitware.com/help/api/merge_requests.md#reset-the-time-estimate-for-a-merge-request
  * `GET    /projects/:project/merge_requests/:merge_request/resource_label_events/:event` https://gitlab.kitware.com/help/api/resource_label_events.md#get-single-merge-request-label-event
  * `POST   /projects/:project/merge_requests/:merge_request/subscribe` https://gitlab.kitware.com/help/api/merge_requests.md#subscribe-to-a-merge-request
  * `POST   /projects/:project/merge_requests/:merge_request/time_estimate` https://gitlab.kitware.com/help/api/merge_requests.md#set-a-time-estimate-for-a-merge-request
  * `GET    /projects/:project/merge_requests/:merge_request/time_stats` https://gitlab.kitware.com/help/api/merge_requests.md#get-time-tracking-stats
  * `POST   /projects/:project/merge_requests/:merge_request/todo` https://gitlab.kitware.com/help/api/merge_requests.md#create-a-to-do-item
  * `POST   /projects/:project/merge_requests/:merge_request/unsubscribe` https://gitlab.kitware.com/help/api/merge_requests.md#unsubscribe-from-a-merge-request
  * `GET    /projects/:project/merge_requests/:merge_request/versions` https://gitlab.kitware.com/help/api/merge_requests.md#get-mr-diff-versions
  * `GET    /projects/:project/merge_requests/:merge_request/versions/:version` https://gitlab.kitware.com/help/api/merge_requests.md#get-a-single-mr-diff-version
  * `GET    /projects/:project/milestones` https://gitlab.kitware.com/help/api/milestones.md#list-project-milestones
  * `GET    /projects/:project/milestones/:milestone` https://gitlab.kitware.com/help/api/milestones.md#get-single-milestone
  * `PUT    /projects/:project/milestones/:milestone` https://gitlab.kitware.com/help/api/milestones.md#edit-milestone
  * `DELETE /projects/:project/milestones/:milestone` https://gitlab.kitware.com/help/api/milestones.md#delete-project-milestone
  * `GET    /projects/:project/milestones/:milestone/burndown_events` https://gitlab.kitware.com/help/api/milestones.md#get-all-burndown-chart-events-for-a-single-milestone-starter
  * `GET    /projects/:project/milestones/:milestone/issues` https://gitlab.kitware.com/help/api/milestones.md#get-all-issues-assigned-to-a-single-milestone
  * `GET    /projects/:project/milestones/:milestone/merge_requests` https://gitlab.kitware.com/help/api/milestones.md#get-all-merge-requests-assigned-to-a-single-milestone
  * `POST   /projects/:project/milestones/:milestone/promote` https://gitlab.kitware.com/help/api/milestones.md#promote-project-milestone-to-a-group-milestone
  * `POST   /projects/:project/mirror/pull` https://gitlab.kitware.com/help/api/projects.md#start-the-pull-mirroring-process-for-a-project-starter
  * `PATCH  /projects/:project/protected_branches/:branch` https://gitlab.kitware.com/help/api/protected_branches.md#require-code-owner-approvals-for-a-single-branch
  * `POST   /projects/:project/pipeline` https://gitlab.kitware.com/help/api/pipelines.md#create-a-new-pipeline
  * `GET    /projects/:project/pipelines/:pipeline/bridges` https://gitlab.kitware.com/help/api/pipelines.md#list-pipeline-bridges
  * `GET    /projects/:project/pipelines/:pipeline/test_report` https://gitlab.kitware.com/help/api/pipelines.md#get-a-pipelines-test-report
  * `GET    /projects/:project/pipelines/:pipeline/test_report_summary` https://gitlab.kitware.com/help/api/pipelines.md#get-a-pipelines-test-report-summary
  * `GET    /projects/:project/push_rule` https://gitlab.kitware.com/help/api/projects.md#get-project-push-rules
  * `POST   /projects/:project/push_rule` https://gitlab.kitware.com/help/api/projects.md#add-project-push-rule
  * `PUT    /projects/:project/push_rule` https://gitlab.kitware.com/help/api/projects.md#edit-project-push-rule
  * `DELETE /projects/:project/push_rule` https://gitlab.kitware.com/help/api/projects.md#delete-project-push-rule
  * `DELETE /projects/:project/registry/repositories/:repository_id/tags` https://gitlab.kitware.com/help/api/container_registry.md#delete-registry-repository-tags-in-bulk
  * `POST   /projects/:project/releases` https://gitlab.kitware.com/help/api/releases/index.md#create-a-release
  * `GET    /projects/:project/releases/:tag_name` https://gitlab.kitware.com/help/api/releases/index.md#get-a-release-by-a-tag-name
  * `PUT    /projects/:project/releases/:tag_name` https://gitlab.kitware.com/help/api/releases/index.md#update-a-release
  * `DELETE /projects/:project/releases/:tag_name` https://gitlab.kitware.com/help/api/releases/index.md#delete-a-release
  * `GET    /projects/:project/releases/:tag_name/evidence` https://gitlab.kitware.com/help/api/releases/index.md#collect-release-evidence
  * `GET    /projects/:project/repository/blobs/:sha` https://gitlab.kitware.com/help/api/repositories.md#get-a-blob-from-repository
  * `GET    /projects/:project/repository/blobs/:sha/raw` https://gitlab.kitware.com/help/api/repositories.md#raw-blob-content
  * `DELETE /projects/:project/repository/branches/:branch` https://gitlab.kitware.com/help/api/branches.md#delete-repository-branch
  * `GET    /projects/:project/repository/changelog` https://gitlab.kitware.com/help/api/repositories.md#generate-changelog-data
  * `POST   /projects/:project/repository/changelog` https://gitlab.kitware.com/help/api/repositories.md#add-changelog-data-to-a-changelog-file
  * `POST   /projects/:project/repository/commits` https://gitlab.kitware.com/help/api/commits.md#create-a-commit-with-multiple-files-and-actions
  * `POST   /projects/:project/repository/commits/:sha/cherry_pick` https://gitlab.kitware.com/help/api/commits.md#cherry-pick-a-commit
  * `GET    /projects/:project/repository/commits/:sha/diff` https://gitlab.kitware.com/help/api/commits.md#get-the-diff-of-a-commit
  * `GET    /projects/:project/repository/commits/:sha/discussions` https://gitlab.kitware.com/help/api/commits.md#get-the-discussions-of-a-commit
  * `GET    /projects/:project/repository/commits/:sha/refs` https://gitlab.kitware.com/help/api/commits.md#get-references-a-commit-is-pushed-to
  * `POST   /projects/:project/repository/commits/:sha/revert` https://gitlab.kitware.com/help/api/commits.md#revert-a-commit
  * `GET    /projects/:project/repository/commits/:sha/signature` https://gitlab.kitware.com/help/api/commits.md#get-gpg-signature-of-a-commit
  * `GET    /projects/:project/repository/compare` https://gitlab.kitware.com/help/api/repositories.md#compare-branches-tags-or-commits
  * `GET    /projects/:project/repository/contributors` https://gitlab.kitware.com/help/api/repositories.md#contributors
  * `GET    /projects/:project/repository/files/*file_path` https://gitlab.kitware.com/help/api/repository_files.md#get-file-from-repository
  * `HEAD   /projects/:project/repository/files/*file_path` https://gitlab.kitware.com/help/api/repository_files.md#get-file-from-repository
  * `GET    /projects/:project/repository/files/*file_path/blame` https://gitlab.kitware.com/help/api/repository_files.md#get-file-blame-from-repository
  * `GET    /projects/:project/repository/merge_base` https://gitlab.kitware.com/help/api/repositories.md#merge-base
  * `DELETE /projects/:project/repository/merged_branches` https://gitlab.kitware.com/help/api/branches.md#delete-merged-branches
    Arguably this should be `POST /projects/:project/repository/delete_merged_branches`
  * `DELETE /projects/:project/repository/tags/:tag` https://gitlab.kitware.com/help/api/tags.md#delete-a-tag
  * `POST   /projects/:project/restore` https://gitlab.kitware.com/help/api/projects.md#restore-project-marked-for-deletion-premium
  * `GET    /projects/:project/snapshot` https://gitlab.kitware.com/help/api/projects.md#download-snapshot-of-a-git-repository
  * `GET    /projects/:project/snippets/:snippet/award_emoji` https://gitlab.kitware.com/help/api/award_emoji.md#list-an-awardables-award-emoji
  * `POST   /projects/:project/snippets/:snippet/award_emoji` https://gitlab.kitware.com/help/api/award_emoji.md#award-a-new-emoji
  * `GET    /projects/:project/snippets/:snippet/award_emoji/:award` https://gitlab.kitware.com/help/api/award_emoji.md#get-single-award-emoji
  * `DELETE /projects/:project/snippets/:snippet/award_emoji/:award` https://gitlab.kitware.com/help/api/award_emoji.md#delete-an-award-emoji
  * `GET    /projects/:project/snippets/:snippet/discussions` https://gitlab.kitware.com/help/api/discussions.md#list-project-snippet-discussion-items
  * `POST   /projects/:project/snippets/:snippet/discussions` https://gitlab.kitware.com/help/api/discussions.md#create-new-snippet-thread
  * `GET    /projects/:project/snippets/:snippet/discussions/:discussion` https://gitlab.kitware.com/help/api/discussions.md#get-single-snippet-discussion-item
  * `POST   /projects/:project/snippets/:snippet/discussions/:discussion/notes` https://gitlab.kitware.com/help/api/discussions.md#add-note-to-existing-snippet-thread
  * `PUT    /projects/:project/snippets/:snippet/discussions/:discussion/notes/:note` https://gitlab.kitware.com/help/api/discussions.md#modify-existing-snippet-thread-note
  * `DELETE /projects/:project/snippets/:snippet/discussions/:discussion/notes/:note` https://gitlab.kitware.com/help/api/discussions.md#delete-a-snippet-thread-note
  * `GET    /projects/:project/snippets/:snippet/notes` https://gitlab.kitware.com/help/api/notes.md#snippets
  * `POST   /projects/:project/snippets/:snippet/notes` https://gitlab.kitware.com/help/api/notes.md#get-single-snippet-note
  * `GET    /projects/:project/snippets/:snippet/notes/:note` https://gitlab.kitware.com/help/api/notes.md#get-single-snippet-note
  * `PUT    /projects/:project/snippets/:snippet/notes/:note` https://gitlab.kitware.com/help/api/notes.md#modify-existing-snippet-note
  * `DELETE /projects/:project/snippets/:snippet/notes/:note` https://gitlab.kitware.com/help/api/notes.md#delete-a-snippet-note
  * `GET    /projects/:project/snippets/:snippet/notes/:note/award_emoji` https://gitlab.kitware.com/help/api/award_emoji.md#list-a-comments-award-emoji
  * `POST   /projects/:project/snippets/:snippet/notes/:note/award_emoji` https://gitlab.kitware.com/help/api/award_emoji.md#award-a-new-emoji-on-a-comment
  * `GET    /projects/:project/snippets/:snippet/notes/:note/award_emoji/:award` https://gitlab.kitware.com/help/api/award_emoji.md#get-an-award-emoji-for-a-comment
  * `DELETE /projects/:project/snippets/:snippet/notes/:note/award_emoji/:award` https://gitlab.kitware.com/help/api/award_emoji.md#delete-an-award-emoji-from-a-comment
  * `POST   /projects/:project/star` https://gitlab.kitware.com/help/api/projects.md#star-a-project
  * `GET    /projects/:project/starrers` https://gitlab.kitware.com/help/api/projects.md#list-starrers-of-a-project
  * `GET    /projects/:project/storage` https://gitlab.kitware.com/help/api/projects.md#get-the-path-to-repository-storage
  * `PUT    /projects/:project/transfer` https://gitlab.kitware.com/help/api/projects.md#transfer-a-project-to-a-new-namespace
  * `POST   /projects/:project/unarchive` https://gitlab.kitware.com/help/api/projects.md#unarchive-a-project
  * `POST   /projects/:project/unstar` https://gitlab.kitware.com/help/api/projects.md#unstar-a-project
  * `POST   /projects/:project/uploads` https://gitlab.kitware.com/help/api/projects.md#upload-a-file
  * `GET    /projects/:project/users` https://gitlab.kitware.com/help/api/projects.md#get-project-users
  * `GET    /projects/:project/variables` `https://gitlab.kitware.com/help/api/project_level_variables.md#list-project-variables`
  * `DELETE /projects/:project/variables/:key` `https://gitlab.kitware.com/help/api/project_level_variables.md#remove-variable
  * `POST   /projects/user/:user` https://gitlab.kitware.com/help/api/projects.md#create-project-for-user
  * `GET    /registry/repositories/:id` https://gitlab.kitware.com/help/api/container_registry.md#get-details-of-a-single-repository
  * `GET    /user/activities` https://gitlab.kitware.com/help/api/users.md#get-user-activities-admin-only
  * `POST   /user/disable_two_factor` https://gitlab.kitware.com/help/api/users.md#disable-two-factor-authentication
  * `GET    /user/emails` https://gitlab.kitware.com/help/api/users.md#list-emails
  * `POST   /user/emails` https://gitlab.kitware.com/help/api/users.md#add-email
  * `GET    /user/emails/:email` https://gitlab.kitware.com/help/api/users.md#single-email
  * `DELETE /user/emails/:email` https://gitlab.kitware.com/help/api/users.md#delete-email-for-current-user
  * `GET    /user/gpg_keys` https://gitlab.kitware.com/help/api/users.md#list-all-gpg-keys
  * `POST   /user/gpg_keys` https://gitlab.kitware.com/help/api/users.md#add-a-gpg-key
  * `GET    /user/gpg_keys/:gpg_key` https://gitlab.kitware.com/help/api/users.md#get-a-specific-gpg-key
  * `DELETE /user/gpg_keys/:gpg_key` https://gitlab.kitware.com/help/api/users.md#delete-a-gpg-key
  * `GET    /user/keys` https://gitlab.kitware.com/help/api/users.md#list-user-projects
  * `POST   /user/keys` https://gitlab.kitware.com/help/api/users.md#add-ssh-key
  * `GET    /user/keys/:key` https://gitlab.kitware.com/help/api/users.md#single-ssh-key
  * `DELETE /user/keys/:key` https://gitlab.kitware.com/help/api/users.md#delete-ssh-key-for-current-user
  * `GET    /user/preferences` https://gitlab.kitware.com/help/api/users.md#user-preferences
  * `PUT    /user/preferences` https://gitlab.kitware.com/help/api/users.md#user-preference-modification
  * `GET    /user/status` https://gitlab.kitware.com/help/api/users.md#user-status
  * `PUT    /user/status` https://gitlab.kitware.com/help/api/users.md#set-user-status
  * `POST   /users` https://gitlab.kitware.com/help/api/users.md#user-creation
  * `DELETE /users/:user` https://gitlab.kitware.com/help/api/users.md#user-deletion
  * `PUT    /users/:user` https://gitlab.kitware.com/help/api/users.md#user-modification
  * `POST   /users/:user/activate` https://gitlab.kitware.com/help/api/users.md#activate-user
  * `POST   /users/:user/approve` https://gitlab.kitware.com/help/api/users.md#approve-user
  * `POST   /users/:user/ban` https://gitlab.kitware.com/help/api/users.md#ban-user
  * `POST   /users/:user/block` https://gitlab.kitware.com/help/api/users.md#block-user
  * `POST   /users/:user/deactivate` https://gitlab.kitware.com/help/api/users.md#deactivate-user
  * `GET    /users/:user/emails` https://gitlab.kitware.com/help/api/users.md#list-emails-for-user
  * `POST   /users/:user/emails` https://gitlab.kitware.com/help/api/users.md#add-email-for-user
  * `DELETE /users/:user/emails/:email` https://gitlab.kitware.com/help/api/users.md#delete-email-for-given-user
  * `GET    /users/:user/followers` https://gitlab.kitware.com/help/api/users.md#followers-and-following
  * `GET    /users/:user/following` https://gitlab.kitware.com/help/api/users.md#followers-and-following
  * `POST   /users/:user/follow` https://gitlab.kitware.com/help/api/users.md#follow-and-unfollow-users
  * `GET    /users/:user/gpg_keys` https://gitlab.kitware.com/help/api/users.md#list-all-gpg-keys-for-given-user
  * `POST   /users/:user/gpg_keys` https://gitlab.kitware.com/help/api/users.md#add-a-gpg-key-for-a-given-user
  * `GET    /users/:user/gpg_keys/:gpg_key` https://gitlab.kitware.com/help/api/users.md#get-a-specific-gpg-key-for-a-given-user
  * `DELETE /users/:user/gpg_keys/:gpg_key` https://gitlab.kitware.com/help/api/users.md#add-a-gpg-key-for-a-given-user
  * `DELETE /users/:user/identities/:provider` https://gitlab.kitware.com/help/api/users.md#delete-authentication-identity-from-user
  * `GET    /users/:user/keys` https://gitlab.kitware.com/help/api/users.md#list-ssh-keys-for-user
  * `POST   /users/:user/keys` https://gitlab.kitware.com/help/api/users.md#add-ssh-key-for-user
  * `DELETE /users/:user/keys/:key` https://gitlab.kitware.com/help/api/users.md#delete-ssh-key-for-given-user
  * `GET    /users/:user/memberships` https://gitlab.kitware.com/help/api/users.md#user-memberships-admin-only
  * `POST   /users/:user/personal_access_tokens` https://gitlab.kitware.com/help/api/users.md#create-a-personal-access-token
  * `GET    /users/:user/project_deploy_keys` https://gitlab.kitware.com/help/api/deploy_keys.md#list-project-deploy-keys-for-user
  * `GET    /users/:user/projects` https://gitlab.kitware.com/help/api/projects.md#list-user-projects
  * `POST   /users/:user/reject` https://gitlab.kitware.com/help/api/users.md#reject-user
  * `GET    /users/:user/starred_projects` https://gitlab.kitware.com/help/api/projects.md#list-projects-starred-by-a-user
  * `GET    /users/:user/status` https://gitlab.kitware.com/help/api/users.md#get-the-status-of-a-user
  * `POST   /users/:user/unban` https://gitlab.kitware.com/help/api/users.md#unban-user
  * `POST   /users/:user/unblock` https://gitlab.kitware.com/help/api/users.md#unblock-user
  * `POST   /users/:user/unfollow` https://gitlab.kitware.com/help/api/users.md#follow-and-unfollow-users
  * `GET    /user_counts` https://gitlab.kitware.com/help/api/users.md#user-counts
  * `GET    /v2/_catalog` https://gitlab.kitware.com/help/api/container_registry.md#listing-all-container-repositories

## Endpoint groups

These pages document other endpoints not mentioned above:

  * https://gitlab.kitware.com/help/api/access_requests.md
  * https://gitlab.kitware.com/help/api/access_tokens.md
  * https://gitlab.kitware.com/help/api/admin_sidekiq_queues.md
  * https://gitlab.kitware.com/help/api/appearance.md
  * https://gitlab.kitware.com/help/api/applications.md
  * https://gitlab.kitware.com/help/api/audit_events.md
    - keyset pagination is supported for group audit events with `order_by=id` and `sort=desc`
  * https://gitlab.kitware.com/help/api/avatar.md
  * https://gitlab.kitware.com/help/api/boards.md
  * https://gitlab.kitware.com/help/api/broadcast_messages.md
  * https://gitlab.kitware.com/help/api/cluster_agents.md
  * https://gitlab.kitware.com/help/api/custom_attributes.md
  * https://gitlab.kitware.com/help/api/dependencies.md
  * https://gitlab.kitware.com/help/api/dependency_proxy.md
  * https://gitlab.kitware.com/help/api/deploy_tokens.md
  * https://gitlab.kitware.com/help/api/deployments.md
  * https://gitlab.kitware.com/help/api/epic_issues.md
  * https://gitlab.kitware.com/help/api/epic_links.md
  * https://gitlab.kitware.com/help/api/epics.md
  * https://gitlab.kitware.com/help/api/error_tracking.md
  * https://gitlab.kitware.com/help/api/events.md
  * https://gitlab.kitware.com/help/api/experiments.md
  * https://gitlab.kitware.com/help/api/feature_flag_users_lists.md
  * https://gitlab.kitware.com/help/api/feature_flags.md
  * https://gitlab.kitware.com/help/api/features.md
  * https://gitlab.kitware.com/help/api/freeze_periods.md
  * https://gitlab.kitware.com/help/api/geo_nodes.md
  * https://gitlab.kitware.com/help/api/group_access_tokens.md
  * https://gitlab.kitware.com/help/api/group_activity_analytics.md
  * https://gitlab.kitware.com/help/api/group_badges.md
  * https://gitlab.kitware.com/help/api/group_boards.md
  * https://gitlab.kitware.com/help/api/group_clusters.md (deprecated)
  * https://gitlab.kitware.com/help/api/group_import_export.md
  * https://gitlab.kitware.com/help/api/group_iterations.md
  * https://gitlab.kitware.com/help/api/group_labels.md
  * https://gitlab.kitware.com/help/api/group_level_variables.md
  * https://gitlab.kitware.com/help/api/group_releases.md
  * https://gitlab.kitware.com/help/api/group_repository_storage_moves.md
  * https://gitlab.kitware.com/help/api/group_wikis.md
  * https://gitlab.kitware.com/help/api/import.md
  * https://gitlab.kitware.com/help/api/instance_clusters.md
  * https://gitlab.kitware.com/help/api/instance_level_ci_variables.md
  * https://gitlab.kitware.com/help/api/integrations.md
  * https://gitlab.kitware.com/help/api/invitations.md
  * https://gitlab.kitware.com/help/api/issue_links.md
  * https://gitlab.kitware.com/help/api/issues_statistics.md
  * https://gitlab.kitware.com/help/api/iterations.md
  * https://gitlab.kitware.com/help/api/job_artifacts.md
  * https://gitlab.kitware.com/help/api/keys.md
  * https://gitlab.kitware.com/help/api/license.md
  * https://gitlab.kitware.com/help/api/linked_epics.md
  * https://gitlab.kitware.com/help/api/lint.md
  * https://gitlab.kitware.com/help/api/managed_licenses.md
  * https://gitlab.kitware.com/help/api/markdown.md
  * https://gitlab.kitware.com/help/api/merge_request_context_commits.md
  * https://gitlab.kitware.com/help/api/metadata.md
  * https://gitlab.kitware.com/help/api/metrics_dashboard_annotations.md
  * https://gitlab.kitware.com/help/api/metrics_user_starred_dashboards.md
  * https://gitlab.kitware.com/help/api/namespaces.md
  * https://gitlab.kitware.com/help/api/notification_settings.md
  * https://gitlab.kitware.com/help/api/oauth2.md
  * https://gitlab.kitware.com/help/api/packages.md
  * https://gitlab.kitware.com/help/api/packages/debian_project_distributions.md
  * https://gitlab.kitware.com/help/api/pages.md
  * https://gitlab.kitware.com/help/api/pages_domains.md
  * https://gitlab.kitware.com/help/api/personal_access_tokens.md
  * https://gitlab.kitware.com/help/api/pipeline_schedules.md
  * https://gitlab.kitware.com/help/api/pipeline_triggers.md
  * https://gitlab.kitware.com/help/api/plan_limits.md
  * https://gitlab.kitware.com/help/api/project_aliases.md
  * https://gitlab.kitware.com/help/api/project_badges.md
  * https://gitlab.kitware.com/help/api/project_clusters.md
  * https://gitlab.kitware.com/help/api/project_import_export.md
  * https://gitlab.kitware.com/help/api/project_repository_storage_moves.md
  * https://gitlab.kitware.com/help/api/project_snippets.md
  * https://gitlab.kitware.com/help/api/project_statistics.md
  * https://gitlab.kitware.com/help/api/project_templates.md
  * https://gitlab.kitware.com/help/api/project_vulnerabilities.md
  * https://gitlab.kitware.com/help/api/protected_environments.md
  * https://gitlab.kitware.com/help/api/releases/links.md
  * https://gitlab.kitware.com/help/api/remote_mirrors.md
  * https://gitlab.kitware.com/help/api/repository_submodules.md
  * https://gitlab.kitware.com/help/api/resource_iteration_events.md
  * https://gitlab.kitware.com/help/api/resource_milestone_events.md
  * https://gitlab.kitware.com/help/api/resource_state_events.md
  * https://gitlab.kitware.com/help/api/resource_weight_events.md
  * https://gitlab.kitware.com/help/api/runners.md
  * https://gitlab.kitware.com/help/api/scim.md
  * https://gitlab.kitware.com/help/api/search.md
  * https://gitlab.kitware.com/help/api/settings.md
  * https://gitlab.kitware.com/help/api/sidekiq_metrics.md
  * https://gitlab.kitware.com/help/api/snippet_repository_storage_moves.md
  * https://gitlab.kitware.com/help/api/snippets.md
  * https://gitlab.kitware.com/help/api/statistics.md
  * https://gitlab.kitware.com/help/api/suggestions.md
  * https://gitlab.kitware.com/help/api/system_hooks.md
  * https://gitlab.kitware.com/help/api/templates/dockerfiles.md
  * https://gitlab.kitware.com/help/api/templates/gitignores.md
  * https://gitlab.kitware.com/help/api/templates/gitlab_ci_ymls.md
  * https://gitlab.kitware.com/help/api/templates/licenses.md
  * https://gitlab.kitware.com/help/api/todos.md
  * https://gitlab.kitware.com/help/api/topics.md
  * https://gitlab.kitware.com/help/api/usage_data.md
  * https://gitlab.kitware.com/help/api/version.md
  * https://gitlab.kitware.com/help/api/visual_review_discussions.md
  * https://gitlab.kitware.com/help/api/vulnerabilities.md
  * https://gitlab.kitware.com/help/api/vulnerability_exports.md
  * https://gitlab.kitware.com/help/api/vulnerability_findings.md
  * https://gitlab.kitware.com/help/api/wikis.md
