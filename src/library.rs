use libc::{dlopen, dlsym, dlerror, dlclose, c_void, RTLD_LAZY};
use std::ffi::{CString};
use symbols::{Symbol, Pointer};

use super::err::{Error, DlError};

pub struct Library {
    handle: * mut c_void
}

impl Library {
    pub fn open(name: &str) -> Result<Library, Error> {
        //str can't hold null character at the end, so wee need to convert it
        //This introduces overhead but is negligible - usually the library is loaded only once
        //per application run
        //this could be further optimized by the use MAX_PATH constant and
        // allocation of memory on the stack
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

    pub unsafe fn symbol<T>(&self, name: &str) -> Result<Symbol<T> , Error> {
        //we need to call dlerror in order to clear error buffer
        let _ = dlerror();
        let cname = CString::new(name)?;
        let symbol = dlsym(self.handle, cname.as_ptr());
        //This can be either error or just the library has a NULl pointer - legal
        if symbol.is_null() {
            let msg = dlerror();
            return Err(if msg.is_null() {
                //this is correct behavior but we can't convert NULL to reference
                Error::NullPointer
            } else {
                //this is just error
                Error::DlError(DlError::from_ptr(msg))
            })
        }
        Ok(Symbol::new(symbol))
    }

    pub unsafe fn pointer<T>(&self, name: &str) -> Result<Pointer<T> , Error> {
        //we need to call dlerror in order to clear error buffer
        let _ = dlerror();
        let cname = CString::new(name)?;
        let symbol = dlsym(self.handle, cname.as_ptr());
        //This can be either error or just the library has a NULl pointer - legal
        if symbol.is_null() {
            //for pointer null is a legal value
            let msg = dlerror();
            if !msg.is_null() {
                return Err(Error::DlError(DlError::from_ptr(msg)));
            }
        }
        Ok(Pointer::new(symbol))
    }
}

unsafe impl Send for Library {}
unsafe impl Sync for Library {}

impl Drop for Library {
    fn drop(&mut self) {
        assert_eq!(unsafe {dlclose(self.handle)}, 0);
    }
}