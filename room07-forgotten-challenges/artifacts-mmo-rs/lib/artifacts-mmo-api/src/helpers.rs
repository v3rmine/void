#![allow(clippy::declare_interior_mutable_const)]
use http::{
    header::{ACCEPT, CONTENT_TYPE},
    HeaderName, HeaderValue,
};

pub const ACCEPT_JSON: (HeaderName, HeaderValue) =
    (ACCEPT, HeaderValue::from_static("application/json"));

pub const CONTENT_TYPE_JSON: (HeaderName, HeaderValue) =
    (CONTENT_TYPE, HeaderValue::from_static("application/json"));
