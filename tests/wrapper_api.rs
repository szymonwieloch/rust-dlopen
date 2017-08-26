extern crate dynlib;
#[macro_use]
extern crate dynlib_derive;
extern crate libc;
#[macro_use]
extern crate const_cstr;
use dynlib::wrapper::{Container, WrapperApi};
use libc::{c_int, c_char};
use std::ffi::CStr;

use std::io::Write;
mod commons;
use commons::{example_lib_path, SomeData};

macro_rules! println_stderr(
    ($($arg:tt)*) => { {
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
);

#[derive(WrapperApi)]
struct Api<'a> {
    rust_fun_print_something: fn(),
    rust_fun_add_one: fn(arg: i32) -> i32,
    c_fun_print_something_else: unsafe extern "C" fn(),
    c_fun_add_two: unsafe extern "C" fn(arg: c_int) -> c_int,
    rust_i32: &'a i32,
    rust_i32_mut: &'a mut i32,
    #[dynlib_name="rust_i32_mut"]
    rust_i32_ptr: * const i32,
    c_int: &'a c_int,
    c_struct: &'a SomeData,
    rust_str: &'a &'static str,
    c_const_char_ptr: * const c_char
}

//those methods won't be generated
impl<'a> Api<'a> {
    fn rust_i32_ptr(&self) -> * const i32 {self.rust_i32_ptr}

    fn c_const_str(&self) -> &CStr {
        unsafe {CStr::from_ptr(self.c_const_char_ptr)}
    }
}

//#[cfg(not(any(target_os="macos", target_os="ios")))]
#[test]
fn open_play_close_wrapper_api(){
    let lib_path = example_lib_path();
    let mut cont: Container<Api> = unsafe{ Container::load(lib_path)}.expect("Could not open library or load symbols");

    cont.rust_fun_print_something(); //should not crash
    assert_eq!(cont.rust_fun_add_one(5), 6);
    unsafe{ cont.c_fun_print_something_else()}; //should not crash
    println_stderr!("something else call OK");
    assert_eq!(unsafe{cont.c_fun_add_two(2)}, 4);
    println_stderr!("add_two called OK");
    assert_eq!(43, *cont.rust_i32());
    println_stderr!("obtaining const data OK");
    assert_eq!(42, *cont.rust_i32_mut_mut());
    println_stderr!("obtaining mutable data OK");
    *cont.rust_i32_mut_mut() = 55; //should not crash
    println_stderr!("assigning mutable data OK");
    assert_eq!(55, unsafe{*cont.rust_i32_ptr()});
    println_stderr!("obtaining pointer OK");
    //the same with C
    assert_eq!(45, *cont.c_int());
    println_stderr!("obtaining C data OK");
    //now static c struct

    assert_eq!(1, cont.c_struct().first);
    assert_eq!(2, cont.c_struct().second);
    println_stderr!("obtaining C structure OK");
    //let's play with strings

    assert_eq!("Hello!", *cont.rust_str());
    println_stderr!("obtaining str OK");
    let converted = cont.c_const_str().to_str().unwrap();
    assert_eq!(converted, "Hi!");
    println_stderr!("obtaining C string OK");

    //It turns out that there is a bug in rust.
    //On OSX calls to dynamic libraries written in Rust causes segmentation fault
    //please note that this ia a problem with the example library, not this library
    //maybe converting the example library into cdylib would help?
    //https://github.com/rust-lang/rust/issues/28794
    //#[cfg(any(target_os="macos", target_os="ios"))]
    //::std::mem::forget(lib);
}
