use super::super::raw::{Library};
use libc::{c_int, c_char};

use std::ffi::CStr;
use super::{example_lib_path, SomeData};

use std::io::Write;

macro_rules! println_stderr(
    ($($arg:tt)*) => { {
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
);

//#[cfg(not(any(target_os="macos", target_os="ios")))]
#[test]
fn open_play_close(){
    let lib_path = example_lib_path();
    let lib = Library::open(lib_path).expect("Could not open library");
    let rust_fun_print_something: fn() = unsafe { lib.symbol_cstr(const_cstr!("rust_fun_print_something").as_cstr())}.unwrap();
    rust_fun_print_something(); //should not crash
    let rust_fun_add_one: fn(i32) -> i32 = unsafe { lib.symbol_cstr(const_cstr!("rust_fun_add_one").as_cstr())}.unwrap();
    assert_eq!(rust_fun_add_one(5), 6);

    let c_fun_print_something_else: unsafe extern "C" fn() = unsafe { lib.symbol_cstr(const_cstr!("c_fun_print_something_else").as_cstr())}.unwrap();
    unsafe{ c_fun_print_something_else()}; //should not crash
    println_stderr!("something else call OK");
    let c_fun_add_two: unsafe extern "C" fn(c_int) -> c_int = unsafe { lib.symbol_cstr(const_cstr!("c_fun_add_two").as_cstr())}.unwrap();
    assert_eq!(unsafe{c_fun_add_two(2)}, 4);
    println_stderr!("add_two called OK");
    let rust_i32: & i32 = unsafe { lib.symbol_cstr(const_cstr!("rust_i32").as_cstr())}.unwrap();
    assert_eq!(43, *rust_i32);
    println_stderr!("obtaining const data OK");
    let rust_i32_mut: &mut i32 = unsafe { lib.symbol_cstr(const_cstr!("rust_i32_mut").as_cstr())}.unwrap();
    assert_eq!(42, *rust_i32_mut);
    println_stderr!("obtaining mutable data OK");
    *rust_i32_mut = 55; //should not crash
    println_stderr!("assigning mutable data OK");
    //for a change use pointer to obtain its value
    let rust_i32_ptr: *const i32 = unsafe { lib.symbol_cstr(const_cstr!("rust_i32_mut").as_cstr())}.unwrap();
    assert_eq!(55, unsafe{*rust_i32_ptr});
    println_stderr!("obtaining pointer OK");
    //the same with C
    let c_int: & c_int = unsafe { lib.symbol_cstr(const_cstr!("c_int").as_cstr())}.unwrap();
    assert_eq!(45, *c_int);
    println_stderr!("obtaining C data OK");
    //now static c struct

    let c_struct: & SomeData = unsafe { lib.symbol_cstr(const_cstr!("c_struct").as_cstr())}.unwrap();
    assert_eq!(1, c_struct.first);
    assert_eq!(2, c_struct.second);
    println_stderr!("obtaining C structure OK");
    //let's play with strings

    let  rust_str: &&str = unsafe { lib.symbol_cstr(const_cstr!("rust_str").as_cstr())}.unwrap();
    assert_eq!("Hello!", *rust_str);
    println_stderr!("obtaining str OK");
    let c_const_char_ptr: * const c_char = unsafe { lib.symbol_cstr(const_cstr!("c_const_char_ptr").as_cstr())}.unwrap();
    let converted = unsafe{CStr::from_ptr(c_const_char_ptr)}.to_str().unwrap();
    assert_eq!(converted, "Hi!");
    println_stderr!("obtaining C string OK");

    //It turns out that there is a bug in rust.
    //On OSX calls to dynamic libraries written in Rust causes segmentation fault
    //please note that this ia a problem with the example library, not this library
    //maybe converting the example library into cdylib would help?
    //https://github.com/rust-lang/rust/issues/28794
    #[cfg(any(target_os="macos", target_os="ios"))]
    ::std::mem::forget(lib);
}
