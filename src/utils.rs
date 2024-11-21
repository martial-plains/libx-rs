use core::ffi::CStr;

use alloc::string::{String, ToString};

pub fn get_env_var(var_name: &str) -> Option<String> {
    let var_name = var_name.as_bytes();
    let mut i = 0;
    while i < var_name.len() && var_name[i] != b'=' {
        i += 1;
    }

    let (name, value) = var_name.split_at(i);

    let value = match unsafe { libc::getenv(name.as_ptr().cast::<libc::c_char>()) } {
        _ if value.starts_with(b"=") => {
            let value = &value[1..];
            if value.is_empty() {
                None
            } else {
                Some(String::from_utf8_lossy(value).to_string())
            }
        }
        ptr => {
            if ptr.is_null() {
                None
            } else {
                Some(String::from_utf8_lossy(unsafe { CStr::from_ptr(ptr) }.to_bytes()).to_string())
            }
        }
    };

    value
}
