// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(clippy::module_inception)]

//! Group-related API endpoints
//!
//! These endpoints are used for querying and modifying groups and their resources.

mod create;
mod edit;
mod group;
mod groups;
pub mod hooks;
pub mod issues;
pub mod members;
pub mod milestones;
pub mod projects;
mod share;
pub mod subgroups;
mod unshare;

pub use create::BranchProtection;
pub use create::CreateGroup;
pub use create::CreateGroupBuilder;
pub use create::CreateGroupBuilderError;
pub use create::GroupProjectCreationAccessLevel;
pub use create::SharedRunnersMinutesLimit;
pub use create::SubgroupCreationAccessLevel;

pub use edit::EditGroup;
pub use edit::EditGroupBuilder;
pub use edit::EditGroupBuilderError;
pub use edit::SharedRunnersSetting;

pub use group::Group;
pub use group::GroupBuilder;
pub use group::GroupBuilderError;

pub use groups::GroupOrderBy;
pub use groups::Groups;
pub use groups::GroupsBuilder;
pub use groups::GroupsBuilderError;

pub use share::ShareGroup;
pub use share::ShareGroupBuilder;
pub use share::ShareGroupBuilderError;

pub use unshare::UnshareGroup;
pub use unshare::UnshareGroupBuilder;
pub use unshare::UnshareGroupBuilderError;
