extern crate winapi;
extern crate advapi32;

use winapi::lmcons::UNLEN;
use winapi::winnt::WCHAR;
use advapi32::GetUserNameW;

    
pub fn user_id() -> u64 {
    12
}

pub fn user_name() -> String {
    let mut buf: [WCHAR; (UNLEN + 1) as usize] = [0; (UNLEN + 1) as usize];
    let mut buf_size = UNLEN + 1;
    unsafe {
        GetUserNameW(buf.as_mut_ptr(), &mut buf_size);
    }
    String::from_utf16(&buf).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_id() {
        println!("your name is {}", user_name());
        assert_eq!(user_id(), 12);
    }
}
