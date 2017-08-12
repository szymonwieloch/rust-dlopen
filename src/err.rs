use std::error::Error as ErrorTrait;
use std::fmt::{Display, Debug, Formatter, Error as FmtError, Result as FmtResult};
use std::convert::From;
use std::ffi::{NulError, CStr};
use libc::{dlerror, c_char};

#[derive(Debug, Clone)]
pub enum Error{
    NullError(NulError),
    DlError(DlError),
    NullPointer
}
#[derive(Debug, Clone)]
pub struct DlError {
    //This is unfortunate but because of Rust limitations we can't put here &str directly
    //because there is no direct guarantee that the pointer refers to statically allocated strings
    //Yet this is true for probably all existing platforms. So just use lazy conversion
    msg: * const c_char
}

impl DlError {
    pub fn new() -> DlError {
        DlError::from_ptr(unsafe {dlerror()})
    }

    pub fn from_ptr(msg: * const c_char) -> DlError {
        if msg.is_null() {
            // this is unexpected, probably user called this function without error
            panic!("dlerror() returned NULL");
        }
        DlError {
            msg: msg
        }
    }
}

impl ErrorTrait for DlError {
    fn description(&self) -> &str {
        //it is unexpected to see any errors with os returned messages
        unsafe { CStr::from_ptr(self.msg) }.to_str().expect("dlerror() returned inconvertible string")
    }
}

impl Display for DlError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "dlerror: {}", self.description())
    }
}

impl ErrorTrait for Error {
    fn description(&self) -> &str {
        match self {
            &Error::NullError(_) => "String had a null character",
            &Error::DlError(_) => "dlerror() reported error",
            &Error::NullPointer => "dlsym() returned NULL as a symbol"
        }
    }

    fn cause(&self) -> Option<&ErrorTrait> {
        match self {
            &Error::NullError(ref val) => Some(val),
            &Error::DlError(ref val) => Some(val),
            &Error::NullPointer => None
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str(self.description())
    }
}

impl From<NulError> for Error {
    fn from(val: NulError) -> Error {
        Error::NullError(val)
    }
}

impl From<DlError> for Error {
    fn from(val: DlError) -> Error {
        Error::DlError(val)
    }
}