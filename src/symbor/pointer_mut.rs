use std::marker::PhantomData;
use std::ops::Deref;
use super::from_raw::{FromRawResult, RawResult};
use super::super::err::Error;

#[derive(Debug, Clone, Copy)]
pub struct PointerMut<'lib, T: 'lib> {
    pointer: * mut T,
    pd: PhantomData<&'lib T>
}

impl<'lib, T> PointerMut<'lib, T> {
    pub fn new(pointer: * mut T) -> PointerMut<'lib, T> {
        PointerMut{
            pointer: pointer,
            pd: PhantomData
        }
    }
}

impl<'lib, T> FromRawResult for PointerMut<'lib, T> {
    unsafe fn from_raw_result(raw_result: RawResult) -> Result<Self, Error> {
        match raw_result {
            Ok(ptr) => Ok(PointerMut{
                    pointer: *ptr as * mut T,
                    pd: PhantomData
                }),
            Err(err) => Err(err)
        }
    }
}

impl<'lib, T> Deref for PointerMut<'lib, T> {
    type Target = *mut T;
    fn deref(&self) -> & * mut T {
        &self.pointer
    }
}

unsafe impl<'lib, T: Send> Send for PointerMut<'lib, T> {}
unsafe impl<'lib, T: Sync> Sync for PointerMut<'lib, T> {}
