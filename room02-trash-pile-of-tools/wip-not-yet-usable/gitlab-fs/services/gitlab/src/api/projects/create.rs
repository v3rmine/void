// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::cmp::Ordering;
use std::collections::BTreeSet;

use derive_builder::Builder;

use crate::api::common::{EnableState, VisibilityLevel};
use crate::api::endpoint_prelude::*;
use crate::api::ParamValue;

/// Access levels available for most features.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeatureAccessLevel {
    /// The feature is not available at all.
    Disabled,
    /// The features is only available to project members.
    Private,
    /// The feature is available to everyone with access to the project.
    Enabled,
}

impl FeatureAccessLevel {
    /// The variable type query parameter.
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            FeatureAccessLevel::Disabled => "disabled",
            FeatureAccessLevel::Private => "private",
            FeatureAccessLevel::Enabled => "enabled",
        }
    }
}

impl ParamValue<'static> for FeatureAccessLevel {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Access levels available for features.
///
/// Note that only the `pages` feature currently uses this.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeatureAccessLevelPublic {
    /// The feature is not available at all.
    Disabled,
    /// The features is only available to project members.
    Private,
    /// The feature is available to everyone with access to the project.
    Enabled,
    /// The feature is publicly available regardless of project access.
    Public,
}

impl FeatureAccessLevelPublic {
    /// The variable type query parameter.
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            FeatureAccessLevelPublic::Disabled => "disabled",
            FeatureAccessLevelPublic::Private => "private",
            FeatureAccessLevelPublic::Enabled => "enabled",
            FeatureAccessLevelPublic::Public => "public",
        }
    }
}

impl ParamValue<'static> for FeatureAccessLevelPublic {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// How often the container expiration policy is applied.
///
/// Note that GitLab only supports a few discrete values for this setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ContainerExpirationCadence {
    /// Every day.
    OneDay,
    /// Every week.
    OneWeek,
    /// Every other week.
    TwoWeeks,
    /// Every month.
    OneMonth,
    /// Quaterly.
    ThreeMonths,
}

impl ContainerExpirationCadence {
    /// The variable type query parameter.
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            ContainerExpirationCadence::OneDay => "1d",
            ContainerExpirationCadence::OneWeek => "7d",
            ContainerExpirationCadence::TwoWeeks => "14d",
            ContainerExpirationCadence::OneMonth => "1month",
            ContainerExpirationCadence::ThreeMonths => "3month",
        }
    }
}

impl ParamValue<'static> for ContainerExpirationCadence {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// How many container instances to keep around.
///
/// Note that GitLab only supports a few discrete values for this setting.
#[derive(Debug, Clone, Copy, Eq)]
pub enum ContainerExpirationKeepN {
    /// Only one.
    One,
    /// Up to five.
    Five,
    /// Up to ten.
    Ten,
    /// Up to twenty-five.
    TwentyFive,
    /// Up to fifty.
    Fifty,
    /// Up to one hundred.
    OneHundred,
    /// Arbitrary number.
    Arbitrary(u64),
}

impl From<u64> for ContainerExpirationKeepN {
    fn from(n: u64) -> Self {
        Self::Arbitrary(n)
    }
}

impl PartialEq for ContainerExpirationKeepN {
    fn eq(&self, other: &Self) -> bool {
        self.as_u64().eq(&other.as_u64())
    }
}

impl PartialOrd for ContainerExpirationKeepN {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_u64().partial_cmp(&other.as_u64())
    }
}

impl Ord for ContainerExpirationKeepN {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_u64().cmp(&other.as_u64())
    }
}

impl ContainerExpirationKeepN {
    /// The variable type query parameter.
    pub(crate) fn as_str(self) -> Cow<'static, str> {
        match self {
            ContainerExpirationKeepN::One => "1".into(),
            ContainerExpirationKeepN::Five => "5".into(),
            ContainerExpirationKeepN::Ten => "10".into(),
            ContainerExpirationKeepN::TwentyFive => "25".into(),
            ContainerExpirationKeepN::Fifty => "50".into(),
            ContainerExpirationKeepN::OneHundred => "100".into(),
            ContainerExpirationKeepN::Arbitrary(n) => n.to_string().into(),
        }
    }

    fn as_u64(self) -> u64 {
        match self {
            ContainerExpirationKeepN::One => 1,
            ContainerExpirationKeepN::Five => 5,
            ContainerExpirationKeepN::Ten => 10,
            ContainerExpirationKeepN::TwentyFive => 25,
            ContainerExpirationKeepN::Fifty => 50,
            ContainerExpirationKeepN::OneHundred => 100,
            ContainerExpirationKeepN::Arbitrary(n) => n,
        }
    }
}

impl ParamValue<'static> for ContainerExpirationKeepN {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str()
    }
}

/// How old containers need to be before they are candidates for expiration.
///
/// Note that GitLab only supports a few discrete values for this setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ContainerExpirationOlderThan {
    /// One week old.
    OneWeek,
    /// Two weeks old.
    TwoWeeks,
    /// One month old.
    OneMonth,
    /// Three months old.
    ThreeMonths,
}

impl ContainerExpirationOlderThan {
    /// The variable type query parameter.
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            ContainerExpirationOlderThan::OneWeek => "7d",
            ContainerExpirationOlderThan::TwoWeeks => "14d",
            ContainerExpirationOlderThan::OneMonth => "30d",
            ContainerExpirationOlderThan::ThreeMonths => "90d",
        }
    }
}

impl ParamValue<'static> for ContainerExpirationOlderThan {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// The expiration policies for container images attached to the project.
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct ContainerExpirationPolicy<'a> {
    /// How often the policy should be applied.
    #[builder(default)]
    cadence: Option<ContainerExpirationCadence>,
    /// Whether the policy is enabled or not.
    #[builder(setter(into), default)]
    enabled: Option<bool>,
    /// How many container images to keep.
    #[builder(setter(into), default)]
    keep_n: Option<ContainerExpirationKeepN>,
    /// Only consider containers older than this age.
    #[builder(default)]
    older_than: Option<ContainerExpirationOlderThan>,
    /// Only apply to images with names maching a regular expression.
    ///
    /// See the [Ruby documentation](https://ruby-doc.org/core-2.7.1/Regexp.html) for supported
    /// syntax.
    #[deprecated(note = "use `name_regex_delete` instead")]
    #[builder(setter(into), default)]
    name_regex: Option<Cow<'a, str>>,
    /// Delete images with names matching a regular expression.
    ///
    /// See the [Ruby documentation](https://ruby-doc.org/core-2.7.1/Regexp.html) for supported
    /// syntax.
    #[builder(setter(into), default)]
    name_regex_delete: Option<Cow<'a, str>>,
    /// Keep images with names matching a regular expression.
    ///
    /// See the [Ruby documentation](https://ruby-doc.org/core-2.7.1/Regexp.html) for supported
    /// syntax.
    #[builder(setter(into), default)]
    name_regex_keep: Option<Cow<'a, str>>,
}

impl<'a> ContainerExpirationPolicy<'a> {
    /// Create a builder for the container expiration policy.
    pub fn builder() -> ContainerExpirationPolicyBuilder<'a> {
        ContainerExpirationPolicyBuilder::default()
    }

    pub(crate) fn add_query<'b>(&'b self, params: &mut FormParams<'b>) {
        params
            .push_opt(
                "container_expiration_policy_attributes[cadence]",
                self.cadence,
            )
            .push_opt(
                "container_expiration_policy_attributes[enabled]",
                self.enabled,
            )
            .push_opt(
                "container_expiration_policy_attributes[keep_n]",
                self.keep_n,
            )
            .push_opt(
                "container_expiration_policy_attributes[older_than]",
                self.older_than,
            )
            .push_opt(
                "container_expiration_policy_attributes[name_regex_delete]",
                self.name_regex_delete.as_ref(),
            )
            .push_opt(
                "container_expiration_policy_attributes[name_regex_keep]",
                self.name_regex_keep.as_ref(),
            );

        #[allow(deprecated)]
        {
            params.push_opt(
                "container_expiration_policy_attributes[name_regex]",
                self.name_regex.as_ref(),
            );
        }
    }
}

/// The deploy strategy used when Auto DevOps is enabled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutoDevOpsDeployStrategy {
    /// Continuous deployment.
    Continuous,
    /// Manual deployment.
    Manual,
    /// Interval deployments.
    TimedIncremental,
}

impl AutoDevOpsDeployStrategy {
    /// The variable type query parameter.
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            AutoDevOpsDeployStrategy::Continuous => "continuous",
            AutoDevOpsDeployStrategy::Manual => "manual",
            AutoDevOpsDeployStrategy::TimedIncremental => "timed_incremental",
        }
    }
}

impl ParamValue<'static> for AutoDevOpsDeployStrategy {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// How merge requests should be merged when using the "Merge" button.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MergeMethod {
    /// Always create a merge commit.
    Merge,
    /// Always create a merge commit, but require that the branch be fast-forward capable.
    RebaseMerge,
    /// Only fast-forward merges are allowed.
    FastForward,
}

impl MergeMethod {
    /// The variable type query parameter.
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            MergeMethod::Merge => "merge",
            MergeMethod::RebaseMerge => "rebase_merge",
            MergeMethod::FastForward => "ff",
        }
    }
}

impl ParamValue<'static> for MergeMethod {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// How squashing should be presented in the project.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SquashOption {
    /// Never allow squashing.
    Never,
    /// Always squash.
    Always,
    /// Default to squashing.
    DefaultOn,
    /// Default to not squashing.
    DefaultOff,
}

impl SquashOption {
    /// The variable type query parameter.
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            SquashOption::Never => "never",
            SquashOption::Always => "always",
            SquashOption::DefaultOn => "default_on",
            SquashOption::DefaultOff => "default_off",
        }
    }
}

impl ParamValue<'static> for SquashOption {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// The default Git strategy for CI jobs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BuildGitStrategy {
    /// Clone the reopsitory every time.
    Clone,
    /// Fetch into an existing checkout (will clone if not available).
    #[default]
    Fetch,
    /// Do not update the repository at all.
    None,
}

impl BuildGitStrategy {
    /// The variable type query parameter.
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            BuildGitStrategy::Clone => "clone",
            BuildGitStrategy::Fetch => "fetch",
            BuildGitStrategy::None => "none",
        }
    }
}

impl ParamValue<'static> for BuildGitStrategy {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// A structure to handle the fact that at least one of the name and path is required.
#[derive(Debug, Clone)]
enum ProjectName<'a> {
    /// The name of the new project.
    ///
    /// The `path` is based on the name.
    Name { name: Cow<'a, str> },
    /// The path of the new project.
    ///
    /// The `name` is the path.
    Path { path: Cow<'a, str> },
    /// Provide both the name and path manually.
    NameAndPath {
        name: Cow<'a, str>,
        path: Cow<'a, str>,
    },
}

impl<'a> ProjectName<'a> {
    fn with_name(self, name: Cow<'a, str>) -> Self {
        match self {
            ProjectName::Name { .. } => ProjectName::Name { name },
            ProjectName::NameAndPath { path, .. } | ProjectName::Path { path } => {
                ProjectName::NameAndPath { name, path }
            },
        }
    }

    fn with_path(self, path: Cow<'a, str>) -> Self {
        match self {
            ProjectName::Path { .. } => ProjectName::Path { path },
            ProjectName::NameAndPath { name, .. } | ProjectName::Name { name } => {
                ProjectName::NameAndPath { name, path }
            },
        }
    }
}

/// Create a new project on an instance.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct CreateProject<'a> {
    /// The name and/or path of the project.
    #[builder(private)]
    name_and_path: ProjectName<'a>,
    /// The namespace of the new project.
    ///
    /// By default, the project is created in the API caller's namespace.
    #[builder(default)]
    namespace_id: Option<u64>,
    /// The default branch of the new project.
    ///
    /// Defaults to `master`.
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
    /// The merge method to use for the project.
    #[builder(default)]
    merge_method: Option<MergeMethod>,
    /// Whether merge pipelines are enabled.
    #[builder(default)]
    merge_pipelines_enabled: Option<bool>,
    /// Whether merge trains are enabled.
    #[builder(default)]
    merge_trains_enabled: Option<bool>,
    /// The squash option for the project.
    #[builder(default)]
    squash_option: Option<SquashOption>,
    /// Whether issues referenced on the default branch should be closed or not.
    #[builder(default)]
    autoclose_referenced_issues: Option<bool>,
    /// Whether to enabled the "Remove source branch" option in new merge requests by default or
    /// not.
    #[builder(default)]
    remove_source_branch_after_merge: Option<bool>,
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
    /// Whether to show a link to create or view a merge request when pushing a branch from the
    /// command line or not.
    #[builder(default)]
    printing_merge_request_link_enabled: Option<bool>,
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
    /// Whether mirror updates trigger CI builds ir not.
    #[builder(default)]
    mirror_trigger_builds: Option<bool>,
    /// Initialize the project with a readme.
    #[builder(default)]
    initialize_with_readme: Option<bool>,
    /// The name of a template project to use.
    #[builder(setter(into), default)]
    template_name: Option<Cow<'a, str>>,
    /// The ID of the template project to use.
    #[builder(default)]
    template_project_id: Option<u64>,
    /// Whether to use a custom instance or group template.
    #[builder(default)]
    use_custom_template: Option<bool>,
    /// Whether the template project should come from the group or the instance.
    #[builder(setter(name = "_group_with_project_templates_id"), default, private)]
    group_with_project_templates_id: Option<u64>,
    /// Whether the package repository is enabled or not.
    #[builder(default)]
    packages_enabled: Option<bool>,

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

impl<'a> CreateProject<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> CreateProjectBuilder<'a> {
        CreateProjectBuilder::default()
    }
}

impl<'a> CreateProjectBuilder<'a> {
    /// Set the name of the project.
    ///
    /// If not set, it will default to the value of `path`.
    pub fn name<N>(&mut self, name: N) -> &mut Self
    where
        N: Into<Cow<'a, str>>,
    {
        let name = name.into();
        self.name_and_path = Some(if let Some(name_and_path) = self.name_and_path.take() {
            name_and_path.with_name(name)
        } else {
            ProjectName::Name { name }
        });
        self
    }

    /// Set the path of the project.
    ///
    /// If not set, it will default to the value of `name` after processing to make it a valid
    /// path.
    pub fn path<P>(&mut self, path: P) -> &mut Self
    where
        P: Into<Cow<'a, str>>,
    {
        let path = path.into();
        self.name_and_path = Some(if let Some(name_and_path) = self.name_and_path.take() {
            name_and_path.with_path(path)
        } else {
            ProjectName::Path { path }
        });
        self
    }

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

    /// Add a optic.
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

    /// Whether the template project should come from the group or the instance.
    ///
    /// Note that setting this also sets `use_custom_template` to `true` automatically.
    pub fn group_with_project_templates_id(&mut self, id: u64) -> &mut Self {
        self.group_with_project_templates_id = Some(Some(id));
        self.use_custom_template(true);
        self
    }
}

impl<'a> Endpoint for CreateProject<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "projects".into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        match &self.name_and_path {
            ProjectName::Name { name } => {
                params.push("name", name);
            },
            ProjectName::Path { path } => {
                params.push("path", path);
            },
            ProjectName::NameAndPath { name, path } => {
                params.push("name", name).push("path", path);
            },
        }

        params
            .push_opt("namespace_id", self.namespace_id)
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
            .push_opt("merge_method", self.merge_method)
            .push_opt("merge_pipelines_enabled", self.merge_pipelines_enabled)
            .push_opt("merge_trains_enabled", self.merge_trains_enabled)
            .push_opt("squash_option", self.squash_option)
            .push_opt(
                "autoclose_referenced_issues",
                self.autoclose_referenced_issues,
            )
            .push_opt(
                "remove_source_branch_after_merge",
                self.remove_source_branch_after_merge,
            )
            .push_opt("lfs_enabled", self.lfs_enabled)
            .push_opt("request_access_enabled", self.request_access_enabled)
            .extend(self.tag_list.iter().map(|value| ("tag_list[]", value)))
            .extend(self.topics.iter().map(|value| ("topics[]", value)))
            .push_opt(
                "printing_merge_request_link_enabled",
                self.printing_merge_request_link_enabled,
            )
            .push_opt("build_git_strategy", self.build_git_strategy)
            .push_opt("build_timeout", self.build_timeout)
            .push_opt(
                "auto_cancel_pending_pipelines",
                self.auto_cancel_pending_pipelines,
            )
            .push_opt("build_coverage_regex", self.build_coverage_regex.as_ref())
            .push_opt("ci_config_path", self.ci_config_path.as_ref())
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
            .push_opt("mirror_trigger_builds", self.mirror_trigger_builds)
            .push_opt("initialize_with_readme", self.initialize_with_readme)
            .push_opt("template_name", self.template_name.as_ref())
            .push_opt("template_project_id", self.template_project_id)
            .push_opt("use_custom_template", self.use_custom_template)
            .push_opt(
                "group_with_project_templates_id",
                self.group_with_project_templates_id,
            )
            .push_opt("packages_enabled", self.packages_enabled);

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
        CreateProject, CreateProjectBuilderError, FeatureAccessLevel, FeatureAccessLevelPublic,
        MergeMethod, SquashOption,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn feature_access_level_as_str() {
        let items = &[
            (FeatureAccessLevel::Disabled, "disabled"),
            (FeatureAccessLevel::Private, "private"),
            (FeatureAccessLevel::Enabled, "enabled"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn feature_access_level_public_as_str() {
        let items = &[
            (FeatureAccessLevelPublic::Disabled, "disabled"),
            (FeatureAccessLevelPublic::Private, "private"),
            (FeatureAccessLevelPublic::Enabled, "enabled"),
            (FeatureAccessLevelPublic::Public, "public"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn container_expiration_cadence_as_str() {
        let items = &[
            (ContainerExpirationCadence::OneDay, "1d"),
            (ContainerExpirationCadence::OneWeek, "7d"),
            (ContainerExpirationCadence::TwoWeeks, "14d"),
            (ContainerExpirationCadence::OneMonth, "1month"),
            (ContainerExpirationCadence::ThreeMonths, "3month"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn container_expiration_keep_n_ordering() {
        let items = &[
            ContainerExpirationKeepN::One,
            ContainerExpirationKeepN::Five,
            ContainerExpirationKeepN::Ten,
            ContainerExpirationKeepN::Arbitrary(11),
            ContainerExpirationKeepN::TwentyFive,
            30.into(),
            ContainerExpirationKeepN::Fifty,
            ContainerExpirationKeepN::OneHundred,
        ];

        let mut last = None;
        for item in items {
            if let Some(prev) = last {
                assert!(prev < item);
            }
            last = Some(item);
        }
    }

    #[test]
    fn container_expiration_keep_n_as_str() {
        let items = &[
            (ContainerExpirationKeepN::One, "1"),
            (ContainerExpirationKeepN::Five, "5"),
            (ContainerExpirationKeepN::Ten, "10"),
            (ContainerExpirationKeepN::Arbitrary(11), "11"),
            (ContainerExpirationKeepN::TwentyFive, "25"),
            (30.into(), "30"),
            (ContainerExpirationKeepN::Fifty, "50"),
            (ContainerExpirationKeepN::OneHundred, "100"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn container_expiration_older_than_ordering() {
        let items = &[
            ContainerExpirationOlderThan::OneWeek,
            ContainerExpirationOlderThan::TwoWeeks,
            ContainerExpirationOlderThan::OneMonth,
            ContainerExpirationOlderThan::ThreeMonths,
        ];

        let mut last = None;
        for item in items {
            if let Some(prev) = last {
                assert!(prev < item);
            }
            last = Some(item);
        }
    }

    #[test]
    fn container_expiration_older_than_as_str() {
        let items = &[
            (ContainerExpirationOlderThan::OneWeek, "7d"),
            (ContainerExpirationOlderThan::TwoWeeks, "14d"),
            (ContainerExpirationOlderThan::OneMonth, "30d"),
            (ContainerExpirationOlderThan::ThreeMonths, "90d"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn container_expiration_policy_default() {
        ContainerExpirationPolicy::builder().build().unwrap();
    }

    #[test]
    fn auto_dev_ops_deploy_strategy_as_str() {
        let items = &[
            (AutoDevOpsDeployStrategy::Continuous, "continuous"),
            (AutoDevOpsDeployStrategy::Manual, "manual"),
            (
                AutoDevOpsDeployStrategy::TimedIncremental,
                "timed_incremental",
            ),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn merge_method_as_str() {
        let items = &[
            (MergeMethod::Merge, "merge"),
            (MergeMethod::RebaseMerge, "rebase_merge"),
            (MergeMethod::FastForward, "ff"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn squash_option_as_str() {
        let items = &[
            (SquashOption::Never, "never"),
            (SquashOption::Always, "always"),
            (SquashOption::DefaultOn, "default_on"),
            (SquashOption::DefaultOff, "default_off"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn build_git_strategy_default() {
        assert_eq!(BuildGitStrategy::default(), BuildGitStrategy::Fetch);
    }

    #[test]
    fn build_git_strategy_as_str() {
        let items = &[
            (BuildGitStrategy::Clone, "clone"),
            (BuildGitStrategy::Fetch, "fetch"),
            (BuildGitStrategy::None, "none"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn name_and_path_is_needed() {
        let err = CreateProject::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, CreateProjectBuilderError, "name_and_path");
    }

    #[test]
    fn name_is_sufficient() {
        CreateProject::builder().name("name").build().unwrap();
    }

    #[test]
    fn path_is_sufficient() {
        CreateProject::builder().path("path").build().unwrap();
    }

    #[test]
    fn endpoint_name() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str("name=name")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder().name("name").build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_path() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str("path=path")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder().path("path").build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_name_and_path() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&path=path"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .path("path")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_path_and_name() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&path=path"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .path("path")
            .name("name")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_namespace_id() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&namespace_id=1"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .namespace_id(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_default_branch() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&default_branch=master"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .default_branch("master")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_description() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&description=description"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .description("description")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_issues_access_level() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&issues_access_level=enabled"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .issues_access_level(FeatureAccessLevel::Enabled)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_repository_access_level() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&repository_access_level=disabled"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .repository_access_level(FeatureAccessLevel::Disabled)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_container_registry_access_level() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "name=name",
                "&container_registry_access_level=disabled",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .container_registry_access_level(FeatureAccessLevel::Disabled)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_merge_requests_access_level() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&merge_requests_access_level=private"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .merge_requests_access_level(FeatureAccessLevel::Private)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_forking_access_level() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&forking_access_level=enabled"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .forking_access_level(FeatureAccessLevel::Enabled)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_builds_access_level() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&builds_access_level=enabled"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .builds_access_level(FeatureAccessLevel::Enabled)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_wiki_access_level() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&wiki_access_level=disabled"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .wiki_access_level(FeatureAccessLevel::Disabled)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_snippets_access_level() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&snippets_access_level=disabled"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .snippets_access_level(FeatureAccessLevel::Disabled)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_pages_access_level() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&pages_access_level=public"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .pages_access_level(FeatureAccessLevelPublic::Public)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_operations_access_level() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&operations_access_level=enabled"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .operations_access_level(FeatureAccessLevel::Enabled)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_requirements_access_level() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&requirements_access_level=enabled"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .requirements_access_level(FeatureAccessLevel::Enabled)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_analytics_access_level() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&analytics_access_level=private"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .analytics_access_level(FeatureAccessLevel::Private)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_security_and_compliance_access_level() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "name=name",
                "&security_and_compliance_access_level=private",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .security_and_compliance_access_level(FeatureAccessLevel::Private)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_emails_disabled() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&emails_disabled=true"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .emails_disabled(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_show_default_award_emojis() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&show_default_award_emojis=false"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .show_default_award_emojis(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_restrict_user_defined_variables() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "name=name",
                "&restrict_user_defined_variables=false",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .restrict_user_defined_variables(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_resolve_outdated_diff_discussions() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "name=name",
                "&resolve_outdated_diff_discussions=false",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .resolve_outdated_diff_discussions(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    #[allow(deprecated)]
    fn endpoint_container_registry_enabled() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&container_registry_enabled=true"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .container_registry_enabled(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_container_expiration_policy_attributes_cadence() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "name=name",
                "&container_expiration_policy_attributes%5Bcadence%5D=7d",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
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
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "name=name",
                "&container_expiration_policy_attributes%5Benabled%5D=true",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
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
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "name=name",
                "&container_expiration_policy_attributes%5Bkeep_n%5D=5",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
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
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "name=name",
                "&container_expiration_policy_attributes%5Bolder_than%5D=7d",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
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
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "name=name",
                "&container_expiration_policy_attributes%5Bname_regex%5D=%3Alatest",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
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
    fn endpoint_container_expiration_policy_attributes_name_regex_delete() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "name=name",
                "&container_expiration_policy_attributes%5Bname_regex_delete%5D=%3Alatest",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .container_expiration_policy_attributes(
                ContainerExpirationPolicy::builder()
                    .name_regex_delete(":latest")
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_container_expiration_policy_attributes_name_regex_keep() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "name=name",
                "&container_expiration_policy_attributes%5Bname_regex_keep%5D=%3Alatest",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .container_expiration_policy_attributes(
                ContainerExpirationPolicy::builder()
                    .name_regex_keep(":latest")
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
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "name=name",
                "&container_expiration_policy_attributes%5Bcadence%5D=7d",
                "&container_expiration_policy_attributes%5Benabled%5D=true",
                "&container_expiration_policy_attributes%5Bkeep_n%5D=5",
                "&container_expiration_policy_attributes%5Bolder_than%5D=7d",
                "&container_expiration_policy_attributes%5Bname_regex%5D=%3Alatest",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
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
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&shared_runners_enabled=false"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .shared_runners_enabled(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_visibility() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&visibility=public"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .visibility(VisibilityLevel::Public)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_import_url() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "name=name",
                "&import_url=https%3A%2F%2Ftest.invalid%2Fpath%3Fsome%3Dfoo",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .import_url("https://test.invalid/path?some=foo")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_public_builds() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&public_builds=true"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .public_builds(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_only_allow_merge_if_pipeline_succeeds() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "name=name",
                "&only_allow_merge_if_pipeline_succeeds=false",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .only_allow_merge_if_pipeline_succeeds(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_allow_merge_on_skipped_pipeline() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "name=name",
                "&allow_merge_on_skipped_pipeline=false",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .allow_merge_on_skipped_pipeline(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_only_allow_merge_if_all_discussions_are_resolved() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "name=name",
                "&only_allow_merge_if_all_discussions_are_resolved=true",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .only_allow_merge_if_all_discussions_are_resolved(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_merge_method() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&merge_method=ff"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .merge_method(MergeMethod::FastForward)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_merge_pipelines_enabled() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&merge_pipelines_enabled=true"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .merge_pipelines_enabled(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_merge_trains_enabled() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&merge_trains_enabled=true"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .merge_trains_enabled(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_squash_option() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&squash_option=never"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .squash_option(SquashOption::Never)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_autoclose_referenced_issues() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&autoclose_referenced_issues=true"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .autoclose_referenced_issues(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_remove_source_branch_after_merge() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "name=name",
                "&remove_source_branch_after_merge=true",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .remove_source_branch_after_merge(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_lfs_enabled() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&lfs_enabled=false"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .lfs_enabled(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_request_access_enabled() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&request_access_enabled=true"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .request_access_enabled(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    #[allow(deprecated)]
    fn endpoint_tag_list() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "name=name",
                "&tag_list%5B%5D=tag1",
                "&tag_list%5B%5D=tag2",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .tag("tag1")
            .tags(["tag1", "tag2"].iter().copied())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_topics() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "name=name",
                "&topics%5B%5D=topic1",
                "&topics%5B%5D=topic2",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .topic("topic1")
            .topics(["topic1", "topic2"].iter().copied())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_printing_merge_request_link_enabled() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "name=name",
                "&printing_merge_request_link_enabled=false",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .printing_merge_request_link_enabled(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_build_git_strategy() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&build_git_strategy=fetch"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .build_git_strategy(BuildGitStrategy::Fetch)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_build_timeout() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&build_timeout=1"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .build_timeout(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_auto_cancel_pending_pipelines() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "name=name",
                "&auto_cancel_pending_pipelines=enabled",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .auto_cancel_pending_pipelines(EnableState::Enabled)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_build_coverage_regex() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&build_coverage_regex=%5Cd%25"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .build_coverage_regex("\\d%")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_ci_config_path() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&ci_config_path=.gitlab-ci.yaml"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .ci_config_path(".gitlab-ci.yaml")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_auto_devops_enabled() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&auto_devops_enabled=false"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .auto_devops_enabled(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_auto_devops_deploy_strategy() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&auto_devops_deploy_strategy=manual"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .auto_devops_deploy_strategy(AutoDevOpsDeployStrategy::Manual)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_repository_storage() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&repository_storage=shard1"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .repository_storage("shard1")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_approvals_before_merge() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&approvals_before_merge=2"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .approvals_before_merge(2)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_external_authorization_classification_label() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "name=name",
                "&external_authorization_classification_label=external",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .external_authorization_classification_label("external")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_mirror() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&mirror=true"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .mirror(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_mirror_trigger_builds() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&mirror_trigger_builds=false"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .mirror_trigger_builds(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_initialize_with_readme() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&initialize_with_readme=false"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .initialize_with_readme(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_template_name() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&template_name=template"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .template_name("template")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_template_project_id() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&template_project_id=1"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .template_project_id(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_use_custom_template() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&use_custom_template=false"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .use_custom_template(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_group_with_project_templates_id() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "name=name",
                "&use_custom_template=true",
                "&group_with_project_templates_id=1",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .group_with_project_templates_id(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_packages_enabled() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&packages_enabled=false"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .packages_enabled(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    #[allow(deprecated)]
    fn endpoint_issues_enabled() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&issues_enabled=true"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .issues_enabled(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    #[allow(deprecated)]
    fn endpoint_merge_requests_enabled() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&merge_requests_enabled=true"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .merge_requests_enabled(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    #[allow(deprecated)]
    fn endpoint_jobs_enabled() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&jobs_enabled=true"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .jobs_enabled(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    #[allow(deprecated)]
    fn endpoint_wiki_enabled() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&wiki_enabled=false"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .wiki_enabled(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    #[allow(deprecated)]
    fn endpoint_snippets_enabled() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&snippets_enabled=false"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProject::builder()
            .name("name")
            .snippets_enabled(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
