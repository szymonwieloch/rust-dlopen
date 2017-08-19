use super::pointer::RawPointer;
use super::super::err::Error;
pub type RawResult<'a> = Result<RawPointer<'a>, Error>;

pub trait FromRawResult where Self: Sized {
    unsafe fn from_raw_result(raw: RawResult) -> Result<Self, Error>;
}