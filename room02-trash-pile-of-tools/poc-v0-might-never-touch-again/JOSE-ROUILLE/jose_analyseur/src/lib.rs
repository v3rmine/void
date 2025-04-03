#![forbid(unsafe_code)]
#![deny(
    clippy::complexity,
    clippy::perf,
    clippy::checked_conversions,
    clippy::filter_map_next
)]
#![warn(
    clippy::style,
    clippy::map_unwrap_or,
    clippy::missing_const_for_fn,
    clippy::use_self,
    future_incompatible,
    rust_2018_idioms,
    nonstandard_style
)]
// with configurable values
#![warn(
    clippy::blacklisted_name,
    clippy::cognitive_complexity,
    clippy::disallowed_method,
    clippy::fn_params_excessive_bools,
    clippy::struct_excessive_bools,
    clippy::too_many_lines,
    clippy::type_complexity,
    clippy::trivially_copy_pass_by_ref,
    clippy::type_repetition_in_bounds,
    clippy::unreadable_literal
)]
#![deny(clippy::wildcard_imports)]
// crate-specific exceptions:
#![allow()]

pub mod errors;
pub mod types;
