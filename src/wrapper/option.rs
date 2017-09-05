use super::api::WrapperApi;
use super::super::raw::Library;
use super::super::Error;

impl<T> WrapperApi for Option<T>
where
    T: WrapperApi,
{
    unsafe fn load(lib: &Library) -> Result<Self, Error> {
        match T::load(lib) {
            Ok(val) => Ok(Some(val)),
            Err(_) => Ok(None),
        }
    }
}
