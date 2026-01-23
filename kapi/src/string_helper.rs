//! String helper functions for kernel modules
//!
//! References: <https://elixir.bootlin.com/linux/v6.6/source/lib/string_helpers.c>

use core::ffi::c_char;

use kmod::capi_fn;

/// Removes leading whitespace from @s.
///
/// Returns a pointer to the first non-whitespace character in @s.
#[capi_fn]
pub unsafe extern "C" fn skip_spaces(s: *const c_char) -> *mut c_char {
    let mut ptr = s;
    while (*ptr as u8).is_ascii_whitespace() {
        ptr = ptr.add(1);
    }
    ptr as *mut c_char
}

/// Removes leading and trailing whitespace from @s.
///
/// Note that the first trailing whitespace is replaced with a %NUL-terminator
/// in the given string @s. Returns a pointer to the first non-whitespace
/// character in @s.
#[capi_fn]
pub unsafe extern "C" fn strstrip(s: *mut c_char) -> *mut c_char {
    let size = crate::string::strlen(s);
    if size == 0 {
        return s;
    }
    let mut end = s.add(size - 1);
    while end >= s && (*end as u8).is_ascii_whitespace() {
        end = end.sub(1);
    }
    *end.add(1) = 0;
    skip_spaces(s)
}

#[capi_fn]
pub unsafe extern "C" fn strim(s: *mut c_char) -> *mut c_char {
    strstrip(s)
}

#[cfg(test)]
mod tests {
    use core::ffi::CStr;

    use alloc::ffi::CString;

    use super::*;

    #[test]
    fn test_skip_spaces() {
        let s = c"   Hello, World!";
        let result = unsafe { skip_spaces(s.as_ptr()) };
        let result_str = unsafe { core::ffi::CStr::from_ptr(result) };
        assert_eq!(result_str.to_str().unwrap(), "Hello, World!");
    }

    #[test]
    fn test_strstrip() {
        let c_string = CString::new("   Hello, World!   ").unwrap();
        let result = unsafe { strstrip(c_string.into_raw()) };
        let result_str = unsafe { CStr::from_ptr(result) };
        assert_eq!(result_str.to_str().unwrap(), "Hello, World!");
    }
}
