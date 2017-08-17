use super::super::err::{Error};
use std::ffi::{CString, CStr, OsStr};

#[cfg(unix)]
use super::unix::{open_lib, get_sym, close_lib};

use std::mem::{transmute_copy, size_of};

#[derive(Debug)]
pub struct DynLib {
    #[cfg(unix)]
    handle: * mut ::libc::c_void,
    #[cfg(windows)]
    handle: winapi::HMODULE
}

impl DynLib {
    pub  fn open<S>(name: S) -> Result<DynLib, Error> where S: AsRef<OsStr> {
;
        Ok(Self {
            handle: unsafe{open_lib(name.as_ref())}?
        })
    }

    pub unsafe fn symbol<T>(&self, name: &str) -> Result<T, Error> {
        let cname = CString::new(name)?;
        self.symbol_cstr(cname.as_ref())
    }

    pub unsafe fn symbol_cstr<T>(&self, name: &CStr) -> Result<T, Error> {
        //TODO: convert it to some kind of static assertion (not yet supported in Rust)
        //this comparison should be calculated by compiler at compilation time - zero cost
        if size_of::<T>() != size_of::<*mut ()>() {
            panic!("The type passed to dlopen::DynLib::symbol() function has a different size than a pointer - cannot transmute");
        }
        let raw = get_sym(self.handle, name)?;
        if raw.is_null() {
            return Err(Error::NullPointer)
        } else {
            Ok(transmute_copy(&raw))
        }
    }

    pub unsafe fn pointer_cstr<T>(&self, name: &CStr) -> Result<*const T, Error> {
        match get_sym(self.handle, name) {
            Err(err) => Err(err),
            Ok(ptr) => Ok(ptr as *const T)
        }
    }

    pub unsafe fn pointer<T>(&self, name: &str) -> Result<*const T, Error> {
        let cname = CString::new(name)?;
        self.pointer_cstr(cname.as_ref())
    }
}

impl Drop for DynLib {
    fn drop(&mut self) {
        self.handle = close_lib(self.handle);
    }
}

unsafe impl Sync for DynLib {}
unsafe impl Send for DynLib {}
