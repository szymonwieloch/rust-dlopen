use std::marker::PhantomData;
use std::ops::Deref;
use libc::c_void;
use super::from_raw::FromRawPointer;
use super::super::err::Error;

#[derive(Debug, Clone, Copy)]
pub struct Pointer<'lib, T: 'lib> {
    pointer: * const T,
    pd: PhantomData<&'lib T>
}

pub type RawPointer<'lib> = Pointer<'lib, c_void>;

impl<'lib, T> Pointer<'lib, T> {
    pub fn new(pointer: * const T) -> Pointer<'lib, T> {
        Pointer{
            pointer: pointer,
            pd: PhantomData
        }
    }
}

impl<'lib, T> FromRawPointer for Pointer<'lib, T> {
    type Error = Error;
    unsafe fn from_raw_ptr(raw: RawPointer) -> Result<Self, Self::Error> {
        Ok(Pointer{
            pointer: *raw as * const T,
            pd: PhantomData
        })
    }
}

impl<'lib, T> Deref for Pointer<'lib, T> {
    type Target = *const T;
    fn deref(&self) -> & * const T {
        &self.pointer
    }
}

unsafe impl<'lib, T: Send> Send for Pointer<'lib, T> {}
unsafe impl<'lib, T: Sync> Sync for Pointer<'lib, T> {}
