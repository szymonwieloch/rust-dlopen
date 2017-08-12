extern crate libc;
mod err;
mod library;
mod symbol;

pub use library::Library;
pub use err::{Error, DlError};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
