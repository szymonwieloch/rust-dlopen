#[macro_use]
extern crate dlopen_derive;
extern crate dlopen;
extern crate libc;
use libc::c_double;
use dlopen::{Symbol, Library, LibraryApi};

#[derive(LibraryApi)]
struct LibMApi<'a> {
    #[dlopen_name = "sin"]
    pub this_will_be_sin: Symbol<'a, unsafe extern fn(c_double) -> c_double>,
    pub cos: Symbol<'a, unsafe extern fn(c_double) -> c_double>,
}

const LIB_NAME: &str = "libm.so.6";

fn main () {
    let lib = Library::open(LIB_NAME).expect(&format!("Could not open {}", LIB_NAME));
    let api = unsafe {LibMApi::load(&lib)}.expect(&format!("Could not load symbols of {}", LIB_NAME));

    let arg = 2.0;
    let result = unsafe {(api.cos)(arg)};
    println!("cos({}) = {}", arg, result);
    let result = unsafe {(api.this_will_be_sin)(arg)};
    println!("sin({}) = {}", arg, result);
}