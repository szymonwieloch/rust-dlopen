use super::api::WrapperApi;

/**
Allows creattion of complex, optional APIs.

Real life dynamic link libraries often come in multiple version. Sometimes additional functions
are added for the specific operating system, sometimes the library gets extended and new versions
can export more symbols. If you have only one optional part of the API, it is recommended to use `WrapperOptional`.
For more complex cases this is the API that is the most suitable.

`WrapperMultiApi` is inteded to be used together with the derive macro. You should create a new
structure where all fields implement the `WrapperApi` trait (this includes `Option<T>` where `T` implements `WrapperApi`).
The derive macro will generate required methods.

**WARNING!!!** Because of Rust lifetimes and borrowing rules structures created by calling the
`WrapperApi::load()` function won't have borrowed dependency on the library passed as an argument.
To prevent dangling symbols it is recommended that you use this trait together with the `Wrapper` structure.

```no_run
#[macro_use]
extern crate dynlib_derive;
extern crate dynlib;
use dynlib::wrapper::{Container, WrapperApi, WrapperMultiApi};

//Define 3 APIs:

#[derive(WrapperApi)]
struct Obligatory{
    some_fun: unsafe extern "C" fn()
}

#[derive(WrapperApi)]
struct Optional1<'a>{
    static_val: &'a i32
}

//this one wont' work in the example
#[derive(WrapperApi)]
struct Optional2{
   another_fun: unsafe extern "C" fn()
}

//Now define a multi wrapper that wraps sub APIs into one bigger API.
//This example assumes that the first API is obligatory and the other two are optional.

#[derive(WrapperMultiApi)]
struct Api<'a>{
    pub obligatory: Obligatory,
    pub optional1: Option<Optional1<'a>>,
    pub optional2: Option<Optional2>
}

fn main(){
    let mut container: Container<Api> = unsafe { Container::open("libexample.so")}.expect("Could not open library");

    //use obligatory API:
    unsafe{container.obligatory.some_fun()};

    //use first optional API:
    if let Some(ref opt) = container.optional1{
        let _val = *opt.static_val();
    }

    //use second optional API:
    if let Some(ref opt) = container.optional2{
        unsafe {opt.another_fun()};
    }
}
```
*/


pub trait WrapperMultiApi: WrapperApi {

}