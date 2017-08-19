/*!
This is low-level API for opening and getting symbols from dynamic libraries.
It is supposed to cover all existing platforms, be thread-safe and provide consistent Rust interface.
As a low-level API it does not guarantee full safety.

The main problems with this API may be dangling symbols after closing the library.
Use other APIs for solving this issue.
Other thing that is not performed by this API is automation of obtaining symbols -
you need to manually write a lot of code.
This API however is the only one that provides full flexibility.
# Example
```
extern crate dynlib;
extern crate libc;
use dynlib::lowlevel::DynLib;
 use libc::{c_double};
fn main(){
//This is a Ubuntu specific example but you should get the idea.
    let lib = DynLib::open("libm.so.6").unwrap();
    let cos: unsafe extern "C" fn(c_double)->c_double = unsafe{lib.symbol("cos")}.unwrap();
    println!("cos(1) = {}", unsafe{cos(1.0)});

    drop(lib);
    //warning! cos is now a dangling symbol and use of it may crash your application.
}
```
*/



//!

mod common;
#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod windows;
#[cfg(test)]
mod tests;

pub use self::common::DynLib;