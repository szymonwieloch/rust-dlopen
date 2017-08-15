extern crate libc;

mod err;
mod library;
mod symbols;
mod api;
mod wrapper;
mod drop;
mod dlopen;
mod interface;

pub use library::Library;
pub use err::{Error, DlError};
pub use symbols::{Symbol, Pointer, FromRawPointer};
pub use api::LibraryApi;
pub use drop::DlDrop;
pub use dlopen::DlOpen;
pub use wrapper::LibraryWrapper;
pub use interface::{LibraryInterface, Wrapper};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
