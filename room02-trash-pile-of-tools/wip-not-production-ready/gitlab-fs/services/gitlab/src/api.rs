// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![warn(missing_docs)]

//! API endpoint structures
//!
//! The types in this module are meant to aid in constructing the appropriate calls using type-safe
//! Rust idioms.
//!
//! All endpoints use the builder pattern and have their members as private so that there are no
//! API implications of adding new members for additional query parameters in future GitLab
//! releases.
//!
//! # Example
//!
//! ```rust,no_run
//! use serde::Deserialize;
//! use gitlab::Gitlab;
//! use gitlab::api::{self, projects, Query};
//!
//! // The return type of a `Project`. Note that GitLab may contain more information, but you can
//! // define your structure to only fetch what is needed.
//! #[derive(Debug, Deserialize)]
//! struct Project {
//!     name: String,
//! }
//!
//! // Create the client.
//! let client = Gitlab::new("gitlab.com", "private-token").unwrap();
//!
//! // Create a simple endpoint. This one gets the "gitlab-org/gitlab" project information.
//! let endpoint = projects::Project::builder().project("gitlab-org/gitlab").build().unwrap();
//! // Call the endpoint. The return type decides how to represent the value.
//! let project: Project = endpoint.query(&client).unwrap();
//! // For some endpoints (mainly `POST` endpoints), you may want to ignore the result.
//! // `api::ignore` can be used to do this.
//! let _: () = api::ignore(endpoint).query(&client).unwrap();
//!
//! // Some endpoints support pagination. They work on their own or via the `api::paged` function
//! // to get further results.
//! let pageable_endpoint = projects::Projects::builder().build().unwrap();
//! // The endpoint on its own is just the first page of results (usually 20 entries).
//! let first_page: Vec<Project> = pageable_endpoint.query(&client).unwrap();
//! // `api::paged` can be used to get results up to some count or all results.
//! let first_200_projects: Vec<Project> = api::paged(pageable_endpoint, api::Pagination::Limit(200)).query(&client).unwrap();
//!
//! // Builders accept strings or integers for some fields. This is done wherever GitLab supports
//! // either IDs or names being used.
//! let endpoint = projects::Project::builder().project(278964).build().unwrap();
//! // The `api::raw` function can be used to return the raw data from the endpoint. This is
//! // usually meant for endpoints which represent file contents, pipeline artifacts, etc., but may
//! // be used with any endpoint.
//! let raw_data: Vec<u8> = api::raw(endpoint).query(&client).unwrap();
//! ```

mod client;
mod endpoint;
mod error;
mod ignore;
mod paged;
mod params;
pub(crate) mod query;
mod raw;
mod sudo;

pub mod endpoint_prelude;

pub mod common;
pub mod deploy_keys;
pub mod groups;
pub mod issues;
pub mod projects;
pub mod retry;
pub mod users;

pub(crate) mod helpers;

pub use self::client::AsyncClient;
pub use self::client::Client;
pub use self::client::RestClient;

pub use self::endpoint::Endpoint;

pub use self::error::ApiError;
pub use self::error::BodyError;

pub use self::ignore::ignore;
pub use self::ignore::Ignore;

pub use self::paged::paged;
pub use self::paged::LazilyPagedIter;
pub use self::paged::LinkHeaderParseError;
pub use self::paged::Pageable;
pub use self::paged::Paged;
pub use self::paged::Pagination;
pub use self::paged::PaginationError;

pub use self::params::FormParams;
pub use self::params::ParamValue;
pub use self::params::QueryParams;

pub use self::query::AsyncQuery;
pub use self::query::Query;

pub use self::raw::raw;
pub use self::raw::Raw;

pub use self::sudo::sudo;
pub use self::sudo::Sudo;
pub use self::sudo::SudoContext;
