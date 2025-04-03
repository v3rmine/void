// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Project repository protected branches API endpoints.
//!
//! These endpoints are used for querying a project's protected branches.

mod protect;
mod protected_branch;
mod protected_branches;
mod unprotect;

pub use self::protect::ProtectBranch;
pub use self::protect::ProtectBranchBuilder;
pub use self::protect::ProtectBranchBuilderError;
pub use self::protect::ProtectedAccess;
pub use self::protect::ProtectedAccessLevel;

pub use self::unprotect::UnprotectBranch;
pub use self::unprotect::UnprotectBranchBuilder;
pub use self::unprotect::UnprotectBranchBuilderError;

pub use self::protected_branch::ProtectedBranch;
pub use self::protected_branch::ProtectedBranchBuilder;
pub use self::protected_branch::ProtectedBranchBuilderError;

pub use self::protected_branches::ProtectedBranches;
pub use self::protected_branches::ProtectedBranchesBuilder;
pub use self::protected_branches::ProtectedBranchesBuilderError;
