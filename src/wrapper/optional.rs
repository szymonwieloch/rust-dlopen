use super::super::lowlevel::DynLib;
use super::super::Error;
use std::ops::{Deref, DerefMut};
use super::api::WrapperApi;
use std::ffi::{OsStr};

/**
Wraps a library handle and both obligatory and optional API inside one structure.

A common problem with dynamic load libraries is that they often have different versions and some
of those versions have broader API than others. This structure allows you to use two APIs at the
same time - an obligatory one and an optional one. This library does not cover more cases (such as
several optional APIs) - you need to write a custom wrapper on your own.

```no_run
#[macro_use]
extern crate dynlib_derive;
extern crate dynlib;
extern crate libc;
use dynlib::wrapper::{WrapperOptional, WrapperApi};
use libc::{c_char};
use std::ffi::CStr;

#[derive(WrapperApi)]
struct Obligatory<'a> {
    do_something: extern "C" fn(),
    global_count: &'a mut u32,
}

#[derive(WrapperApi)]
struct Optional{
    add_one: unsafe extern "C" fn (arg: i32) -> i32,
    c_string: * const c_char
}

//wrapper for c_string won't be generated, implement it here
impl<'a> Optional {
    pub fn c_string(&self) -> &CStr {
        unsafe {CStr::from_ptr(self.c_string)}
    }
}

fn main () {
    let mut wrapper: WrapperOptional<Obligatory, Optional> = unsafe { WrapperOptional::open("libexample.dynlib")}.unwrap();
    wrapper.do_something();

    *wrapper.global_count_mut() += 1;
    match wrapper.optional(){
        &Some(ref opt) => {
            let _result = unsafe { opt.add_one(5) };
            println!("C string: {}", opt.c_string().to_str().unwrap())
        },
        &None => println!("The optional API was not loaded!")
    }
}
```
*/
pub struct WrapperOptional<Api, Optional> where Api: WrapperApi, Optional: WrapperApi {
    #[allow(dead_code)] //this is not dead code because destructor of DynLib deallocates the library
    lib: DynLib,
    api: Api,
    optional: Option<Optional>
}

impl<Api, Optional> WrapperOptional<Api, Optional> where Api: WrapperApi, Optional: WrapperApi {
    ///Opens the library using provided file name or path and loads all symbols (including optional if it is possible).
    pub unsafe fn open<S>(name: S) -> Result<WrapperOptional<Api, Optional>, Error>  where S: AsRef<OsStr> {
        let lib = DynLib::open(name)?;
        let api = Api::load(&lib)?;
        let optional = match Optional::load(&lib) {
            Ok(val) => Some(val),
            Err(_) => None
        };
        Ok(Self{
            lib: lib,
            api: api,
            optional: optional
        })
    }
    ///Gives access to the optional API - constant version.
    pub fn optional(&self) -> &Option<Optional> {
        return &self.optional
    }

    ///Gives access to the optional API - constant version.
    pub fn optional_mut(&mut self) -> &Option<Optional> {
        return &mut self.optional
    }
}

impl<Api, Optional> Deref for WrapperOptional<Api, Optional> where Api: WrapperApi, Optional: WrapperApi{
    type Target = Api;
    fn deref(&self) -> &Api {
        &self.api
    }
}

impl<Api, Optional> DerefMut for WrapperOptional<Api, Optional> where Api: WrapperApi, Optional: WrapperApi{
    fn deref_mut(&mut self) -> &mut Api {
        &mut self.api
    }
}