use std::error::Error as ErrorTrait;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::convert::From;
use std::ffi::NulError;
use std::io::Error as IoError;

///This is a library-specific error that is returned by all calls to all APIs.
#[derive(Debug)]
pub enum Error {
    ///Provided string could not be coverted into `std::ffi::CString` because it contained null
    /// character.
    NullCharacter(NulError),
    ///The library could not be opened.
    OpeningLibraryError(IoError),
    ///The symbol could not be obtained.
    SymbolGettingError(IoError),
    ///Value of the symbol was null.
    NullSymbol,
}

impl ErrorTrait for Error {
    fn description(&self) -> &str {
        match self {
            &Error::NullCharacter(_) => "String had a null character",
            &Error::OpeningLibraryError(_) => "Could not open library",
            &Error::SymbolGettingError(_) => "Could not obtain symbol from the library",
            &Error::NullSymbol => "The symbol is NULL",
        }
    }

    fn cause(&self) -> Option<&ErrorTrait> {
        match self {
            &Error::NullCharacter(ref val) => Some(val),
            &Error::OpeningLibraryError(_) | &Error::SymbolGettingError(_) | &Error::NullSymbol => {
                None
            }
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
            }
            &Error::SymbolGettingError(ref msg) => {
                f.write_str(": ")?;
                msg.fmt(f)
            }
            &Error::NullSymbol | &Error::NullCharacter(_) => Ok(()),
        }
    }
}

impl From<NulError> for Error {
    fn from(val: NulError) -> Error {
        Error::NullCharacter(val)
    }
}
