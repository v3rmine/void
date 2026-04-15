// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Project variable API endpoints.
//!
//! These endpoints are used for querying a project's variables.

mod create;
mod update;
mod variable;
mod variables;

pub use self::create::CreateProjectVariable;
pub use self::create::CreateProjectVariableBuilder;
pub use self::create::CreateProjectVariableBuilderError;
pub use self::create::ProjectVariableType;

pub use self::update::UpdateProjectVariable;
pub use self::update::UpdateProjectVariableBuilder;
pub use self::update::UpdateProjectVariableBuilderError;

pub use self::variable::ProjectVariable;
pub use self::variable::ProjectVariableBuilder;
pub use self::variable::ProjectVariableBuilderError;
pub use self::variable::ProjectVariableFilter;

pub use self::variables::ProjectVariables;
pub use self::variables::ProjectVariablesBuilder;
pub use self::variables::ProjectVariablesBuilderError;
pub use self::variables::ProjectVariablesFilter;
