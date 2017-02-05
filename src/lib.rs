#[cfg(unix)]
extern crate libc;

#[cfg(unix)]
mod unix_impl;

#[cfg(unix)]
pub use unix_impl::*;


