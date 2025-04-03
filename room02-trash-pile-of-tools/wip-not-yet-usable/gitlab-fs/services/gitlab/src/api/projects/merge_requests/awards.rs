// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Project merge request award API endpoints.
//!
//! These endpoints are used for querying project merge request awards.

mod awards;

pub use self::awards::MergeRequestAwards;
pub use self::awards::MergeRequestAwardsBuilder;
pub use self::awards::MergeRequestAwardsBuilderError;
