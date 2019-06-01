use super::super::raw::Library;
use super::super::Error;
use std::ops::{Deref, DerefMut};
use super::api::WrapperApi;
use std::ffi::OsStr;

/**
Container for both a dynamic load library handle and its API.

Keeping both library and its symbols together makes it safe to use it because symbols are released
together with the library. `Container` also doesn't have any external lifetimes - this makes it
easy to use `Container` inside structures.

#Example

```no_run
#[macro_use]
extern crate dlopen_derive;
extern crate dlopen;
extern crate libc;
use dlopen::wrapper::{Container, WrapperApi};
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
    let mut container: Container<Example> = unsafe { Container::load("libexample.dylib")}.unwrap();
    container.do_something();
    let _result = unsafe { container.add_one(5) };
    *container.global_count_mut() += 1;
    println!("C string: {}", container.c_string().to_str().unwrap())
}
```
*/
pub struct Container<T>
where
    T: WrapperApi,
{
    #[allow(dead_code)]
    //this is not dead code because destructor of Library deallocates the library
    lib: Library,
    api: T,
}

impl<T> Container<T>
where
    T: WrapperApi,
{
    ///Open the library using provided file name or path and load all symbols.
    pub unsafe fn load<S>(name: S) -> Result<Container<T>, Error>
    where
        S: AsRef<OsStr>,
    {
        let lib = Library::open(name)?;
        let api = T::load(&lib)?;
        Ok(Self { lib: lib, api: api })
    }
    ///Load all symbols from the program itself.
    ///
    /// This allows a shared library to load symbols of the program it was
    /// loaded into.
    pub unsafe fn load_self() -> Result<Container<T>, Error> {
        let lib = Library::open_self()?;
        let api = T::load(&lib)?;
        Ok(Self { lib: lib, api: api })
    }
}

impl<T> Deref for Container<T>
where
    T: WrapperApi,
{
    type Target = T;
    fn deref(&self) -> &T {
        &self.api
    }
}

impl<T> DerefMut for Container<T>
where
    T: WrapperApi,
{
    fn deref_mut(&mut self) -> &mut T {
        &mut self.api
    }
}
