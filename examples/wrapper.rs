#[macro_use]
extern crate dynlib_derive;
extern crate dynlib;
extern crate libc;
use libc::{c_double, c_char, c_int};
use dynlib::wrapper::{Wrapper, WrapperApi};
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
    rust_fun_add_one: fn(i32) -> i32,
    c_fun_print_something_else: extern "C" fn(),
    c_fun_add_two: extern "C" fn(c_int) -> c_int,
    rust_i32_mut: &'a mut i32,
    //rust_i32: &'a i32,
    //c_int_mut: &'a mut c_int,
    //c_int: &'a c_int,
    //c_struct: &'a SomeData,
    //rust_str: &'a &'static str,
    c_const_char_ptr: * const c_char
}
/*
impl<'a> WrapperApi for Example<'a> {
    unsafe fn load ( lib : & :: dynlib :: lowlevel :: DynLib ) -> Result < Self , :: dynlib :: Error > {
        Ok ( Self { rust_fun_print_something : lib . symbol_cstr ( :: std :: ffi :: CStr :: from_bytes_with_nul_unchecked ( concat ! ( "rust_fun_print_something" , "\0" ) . as_bytes ( ) ) ) ? ,
            rust_fun_add_one : lib . symbol_cstr ( :: std :: ffi :: CStr :: from_bytes_with_nul_unchecked ( concat ! ( "rust_fun_add_one" , "\0" ) . as_bytes ( ) ) ) ? ,
            c_fun_print_something_else : lib . symbol_cstr ( :: std :: ffi :: CStr :: from_bytes_with_nul_unchecked ( concat ! ( "c_fun_print_something_else" , "\0" ) . as_bytes ( ) ) ) ? ,
            c_fun_add_two : lib . symbol_cstr ( :: std :: ffi :: CStr :: from_bytes_with_nul_unchecked ( concat ! ( "c_fun_add_two" , "\0" ) . as_bytes ( ) ) ) ? ,
            //rust_i32_mut : lib . symbol_cstr ( :: std :: ffi :: CStr :: from_bytes_with_nul_unchecked ( concat ! ( "rust_i32_mut" , "\0" ) . as_bytes ( ) ) ) ? ,
            //rust_i32 : lib . symbol_cstr ( :: std :: ffi :: CStr :: from_bytes_with_nul_unchecked ( concat ! ( "rust_i32" , "\0" ) . as_bytes ( ) ) ) ? ,
            //c_int_mut : lib . symbol_cstr ( :: std :: ffi :: CStr :: from_bytes_with_nul_unchecked ( concat ! ( "c_int_mut" , "\0" ) . as_bytes ( ) ) ) ? ,
            //c_int : lib . symbol_cstr ( :: std :: ffi :: CStr :: from_bytes_with_nul_unchecked ( concat ! ( "c_int" , "\0" ) . as_bytes ( ) ) ) ? ,
            //c_struct : lib . symbol_cstr ( :: std :: ffi :: CStr :: from_bytes_with_nul_unchecked ( concat ! ( "c_struct" , "\0" ) . as_bytes ( ) ) ) ? ,
            //rust_str : lib . symbol_cstr ( :: std :: ffi :: CStr :: from_bytes_with_nul_unchecked ( concat ! ( "rust_str" , "\0" ) . as_bytes ( ) ) ) ? ,
            c_const_char_ptr : lib . symbol_cstr ( :: std :: ffi :: CStr :: from_bytes_with_nul_unchecked ( concat ! ( "c_const_char_ptr" , "\0" ) . as_bytes ( ) ) ) ?
        })
    }
}*/


fn main(){
    //build path to the example library that covers most cases
    let mut lib_path = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    lib_path.extend(["target", "debug", "deps"].iter());
    lib_path.push(platform_file_name("example"));
    println!("Library path: {}", lib_path.to_str().unwrap());
    let mut wrapper: Wrapper<Example> = unsafe { Wrapper::open(lib_path)}.expect("Could not open library");
    println!("rust_i32_mut={}", unsafe {wrapper.rust_i32_mut()})
}