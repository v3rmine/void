// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Project hook API endpoints.
//!
//! These endpoints are used for querying webhooks for a project.

mod create;
mod delete;
mod edit;
mod hook;
mod hooks;

pub use self::create::CreateHook;
pub use self::create::CreateHookBuilder;
pub use self::create::CreateHookBuilderError;

pub use self::edit::EditHook;
pub use self::edit::EditHookBuilder;
pub use self::edit::EditHookBuilderError;

pub use self::delete::DeleteHook;
pub use self::delete::DeleteHookBuilder;
pub use self::delete::DeleteHookBuilderError;

pub use self::hook::Hook;
pub use self::hook::HookBuilder;
pub use self::hook::HookBuilderError;

pub use self::hooks::Hooks;
pub use self::hooks::HooksBuilder;
pub use self::hooks::HooksBuilderError;
