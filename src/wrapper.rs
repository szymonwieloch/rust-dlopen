use super::err::Error;

pub trait LibraryWrapper where Self: Sized {
    unsafe fn load(lib_name: &str) -> Result<Self, Error>;
}