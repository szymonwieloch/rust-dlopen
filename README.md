# rust-dlopen

[![Travis CI][tcii]][tci] [![Appveyor CI][acii]][aci] [![Crates CI][ccii]][cci]  [![Codedov CI][vcii]][vci]  [![Docs][dcii]][dci]

[tcii]: https://travis-ci.org/szymonwieloch/rust-dlopen.svg?branch=master
[tci]: https://travis-ci.org/szymonwieloch/rust-dlopen
[acii]: https://ci.appveyor.com/api/projects/status/github/szymonwieloch/rust-dlopen?svg=true
[aci]: https://ci.appveyor.com/project/szymonwieloch/rust-dlopen
[ccii]: https://img.shields.io/crates/v/dlopen.svg
[cci]: https://crates.io/crates/dlopen
[vcii]: https://codecov.io/api/gh/szymonwieloch/rust-dlopen/branch/master/graph/badge.svg
[vci]: https://codecov.io/gh/szymonwieloch/rust-dlopen
[dcii]: https://docs.rs/dlopen/badge.svg
[dci]: https://docs.rs/dlopen

# Overview

This library is my effort to make use of dynamic link libraries in Rust simple.
Previously existing solutions were either unsafe, provided huge overhead of required writing too much code to achieve simple things.
I hope that this library will help you to quickly get what you need and avoid errors.

# Quick example

```rust
extern crate dlopen;
#[macro_use]
extern crate dlopen_derive;
use dlopen::wrapper::{Container, WrapperApi};

#[derive(WrapperApi)]
struct Api<'a> {
    example_rust_fun: fn(arg: i32) -> u32,
    example_c_fun: unsafe extern "C" fn(),
    example_reference: &'a mut i32,
}

fn main(){
    let mut cont: Container<Api> =
        unsafe { Container::load("libexample.so") }.expect("Could not open library or load symbols");
    cont.example_rust_fun(5);
    unsafe{cont.example_c_fun()};
    *cont.example_reference_mut() = 5;
}
```


# Features

## Main features

* Supports majority of platforms and is platform independent.
* Is consistent with Rust error handling mechanism and makes making mistakes much more difficult.
* Is very lightweight. It mostly uses zero cost wrappers to create safer abstractions over platform API.
* Is thread safe.
* Is object-oriented programming friendly.
* Has a low-level API that provides full flexibility of using libraries.
* Has two high-level APIs that protect against dangling symbols - each in its own way.
* High level APIs support automatic loading of symbols into structures. You only need to define a
    structure that represents an API. The rest happens automatically and requires only minimal amount of code.
* Automatic loading of symbols helps you to follow the DRY paradigm.

## Compare with other libraries

|Feature                             | dlopen     | [libloading](https://github.com/nagisa/rust_libloading) | [sharedlib](https://github.com/Tyleo/sharedlib) |
|------------------------------------|------------|---------------------------------------------------------|-------------------------------------------------|
| Basic functionality                | Yes        | Yes        | Yes       |
| Multiplatform                      | Yes        | Yes        | Yes       |
|Dangling symbol prevention          | Yes        | Yes        | Yes       |
| Thread safety                      | Yes        | **Potential problem with SetErrorMode() on older Windows platforms** | **No support for SetErrorMode (library may block the application on Windows)**|
| Loading of symbols into structures | Yes        | **No**     | **No**     
| Overhead                           | Minimal    | Minimal    | **Some overhead** |
| Low-level, unsafe API              | Yes        | Yes        | Yes       |
| Object-oriented friendly           | Yes        | **No**       | Yes     | 
| Load from the program itself       | Yes        | **No**       | **No**  |
| Obtaining address information (dladdr) | Yes    |  **Unix only** | **No**|

## Safety
	
Please note that while Rust aims at being 100% safe language, it does not yet provide mechanisms that would allow me to create a 100% safe library, so I had to settle on 99%.
Also the nature of dynamic link libraries requires casting obtained pointers into types that are defined on the application side. And this cannot be safe. 
Having said that I still think that this library provides the best approach and greatest safety possible in Rust.

# Usage:

Cargo.toml:

```toml
[dependencies]
dlopen = "0.1"
```

# Documentation
    
[Cargo documentation](https://docs.rs/dlopen)
    
# License
This code is licensed under [MIT](./LICENSE) license.

# Changelog

[GitHub changelog](https://github.com/szymonwieloch/rust-dlopen/releases)

# Acknowledgement

Special thanks to [Simonas Kazlauskas](https://github.com/nagisa) whose [libloading](https://github.com/nagisa/rust_libloading) became code base for my project.
