use super::super::err::Error;
use std::ffi::{CStr, CString, OsStr};

//choose the right platform implementation here
#[cfg(unix)]
use super::unix::{close_lib, get_sym, open_self, open_lib, addr_info_obtain, addr_info_init, addr_info_cleanup, Handle};
#[cfg(windows)]
use super::windows::{close_lib, get_sym, open_self, open_lib, addr_info_obtain, addr_info_init, addr_info_cleanup, Handle};

use std::mem::{size_of, transmute_copy};

/**
Main interface for opening and working with a dynamic link library.

**Note:** Several methods have their "*_cstr" equivalents. This is because all native OS
interfaces actually use C-strings. If you pass
[`CStr`](https://doc.rust-lang.org/std/ffi/struct.CStr.html)
as an argument, Library doesn't need to perform additional conversion from Rust string to
C-string.. This makes `*_cstr" functions slightly more optimal than their normal equivalents.
It is recommended that you use
[const-cstr](https://github.com/abonander/const-cstr) crate to create statically allocated
C-strings.

**Note:** The handle to the library gets released when the library object gets dropped.
Unless your application opened the library multiple times, this is the moment when symbols
obtained from the library become dangling symbols.
*/
#[derive(Debug)]
pub struct Library {
    handle: Handle,
}

impl Library {
    /**
    Open a dynamic library.

    **Note:** different platforms search for libraries in different directories.
    Therefore this function cannot be 100% platform independent.
    However it seems that all platforms support the full path and
    searching in default os directories if you provide only the file name.
    Please refer to your operating system guide for precise information about the directories
    where the operating system searches for dynamic link libraries.

    # Example

    ```no_run
    extern crate dlopen;
    use dlopen::raw::Library;

    fn main() {
        //use full path
        let lib = Library::open("/lib/i386-linux-gnu/libm.so.6").unwrap();
        //use only file name
        let lib = Library::open("libm.so.6").unwrap();
    }
    ```
    */
    pub fn open<S>(name: S) -> Result<Library, Error>
    where
        S: AsRef<OsStr>,
    {
        Ok(Self {
            handle: unsafe { open_lib(name.as_ref()) }?,
        })
    }

    /**
    Open the main program itself as a library.

    This allows a shared library to load symbols of the program it was loaded
    into.
    */
    pub fn open_self() -> Result<Library, Error> {
        Ok(Self {
            handle: unsafe { open_self() }?,
        })
    }

    /**
    Obtain symbol from opened library.

    **Note:** the `T` template type needs to have a size of a pointer.
    Because Rust does not support static casts at the moment, the size of the type
    is checked in runtime and causes panic if it doesn't match.

    **Note:** It is legal for a library to export null symbols.
    However this is something that almost nobody expects.
    Therefore allowing it here would bring many problems, especially if user obtains references
    or functions.
    This method checks the address value and returns `Error::NullSymbol` error if the value is null.
    If your code does require obtaining symbols with null value, please do something like this:

    # Example

    ```no_run
    extern crate dlopen;
    use dlopen::raw::Library;
    use dlopen::Error;
    use std::ptr::null;
    fn main(){
        let lib = Library::open("libyourlib.so").unwrap();
        let ptr_or_null: * const i32 = match unsafe{ lib.symbol("symbolname") } {
            Ok(val) => val,
            Err(err) => match err {
                Error::NullSymbol => null(),
                _ => panic!("Could not obtain the symbol")
            }
        };
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
            panic!(
                "The type passed to dlopen::Library::symbol() function has a different size than a \
                 pointer - cannot transmute"
            );
        }
        let raw = get_sym(self.handle, name)?;
        if raw.is_null() {
            return Err(Error::NullSymbol);
        } else {
            Ok(transmute_copy(&raw))
        }
    }
}

impl Drop for Library {
    fn drop(&mut self) {
        self.handle = close_lib(self.handle);
    }
}

unsafe impl Sync for Library {}
unsafe impl Send for Library {}

///Container for information about overlapping symbol from dynamic load library.
#[derive(Debug)]
pub struct OverlappingSymbol{
    ///Overlapping symbol name
    pub name: String,
    /// Overlapping symbol address
    pub addr: * const ()
}

/// Container for information about an address obtained from dynamic load library.
#[derive(Debug)]
pub struct AddressInfo {
    /// Path to the library that is the source of this symbol.
    pub dll_path: String,
    /// Base address of the library that is the source of this symbol.
    pub dll_base_addr: * const (),
    /// Information about the overlapping symbol from the dynamic load library.
    ///
    /// The information is optional since the given address may not overlap with any symbol.
    pub overlapping_symbol: Option<OverlappingSymbol>
}

///Obtains information about an address previously loaded from a dynamic load library.
pub struct AddressInfoObtainer {
}

impl AddressInfoObtainer {
    pub fn new() -> AddressInfoObtainer {
        unsafe{addr_info_init()};
        AddressInfoObtainer{}
    }

    /**
    Obtains information about an address previously loaded from a dynamic load library.

    # Example

    ```no_run
    extern crate dlopen;
    use dlopen::raw::{Library, AddressInfoObtainer};
    fn main() {
        let lib = Library::open("libyourlib.so").unwrap();
        let ptr: * const i32 = unsafe{ lib.symbol("symbolname") }.unwrap();

        // now we can obtain information about the symbol - library, base address etc.
        let aio = AddressInfoObtainer::new();
        let addr_info = aio.obtain(ptr as * const ()).unwrap();
        println!("Library path: {}", &addr_info.dll_path);
        println!("Library base address: {:?}", addr_info.dll_base_addr);
        if let Some(os) = addr_info.overlapping_symbol{
            println!("Overlapping symbol name: {}", &os.name);
            println!("Overlapping symbol address: {:?}", os.addr);
        }

    }
    ```
    */
    pub fn obtain(&self, addr: * const ()) -> Result<AddressInfo, Error>{
        unsafe {addr_info_obtain(addr)}
    }
}

impl Drop for AddressInfoObtainer{
    fn drop(&mut self) {
        unsafe{addr_info_cleanup()}
    }
}
