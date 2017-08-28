#rust-dynlib examples

Files in directory perform very similar operations
but they use different APIs. You can compare these
approaches and choose the API that suits your needs.
Operations include calling both Rust and C functions,
access to constant and mutable static data,
modifying mutable data and operations on common data types.

All examples use an example library that gets built
together with this project. It covers most types
of exported symbols, therefor it allows you to check
if the library actually obtains correctly all kinds 
of symbols.

**Note:** Rust has still a [**bug**](https://github.com/rust-lang/rust/issues/28794) that results in 
generating invalid dynamic link libraries.
On OSX you can expect to have a crash when the
library gets unloaded. Please notice that this
bug is related to building dynamic link libraries
 (in this case the example library), not to loading
 libraries.
 If you userust-dynlib for working with correctly built
dynamic link libraries, everything should work
normally.