#[macro_use]
extern crate dlopen_derive;
extern crate dlopen;
extern crate libc;
use dlopen::{DlOpen, LibraryInterface, Wrapper};
use libc::{c_double, c_char};
const LIB_NAME: &str = "libm.so.6";

#[derive(LibraryInterface)]
struct LibM {
    cos: unsafe extern fn (arg: c_double) -> c_double,
    tan: * const c_char
}

type LibMWrapper = Wrapper<LibM>;


fn main() {
    //let lib = DlOpen::open(LIB_NAME).expect(&format!("Could not open {}", LIB_NAME));
    //let libm = unsafe {LibM::load(&lib)}.expect("Could not load the library");

    let libm = unsafe { LibMWrapper::load(LIB_NAME)}.expect("Could not load libm");

    let arg = 2.0;
    let result = unsafe { (libm.cos)(arg) };
    println!("cos({}) = {}", arg, result);
    println!("*tan={}", unsafe {*libm.tan});
    println!("cos wrapper={}", unsafe {libm.cos(arg)});
}