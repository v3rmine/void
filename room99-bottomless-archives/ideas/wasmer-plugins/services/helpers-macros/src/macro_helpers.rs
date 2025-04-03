use std::{fmt, str::FromStr};

use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{parse::Parser, punctuated::Punctuated, Expr, Ident, Lit, Token};

pub fn number_from_expr<N>(expr: &Expr, panic_message: &str) -> N
where
    N: FromStr,
    N::Err: fmt::Display,
{
    match expr {
        Expr::Lit(l) => match &l.lit {
            Lit::Int(i) => i.base10_parse::<N>().unwrap(),
            _ => panic!("{}", panic_message),
        },
        _ => panic!("{}", panic_message),
    }
}

pub fn impl_ident_from_to(attrs: TokenStream) -> TokenStream {
    let attrs2 = proc_macro2::TokenStream::from(attrs);
    let parser = Punctuated::<Expr, Token![,]>::parse_terminated;
    let idents = parser.parse2(attrs2).expect("Could not parse macros attrs");
    if idents.len() != 3 {
        panic!("impl_call_with_tuple_from_to! macro takes 3 arguments")
    }

    let start = number_from_expr::<u8>(&idents[0], "First arg is not a number");
    let end = number_from_expr::<u8>(&idents[1], "Second arg is not a number") + 1;
    let macro_to_repeat = match &idents[2] {
        Expr::Path(m) => m,
        _ => panic!("Third arg is not a macro"),
    };

    let implements = (start..end).map(|max| {
        let idents = (0..max).map(|idx| Ident::new(&format!("A{}", idx + 1), Span::call_site()));
        quote::quote! { #macro_to_repeat!(#(#idents),*); }
    });

    quote::quote! {
        #(#implements)*
    }
    .into()
}
