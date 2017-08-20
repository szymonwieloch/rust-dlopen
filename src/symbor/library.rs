use std::ffi::{CStr};
use super::symbol::Symbol;
use super::ptr_or_null::PtrOrNull;
use super::ptr_or_null_mut::PtrOrNullMut;
use super::super::lowlevel::DynLib;
use std::ffi::{OsStr, CString};
use std::ptr::{null, null_mut};

use super::super::err::{Error};

/**
A safer wrapper around `DynLib`. Contrary to the original API this one has methods
that return safe wrappers around obtained symbols.

**Note:**: It is recommended that you use certain methods in certain situations:

* `symbol()` - for obtaining functions and pointers but only if you can't use references
    instead of pointers and you do not accept null value of a pointer.
* `reference()` and `reference_mut()` - this is the recommended method for obtaining acess to
    statically allocated objects - either constant or mutable.
* `ptr_or_null()` and `ptr_or_null_mut()` - for obtaining pointers if you accept null values of pointers
    (in 99% of cases you should rather use previously mentioned methods).
*/
pub struct Library {
    lib: DynLib
}

impl Library {
    ///Open the library using provided file name or path.
    pub fn open<S>(name: S) -> Result<Library, Error>  where S: AsRef<OsStr> {
        Ok(Library{
            lib: DynLib::open(name)?
        })
    }

    /// Obtains a symbol from library.
    /// This method is the most general one and allows obtaining basically everything assuming
    /// that the value of a symbol cannot be null (use `ptr_or_null()` for this case).
    /// However the `reference()` and `reference_mut` methods return a native reference and they
    /// are recommended for accessing statically allocated data.
    pub unsafe fn symbol<T>(&self, name: &str) -> Result<Symbol<T> , Error> {
        Ok(Symbol::new(self.lib.symbol(name)?))
    }

    ///Equivalent of the `symbol` method but takes `CStr` as a argument.
    pub unsafe fn symbol_cstr<T>(&self, name: &CStr) -> Result<Symbol<T> , Error> {
        Ok(Symbol::new(self.lib.symbol_cstr(name)?))
    }

    ///Obtains a const pointer from library. This method is only recommended for data
    /// that can't be accessed as a reference and that can have a null pointer value
    /// (so not in 99% of cases).
    pub unsafe fn ptr_or_null<T>(&self, name: &str) -> Result<PtrOrNull<T> , Error> {
       let cname = CString::new(name)?;
        self.ptr_or_null_cstr(cname.as_ref())
    }

    ///Equivalent of the `pointer` method but takes `CStr` as a argument.
    pub unsafe fn ptr_or_null_cstr<T>(&self, name: &CStr) -> Result<PtrOrNull<T> , Error> {
        let raw_ptr = match self.lib.symbol_cstr(name) {
            Ok(val) => val,
            Err(err) => match err {
                Error::NullSymbol => null(),
                _ => return Err(err)
            }
        };
        Ok(PtrOrNull::new(raw_ptr))
    }

    ///Obtains a mutable pointer from library. This method is only recommended for data
    /// that can't be accessed as a reference and that can have a null pointer value
    /// (so not in 99% of cases).
    pub unsafe fn ptr_or_null_mut<T>(&self, name: &str) -> Result<PtrOrNullMut<T> , Error> {
        let cname = CString::new(name)?;
        self.ptr_or_null_mut_cstr(cname.as_ref())
    }

    ///Equivalent of the `pointer_mut` method but takes `CStr` as a argument.
    pub unsafe fn ptr_or_null_mut_cstr<T>(&self, name: &CStr) -> Result<PtrOrNullMut<T> , Error> {
        let raw_ptr = match self.lib.symbol_cstr(name) {
            Ok(val) => val,
            Err(err) => match err {
                Error::NullSymbol => null_mut(),
                _ => return Err(err)
            }
        };
        Ok(PtrOrNullMut::new(raw_ptr))
    }

    ///This method is recommended for accessing statically allocated data that can't be modified.
    pub unsafe fn reference<T>(&self, name: &str) -> Result<&T, Error> {
        self.lib.symbol(name)
    }

    ///Equivalent of the `reference` method but takes `CStr` as a argument.
    pub unsafe fn reference_cstr<T>(&self, name: &CStr) -> Result<&T, Error> {
        self.lib.symbol_cstr(name)
    }

    ///This method is recommended for accessing statically allocated data that can be modified.
    pub unsafe fn reference_mut<T>(&self, name: &str) -> Result<&mut T, Error> {
        self.lib.symbol(name)
    }

    ///Equivalent of the `reference_mut` method but takes `CStr` as a argument.
    pub unsafe fn reference_mut_cstr<T>(&self, name: &CStr) -> Result<&mut T, Error> {
        self.lib.symbol_cstr(name)
    }
}

unsafe impl Send for Library {}
unsafe impl Sync for Library {}