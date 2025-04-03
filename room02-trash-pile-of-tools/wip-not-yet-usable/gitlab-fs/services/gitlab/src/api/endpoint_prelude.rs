// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Endpoint prelude
//!
//! This module re-exports all of the types needed for endpoints to implement the
//! [`Endpoint`](../trait.Endpoint.html) trait.

pub use std::borrow::Cow;

pub use http::Method;

pub use crate::api::BodyError;
pub use crate::api::Client;
pub use crate::api::Endpoint;
pub use crate::api::FormParams;
pub use crate::api::Pageable;
pub use crate::api::QueryParams;
