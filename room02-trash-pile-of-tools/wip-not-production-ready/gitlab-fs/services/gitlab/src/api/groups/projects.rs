// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Group projects API endpoints.
//!
//! These endpoints are used for querying group projects.

mod projects;
mod shared;

pub use self::projects::GroupProjects;
pub use self::projects::GroupProjectsBuilder;
pub use self::projects::GroupProjectsBuilderError;
pub use self::projects::GroupProjectsOrderBy;

pub use self::shared::SharedGroupProjects;
pub use self::shared::SharedGroupProjectsBuilder;
pub use self::shared::SharedGroupProjectsBuilderError;
pub use self::shared::SharedGroupProjectsOrderBy;
