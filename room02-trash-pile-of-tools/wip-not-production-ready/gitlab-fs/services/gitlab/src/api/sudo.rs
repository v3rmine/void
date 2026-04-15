// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::api::endpoint_prelude::*;

/// A `sudo` modifier that can be applied to any endpoint.
#[derive(Debug, Clone)]
pub struct SudoContext<'a> {
    /// The username to use for the endpoint.
    sudo: Cow<'a, str>,
}

impl<'a> SudoContext<'a> {
    /// Create a new `sudo` context for API endpoints.
    pub fn new<S>(sudo: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        SudoContext { sudo: sudo.into() }
    }

    /// Apply the context to an endpoint.
    pub fn apply<E>(&self, endpoint: E) -> Sudo<'a, E> {
        Sudo {
            endpoint,
            sudo: self.sudo.clone(),
        }
    }
}

/// Query information about the API calling user.
#[derive(Debug, Clone)]
pub struct Sudo<'a, E> {
    /// The endpoint to call with `sudo`.
    endpoint: E,

    /// The username to use for the endpoint.
    sudo: Cow<'a, str>,
}

/// Create a `sudo`-elevated version of an endpoint.
pub fn sudo<'a, E, S>(endpoint: E, sudo: S) -> Sudo<'a, E>
where
    S: Into<Cow<'a, str>>,
{
    Sudo {
        endpoint,
        sudo: sudo.into(),
    }
}

impl<'a, E> Endpoint for Sudo<'a, E>
where
    E: Endpoint,
{
    fn method(&self) -> Method {
        self.endpoint.method()
    }

    fn endpoint(&self) -> Cow<'static, str> {
        self.endpoint.endpoint()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = self.endpoint.parameters();
        params.push("sudo", &self.sudo);
        params
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        self.endpoint.body()
    }
}

impl<'a, E> Pageable for Sudo<'a, E>
where
    E: Pageable,
{
    fn use_keyset_pagination(&self) -> bool {
        self.endpoint.use_keyset_pagination()
    }
}

#[cfg(test)]
mod tests {
    use http::StatusCode;
    use serde::Deserialize;
    use serde_json::json;

    use crate::api::endpoint_prelude::*;
    use crate::api::{self, ApiError, Query, SudoContext};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    struct Dummy;

    impl Endpoint for Dummy {
        fn method(&self) -> Method {
            Method::GET
        }

        fn endpoint(&self) -> Cow<'static, str> {
            "dummy".into()
        }
    }

    #[derive(Debug, Deserialize)]
    struct DummyResult {
        value: u8,
    }

    #[test]
    fn test_gitlab_non_json_response() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("dummy")
            .add_query_params(&[("sudo", "user")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "not json");

        let res: Result<DummyResult, _> = api::sudo(Dummy, "user").query(&client);
        let err = res.unwrap_err();
        if let ApiError::GitlabService { status, .. } = err {
            assert_eq!(status, http::StatusCode::OK);
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn test_gitlab_empty_response() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("dummy")
            .add_query_params(&[("sudo", "user")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let res: Result<DummyResult, _> = api::sudo(Dummy, "user").query(&client);
        let err = res.unwrap_err();
        if let ApiError::GitlabService { status, .. } = err {
            assert_eq!(status, http::StatusCode::OK);
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn test_gitlab_error_bad_json() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("dummy")
            .add_query_params(&[("sudo", "user")])
            .status(StatusCode::NOT_FOUND)
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let res: Result<DummyResult, _> = api::sudo(Dummy, "user").query(&client);
        let err = res.unwrap_err();
        if let ApiError::GitlabService { status, .. } = err {
            assert_eq!(status, http::StatusCode::NOT_FOUND);
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn test_gitlab_error_detection() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("dummy")
            .add_query_params(&[("sudo", "user")])
            .status(StatusCode::NOT_FOUND)
            .build()
            .unwrap();
        let client = SingleTestClient::new_json(
            endpoint,
            &json!({
                "message": "dummy error message",
            }),
        );

        let res: Result<DummyResult, _> = api::sudo(Dummy, "user").query(&client);
        let err = res.unwrap_err();
        if let ApiError::Gitlab { msg } = err {
            assert_eq!(msg, "dummy error message");
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn test_gitlab_error_detection_legacy() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("dummy")
            .add_query_params(&[("sudo", "user")])
            .status(StatusCode::NOT_FOUND)
            .build()
            .unwrap();
        let client = SingleTestClient::new_json(
            endpoint,
            &json!({
                "error": "dummy error message",
            }),
        );

        let res: Result<DummyResult, _> = api::sudo(Dummy, "user").query(&client);
        let err = res.unwrap_err();
        if let ApiError::Gitlab { msg } = err {
            assert_eq!(msg, "dummy error message");
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn test_gitlab_error_detection_unknown() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("dummy")
            .add_query_params(&[("sudo", "user")])
            .status(StatusCode::NOT_FOUND)
            .build()
            .unwrap();
        let err_obj = json!({
            "bogus": "dummy error message",
        });
        let client = SingleTestClient::new_json(endpoint, &err_obj);

        let res: Result<DummyResult, _> = api::sudo(Dummy, "user").query(&client);
        let err = res.unwrap_err();
        if let ApiError::GitlabUnrecognized { obj } = err {
            assert_eq!(obj, err_obj);
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn test_bad_deserialization() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("dummy")
            .add_query_params(&[("sudo", "user")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_json(
            endpoint,
            &json!({
                "not_value": 0,
            }),
        );

        let res: Result<DummyResult, _> = api::sudo(Dummy, "user").query(&client);
        let err = res.unwrap_err();
        if let ApiError::DataType { source, typename } = err {
            assert_eq!(source.to_string(), "missing field `value`");
            assert_eq!(typename, "gitlab::api::sudo::tests::DummyResult");
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn test_good_deserialization() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("dummy")
            .add_query_params(&[("sudo", "user")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_json(
            endpoint,
            &json!({
                "value": 0,
            }),
        );

        let res: DummyResult = api::sudo(Dummy, "user").query(&client).unwrap();
        assert_eq!(res.value, 0);
    }

    #[test]
    fn test_sudo_context() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("dummy")
            .add_query_params(&[("sudo", "user")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_json(
            endpoint,
            &json!({
                "value": 0,
            }),
        );
        let sudo_ctx = SudoContext::new("user");
        let endpoint = sudo_ctx.apply(Dummy);

        let res: DummyResult = endpoint.query(&client).unwrap();
        assert_eq!(res.value, 0);
    }
}
