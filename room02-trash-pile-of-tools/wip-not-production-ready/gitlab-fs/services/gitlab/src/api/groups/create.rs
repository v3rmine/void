// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::VisibilityLevel;
use crate::api::endpoint_prelude::*;
use crate::api::ParamValue;

/// Access levels for creating a project within a group.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroupProjectCreationAccessLevel {
    /// No new projects may be added to the group.
    NoOne,
    /// Only maintainers may add projects to the group.
    Maintainer,
    /// Developers and maintainers may add projects to the group.
    Developer,
}

impl GroupProjectCreationAccessLevel {
    fn as_str(self) -> &'static str {
        match self {
            GroupProjectCreationAccessLevel::NoOne => "noone",
            GroupProjectCreationAccessLevel::Maintainer => "maintainer",
            GroupProjectCreationAccessLevel::Developer => "developer",
        }
    }
}

impl ParamValue<'static> for GroupProjectCreationAccessLevel {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Access levels for creating a subgroup within a group.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubgroupCreationAccessLevel {
    /// Owners may add new subprojects.
    Owner,
    /// Maintainers may add new subprojects.
    Maintainer,
}

impl SubgroupCreationAccessLevel {
    fn as_str(self) -> &'static str {
        match self {
            SubgroupCreationAccessLevel::Owner => "owner",
            SubgroupCreationAccessLevel::Maintainer => "maintainer",
        }
    }
}

impl ParamValue<'static> for SubgroupCreationAccessLevel {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Branch protection rules for groups.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BranchProtection {
    /// Developers and maintainers may push, force push, and delete branches.
    None,
    /// Developers and maintainers may push branches.
    Partial,
    /// Maintainers may push branches.
    Full,
    /// Developers may accept merge requests; maintainers may push, force push, and accept merge
    /// requests.
    Push,
}

impl BranchProtection {
    fn as_str(self) -> &'static str {
        match self {
            BranchProtection::None => "0",
            BranchProtection::Partial => "1",
            BranchProtection::Full => "2",
            BranchProtection::Push => "3",
        }
    }
}

impl ParamValue<'static> for BranchProtection {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Settings for a group's shared runner minute allocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SharedRunnersMinutesLimit {
    /// Inherit the setting from the parent group or instance.
    Inherit,
    /// Unlimited shared minutes are allowed.
    Unlimited,
    /// A set number of minutes are allowed.
    Minutes(u64),
}

impl SharedRunnersMinutesLimit {
    fn as_str(self) -> Cow<'static, str> {
        match self {
            SharedRunnersMinutesLimit::Inherit => "nil".into(),
            SharedRunnersMinutesLimit::Unlimited => "0".into(),
            SharedRunnersMinutesLimit::Minutes(m) => m.to_string().into(),
        }
    }
}

impl From<u64> for SharedRunnersMinutesLimit {
    fn from(i: u64) -> Self {
        if i == 0 {
            Self::Unlimited
        } else {
            Self::Minutes(i)
        }
    }
}

impl ParamValue<'static> for SharedRunnersMinutesLimit {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str()
    }
}

/// Create a new group on an instance.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct CreateGroup<'a> {
    /// The name of the group.
    #[builder(setter(into))]
    name: Cow<'a, str>,
    /// The path of the group.
    #[builder(setter(into))]
    path: Cow<'a, str>,

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
    /// Pipeline quota (in minutes) for the group on shared runners.
    #[builder(setter(into), default)]
    shared_runners_minutes_limit: Option<SharedRunnersMinutesLimit>,
    /// Pipeline quota excess (in minutes) for the group on shared runners.
    #[builder(default)]
    extra_shared_runners_minutes_limit: Option<u64>,
}

impl<'a> CreateGroup<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> CreateGroupBuilder<'a> {
        CreateGroupBuilder::default()
    }
}

impl<'a> Endpoint for CreateGroup<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "groups".into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params
            .push("name", &self.name)
            .push("path", &self.path)
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
            .push_opt("lfs_enabled", self.lfs_enabled)
            .push_opt("request_access_enabled", self.request_access_enabled)
            .push_opt("parent_id", self.parent_id)
            .push_opt("default_branch_protection", self.default_branch_protection)
            .push_opt(
                "shared_runners_minutes_limit",
                self.shared_runners_minutes_limit,
            )
            .push_opt(
                "extra_shared_runners_minutes_limit",
                self.extra_shared_runners_minutes_limit,
            );

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::common::VisibilityLevel;
    use crate::api::groups::{
        BranchProtection, CreateGroup, CreateGroupBuilderError, GroupProjectCreationAccessLevel,
        SharedRunnersMinutesLimit, SubgroupCreationAccessLevel,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn group_project_creation_access_level_as_str() {
        let items = &[
            (GroupProjectCreationAccessLevel::NoOne, "noone"),
            (GroupProjectCreationAccessLevel::Maintainer, "maintainer"),
            (GroupProjectCreationAccessLevel::Developer, "developer"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn subgroup_creation_access_level_as_str() {
        let items = &[
            (SubgroupCreationAccessLevel::Owner, "owner"),
            (SubgroupCreationAccessLevel::Maintainer, "maintainer"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn branch_protection_as_str() {
        let items = &[
            (BranchProtection::None, "0"),
            (BranchProtection::Partial, "1"),
            (BranchProtection::Full, "2"),
            (BranchProtection::Push, "3"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn shared_runners_minutes_limit_as_str() {
        let items = &[
            (SharedRunnersMinutesLimit::Inherit, "nil"),
            (SharedRunnersMinutesLimit::Unlimited, "0"),
            (SharedRunnersMinutesLimit::Minutes(10), "10"),
            (SharedRunnersMinutesLimit::Minutes(24), "24"),
            (15.into(), "15"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn name_and_path_are_necessary() {
        let err = CreateGroup::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, CreateGroupBuilderError, "name");
    }

    #[test]
    fn name_is_necessary() {
        let err = CreateGroup::builder().path("path").build().unwrap_err();
        crate::test::assert_missing_field!(err, CreateGroupBuilderError, "name");
    }

    #[test]
    fn path_is_necessary() {
        let err = CreateGroup::builder().name("name").build().unwrap_err();
        crate::test::assert_missing_field!(err, CreateGroupBuilderError, "path");
    }

    #[test]
    fn name_and_path_are_sufficient() {
        CreateGroup::builder()
            .name("name")
            .path("path")
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("groups")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&path=path"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateGroup::builder()
            .name("name")
            .path("path")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_description() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("groups")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "name=name",
                "&path=path",
                "&description=description",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateGroup::builder()
            .name("name")
            .path("path")
            .description("description")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_membership_lock() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("groups")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&path=path", "&membership_lock=true"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateGroup::builder()
            .name("name")
            .path("path")
            .membership_lock(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_visibility() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("groups")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&path=path", "&visibility=internal"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateGroup::builder()
            .name("name")
            .path("path")
            .visibility(VisibilityLevel::Internal)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_share_with_group_lock() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("groups")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "name=name",
                "&path=path",
                "&share_with_group_lock=true",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateGroup::builder()
            .name("name")
            .path("path")
            .share_with_group_lock(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_require_two_factor_authentication() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("groups")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "name=name",
                "&path=path",
                "&require_two_factor_authentication=true",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateGroup::builder()
            .name("name")
            .path("path")
            .require_two_factor_authentication(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_two_factor_grace_period() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("groups")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "name=name",
                "&path=path",
                "&two_factor_grace_period=1",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateGroup::builder()
            .name("name")
            .path("path")
            .two_factor_grace_period(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_project_creation_level() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("groups")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "name=name",
                "&path=path",
                "&project_creation_level=maintainer",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateGroup::builder()
            .name("name")
            .path("path")
            .project_creation_level(GroupProjectCreationAccessLevel::Maintainer)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_auto_devops_enabled() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("groups")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "name=name",
                "&path=path",
                "&auto_devops_enabled=false",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateGroup::builder()
            .name("name")
            .path("path")
            .auto_devops_enabled(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_subgroup_creation_level() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("groups")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "name=name",
                "&path=path",
                "&subgroup_creation_level=owner",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateGroup::builder()
            .name("name")
            .path("path")
            .subgroup_creation_level(SubgroupCreationAccessLevel::Owner)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_emails_disabled() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("groups")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&path=path", "&emails_disabled=false"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateGroup::builder()
            .name("name")
            .path("path")
            .emails_disabled(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_mentions_disabled() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("groups")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "name=name",
                "&path=path",
                "&mentions_disabled=true",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateGroup::builder()
            .name("name")
            .path("path")
            .mentions_disabled(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_lfs_enabled() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("groups")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&path=path", "&lfs_enabled=true"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateGroup::builder()
            .name("name")
            .path("path")
            .lfs_enabled(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_request_access_enabled() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("groups")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "name=name",
                "&path=path",
                "&request_access_enabled=true",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateGroup::builder()
            .name("name")
            .path("path")
            .request_access_enabled(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_parent_id() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("groups")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&path=path", "&parent_id=1"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateGroup::builder()
            .name("name")
            .path("path")
            .parent_id(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_default_branch_protection() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("groups")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "name=name",
                "&path=path",
                "&default_branch_protection=2",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateGroup::builder()
            .name("name")
            .path("path")
            .default_branch_protection(BranchProtection::Full)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_shared_runners_minutes_limit() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("groups")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "name=name",
                "&path=path",
                "&shared_runners_minutes_limit=0",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateGroup::builder()
            .name("name")
            .path("path")
            .shared_runners_minutes_limit(SharedRunnersMinutesLimit::Unlimited)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_shared_runners_minutes_limit_into() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("groups")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "name=name",
                "&path=path",
                "&shared_runners_minutes_limit=1",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateGroup::builder()
            .name("name")
            .path("path")
            .shared_runners_minutes_limit(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_extra_shared_runners_minutes_limit() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("groups")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "name=name",
                "&path=path",
                "&extra_shared_runners_minutes_limit=1",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateGroup::builder()
            .name("name")
            .path("path")
            .extra_shared_runners_minutes_limit(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
