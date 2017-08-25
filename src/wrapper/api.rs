use super::super::err::Error;
use super::super::raw::Library;

/**
Trait for defining library API.

This trait is intended to be used with `#[derive(WrapperApi)]` macro defined in the
`dynlib_derive` crate. It forces several restrictions on types that implement it:

* Only sructures can implement this trait.
* All fields need to be private.
* Only functions, references and pointers are allowed.
* You can't define a type using `type Fun =fn();` and use it in the structure. This is a limitation
    of Rut reflections mechanism. Only raw functions, references and functions are allowed.
* All arguments of functions need to be named.


The `derive` macro not only generates implementation of `load()` function, but it also generates
safe accessors or wrappers to the loaded symbols. These functions are named exactly like the field that
they wrap. Functions have the same arguments like original symbols and references are
just simple accessors in the form of `<field_name>(self)->&FieldType` or `<field_name>_mut(&mut self) -> &mut FieldType`.
Wrappers are not generated only for:

* Pointers - there is no safe way of preventing dangling symbols if a user has a direct access to pointers.
    The recommended approach here is to either use references instead of pointers or
    to manually create safe wrappers. For example C `const char *` can be manually converted into `& std::ffi::CStr`.
* Variadic functions. Rust doesn't have any mechanism that allows creating safe wrappers around them.
    You need to handle them manually.

#Example

'''no_run
#[macro_use]
extern crate dynlib_derive;
extern crate dynlib;
extern crate libc;
use dynlib::wrapper::{WrapperApi};
use dynlib::raw::Library;
use libc::{c_char};
use std::ffi::CStr;

#[derive(WrapperApi)]
struct Example<'a> {
    do_something: extern "C" fn(),
    add_one: unsafe extern "C" fn (arg: i32) -> i32,
    global_count: &'a mut u32,
    c_string: * const c_char
}

//wrapper for c_string won't be generated, implement it here
impl<'a> Example<'a> {
    pub fn c_string(&self) -> &CStr {
        unsafe {CStr::from_ptr(self.c_string)}
    }
}

fn main () {
let lib = unsafe { Library::open("libexample.dynlib")}.unwrap();
let api = Example::load(&lib).unwrap();
api.do_something();
let _result = unsafe { api.add_one(5) };
*api.global_count_mut() += 1;
println!("C string: {}", api.c_string().to_str().unwrap())

//please notice that this compiles because api does not have any reference to the library.
//This is why direct use of API may be unsafe. The recommended way to handle this problem is by using the Wrapper structure.
drop(lib);


}
'''

*/
pub trait WrapperApi where Self: Sized {
    ///Load symbols from provided library.
    unsafe fn load(lib: &Library) -> Result<Self, Error>;
}