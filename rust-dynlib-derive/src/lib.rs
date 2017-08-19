#![recursion_limit="128"]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

mod api;
mod wrapper;
mod common;
mod interface;



use proc_macro::TokenStream;
use api::impl_library_api;
use wrapper::impl_library_wrapper;
use interface::impl_library_interface;

#[proc_macro_derive(LibraryWrapper, attributes(dlopen_name, dlopen_drop, dlopen_allow_null))]
pub fn library_wrapper(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = impl_library_wrapper(&ast);

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

#[proc_macro_derive(LibraryInterface, attributes(dlopen_name))]
pub fn library_interface(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = impl_library_interface(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}