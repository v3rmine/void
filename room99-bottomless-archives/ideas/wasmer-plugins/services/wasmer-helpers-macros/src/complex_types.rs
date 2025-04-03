use proc_macro::TokenStream;
use syn::Item as SynItem;

use quote::quote;

pub fn complex_export(attr: TokenStream, tokens: TokenStream) -> TokenStream {
    if attr.is_empty() {
        panic!("complex_plugin require the function type as attribute")
    };

    let tokens2 = proc_macro2::TokenStream::from(tokens);
    let parsed_tokens = syn::parse2::<SynItem>(tokens2).expect("Failed to parse tokens");

    let attr2 = proc_macro2::TokenStream::from(attr);
    let func_type = syn::parse2::<syn::Type>(attr2).expect("Failed to parse attr");

    let func = match parsed_tokens {
        SynItem::Fn(func) => func,
        _ => panic!("Only functions are currently supported"),
    };

    match &func.vis {
        syn::Visibility::Public(_) => (),
        _ => panic!("Fns marked with complex_plugin mut be public"),
    }

    let func_ident = &func.sig.ident;
    let func_attrs_with_types = &func.sig.inputs;
    let func_attrs_count = func_attrs_with_types.len();
    let func_body = &func.block;

    let ret = match func_attrs_count {
        0 => quote! {
            #[allow(unsafe_code)]
            #[no_mangle]
            pub fn #func_ident() -> i32 {
                let ret = #func_type(|| { #func_body }).0();
                let serialized = ::wasm_plugins::interface::convert_ret(ret);
                let len = serialized.len() as u32;
                unsafe {
                    ::std::ptr::write(1 as _, len);
                }
                serialized.as_ptr() as i32
            }
        },
        1 => quote! {
            #[allow(unsafe_code)]
            #[no_mangle]
            pub fn #func_ident(ptr: i32, len: u32) -> i32 {
                let mut bytes: &[u8] = unsafe { ::std::slice::from_raw_parts(ptr as _, len as _) };
                let arg = ::wasm_plugins::interface::convert_slice(bytes);
                let ret = #func_type(|#func_attrs_with_types| { #func_body }).0(arg);
                let serialized = ::wasm_plugins::interface::convert_ret(ret);
                let len = serialized.len() as u32;
                unsafe {
                    ::std::ptr::write(1 as _, len);
                }
                serialized.as_ptr() as i32
            }
        },
        _ => panic!("Fns marked with complex_plugin can only take 0 or 1 argument"),
    };

    ret.into()
}
