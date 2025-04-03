// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::cmp;
use std::collections::BTreeSet;

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

#[deprecated(note = "use `api/common/ProtectedAccessLevel` instead")]
pub use crate::api::common::ProtectedAccessLevel;

/// Granular protected access controls for branches.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtectedAccess {
    /// Give a specific user access.
    User(u64),
    /// Give a group access.
    Group(u64),
    /// Give access to anyone with at least an access level.
    Level(ProtectedAccessLevel),
}

impl ProtectedAccess {
    pub(crate) fn add_query(self, name: &str, params: &mut FormParams<'_>) {
        match self {
            ProtectedAccess::User(user) => {
                params.push(format!("{}[][user_id]", name), user);
            },
            ProtectedAccess::Group(group) => {
                params.push(format!("{}[][group_id]", name), group);
            },
            ProtectedAccess::Level(level) => {
                params.push(format!("{}[][access_level]", name), level);
            },
        }
    }
}

impl PartialOrd for ProtectedAccess {
    fn partial_cmp(&self, rhs: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Ord for ProtectedAccess {
    fn cmp(&self, rhs: &Self) -> cmp::Ordering {
        match (self, rhs) {
            (Self::User(l), Self::User(r)) | (Self::Group(l), Self::Group(r)) => l.cmp(r),
            (Self::Group(_), Self::User(_)) => cmp::Ordering::Greater,
            (Self::Group(_) | Self::User(_), _) => cmp::Ordering::Less,
            (Self::Level(l), Self::Level(r)) => l.cmp(r),
            (Self::Level(_), _) => cmp::Ordering::Greater,
        }
    }
}

impl From<ProtectedAccessLevel> for ProtectedAccess {
    fn from(access: ProtectedAccessLevel) -> Self {
        ProtectedAccess::Level(access)
    }
}

/// Protect a branch or set of branches on a project.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct ProtectBranch<'a> {
    /// The project to protect a branch within.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The name or glob of the branch to protect.
    #[builder(setter(into))]
    name: Cow<'a, str>,
    /// The minimum access level required to push to the branch.
    #[builder(default)]
    push_access_level: Option<ProtectedAccessLevel>,
    /// The minimum access level required to merge into the branch.
    #[builder(default)]
    merge_access_level: Option<ProtectedAccessLevel>,
    /// The minimum access level required to unprotect the branch.
    #[builder(default)]
    unprotect_access_level: Option<ProtectedAccessLevel>,
    /// Allow all users with push access to force push.
    #[builder(default)]
    allow_force_push: Option<bool>,
    /// A discrete set of accesses allowed to push to the branch.
    #[builder(setter(name = "_allowed_to_push"), default, private)]
    allowed_to_push: BTreeSet<ProtectedAccess>,
    /// A discrete set of accesses allowed to merge into the branch.
    #[builder(setter(name = "_allowed_to_merge"), default, private)]
    allowed_to_merge: BTreeSet<ProtectedAccess>,
    /// A discrete set of accesses allowed to unprotect the branch.
    #[builder(setter(name = "_allowed_to_unprotect"), default, private)]
    allowed_to_unprotect: BTreeSet<ProtectedAccess>,
    /// Whether code owner approval is required to merge.
    #[builder(default)]
    code_owner_approval_required: Option<bool>,
}

impl<'a> ProtectBranch<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> ProtectBranchBuilder<'a> {
        ProtectBranchBuilder::default()
    }
}

impl<'a> ProtectBranchBuilder<'a> {
    /// Add access to push to the branch.
    pub fn allowed_to_push(&mut self, access: ProtectedAccess) -> &mut Self {
        self.allowed_to_push
            .get_or_insert_with(BTreeSet::new)
            .insert(access);
        self
    }

    /// Add access to merge into the branch.
    pub fn allowed_to_merge(&mut self, access: ProtectedAccess) -> &mut Self {
        self.allowed_to_merge
            .get_or_insert_with(BTreeSet::new)
            .insert(access);
        self
    }

    /// Add access to unprotect the branch.
    pub fn allowed_to_unprotect(&mut self, access: ProtectedAccess) -> &mut Self {
        self.allowed_to_unprotect
            .get_or_insert_with(BTreeSet::new)
            .insert(access);
        self
    }
}

impl<'a> Endpoint for ProtectBranch<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/protected_branches", self.project).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params
            .push("name", &self.name)
            .push_opt("push_access_level", self.push_access_level)
            .push_opt("merge_access_level", self.merge_access_level)
            .push_opt("unprotect_access_level", self.unprotect_access_level)
            .push_opt("allow_force_push", self.allow_force_push)
            .push_opt(
                "code_owner_approval_required",
                self.code_owner_approval_required,
            );

        self.allowed_to_push
            .iter()
            .for_each(|value| value.add_query("allowed_to_push", &mut params));
        self.allowed_to_merge
            .iter()
            .for_each(|value| value.add_query("allowed_to_merge", &mut params));
        self.allowed_to_unprotect
            .iter()
            .for_each(|value| value.add_query("allowed_to_unprotect", &mut params));

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use std::cmp;

    use http::Method;

    use crate::api::common::ProtectedAccessLevel;
    use crate::api::projects::protected_branches::{
        ProtectBranch, ProtectBranchBuilderError, ProtectedAccess,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn protected_access_ord() {
        let items = &[
            ProtectedAccess::User(1),
            ProtectedAccess::User(2),
            ProtectedAccess::Group(1),
            ProtectedAccess::Group(2),
            ProtectedAccessLevel::Developer.into(),
            ProtectedAccessLevel::Maintainer.into(),
            ProtectedAccessLevel::Admin.into(),
            ProtectedAccessLevel::NoAccess.into(),
        ];

        for i in items {
            // We are asserting that `Eq` is implemented.
            #[allow(clippy::eq_op)]
            {
                assert_eq!(*i, *i);
            }
            assert_eq!(i.cmp(i), cmp::Ordering::Equal);
            assert_eq!(i.partial_cmp(i).unwrap(), cmp::Ordering::Equal);

            let mut expect = cmp::Ordering::Greater;
            for j in items {
                let is_same = i == j;
                if is_same {
                    expect = cmp::Ordering::Equal;
                }
                assert_eq!(i.cmp(j), expect);
                assert_eq!(i.partial_cmp(j).unwrap(), expect);
                if is_same {
                    expect = cmp::Ordering::Less;
                }
            }

            let mut expect = cmp::Ordering::Less;
            for j in items.iter().rev() {
                let is_same = i == j;
                if is_same {
                    expect = cmp::Ordering::Equal;
                }
                assert_eq!(i.cmp(j), expect);
                assert_eq!(i.partial_cmp(j).unwrap(), expect);
                if is_same {
                    expect = cmp::Ordering::Greater;
                }
            }
        }
    }

    #[test]
    fn project_and_name_are_needed() {
        let err = ProtectBranch::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, ProtectBranchBuilderError, "project");
    }

    #[test]
    fn project_is_required() {
        let err = ProtectBranch::builder().name("master").build().unwrap_err();
        crate::test::assert_missing_field!(err, ProtectBranchBuilderError, "project");
    }

    #[test]
    fn name_is_required() {
        let err = ProtectBranch::builder().project(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, ProtectBranchBuilderError, "name");
    }

    #[test]
    fn project_and_name_are_sufficient() {
        ProtectBranch::builder()
            .project(1)
            .name("master")
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/protected_branches")
            .content_type("application/x-www-form-urlencoded")
            .body_str("name=master")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProtectBranch::builder()
            .project("simple/project")
            .name("master")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_push_access_level() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/protected_branches")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=master", "&push_access_level=40"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProtectBranch::builder()
            .project("simple/project")
            .name("master")
            .push_access_level(ProtectedAccessLevel::Maintainer)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_merge_access_level() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/protected_branches")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=master", "&merge_access_level=40"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProtectBranch::builder()
            .project("simple/project")
            .name("master")
            .merge_access_level(ProtectedAccessLevel::Maintainer)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_unprotect_access_level() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/protected_branches")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=master", "&unprotect_access_level=40"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProtectBranch::builder()
            .project("simple/project")
            .name("master")
            .unprotect_access_level(ProtectedAccessLevel::Maintainer)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_allow_force_push() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/protected_branches")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=master", "&allow_force_push=true"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProtectBranch::builder()
            .project("simple/project")
            .name("master")
            .allow_force_push(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_allowed_to_push_user() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/protected_branches")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "name=master",
                "&allowed_to_push%5B%5D%5Buser_id%5D=1",
                "&allowed_to_push%5B%5D%5Bgroup_id%5D=1",
                "&allowed_to_push%5B%5D%5Baccess_level%5D=30",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProtectBranch::builder()
            .project("simple/project")
            .name("master")
            .allowed_to_push(ProtectedAccess::User(1))
            .allowed_to_push(ProtectedAccess::Group(1))
            .allowed_to_push(ProtectedAccess::Level(ProtectedAccessLevel::Developer))
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_allowed_to_merge_user() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/protected_branches")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "name=master",
                "&allowed_to_merge%5B%5D%5Buser_id%5D=1",
                "&allowed_to_merge%5B%5D%5Bgroup_id%5D=1",
                "&allowed_to_merge%5B%5D%5Baccess_level%5D=30",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProtectBranch::builder()
            .project("simple/project")
            .name("master")
            .allowed_to_merge(ProtectedAccess::User(1))
            .allowed_to_merge(ProtectedAccess::Group(1))
            .allowed_to_merge(ProtectedAccess::Level(ProtectedAccessLevel::Developer))
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_allowed_to_unprotect_user() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/protected_branches")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "name=master",
                "&allowed_to_unprotect%5B%5D%5Buser_id%5D=1",
                "&allowed_to_unprotect%5B%5D%5Bgroup_id%5D=1",
                "&allowed_to_unprotect%5B%5D%5Baccess_level%5D=30",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProtectBranch::builder()
            .project("simple/project")
            .name("master")
            .allowed_to_unprotect(ProtectedAccess::User(1))
            .allowed_to_unprotect(ProtectedAccess::Group(1))
            .allowed_to_unprotect(ProtectedAccess::Level(ProtectedAccessLevel::Developer))
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
