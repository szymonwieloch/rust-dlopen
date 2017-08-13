use std::marker::PhantomData;
use std::ops::Deref;
use libc::c_void;
use super::from_raw::FromRawPointer;
use super::super::err::Error;


#[derive(Debug, Clone, Copy)]
pub struct RawPointer<'lib> {
    pointer: * const c_void,
    pd: PhantomData<&'lib c_void>
}

impl<'lib> RawPointer<'lib> {
    pub fn new(pointer: * mut c_void) -> RawPointer<'lib> {
        RawPointer{
            pointer: pointer,
            pd: PhantomData
        }
    }
}

impl<'lib> FromRawPointer for RawPointer<'lib> {
    type Error = Error;
    unsafe fn from_raw_ptr(raw: RawPointer) -> Result<Self, Self::Error> {
        Ok(RawPointer{
            pointer: *raw,
            pd: PhantomData
        })
    }
}

impl<'lib> Deref for RawPointer<'lib> {
    type Target = *const c_void;
    fn deref(&self) -> & * const c_void {
        &self.pointer
    }
}

unsafe impl<'lib> Send for RawPointer<'lib> {}
unsafe impl<'lib> Sync for RawPointer<'lib> {}