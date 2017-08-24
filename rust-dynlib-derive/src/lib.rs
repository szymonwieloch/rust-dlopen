#![recursion_limit="128"]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

mod api;
mod multi_api;
mod wrapper;
mod common;



use proc_macro::TokenStream;
use api::impl_library_api;
use wrapper::impl_wrapper_api;
use multi_api::impl_wrapper_multi_api;

#[proc_macro_derive(WrapperApi, attributes(dynlib_name, dynlib_allow_null))]
pub fn wrapper_api(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = impl_wrapper_api(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}

#[proc_macro_derive(WrapperMultiApi)]
pub fn wrapper_multi_api(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = impl_wrapper_multi_api(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}

#[proc_macro_derive(LibraryApi, attributes(dynlib_name))]
pub fn library_api(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = impl_library_api(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}
