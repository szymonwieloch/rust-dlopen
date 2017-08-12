use libc::{dlopen, dlsym, dlerror, dlclose, dladdr, c_int, c_void, RTLD_LAZY};
use std::ffi::{CString};
use symbol::Symbol;
use std::mem::transmute;

use super::err::{Error, DlError};

pub struct Library {
    handle: * mut c_void
}



impl Library {
    pub fn open(name: &str) -> Result<Library, Error> {
        let name = CString::new(name)?;
        let handle = unsafe { dlopen(name.as_ptr(), RTLD_LAZY) };

        if handle.is_null() {
            Err(Error::DlError(DlError::new()))
        } else {
            Ok(Library {
                handle: handle
            })
        }
    }

    pub fn symbol<T>(&self, name: &str) -> Result<Symbol<T>, Error> {
        let _ = unsafe{dlerror()};
        let name = CString::new(name)?;
        let symbol = unsafe{dlsym(self.handle, name.as_ptr())};
        if symbol.is_null() {
            let msg = unsafe{dlerror()};
            if !msg.is_null() {} {
                return Err(Error::DlError(DlError::from_ptr(msg)));
            }
        }
        let symbol: * mut T = symbol as * mut T;
        Ok(Symbol::new(symbol))
    }
}

unsafe impl Send for Library {}
unsafe impl Sync for Library {}

impl Drop for Library {
    fn drop(&mut self) {
        assert_eq!(unsafe {dlclose(self.handle)}, 0);
    }
}