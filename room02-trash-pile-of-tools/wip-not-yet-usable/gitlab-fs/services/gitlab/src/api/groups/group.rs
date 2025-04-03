// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Query for a specific group on an instance.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Group<'a> {
    /// The group to get.
    #[builder(setter(into))]
    group: NameOrId<'a>,

    /// Include custom attributes in the response.
    #[builder(default)]
    with_custom_attributes: Option<bool>,
    /// Include project information.
    #[deprecated(note = "use `GroupProjects` instead (unimplemented)")]
    #[builder(default)]
    with_projects: Option<bool>,
}

impl<'a> Group<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> GroupBuilder<'a> {
        GroupBuilder::default()
    }
}

impl<'a> Endpoint for Group<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("groups/{}", self.group).into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();

        params.push_opt("with_custom_attributes", self.with_custom_attributes);

        #[allow(deprecated)]
        {
            params.push_opt("with_projects", self.with_projects);
        }

        params
    }
}

#[cfg(test)]
mod tests {
    use crate::api::groups::{Group, GroupBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn group_is_necessary() {
        let err = Group::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, GroupBuilderError, "group");
    }

    #[test]
    fn group_is_sufficient() {
        Group::builder().group(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/group%2Fsubgroup")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Group::builder().group("group/subgroup").build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_with_custom_attributes() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/group%2Fsubgroup")
            .add_query_params(&[("with_custom_attributes", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Group::builder()
            .group("group/subgroup")
            .with_custom_attributes(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_with_projects() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/group%2Fsubgroup")
            .add_query_params(&[("with_projects", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Group::builder()
            .group("group/subgroup")
            .with_projects(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
