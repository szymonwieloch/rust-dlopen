mod common;
#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod windows;

pub use self::common::DynLib;