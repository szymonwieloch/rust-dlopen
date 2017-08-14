use super::pointer::RawPointer;

pub trait FromRawPointer where Self: Sized {
    type Error;
    unsafe fn from_raw_ptr(raw: RawPointer) -> Result<Self, Self::Error>;
}