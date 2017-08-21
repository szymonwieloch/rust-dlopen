use super::super::lowlevel::DynLib;
use super::super::Error;
use std::ops::{Deref, DerefMut};
use super::api::WrapperApi;
use std::ffi::{OsStr};

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