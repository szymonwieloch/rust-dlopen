use super::super::err::{Error};
use std::ffi::{CString, CStr, OsStr};

//choose the right platform implementation here
#[cfg(unix)]
use super::unix::{open_lib, get_sym, close_lib, Handle};
#[cfg(windows)]
use super::windows::{open_lib, get_sym, close_lib, Handle};

use std::mem::{transmute_copy, size_of};

/**
    `DynLib` is the structure that represents low-level API.

    Several methods have their "*_cstr" equivalents. This is because all native OS interfaces
    actually use C-string. DynLib needs to perform additional conversion from Rust string to
    `std::ffi::CString` to be able to call those functions. This introduces certain overhead.
    Therefore you may consider using the "*_cstr" equivalents together with the
    [const-cstr](https://github.com/abonander/const-cstr) crate.
*/
#[derive(Debug)]
pub struct DynLib {
    handle: Handle
}

impl DynLib {
    /**
    Opens a dynamic library.

    **Note:** different platforms search for libraries in different directories.
    Therefore this function cannot be 100% platform independent.
    However it seems that all platforms support the full path and
    searching in default os directories if you provide only the file name.
    */
    pub  fn open<S>(name: S) -> Result<DynLib, Error> where S: AsRef<OsStr> {
;
        Ok(Self {
            handle: unsafe{open_lib(name.as_ref())}?
        })
    }
    /**
    Obtains symbol from opened library.

    **Note:** the `T` template type needs to have a size of a pointer.
    Because Rust does not support static casts at the moment, the size of the type
    is checked in runtime and causes panic if it doesn't match.

    **Note:** It is legal for a library to export null symbols.
    However this is something that almost nobody expects.
    Therefore allowing it here would bring many problems, especially if user obtains references
    or functions.
    This method checks the address value and returns `Error::NullSymbol` error if the value is null.
    If your code does require obtaining symbols with null value, please do something like this:
    ```no_run
    extern crate dynlib;
    use dynlib::lowlevel::DynLib;
    use dynlib::Error;
    use std::ptr::null;
    fn main(){
        let lib = DynLib::open("libyourlib.so").unwrap();
        let ptr_or_null: * const i32 = match lib.symbol("symbolname") {
            Ok(val) => val,
            Err(err) => match err {
                Error::NullSymbol => null(),
                _ => panic!("Could not obtain the symbol")
            }
        }
        //do something with the symbol
    }
    ```
    */
    pub unsafe fn symbol<T>(&self, name: &str) -> Result<T, Error> {
        let cname = CString::new(name)?;
        self.symbol_cstr(cname.as_ref())
    }
    ///Equivalent of the `symbol` method but takes `CStr` as a argument.
    pub unsafe fn symbol_cstr<T>(&self, name: &CStr) -> Result<T, Error> {
        //TODO: convert it to some kind of static assertion (not yet supported in Rust)
        //this comparison should be calculated by compiler at compilation time - zero cost
        if size_of::<T>() != size_of::<*mut ()>() {
            panic!("The type passed to dlopen::DynLib::symbol() function has a different size than a pointer - cannot transmute");
        }
        let raw = get_sym(self.handle, name)?;
        if raw.is_null() {
            return Err(Error::NullSymbol)
        } else {
            Ok(transmute_copy(&raw))
        }
    }
}

impl Drop for DynLib {
    fn drop(&mut self) {
        self.handle = close_lib(self.handle);
    }
}

unsafe impl Sync for DynLib {}
unsafe impl Send for DynLib {}
