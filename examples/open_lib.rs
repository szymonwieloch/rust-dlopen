extern crate dlopen;
extern crate libc;
use dlopen::{Library, Symbol};
use libc::c_double;

const LIBRARY_NAME: &str = "libm.so.6";

fn main() {
    let lib = Library::open("libm.so.6").expect("Could not find libm.so.6");
    println!("Library opened");
    let cosine = unsafe {
        lib.symbol::<unsafe extern fn(c_double) -> c_double>("cos")
            .expect("Could not find \"cos\" symbol")
    };
    println!("Found symbol \"cos\"");
    let arg:c_double = 2.0;
    let result  = unsafe { cosine(arg) };
    println!("cos({}) = {}", arg, result);
}