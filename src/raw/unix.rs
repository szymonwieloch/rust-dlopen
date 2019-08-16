use super::super::err::Error;
use std::ffi::{CStr, OsStr};
use libc::{c_int, c_void, dlclose, dlerror, dlopen, dlsym, dladdr, Dl_info, RTLD_LAZY, RTLD_LOCAL};
use std::ptr::{null, null_mut};
use std::os::unix::ffi::OsStrExt;
use std::io::{Error as IoError, ErrorKind};
use std::mem::uninitialized;
use super::common::{AddressInfo, OverlappingSymbol};

const DEFAULT_FLAGS: c_int = RTLD_LOCAL | RTLD_LAZY;

use std::sync::Mutex;

// calls to dlerror are not thread unsafe. Therefore we need to guard each call with a mutex

lazy_static! {
    static ref DLERROR_MUTEX: Mutex<()> = Mutex::new(());
}

pub type Handle = *mut c_void;

#[inline]
pub unsafe fn get_sym(handle: Handle, name: &CStr) -> Result<*mut (), Error> {
    let _lock = DLERROR_MUTEX.lock();
    //clear the dlerror in order to be able to distinguish between NULL pointer and error
    let _ = dlerror();
    let symbol = dlsym(handle, name.as_ptr());
    //This can be either error or just the library has a NULl pointer - legal
    if symbol.is_null() {
        let msg = dlerror();
        if !msg.is_null() {
            return Err(Error::SymbolGettingError(IoError::new(
                ErrorKind::Other,
                CStr::from_ptr(msg).to_string_lossy().to_string(),
            )));
        }
    }
    Ok(symbol as *mut ())
}

#[inline]
pub unsafe fn open_self() -> Result<Handle, Error> {
    let _lock = DLERROR_MUTEX.lock();
    let handle = dlopen(null(), DEFAULT_FLAGS);
    if handle.is_null() {
        Err(Error::OpeningLibraryError(IoError::new(
            ErrorKind::Other,
            CStr::from_ptr(dlerror()).to_string_lossy().to_string(),
        )))
    } else {
        Ok(handle)
    }
}

#[inline]
pub unsafe fn open_lib(name: &OsStr) -> Result<Handle, Error> {
    let mut v: Vec<u8> = Vec::new();
    //as_bytes i a unix-specific extension
    let cstr = if name.len() > 0 && name.as_bytes()[name.len() - 1] == 0 {
        //don't need to convert
        CStr::from_bytes_with_nul_unchecked(name.as_bytes())
    } else {
        //need to convert
        v.extend_from_slice(name.as_bytes());
        v.push(0);
        CStr::from_bytes_with_nul_unchecked(v.as_slice())
    };
    let _lock = DLERROR_MUTEX.lock();
    let handle = dlopen(cstr.as_ptr(), DEFAULT_FLAGS);
    if handle.is_null() {
        Err(Error::OpeningLibraryError(IoError::new(
            ErrorKind::Other,
            CStr::from_ptr(dlerror()).to_string_lossy().to_string(),
        )))
    } else {
        Ok(handle)
    }
}

#[inline]
pub unsafe fn addr_info_init(){}
pub unsafe fn addr_info_cleanup(){}

#[inline]
pub unsafe fn addr_info_obtain(addr: * const ()) -> Result<AddressInfo, Error>{
    let mut dlinfo: Dl_info = uninitialized();
    let result = dladdr(addr as * const c_void, & mut dlinfo);
    if result == 0 {
        Err(Error::AddrNotMatchingDll(IoError::new(ErrorKind::NotFound, String::new())))
    } else {
        let os = if dlinfo.dli_saddr.is_null() || dlinfo.dli_sname.is_null() {
            None
        } else {
            Some(OverlappingSymbol{
                addr: dlinfo.dli_saddr as * const (),
                name: CStr::from_ptr(dlinfo.dli_sname).to_string_lossy().into_owned()
            })
        };
        Ok(AddressInfo{
            dll_path: CStr::from_ptr(dlinfo.dli_fname).to_string_lossy().into_owned(),
            dll_base_addr: dlinfo.dli_fbase as * const (),
            overlapping_symbol: os
        })
    }

}

#[inline]
pub fn close_lib(handle: Handle) -> Handle {
    let result = unsafe { dlclose(handle) };
    if result != 0 {
        panic!("Call to dlclose() failed");
    }
    null_mut()
}
