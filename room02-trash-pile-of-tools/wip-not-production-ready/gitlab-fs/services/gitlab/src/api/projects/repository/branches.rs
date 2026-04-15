// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Project repository branches API endpoints.
//!
//! These endpoints are used for querying a project's branches.

mod branch;
mod branches;
mod create;

pub use self::branch::Branch;
pub use self::branch::BranchBuilder;
pub use self::branch::BranchBuilderError;

pub use self::branches::Branches;
pub use self::branches::BranchesBuilder;
pub use self::branches::BranchesBuilderError;

pub use self::create::CreateBranch;
pub use self::create::CreateBranchBuilder;
pub use self::create::CreateBranchBuilderError;
