use std::marker::PhantomData;
use std::ops::Deref;
use libc::c_void;
use std::mem::transmute;
use std::convert::From;
use super::raw::RawPointer;
use super::from_raw::FromRawPointer;
use super::super::err::Error;

#[derive(Debug, Clone, Copy)]
pub struct Pointer<'lib, T: 'lib> {
    pointer: * const c_void,
    pd: PhantomData<&'lib T>
}

impl<'lib, T> Pointer<'lib, T> {
    pub fn new(pointer: * mut c_void) -> Pointer<'lib, T> {
        Pointer{
            pointer: pointer,
            pd: PhantomData
        }
    }
}

impl<'lib, T> From<RawPointer<'lib>> for Pointer<'lib, T> {
    fn from(raw: RawPointer<'lib>) -> Self {
        Pointer{
            pointer: *raw,
            pd: PhantomData
        }
    }
}

impl<'lib, T> FromRawPointer for Pointer<'lib, T> {
    type Error = Error;
    unsafe fn from_raw_ptr(raw: RawPointer) -> Result<Self, Self::Error> {
        Ok(Pointer{
            pointer: *raw,
            pd: PhantomData
        })
    }
}



impl<'lib, T> Deref for Pointer<'lib, T> {
    type Target = *const T;
    fn deref(&self) -> & * const T {
        unsafe {transmute(&self.pointer) }
    }
}

unsafe impl<'lib, T: Send> Send for Pointer<'lib, T> {}
unsafe impl<'lib, T: Sync> Sync for Pointer<'lib, T> {}
