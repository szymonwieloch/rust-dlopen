extern crate dynlib;
extern crate libc;
#[macro_use]
extern crate const_cstr;
use dynlib::symbor::{Library};
use dynlib::utils::platform_file_name;
use libc::{c_char};
use std::env;
use std::path::PathBuf;

fn main() {
    //build path to the example library that covers most cases
    let mut lib_path = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    lib_path.extend(["target", "debug", "deps"].iter());
    lib_path.push(platform_file_name("example"));
    println!("Library path: {}", lib_path.to_str().unwrap());
    let lib = Library::open(lib_path).expect("Could not open library");

    let rust_fun_print_something= unsafe { lib.symbol_cstr::<fn()>(const_cstr!("rust_fun_print_something").as_cstr())}.unwrap();
    rust_fun_print_something();
    //This would not compile because rust_fun_print_something has a reference to lib lifetime
    //drop(lib);

    //Let's try other methods

    let rust_i32_mut= unsafe { lib.reference_mut_cstr::<i32>(const_cstr!("rust_i32_mut").as_cstr())}.unwrap();
    *rust_i32_mut += 14;

    let c_const_char_ptr = unsafe { lib.ptr_or_null_cstr::<c_char>(const_cstr!("c_const_char_ptr").as_cstr())}.unwrap();

    //unfortunately we can't forbid copying of pointers in Rust:
    let _str_copy: * const c_char = * c_const_char_ptr;
    let _fun_copy: fn() = *rust_fun_print_something;
    //So it's not perfectly safe, but much safer than using the lowlevel API.

}