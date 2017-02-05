#[cfg(unix)]
extern crate libc;

#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
extern crate advapi32;

#[cfg(unix)]
mod unix_impl;

#[cfg(windows)]
mod windows_impl;

#[cfg(unix)]
pub use unix_impl::*;

#[cfg(windows)]
pub use windows_impl::*;
