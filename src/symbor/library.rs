use std::ffi::{CStr};
use super::symbol::Symbol;
use super::pointer::Pointer;
use super::pointer_mut::PointerMut;
use super::super::lowlevel::DynLib;
use std::ffi::OsStr;

use super::super::err::{Error};

pub struct Library {
    lib: DynLib
}

impl Library {
    pub fn open<S>(name: S) -> Result<Library, Error>  where S: AsRef<OsStr> {
        Ok(Library{
            lib: DynLib::open(name)?
        })
    }

    pub unsafe fn symbol<T>(&self, name: &str) -> Result<Symbol<T> , Error> {
        Ok(Symbol::new(self.lib.symbol(name)?))
    }

    pub unsafe fn symbol_cstr<T>(&self, name: &CStr) -> Result<Symbol<T> , Error> {
        Ok(Symbol::new(self.lib.symbol_cstr(name)?))
    }

    pub unsafe fn pointer<T>(&self, name: &str) -> Result<Pointer<T> , Error> {
        Ok(Pointer::new(self.lib.pointer(name)?))
    }

    pub unsafe fn pointer_cstr<T>(&self, name: &CStr) -> Result<Pointer<T> , Error> {
        Ok(Pointer::new(self.lib.pointer_cstr(name)?))
    }

    pub unsafe fn pointer_mut<T>(&self, name: &str) -> Result<PointerMut<T> , Error> {
        Ok(PointerMut::new(self.lib.pointer_mut(name)?))
    }

    pub unsafe fn pointer_mut_cstr<T>(&self, name: &CStr) -> Result<PointerMut<T> , Error> {
        Ok(PointerMut::new(self.lib.pointer_mut_cstr(name)?))
    }

    pub unsafe fn reference<T>(&self, name: &str) -> Result<&T, Error> {
        self.lib.symbol(name)
    }

    pub unsafe fn reference_cstr<T>(&self, name: &CStr) -> Result<&T, Error> {
        self.lib.symbol_cstr(name)
    }

    pub unsafe fn reference_mut<T>(&self, name: &str) -> Result<&mut T, Error> {
        self.lib.symbol(name)
    }

    pub unsafe fn reference_mut_cstr<T>(&self, name: &CStr) -> Result<&mut T, Error> {
        self.lib.symbol_cstr(name)
    }


}

unsafe impl Send for Library {}
unsafe impl Sync for Library {}