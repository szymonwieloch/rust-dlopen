use super::super::err::Error;
use super::super::lowlevel::DynLib;

pub trait WrapperApi where Self: Sized {
    unsafe fn load(lib: & DynLib ) -> Result<Self, Error>;
}