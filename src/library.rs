use std::ffi::{CStr};
use symbols::{Symbol, Pointer};
use super::dlopen::DlOpen;

use super::err::{Error};

pub struct Library {
    lib: DlOpen
}

impl Library {
    pub fn open(name: &str) -> Result<Library, Error> {
        Ok(Library{
            lib: DlOpen::open(name)?
        })
    }

    pub fn open_cstr(name: &CStr) -> Result<Library, Error> {
        Ok(Library{
            lib: DlOpen::open_cstr(name)?
        })
    }

    pub unsafe fn symbol<T>(&self, name: &str) -> Result<Symbol<T> , Error> where T: Clone{
        Ok(Symbol::new(self.lib.symbol(name)?))
    }

    pub unsafe fn symbol_str<T>(&self, name: &CStr) -> Result<Symbol<T> , Error> where T: Clone{
        Ok(Symbol::new(self.lib.symbol_cstr(name)?))
    }

    pub unsafe fn pointer<T>(&self, name: &str) -> Result<Pointer<T> , Error> {
        Ok(Pointer::new(self.lib.pointer(name)?))
    }

    pub unsafe fn pointer_cstr<T>(&self, name: &CStr) -> Result<Pointer<T> , Error> {
        Ok(Pointer::new(self.lib.pointer_cstr(name)?))
    }
}

unsafe impl Send for Library {}
unsafe impl Sync for Library {}