pub mod endpoints;
mod helpers;
pub mod rate_limits;
pub mod schemas;

use std::marker::PhantomData;

use http::{uri::PathAndQuery, HeaderMap, Method, Request};
use serde::Deserialize;
use thiserror::Error;

use self::rate_limits::RateLimit;

pub const API_VERSION: &str = "v1.3";
pub const API_BASE_URL: &str = "https://api.artifactsmmo.com";

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to parse header value: {0}")]
    InvalidHeaderValue(#[from] http::header::InvalidHeaderValue),
    #[error("Failed to parse URI: {0}")]
    InvalidUri(#[from] http::uri::InvalidUri),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Failed to parse JSON: {0}")]
    ParseJson(#[from] serde_json::Error),
}

#[derive(Debug, Clone)]
pub struct EncodedRequest<T> {
    pub method: Method,
    pub path: PathAndQuery,
    pub headers: HeaderMap,
    // Content as binary because the client doesn't need to know the format (sans-io style)
    pub content: Vec<u8>,
    // Rate limit is part of the API definition so we know it at comptime
    pub rate_limit: RateLimit<'static>,
    marker: PhantomData<T>,
}

pub trait ParseResponse<'de> {
    type Response: Deserialize<'de>;

    fn parse_response(response: &'de [u8]) -> Result<Self::Response, Error> {
        Ok(serde_json::from_slice(response)?)
    }
}

impl<T> TryFrom<EncodedRequest<T>> for Request<Vec<u8>> {
    type Error = http::Error;

    fn try_from(value: EncodedRequest<T>) -> Result<Self, Self::Error> {
        let request = Request::builder().method(&value.method).uri(value.path);

        let request = value
            .headers
            .iter()
            .fold(request, |request, (key, value)| request.header(key, value));

        request.body(value.content)
    }
}
