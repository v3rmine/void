// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Unshare a group from another group.
#[derive(Debug, Builder, Clone)]
pub struct UnshareGroup<'a> {
    /// The ID or URL-encoded path of the group
    #[builder(setter(into))]
    id: NameOrId<'a>,
    /// The ID of the group to unlink sharing with.
    group_id: u64,
}

impl<'a> UnshareGroup<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> UnshareGroupBuilder<'a> {
        UnshareGroupBuilder::default()
    }
}

impl<'a> Endpoint for UnshareGroup<'a> {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("groups/{}/share/{}", self.id, self.group_id).into()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::groups::{UnshareGroup, UnshareGroupBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn all_parameters_are_needed() {
        let err = UnshareGroup::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, UnshareGroupBuilderError, "id");
    }

    #[test]
    fn id_is_necessary() {
        let err = UnshareGroup::builder().group_id(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, UnshareGroupBuilderError, "id");
    }

    #[test]
    fn group_id_is_necessary() {
        let err = UnshareGroup::builder().id(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, UnshareGroupBuilderError, "group_id");
    }

    #[test]
    fn sufficient_parameters() {
        UnshareGroup::builder().id(1).group_id(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::DELETE)
            .endpoint("groups/group%2Fsubgroup/share/1")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = UnshareGroup::builder()
            .id("group/subgroup")
            .group_id(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
