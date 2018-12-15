extern crate dlopen;
extern crate libc;
extern crate regex;
use dlopen::utils::{PLATFORM_FILE_EXTENSION, PLATFORM_FILE_PREFIX};
use std::env;
use std::path::{Path, PathBuf};
use libc::c_int;
use std::fs;



pub fn example_lib_path() -> PathBuf {

    //Rust when building dependencies adds some weird numbers to file names
    // find the file using this pattern:
    //const FILE_PATTERN: &str = concat!(PLATFORM_FILE_PREFIX, "example.*\\.", PLATFORM_FILE_EXTENSION);
    let file_pattern = format!(
        r"{}example.*\.{}",
        PLATFORM_FILE_PREFIX,
        PLATFORM_FILE_EXTENSION
    );
    let file_regex = regex::Regex::new(file_pattern.as_ref()).unwrap();

    //find the directory with dependencies - there shold be our example library
    let mut deps_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    deps_dir.extend(["target", "debug", "deps"].iter());

    //unfortunately rust has no strict pattern of naming dependencies in this directory
    //this is partially platform dependent as there was a bug reported that while the code runs
    //well on Linux, Windows, it stopped working on a new version of Mac.
    //The only way to handle this correctly is by traversing the directory recursively and
    // finding a match.



    let lib_path = recursive_find(deps_dir.as_path(), &file_regex).expect(
        "Could not find the example library");
    println!("Library path: {}", lib_path.to_str().unwrap());
    lib_path
}

fn recursive_find(path: &Path, file_regex: &regex::Regex) -> Option<PathBuf> {
    if path.is_dir(){
        match fs::read_dir(path) {
            Result::Err(_) => None,
            Result::Ok(dir) => {
                for entry in dir.filter_map(Result::ok) {
                    if let Some(p) = recursive_find(&entry.path(), file_regex){
                        return Some(p);
                    }
                }
                None
            }
        }
    } else {
        if file_regex.is_match(path.file_name().unwrap().to_str().unwrap()){
            Some(path.to_path_buf())
        } else {
            None
        }
    }
}

#[repr(C)]
pub struct SomeData {
    pub first: c_int,
    pub second: c_int,
}
