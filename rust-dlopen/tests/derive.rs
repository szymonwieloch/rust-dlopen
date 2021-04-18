
extern crate dlopen;

// Don't need to re-import dlopen-derive as long as the derive feature is enabled.
use dlopen::wrapper::WrapperApi;

#[derive(WrapperApi)]
struct Api {
    bruh: fn()
}