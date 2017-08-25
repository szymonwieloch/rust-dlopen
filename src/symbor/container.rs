use super::api::SymBorApi;
use super::Library;
use std::mem::transmute;
use std::ops::{Deref, DerefMut};
use std::ffi::OsStr;
use super::super::Error;

pub struct Container<T> where T: SymBorApi<'static> {
    #[allow(dead_code)]
    lib: Library,
    api: T
}

impl <T> Container<T> where T: SymBorApi<'static> {
    pub unsafe fn load<S>(name: S) -> Result<Self, Error>  where S: AsRef<OsStr> {
        let lib = Library::open(name)?;
        //this is cheating of course
        //but it is safe because Library and api is placed in the same structure
        //and therefore it is released at the same time.
        let static_ref: &'static Library = transmute(&lib);
        let api = T::load(static_ref)?;
        Ok(Self{
            api: api,
            lib: lib
        })
    }
}

impl<T> Deref for Container<T> where T: SymBorApi<'static> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.api
    }
}

impl<T> DerefMut for Container<T> where T: SymBorApi<'static> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.api
    }
}