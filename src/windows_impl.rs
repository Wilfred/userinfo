extern crate winapi;
extern crate advapi32;

use std::path::PathBuf;
use winapi::lmcons::UNLEN;
use winapi::winnt::WCHAR;
use advapi32::GetUserNameW;

type uid_t = u64;
type gid_t = u64;
    
pub fn current_user_id() -> uid_t {
    12
}

pub fn current_group_id() -> gid_t {
    12
}

pub fn login_name(_uid: uid_t) -> Option<String> {
    let mut buf: [WCHAR; (UNLEN + 1) as usize] = [0; (UNLEN + 1) as usize];
    let mut buf_size = UNLEN + 1;
    unsafe {
        GetUserNameW(buf.as_mut_ptr(), &mut buf_size);
    }
    match String::from_utf16(&buf) {
        Ok(name) => Some(name),
        Err(_) => None,
    }
}

pub fn user_full_name(_uid: uid_t) -> Option<String> {
    Some("foobar".to_owned())
}

pub fn user_home_directory(_uid: uid_t) -> Option<PathBuf> {
    Some(PathBuf::from("c:\\"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_id() {
        println!("your name is {:?}", login_name(0));
        assert_eq!(current_user_id(), 12);
    }
}
