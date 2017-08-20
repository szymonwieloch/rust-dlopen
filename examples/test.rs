#[macro_use]
extern crate dynlib_derive;
extern crate dynlib;
extern crate libc;
use dynlib::symbor::{Library, Symbol, LibraryApi, PtrOrNull, RefMut, PtrOrNullMut};
use libc::{c_double, c_char};

#[derive(LibraryApi)]
struct Example<'a> {
    pub simple_fun: Symbol<'a, unsafe extern "C" fn()>,
    pub complex_fun: Symbol<'a, unsafe extern "C" fn(c_double)->c_double>,
    pub optional_fun: Option<Symbol<'a, unsafe extern "C" fn()>>,
    pub nullable_ptr: PtrOrNullMut<'a, c_char>,
    pub mut_ref_i32: Symbol<'a, &'a mut i32>,
    #[dynlib_name="mut_ref_i32"]
    pub the_same_mut_ref_i32: RefMut<'a, i32>,
    pub not_nullable_ptr: Symbol<'a, * mut c_double>
}

fn main(){
    let lib = Library::open("example.dll").expect("Could not open library");
    let mut api = unsafe{Example::load(&lib)}.expect("Could not load symbols");

    //now we can do something with loaded symbols.
    unsafe{(api.simple_fun)()};
    let _ = unsafe{(api.complex_fun)(1.0)};
    match api.optional_fun {
        Some(fun) => unsafe {fun()},
        None => println!("Optional function could not be loaded"),
    };
    if api.nullable_ptr.is_null(){
        println!("Library has a null symbol");
    }
    //while Symbol is good for everything, RefMut requires one less dereference to use
    **api.mut_ref_i32 =34;
    *api.the_same_mut_ref_i32 =35;
    unsafe{**api.not_nullable_ptr = 55.0};
    unsafe{**api.nullable_ptr = 0};
}