// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Query a single member of a group.
#[derive(Debug, Builder, Clone)]
pub struct GroupMember<'a> {
    /// The group to query for membership.
    #[builder(setter(into))]
    group: NameOrId<'a>,
    /// The ID of the user.
    user: u64,
}

impl<'a> GroupMember<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> GroupMemberBuilder<'a> {
        GroupMemberBuilder::default()
    }
}

impl<'a> Endpoint for GroupMember<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("groups/{}/members/{}", self.group, self.user).into()
    }
}

#[cfg(test)]
mod tests {
    use crate::api::groups::members::{GroupMember, GroupMemberBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn group_and_user_are_needed() {
        let err = GroupMember::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, GroupMemberBuilderError, "group");
    }

    #[test]
    fn group_is_needed() {
        let err = GroupMember::builder().user(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, GroupMemberBuilderError, "group");
    }

    #[test]
    fn user_is_needed() {
        let err = GroupMember::builder().group(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, GroupMemberBuilderError, "user");
    }

    #[test]
    fn group_and_user_are_sufficient() {
        GroupMember::builder().group(1).user(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/group%2Fsubgroup/members/1")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GroupMember::builder()
            .group("group/subgroup")
            .user(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
