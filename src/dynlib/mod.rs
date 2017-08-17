mod common;
#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod windows;
#[cfg(test)]
mod tests;

pub use self::common::DynLib;