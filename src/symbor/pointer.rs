use std::marker::PhantomData;
use std::ops::Deref;
use libc::c_void;
use super::from_raw::{FromRawResult, RawResult};
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

impl<'lib, T> FromRawResult for Pointer<'lib, T> {
    unsafe fn from_raw_result(raw_result: RawResult) -> Result<Self, Error> {
        match raw_result {
            Ok(ptr) => Ok(Pointer{
                    pointer: *ptr as * const T,
                    pd: PhantomData
                }),
            Err(err) => Err(err)
        }
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
