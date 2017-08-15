use super::err::Error;
use std::ffi::{CStr, CString};

pub trait LibraryWrapper where Self: Sized {
    unsafe fn load(lib_name: &str) -> Result<Self, Error> {
        let cname = CString::new(lib_name)?;
        Self::load_cstr(cname.as_ref())
    }
    unsafe fn load_cstr(lib_name: &CStr) -> Result<Self, Error>;
}