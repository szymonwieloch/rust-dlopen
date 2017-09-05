# rust-dlopen

[![Travis CI][tcii]][tci] [![Appveyor CI][acii]][aci] [![Crates CI][ccii]][cci]  

[tcii]: https://travis-ci.org/szymonwieloch/rust-dlopen.svg?branch=master
[tci]: https://travis-ci.org/szymonwieloch/rust-dlopen
[acii]: https://ci.appveyor.com/api/projects/status/github/szymonwieloch/rust-dlopen?svg=true
[aci]: https://ci.appveyor.com/project/szymonwieloch/rust-dlopen
[ccii]: https://img.shields.io/crates/v/dlopen.svg
[cci]: https://crates.io/crates/dlopen

A long time ago in a dirty basement far, far away a programmer was trying to dynamically load a library using standard C API (because he didnt know Rust yet):

```c
/* Open a dynamic library, get function, use it and then close the library ... */
//Problem: this is Unix API, won't work on Windows
#include <dlfcn.h>
#include <stdio.h>
void main() {
    //Problem: no type safety
    void *mylib;
    int eret;
    //Problem: dlopen() accepts int as a second argument, so you can accidentally pass here almost anything
    mylib = dlopen("libm.so", RTLD_LOCAL | RTLD_LAZY);
    //Problem: forgot to check returned value

    void (*cos)(double);

    //Problem: no way to actually check symbol type
    //Second problem: no error checking
    //Third problme: NULL is a legal value of a pointer exported by the library.
    //You always need to check it before converting into a function.
    *(void**)(&my_function) = dlsym(handle,"something");

    printf("cos(1.0)=%f", cos(1.0));

    //Problem: returned value not checked
    dlclose(mylib);

    //Problem: now the library is closed, but the function cos still exists.
    //This is a dangling symbol problem and may result in crashes
}
```

Basically use of dynamic link libraries is **extremely prone to errors**
and requires **a lot of coding** to perform even the simplest operations.

This library aims to simplify the process of developing APIs for dynamically loaded libraries in Rust
language and to reduce number of mistakes (please note that you can't create any library that is 100% safe because
loading libraries requires transmutation of obtained pointers).

# Main features

* Supports majority of platforms and is platform independent.
* Is consistent with Rust error handling mechanism and makes making mistakes much more difficult.
* Is very lightweight. It mostly uses zero cost wrappers to create safer abstractions over platform API.
* Is thread safe.
* Is object-oriented programming friendly.
* Has a low-level API that provides full flexibility of using libraries.
* Has two high-level APIs that protect against dangling symbols - each in its own way.
* High level APIs support automatic loading of symbols into structures. You only need to define a
    structure that represents an API. The rest happens automatically and requires only minimal amount of code.

# Usage:
Cargo.toml:
```toml
[dependencies]
dlopen = "0.1"
```

# Documentation
    
[Cargo documentation](https://docs.rs/dlopen)
    
[Examples](./examples)

[Changelog](./CHANGELOG.md)
    
# License
This code is licensed under [MIT](./LICENSE) license.
