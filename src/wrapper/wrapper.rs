use super::super::lowlevel::DynLib;
use super::super::Error;
use std::ops::{Deref, DerefMut};
use super::api::WrapperApi;
use std::ffi::{OsStr};

/**
Wraps a dynamic load library handle and a API if this library into one single structure.

Wrapping both library handle and symbols makes it safe to use it because symbols are released
together with the library. Wrapper also doesn't have any external lifetimes - this makes it
easy to use Wrapper inside structures.

```no_run
#[macro_use]
extern crate dynlib_derive;
extern crate dynlib;
extern crate libc;
use dynlib::wrapper::{Wrapper, WrapperApi};
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
let mut wrapper: Wrapper<Example> = unsafe { Wrapper::open("libexample.dynlib")}.unwrap();
wrapper.do_something();
let _result = unsafe { wrapper.add_one(5) };
*wrapper.global_count_mut() += 1;
println!("C string: {}", wrapper.c_string().to_str().unwrap())
}
```
*/
pub struct Wrapper<T> where T: WrapperApi {
    #[allow(dead_code)] //this is not dead code because destructor of DynLib deallocates the library
    lib: DynLib,
    api: T
}

impl<T> Wrapper<T> where T: WrapperApi {
    ///Open the library using provided file name or path and load all symbols.
    pub unsafe fn open<S>(name: S) -> Result<Wrapper<T>, Error>  where S: AsRef<OsStr> {
        let lib = DynLib::open(name)?;
        let api = T::load(&lib)?;
        Ok(Self{
            lib: lib,
            api: api
        })
    }
}

impl<T> Deref for Wrapper<T> where T: WrapperApi{
    type Target = T;
    fn deref(&self) -> &T {
        &self.api
    }
}

impl<T> DerefMut for Wrapper<T> where T: WrapperApi{
    fn deref_mut(&mut self) -> &mut T {
        &mut self.api
    }
}