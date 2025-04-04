//! High-level library providing a trait that, once implemented, hides the complexity of
//! Wit bindings.
//!
//! There are a few problems at the moment:
//!
//! - It's not possible for a lib to implement a component interface, as it has to be the final
//! binary implementing the component's interface; see also
//! https://github.com/bytecodealliance/cargo-component/issues/75.
//!
//! - Because of that, I've had to put most of the code, including the whole `impl Interface for X`
//! block, in the macro body. It's ugly and not practical for maintainability purposes.
/// Implements a command for a given type, assuming the type implements the `TrinityCommand` trait.
#[macro_export]
macro_rules! impl_transformer {
    ($ident:ident) => {
        const _: () = {
            // 
            bindings::export!($ident);
        };
    };
}
