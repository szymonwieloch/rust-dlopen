use std::marker::PhantomData;
use std::ops::Deref;

#[derive(Debug, Clone, Copy)]
pub struct Symbol<'lib, T: 'lib> {
    symbol: * mut T,
    pd: PhantomData<&'lib T>
}

impl<'lib, T> Symbol<'lib, T> {
    pub fn new(symbol: * mut T) -> Symbol<'lib, T> {
        Symbol{
            symbol: symbol,
            pd: PhantomData
        }
    }
}
/*
impl<'lib, T> Deref for Symbol<'lib, T> {
    type Target = T;
    fn deref(&self) -> *mut T {
        self.symbol
    }
}*/
/*
impl<'lib, T> fmt::Debug for Symbol<'lib, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.fmt(f)
    }
}
*/

unsafe impl<'lib, T: Send> Send for Symbol<'lib, T> {}
unsafe impl<'lib, T: Sync> Sync for Symbol<'lib, T> {}