/*!

Library for opening and working with dynamic link libraries (also known as shared object).

# Example of problems with loading dynamic link libraries

Opening and using dynamic link libraries can be a tricky thing. Let's take a look at a typical
piece of code in C language:

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

# Purpose

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

# Comparison of APIs:

* [**raw**](./raw/index.html) - a low-level API. It is mainly intended to give you full flexibility
    if you decide to create you own custom solution for handling dynamic link libraries.
    For typical operations you probably should use one of high-level APIs.

* [**symbor**](./symbor/index.html) - a high-level API. It prevents dangling symbols by creating
    zero cost structural wrappers around symbols obtained from the library. These wrappers use
    Rust borrowing mechanism to make sure that the library will never get released before obtained symbols.

* [**wrapper**](./symbor/index.html) - a high-level API. It prevents dangling symbols by creating
    zero cost functional wrappers around symbols obtained from the library. These wrappers prevent
    accidental copying of raw symbols from library API. Dangling symbols are prevented by keeping
    library and its API in one structure - this makes sure that symbols and library are released together.

Additionally both high-level APIs provide a way to automatically load symbols into a structure using
Rust reflection mechanism. Decision witch API should be used is a matter of taste - please check
documentation of both of them and use the one that you prefer.
At the moment none seems to have any reasonable advantage over the other.

*/


#[cfg(any(unix, test))]
extern crate libc;
#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
extern crate kernel32;
#[cfg(unix)]
#[macro_use]
extern crate lazy_static;

#[cfg(test)]
#[macro_use]
extern crate const_cstr;
#[cfg(test)]
mod tests;

pub mod raw;
pub mod symbor;
pub mod utils;
pub mod wrapper;
mod err;
pub use err::Error;
