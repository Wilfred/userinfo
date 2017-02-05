extern crate libc;

use std::ffi::CStr;
use libc::uid_t;

/// Return the UID associated with the current user.
pub fn current_user_id() -> uid_t {
    unsafe { libc::getuid() }
}

/// Return the login name of the current user.
pub fn current_username() -> String {
    username(current_user_id()).unwrap()
}

/// Return the login name of the user with the UID given. For example,
/// `"jsmith"`.
///
/// If the UID does not exist, or the username is invalid UTF-8,
/// returns None.
pub fn username(uid: uid_t) -> Option<String> {
    let passwd_info = unsafe { libc::getpwuid(uid) };
    if passwd_info.is_null() {
        return None;
    }
    let name_cstr = unsafe { CStr::from_ptr((*passwd_info).pw_name) };
    match name_cstr.to_str() {
        Ok(name) => Some(name.to_owned()),
        Err(_) => None,
    }
}

/// Return the login name of the user with the UID given. For example,
/// `"John Smith"`.
///
/// If the UID does not exist, if the full name is not set, or if the
/// full is invalid UTF-8, returns None.
pub fn user_full_name(uid: uid_t) -> Option<String> {
    let passwd_info = unsafe { libc::getpwuid(uid) };
    if passwd_info.is_null() {
        return None;
    }
    let name_cstr = unsafe { CStr::from_ptr((*passwd_info).pw_gecos) };
    match name_cstr.to_str() {
        Ok(name) => {
            if name.len() > 0 {
                Some(name.to_owned())
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_root_uid() {
        println!("your uid is {}", current_user_id());
        println!("your username is {}", current_username());
        println!("your full name is {}",
                 user_full_name(current_user_id()).unwrap());
        assert_eq!(username(0).unwrap(), "root");
    }
}
