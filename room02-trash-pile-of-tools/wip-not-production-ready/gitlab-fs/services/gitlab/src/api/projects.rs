// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(clippy::module_inception)]

//! Project-related API endpoints
//!
//! These endpoints are used for querying and modifying projects and their resources.

mod create;
pub mod deploy_keys;
mod edit;
pub mod environments;
pub mod hooks;
pub mod issues;
pub mod jobs;
pub mod labels;
pub mod members;
pub mod merge_requests;
pub mod merge_trains;
pub mod milestones;
pub mod pipelines;
mod project;
mod projects;
pub mod protected_branches;
pub mod protected_tags;
pub mod registry;
pub mod releases;
pub mod repository;
mod share;
mod unshare;
pub mod variables;

pub use self::create::AutoDevOpsDeployStrategy;
pub use self::create::BuildGitStrategy;
pub use self::create::ContainerExpirationCadence;
pub use self::create::ContainerExpirationKeepN;
pub use self::create::ContainerExpirationOlderThan;
pub use self::create::ContainerExpirationPolicy;
pub use self::create::ContainerExpirationPolicyBuilder;
pub use self::create::ContainerExpirationPolicyBuilderError;
pub use self::create::CreateProject;
pub use self::create::CreateProjectBuilder;
pub use self::create::CreateProjectBuilderError;
pub use self::create::FeatureAccessLevel;
pub use self::create::FeatureAccessLevelPublic;
pub use self::create::MergeMethod;
pub use self::create::SquashOption;

pub use self::edit::EditProject;
pub use self::edit::EditProjectBuilder;
pub use self::edit::EditProjectBuilderError;

pub use self::project::Project;
pub use self::project::ProjectBuilder;
pub use self::project::ProjectBuilderError;

pub use self::projects::ProjectOrderBy;
pub use self::projects::Projects;
pub use self::projects::ProjectsBuilder;
pub use self::projects::ProjectsBuilderError;

pub use self::share::ShareProject;
pub use self::share::ShareProjectBuilder;
pub use self::share::ShareProjectBuilderError;

pub use self::unshare::UnshareProject;
pub use self::unshare::UnshareProjectBuilder;
pub use self::unshare::UnshareProjectBuilderError;
