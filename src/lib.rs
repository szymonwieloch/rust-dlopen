#[cfg(unix)]
extern crate libc;
#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
extern crate kernel32;
#[cfg(unix)]
#[macro_use]
extern crate lazy_static;

mod err;
mod dynlib;

pub use dynlib::DynLib;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
