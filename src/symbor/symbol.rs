use std::marker::PhantomData;
use std::ops::Deref;
use libc::c_void;
use std::mem::transmute_copy;
use super::from_raw::{FromRawResult, RawResult};
use super::super::err::Error;


#[derive(Debug, Clone, Copy)]
pub struct Symbol<'lib, T: 'lib> {
    symbol: T,
    pd: PhantomData<&'lib T>
}

impl<'lib, T> Symbol<'lib, T> {
    pub fn new(symbol: T) -> Symbol<'lib, T> {
        Symbol{
            symbol: symbol,
            pd: PhantomData
        }
    }
}

impl<'lib, T> FromRawResult for Symbol<'lib, T> {
    unsafe fn from_raw_result(raw_result: RawResult) -> Result<Self, Error> {
        match raw_result {
            Ok(ptr) => if ptr.is_null(){
                Err(Error::NullSymbol)
            } else {
                let raw: * const c_void = *ptr;
                Ok(Symbol {
                    symbol: transmute_copy(&raw),
                    pd: PhantomData
                })
            },
            Err(err) => Err(err)
        }

    }
}

impl<'lib, T> Deref for Symbol<'lib, T> {
    type Target =  T;
    fn deref(&self) -> & T {
        return &self.symbol
    }
}

unsafe impl<'lib, T: Send> Send for Symbol<'lib, T> {}
unsafe impl<'lib, T: Sync> Sync for Symbol<'lib, T> {}