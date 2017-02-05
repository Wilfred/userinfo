# userinfo
[![Build Status](https://travis-ci.org/Wilfred/userinfo.svg?branch=master)](https://travis-ci.org/Wilfred/userinfo)

This is a cross-platform crate for finding user attributes:

* login name
* full name
* home directory
* user ID
* group ID

## Example Usage

See [examples/main.rs](https://github.com/Wilfred/userinfo/blob/master/examples/main.rs).

## Related Projects

The [user crate](https://crates.io/crates/user) is cross-platform, but
only supports getting the login name.

The [users crate](https://crates.io/crates/users) (uses libc APIs
defined in POSIX) and
[users_native crate](https://crates.io/crates/users_native) (uses
`/etc/passwd`) both assume a unix environment.
