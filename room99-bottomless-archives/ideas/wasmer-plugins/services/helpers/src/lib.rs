#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]

//! Library to help
//! - with the usage of functions types in macros
//! - to convert rust types to/from binary

pub use helpers_macros::*;

mod converters;
pub use converters::*;

/// Struct to hold rust function type details to use in structs
pub trait FuncInfo {
    /// It's the name of the function as a static str
    const FUNC_ID: &'static str;
    /// The function inputs, *default as ()*
    type Inputs;
    /// The function outputs, **WARN: does not support [`!`] or empty**
    type Output;
    /// The function declaration with a `Result<Output, std::error::Error>` as output value
    type FallibleDef;
}
