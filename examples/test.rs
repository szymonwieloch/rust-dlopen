#[macro_use]
extern crate dlopen_derive;
extern crate dlopen;
extern crate libc;
use libc::c_double;
use dlopen::{Symbol, Library, LibraryApi, FromRawPointer};
use std::convert::From;

const LIB_NAME: &str = "libm.so.6";

struct Test<'a> {
    pub sin: Symbol<'a, unsafe extern fn(c_double) -> c_double>,
    //pub cos: Symbol<'a, unsafe extern fn(c_double) -> c_double>,
}

fn main () {
    let lib = Library::open(LIB_NAME).expect(&format!("Could not open {}", LIB_NAME));

    let raw_sin = unsafe { lib.raw("sin").unwrap()};
    let raw_cos = unsafe { lib.raw("cos").unwrap()};
    let raw_tan = unsafe { lib.raw("tan").unwrap()};

    let var: Symbol<unsafe extern fn(c_double) -> c_double> = Symbol::from(raw_tan);

    let t = Test {
        sin : raw_sin.into()
        //cos: raw_cos.into()
    };


}