use super::api::SymBorApi;
use super::Library;
use std::mem::transmute;
use std::ops::{Deref, DerefMut};
use std::ffi::OsStr;
use super::super::Error;

/**
Container for both dynamic link library handle and its API.

This structure solves an important issue: object oriented programming where the given
structure has two objects and one of the objects has a reference to the second one.
Normally you can't put `Library` and a structure that implements `SymBorApi` into one structure.
This structure allows you to do it.

# Example

```no_run
#[macro_use]
extern crate dlopen_derive;
extern crate dlopen;
use dlopen::symbor::{Library, Symbol, Ref, PtrOrNull, SymBorApi, Container};

 #[derive(SymBorApi)]
 struct ExampleApi<'a> {
    pub fun: Symbol<'a, unsafe extern "C" fn(i32) -> i32>,
    pub glob_i32: Ref<'a, i32>,
    pub maybe_c_str: PtrOrNull<'a, u8>,
 }

fn main(){
    let cont: Container<ExampleApi> = unsafe{Container::load("libexample.so")}
        .expect("Could not load library or symbols");
    println!("fun(4)={}", unsafe{(cont.fun)(4)});
    println!("glob_i32={}", *cont.glob_i32);
    println!("The pointer is null={}", cont.maybe_c_str.is_null());
}
```
*/
pub struct Container<T>
where
    T: SymBorApi<'static>,
{
    #[allow(dead_code)] lib: Library,
    api: T,
}

impl<T> Container<T>
where
    T: SymBorApi<'static>,
{
    ///Open dynamic link library and load symbols.
    pub unsafe fn load<S>(name: S) -> Result<Self, Error>
    where
        S: AsRef<OsStr>,
    {
        let lib = Library::open(name)?;
        //this is cheating of course
        //but it is safe because Library and api is placed in the same structure
        //and therefore it is released at the same time.
        let static_ref: &'static Library = transmute(&lib);
        let api = T::load(static_ref)?;
        Ok(Self { api: api, lib: lib })
    }
    ///Load all symbols from the program itself.
    ///
    /// This allows a shared library to load symbols of the program it was
    /// loaded into.
    pub unsafe fn load_self() -> Result<Self, Error> {
        let lib = Library::open_self()?;
        //this is cheating of course
        //but it is safe because Library and api is placed in the same structure
        //and therefore it is released at the same time.
        let static_ref: &'static Library = transmute(&lib);
        let api = T::load(static_ref)?;
        Ok(Self { api: api, lib: lib })
    }
}

impl<T> Deref for Container<T>
where
    T: SymBorApi<'static>,
{
    type Target = T;
    fn deref(&self) -> &T {
        &self.api
    }
}

impl<T> DerefMut for Container<T>
where
    T: SymBorApi<'static>,
{
    fn deref_mut(&mut self) -> &mut T {
        &mut self.api
    }
}
