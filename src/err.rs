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
    ///Address could not be matched to a dynamic link library
    AddrNotMatchingDll(IoError)
}

impl ErrorTrait for Error {
    fn description(&self) -> &str {
        use self::Error::*;
        match self {
            &NullCharacter(_) => "String had a null character",
            &OpeningLibraryError(_) => "Could not open library",
            &SymbolGettingError(_) => "Could not obtain symbol from the library",
            &NullSymbol => "The symbol is NULL",
            &AddrNotMatchingDll(_) => "Address does not match any dynamic link library"
        }
    }

    fn cause(&self) -> Option<&ErrorTrait> {
        use self::Error::*;
        match self {
            &NullCharacter(ref val) => Some(val),
            &OpeningLibraryError(_) | &SymbolGettingError(_) | &NullSymbol | &AddrNotMatchingDll(_)=> {
                None
            }
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use self::Error::*;
        f.write_str(self.description())?;
        match self {
            &OpeningLibraryError(ref msg) => {
                f.write_str(": ")?;
                msg.fmt(f)
            }
            &SymbolGettingError(ref msg) => {
                f.write_str(": ")?;
                msg.fmt(f)
            }
            &NullSymbol | &NullCharacter(_) | &AddrNotMatchingDll(_)=> Ok(()),
        }
    }
}

impl From<NulError> for Error {
    fn from(val: NulError) -> Error {
        Error::NullCharacter(val)
    }
}
