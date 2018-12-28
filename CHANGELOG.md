# rust-dlopen changelog

## 0.1.0

- Initial version. Three complete APIs, tested

## 0.1.1

- Removed warning during compilation in some rare cases
- Fixed code formatting using rustfmt

## 0.1.2

- Fixed synchronization issues on Windows

## 0.1.3

- Updated documentation - it is easier for users to understand the value of the library.

## 0.1.4

- Added badges showing the library quality.
- Fixed collision of the "Result" name. 
    Other possible sources of collisions removed too.
- Fixed small typos in error messages.

## 0.1.5

- Fixed possible name collision in generated code- big thanks to kzys for finding it!


## 0.1.6
- Fixed typo in "which" (docs)
- Fixed build that stopped working for rust 1.18.0
- Added code coverage (codedov.io), added badge
- Fixed tests on certain MAC OS platforms - added recursive search
    for a built test library.