use super::ptr_or_null::PtrOrNull;
use super::super::err::Error;
pub type RawResult<'a> = Result<PtrOrNull<'a, ()>, Error>;

///This trait needs to be implemented by all members of structures that implement
/// the `SymBorApi` trait. It is used to covert raw result obtained from library
/// into the given object accessible to the user.
pub trait FromRawResult where Self: Sized {
    unsafe fn from_raw_result(raw: RawResult) -> Result<Self, Error>;
}