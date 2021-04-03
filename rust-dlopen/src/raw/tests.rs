use super::super::err::Error;
#[cfg(unix)]
use super::unix::{close_lib, get_sym, open_lib};
#[cfg(windows)]
use super::windows::{close_lib, get_sym, open_lib};

#[cfg(windows)]
const EXISTING_LIB: &str = "kernel32.dll";
#[cfg(all(unix, not(any(target_os = "macos", target_os = "ios"))))]
const EXISTING_LIB: &str = "libm.so.6";
#[cfg(any(target_os = "macos", target_os = "ios"))]
const EXISTING_LIB: &str = "libm.dylib";
const NOT_EXISTING_LIB: &str = "notexisting.ext";
#[cfg(windows)]
const_cstr! {EXISTING_SYM = "GetLastError";}
#[cfg(unix)]
const_cstr! {EXISTING_SYM = "cos";}
const_cstr! {NOT_EXISTING_SYM = "notexisting";}


//This is an example of opening and closing a library
//It's going to work only on Windows but this is what it is supposed to do
#[test]
fn load_get_close() {
    unsafe {
        let handle = open_lib(EXISTING_LIB.as_ref()).expect("Could not open library");
        let sym = get_sym(handle, &EXISTING_SYM.as_cstr()).expect("Could not get symbol");
        assert!(!sym.is_null());
        assert!(close_lib(handle).is_null());
    }
}

#[test]
fn open_err() {
    unsafe {
        match open_lib(NOT_EXISTING_LIB.as_ref()) {
            Ok(_) => panic!("Library should not get opened"),
            Err(err) => match err {
                Error::OpeningLibraryError(_) => (),
                _ => panic!("Invalid error kind"),
            },
        }
    }
}

#[test]
fn get_err() {
    unsafe {
        let handle = open_lib(EXISTING_LIB.as_ref()).expect("Could not open library");
        match get_sym(handle, NOT_EXISTING_SYM.as_cstr()) {
            Ok(_) => panic!("Should not get the symbol"),
            Err(err) => match err {
                Error::SymbolGettingError(_) => (),
                _ => panic!("Invalid error kind"),
            },
        }
        assert!(close_lib(handle).is_null());
    }
}
