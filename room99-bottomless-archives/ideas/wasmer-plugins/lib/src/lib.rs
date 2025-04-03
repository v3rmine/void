#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]

//! Low level library to help with creation, definition and usage of WASM plugins
//!
//! **WARNING!**: This library is currently a **proof of concept** and so **is not intended
//! for production usage**. It has not been battle tested and only support [`WASMER`]
//! for the moment
//!
//! # Getting started with wasm-plugins
//!
//! ## Designing an ABI
//! wasm-plugins is centered around methods defined in a standard interface, they'll
//! be typed in the plugin and in the runner.
//!
//! (*The ABI must support being build as wasm*)
//!
//! ### Example
//! ```toml
//! wasm-plugins = "0.1"
//! ```
//!
//! ## Creating a plugin
//! The plugin will be a standalone .wasm file that will be run by the runner.
//! It must be typed using a defined ABI.
//!
//! ### Example
//! ```toml
//! wasm-plugins = { version = "0.1", features = ["wasmer-backend"] }
//! ```
//!
//! ## Creating a runnner
//! The runner will run the wasm plugins and it must use the defined ABI too.
//!
//! ### Example
//! ```toml
//! wasm-plugins = { version = "0.1", features = ["wasmer-rt"] }
//! ```
//!
//! # Feature flags
//! - `wasmer-backend`: Enable the exports macros to define methods in the plugins
//! - `wasmer-rt-sys`: Enable the sys runtime of [`wasmer`] for the runner
//! - `wasmer-rt-js`: Enable the js runtime of [`wasmer`] for the runner
//!
//! *Note: `wasmer-rt-sys` and `wasmer-rt-js` are mutually exclusives.*
//!
//! [`wasmer`]: https://wasmer.io/

// @TODO: Make wasmer-rt-sys and wasmer-rt-js mutually exclusive with compiler panic

#[cfg(feature = "wasmer-rt")]
pub mod runtime {
    //! All the methods usefull to make a runner
    pub use wasmer_helpers::{
        get_func_complex_with_args, get_func_complex_without_args, get_func_simple,
        instanciate_simple, CallWithTuple, InstanceBuilder,
    };
}

#[cfg(feature = "wasmer-backend")]
pub mod plugin {
    //! The macros to export the plugins methods
    pub use wasmer_helpers_macros::{complex_export, simple_export};
}

pub mod abi {
    //! The tools to help you type your ABI
    pub use helpers::{FuncInfo, InnerFunctionDetails};
}

pub mod interface {
    //! Tools to interface rust types and binary
    //!
    //! - binary to rust types: [`convert_ret`]
    //! - rust types to binary: [`convert_slice`]
    //!
    //! *NOTE: It use [`serde`] and [`bincode`] internally.*
    //!
    //! [`serde`]: https://docs.rs/serde
    //! [`bincode`]: https://docs.rs/bincode
    pub use helpers::{convert_ret, convert_slice};
}
