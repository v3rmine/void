// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Project repository tags API endpoints.
//!
//! These endpoints are used for interacting with a project's git tags.

mod create;
mod tag;
mod tags;

pub use self::create::CreateTag;
pub use self::create::CreateTagBuilder;
pub use self::create::CreateTagBuilderError;

pub use self::tag::Tag;
pub use self::tag::TagBuilder;
pub use self::tag::TagBuilderError;

pub use self::tags::Tags;
pub use self::tags::TagsBuilder;
pub use self::tags::TagsBuilderError;
pub use self::tags::TagsOrderBy;
