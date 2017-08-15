#[macro_use]
extern crate dlopen_derive;
extern crate dlopen;
extern crate libc;
use libc::{c_double, c_char};
use dlopen::{Symbol, LibraryWrapper, DlDrop as Whatever, DlOpen};
use std::ffi::CStr;

#[derive(LibraryWrapper)]
struct LibMWrapper{
    cos: unsafe extern fn(c_double)->c_double,
    sin: unsafe extern fn(c_double)->c_double,
    #[dlopen_allow_null]
    #[dlopen_name = "tan"]
    something: * const c_char,
    ctg: unsafe extern fn(c_double)->c_double,
    #[dlopen_drop]
    drop: Whatever,
}

const LIB_NAME: &str = "libm.so.6";

fn main(){
    println!("Loading library");
    let libm = unsafe {LibMWrapper::load("libm.so.6")}.expect("Could not load library");
    println!("Library loaded, calling sin()");
    let arg = 1.0;
    println!("sin -> {:?}", libm.sin);
    let result = unsafe {(libm.sin)(arg)};
    println!("sin({}) = {}", arg, result);
}