// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Project repository protected tags API endpoints.
//!
//! These endpoints are used for querying a project's protected tags.

mod protect;
mod protected_tag;
mod protected_tags;
mod unprotect;

pub use self::protect::ProtectTag;
pub use self::protect::ProtectTagBuilder;
pub use self::protect::ProtectTagBuilderError;
pub use super::protected_branches::ProtectedAccess;

pub use self::unprotect::UnprotectTag;
pub use self::unprotect::UnprotectTagBuilder;
pub use self::unprotect::UnprotectTagBuilderError;

pub use self::protected_tag::ProtectedTag;
pub use self::protected_tag::ProtectedTagBuilder;
pub use self::protected_tag::ProtectedTagBuilderError;

pub use self::protected_tags::ProtectedTags;
pub use self::protected_tags::ProtectedTagsBuilder;
pub use self::protected_tags::ProtectedTagsBuilderError;
