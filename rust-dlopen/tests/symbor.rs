#[macro_use]
extern crate const_cstr;
extern crate dlopen;
extern crate libc;
use dlopen::symbor::Library;
use libc::{c_char, c_int};
use std::ffi::CStr;

mod commons;
use commons::{example_lib_path, SomeData};

#[test]
fn open_play_close_symbor() {
    let lib_path = example_lib_path();
    let lib = Library::open(lib_path).expect("Could not open library");
    let rust_fun_print_something = unsafe {
        lib.symbol_cstr::<fn()>(const_cstr!("rust_fun_print_something").as_cstr())
    }.unwrap();
    rust_fun_print_something(); //should not crash
    let rust_fun_add_one = unsafe {
        lib.symbol_cstr::<fn(i32) -> i32>(const_cstr!("rust_fun_add_one").as_cstr())
    }.unwrap();
    assert_eq!(rust_fun_add_one(5), 6);

    let c_fun_print_something_else = unsafe {
        lib.symbol_cstr::<unsafe extern "C" fn()>(
            const_cstr!("c_fun_print_something_else").as_cstr(),
        )
    }.unwrap();
    unsafe { c_fun_print_something_else() }; //should not crash
    let c_fun_add_two = unsafe {
        lib.symbol_cstr::<unsafe extern "C" fn(c_int) -> c_int>(
            const_cstr!("c_fun_add_two").as_cstr(),
        )
    }.unwrap();
    assert_eq!(unsafe { c_fun_add_two(2) }, 4);
    let rust_i32: &i32 = unsafe { lib.reference_cstr(const_cstr!("rust_i32").as_cstr()) }.unwrap();
    assert_eq!(43, *rust_i32);
    let rust_i32_mut: &mut i32 =
        unsafe { lib.reference_mut_cstr(const_cstr!("rust_i32_mut").as_cstr()) }.unwrap();
    assert_eq!(42, *rust_i32_mut);
    *rust_i32_mut = 55; //should not crash
    //for a change use pointer to obtain its value
    let rust_i32_ptr =
        unsafe { lib.symbol_cstr::<*const i32>(const_cstr!("rust_i32_mut").as_cstr()) }.unwrap();
    assert_eq!(55, unsafe { **rust_i32_ptr });
    //the same with C
    let c_int: &c_int = unsafe { lib.reference_cstr(const_cstr!("c_int").as_cstr()) }.unwrap();
    assert_eq!(45, *c_int);
    //now static c struct

    let c_struct: &SomeData =
        unsafe { lib.reference_cstr(const_cstr!("c_struct").as_cstr()) }.unwrap();
    assert_eq!(1, c_struct.first);
    assert_eq!(2, c_struct.second);
    //let's play with strings

    let rust_str: &&str = unsafe { lib.reference_cstr(const_cstr!("rust_str").as_cstr()) }.unwrap();
    assert_eq!("Hello!", *rust_str);
    let c_const_char_ptr = unsafe {
        lib.symbol_cstr::<*const c_char>(const_cstr!("c_const_char_ptr").as_cstr())
    }.unwrap();
    let converted = unsafe { CStr::from_ptr(*c_const_char_ptr) }
        .to_str()
        .unwrap();
    assert_eq!(converted, "Hi!");
}
