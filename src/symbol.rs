use std::marker::PhantomData;
use std::ops::Deref;
use libc::c_void;
use std::mem::transmute;

#[derive(Debug, Clone, Copy)]
pub struct Symbol<'lib, T: 'lib> {
    symbol: * mut c_void,
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

impl<'lib, T> Deref for Symbol<'lib, T> {
    type Target =  T;
    fn deref(&self) -> & T {
        unsafe { transmute(&self.symbol) }
    }
}

unsafe impl<'lib, T: Send> Send for Symbol<'lib, T> {}
unsafe impl<'lib, T: Sync> Sync for Symbol<'lib, T> {}