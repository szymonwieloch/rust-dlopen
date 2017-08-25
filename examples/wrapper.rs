#[macro_use]
extern crate dynlib_derive;
extern crate dynlib;
extern crate libc;
use libc::{c_double, c_char, c_int};
use dynlib::wrapper::{Container, WrapperApi};
use dynlib::utils::platform_file_name;
use std::ffi::CStr;
use std::env;
use std::path::PathBuf;

#[repr(C)]
pub struct SomeData {
    first: c_int,
    second: c_int
}

#[derive(WrapperApi)]
struct Example<'a>{
    rust_fun_print_something: fn(),
    rust_fun_add_one: fn(arg: i32) -> i32,
    c_fun_print_something_else: extern "C" fn(),
    c_fun_add_two: extern "C" fn(arg: c_int) -> c_int,
    c_fun_variadic: extern "C" fn(txt: * const c_char, ...),
    rust_i32_mut: &'a mut i32,
    rust_i32: &'a i32,
    c_int_mut: &'a mut c_int,
    c_int: &'a c_int,
    c_struct: &'a SomeData,
    rust_str: &'a &'static str,
    c_const_char_ptr: * const c_char,
    #[dynlib_name="c_const_char_ptr"]
    #[dynlib_allow_null]
    null_ptr: * const c_char
}

//WrapperApi on purpose won't generate accessors for pointers
//implement it here manually
impl<'a> Example<'a>{
    pub fn c_str(&self) -> &CStr {
        unsafe {CStr::from_ptr(self.c_const_char_ptr)}
    }
}



fn main(){
    //build path to the example library that covers most cases
    let mut lib_path = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    lib_path.extend(["target", "debug", "deps"].iter());
    lib_path.push(platform_file_name("example"));
    println!("Library path: {}", lib_path.to_str().unwrap());

    //her actually sart the example
    let mut wrapper: Container<Example> = unsafe { Container::open(lib_path)}.expect("Could not open library");
    wrapper.rust_fun_print_something();
    wrapper.c_fun_print_something_else();
    println!("rust_i32_mut={}", unsafe {wrapper.rust_i32_mut()});
    println!("4+1={}", wrapper.rust_fun_add_one(4));
    println!("6+2={}", wrapper.c_fun_add_two(6));
}