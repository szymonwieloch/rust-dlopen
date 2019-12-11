#![recursion_limit = "128"]

extern crate proc_macro;

mod api;
mod common;
mod multi_api;
mod wrapper;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

use self::api::impl_library_api;
use self::multi_api::impl_wrapper_multi_api;
use self::wrapper::impl_wrapper_api;

#[proc_macro_derive(WrapperApi, attributes(dlopen_name, dlopen_allow_null))]
pub fn wrapper_api(input: TokenStream) -> TokenStream {
    // Parse the string representation
    let ast = parse_macro_input!(input as DeriveInput);

    // Build the impl
    let gen = impl_wrapper_api(&ast);

    // Return the generated impl
    TokenStream::from(gen)
}

#[proc_macro_derive(WrapperMultiApi)]
pub fn wrapper_multi_api(input: TokenStream) -> TokenStream {
    // Parse the string representation
    let ast = parse_macro_input!(input as DeriveInput);

    // Build the impl
    let gen = impl_wrapper_multi_api(&ast);

    // Return the generated impl
    TokenStream::from(gen)
}

#[proc_macro_derive(SymBorApi, attributes(dlopen_name))]
pub fn library_api(input: TokenStream) -> TokenStream {
    // Parse the string representation
    let ast = parse_macro_input!(input as DeriveInput);

    // Build the impl
    let gen = impl_library_api(&ast);

    // Return the generated impl
    TokenStream::from(gen)
}
