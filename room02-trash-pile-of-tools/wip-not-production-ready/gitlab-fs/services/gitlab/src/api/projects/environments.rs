// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Project environments API endpoints.
//!
//! These endpoints are used for querying environments.

mod environment;
mod environments;

pub use self::environments::EnvironmentState;

pub use self::environment::Environment;
pub use self::environment::EnvironmentBuilder;
pub use self::environment::EnvironmentBuilderError;

pub use self::environments::Environments;
pub use self::environments::EnvironmentsBuilder;
pub use self::environments::EnvironmentsBuilderError;
