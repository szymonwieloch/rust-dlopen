# rust-dlopen

**WARNING!!! This library is jus being developped and therefore is yet unstable!!!**

A long time ago in a dirty basement far, far away a programmer was trying to dynamically load a library using standard C API (because he didnt know Rust yet):

```c
/* Open a dynamic library, get function, use it and then close the library ... */
#include <dlfcn.h>
#include <stdio.h>
void main() {
    void *mylib;
    int eret;
    //Ups! dlopen() accepst int as a second argument, so you can accidentally pass here almost anything
    mylib = dlopen("libm.so", RTLD_LOCAL | RTLD_LAZY);
    //Ups! Forgot to check returned value.
    
    void (*cos)(double);

    //Ups! no way to actually check symbol type.
    //Double ups! No error checking.

    //Ups! NULL is a legal value of a pointer exported by the library.
    //You always need to check it before converting into a function.
    *(void**)(&my_function) = dlsym(handle,"something");

    printf("cos(1.0)=%f", cos(1.0));

    dlclose(mylib);

    //Ups! Now the library is closed, but the function cos still exists!
}
```

Basicly doing even the basic operations around dynamic libraries is **extreamly prone to errors** 
and requires **a lot of coding** to perform even the simplest operatoins. 
This library aims to simplify the process of developing APIs for dynamically loaded libraries
and to reduce number of mistakes (please note that you can't create any library that is 100% safe because
loading libraries requires transmutation of obtained pointers). 
The **dlopen** library is basicly a wrapper around the dl-something functions from the [libc](https://github.com/rust-lang/libc)
library.

## Features
* Supports majority of platforms (libc has wide support).
* Platform independent.
* Low-level API that is a much safer wrapper around dl-something functions.
* High-level API that ensures that symbols loaded from a library won't survive the library.
* High-leve API that automatically generates a lot of code for you and works much better with object-oriented approach.

## Low-level API
Let's just open a math library on Ubuntu(because most libraries are platform-specific, I will show you examples on my platform):

```rust
extern crate dlopen;
extern crate libc;
use dlopen::{DlOpen};
use libc::{c_double};

const LIB_NAME: &str = "libm.so.6";
fn main() {
    let libm = DlOpen::open(LIB_NAME).expect(&format!("Could not open {}", LIB_NAME));
    let cos: unsafe fn(c_double) -> c_double = unsafe {libm.symbol("cos")}.expect("cos not found");
    let arg = 2.0;
    let result = unsafe { cos(arg) };
    println!("cos({}) = {}", arg, result);
}
```

As you see lopening and closing libraries is quite straightforward. 
Rust `Result<>` together with `dlopen::Error` allow simple handling of errors.
`DlOpen` comes with two functions for opening library:

```rust
pub fn open(name: &str) -> Result<DlOpen, Error>;
pub fn open_cstr(name: &CStr) -> Result<DlOpen, Error>;
```

The first one accepts normal rusty '&str', the second one requires C-like string.
The second one actually is a little faster because it does not require conversion between Rust and C strings. 
You may check out the [const-cstr](https://github.com/abonander/const-cstr) that actually allows you to effectively use C strings in Rust code.
Usually one allocation is negligible when loading a library but depending on your code you may want to use the first or the second one.
Many functions in the `dlopen` crate follow the same pattern. Releasing the library is automatic thanks to `Drop` trait.

Obtaining symbols from the given library is also very simple. There are two methods for it:

```rust
pub unsafe fn symbol<T>(&self, name: &str) -> Result<T, Error>;
pub unsafe fn pointer<T>(&self, name: &str) -> Result<* const T, Error>;

```

And of course there are also their *_cstr equivalents. You may be wondering - why two methods?
The original API defines only `dlsym()` function.
There are two reasons. First is that `null` is a legal value of a pointer in a dynamically loaded library.
But it's totally unsafe to convert it into a function. Of course `null` is a legal value for a pointer.
The second reason is easyness of conversion. 
Please notice that a function pointer obtained from library does not point to a function.
It **IS** a function. 
So there is a different number of derefences that you would normally do for a pointer (for example a pointer to a C string)
than for a function. Therefor it's best to obtain functions using the `symbol()` method and pointers using the `pointer'() method.

```rust
unsafe {
    //Please notice automatic conversions that take place!

    //get a function
    let function: unsafe extern fn(c_double) = lib.symbol("function_name").unwrap();
    //get a pointer
    let pointer: *const c_char = lib.pointer("some_exported_string").unwrap();
    //this is equivalent to pointer() but also returns error if the value is null
    let safe_pointer: *const c_char = lib.symbol("some_exported_string").unwrap();
    
    //this would not compile - this prevents mistakes
    //let other_function: unsafe extern fn(c_double) = lib.pointer("other_function_name").unwrap();
}
```

Please notice that the `pointer()` method does not allow you to obtain mutable pointers. 
This was a conscious decision - you should not modify content of the pointer - these are library internals and write access to it would abort your application. There was no method to block if for `symbol()` method. 
Also you can try to obtain symbols that are bigger or smaller in size from pointers.
Unfortunately `static_cast` is not supported in Rust at the moment. 
However the library perform the check in runtime with zero overhead cost and it's going to panic if you try to do something nasty.