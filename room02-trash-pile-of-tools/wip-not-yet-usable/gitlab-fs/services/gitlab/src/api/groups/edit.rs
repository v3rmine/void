// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::{NameOrId, VisibilityLevel};
use crate::api::endpoint_prelude::*;
use crate::api::groups::{
    BranchProtection, GroupProjectCreationAccessLevel, SharedRunnersMinutesLimit,
    SubgroupCreationAccessLevel,
};
use crate::api::ParamValue;

/// Access levels for creating a project within a group.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SharedRunnersSetting {
    /// All projects and subgroups can use shared runners.
    Enabled,
    /// Shared runners are not allowed, but subgroups can enable.
    DisabledWithOverride,
    /// Shared runners are not allowed for this group and all subgroups.
    DisableAndUnoverridable,
}

impl SharedRunnersSetting {
    fn as_str(self) -> &'static str {
        match self {
            SharedRunnersSetting::Enabled => "enabled",
            SharedRunnersSetting::DisabledWithOverride => "disabled_with_override",
            SharedRunnersSetting::DisableAndUnoverridable => "disabled_and_unoverridable",
        }
    }
}

impl ParamValue<'static> for SharedRunnersSetting {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Edit an existing group.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct EditGroup<'a> {
    /// The group to edit.
    #[builder(setter(into))]
    group: NameOrId<'a>,

    /// The name of the group.
    #[builder(setter(into), default)]
    name: Option<Cow<'a, str>>,
    /// The path of the group.
    #[builder(setter(into), default)]
    path: Option<Cow<'a, str>>,
    /// A short description for the group.
    #[builder(setter(into), default)]
    description: Option<Cow<'a, str>>,
    /// Prevent adding members directly to projects within the group.
    #[builder(default)]
    membership_lock: Option<bool>,
    /// The visibility of the group.
    #[builder(default)]
    visibility: Option<VisibilityLevel>,
    /// Prevent sharing a project in this group with another group.
    #[builder(default)]
    share_with_group_lock: Option<bool>,
    /// Require two-factor authentication to be a member of this group.
    #[builder(default)]
    require_two_factor_authentication: Option<bool>,
    /// Time (in hours) for users to enable two-factor before enforcing it.
    #[builder(default)]
    two_factor_grace_period: Option<u64>,
    /// The access level to the group that is required to create new projects.
    #[builder(default)]
    project_creation_level: Option<GroupProjectCreationAccessLevel>,
    /// Default to Auto DevOps for new projects in the group.
    #[builder(default)]
    auto_devops_enabled: Option<bool>,
    /// The access level to the group that is required to create subgroups.
    #[builder(default)]
    subgroup_creation_level: Option<SubgroupCreationAccessLevel>,
    /// Disable email notifications from the group.
    #[builder(default)]
    emails_disabled: Option<bool>,
    // TODO: Figure out how to actually use this.
    // avatar   mixed   no  Image file for avatar of the group
    // avatar: ???,
    /// Disable group-wide mentions.
    #[builder(default)]
    mentions_disabled: Option<bool>,
    /// Disable sharing outside of the group hierarchy.
    ///
    /// Only available on top-level groups.
    #[builder(default)]
    prevent_sharing_groups_outside_hierarchy: Option<bool>,
    /// Whether `git-lfs` is enabled by default for projects within the group.
    #[builder(default)]
    lfs_enabled: Option<bool>,
    /// Whether access to the group may be requested.
    #[builder(default)]
    request_access_enabled: Option<bool>,
    /// The parent group ID (for subgroups).
    #[builder(default)]
    parent_id: Option<u64>,
    /// The default branch protection for projects within the group.
    #[builder(default)]
    default_branch_protection: Option<BranchProtection>,
    /// Shared runner settings for the group.
    #[builder(default)]
    shared_runners_setting: Option<SharedRunnersSetting>,
    /// Pipeline quota (in minutes) for the group on shared runners.
    #[builder(setter(into), default)]
    shared_runners_minutes_limit: Option<SharedRunnersMinutesLimit>,
    /// Pipeline quota excess (in minutes) for the group on shared runners.
    #[builder(default)]
    extra_shared_runners_minutes_limit: Option<u64>,
    /// The project id to load custom file templates from.
    #[builder(default)]
    file_template_project_id: Option<u64>,
    /// When enabled, users cannot fork projects from this group to other namespaces.
    #[builder(default)]
    prevent_forking_outside_group: Option<bool>,
}

impl<'a> EditGroup<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> EditGroupBuilder<'a> {
        EditGroupBuilder::default()
    }
}

impl<'a> Endpoint for EditGroup<'a> {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("groups/{}", self.group).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params
            .push_opt("name", self.name.as_ref())
            .push_opt("path", self.path.as_ref())
            .push_opt("description", self.description.as_ref())
            .push_opt("membership_lock", self.membership_lock)
            .push_opt("visibility", self.visibility)
            .push_opt("share_with_group_lock", self.share_with_group_lock)
            .push_opt(
                "require_two_factor_authentication",
                self.require_two_factor_authentication,
            )
            .push_opt("two_factor_grace_period", self.two_factor_grace_period)
            .push_opt("project_creation_level", self.project_creation_level)
            .push_opt("auto_devops_enabled", self.auto_devops_enabled)
            .push_opt("subgroup_creation_level", self.subgroup_creation_level)
            .push_opt("emails_disabled", self.emails_disabled)
            .push_opt("mentions_disabled", self.mentions_disabled)
            .push_opt(
                "prevent_sharing_groups_outside_hierarchy",
                self.prevent_sharing_groups_outside_hierarchy,
            )
            .push_opt("lfs_enabled", self.lfs_enabled)
            .push_opt("request_access_enabled", self.request_access_enabled)
            .push_opt("parent_id", self.parent_id)
            .push_opt("default_branch_protection", self.default_branch_protection)
            .push_opt("shared_runners_setting", self.shared_runners_setting)
            .push_opt(
                "shared_runners_minutes_limit",
                self.shared_runners_minutes_limit,
            )
            .push_opt(
                "extra_shared_runners_minutes_limit",
                self.extra_shared_runners_minutes_limit,
            )
            .push_opt("file_template_project_id", self.file_template_project_id)
            .push_opt(
                "prevent_forking_outside_group",
                self.prevent_forking_outside_group,
            );

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::common::VisibilityLevel;
    use crate::api::groups::{
        BranchProtection, EditGroup, EditGroupBuilderError, GroupProjectCreationAccessLevel,
        SharedRunnersMinutesLimit, SharedRunnersSetting, SubgroupCreationAccessLevel,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn shared_runners_setting_as_str() {
        let items = &[
            (SharedRunnersSetting::Enabled, "enabled"),
            (
                SharedRunnersSetting::DisabledWithOverride,
                "disabled_with_override",
            ),
            (
                SharedRunnersSetting::DisableAndUnoverridable,
                "disabled_and_unoverridable",
            ),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn project_is_necessary() {
        let err = EditGroup::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, EditGroupBuilderError, "group");
    }

    #[test]
    fn group_is_sufficient() {
        EditGroup::builder().group("group").build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder().group("simple/group").build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_name() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("name=name")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .name("name")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_path() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("path=path")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .path("path")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_description() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("description=description")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .description("description")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_membership_lock() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("membership_lock=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .membership_lock(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_visibility() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("visibility=internal")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .visibility(VisibilityLevel::Internal)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_share_with_group_lock() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("share_with_group_lock=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .share_with_group_lock(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_require_two_factor_authentication() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("require_two_factor_authentication=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .require_two_factor_authentication(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_two_factor_grace_period() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("two_factor_grace_period=1")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .two_factor_grace_period(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_project_creation_level() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("project_creation_level=maintainer")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .project_creation_level(GroupProjectCreationAccessLevel::Maintainer)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_auto_devops_enabled() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("auto_devops_enabled=false")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .auto_devops_enabled(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_subgroup_creation_level() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("subgroup_creation_level=owner")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .subgroup_creation_level(SubgroupCreationAccessLevel::Owner)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_emails_disabled() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("emails_disabled=false")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .emails_disabled(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_mentions_disabled() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("mentions_disabled=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .mentions_disabled(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_prevent_sharing_groups_outside_hierarchy() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("prevent_sharing_groups_outside_hierarchy=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .prevent_sharing_groups_outside_hierarchy(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_lfs_enabled() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("lfs_enabled=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .lfs_enabled(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_request_access_enabled() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("request_access_enabled=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .request_access_enabled(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_parent_id() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("parent_id=1")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .parent_id(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_default_branch_protection() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("default_branch_protection=2")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .default_branch_protection(BranchProtection::Full)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_shared_runners_setting() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("shared_runners_setting=disabled_with_override")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .shared_runners_setting(SharedRunnersSetting::DisabledWithOverride)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_shared_runners_minutes_limit() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("shared_runners_minutes_limit=0")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .shared_runners_minutes_limit(SharedRunnersMinutesLimit::Unlimited)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_shared_runners_minutes_limit_into() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("shared_runners_minutes_limit=1")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .shared_runners_minutes_limit(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_extra_shared_runners_minutes_limit() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("extra_shared_runners_minutes_limit=1")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .extra_shared_runners_minutes_limit(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_file_template_project_id() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("file_template_project_id=1")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .file_template_project_id(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_prevent_forking_outside_group() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("prevent_forking_outside_group=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .prevent_forking_outside_group(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
