extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(IntoResponse)]
pub fn into_response_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    into_response_derive_impl(input)
}

fn into_response_derive_impl(input: DeriveInput) -> TokenStream {
    let name = &input.ident;

    let expanded = quote! {
        impl IntoResponse for #name {
            fn into_response(self) -> Response {
                BaseResponse::create(self)
            }
        }
    };

    TokenStream::from(expanded)
}
