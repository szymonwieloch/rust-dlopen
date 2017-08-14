use libc::{c_void, dlopen, dlerror, dlsym, dlclose, RTLD_LAZY};
use super::drop::DlDrop;
use super::err::{Error, DlError};
use std::ffi::{CString, CStr};
use std::mem::{transmute, size_of};

#[derive(Debug)]
pub struct DlOpen {
    handle: * mut c_void
}

impl DlOpen {
    pub fn open_cstr(name: &CStr) -> Result<DlOpen, Error> {
        let handle = unsafe { dlopen(name.as_ptr(), RTLD_LAZY) };
        if handle.is_null() {
            Err(Error::DlError(DlError::new()))
        } else {
            Ok(DlOpen {
                handle: handle
            })
        }
    }

    pub fn open(name: &str) -> Result<DlOpen, Error> {
        let cname = CString::new(name)?;
        Self::open_cstr(cname.as_ref())
    }

    pub unsafe fn symbol<T>(&self, name: &str) -> Result<T, Error> where T: Clone {
        let cname = CString::new(name)?;
        self.symbol_cstr(cname.as_ref())
    }

    pub unsafe fn symbol_cstr<T> (&self, name: &CStr) -> Result<T, Error> where T:Clone{
        //TODO: convert it to some kind of static assertion (not yet supported in Rust)
        //this comparison should be calculated by compiler at compilation time - zero cost
        if size_of::<T>() != size_of::<*mut c_void>() {
            panic!("The type passed to dlopen::DlOpen::symbol() function has a different size than a pointer - cannot transmute");
        }
        let raw = self.raw_cstr(name)?;
        if raw.is_null(){
            Err(Error::NullPointer)
        } else {
            let r: &T = transmute(&raw);
            Ok(r.clone())
        }
    }

    pub unsafe fn pointer_cstr<T>(&self, name: &CStr) -> Result<* const T, Error> {
        match self.raw_cstr(name) {
            Err(err) => Err(err),
            Ok(ptr) => Ok(ptr as * const T)
        }
    }

    pub unsafe fn pointer<T>(&self, name: &str) -> Result<* const T, Error> {
        let cname = CString::new(name)?;
        self.pointer_cstr(cname.as_ref())
    }

    fn raw_cstr(&self, name: &CStr) -> Result<* mut c_void, Error> {
        //clear the dlerror in order to be able to distinguish between NULL pointer and error
        let _ = unsafe {dlerror()};
        let symbol = unsafe {dlsym(self.handle, name.as_ptr())};
        //This can be either error or just the library has a NULl pointer - legal
        if symbol.is_null() {
            let msg = unsafe { dlerror()};
            if !msg.is_null() {
                return Err(Error::DlError(DlError::from_ptr(msg)));
            }
        }
        Ok(symbol)
    }

    pub fn into_drop(self) -> DlDrop {
        DlDrop::new(self.handle)
    }
}

impl Drop for DlOpen {
    fn drop(&mut self) {
        assert_eq!(unsafe {dlclose(self.handle)}, 0);
    }
}
