/*!
High-level and safe API for opening and getting symbols from dynamic libraries.
It is based on symbol borrowing mechanism and supports automatic loading of symbols into structures.

This API uses Rust borrowing mechanism to prevent problem with dangling symbols
that take place when the library gets closed but the symbols still exist and are used.

#Example of a dangling symbol prevention
```
extern crate dynlib;
extern crate libc;
use dynlib::symbor::{Library, Symbol};
 use libc::{c_double};
fn main(){
    //This is a Linux specific example because existing libraries depend on OS.
    //But you should get an idea how it works on other platforms.
    #[cfg(not(target_os="linux"))]
    return;
    let lib = Library::open("libm.so.6").unwrap();
    let cos = unsafe{lib.symbol::<unsafe extern "C" fn(c_double)->c_double>("cos")}.unwrap();
    println!("cos(1) = {}", unsafe{cos(1.0)});

    //this would not compile:
    //drop(lib);
}
```

It also allows automatic loading of symbols into a structure.
This is especially handy if you have a huge API with multiple symbols:

# Example of automatic symbol loading:
```
#[macro_use]
extern crate dynlib_derive;
extern crate dynlib;
extern crate libc;
use dynlib::symbor::{Library, Symbol, LibraryApi};
 use libc::{c_double};

 #[derive(LibraryApi)]
 struct LibM<'a> {
    pub cos: Symbol<'a, unsafe extern "C" fn(c_double)->c_double>
 }

fn main(){
    //This is a Linux specific example because existing libraries depend on OS.
    //But you should get an idea how it works on other platforms.
    #[cfg(not(target_os="linux"))]
    return;
    let libm = Library::open("libm.so.6").expect("Could not open library");
    let api = unsafe{LibM::load(&libm)}.expect("Could not load symbols");
    println!("cos(1) = {}", unsafe{(api.cos)(1.0)});

    //this would not compile:
    //drop(lib);
}
```
Original idea for this solution comes from Simonas Kazlauskas and his crate
[libloading](https://github.com/nagisa/rust_libloading).
Many improvements were added to solve several issues. This API has two kinds of known problems:

* It is still possible to convert wrappers of symbols into primitive types and therefore it
    is still possible to have a dangling symbol. But it is **much** harder to make this mistake.
* It doesn't go well with object-oriented programming because Rust disallows
    stuctures to have fields with references between them.

*/

mod ptr_or_null;
mod ptr_or_null_mut;
mod symbol;
mod from_raw;
mod library;
mod option;
mod reference;
mod reference_mut;
mod api;
mod wrapper;

pub use self::library::Library;
pub use self::symbol::Symbol;
pub use self::api::LibraryApi;
pub use self::ptr_or_null::PtrOrNull;
pub use self::ptr_or_null_mut::PtrOrNullMut;
pub use self::reference::Ref;
pub use self::reference_mut::RefMut;
pub use self::from_raw::FromRawResult;

