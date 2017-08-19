#[cfg(any(unix, test))]
extern crate libc;
#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
extern crate kernel32;
#[cfg(unix)]
#[macro_use]
extern crate lazy_static;

#[cfg(test)]
#[macro_use]
extern crate const_cstr;
#[cfg(test)]
mod tests;

mod err;
pub mod lowlevel;
pub mod utils;

pub use err::Error;
