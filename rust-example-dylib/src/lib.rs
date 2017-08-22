//!An example dynamically loadable library.
//!
//! This crate creates a dynamic library that can be used for testing purposes.
//! It exports multiple symbols with different types and abis.
//! It's main purpose is to be used in tests of dynlib crate.

#![allow(non_upper_case_globals)]
extern crate libc;
use libc::{c_int, c_char};


//FUNCTIONS
#[no_mangle]
pub fn rust_fun_print_something() {
    println!("something");
}

#[no_mangle]
pub fn rust_fun_add_one(arg: i32) -> i32 {
    arg + 1
}

#[no_mangle]
pub extern "C" fn c_fun_print_something_else() {
    println!("something else");
}

#[no_mangle]
pub extern "C" fn c_fun_add_two(arg: c_int) -> c_int {
    arg + 2
}

#[no_mangle]
pub extern "C" fn c_fun_variadic(txt: * const c_char) {
    //pretend to be variadic - impossible to do in Rust code
}

//STATIC DATA
#[no_mangle]
pub static mut rust_i32_mut: i32 = 42;
#[no_mangle]
pub static rust_i32: i32 = 43;

#[no_mangle]
pub static mut c_int_mut: c_int = 44;
#[no_mangle]
pub static c_int: c_int = 45;

#[repr(C)]
pub struct SomeData {
    first: c_int,
    second: c_int
}

#[no_mangle]
pub static c_struct: SomeData = SomeData{first: 1, second: 2};

//STATIC STRINGS

//exporting str directly is not so easy - it is not Sized!
//you can only export a reference to str and this requires double dereference
#[no_mangle]
pub static rust_str: &str = "Hello!";

#[no_mangle]
pub static c_const_char_ptr: [u8; 4] = [b'H', b'i', b'!', 0];






