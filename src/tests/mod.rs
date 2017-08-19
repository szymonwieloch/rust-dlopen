///This module performs integration tests using the dependendent example library module
mod lowlevel;

use std::env;
use std::path::PathBuf;
use super::utils::platform_file_name;
use std::ffi::OsString;
use libc::{c_int};

pub fn example_lib_path() -> OsString {
    let mut lib_path = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    lib_path.extend(["target", "debug", "deps"].iter());
    lib_path.push(platform_file_name("example"));
    lib_path.into_os_string()
}

#[repr(C)]
pub struct SomeData {
    first: c_int,
    second: c_int
}