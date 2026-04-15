// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Project deploy keys API endpoints.
//!
//! These endpoints are used for querying projects deploy keys

mod create;
mod delete;
mod deploy_key;
mod deploy_keys;
mod edit;
mod enable;

pub use self::create::CreateDeployKey;
pub use self::create::CreateDeployKeyBuilder;
pub use self::create::CreateDeployKeyBuilderError;

pub use self::delete::DeleteDeployKey;
pub use self::delete::DeleteDeployKeyBuilder;
pub use self::delete::DeleteDeployKeyBuilderError;

pub use self::edit::EditDeployKey;
pub use self::edit::EditDeployKeyBuilder;
pub use self::edit::EditDeployKeyBuilderError;

pub use self::enable::EnableDeployKey;
pub use self::enable::EnableDeployKeyBuilder;
pub use self::enable::EnableDeployKeyBuilderError;

pub use self::deploy_key::DeployKey;
pub use self::deploy_key::DeployKeyBuilder;
pub use self::deploy_key::DeployKeyBuilderError;

pub use self::deploy_keys::DeployKeys;
pub use self::deploy_keys::DeployKeysBuilder;
pub use self::deploy_keys::DeployKeysBuilderError;
