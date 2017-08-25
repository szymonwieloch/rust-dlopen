#[macro_use]
extern crate dynlib_derive;
#[macro_use]
extern crate dynlib;
extern crate libc;
#[macro_use]
extern crate const_cstr;
use dynlib::symbor::{Library, PtrOrNull, PtrOrNullMut, Ref, RefMut, Symbol, SymBorApi};
use dynlib::utils::platform_file_name;
use libc::{c_char};
use std::env;
use std::path::PathBuf;

#[derive(SymBorApi)]
struct ExampleApi<'a>{
    pub rust_fun_print_something: Symbol<'a, fn()>,
    pub rust_i32_mut: RefMut<'a, i32>,
    pub c_const_char_ptr: PtrOrNull<'a, * const c_char>,
    pub optional_function: Option<Symbol<'a, fn()->i32>>,
    #[dynlib_name="rust_fun_add_one"]
    pub i_want_other_name: Symbol<'a, fn(i32)->i32>,
}

fn main() {
    //build path to the example library that covers most cases
    let mut lib_path = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    lib_path.extend(["target", "debug", "deps"].iter());
    lib_path.push(platform_file_name("example"));
    println!("Library path: {}", lib_path.to_str().unwrap());
    let lib = Library::open(lib_path).expect("Could not open library");

    //mut is needed because we want to use rust_i32_mut
    let mut api = unsafe {ExampleApi::load(&lib)}.expect("Could not load the API");

    //now we can play:

    //brackets are used in Rust to distinguish between field and method.
    (api.rust_fun_print_something)();
    *api.rust_i32_mut += 12;
    assert!(!api.c_const_char_ptr.is_null());
    if let Some(fun) = api.optional_function {
        let _result = fun();
    }
    assert_eq!((api.i_want_other_name)(5), 6);
}