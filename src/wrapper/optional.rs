use super::super::raw::Library;
use super::super::Error;
use std::ops::{Deref, DerefMut};
use super::api::WrapperApi;
use std::ffi::OsStr;

/**
Container for a library handle and both obligatory and optional APIs inside one structure.

A common problem with dynamic link libraries is that they often have different versions and some
of those versions have broader API than others. This structure allows you to use two APIs at the
same time - one obligatory and one optional. If symbols of the optional API are found in the
library, the optional API gets loaded. Otherwise the `optional()` method will return `None`.

#Example

```no_run
#[macro_use]
extern crate dlopen_derive;
extern crate dlopen;
use dlopen::wrapper::{OptionalContainer, WrapperApi};

#[derive(WrapperApi)]
struct Obligatory<'a> {
    do_something: extern "C" fn(),
    global_count: &'a mut u32,
}

#[derive(WrapperApi)]
struct Optional{
    add_one: unsafe extern "C" fn (arg: i32) -> i32,
    c_string: * const u8
}

fn main () {
    let mut container: OptionalContainer<Obligatory, Optional> = unsafe {
        OptionalContainer::load("libexample.dylib")
    }.unwrap();
    container.do_something();
    *container.global_count_mut() += 1;

    match container.optional(){
        &Some(ref opt) => {
            let _result = unsafe { opt.add_one(5) };
            println!("First byte of C string is {}", unsafe{*opt.c_string});
        },
        &None => println!("The optional API was not loaded!")
    }
}
```

**Note:** For more complex cases (multiple versions of API) you can use
[`WrapperMultiApi`](./trait.WrapperMultiApi.html).
*/
pub struct OptionalContainer<Api, Optional>
where
    Api: WrapperApi,
    Optional: WrapperApi,
{
    #[allow(dead_code)]
    //this is not dead code because destructor of Library deallocates the library
    lib: Library,
    api: Api,
    optional: Option<Optional>,
}

impl<Api, Optional> OptionalContainer<Api, Optional>
where
    Api: WrapperApi,
    Optional: WrapperApi,
{
    ///Opens the library using provided file name or path and loads all symbols (including optional
    ///if it is possible).
    pub unsafe fn load<S>(name: S) -> Result<OptionalContainer<Api, Optional>, Error>
    where
        S: AsRef<OsStr>,
    {
        let lib = Library::open(name)?;
        let api = Api::load(&lib)?;
        let optional = Optional::load(&lib).ok();
        Ok(Self { lib, api, optional })
    }

    ///Load all symbols (including optional if it is possible) from the
    ///program itself.
    ///
    /// This allows a shared library to load symbols of the program it was
    /// loaded into.
    pub unsafe fn load_self() -> Result<OptionalContainer<Api, Optional>, Error> {
        let lib = Library::open_self()?;
        let api = Api::load(&lib)?;
        let optional = Optional::load(&lib).ok();
        Ok(Self { lib, api, optional })
    }

    ///Gives access to the optional API - constant version.
    pub fn optional(&self) -> &Option<Optional> {
        return &self.optional;
    }

    ///Gives access to the optional API - constant version.
    pub fn optional_mut(&mut self) -> &Option<Optional> {
        return &mut self.optional;
    }
}

impl<Api, Optional> Deref for OptionalContainer<Api, Optional>
where
    Api: WrapperApi,
    Optional: WrapperApi,
{
    type Target = Api;
    fn deref(&self) -> &Api {
        &self.api
    }
}

impl<Api, Optional> DerefMut for OptionalContainer<Api, Optional>
where
    Api: WrapperApi,
    Optional: WrapperApi,
{
    fn deref_mut(&mut self) -> &mut Api {
        &mut self.api
    }
}
