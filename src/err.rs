use std::error::Error as ErrorTrait;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::convert::From;
use std::ffi::{NulError};
use std::io::Error as IoError;

#[derive(Debug)]
pub enum Error{
    NullError(NulError),
    OpeningLibraryError(IoError),
    SymbolGettingError(IoError),
    NullPointer
}

impl ErrorTrait for Error {
    fn description(&self) -> &str {
        match self {
            &Error::NullError(_) => "String had a null character",
            &Error::OpeningLibraryError(_) => "Could not open library",
            &Error::SymbolGettingError(_) => "Could not obtain symbol from the library",
            &Error::NullPointer => "The symbol is NULL"
        }
    }

    fn cause(&self) -> Option<&ErrorTrait> {
        match self {
            &Error::NullError(ref val) => Some(val),
            &Error::OpeningLibraryError(_) |
            &Error::SymbolGettingError(_) |
            &Error::NullPointer=> None
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str(self.description())?;
        match self {
            &Error::OpeningLibraryError(ref msg) => {
                f.write_str(": ")?;
                msg.fmt(f)
            },
            &Error::SymbolGettingError(ref msg) => {
                f.write_str(": ")?;
                msg.fmt(f)
            },
            &Error::NullPointer | &Error::NullError(_) => {Ok(())}
        }
    }
}

impl From<NulError> for Error {
    fn from(val: NulError) -> Error {
        Error::NullError(val)
    }
}