#[macro_use]
extern crate const_cstr;
extern crate dlopen;
extern crate libc;

mod commons;

use commons::{example_lib_path, SomeData};
use dlopen::raw::Library;
use libc::{c_char, c_int};
use std::ffi::CStr;

fn main() {
    let lib_path = example_lib_path();
    let lib = Library::open(lib_path).expect("Could not open library");

    //get several symbols and play around
    let rust_fun_print_something: fn() =
        unsafe { lib.symbol_cstr(const_cstr!("rust_fun_print_something").as_cstr()) }.unwrap();
    rust_fun_print_something();

    let rust_fun_add_one: fn(i32) -> i32 =
        unsafe { lib.symbol_cstr(const_cstr!("rust_fun_add_one").as_cstr()) }.unwrap();
    println!(" 5+1={}", rust_fun_add_one(5));

    let c_fun_print_something_else: unsafe extern "C" fn() =
        unsafe { lib.symbol_cstr(const_cstr!("c_fun_print_something_else").as_cstr()) }.unwrap();
    unsafe { c_fun_print_something_else() };

    let c_fun_add_two: unsafe extern "C" fn(c_int) -> c_int =
        unsafe { lib.symbol_cstr(const_cstr!("c_fun_add_two").as_cstr()) }.unwrap();
    println!("2+2={}", unsafe { c_fun_add_two(2) });

    let rust_i32: &i32 = unsafe { lib.symbol_cstr(const_cstr!("rust_i32").as_cstr()) }.unwrap();
    println!("const rust i32 value: {}", rust_i32);

    let rust_i32_mut: &mut i32 =
        unsafe { lib.symbol_cstr(const_cstr!("rust_i32_mut").as_cstr()) }.unwrap();
    println!("mutable rust i32 value: {}", rust_i32_mut);

    *rust_i32_mut = 55;
    //for a change use pointer to obtain its value
    let rust_i32_ptr: *const i32 =
        unsafe { lib.symbol_cstr(const_cstr!("rust_i32_mut").as_cstr()) }.unwrap();
    println!("after change: {}", unsafe { *rust_i32_ptr });

    //the same with C
    let c_int: &c_int = unsafe { lib.symbol_cstr(const_cstr!("c_int").as_cstr()) }.unwrap();
    println!("c_int={}", c_int);

    //now static c struct

    let c_struct: &SomeData =
        unsafe { lib.symbol_cstr(const_cstr!("c_struct").as_cstr()) }.unwrap();
    println!(
        "c struct first: {}, second:{}",
        c_struct.first,
        c_struct.second
    );

    //let's play with strings

    let rust_str: &&str = unsafe { lib.symbol_cstr(const_cstr!("rust_str").as_cstr()) }.unwrap();
    println!("Rust says: {}", *rust_str);

    let c_const_char_ptr: *const c_char =
        unsafe { lib.symbol_cstr(const_cstr!("c_const_char_ptr").as_cstr()) }.unwrap();
    let converted = unsafe { CStr::from_ptr(c_const_char_ptr) }
        .to_str()
        .unwrap();
    println!("And now C says: {}", converted);
}
