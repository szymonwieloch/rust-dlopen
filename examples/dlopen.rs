extern crate dlopen;
extern crate libc;
use dlopen::{DlOpen};
use libc::{c_double};
const LIB_NAME: &str = "libm.so.6";

fn main() {
    let libm = DlOpen::open(LIB_NAME).expect(&format!("Could not open {}", LIB_NAME));
    let cos: unsafe fn(c_double) -> c_double = unsafe {libm.symbol("cos")}.expect("cos not found");

    let arg = 2.0;
    let result = unsafe { cos(arg) };
    println!("cos({}) = {}", arg, result);
}