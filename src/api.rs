use super::library::Library;
use super::err::Error;

pub trait LibraryApi<'a> where Self:Sized {
    unsafe fn load(lib: &'a Library) -> Result<Self, Error>;
}