#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]

//! Macro library to help with functions types and macros

use proc_macro::TokenStream;

mod struct_helpers;

/// Impl `wasm_plugins::abi::FuncInfo` for the type
///
/// # Examples
/// ## Without custom ident
/// ```ignore
/// # use helpers_macros::InnerFunctionDetails;
/// #[derive(InnerFunctionDetails)]
/// struct Adder(pub fn(i32, i32) -> i64);
///
/// fn main() {
///   assert!(Adder::FUNC_ID, "adder");
/// }
/// ```
///
/// ## With custom ident
/// ```ignore
/// # use helpers_macros::InnerFunctionDetails;
/// #[derive(InnerFunctionDetails)]
/// #[details(function_id = "some")]
/// struct Subber(pub fn(i32, i32) -> i64);
///
/// fn main() {
///   assert!(Subber::FUNC_ID, "some");
/// }
/// ```
#[proc_macro_derive(InnerFunctionDetails, attributes(details))]
pub fn impl_inner_function_details(item: TokenStream) -> TokenStream {
    struct_helpers::define_inner_function_details(item)
}

mod macro_helpers;

/// Call macro multiple times for each number in range of params, with
/// new idents. It can be used to implement generic tuples.
///
/// # Example
/// ```ignore
/// impl_ident_from_to!(0, 5, <macro_to_call_with(A1, A2, A3, A4, A5)>);
/// ```
#[proc_macro]
pub fn impl_ident_from_to(attrs: TokenStream) -> TokenStream {
    macro_helpers::impl_ident_from_to(attrs)
}
