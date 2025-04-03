// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use chrono::NaiveDate;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Create a new milestone on a group.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct CreateGroupMilestone<'a> {
    /// The group to create a new milestone within.
    #[builder(setter(into))]
    group: NameOrId<'a>,
    /// The title of the milestone.
    #[builder(setter(into))]
    title: Cow<'a, str>,

    /// A short description for the milestone.
    #[builder(setter(into), default)]
    description: Option<Cow<'a, str>>,
    /// When the milestone is due.
    #[builder(default)]
    due_date: Option<NaiveDate>,
    /// When the milestone starts.
    #[builder(default)]
    start_date: Option<NaiveDate>,
}

impl<'a> CreateGroupMilestone<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> CreateGroupMilestoneBuilder<'a> {
        CreateGroupMilestoneBuilder::default()
    }
}

impl<'a> Endpoint for CreateGroupMilestone<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("groups/{}/milestones", self.group).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params
            .push("title", &self.title)
            .push_opt("description", self.description.as_ref())
            .push_opt("due_date", self.due_date)
            .push_opt("start_date", self.start_date);

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use http::Method;

    use crate::api::groups::milestones::{CreateGroupMilestone, CreateGroupMilestoneBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn group_and_title_are_necessary() {
        let err = CreateGroupMilestone::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, CreateGroupMilestoneBuilderError, "group");
    }

    #[test]
    fn group_is_necessary() {
        let err = CreateGroupMilestone::builder()
            .title("title")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateGroupMilestoneBuilderError, "group");
    }

    #[test]
    fn title_is_necessary() {
        let err = CreateGroupMilestone::builder()
            .group("group")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateGroupMilestoneBuilderError, "title");
    }

    #[test]
    fn group_and_title_are_sufficient() {
        CreateGroupMilestone::builder()
            .group("group")
            .title("title")
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("groups/group%2Fsubgroup/milestones")
            .content_type("application/x-www-form-urlencoded")
            .body_str("title=title")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateGroupMilestone::builder()
            .group("group/subgroup")
            .title("title")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_description() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("groups/group%2Fsubgroup/milestones")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("title=title", "&description=description"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateGroupMilestone::builder()
            .group("group/subgroup")
            .title("title")
            .description("description")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_due_date() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("groups/group%2Fsubgroup/milestones")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("title=title", "&due_date=2020-01-01"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateGroupMilestone::builder()
            .group("group/subgroup")
            .title("title")
            .due_date(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_start_date() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("groups/group%2Fsubgroup/milestones")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("title=title", "&start_date=2020-01-01"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateGroupMilestone::builder()
            .group("group/subgroup")
            .title("title")
            .start_date(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
