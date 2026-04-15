// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use chrono::NaiveDate;
use derive_builder::Builder;

use crate::api::common::{AccessLevel, NameOrId};
use crate::api::endpoint_prelude::*;

/// Share group with another group.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct ShareGroup<'a> {
    /// The ID or URL-encoded path of the group.
    #[builder(setter(into))]
    id: NameOrId<'a>,
    /// The ID of the group to share with.
    group_id: u64,
    /// The access level to grant the group.
    group_access: AccessLevel,
    /// When the group's access expires.
    #[builder(default)]
    expires_at: Option<NaiveDate>,
}

impl<'a> ShareGroup<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> ShareGroupBuilder<'a> {
        ShareGroupBuilder::default()
    }
}

impl<'a> Endpoint for ShareGroup<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("groups/{}/share", self.id).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params
            .push("group_id", self.group_id)
            .push("group_access", self.group_access.as_u64())
            .push_opt("expires_at", self.expires_at);

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use http::Method;

    use crate::api::common::AccessLevel;
    use crate::api::groups::{ShareGroup, ShareGroupBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn all_parameters_are_needed() {
        let err = ShareGroup::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, ShareGroupBuilderError, "id");
    }

    #[test]
    fn id_is_necessary() {
        let err = ShareGroup::builder()
            .group_id(1)
            .group_access(AccessLevel::Developer)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, ShareGroupBuilderError, "id");
    }

    #[test]
    fn group_id_is_necessary() {
        let err = ShareGroup::builder()
            .id(1)
            .group_access(AccessLevel::Developer)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, ShareGroupBuilderError, "group_id");
    }

    #[test]
    fn group_access_is_necessary() {
        let err = ShareGroup::builder().id(1).group_id(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, ShareGroupBuilderError, "group_access");
    }

    #[test]
    fn sufficient_parameters() {
        ShareGroup::builder()
            .id("id")
            .group_id(1)
            .group_access(AccessLevel::Developer)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("groups/simple%2Fproject/share")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("group_id=1", "&group_access=30"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ShareGroup::builder()
            .id("simple/project")
            .group_id(1)
            .group_access(AccessLevel::Developer)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_expires_at() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("groups/1/share")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "group_id=1",
                "&group_access=30",
                "&expires_at=2020-01-01",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ShareGroup::builder()
            .id(1)
            .group_id(1)
            .group_access(AccessLevel::Developer)
            .expires_at(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
