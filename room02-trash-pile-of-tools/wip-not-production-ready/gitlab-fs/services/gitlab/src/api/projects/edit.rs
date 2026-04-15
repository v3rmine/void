// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::BTreeSet;

use derive_builder::Builder;

use crate::api::common::{EnableState, NameOrId, VisibilityLevel};
use crate::api::endpoint_prelude::*;
use crate::api::projects::{
    AutoDevOpsDeployStrategy, BuildGitStrategy, ContainerExpirationPolicy, FeatureAccessLevel,
    FeatureAccessLevelPublic, MergeMethod, SquashOption,
};

/// Edit an existing project.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct EditProject<'a> {
    /// The project to edit.
    #[builder(setter(into))]
    project: NameOrId<'a>,

    /// The name of the project.
    #[builder(setter(into), default)]
    name: Option<Cow<'a, str>>,
    /// The path of the project.
    #[builder(setter(into), default)]
    path: Option<Cow<'a, str>>,

    /// The default branch of the new project.
    #[builder(setter(into), default)]
    default_branch: Option<Cow<'a, str>>,
    /// The description of the new project.
    #[builder(setter(into), default)]
    description: Option<Cow<'a, str>>,

    /// Set the access level for issues.
    #[builder(default)]
    issues_access_level: Option<FeatureAccessLevel>,
    /// Set the access level for repository access.
    #[builder(default)]
    repository_access_level: Option<FeatureAccessLevel>,
    /// Set the access level for container registry access.
    #[builder(default)]
    container_registry_access_level: Option<FeatureAccessLevel>,
    /// Set the access level for merge requests.
    #[builder(default)]
    merge_requests_access_level: Option<FeatureAccessLevel>,
    /// Set the access level for making a fork of the project.
    #[builder(default)]
    forking_access_level: Option<FeatureAccessLevel>,
    /// Set the access level for CI pipeline access.
    #[builder(default)]
    builds_access_level: Option<FeatureAccessLevel>,
    /// Set the access level for access to view the wiki.
    #[builder(default)]
    wiki_access_level: Option<FeatureAccessLevel>,
    /// Set the access level for snippets.
    #[builder(default)]
    snippets_access_level: Option<FeatureAccessLevel>,
    /// Set the access level for GitLab Pages on the project.
    #[builder(default)]
    pages_access_level: Option<FeatureAccessLevelPublic>,
    /// Set the access level for operations features.
    #[builder(default)]
    operations_access_level: Option<FeatureAccessLevel>,
    /// Set the access level for requirements features.
    #[builder(default)]
    requirements_access_level: Option<FeatureAccessLevel>,
    /// Set the access level for analytics features.
    #[builder(default)]
    analytics_access_level: Option<FeatureAccessLevel>,
    /// Set the access level for security and compliance features.
    #[builder(default)]
    security_and_compliance_access_level: Option<FeatureAccessLevel>,

    /// Whether to enable email notifications or not.
    #[builder(default)]
    emails_disabled: Option<bool>,
    /// Whether the default set of award emojis are shown for this project.
    #[builder(default)]
    show_default_award_emojis: Option<bool>,
    /// Whether to allow non-members to set pipeline variables when triggering piplines or not.
    #[builder(default)]
    restrict_user_defined_variables: Option<bool>,
    /// Whether outdated diff discussions are resolved when a merge request is updated or not.
    #[builder(default)]
    resolve_outdated_diff_discussions: Option<bool>,
    /// Whether the container registry is enabled or not.
    #[builder(default)]
    #[deprecated(note = "use `container_registry_access_level` instead")]
    container_registry_enabled: Option<bool>,
    /// The expiration policy for containers.
    #[builder(default)]
    container_expiration_policy_attributes: Option<ContainerExpirationPolicy<'a>>,
    /// Whether the project can use shared runners or not.
    #[builder(default)]
    shared_runners_enabled: Option<bool>,
    /// The visibility level of the project.
    #[builder(default)]
    visibility: Option<VisibilityLevel>,
    /// A URL to import the repository from.
    #[builder(setter(into), default)]
    import_url: Option<Cow<'a, str>>,
    /// Whether job results are visible to non-project members or not.
    #[builder(default)]
    public_builds: Option<bool>,
    /// Whether the CI pipeline is required to succeed before merges are allowed.
    #[builder(default)]
    only_allow_merge_if_pipeline_succeeds: Option<bool>,
    /// Whether the CI pipeline can be skipped before merges are allowed.
    #[builder(default)]
    allow_merge_on_skipped_pipeline: Option<bool>,
    /// Whether all discussions must be resolved before merges are allowed.
    #[builder(default)]
    only_allow_merge_if_all_discussions_are_resolved: Option<bool>,
    /// The merge commit template.
    #[builder(setter(into), default)]
    merge_commit_template: Option<Cow<'a, str>>,
    /// The squash merge commit template.
    #[builder(setter(into), default)]
    squash_commit_template: Option<Cow<'a, str>>,
    /// The default description for issues.
    #[builder(setter(into), default)]
    issues_template: Option<Cow<'a, str>>,
    /// The default description for issues.
    #[builder(setter(into), default)]
    merge_requests_template: Option<Cow<'a, str>>,
    /// The merge method to use for the project.
    #[builder(default)]
    merge_method: Option<MergeMethod>,
    /// The squash option for the project.
    #[builder(default)]
    squash_option: Option<SquashOption>,
    /// Whether merge pipelines are enabled.
    #[builder(default)]
    merge_pipelines_enabled: Option<bool>,
    /// Whether merge trains are enabled.
    #[builder(default)]
    merge_trains_enabled: Option<bool>,
    /// Whether MRs default to targing this project or the upstream project.
    #[builder(default)]
    mr_default_target_self: Option<bool>,
    /// Whether issues referenced on the default branch should be closed or not.
    #[builder(default)]
    autoclose_referenced_issues: Option<bool>,
    /// The commit message to use for code suggestion commits.
    #[builder(setter(into), default)]
    suggestion_commit_message: Option<Cow<'a, str>>,
    /// Whether to enabled the "Remove source branch" option in new merge requests by default or
    /// not.
    #[builder(default)]
    remove_source_branch_after_merge: Option<bool>,
    /// Whether to enable print merge request links if branch/commits are pushed by console
    #[builder(default)]
    printing_merge_request_link_enabled: Option<bool>,
    /// Whether `git-lfs` support should be enabled or not.
    ///
    /// See the [git-lfs](https://git-lfs.github.com/) website for more information.
    #[builder(default)]
    lfs_enabled: Option<bool>,
    /// Whether users may request access to the repository or not.
    #[builder(default)]
    request_access_enabled: Option<bool>,
    /// A list of tags to apply to the repository.
    #[builder(setter(name = "_tag_list"), default, private)]
    tag_list: BTreeSet<Cow<'a, str>>,
    /// A list of topics to apply to the repository.
    #[builder(setter(name = "_topics"), default, private)]
    topics: BTreeSet<Cow<'a, str>>,
    // TODO: Figure out how to actually use this.
    // avatar   mixed   no  Image file for avatar of the project
    // avatar: ???,
    /// The default Git strategy for CI jobs of the project.
    #[builder(default)]
    build_git_strategy: Option<BuildGitStrategy>,
    /// The default timeout for jobs of the project (in seconds).
    #[builder(default)]
    build_timeout: Option<u64>,
    /// Whether to automatically cancel pipelines when branches are updated when using a previous
    /// version of the branch.
    #[builder(setter(into), default)]
    auto_cancel_pending_pipelines: Option<EnableState>,
    /// The default regular expression to use for build coverage extraction.
    #[builder(setter(into), default)]
    build_coverage_regex: Option<Cow<'a, str>>,
    /// The path to the GitLab CI configuration file within the repository.
    ///
    /// Defaults to `.gitlab-ci.yml`.
    #[builder(setter(into), default)]
    ci_config_path: Option<Cow<'a, str>>,
    /// The default number of revisions to fetch in CI jobs.
    #[builder(default)]
    ci_default_git_depth: Option<u64>,
    /// Whether to skip pending deployment jobs when a newer one is started.
    #[builder(default)]
    ci_forward_deployment_enabled: Option<bool>,
    /// Whether Auto DevOps are enabled or not.
    #[builder(default)]
    auto_devops_enabled: Option<bool>,
    /// The Auto Deploy strategy of the project.
    #[builder(default)]
    auto_devops_deploy_strategy: Option<AutoDevOpsDeployStrategy>,
    /// The storage shard on which to store the repository.
    #[builder(setter(into), default)]
    repository_storage: Option<Cow<'a, str>>,
    /// How many approvals are required before allowing merges.
    #[builder(default)]
    approvals_before_merge: Option<u64>,
    /// The classification label of the project.
    #[builder(setter(into), default)]
    external_authorization_classification_label: Option<Cow<'a, str>>,
    /// Whether to enable pull mirroring for the project or not.
    #[builder(default)]
    mirror: Option<bool>,
    /// User to attribute all mirror activity to.
    #[builder(default)]
    mirror_user_id: Option<u64>,
    /// Whether mirror updates trigger CI builds ir not.
    #[builder(default)]
    mirror_trigger_builds: Option<bool>,
    /// Whether to only mirror protected branches or not.
    #[builder(default)]
    only_mirror_protected_branches: Option<bool>,
    /// Whether the mirror overwrites diverged branches in this project or not.
    #[builder(default)]
    mirror_overwrites_diverged_branches: Option<bool>,
    /// Whether the package repository is enabled or not.
    #[builder(default)]
    packages_enabled: Option<bool>,
    /// Whether the service desk is enabled or not.
    #[builder(default)]
    service_desk_enabled: Option<bool>,
    /// Whether to keep the latest artifact for pipelines or not.
    #[builder(default)]
    keep_latest_artifact: Option<bool>,
    /// Whether to or not caches should be separated based on branch protection status or not.
    #[builder(default)]
    ci_separated_caches: Option<bool>,

    /// Whether to enable issues or not.
    #[deprecated(note = "use `issues_access_level` instead")]
    #[builder(default)]
    issues_enabled: Option<bool>,
    /// Whether to enable merge requests or not.
    #[deprecated(note = "use `merge_requests_access_level` instead")]
    #[builder(default)]
    merge_requests_enabled: Option<bool>,
    /// Whether to enable CI pipelines or not.
    #[deprecated(note = "use `builds_access_level` instead")]
    #[builder(default)]
    jobs_enabled: Option<bool>,
    /// Whether to enable the wiki or not.
    #[deprecated(note = "use `wiki_access_level` instead")]
    #[builder(default)]
    wiki_enabled: Option<bool>,
    /// Whether to enable snippets or not.
    #[deprecated(note = "use `snippets_access_level` instead")]
    #[builder(default)]
    snippets_enabled: Option<bool>,
}

impl<'a> EditProject<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> EditProjectBuilder<'a> {
        EditProjectBuilder::default()
    }
}

impl<'a> EditProjectBuilder<'a> {
    /// Add a tag.
    #[deprecated(note = "use `topic` instead")]
    pub fn tag<T>(&mut self, tag: T) -> &mut Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.tag_list
            .get_or_insert_with(BTreeSet::new)
            .insert(tag.into());
        self
    }

    /// Add multiple tags.
    #[deprecated(note = "use `topics` instead")]
    pub fn tags<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.tag_list
            .get_or_insert_with(BTreeSet::new)
            .extend(iter.map(Into::into));
        self
    }

    /// Add a topic.
    pub fn topic<T>(&mut self, topic: T) -> &mut Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.topics
            .get_or_insert_with(BTreeSet::new)
            .insert(topic.into());
        self
    }

    /// Add multiple topics.
    pub fn topics<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.topics
            .get_or_insert_with(BTreeSet::new)
            .extend(iter.map(Into::into));
        self
    }

    /// Whether to enable print merge request links if branch/commits are pushed by console
    ///
    /// This was a typo previously; use `printing_merge_request_link_enabled` instead.
    #[deprecated(note = "use `printing_merge_request_link_enabled` instead")]
    pub fn printing_merge_requests_link_enabled(&mut self, enable: bool) -> &mut Self {
        self.printing_merge_request_link_enabled(enable)
    }
}

impl<'a> Endpoint for EditProject<'a> {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}", self.project).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params
            .push_opt("name", self.name.as_ref())
            .push_opt("path", self.path.as_ref())
            .push_opt("default_branch", self.default_branch.as_ref())
            .push_opt("description", self.description.as_ref())
            .push_opt("issues_access_level", self.issues_access_level)
            .push_opt("repository_access_level", self.repository_access_level)
            .push_opt(
                "container_registry_access_level",
                self.container_registry_access_level,
            )
            .push_opt(
                "merge_requests_access_level",
                self.merge_requests_access_level,
            )
            .push_opt("forking_access_level", self.forking_access_level)
            .push_opt("builds_access_level", self.builds_access_level)
            .push_opt("wiki_access_level", self.wiki_access_level)
            .push_opt("snippets_access_level", self.snippets_access_level)
            .push_opt("pages_access_level", self.pages_access_level)
            .push_opt("operations_access_level", self.operations_access_level)
            .push_opt("requirements_access_level", self.requirements_access_level)
            .push_opt("analytics_access_level", self.analytics_access_level)
            .push_opt(
                "security_and_compliance_access_level",
                self.security_and_compliance_access_level,
            )
            .push_opt("emails_disabled", self.emails_disabled)
            .push_opt("show_default_award_emojis", self.show_default_award_emojis)
            .push_opt(
                "restrict_user_defined_variables",
                self.restrict_user_defined_variables,
            )
            .push_opt(
                "resolve_outdated_diff_discussions",
                self.resolve_outdated_diff_discussions,
            )
            .push_opt("shared_runners_enabled", self.shared_runners_enabled)
            .push_opt("visibility", self.visibility)
            .push_opt("import_url", self.import_url.as_ref())
            .push_opt("public_builds", self.public_builds)
            .push_opt(
                "only_allow_merge_if_pipeline_succeeds",
                self.only_allow_merge_if_pipeline_succeeds,
            )
            .push_opt(
                "allow_merge_on_skipped_pipeline",
                self.allow_merge_on_skipped_pipeline,
            )
            .push_opt(
                "only_allow_merge_if_all_discussions_are_resolved",
                self.only_allow_merge_if_all_discussions_are_resolved,
            )
            .push_opt("merge_commit_template", self.merge_commit_template.as_ref())
            .push_opt(
                "squash_commit_template",
                self.squash_commit_template.as_ref(),
            )
            .push_opt("issues_template", self.issues_template.as_ref())
            .push_opt(
                "merge_requests_template",
                self.merge_requests_template.as_ref(),
            )
            .push_opt("merge_method", self.merge_method)
            .push_opt("squash_option", self.squash_option)
            .push_opt("merge_pipelines_enabled", self.merge_pipelines_enabled)
            .push_opt("merge_trains_enabled", self.merge_trains_enabled)
            .push_opt("mr_default_target_self", self.mr_default_target_self)
            .push_opt(
                "autoclose_referenced_issues",
                self.autoclose_referenced_issues,
            )
            .push_opt(
                "suggestion_commit_message",
                self.suggestion_commit_message.as_ref(),
            )
            .push_opt(
                "remove_source_branch_after_merge",
                self.remove_source_branch_after_merge,
            )
            .push_opt(
                "printing_merge_request_link_enabled",
                self.printing_merge_request_link_enabled,
            )
            .push_opt("lfs_enabled", self.lfs_enabled)
            .push_opt("request_access_enabled", self.request_access_enabled)
            .extend(self.tag_list.iter().map(|value| ("tag_list[]", value)))
            .extend(self.topics.iter().map(|value| ("topics[]", value)))
            .push_opt("build_git_strategy", self.build_git_strategy)
            .push_opt("build_timeout", self.build_timeout)
            .push_opt(
                "auto_cancel_pending_pipelines",
                self.auto_cancel_pending_pipelines,
            )
            .push_opt("build_coverage_regex", self.build_coverage_regex.as_ref())
            .push_opt("ci_config_path", self.ci_config_path.as_ref())
            .push_opt("ci_default_git_depth", self.ci_default_git_depth)
            .push_opt(
                "ci_forward_deployment_enabled",
                self.ci_forward_deployment_enabled,
            )
            .push_opt("auto_devops_enabled", self.auto_devops_enabled)
            .push_opt(
                "auto_devops_deploy_strategy",
                self.auto_devops_deploy_strategy,
            )
            .push_opt("repository_storage", self.repository_storage.as_ref())
            .push_opt("approvals_before_merge", self.approvals_before_merge)
            .push_opt(
                "external_authorization_classification_label",
                self.external_authorization_classification_label.as_ref(),
            )
            .push_opt("mirror", self.mirror)
            .push_opt("mirror_user_id", self.mirror_user_id)
            .push_opt("mirror_trigger_builds", self.mirror_trigger_builds)
            .push_opt(
                "only_mirror_protected_branches",
                self.only_mirror_protected_branches,
            )
            .push_opt(
                "mirror_overwrites_diverged_branches",
                self.mirror_overwrites_diverged_branches,
            )
            .push_opt("packages_enabled", self.packages_enabled)
            .push_opt("service_desk_enabled", self.service_desk_enabled)
            .push_opt("keep_latest_artifact", self.keep_latest_artifact)
            .push_opt("ci_separated_caches", self.ci_separated_caches);

        if let Some(policy) = self.container_expiration_policy_attributes.as_ref() {
            policy.add_query(&mut params);
        }

        #[allow(deprecated)]
        {
            params
                .push_opt("issues_enabled", self.issues_enabled)
                .push_opt("merge_requests_enabled", self.merge_requests_enabled)
                .push_opt("jobs_enabled", self.jobs_enabled)
                .push_opt("wiki_enabled", self.wiki_enabled)
                .push_opt("snippets_enabled", self.snippets_enabled)
                .push_opt(
                    "container_registry_enabled",
                    self.container_registry_enabled,
                );
        }

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::common::{EnableState, VisibilityLevel};
    use crate::api::projects::{
        AutoDevOpsDeployStrategy, BuildGitStrategy, ContainerExpirationCadence,
        ContainerExpirationKeepN, ContainerExpirationOlderThan, ContainerExpirationPolicy,
        EditProject, EditProjectBuilderError, FeatureAccessLevel, FeatureAccessLevelPublic,
        MergeMethod, SquashOption,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_is_needed() {
        let err = EditProject::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, EditProjectBuilderError, "project");
    }

    #[test]
    fn project_is_sufficient() {
        EditProject::builder().project("project").build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_name() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("name=name")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .name("name")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_path() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("path=path")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .path("path")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_default_branch() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("default_branch=master")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .default_branch("master")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_description() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("description=description")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .description("description")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_issues_access_level() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("issues_access_level=enabled")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .issues_access_level(FeatureAccessLevel::Enabled)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_repository_access_level() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("repository_access_level=disabled")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .repository_access_level(FeatureAccessLevel::Disabled)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_container_registry_access_level() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("container_registry_access_level=disabled")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .container_registry_access_level(FeatureAccessLevel::Disabled)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_merge_requests_access_level() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("merge_requests_access_level=private")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .merge_requests_access_level(FeatureAccessLevel::Private)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_forking_access_level() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("forking_access_level=enabled")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .forking_access_level(FeatureAccessLevel::Enabled)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_builds_access_level() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("builds_access_level=enabled")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .builds_access_level(FeatureAccessLevel::Enabled)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_wiki_access_level() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("wiki_access_level=disabled")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .wiki_access_level(FeatureAccessLevel::Disabled)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_snippets_access_level() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("snippets_access_level=disabled")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .snippets_access_level(FeatureAccessLevel::Disabled)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_pages_access_level() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("pages_access_level=public")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .pages_access_level(FeatureAccessLevelPublic::Public)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_operations_access_level() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("operations_access_level=enabled")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .operations_access_level(FeatureAccessLevel::Enabled)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_requirements_access_level() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("requirements_access_level=enabled")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .requirements_access_level(FeatureAccessLevel::Enabled)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_analytics_access_level() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("analytics_access_level=private")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .analytics_access_level(FeatureAccessLevel::Private)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_security_and_compliance_access_level() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("security_and_compliance_access_level=private")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .security_and_compliance_access_level(FeatureAccessLevel::Private)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_emails_disabled() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("emails_disabled=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .emails_disabled(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_show_default_award_emojis() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("show_default_award_emojis=false")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .show_default_award_emojis(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_restrict_user_defined_variables() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("restrict_user_defined_variables=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .restrict_user_defined_variables(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_resolve_outdated_diff_discussions() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("resolve_outdated_diff_discussions=false")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .resolve_outdated_diff_discussions(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    #[allow(deprecated)]
    fn endpoint_container_registry_enabled() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("container_registry_enabled=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .container_registry_enabled(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_container_expiration_policy_attributes_cadence() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("container_expiration_policy_attributes%5Bcadence%5D=7d")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .container_expiration_policy_attributes(
                ContainerExpirationPolicy::builder()
                    .cadence(ContainerExpirationCadence::OneWeek)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_container_expiration_policy_attributes_enabled() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("container_expiration_policy_attributes%5Benabled%5D=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .container_expiration_policy_attributes(
                ContainerExpirationPolicy::builder()
                    .enabled(true)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_container_expiration_policy_attributes_keep_n() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("container_expiration_policy_attributes%5Bkeep_n%5D=5")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .container_expiration_policy_attributes(
                ContainerExpirationPolicy::builder()
                    .keep_n(ContainerExpirationKeepN::Five)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_container_expiration_policy_attributes_older_than() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("container_expiration_policy_attributes%5Bolder_than%5D=7d")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .container_expiration_policy_attributes(
                ContainerExpirationPolicy::builder()
                    .older_than(ContainerExpirationOlderThan::OneWeek)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_container_expiration_policy_attributes_name_regex() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("container_expiration_policy_attributes%5Bname_regex%5D=%3Alatest")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .container_expiration_policy_attributes(
                ContainerExpirationPolicy::builder()
                    .name_regex(":latest")
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_container_expiration_policy_attributes_all() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "container_expiration_policy_attributes%5Bcadence%5D=7d",
                "&container_expiration_policy_attributes%5Benabled%5D=true",
                "&container_expiration_policy_attributes%5Bkeep_n%5D=5",
                "&container_expiration_policy_attributes%5Bolder_than%5D=7d",
                "&container_expiration_policy_attributes%5Bname_regex%5D=%3Alatest",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .container_expiration_policy_attributes(
                ContainerExpirationPolicy::builder()
                    .cadence(ContainerExpirationCadence::OneWeek)
                    .enabled(true)
                    .keep_n(ContainerExpirationKeepN::Five)
                    .older_than(ContainerExpirationOlderThan::OneWeek)
                    .name_regex(":latest")
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_shared_runners_enabled() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("shared_runners_enabled=false")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .shared_runners_enabled(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_visibility() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("visibility=public")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .visibility(VisibilityLevel::Public)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_import_url() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("import_url=https%3A%2F%2Ftest.invalid%2Fpath%3Fsome%3Dfoo")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .import_url("https://test.invalid/path?some=foo")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_public_builds() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("public_builds=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .public_builds(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_only_allow_merge_if_pipeline_succeeds() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("only_allow_merge_if_pipeline_succeeds=false")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .only_allow_merge_if_pipeline_succeeds(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_allow_merge_on_skipped_pipeline() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("allow_merge_on_skipped_pipeline=false")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .allow_merge_on_skipped_pipeline(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_only_allow_merge_if_all_discussions_are_resolved() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("only_allow_merge_if_all_discussions_are_resolved=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .only_allow_merge_if_all_discussions_are_resolved(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_merge_commit_template() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("merge_commit_template=template")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .merge_commit_template("template")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_squash_commit_template() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("squash_commit_template=template")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .squash_commit_template("template")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_issues_template() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("issues_template=template")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .issues_template("template")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_merge_requests_template() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("merge_requests_template=template")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .merge_requests_template("template")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_merge_method() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("merge_method=ff")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .merge_method(MergeMethod::FastForward)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_squash_option() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("squash_option=never")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .squash_option(SquashOption::Never)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_merge_pipelines_enabled() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("merge_pipelines_enabled=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .merge_pipelines_enabled(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_merge_trains_enabled() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("merge_trains_enabled=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .merge_trains_enabled(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_mr_default_target_self() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("mr_default_target_self=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .mr_default_target_self(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_autoclose_referenced_issues() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("autoclose_referenced_issues=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .autoclose_referenced_issues(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_suggestion_commit_message() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("suggestion_commit_message=suggested+change")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .suggestion_commit_message("suggested change")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_remove_source_branch_after_merge() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("remove_source_branch_after_merge=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .remove_source_branch_after_merge(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_printing_merge_request_link_enabled() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("printing_merge_request_link_enabled=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .printing_merge_request_link_enabled(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_lfs_enabled() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("lfs_enabled=false")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .lfs_enabled(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_request_access_enabled() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("request_access_enabled=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .request_access_enabled(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    #[allow(deprecated)]
    fn endpoint_tag_list() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("tag_list%5B%5D=tag1", "&tag_list%5B%5D=tag2"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .tag("tag1")
            .tags(["tag1", "tag2"].iter().copied())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_topics() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("topics%5B%5D=topic1", "&topics%5B%5D=topic2"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .topic("topic1")
            .topics(["topic1", "topic2"].iter().copied())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_build_git_strategy() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("build_git_strategy=fetch")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .build_git_strategy(BuildGitStrategy::Fetch)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_build_timeout() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("build_timeout=1")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .build_timeout(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_auto_cancel_pending_pipelines() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("auto_cancel_pending_pipelines=enabled")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .auto_cancel_pending_pipelines(EnableState::Enabled)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_build_coverage_regex() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("build_coverage_regex=%5Cd%25")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .build_coverage_regex("\\d%")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_ci_config_path() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("ci_config_path=.gitlab-ci.yaml")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .ci_config_path(".gitlab-ci.yaml")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_ci_default_git_depth() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("ci_default_git_depth=1")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .ci_default_git_depth(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_ci_forward_deployment_enabled() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("ci_forward_deployment_enabled=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .ci_forward_deployment_enabled(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_auto_devops_enabled() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("auto_devops_enabled=false")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .auto_devops_enabled(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_auto_devops_deploy_strategy() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("auto_devops_deploy_strategy=manual")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .auto_devops_deploy_strategy(AutoDevOpsDeployStrategy::Manual)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_repository_storage() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("repository_storage=shard1")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .repository_storage("shard1")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_approvals_before_merge() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("approvals_before_merge=2")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .approvals_before_merge(2)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_external_authorization_classification_label() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("external_authorization_classification_label=external")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .external_authorization_classification_label("external")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_mirror() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("mirror=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .mirror(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_mirror_trigger_builds() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("mirror_trigger_builds=false")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .mirror_trigger_builds(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_only_mirror_protected_branches() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("only_mirror_protected_branches=false")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .only_mirror_protected_branches(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_mirror_overwrites_diverged_branches() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("mirror_overwrites_diverged_branches=false")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .mirror_overwrites_diverged_branches(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_packages_enabled() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("packages_enabled=false")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .packages_enabled(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_service_desk_enabled() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("service_desk_enabled=false")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .service_desk_enabled(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_keep_latest_artifact() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("keep_latest_artifact=false")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .keep_latest_artifact(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_ci_separated_caches() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("ci_separated_caches=false")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .ci_separated_caches(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    #[allow(deprecated)]
    fn endpoint_issues_enabled() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("issues_enabled=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .issues_enabled(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    #[allow(deprecated)]
    fn endpoint_merge_requests_enabled() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("merge_requests_enabled=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .merge_requests_enabled(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    #[allow(deprecated)]
    fn endpoint_jobs_enabled() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("jobs_enabled=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .jobs_enabled(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    #[allow(deprecated)]
    fn endpoint_wiki_enabled() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("wiki_enabled=false")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .wiki_enabled(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    #[allow(deprecated)]
    fn endpoint_snippets_enabled() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject")
            .content_type("application/x-www-form-urlencoded")
            .body_str("snippets_enabled=false")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditProject::builder()
            .project("simple/project")
            .snippets_enabled(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
