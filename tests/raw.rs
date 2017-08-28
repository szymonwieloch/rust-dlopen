extern crate dynlib;
extern crate libc;
#[macro_use]
extern crate const_cstr;
use dynlib::raw::Library;
use libc::{c_int, c_char};
use std::ffi::CStr;

use std::io::Write;
mod commons;
use commons::{example_lib_path, SomeData};

//#[cfg(not(any(target_os="macos", target_os="ios")))]
#[test]
fn open_play_close_raw(){
    let lib_path = example_lib_path();
    let lib = Library::open(lib_path).expect("Could not open library");
    let rust_fun_print_something: fn() = unsafe { lib.symbol_cstr(const_cstr!("rust_fun_print_something").as_cstr())}.unwrap();
    rust_fun_print_something(); //should not crash
    let rust_fun_add_one: fn(i32) -> i32 = unsafe { lib.symbol_cstr(const_cstr!("rust_fun_add_one").as_cstr())}.unwrap();
    assert_eq!(rust_fun_add_one(5), 6);
    let c_fun_print_something_else: unsafe extern "C" fn() = unsafe { lib.symbol_cstr(const_cstr!("c_fun_print_something_else").as_cstr())}.unwrap();
    unsafe{ c_fun_print_something_else()}; //should not crash
    let c_fun_add_two: unsafe extern "C" fn(c_int) -> c_int = unsafe { lib.symbol_cstr(const_cstr!("c_fun_add_two").as_cstr())}.unwrap();
    assert_eq!(unsafe{c_fun_add_two(2)}, 4);
    let rust_i32: & i32 = unsafe { lib.symbol_cstr(const_cstr!("rust_i32").as_cstr())}.unwrap();
    assert_eq!(43, *rust_i32);
    let rust_i32_mut: &mut i32 = unsafe { lib.symbol_cstr(const_cstr!("rust_i32_mut").as_cstr())}.unwrap();
    assert_eq!(42, *rust_i32_mut);
    *rust_i32_mut = 55; //should not crash
    //for a change use pointer to obtain its value
    let rust_i32_ptr: *const i32 = unsafe { lib.symbol_cstr(const_cstr!("rust_i32_mut").as_cstr())}.unwrap();
    assert_eq!(55, unsafe{*rust_i32_ptr});
    //the same with C
    let c_int: & c_int = unsafe { lib.symbol_cstr(const_cstr!("c_int").as_cstr())}.unwrap();
    assert_eq!(45, *c_int);
    //now static c struct

    let c_struct: & SomeData = unsafe { lib.symbol_cstr(const_cstr!("c_struct").as_cstr())}.unwrap();
    assert_eq!(1, c_struct.first);
    assert_eq!(2, c_struct.second);
    //let's play with strings

    let  rust_str: &&str = unsafe { lib.symbol_cstr(const_cstr!("rust_str").as_cstr())}.unwrap();
    assert_eq!("Hello!", *rust_str);
    let c_const_char_ptr: * const c_char = unsafe { lib.symbol_cstr(const_cstr!("c_const_char_ptr").as_cstr())}.unwrap();
    let converted = unsafe{CStr::from_ptr(c_const_char_ptr)}.to_str().unwrap();
    assert_eq!(converted, "Hi!");

    //It turns out that there is a bug in rust.
    //On OSX calls to dynamic libraries written in Rust causes segmentation fault
    //please note that this ia a problem with the example library, not with dynlib
    //https://github.com/rust-lang/rust/issues/28794
    #[cfg(any(target_os="macos", target_os="ios"))]
    ::std::mem::forget(lib);
}
