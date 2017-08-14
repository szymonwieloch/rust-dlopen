#[macro_use]
extern crate dlopen_derive;
extern crate dlopen;
extern crate libc;
use libc::c_double;
use dlopen::{Symbol, Library, LibraryApi, FromRawPointer, DlOpen, DlDrop};
use std::convert::From;

const LIB_NAME: &str = "libm.so.6";

struct Test<'a> {
    pub sin: Symbol<'a, unsafe extern fn(c_double) -> c_double>,
    //pub cos: Symbol<'a, unsafe extern fn(c_double) -> c_double>,
}

fn main () {

}