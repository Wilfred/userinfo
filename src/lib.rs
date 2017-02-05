extern crate libc;

use std::path::PathBuf;
use std::ffi::{CStr, OsStr};
use libc::{uid_t, gid_t};
use std::os::unix::ffi::OsStrExt;

/// Return the user ID (often called UID on Unix) associated with the
/// current user.
pub fn current_user_id() -> uid_t {
    unsafe { libc::getuid() }
}

/// Return the group ID (often called GID on Unix) associated with the
/// current user.
pub fn current_group_id() -> gid_t {
    unsafe { libc::getgid() }
}

/// Return the login name of the user with the UID given. For example,
/// `"jsmith"`.
///
/// If the UID does not exist, or the login name is invalid UTF-8,
/// returns None.
pub fn login_name(uid: uid_t) -> Option<String> {
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

/// Return the full name of the user with the UID given. For example,
/// `"John Smith"`.
///
/// If the UID does not exist, if the full name is not set, or if the
/// full name is invalid UTF-8, returns None.
pub fn user_full_name(uid: uid_t) -> Option<String> {
    let passwd_info = unsafe { libc::getpwuid(uid) };
    if passwd_info.is_null() {
        return None;
    }
    let name_cstr = unsafe { CStr::from_ptr((*passwd_info).pw_gecos) };
    match name_cstr.to_str() {
        Ok(name) => {
            if name.is_empty() {
                Some(name.to_owned())
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

/// Return the path to the home directory for the user with this UID.
///
/// Returns None if the UID doesn't exist.
pub fn user_home_directory(uid: uid_t) -> Option<PathBuf> {
    let passwd_info = unsafe { libc::getpwuid(uid) };
    if passwd_info.is_null() {
        return None;
    }
    let dir_cstr = unsafe { CStr::from_ptr((*passwd_info).pw_dir) };
    let dir_ostr = OsStr::from_bytes(dir_cstr.to_bytes());
    Some(PathBuf::from(dir_ostr))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(unix)]
    #[test]
    fn test_root_uid() {
        assert_eq!(login_name(0).unwrap(), "root");
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_root_home_dir() {
        assert_eq!(user_home_directory(0).unwrap(), PathBuf::from("/root"));
    }

    #[cfg(target_os = "mac_os")]
    #[test]
    fn test_root_home_dir() {
        assert_eq!(user_home_directory(0).unwrap(), PathBuf::from("/var/root"));
    }
}
