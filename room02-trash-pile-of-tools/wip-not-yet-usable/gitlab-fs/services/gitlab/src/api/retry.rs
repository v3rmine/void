// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Retry client wrapper
//!
//! This module provides a `Client` implementation which can wrap other `ApiClient` instances in
//! order to retry requests with an exponential backoff. Only service errors (those in the `5xx`
//! range) are retried and all others are passed through as final statuses.

use std::error::Error as StdError;
use std::iter;
use std::thread;
use std::time::Duration;

use bytes::Bytes;
use http::Response;
use url::Url;

use derive_builder::Builder;
use thiserror::Error;

use crate::api;

/// Parameters for retrying queries with an exponential backoff.
#[derive(Debug, Builder, Clone)]
pub struct Backoff {
    /// The maximum number of times to backoff.
    ///
    /// Defaults to `5`.
    #[builder(default = "5")]
    limit: usize,
    /// How long to wait after the first failure.
    ///
    /// Defaults to 1 second.
    #[builder(default = "Duration::from_secs(1)")]
    init: Duration,
    /// The scale parameter for timeouts after each subsequent failure.
    ///
    /// Defaults to `2.0`.
    #[builder(default = "2.0")]
    scale: f64,
}

fn should_backoff<E>(err: &api::ApiError<E>) -> bool
where
    E: StdError + Send + Sync + 'static,
{
    if let api::ApiError::GitlabService { status, .. } = err {
        status.is_server_error()
    } else {
        false
    }
}

impl Backoff {
    /// Create a builder for retry backoff parameters.
    pub fn builder() -> BackoffBuilder {
        BackoffBuilder::default()
    }

    fn retry<F, E>(&self, mut tryf: F) -> Result<Response<Bytes>, api::ApiError<Error<E>>>
    where
        F: FnMut() -> Result<Response<Bytes>, api::ApiError<E>>,
        E: StdError + Send + Sync + 'static,
    {
        iter::repeat(())
            .take(self.limit)
            .scan(self.init, |timeout, _| match tryf() {
                Ok(rsp) => {
                    if rsp.status().is_server_error() {
                        thread::sleep(*timeout);
                        *timeout = timeout.mul_f64(self.scale);
                        Some(None)
                    } else {
                        Some(Some(Ok(rsp)))
                    }
                },
                Err(err) => {
                    if should_backoff(&err) {
                        thread::sleep(*timeout);
                        *timeout = timeout.mul_f64(self.scale);
                        Some(None)
                    } else {
                        Some(Some(Err(err.map_client(Error::inner))))
                    }
                },
            })
            .flatten()
            .next()
            .unwrap_or_else(|| Err(api::ApiError::client(Error::backoff())))
    }
}

impl Default for Backoff {
    fn default() -> Self {
        Self::builder().build().unwrap()
    }
}

/// An error from a client even after retrying multiple times.
#[derive(Debug, Error)]
pub enum Error<E>
where
    E: StdError + Send + Sync + 'static,
{
    /// The request failed after multiple attempts.
    #[error("exponential backoff expired")]
    Backoff {},
    /// An error occurred within the client.
    #[error("{}", source)]
    Inner {
        /// The source of the error.
        #[from]
        source: E,
    },
}

impl<E> Error<E>
where
    E: StdError + Send + Sync + 'static,
{
    fn backoff() -> Self {
        Self::Backoff {}
    }

    fn inner(source: E) -> Self {
        Self::Inner { source }
    }
}

/// A wrapper around a client to perform exponential backoff while retrying errors.
///
/// ## Notes
///
/// Currently, the wrapping is not 100% compatible, however the gaps should not be common. Of note
/// is that the HTTP version is 1.1 since there is not a way to query the version from an existing
/// builder. Also, all requested extensions are ignored since they cannot be cloned reliably. In
/// the future, requests with extensions will be passed through, but without any backoff support.
pub struct Client<C> {
    client: C,
    backoff: Backoff,
}

impl<C> Client<C> {
    /// Create a client which retries in the face of service errors with an exponential backoff.
    pub fn new(client: C, backoff: Backoff) -> Self {
        Self { client, backoff }
    }
}

impl<C> api::RestClient for Client<C>
where
    C: api::RestClient,
{
    type Error = Error<C::Error>;

    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, api::ApiError<Self::Error>> {
        self.client
            .rest_endpoint(endpoint)
            .map_err(|e| e.map_client(Error::inner))
    }
}

impl<C> api::Client for Client<C>
where
    C: api::Client,
{
    fn rest(
        &self,
        request: http::request::Builder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, api::ApiError<Self::Error>> {
        self.backoff.retry(|| {
            let mut builder = http::request::Request::builder();
            if let Some(method) = request.method_ref() {
                builder = builder.method(method);
            }
            if let Some(uri) = request.uri_ref() {
                builder = builder.uri(uri);
            }
            // https://github.com/hyperium/http/pull/495
            // if let Some(version) = request.version_ref() {
            //     builder = builder.version(version);
            // }
            if let Some(headers) = request.headers_ref() {
                for (key, value) in headers.iter() {
                    builder = builder.header(key, value);
                }
            }
            // Ignore extensions for now. Can be handled once this is released:
            // https://github.com/hyperium/http/pull/497

            self.client.rest(builder, body.clone())
        })
    }
}

#[cfg(test)]
mod test {
    use http::{Response, StatusCode};
    use serde::Deserialize;
    use serde_json::json;
    use thiserror::Error;

    use crate::api::endpoint_prelude::*;
    use crate::api::{self, retry, ApiError, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[derive(Debug, Error)]
    #[error("bogus")]
    struct BogusError {}

    #[test]
    fn backoff_first_success() {
        let backoff = retry::Backoff::default();
        let mut call_count = 0;
        let body: &'static [u8] = b"";
        backoff
            .retry::<_, BogusError>(|| {
                call_count += 1;
                Ok(Response::builder()
                    .status(StatusCode::OK)
                    .body(body.into())
                    .unwrap())
            })
            .unwrap();
        assert_eq!(call_count, 1);
    }

    #[test]
    fn backoff_second_success() {
        let backoff = retry::Backoff::default();
        let mut call_count = 0;
        let mut did_err = false;
        let body: &'static [u8] = b"";
        backoff
            .retry::<_, BogusError>(|| {
                call_count += 1;
                if did_err {
                    Ok(Response::builder()
                        .status(StatusCode::OK)
                        .body(body.into())
                        .unwrap())
                } else {
                    did_err = true;
                    Ok(Response::builder()
                        .status(StatusCode::SERVICE_UNAVAILABLE)
                        .body(body.into())
                        .unwrap())
                }
            })
            .unwrap();
        assert_eq!(call_count, 2);
    }

    #[test]
    fn backoff_second_success_gitlab_service_err() {
        let backoff = retry::Backoff::default();
        let mut call_count = 0;
        let mut did_err = false;
        let body: &'static [u8] = b"";
        backoff
            .retry::<_, BogusError>(|| {
                call_count += 1;
                if did_err {
                    Ok(Response::builder()
                        .status(StatusCode::OK)
                        .body(body.into())
                        .unwrap())
                } else {
                    did_err = true;
                    Err(api::ApiError::GitlabService {
                        status: StatusCode::INTERNAL_SERVER_ERROR,
                        data: Vec::default(),
                    })
                }
            })
            .unwrap();
        assert_eq!(call_count, 2);
    }

    #[test]
    fn backoff_no_success() {
        let backoff = retry::Backoff::builder().limit(3).build().unwrap();
        let mut call_count = 0;
        let body: &'static [u8] = b"";
        let err = backoff
            .retry::<_, BogusError>(|| {
                call_count += 1;
                Ok(Response::builder()
                    .status(StatusCode::SERVICE_UNAVAILABLE)
                    .body(body.into())
                    .unwrap())
            })
            .unwrap_err();
        assert_eq!(call_count, backoff.limit);
        if let api::ApiError::Client {
            source: retry::Error::Backoff {},
        } = err
        {
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn backoff_no_success_gitlab_service_err() {
        let backoff = retry::Backoff::builder().limit(3).build().unwrap();
        let mut call_count = 0;
        let err = backoff
            .retry::<_, BogusError>(|| {
                call_count += 1;
                Err(api::ApiError::GitlabService {
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    data: Vec::default(),
                })
            })
            .unwrap_err();
        assert_eq!(call_count, backoff.limit);
        if let api::ApiError::Client {
            source: retry::Error::Backoff {},
        } = err
        {
        } else {
            panic!("unexpected error: {}", err);
        }
    }

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
    fn retry_client_ok() {
        let endpoint = ExpectedUrl::builder().endpoint("dummy").build().unwrap();
        let client = SingleTestClient::new_json(
            endpoint,
            &json!({
                "value": 0,
            }),
        );
        let backoff = retry::Backoff::default();
        let client = retry::Client::new(client, backoff);

        let res: DummyResult = Dummy.query(&client).unwrap();
        assert_eq!(res.value, 0);
    }

    #[test]
    fn retry_client_err() {
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
        let backoff = retry::Backoff::default();
        let client = retry::Client::new(client, backoff);

        let res: Result<DummyResult, _> = Dummy.query(&client);
        let err = res.unwrap_err();
        if let ApiError::Gitlab { msg } = err {
            assert_eq!(msg, "dummy error message");
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn retry_client_retry_timeout() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("dummy")
            .status(StatusCode::SERVICE_UNAVAILABLE)
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");
        let backoff = retry::Backoff::builder().limit(3).build().unwrap();
        let client = retry::Client::new(client, backoff);

        let res: Result<DummyResult, _> = Dummy.query(&client);
        let err = res.unwrap_err();
        if let ApiError::Client {
            source: retry::Error::Backoff {},
        } = err
        {
            // expected
        } else {
            panic!("unexpected error: {}", err);
        }
    }
}
