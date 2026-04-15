# v0.1508.0

## Additions

  * Add API endpoint `api::projects::merge_requests::approval_rules`.

# v0.1507.0

## Additions

  * Implement `Clone` for structs that implement `Endpoint`.
  * Support for container registry endpoints.
  * Support `/project/:project/repository/archive` endpoint.

# v0.1506.0

  * No changes needed for GitLab 15.6.

# v0.1505.0

  * No changes needed for GitLab 15.5.

# v0.1504.0

## Additions

  * API endpoints for user impersonation tokens.

# v0.1503.0

## Additions

  * `types::GroupHook` has been added.
  * New endpoints for:
    - Creating, getting, editing, listing and deleting group hooks.
  * Support for `Push` branch protections when creating groups.
  * Support for sorting project search results by `Similarity`.
  * Support for setting group shared runner settings, `path`,
    `prevent_sharing_groups_outside_hierarchy`,
    `prevent_forking_outside_group`, and `file_template_project_id`.
  * Support for `invite_source`, `tasks_to_be_done`, and `tasks_project_id`
    when adding a member to a group or project.
  * Support `skip_subresources` when deleting group memberships.
  * Support for filtering on `epic_id` when listing issues.
  * Support for issue types when creating, editing, and filtering issues.
  * Support sorting merge requests by title.
  * Support requesting retried jobs when requesting pipeline jobs.
  * Support pipeline status states `Preparing` and `WaitingForResource`.
  * Support setting the `security_and_compliance_access_level` when creating
    and editing projects.
  * Support editing the `mr_default_target_self` and `ci_separated_caches`
    project fields.
  * Support setting variables when playing manual jobs.
  * Support `merge_request_diff_sha` when editing and creating MR notes
    (required for the `/merge` command).
  * Support filtering pipelines by their source.
  * Support allowing force pushes when protecting branches.
  * Support including HTML descriptions in project releases.
  * Support sorting parameters for project releases.
  * Support including trailers in commit queries.
  * Support `execute_filemode` and `last_commit_id` when editing files through
    the API.
  * Support querying custom attributes of users.
  * Support `without_project_bots` and `saml_provider_id` when listing users.
  * Support for filtering projects based on import status and topics.
  * Support for finer-grained creation rules for protected tags.
  * Support filters when updating project pipeline variables.
  * Keyset pagination is now supported for group searches ordered by name.
  * Support filtering groups based on custom attributes.
  * The `all` project membership endpoints have been split out.

## Deprecations

  * `api::issues::IssueOrderBy::WeightFields` is now
    `api::issues::IssueOrderBy::Weight`. The old name seems to come from a
    misreading of the documentation's list of supported fields. Its field name
    has also been fixed to `weight` instead of `weight_fields`.
  * `api::projects::{CreateProject, EditProject}::container_registry_enabled`
    is deprecated in favor of `container_registry_access_level`.
  * `api::projects::{CreateProject, EditProject}::{tag, tags}` are deprecated
    in favor of `topic` and `topics`.
  * `api::projects::merge_requests::MergeRequests::search_in` is deprecated
    because GitLab doesn't actually have such a query parameter.
  * `api::projects::merge_requests::MergeRequestSearchScope` is deprecated
    because GitLab doesn't actually have such a query parameter it represented.
  * `api::projects::pipelines::Pipelines::name` is deprecated; GitLab never
    supported such a parameter.
  * `api::projects::members::{ProjectMember, ProjectMembers}::all_members` is
    deprecated. The `all` endpoints have different parameters, so merging them
    doesn't work anymore.

## Breaking changes

  * `api::deploy_keys::DeployKeys::public` is now private.
  * `api::projects::{CreateProject, EditProject}::requirements_access_level` do
    not support `Public`, so they now uses `api::FeatureAccessLevel` instead of
    `api::FeatureAccessLevelPublic`.
  * `api::users::Users::external` is now a `()` instead of `bool` because
    GitLab doesn't actually support `external=false`.
  * `api::projects::repository::Tree` now uses keyset pagination and requires
    GitLab 15.0. Page-based iteration has been deprecated by GitLab.
  * `api::projects::variables::ProjectVariableFilter::environment_scope` is now
    private.

# v0.1502.0

## Changes

  * `graphql_client` has been updated to 0.11.

# v0.1501.0

## Changes

  * `api::projects::merge_requests::MergeRequestPipelines` and associated
    structures have been moved to `api::projects::merge_requests::pipelines`.

## Fixes

  * `api::projects::edit::EditProject` fixes a typo from
    `printing_merge_requests_link_enabled` to
    `printing_merge_request_link_enabled`
  * `api::projects::protected_branches::ProtectedBranches` is now paginated.

## Additions

  * `types::UserState::Deactivated` has been added.
  * New endpoints for:
    - Sharing and unsharing groups
    - Listing shared projects in a group
    - Sharing and unsharing projects
    - Creating a pipeline on a merge request
  * New fields on `types::Hook`:
    - `push_events_branch_filter`
    - `issues_events`
    - `confidential_issues_events`
    - `merge_requests_events`
    - `note_events`
    - `confidential_note_events`
    - `repository_update_events`
    - `job_events`
    - `pipeline_events`
    - `wiki_page_events`
  * New fields on `types::ProjectHook`:
    - `deployment_events`
    - `releases_events`
  * New fields on `types::Project`:
    - `build_git_strategy`
    - `ci_default_git_depth`
  * New fields on `webhooks::MergeRequestHook`:
    - `changes`
  * New `webhooks::MergeRequestChanges` structure

# v0.1500.0

  * No changes needed for GitLab 15.0.

# v0.1410.0

  * No changes needed for GitLab 14.10.

-------
# v0.1409.1 (unreleased)

## Additions
  * The `api::projects::repository::branches::Branches` endpoint is now pageable.
  * Entend type `webhooks::MergeRequestHook` with changes entry
  * The `api::projects::protected_branches::ProtectedBranches` endpoint is now pageable.
  * Extend UserState enum with Deactivated
  * Extend Hook type with fields:
    - push_events_branch_filter
    - issues_events
    - confidential_issues_events
    - merge_requests_events
    - note_events
    - confidential_note_events
    - job_events
    - pipeline_events
    - wiki_page_events
    - repository_update_events
  * Extend Project type with fields:
    - build_git_strategy
    - ci_default_git_depth
  * Move api endpoint `api::projects::merge_requests::pipelines` to `api::projects::merge_requests::pipelines::pipelines`
  * Add api endpoint `api::projects::merge_requests::pipelines::create`
  * Fix parameters for editing projects:
    - printing_merge_request_link_enabled
-------

# v0.1409.0

  * No changes needed for GitLab 14.9.

# v0.1408.0

## Additions

  * Added the `api::projects::merge_trains` endpoint to get merge trains
    for a specific project.

# v0.1407.0

## Breaking changes

  * `types::RepoCommit::parent_ids` is now an `Option`.

## Fixes

  * Protected tags with special URL characters (namely `/`) are now handled
    properly with `api::projects::protected_tags::ProtectedTag`.
  * Tags on a repository are now pageable.
  * The `types::DiscussionNoteType::Note` variant has been added.

## Additions

  * Groups can now be edited via `EditGroup`.
  * New `rustls` feature to support static linking (no OpenSSL).
  * New type definitions for:
    - `MergeRequestCommit`
    - `ProjectVariable`
    - `ProtectedTagAccessLevel`
    - `ProtectedTag`
    - `ReleaseTag`
    - `Tag`
  * New endpoints for:
    - Creating, updating, and inspecting project-level pipeline variables.
    - Deleting project hooks.
    - Getting tree object from projects.
    - Updating a file in a project.
    - Getting merge requests associated with a commit.
    - Getting commits of a merge request.
    - Deleting a file from the repository.
    - Deploy keys
  * New parameters for creating projects:
    - `container_registry_access_level`
    - `merge_pipelines_enabled`
    - `merge_trains_enabled`
    - `squash_option`
    - `topics`
  * New parameters for editing projects:
    - `container_registry_access_level`
    - `merge_commit_template`
    - `squash_commit_template`
    - `issues_template`
    - `merge_requests_template`
    - `squash_option`
    - `merge_pipelines_enabled`
    - `merge_trains_enabled`
    - `printing_merge_requests_link_enabled`
    - `topics`
    - `keep_latest_artifact`
  * Repository branches are now pageable

## Changes

  * Use explicit type for referencing associated items for rustc 1.57

# v0.1406.0

  * No changes needed for GitLab 14.6.

# v0.1405.1

  * No functional changes.  Only documentation and tests were updated.

# v0.1405.0

## Additions

  * Missing `Builder` type exports have been added.

## Breaking changes

  * The error types from `Builder` instances are now distinct per builder type.

# v0.1404.0

## Additions

  * Added the `first_contributors` field to `api::projects::merge_requests::MergeRequest`
  * Added the `api::projects::merge_requests::pipelines` endpoint to get
    the pipelines attached to a merge request
  * Added the `api::projects::merge_requests::changes` endpoint to get
    information about a merge request along with diffs
  * Made field `code_owner_approval_required` in `types::ProtectedRepoBranch`
    an `Option<bool>` instead of `bool` as the field is only present in
    Gitlab premium
  * Added `Paged::iter_async` returning a paginated asynchronous stream
    analogous to `Pages::iter`.
  * Added the `api::groups::issues::Issues` endpoint to get issues associated
    with a group.
  * Added support for TLS-authenticated clients to `GitlabBuilder`

# v0.1403.0

  * No changes needed for GitLab 14.3.

# v0.1402.0

## Additions

  * Added the `api::projects::merge_requests::approvals::MergeRequestApprovals`
    endpoint to get approvals of a merge request.
  * Added graphql requests to the async client.
  * The `Gitlab` client now supports unauthenticated connections.

## New features

  * Clients may be wrapped by `api::retry::Client` in order to perform
    exponential backoff for service-side errors. Backoff parameters are
    configurable via the `api::retry::Backoff` structure.

# v0.1401.0

## Breaking changes

  * `types::MergeRequest::source_project_id` is now an `Option<ProjectId>`

# v0.1400.0

## Changes

  * A new `RestClient` trait has been refactored from `Client` and
    `AsyncClient`.
  * The `api::paged` type now has an `.iter()` method which may be used to
    iterate over paginated results using lazily fetched API results. This can
    be used to reduce memory usage for large result sets.
  * Added a `confidential` parameter for `api::projects::issues::EditIssue`
  * GitLab responses which do not return JSON (e.g., 5xx status codes) are now
    caught as the `ApiError::GitlabService` error variant. Previously, the JSON
    deserialization would have been exposed.

# v0.1312.0

## Breaking changes

  * Allow arbitrary strings for the UID of `ExternalProvider` matching what the
    API expects and allows.

# v0.1311.2

## Additions

  * Project hooks can now be edited via `EditHook`.

# v0.1311.1

## Breaking changes

  * Upgraded `reqwest` and `bytes` which use tokio 1.0.

# v0.1311.0

## Additions

  * Added the `api::projects::releases::ProjectReleases` endpoint to list all
    releases for a project.
  * Added tags related api endpoints under `api::projects::repository::tags`
  * Listing commits in a repository can now be done via `Commits`
  * Added asynchronous API for query `api::AsyncQuery` and client `api::AsyncClient`.
  * Added asynchronous client `AsyncGitlab` (created by `GitlabBuilder::build_async`).

# v0.1310.0

## Additions

  * `types::{Pipeline,PipelineBasic}` now have a `project_id` member.

# v0.1309.0

## Breaking changes

  * `ParamValue::as_value` now takes its value as `&self` rather than `self`.
    This was required in order to implement `CommaSeparatedList` reliably.
  * Merge request discussions on code now have a more fine-grained API. This
    change was made by GitLab and is just being followed by the crate.

## Additions

  * `api::common::CommaSeparatedList` now exists for easy use of
    comma-separated values.
  * Project members can now be removed via `RemoveProjectMember`.
  * Group members can now be edited via `EditGroupMember`.
  * Project members can now be edited via `EditProjectMember`.

## Deprecations

  * `EditIssue::remove_labels` is deprecated in favor of the better
    `clear_labels` wording.
  * `EditMergeRequest::remove_labels` is deprecated in favor of the better
    `clear_labels` wording.

## Changes

  * API bindings for the `"minimal"` access level.
  * Groups can have "inherit" set as their shared runner minute limit.
  * Listing groups can now be set to only return top-level groups.
  * Searching for projects within a group can now be sorted by a similarity
    score based on the search criteria.
  * Project container expiration policies can now use an arbitrary "keep n"
    count.
  * Project container expiration policies now have `name_regex_delete`
    (replacing the now-deprecated `name_regex`) and `name_regex_keep`.
  * Projects can now be created and edited with `operations_access_level`
    settings.
  * Projects can now be created and edited with `requirements_access_level`
    settings.
  * Projects can now be created and edited with `analytics_access_level`
    settings.
  * Projects can now be created and edited with `show_default_award_emojis`
    settings.
  * Projects can now be created and edited with
    `restrict_user_defined_variables` settings.
  * Projects can now be created and edited with
    `allow_merge_on_skipped_pipeline` settings.
  * Projects can now be edited with `ci_forward_deployment_enabled` settings.
  * Environments can now be filtered by their deployment state.
  * Project hooks can now be registered for events related to confidential
    notes, deployments, and releases.
  * Issues can now be edited with incremental label changes.
  * Issues can now be filtered by iterations, due dates, and search queries can
    now be scoped.
  * Issue notes can now be created and edited with the confidential flag.
  * Project labels can be filtered by search queries.
  * Project members can now be edited in batch (using multiple IDs).
  * Merge requests can now be created and edited with reviewer settings.
  * Merge requests can now be created with the `approvals_before_merge`
    setting.
  * Merge request discussions can now be created on a specific commit.
  * Merge requests can now be edited with incremental label changes.
  * Merge requests can now be filtered by search scopes.
  * Merge requests can now trigger merge status rechecks when listing.
  * Merge requests can now be filtered by reviewer.
  * Merge requests can now be filtered by environment status.
  * API bindings for the `"scheduled"` pipeline status.
  * Projects can now be sorted by various resource sizes.
  * Projects can now be filtered by storage backend.
  * Users can now be filtered by GitLab-internal users and administrator
    status.
  * Group member removal can now specify to unassign issuables.

# v0.1308.0

  * No changes needed for GitLab 13.8.

# v0.1307.0

## Additions

  * Added `api::projects::repository::files::FileRaw`
  * Added `api::projects::merge_requests::approval_state::MergeRequestApprovalState`
    query to access the approval rules state of a particular merge request.

# v0.1306.0

  * No changes needed for GitLab 13.6.

# v0.1305.1

## Changes

  * Changed `ci_config_path` to `Option<String>` in `gitlab::webhooks::PipelineHookAttrs`

# v0.1305.0

## Additions

  * Added `head_pipeline_id` field to `gitlab::webhooks::MergeRequestHookAttrs`

# v0.1304.0

## Changes

  * Error types now use `#[non_exhaustive]`

# v0.1303.0

## Additions

  * Added `gitlab::webhooks::PipelineHook`

# v0.1302.2

## Additions

  * `Id` types now implement `Hash`

# v0.1302.1

## Additions

  * Added `api::projects::issues::MergeRequestsClosing` and
    `api::projects::issues::MergeRequestsClosing`

## Fixes

  * GitLab 13.2 added the `approved` and `unapproved` merge request actions for
    CE.

# v0.1302.0

## Additions

  * Added the `api::projects::protected_tags::ProtectTag`
    `api::projects::protected_tags::UnprotectTag`
    `api::projects::protected_tags::ProtectedTag`
    `api::projects::protected_tags::ProtectedTags` endpoint to query, protect
    and unprotect a projects tags.
  * Added the `api::projects::labels::DeleteLabel` endpoint to delete existing
    labels from a project.
  * Added the `api::projects::labels::PromoteLabel` endpoint to promote a project
    label to a group label.
  * Added the `api::projects:merge_requests::MergeMergeRequest` endpoint to
    merge open merge requests.
  * Added the `api::projects:merge_requests::RebaseMergeRequest` endpoint to
    rebase open merge requests when using the fast-forward merge model.
  * Added the `api::projects:merge_requests::ApproveMergeRequest` endpoint to
    approve open merge requests.
  * Added the `api::projects:merge_requests::UnapproveMergeRequest` endpoint to
    unapprove approved merge requests.

# v0.1301.1

## Changes

  * Updated `api::projects::members::ProjectMember[s]` to support the ability
    to include member details for those members that have access as a result
    of belonging to ancestor/enclosing groups, in addition to directly added
    members.
  * Allow a label via the `api::projects::labels::Label` endpoint to be queried
    by id or name.

## Additions

  * Added the `api::groups::projects::GroupProjects` endpoint to list a groups
    projects.
  * Added the `api::groups::subgroups::GroupSubgroups` endpoint to list a
    groups subgroups.
  * Added the `api::projects::protected_branches::ProtectedBranches` endpoint
    to list a projects protected branches.
  * Added the `api::projects::protected_branches::ProtectedBranch` endpoint
    to query a projects protected branch.

## Fixes

  * Added pagination support to `api::projects::labels::Labels`
  * Keyset pagination also supports the to-be-removed (14.0) `Links` HTTP
    header.

# v0.1301.0

## Deprecations

  * The REST endpoint methods on the `Gitlab` structure have been removed.
    Associated helper structures for resource creation endpoints have been
    removed as well:
    - `CreateMergeRequestParams`
    - `CreateMergeRequestParamsBuilder`
    - `CreateGroupParams`
    - `CreateGroupParamsBuilder`
    - `CreateProjectParams`
    - `CreateProjectParamsBuilder`
    - `MergeMethod`
    - `BuildGitStrategy`
    - `AutoDeployStrategy`
    - `WebhookEvents`
    - `CommitStatusInfo`
    - `MergeRequestStateFilter`
    - `RepoFile`
    - `ProjectFeatures`
    - `QueryParamSlice`
    - `QueryParamVec`
  * Now-impossible error conditions have been removed from `GitlabError`.

# v0.1300.0

## Deprecations

  * All methods on the `Gitlab` structure now have `Endpoint` structures
    implemented. In a future release, these methods (and their support types)
    will be removed.
  * The `Serialize` implementations of the API types are deprecated (though
    marking them as such is difficult).

## Changes

  * The `api::projects::issues::Issues` endpoint's `milestone` field was
    changed to match the actual API exposed by GitLab (with `None` and `Any`
    options).
  * The `api::projects::pipelines::PipelineVariables` endpoint is now pageable.
  * All `EnableState` fields may now be set using `bool` values.
  * The `api::projects::merge_requests::EditMergeRequest` endpoint now supports
    unlabeling a merge request.
  * The `api::Client` trait has been changed to use the `http` crate types.
    This allows for clients to not be tied to `reqwest` and for mocking and
    testing of the endpoints themselves.
  * GitLab errors now detect error objects returned from the API.

## Fixes

  * The `min_access_level` field for `api::groups::Groups` and the
    `access_level` for `api::projects::members::AddProjectMember` are now
    properly passed as integers to the API. (#42)
  * The path used for the project and group milestone endpoints has been fixed.

# v0.1210.2

## New request body handling

It was observed (#41) that the new API pattern was not handling `POST` and
`PUT` parameters properly. This has now been fixed.

## New request parameter handling

In the process of updating the body handling, a simpler pattern for query
parameters was also implemented.

## Additional merge status cases

Some additional merge status names for merge requests were missing and have
been added.

## Fixes

  * The `api::projects::environments::Environment` endpoint uses the correct
    path now.
  * The `api::groups::members::GroupMembers`,
    `api::projects::members::ProjectMembers`, and
    `api::projects::repository::Branches` endpoints now accepts plain strings
    for their `query` fields.
  * The `api::projects::protected_branches::UnprotectBranch` endpoint now
    properly escapes branch names with URL-special characters.
  * The `api::projects::repository::CreateFile` endpoint now properly upgrades
    the encoding when attempting to encode binary contents using
    `Encoding::Text`.
  * The `api::projects::CreateProject` and `api::projects::EditProject`
    endpoints now accepts plain strings in its `import_url` field.

## Changes

  * The `api::projects::issues::EditIssue` now uses `issue` rather than
    `issue_iid` for consistency.

# v0.1210.1

## New API strategy

A new pattern for API implementation is now underway. Instead of methods
directly on the `Gitlab` instance, there are now structures which implement an
`api::Endpoint` trait. This trait may be used to query any structure
implementing the `api::Client` trait using the `api::Query` trait. All
endpoints use the "builder" pattern to collect required and optional
parameters.

There are some adaptor functions to handle various use cases:

  - `api::paged`: This may be used to handle pagination of any endpoint which
    supports it (checked at compile time).
  - `api::ignore`: This may be used to ignore the content of the response for
    any endpoint. HTTP and GitLab error messages are still captured.
  - `api::raw`: Instead of deserializing the contents of the result from GitLab
    into a structure, the raw bytes may be fetched instead using this function.
  - `api::sudo`: This function adapts any endpoint into being called as another
    user if the client is able to do so (basically, is an administrator).

The `api::Query` trait deserializes the contents from GitLab into any structure
which implements the `serde::DeserializeOwned` trait. This can be used to only
grab information of interest to the caller instead of extracting all of the
information available through the `types` module.

If your endpoint is deprecated, it has been marked as such and you should
migrate to the new pattern. Please see the docs for available endpoints.

All new endpoint implementations should use the new pattern rather than adding
methods to `Gitlab`. Result structures do not need to be added to this crate
either. It is expected that they too will be deprecated at some point and
either not provided or moved to a dedicated crate.

### Examples:

```rust
use std::env;

use serde::Deserialize;
use gitlab::Gitlab;
use gitlab::api::{self, projects, Query};

#[derive(Debug, Deserialize)]
struct Project {
    name: String,
}

fn example() {
    // Create the client.
    let client = Gitlab::new("gitlab.com", env::get("GITLAB_TOKEN").unwrap()).unwrap();

    // Create a simple endpoint.
    let endpoint = projects::Project::builder().project("gitlab-org/gitlab").build().unwrap();
    // Get the information.
    let project: Project = endpoint.query(&client).unwrap();
    // Call it again, but ignore the response from GitLab.
    let _: () = api::ignore(endpoint).query(&client).unwrap();

    // Create an endpoint that supports pagination.
    let pageable_endpoint = projects::Projects::builder().build().unwrap();
    // Get just the first page (20 results).
    let first_page: Vec<Project> = pageable_endpoint.query(&client).unwrap();
    // Get 200 results instead.
    let first_200_projects: Vec<Project> = api::paged(pageable_endpoint, api::Pagination::Limit(200)).query(&client).unwrap();

    // Query `gitlab-org/gitlab` except by ID this time.
    let endpoint = projects::Project::builder().project(278964).build().unwrap();
    // Get the raw data from the response.
    let raw_data: Vec<u8> = api::raw(endpoint).query(&client).unwrap();
}
```

## Changes

  * Include a changelog.
