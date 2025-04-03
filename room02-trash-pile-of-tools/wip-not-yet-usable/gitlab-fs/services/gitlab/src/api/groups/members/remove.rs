// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Remove a user from a group.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct RemoveGroupMember<'a> {
    /// The group to remove the user from.
    #[builder(setter(into))]
    group: NameOrId<'a>,
    /// The user to remove from the group.
    user: u64,
    /// Skip deletion of direct membership to subgroups and projects.
    #[builder(default)]
    skip_subresources: Option<bool>,
    /// Unassign from any issues or merge requests inside a given group.
    #[builder(default)]
    unassign_issuables: Option<bool>,
}

impl<'a> RemoveGroupMember<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> RemoveGroupMemberBuilder<'a> {
        RemoveGroupMemberBuilder::default()
    }
}

impl<'a> Endpoint for RemoveGroupMember<'a> {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("groups/{}/members/{}", self.group, self.user).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params
            .push_opt("skip_subresources", self.skip_subresources)
            .push_opt("unassign_issuables", self.unassign_issuables);

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::groups::members::{RemoveGroupMember, RemoveGroupMemberBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn all_parameters_are_needed() {
        let err = RemoveGroupMember::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, RemoveGroupMemberBuilderError, "group");
    }

    #[test]
    fn group_is_necessary() {
        let err = RemoveGroupMember::builder().user(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, RemoveGroupMemberBuilderError, "group");
    }

    #[test]
    fn user_is_necessary() {
        let err = RemoveGroupMember::builder().group(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, RemoveGroupMemberBuilderError, "user");
    }

    #[test]
    fn sufficient_parameters() {
        RemoveGroupMember::builder()
            .group("group")
            .user(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::DELETE)
            .endpoint("groups/group%2Fsubgroup/members/1")
            .content_type("application/x-www-form-urlencoded")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = RemoveGroupMember::builder()
            .group("group/subgroup")
            .user(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_skip_subresources() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::DELETE)
            .endpoint("groups/group%2Fsubgroup/members/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("skip_subresources=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = RemoveGroupMember::builder()
            .group("group/subgroup")
            .user(1)
            .skip_subresources(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_unassign_issuables() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::DELETE)
            .endpoint("groups/group%2Fsubgroup/members/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("unassign_issuables=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = RemoveGroupMember::builder()
            .group("group/subgroup")
            .user(1)
            .unassign_issuables(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
