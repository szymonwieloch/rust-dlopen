use super::super::lowlevel::DynLib;
use super::super::Error;
use std::ops::{Deref, DerefMut};
use super::api::WrapperApi;
use std::ffi::{OsStr};

pub struct WrapperOptional<Api, Optional> where Api: WrapperApi, Optional: WrapperApi {
    #[allow(dead_code)] //this is not dead code because destructor of DynLib deallocates the library
    lib: DynLib,
    api: Api,
    optional: Option<Optional>
}

impl<Api, Optional> WrapperOptional<Api, Optional> where Api: WrapperApi, Optional: WrapperApi {
    ///Open the library using provided file name or path and load all symbols.
    pub unsafe fn open<S>(name: S) -> Result<WrapperOptional<Api, Optional>, Error>  where S: AsRef<OsStr> {
        let lib = DynLib::open(name)?;
        let api = Api::load(&lib)?;
        let optional = match Optional::load(&lib) {
            Ok(val) => Some(val),
            Err(_) => None
        };
        Ok(Self{
            lib: lib,
            api: api,
            optional: optional
        })
    }

    pub fn optional(&self) -> &Option<Optional> {
        return &self.optional
    }
    
    pub fn optional_mut(&mut self) -> &Option<Optional> {
        return &mut self.optional
    }
}

impl<Api, Optional> Deref for WrapperOptional<Api, Optional> where Api: WrapperApi, Optional: WrapperApi{
    type Target = Api;
    fn deref(&self) -> &Api {
        &self.api
    }
}

impl<Api, Optional> DerefMut for WrapperOptional<Api, Optional> where Api: WrapperApi, Optional: WrapperApi{
    fn deref_mut(&mut self) -> &mut Api {
        &mut self.api
    }
}