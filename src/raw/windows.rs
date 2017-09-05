use winapi;
use kernel32;
use std::os::windows::ffi::OsStrExt;
use std::sync::atomic::{AtomicBool, Ordering, ATOMIC_BOOL_INIT};
use std::io::{Error as IoError, ErrorKind};
use super::super::err::Error;
use std::ptr::null_mut;
use std::ffi::{CStr, OsStr};

static USE_ERRORMODE: AtomicBool = ATOMIC_BOOL_INIT;

pub type Handle = winapi::HMODULE;

struct ErrorModeGuard(winapi::DWORD);

impl ErrorModeGuard {
    fn new() -> ErrorModeGuard {
        let mut ret = ErrorModeGuard(0);

        if !USE_ERRORMODE.load(Ordering::Acquire) {
            if unsafe {
                kernel32::SetThreadErrorMode(1, &mut ret.0) == 0 &&
                    kernel32::GetLastError() == winapi::ERROR_CALL_NOT_IMPLEMENTED
            } {
                USE_ERRORMODE.store(true, Ordering::Release);
            } else {
                return ret;
            }
        }
        ret.0 = unsafe { kernel32::SetErrorMode(1) };
        ret
    }
}

impl Drop for ErrorModeGuard {
    fn drop(&mut self) {
        unsafe {
            if !USE_ERRORMODE.load(Ordering::Relaxed) {
                kernel32::SetThreadErrorMode(self.0, null_mut());
            } else {
                kernel32::SetErrorMode(self.0);
            }
        }
    }
}

unsafe fn get_win_error() -> IoError {
    let error = kernel32::GetLastError();
    if error == 0 {
        IoError::new(
            ErrorKind::Other,
            "Could not obtain information about the error",
        )
    } else {
        IoError::from_raw_os_error(error as i32)
    }
}

#[inline]
pub unsafe fn get_sym(handle: Handle, name: &CStr) -> Result<*mut (), Error> {
    let symbol = kernel32::GetProcAddress(handle, name.as_ptr());
    if symbol.is_null() {
        Err(Error::SymbolGettingError(get_win_error()))
    } else {
        Ok(symbol as *mut ())
    }
}

#[inline]
pub unsafe fn open_lib(name: &OsStr) -> Result<Handle, Error> {
    let wide_name: Vec<u16> = name.encode_wide().chain(Some(0)).collect();
    let _guard = ErrorModeGuard::new();
    let handle = kernel32::LoadLibraryW(wide_name.as_ptr());
    if handle.is_null() {
        Err(Error::OpeningLibraryError(get_win_error()))
    } else {
        Ok(handle)
    }
}

#[inline]
pub fn close_lib(handle: Handle) -> Handle {
    if unsafe { kernel32::FreeLibrary(handle) } == 0 {
        //this should not happen
        panic!("FreeLibrary() failed, the error is {}", unsafe {
            get_win_error()
        });
    }
    null_mut()
}
