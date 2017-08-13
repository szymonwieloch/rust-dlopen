use std::marker::PhantomData;
use std::ops::Deref;
use libc::c_void;
use std::mem::transmute;
use std::convert::From;
use super::raw::RawPointer;
use super::from_raw::FromRawPointer;
use super::super::err::Error;


#[derive(Debug, Clone, Copy)]
pub struct Symbol<'lib, T: 'lib> {
    symbol: * const c_void,
    pd: PhantomData<&'lib T>
}

impl<'lib, T> Symbol<'lib, T> {
    pub fn new(symbol: * mut c_void) -> Symbol<'lib, T> {
        Symbol{
            symbol: symbol,
            pd: PhantomData
        }
    }
}

impl<'lib, T> From<RawPointer<'lib>> for Symbol<'lib, T> {
    fn from(raw: RawPointer<'lib>) -> Self {
        Symbol{
            symbol: *raw,
            pd: PhantomData
        }
    }
}

impl<'lib, T> FromRawPointer for Symbol<'lib, T> {
    type Error = Error;
    unsafe fn from_raw_ptr(raw: RawPointer) -> Result<Self, Self::Error> {
        Ok(Symbol{
            symbol: *raw,
            pd: PhantomData
        })
    }
}

impl<'lib, T> Deref for Symbol<'lib, T> {
    type Target =  T;
    fn deref(&self) -> & T {
        unsafe { transmute(&self.symbol) }
    }
}

unsafe impl<'lib, T: Send> Send for Symbol<'lib, T> {}
unsafe impl<'lib, T: Sync> Sync for Symbol<'lib, T> {}