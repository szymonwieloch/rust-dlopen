/*!
Safe and object-oriented friendly API for working with dynamic link libraries.
This is the most advanced API, recommended for most of projects.

This API allows automatic loading of symbols into structures and prevents the problem with dangling symbols.
It does it by wrapping the library handle and symbols into one structure
(so that symbols and library can be released only at the same time) and by wrapping private symbols with
public accessors or functions. Contrary to the `symbor` API this one actually allows creation of
100% safe APIs and supports object oriented design because library and symbols are wrapped into one
structure without external lifetimes (so you can safely move the wrapper or make it a part of a structure).

#Example

```no_run
#[macro_use]
extern crate dynlib_derive;
extern crate dynlib;
extern crate libc;
use dynlib::wrapper::{Container, WrapperApi};
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
let mut container: Container<Example> = unsafe { Container::open("libexample.dynlib")}.unwrap();
container.do_something();
let _result = unsafe { container.add_one(5) };
*container.global_count_mut() += 1;
println!("C string: {}", container.c_string().to_str().unwrap())
}
```
*/

mod api;
mod multi_api;
mod container;
mod optional;
mod option;
pub use self::api::WrapperApi;
pub use self::multi_api::WrapperMultiApi;
pub use self::container::Container;
pub use self::optional::OptionalContainer;
