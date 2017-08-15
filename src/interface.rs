use super::dlopen::DlOpen;
use super::Error;
use std::ffi::{CStr, CString};
use std::ops::Deref;

pub trait LibraryInterface: Sized {
    unsafe fn load(lib: &DlOpen) -> Result<Self, Error>;
}

pub struct Wrapper<T> where T: LibraryInterface {
    lib: DlOpen,
    api: T
}

impl<T> Wrapper<T> where T: LibraryInterface {
    pub unsafe fn load(lib_name: &str) -> Result<Self, Error> {
        let cname = CString::new(lib_name)?;
        Self::load_cstr(cname.as_ref())
    }

    pub unsafe fn load_cstr(lib_name: &CStr) -> Result<Self, Error> {
        let lib = DlOpen::open_cstr(lib_name)?;
        let api = T::load(&lib)?;
        Ok(Self {
            api: api,
            lib: lib
        })
    }
}

impl<T> Deref for Wrapper<T> where T: LibraryInterface{
    type Target = T;
    fn deref(&self) -> &T {
        &self.api
    }
}