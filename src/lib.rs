extern crate libc;

mod err;
mod library;
mod symbols;
mod api;

pub use library::Library;
pub use err::{Error, DlError};
pub use symbols::{Symbol, Pointer, RawPointer, FromRawPointer};
pub use api::LibraryApi;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
