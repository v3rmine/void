// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use async_trait::async_trait;
use http::{header, Request};

use crate::api::{query, ApiError, AsyncClient, AsyncQuery, Client, Endpoint, Query};

/// A query modifier that returns the raw data from the endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Raw<E> {
    endpoint: E,
}

/// Return the raw data from the endpoint.
pub fn raw<E>(endpoint: E) -> Raw<E> {
    Raw { endpoint }
}

impl<E, C> Query<Vec<u8>, C> for Raw<E>
where
    E: Endpoint,
    C: Client,
{
    fn query(&self, client: &C) -> Result<Vec<u8>, ApiError<C::Error>> {
        let mut url = client.rest_endpoint(&self.endpoint.endpoint())?;
        self.endpoint.parameters().add_to_url(&mut url);

        let req = Request::builder()
            .method(self.endpoint.method())
            .uri(query::url_to_http_uri(url));
        let (req, data) = if let Some((mime, data)) = self.endpoint.body()? {
            let req = req.header(header::CONTENT_TYPE, mime);
            (req, data)
        } else {
            (req, Vec::new())
        };
        let rsp = client.rest(req, data)?;
        if !rsp.status().is_success() {
            let v = if let Ok(v) = serde_json::from_slice(rsp.body()) {
                v
            } else {
                return Err(ApiError::server_error(rsp.status(), rsp.body()));
            };
            return Err(ApiError::from_gitlab(v));
        }

        Ok(rsp.into_body().as_ref().into())
    }
}

#[async_trait]
impl<E, C> AsyncQuery<Vec<u8>, C> for Raw<E>
where
    E: Endpoint + Sync,
    C: AsyncClient + Sync,
{
    async fn query_async(&self, client: &C) -> Result<Vec<u8>, ApiError<C::Error>> {
        let mut url = client.rest_endpoint(&self.endpoint.endpoint())?;
        self.endpoint.parameters().add_to_url(&mut url);

        let req = Request::builder()
            .method(self.endpoint.method())
            .uri(query::url_to_http_uri(url));
        let (req, data) = if let Some((mime, data)) = self.endpoint.body()? {
            let req = req.header(header::CONTENT_TYPE, mime);
            (req, data)
        } else {
            (req, Vec::new())
        };
        let rsp = client.rest_async(req, data).await?;
        if !rsp.status().is_success() {
            let v = if let Ok(v) = serde_json::from_slice(rsp.body()) {
                v
            } else {
                return Err(ApiError::server_error(rsp.status(), rsp.body()));
            };
            return Err(ApiError::from_gitlab(v));
        }

        Ok(rsp.into_body().as_ref().into())
    }
}

#[cfg(test)]
mod tests {
    use http::StatusCode;
    use serde_json::json;

    use crate::api::endpoint_prelude::*;
    use crate::api::{self, ApiError, AsyncQuery, Query};
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

    #[derive(Debug)]
    struct DummyResult {
        #[allow(dead_code)]
        value: u8,
    }

    #[test]
    fn test_gitlab_non_json_response() {
        let endpoint = ExpectedUrl::builder().endpoint("dummy").build().unwrap();
        let client = SingleTestClient::new_raw(endpoint, "not json");

        let data = api::raw(Dummy).query(&client).unwrap();
        itertools::assert_equal(data, "not json".bytes());
    }

    #[tokio::test]
    async fn test_gitlab_non_json_response_async() {
        let endpoint = ExpectedUrl::builder().endpoint("dummy").build().unwrap();
        let client = SingleTestClient::new_raw(endpoint, "not json");

        let data = api::raw(Dummy).query_async(&client).await.unwrap();
        itertools::assert_equal(data, "not json".bytes());
    }

    #[test]
    fn test_gitlab_error_bad_json() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("dummy")
            .status(StatusCode::NOT_FOUND)
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let err = api::raw(Dummy).query(&client).unwrap_err();
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
            .status(StatusCode::NOT_FOUND)
            .build()
            .unwrap();
        let client = SingleTestClient::new_json(
            endpoint,
            &json!({
                "message": "dummy error message",
            }),
        );

        let err = api::raw(Dummy).query(&client).unwrap_err();
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
            .status(StatusCode::NOT_FOUND)
            .build()
            .unwrap();
        let client = SingleTestClient::new_json(
            endpoint,
            &json!({
                "error": "dummy error message",
            }),
        );

        let err = api::raw(Dummy).query(&client).unwrap_err();
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
            .status(StatusCode::NOT_FOUND)
            .build()
            .unwrap();
        let err_obj = json!({
            "bogus": "dummy error message",
        });
        let client = SingleTestClient::new_json(endpoint, &err_obj);

        let err = api::raw(Dummy).query(&client).unwrap_err();
        if let ApiError::GitlabUnrecognized { obj } = err {
            assert_eq!(obj, err_obj);
        } else {
            panic!("unexpected error: {}", err);
        }
    }
}
