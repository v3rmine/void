use proc_macro::TokenStream;
use quote::quote;
use syn::{punctuated::Punctuated, FnArg, Item as SynItem};

pub fn simple_export(attr: TokenStream, tokens: TokenStream) -> TokenStream {
    if attr.is_empty() {
        panic!("simple_export require the function type as attribute")
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
        _ => panic!("Fns marked with simple_export must be public"),
    }

    let func_ident = func.sig.ident;
    let func_attrs_with_types = &func.sig.inputs;
    let func_attrs = func
        .sig
        .inputs
        .iter()
        .filter_map(|arg| match arg {
            FnArg::Typed(typed) => Some(&typed.pat),
            FnArg::Receiver(_) => None,
        })
        .collect::<Punctuated<&Box<syn::Pat>, syn::token::Comma>>();
    let func_ret = func.sig.output;
    let func_body = func.block;

    (quote! {
        #[allow(unsafe_code)]
        #[no_mangle]
        pub fn #func_ident(#func_attrs_with_types) #func_ret {
            #func_type(|#func_attrs| { #func_body }).0(#func_attrs)
        }
    })
    .into()
}
