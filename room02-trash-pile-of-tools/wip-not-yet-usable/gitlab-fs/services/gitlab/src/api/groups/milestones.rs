// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Group milestone API endpoints.
//!
//! These endpoints are used for querying group milestones.

mod create;

pub use self::create::CreateGroupMilestone;
pub use self::create::CreateGroupMilestoneBuilder;
pub use self::create::CreateGroupMilestoneBuilderError;
