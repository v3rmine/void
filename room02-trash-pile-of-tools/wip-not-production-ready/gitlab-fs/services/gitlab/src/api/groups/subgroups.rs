// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Group subgroups API endpoints.
//!
//! These endpoints are used for querying group subgroups.

mod subgroups;

pub use self::subgroups::GroupSubgroups;
pub use self::subgroups::GroupSubgroupsBuilder;
pub use self::subgroups::GroupSubgroupsBuilderError;
pub use self::subgroups::GroupSubgroupsOrderBy;
