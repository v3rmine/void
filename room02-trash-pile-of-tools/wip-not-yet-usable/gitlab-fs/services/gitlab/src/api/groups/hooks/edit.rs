// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Edit an existing webhook for a group.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct EditHook<'a> {
    /// The group to edit a webhook within.
    #[builder(setter(into))]
    group: NameOrId<'a>,
    /// The ID of the hook to edit.
    hook_id: u64,

    /// The URL for the webhook to contact.
    #[builder(setter(into), default)]
    url: Option<Cow<'a, str>>,

    /// Whether to send push events for this webhook or not.
    #[builder(default)]
    push_events: Option<bool>,
    /// Filter which branches send push events for this webhook.
    #[builder(setter(into), default)]
    push_events_branch_filter: Option<Cow<'a, str>>,
    /// Whether to send issue events for this webhook or not.
    #[builder(default)]
    issues_events: Option<bool>,
    /// Whether to send confidential issue events for this webhook or not.
    #[builder(default)]
    confidential_issues_events: Option<bool>,
    /// Whether to send merge request events for this webhook or not.
    #[builder(default)]
    merge_requests_events: Option<bool>,
    /// Whether to send tag events for this webhook or not.
    #[builder(default)]
    tag_push_events: Option<bool>,
    /// Whether to send note (comment) events for this webhook or not.
    #[builder(default)]
    note_events: Option<bool>,
    /// Whether to send confidential note (comment) events for this webhook or not.
    #[builder(default)]
    confidential_note_events: Option<bool>,
    /// Whether to send job events for this webhook or not.
    #[builder(default)]
    job_events: Option<bool>,
    /// Whether to send pipeline events for this webhook or not.
    #[builder(default)]
    pipeline_events: Option<bool>,
    /// Whether to send wiki page events for this webhook or not.
    #[builder(default)]
    wiki_page_events: Option<bool>,
    /// Whether to send deployment events for this webhook or not.
    #[builder(default)]
    deployment_events: Option<bool>,
    /// Whether to send release events for this webhook or not.
    #[builder(default)]
    releases_events: Option<bool>,
    /// Whether to send subgroup events for this webhook or not.
    #[builder(default)]
    subgroup_events: Option<bool>,

    /// Whether to verify SSL/TLS certificates for the webhook endpoint or not.
    #[builder(default)]
    enable_ssl_verification: Option<bool>,
    /// A secret token to include in webhook deliveries.
    ///
    /// This may be used to ensure that the webhook is actually coming from the GitLab instance.
    #[builder(setter(into), default)]
    token: Option<Cow<'a, str>>,
}

impl<'a> EditHook<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> EditHookBuilder<'a> {
        EditHookBuilder::default()
    }
}

impl<'a> Endpoint for EditHook<'a> {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("groups/{}/hooks/{}", self.group, self.hook_id).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params
            .push_opt("url", self.url.as_ref())
            .push_opt("push_events", self.push_events)
            .push_opt(
                "push_events_branch_filter",
                self.push_events_branch_filter.as_ref(),
            )
            .push_opt("issues_events", self.issues_events)
            .push_opt(
                "confidential_issues_events",
                self.confidential_issues_events,
            )
            .push_opt("merge_requests_events", self.merge_requests_events)
            .push_opt("tag_push_events", self.tag_push_events)
            .push_opt("note_events", self.note_events)
            .push_opt("confidential_note_events", self.confidential_note_events)
            .push_opt("job_events", self.job_events)
            .push_opt("pipeline_events", self.pipeline_events)
            .push_opt("wiki_page_events", self.wiki_page_events)
            .push_opt("deployment_events", self.deployment_events)
            .push_opt("releases_events", self.releases_events)
            .push_opt("subgroup_events", self.subgroup_events)
            .push_opt("enable_ssl_verification", self.enable_ssl_verification)
            .push_opt("token", self.token.as_ref());

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::groups::hooks::{EditHook, EditHookBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn group_and_hook_id_are_necessary() {
        let err = EditHook::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, EditHookBuilderError, "group");
    }

    #[test]
    fn group_is_necessary() {
        let err = EditHook::builder().hook_id(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, EditHookBuilderError, "group");
    }

    #[test]
    fn hook_id_is_necessary() {
        let err = EditHook::builder().group("group").build().unwrap_err();
        crate::test::assert_missing_field!(err, EditHookBuilderError, "hook_id");
    }

    #[test]
    fn group_and_hook_id_are_sufficient() {
        EditHook::builder()
            .group("group")
            .hook_id(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup/hooks/1")
            .content_type("application/x-www-form-urlencoded")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditHook::builder()
            .group("simple/group")
            .hook_id(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_url() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup/hooks/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("url=https%3A%2F%2Ftest.invalid%2Fpath%3Fsome%3Dfoo")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditHook::builder()
            .group("simple/group")
            .hook_id(1)
            .url("https://test.invalid/path?some=foo")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_push_events() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup/hooks/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("push_events=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditHook::builder()
            .group("simple/group")
            .hook_id(1)
            .push_events(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_push_events_branch_filter() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup/hooks/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("push_events_branch_filter=branch%2F*%2Ffilter")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditHook::builder()
            .group("simple/group")
            .hook_id(1)
            .push_events_branch_filter("branch/*/filter")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_issues_events() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup/hooks/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("issues_events=false")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditHook::builder()
            .group("simple/group")
            .hook_id(1)
            .issues_events(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_confidential_issues_events() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup/hooks/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("confidential_issues_events=false")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditHook::builder()
            .group("simple/group")
            .hook_id(1)
            .confidential_issues_events(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_merge_requests_events() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup/hooks/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("merge_requests_events=false")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditHook::builder()
            .group("simple/group")
            .hook_id(1)
            .merge_requests_events(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_tag_push_events() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup/hooks/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("tag_push_events=false")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditHook::builder()
            .group("simple/group")
            .hook_id(1)
            .tag_push_events(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_note_events() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup/hooks/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("note_events=false")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditHook::builder()
            .group("simple/group")
            .hook_id(1)
            .note_events(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_confidential_note_events() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup/hooks/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("confidential_note_events=false")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditHook::builder()
            .group("simple/group")
            .hook_id(1)
            .confidential_note_events(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_job_events() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup/hooks/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("job_events=false")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditHook::builder()
            .group("simple/group")
            .hook_id(1)
            .job_events(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_pipeline_events() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup/hooks/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("pipeline_events=false")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditHook::builder()
            .group("simple/group")
            .hook_id(1)
            .pipeline_events(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_wiki_page_events() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup/hooks/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("wiki_page_events=false")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditHook::builder()
            .group("simple/group")
            .hook_id(1)
            .wiki_page_events(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_deployment_events() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup/hooks/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("deployment_events=false")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditHook::builder()
            .group("simple/group")
            .hook_id(1)
            .deployment_events(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_releases_events() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup/hooks/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("releases_events=false")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditHook::builder()
            .group("simple/group")
            .hook_id(1)
            .releases_events(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_enable_ssl_verification() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup/hooks/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("enable_ssl_verification=false")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditHook::builder()
            .group("simple/group")
            .hook_id(1)
            .enable_ssl_verification(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_token() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup/hooks/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("token=secret")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditHook::builder()
            .group("simple/group")
            .hook_id(1)
            .token("secret")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
