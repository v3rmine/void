use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, Type, Visibility};

#[derive(FromDeriveInput, Debug)]
#[darling(attributes(details), forward_attrs(allow, doc, cfg))]
struct InnerFunctionDetails {
    pub ident: syn::Ident,
    #[darling(default)]
    pub function_id: Option<String>,
}

pub fn define_inner_function_details(item: TokenStream) -> TokenStream {
    let item2 = proc_macro2::TokenStream::from(item.clone());
    let input_stream = parse_macro_input!(item);
    let parsed_item = syn::parse2::<syn::Item>(item2).expect("Cannot parse item");

    let struct_details = InnerFunctionDetails::from_derive_input(&input_stream)
        .expect("Wrong params for InnerFunctionDetails");

    if let syn::Item::Struct(parsed_item) = parsed_item {
        let func_ident = struct_details.ident;
        let func_id = struct_details.function_id.unwrap_or_else(|| {
            func_ident
                .to_string()
                .chars()
                .enumerate()
                .flat_map(|(idx, c)| match idx {
                    0 => [Some(c.to_ascii_lowercase()), None],
                    _ => {
                        if c.is_uppercase() {
                            [Some('_'), Some(c.to_ascii_lowercase())]
                        } else {
                            [Some(c), None]
                        }
                    }
                })
                .flatten()
                .collect()
        });

        let func = match parsed_item.fields {
            syn::Fields::Unnamed(fields) => {
                if fields.unnamed.is_empty() {
                    panic!("#[derive(InnerFunctionDetails)] need one unnamed field");
                }
                let first_field = &fields.unnamed[0];

                match first_field.vis {
                    Visibility::Public(_) => (),
                    _ => panic!("#[derive(InnerFunctionDetails)] unnamed field must be public"),
                }

                match &first_field.ty {
                    Type::BareFn(func) => func.clone(),
                    _ => panic!("#[derive(InnerFunctionDetails)] unnamed field must be a function"),
                }
            }
            _ => panic!("#[derive(InnerFunctionDetails)] only supports unnamed fields"),
        };

        let func_inputs = func
            .inputs
            .iter()
            .map(|arg| &arg.ty)
            .collect::<Punctuated<&Type, syn::token::Comma>>();
        let func_ret = match func.output {
            syn::ReturnType::Default => {
                panic!("@TODO #[derive(InnerFunctionDetails)] cannot return nothing")
            }
            syn::ReturnType::Type(_, ref ret_type) => ret_type,
        };

        (quote! {
            impl ::std::convert::From<#func_ident> for #func {
                fn from(e: #func_ident) -> Self {
                    e.0
                }
            }

            impl ::wasm_plugins::abi::FuncInfo for #func_ident {
                const FUNC_ID: &'static str = #func_id;
                type Inputs = (#func_inputs);
                type Output = #func_ret;
                type FallibleDef = fn((#func_inputs)) -> ::std::result::Result<#func_ret, ::std::boxed::Box<dyn ::std::error::Error>>;
            }
        })
        .into()
    } else {
        panic!("#[derive(InnerFunctionDetails)] only supports structs");
    }
}
