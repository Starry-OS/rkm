//! The `str` module provides utilities for string handling in kernel modules.
//!
//! References:
//! - <https://elixir.bootlin.com/linux/v6.6/source/lib/string.c>
//!

use core::ffi::{c_char, c_int, c_void};

use kmod::capi_fn;

/// Case insensitive, length-limited string comparison
///
/// # Arguments
/// * `s1` - One string
/// * `s2` - The other string
/// * `n` - the maximum number of characters to compare
#[capi_fn]
pub unsafe extern "C" fn strncasecmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int {
    let mut count = n;
    let mut p1 = s1;
    let mut p2 = s2;

    while count > 0 {
        let c1 = (*p1) as u8;
        let c2 = (*p2) as u8;

        if c1 == 0 || c2 == 0 {
            return (c1 as c_int) - (c2 as c_int);
        }

        let lower_c1 = c1.to_ascii_lowercase();
        let lower_c2 = c2.to_ascii_lowercase();

        if lower_c1 != lower_c2 {
            return (lower_c1 as c_int) - (lower_c2 as c_int);
        }

        p1 = p1.add(1);
        p2 = p2.add(1);
        count -= 1;
    }

    0
}

/// Case insensitive string comparison
///
/// # Arguments
/// * `s1` - One string
/// * `s2` - The other string
#[capi_fn]
pub unsafe extern "C" fn strcasecmp(mut s1: *const c_char, mut s2: *const c_char) -> c_int {
    loop {
        let c1 = (*s1) as u8;
        let c2 = (*s2) as u8;

        let lower_c1 = c1.to_ascii_lowercase();
        let lower_c2 = c2.to_ascii_lowercase();

        if lower_c1 != lower_c2 {
            return (lower_c1 as c_int) - (lower_c2 as c_int);
        }

        if lower_c1 == 0 {
            break;
        }

        s1 = s1.add(1);
        s2 = s2.add(1);
    }

    0
}

/// Copy a string
///
/// # Arguments
/// * `dest` - Destination string buffer
/// * `src` - Source string to copy from
#[capi_fn]
pub unsafe extern "C" fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char {
    let mut tmp = dest;
    let mut s = src;

    loop {
        let c = *s;
        *tmp = c;
        if c == 0 {
            break;
        }
        tmp = tmp.add(1);
        s = s.add(1);
    }

    dest
}

/// Copy a string, length-limited
///
/// # Arguments
/// * `dest` - Destination string buffer
/// * `src` - Source string to copy from
/// * `count` - Maximum number of characters to copy
#[capi_fn]
pub unsafe extern "C" fn strncpy(dest: *mut c_char, src: *const c_char, n: usize) -> *mut c_char {
    let mut tmp = dest;
    let mut s = src;
    let mut count = n;

    while count > 0 {
        let c = *s;
        *tmp = c;
        if c == 0 {
            break;
        }
        tmp = tmp.add(1);
        s = s.add(1);
        count -= 1;
    }

    dest
}

/// Safe string copy with size limit
///
/// # Arguments
/// * `dest` - Destination string buffer
/// * `src` - Source string to copy from
/// * `size` - Maximum size of destination buffer
#[capi_fn]
pub unsafe extern "C" fn strlcpy(dest: *mut c_char, src: *const c_char, size: usize) -> usize {
    let len = strlen(src);

    if size > 0 {
        let copy_len = if len >= size { size - 1 } else { len };
        core::ptr::copy_nonoverlapping(src as *const u8, dest as *mut u8, copy_len);
        *dest.add(copy_len) = 0;
    }

    len
}

/// Concatenate two strings
///
/// # Arguments
/// * `dest` - Destination string
/// * `src` - Source string to append
#[capi_fn]
pub unsafe extern "C" fn strcat(dest: *mut c_char, src: *const c_char) -> *mut c_char {
    let mut tmp = dest.add(strlen(dest));
    let mut s = src;

    // Copy src to end of dest
    while (*s) != 0 {
        *tmp = *s;
        tmp = tmp.add(1);
        s = s.add(1);
    }
    *tmp = 0;

    dest
}

/// Concatenate two strings with length limit
///
/// # Arguments
/// * `dest` - Destination string
/// * `src` - Source string to append
/// * `count` - Maximum number of characters to append
#[capi_fn]
pub unsafe extern "C" fn strncat(dest: *mut c_char, src: *const c_char, n: usize) -> *mut c_char {
    let mut tmp = dest.add(strlen(dest));
    let mut s = src;
    let mut count = n;

    // Copy src to end of dest
    if count > 0 {
        while (*s) != 0 && count > 0 {
            *tmp = *s;
            tmp = tmp.add(1);
            s = s.add(1);
            count -= 1;
        }
    }
    *tmp = 0;

    dest
}

/// Safe string concatenation with size limit
///
/// # Arguments
/// * `dest` - Destination string buffer
/// * `src` - Source string to append
/// * `count` - Maximum total size of destination buffer
#[capi_fn]
pub unsafe extern "C" fn strlcat(dest: *mut c_char, src: *const c_char, size: usize) -> usize {
    let dsize = strlen(dest);
    let len = strlen(src);

    let res = dsize + len;

    // This would be a bug
    if dsize >= size {
        // panic equivalent - should not happen in well-formed code
        return res;
    }

    let dest_end = dest.add(dsize);
    let count = size - dsize;

    let mut copy_len = len;
    if copy_len >= count {
        copy_len = count - 1;
    }

    core::ptr::copy_nonoverlapping(src as *const u8, dest_end as *mut u8, copy_len);
    *dest_end.add(copy_len) = 0;

    res
}

/// Compare two strings
///
/// # Arguments
/// * `cs` - One string
/// * `ct` - Another string
#[capi_fn]
pub unsafe extern "C" fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int {
    let mut p1 = s1;
    let mut p2 = s2;

    loop {
        let c1 = *p1 as u8;
        let c2 = *p2 as u8;

        if c1 != c2 {
            return if c1 < c2 { -1 } else { 1 };
        }

        if c1 == 0 {
            break;
        }

        p1 = p1.add(1);
        p2 = p2.add(1);
    }

    0
}

/// Compare two length-limited strings
///
/// # Arguments
/// * `cs` - One string
/// * `ct` - Another string
/// * `count` - The maximum number of bytes to compare
#[capi_fn]
pub unsafe extern "C" fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int {
    let mut count = n;
    let mut p1 = s1;
    let mut p2 = s2;

    while count > 0 {
        let c1 = *p1 as u8;
        let c2 = *p2 as u8;

        if c1 != c2 {
            return if c1 < c2 { -1 } else { 1 };
        }

        if c1 == 0 {
            break;
        }

        p1 = p1.add(1);
        p2 = p2.add(1);
        count -= 1;
    }

    0
}

/// Find the first occurrence of a character in a string
///
/// # Arguments
/// * `s` - The string to be searched
/// * `c` - The character to search for
///
/// Note that the NUL-terminator is considered part of the string, and can be searched for.
#[capi_fn]
pub unsafe extern "C" fn strchr(s: *const c_char, c: c_int) -> *mut c_char {
    let search_char = c as u8 as c_char;
    let mut p = s;

    loop {
        if *p == search_char {
            return p as *mut c_char;
        }
        if *p == 0 {
            return core::ptr::null_mut();
        }
        p = p.add(1);
    }
}

/// Find and return a character in a string, or end of string
///
/// # Arguments
/// * `s` - The string to be searched
/// * `c` - The character to search for
///
/// Returns pointer to first occurrence of 'c' in s. If c is not found, then
/// return a pointer to the null byte at the end of s.
#[capi_fn]
pub unsafe extern "C" fn strchrnul(s: *const c_char, c: c_int) -> *mut c_char {
    let search_char = c as u8 as c_char;
    let mut p = s;

    loop {
        if *p == search_char || *p == 0 {
            return p as *mut c_char;
        }
        p = p.add(1);
    }
}

/// Find and return a character in a length limited string, or end of string
///
/// # Arguments
/// * `s` - The string to be searched
/// * `c` - The character to search for
/// * `n` - The number of characters to be searched
///
/// Returns pointer to the first occurrence of 'c' in s. If c is not found,
/// then return a pointer to the last character of the string.
#[capi_fn]
pub unsafe extern "C" fn strnchrnul(s: *const c_char, c: c_int, n: usize) -> *mut c_char {
    let search_char = c as u8 as c_char;
    let mut p = s;
    let mut count = n;

    while count > 0 && *p != 0 && *p != search_char {
        p = p.add(1);
        count -= 1;
    }

    p as *mut c_char
}

/// Find the last occurrence of a character in a string
///
/// # Arguments
/// * `s` - The string to be searched
/// * `c` - The character to search for
#[capi_fn]
pub unsafe extern "C" fn strrchr(s: *const c_char, c: c_int) -> *mut c_char {
    let search_char = c as u8 as c_char;
    let mut last: *const c_char = core::ptr::null();
    let mut p = s;

    loop {
        if *p == search_char {
            last = p;
        }
        if *p == 0 {
            break;
        }
        p = p.add(1);
    }

    if last.is_null() {
        core::ptr::null_mut()
    } else {
        last as *mut c_char
    }
}

/// Find a character in a length limited string
///
/// # Arguments
/// * `s` - The string to be searched
/// * `count` - The number of characters to be searched
/// * `c` - The character to search for
///
/// Note that the NUL-terminator is considered part of the string, and can be searched for.
#[capi_fn]
pub unsafe extern "C" fn strnchr(s: *const c_char, c: c_int, n: usize) -> *mut c_char {
    let search_char = c as u8 as c_char;
    let mut p = s;
    let mut count = n;

    while count > 0 {
        if *p == search_char {
            return p as *mut c_char;
        }
        if *p == 0 {
            break;
        }
        p = p.add(1);
        count -= 1;
    }

    core::ptr::null_mut()
}

/// Find the length of a string
///
/// # Arguments
/// * `s` - The string to measure
#[capi_fn]
pub unsafe extern "C" fn strlen(s: *const c_char) -> usize {
    let mut sc = s;
    let mut count = 0;

    while *sc != 0 {
        sc = sc.add(1);
        count += 1;
    }

    count
}

/// Find the length of a length-limited string
///
/// # Arguments
/// * `s` - The string to measure
/// * `count` - The maximum number of characters to search
#[capi_fn]
pub unsafe extern "C" fn strnlen(s: *const c_char, n: usize) -> usize {
    let mut sc = s;
    let mut count = 0;
    let mut limit = n;

    while limit > 0 && *sc != 0 {
        sc = sc.add(1);
        count += 1;
        limit -= 1;
    }

    count
}

/// Calculate the length of the initial substring of @s which only contain letters in @accept
///
/// # Arguments
/// * `s` - The string to be searched
/// * `accept` - The string to search for
#[capi_fn]
pub unsafe extern "C" fn strspn(s: *const c_char, accept: *const c_char) -> usize {
    let mut p = s;
    let mut count = 0;

    while *p != 0 {
        // Check if *p is in accept string
        let mut q = accept;
        let mut found = false;
        while *q != 0 {
            if *p == *q {
                found = true;
                break;
            }
            q = q.add(1);
        }
        if !found {
            break;
        }
        p = p.add(1);
        count += 1;
    }

    count
}

/// Calculate the length of the initial substring of @s which does not contain letters in @reject
///
/// # Arguments
/// * `s` - The string to be searched
/// * `reject` - The string to avoid
#[capi_fn]
pub unsafe extern "C" fn strcspn(s: *const c_char, reject: *const c_char) -> usize {
    let mut p = s;
    let mut count = 0;

    while *p != 0 {
        // Check if *p is in reject string
        let mut q = reject;
        let mut found = false;
        while *q != 0 {
            if *p == *q {
                found = true;
                break;
            }
            q = q.add(1);
        }
        if found {
            break;
        }
        p = p.add(1);
        count += 1;
    }

    count
}

/// Find the first occurrence of a set of characters
///
/// # Arguments
/// * `cs` - The string to be searched
/// * `ct` - The characters to search for
#[capi_fn]
pub unsafe extern "C" fn strpbrk(s: *const c_char, accept: *const c_char) -> *mut c_char {
    let mut p = s;

    while *p != 0 {
        // Check if *p is in accept string
        let mut q = accept;
        while *q != 0 {
            if *p == *q {
                return p as *mut c_char;
            }
            q = q.add(1);
        }
        p = p.add(1);
    }

    core::ptr::null_mut()
}

/// Split a string into tokens
///
/// # Arguments
/// * `s` - Pointer to the string being searched. Updated to point after the token.
/// * `delim` - The characters to search for
///
/// strsep() updates @s to point after the token, ready for the next call.
///
/// It returns empty tokens, too, behaving exactly like the libc function
/// of that name. In fact, it was stolen from glibc2 and de-fancy-fied.
/// Same semantics, slimmer shape.
#[capi_fn]
pub unsafe extern "C" fn strsep(s: *mut *mut c_char, delim: *const c_char) -> *mut c_char {
    let sbegin = *s;

    if sbegin.is_null() {
        return core::ptr::null_mut();
    }

    // Find the delimiter
    let mut p = sbegin;
    let mut end: *mut c_char = core::ptr::null_mut();

    while *p != 0 {
        let mut q = delim;
        while *q != 0 {
            if *p == *q {
                end = p;
                break;
            }
            q = q.add(1);
        }
        if !end.is_null() {
            break;
        }
        p = p.add(1);
    }

    if !end.is_null() {
        *end = 0;
        *s = end.add(1);
    } else {
        *s = core::ptr::null_mut();
    }

    sbegin
}

/// Fill a region of memory with the given value
///
/// # Arguments
/// * `s` - Pointer to the start of the area.
/// * `c` - The byte to fill the area with
/// * `count` - The size of the area.
///
/// Do not use memset() to access IO space, use memset_io() instead.
#[capi_fn]
pub unsafe extern "C" fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_char {
    let xs = s as *mut u8;
    let byte = c as u8;

    for i in 0..n {
        *xs.add(i) = byte;
    }

    s as *mut c_char
}

/// Fill a memory area with a uint16_t
///
/// # Arguments
/// * `s` - Pointer to the start of the area.
/// * `v` - The value to fill the area with
/// * `count` - The number of values to store
///
/// Differs from memset() in that it fills with a uint16_t instead
/// of a byte.  Remember that @count is the number of uint16_ts to
/// store, not the number of bytes.
#[capi_fn]
pub unsafe extern "C" fn memset16(s: *mut u16, c: u16, n: usize) -> *mut c_char {
    let xs = s;

    for i in 0..n {
        *xs.add(i) = c;
    }

    s as *mut c_char
}

/// Fill a memory area with a uint32_t
///
/// # Arguments
/// * `s` - Pointer to the start of the area.
/// * `v` - The value to fill the area with
/// * `count` - The number of values to store
///
/// Differs from memset() in that it fills with a uint32_t instead
/// of a byte.  Remember that @count is the number of uint32_ts to
/// store, not the number of bytes.
#[capi_fn]
pub unsafe extern "C" fn memset32(s: *mut u32, c: u32, n: usize) -> *mut c_char {
    let xs = s;

    for i in 0..n {
        *xs.add(i) = c;
    }

    s as *mut c_char
}

/// Fill a memory area with a uint64_t
///
/// # Arguments
/// * `s` - Pointer to the start of the area.
/// * `v` - The value to fill the area with
/// * `count` - The number of values to store
///
/// Differs from memset() in that it fills with a uint64_t instead
/// of a byte.  Remember that @count is the number of uint64_ts to
/// store, not the number of bytes.
#[capi_fn]
pub unsafe extern "C" fn memset64(s: *mut u64, c: u64, n: usize) -> *mut c_char {
    let xs = s;

    for i in 0..n {
        *xs.add(i) = c;
    }

    s as *mut c_char
}

/// Copy one area of memory to another
///
/// # Arguments
/// * `dest` - Where to copy to
/// * `src` - Where to copy from
/// * `count` - The size of the area.
///
/// You should not use this function to access IO space, use memcpy_toio()
/// or memcpy_fromio() instead.
#[capi_fn]
pub unsafe extern "C" fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void {
    let tmp = dest as *mut u8;
    let s = src as *const u8;
    for i in 0..n {
        *tmp.add(i) = *s.add(i);
    }
    dest
}

/// Copy one area of memory to another
///
/// # Arguments
/// * `dest` - Where to copy to
/// * `src` - Where to copy from
/// * `count` - The size of the area.
///
/// Unlike memcpy(), memmove() copes with overlapping areas.
#[capi_fn]
pub unsafe extern "C" fn memmove(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void {
    let dest_addr = dest as usize;
    let src_addr = src as usize;

    if dest_addr <= src_addr {
        // Non-overlapping or src is after dest: safe to copy forwards
        let tmp = dest as *mut u8;
        let s = src as *const u8;
        for i in 0..n {
            *tmp.add(i) = *s.add(i);
        }
    } else {
        // Overlapping and dest is after src: copy backwards
        let mut tmp = dest as *mut u8;
        tmp = tmp.add(n);
        let mut s = src as *const u8;
        s = s.add(n);
        for _ in 0..n {
            tmp = tmp.sub(1);
            s = s.sub(1);
            *tmp = *s;
        }
    }
    dest
}

/// Compare two areas of memory
///
/// # Arguments
/// * `cs` - One area of memory
/// * `ct` - Another area of memory
/// * `count` - The size of the area.
#[capi_fn]
pub unsafe extern "C" fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int {
    let su1 = s1 as *const u8;
    let su2 = s2 as *const u8;

    for i in 0..n {
        let c1 = *su1.add(i);
        let c2 = *su2.add(i);
        if c1 != c2 {
            return if c1 < c2 { -1 } else { 1 };
        }
    }

    0
}

/// Returns 0 if and only if the buffers have identical contents.
///
/// # Arguments
/// * `a` - pointer to first buffer.
/// * `b` - pointer to second buffer.
/// * `len` - size of buffers.
///
/// The sign or magnitude of a non-zero return value has no particular
/// meaning, and architectures may implement their own more efficient bcmp(). So
/// while this particular implementation is a simple (tail) call to memcmp, do
/// not rely on anything but whether the return value is zero or non-zero.
#[capi_fn]
pub unsafe extern "C" fn bcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int {
    memcmp(s1, s2, n)
}

/// Find a character in an area of memory.
///
/// # Arguments
/// * `addr` - The memory area
/// * `c` - The byte to search for
/// * `size` - The size of the area.
///
/// returns the address of the first occurrence of @c, or 1 byte past
/// the area if @c is not found
#[capi_fn]
pub unsafe extern "C" fn memscan(s: *mut c_void, c: c_int, n: usize) -> *mut c_void {
    let p = s as *mut u8;
    let byte = c as u8;

    for i in 0..n {
        if *p.add(i) == byte {
            return p.add(i) as *mut c_void;
        }
    }

    p.add(n) as *mut c_void
}

/// Find the first substring in a NUL terminated string
///
/// # Arguments
/// * `s1` - The string to be searched
/// * `s2` - The string to search for
#[capi_fn]
pub unsafe extern "C" fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char {
    let l2 = strlen(needle);

    if l2 == 0 {
        return haystack as *mut c_char;
    }

    let mut l1 = strlen(haystack);
    let mut h = haystack;

    // Search
    while l1 >= l2 {
        l1 -= 1;
        if memcmp(h as *const c_void, needle as *const c_void, l2) == 0 {
            return h as *mut c_char;
        }
        h = h.add(1);
    }

    core::ptr::null_mut()
}

/// Find the first substring in a length-limited string
///
/// # Arguments
/// * `s1` - The string to be searched
/// * `s2` - The string to search for
/// * `len` - the maximum number of characters to search
#[capi_fn]
pub unsafe extern "C" fn strnstr(
    haystack: *const c_char,
    needle: *const c_char,
    len: usize,
) -> *mut c_char {
    let l2 = strlen(needle);

    if l2 == 0 {
        return haystack as *mut c_char;
    }

    let mut remaining = len;
    let mut h = haystack;

    while remaining >= l2 {
        remaining -= 1;
        if memcmp(h as *const c_void, needle as *const c_void, l2) == 0 {
            return h as *mut c_char;
        }
        h = h.add(1);
    }

    core::ptr::null_mut()
}

/// Find a character in an area of memory.
///
/// # Arguments
/// * `s` - The memory area
/// * `c` - The byte to search for
/// * `n` - The size of the area.
///
/// returns the address of the first occurrence of @c, or NULL
/// if @c is not found
#[capi_fn]
pub unsafe extern "C" fn memchr(s: *const c_void, c: c_int, n: usize) -> *mut c_void {
    let p = s as *const u8;
    let byte = c as u8;

    for i in 0..n {
        if *p.add(i) == byte {
            return p.add(i) as *mut c_void;
        }
    }

    core::ptr::null_mut()
}

#[capi_fn]
unsafe extern "C" fn sized_strscpy(dest: *mut c_char, src: *const c_char, count: usize) -> isize {
    let src_str = unsafe { core::ffi::CStr::from_ptr(src) };
    let bytes = src_str.to_bytes_with_nul();
    let len = core::cmp::min(bytes.len(), count);
    unsafe {
        core::ptr::copy_nonoverlapping(bytes.as_ptr(), dest as *mut u8, len);
    }
    (len - 1) as isize // exclude null terminator
}

#[cfg(test)]
mod tests {
    use core::ffi::{c_char, c_int, c_void};

    #[test]
    fn test_strncasecmp() {
        use super::strncasecmp;
        let a = b"abc\0";
        let b = b"AbCDEF\0";
        let result =
            unsafe { strncasecmp(a.as_ptr() as *const c_char, b.as_ptr() as *const c_char, 3) };
        assert_eq!(result, 0);
        let result =
            unsafe { strncasecmp(a.as_ptr() as *const c_char, b.as_ptr() as *const c_char, 4) };
        assert!(result < 0);
    }

    #[test]
    fn test_strcasecmp() {
        use super::strcasecmp;
        let a = b"abc\0";
        let b = b"AbCDEF\0";
        let result =
            unsafe { strcasecmp(a.as_ptr() as *const c_char, b.as_ptr() as *const c_char) };
        assert!(result < 0);
        let c = b"abcdef\0";
        let result =
            unsafe { strcasecmp(c.as_ptr() as *const c_char, b.as_ptr() as *const c_char) };
        assert_eq!(result, 0);
    }

    #[test]
    fn test_strcpy() {
        use super::strcpy;
        let src = b"hello\0";
        let mut dest = [0u8; 10];
        let result = unsafe {
            strcpy(
                dest.as_mut_ptr() as *mut c_char,
                src.as_ptr() as *const c_char,
            )
        };
        assert_eq!(
            unsafe { core::slice::from_raw_parts(result as *const u8, 5) },
            b"hello"
        );
    }

    #[test]
    fn test_strncpy() {
        use super::strncpy;
        let src = b"hello world\0";
        let mut dest = [0u8; 10];
        unsafe {
            strncpy(
                dest.as_mut_ptr() as *mut c_char,
                src.as_ptr() as *const c_char,
                5,
            )
        };
        assert_eq!(&dest[0..5], b"hello");
    }

    #[test]
    fn test_strlen() {
        use super::strlen;
        let s = b"hello\0";
        let len = unsafe { strlen(s.as_ptr() as *const c_char) };
        assert_eq!(len, 5);
    }

    #[test]
    fn test_strnlen() {
        use super::strnlen;
        let s = b"hello\0";
        let len = unsafe { strnlen(s.as_ptr() as *const c_char, 10) };
        assert_eq!(len, 5);
        let len = unsafe { strnlen(s.as_ptr() as *const c_char, 3) };
        assert_eq!(len, 3);
    }

    #[test]
    fn test_strcat() {
        use super::strcat;
        let mut dest = *b"hello\0\0\0\0\0\0";
        let src = b" world\0";
        unsafe {
            strcat(
                dest.as_mut_ptr() as *mut c_char,
                src.as_ptr() as *const c_char,
            )
        };
        assert_eq!(&dest[0..11], *b"hello world");
    }

    #[test]
    fn test_strncat() {
        use super::strncat;
        let mut dest = *b"hello\0\0\0\0\0\0\0\0\0";
        let src = b" world\0";
        unsafe {
            strncat(
                dest.as_mut_ptr() as *mut c_char,
                src.as_ptr() as *const c_char,
                3,
            )
        };
        assert_eq!(&dest[0..8], *b"hello wo");
    }

    #[test]
    fn test_strchr() {
        use super::strchr;
        let s = b"hello world\0";
        let result = unsafe { strchr(s.as_ptr() as *const c_char, 'o' as c_int) };
        assert!(!result.is_null());
        assert_eq!(unsafe { *result }, 'o' as c_char);
    }

    #[test]
    fn test_strrchr() {
        use super::strrchr;
        let s = b"hello world\0";
        let result = unsafe { strrchr(s.as_ptr() as *const c_char, 'o' as c_int) };
        assert!(!result.is_null());
        assert_eq!(unsafe { *result }, 'o' as c_char);
    }

    #[test]
    fn test_strstr() {
        use super::strstr;
        let haystack = b"hello world\0";
        let needle = b"wor\0";
        let result = unsafe {
            strstr(
                haystack.as_ptr() as *const c_char,
                needle.as_ptr() as *const c_char,
            )
        };
        assert!(!result.is_null());
        assert_eq!(unsafe { *result }, 'w' as c_char);
    }

    #[test]
    fn test_strcmp() {
        use super::strcmp;
        let a = b"abc\0";
        let b = b"abc\0";
        let result = unsafe { strcmp(a.as_ptr() as *const c_char, b.as_ptr() as *const c_char) };
        assert_eq!(result, 0);

        let c = b"abd\0";
        let result = unsafe { strcmp(a.as_ptr() as *const c_char, c.as_ptr() as *const c_char) };
        assert!(result < 0);
    }

    #[test]
    fn test_strncmp() {
        use super::strncmp;
        let a = b"abc\0";
        let b = b"abd\0";
        let result =
            unsafe { strncmp(a.as_ptr() as *const c_char, b.as_ptr() as *const c_char, 2) };
        assert_eq!(result, 0);
        let result =
            unsafe { strncmp(a.as_ptr() as *const c_char, b.as_ptr() as *const c_char, 3) };
        assert!(result < 0);
    }

    #[test]
    fn test_memset() {
        use super::memset;
        let mut buf = [0u8; 10];
        unsafe { memset(buf.as_mut_ptr() as *mut c_void, 0x41, 10) };
        assert_eq!(&buf, &[0x41u8; 10]);
    }

    #[test]
    fn test_memcpy() {
        use super::memcpy;
        let src = b"hello\0\0\0\0\0";
        let mut dest = [0u8; 10];
        unsafe {
            memcpy(
                dest.as_mut_ptr() as *mut c_void,
                src.as_ptr() as *const c_void,
                5,
            )
        };
        assert_eq!(&dest[0..5], b"hello");
    }

    #[test]
    fn test_memcmp() {
        use super::memcmp;
        let a = b"hello";
        let b = b"hello";
        let result = unsafe { memcmp(a.as_ptr() as *const c_void, b.as_ptr() as *const c_void, 5) };
        assert_eq!(result, 0);

        let c = b"hellx";
        let result = unsafe { memcmp(a.as_ptr() as *const c_void, c.as_ptr() as *const c_void, 5) };
        assert!(result < 0);
    }

    #[test]
    fn test_strlcpy() {
        use super::strlcpy;
        let src = b"hello\0";
        let mut dest = [0u8; 10];
        let len = unsafe {
            strlcpy(
                dest.as_mut_ptr() as *mut c_char,
                src.as_ptr() as *const c_char,
                10,
            )
        };
        assert_eq!(len, 5);
        assert_eq!(&dest[0..5], b"hello");
    }

    #[test]
    fn test_strlcat() {
        use super::{strlcat, strlcpy};
        let mut dest = [0u8; 20];
        unsafe {
            let src1 = b"hello\0";
            strlcpy(
                dest.as_mut_ptr() as *mut c_char,
                src1.as_ptr() as *const c_char,
                20,
            );
            let src2 = b" world\0";
            strlcat(
                dest.as_mut_ptr() as *mut c_char,
                src2.as_ptr() as *const c_char,
                20,
            );
        };
        assert_eq!(&dest[0..11], *b"hello world");
    }

    #[test]
    fn test_strspn() {
        use super::strspn;
        let s = b"aaabbbccc\0";
        let accept = b"ab\0";
        let len = unsafe {
            strspn(
                s.as_ptr() as *const c_char,
                accept.as_ptr() as *const c_char,
            )
        };
        assert_eq!(len, 6);
    }

    #[test]
    fn test_strcspn() {
        use super::strcspn;
        let s = b"aaabbbccc\0";
        let reject = b"c\0";
        let len = unsafe {
            strcspn(
                s.as_ptr() as *const c_char,
                reject.as_ptr() as *const c_char,
            )
        };
        assert_eq!(len, 6);
    }

    #[test]
    fn test_strpbrk() {
        use super::strpbrk;
        let s = b"hello world\0";
        let accept = b"wor\0";
        let result = unsafe {
            strpbrk(
                s.as_ptr() as *const c_char,
                accept.as_ptr() as *const c_char,
            )
        };
        assert!(!result.is_null());
        assert_eq!(unsafe { *result }, 'o' as c_char);
    }

    #[test]
    fn test_memscan() {
        use super::memscan;
        let s = b"hello\0";
        let result = unsafe { memscan(s.as_ptr() as *mut c_void, 'l' as c_int, 5) };
        assert!(!result.is_null());
        assert_eq!(unsafe { *(result as *const u8) }, b'l');
    }

    #[test]
    fn test_memchr() {
        use super::memchr;
        let s = b"hello\0";
        let result = unsafe { memchr(s.as_ptr() as *const c_void, 'o' as c_int, 5) };
        assert!(!result.is_null());
        assert_eq!(unsafe { *(result as *const u8) }, b'o');
    }

    #[test]
    fn test_strchrnul() {
        use super::strchrnul;
        let s = b"hello\0";
        let result = unsafe { strchrnul(s.as_ptr() as *const c_char, 'x' as c_int) };
        assert!(!result.is_null());
        assert_eq!(unsafe { *result }, 0);
    }

    #[test]
    fn test_strnchrnul() {
        use super::strnchrnul;
        let s = b"hello world\0";
        let result = unsafe { strnchrnul(s.as_ptr() as *const c_char, 'w' as c_int, 5) };
        assert!(!result.is_null());
    }

    #[test]
    fn test_strnchr() {
        use super::strnchr;
        let s = b"hello world\0";
        let result = unsafe { strnchr(s.as_ptr() as *const c_char, 'o' as c_int, 5) };
        assert!(!result.is_null());
        assert_eq!(unsafe { *result }, 'o' as c_char);
    }

    #[test]
    fn test_memmove() {
        use super::memmove;
        let mut buf = *b"hello world";
        unsafe {
            memmove(
                (buf.as_mut_ptr() as usize + 3) as *mut c_void,
                buf.as_ptr() as *const c_void,
                5,
            )
        };
        assert_eq!(&buf[3..8], *b"hello");
    }

    #[test]
    fn test_strnstr() {
        use super::strnstr;
        let haystack = b"hello world\0";
        let needle = b"wor\0";
        let result = unsafe {
            strnstr(
                haystack.as_ptr() as *const c_char,
                needle.as_ptr() as *const c_char,
                11,
            )
        };
        assert!(!result.is_null());
        assert_eq!(unsafe { *result }, 'w' as c_char);
    }

    #[test]
    fn test_strsep() {
        use super::strsep;
        let test_data = b"a,b,c\0";
        let mut buf = test_data.to_vec();
        let mut str_ptr = buf.as_mut_ptr() as *mut c_char;
        let delim = b",\0";
        let token = unsafe { strsep(&mut str_ptr, delim.as_ptr() as *const c_char) };
        assert!(!token.is_null());
        assert_eq!(unsafe { *token }, 'a' as c_char);
    }

    #[test]
    fn test_bcmp() {
        use super::bcmp;
        let a = b"hello";
        let b = b"hello";
        let result = unsafe { bcmp(a.as_ptr() as *const c_void, b.as_ptr() as *const c_void, 5) };
        assert_eq!(result, 0);
    }

    #[test]
    fn test_memset16() {
        use super::memset16;
        let mut buf = [0u16; 5];
        unsafe { memset16(buf.as_mut_ptr(), 0x1234, 5) };
        assert_eq!(&buf, &[0x1234u16; 5]);
    }

    #[test]
    fn test_memset32() {
        use super::memset32;
        let mut buf = [0u32; 5];
        unsafe { memset32(buf.as_mut_ptr(), 0x12345678, 5) };
        assert_eq!(&buf, &[0x12345678u32; 5]);
    }

    #[test]
    fn test_memset64() {
        use super::memset64;
        let mut buf = [0u64; 5];
        unsafe { memset64(buf.as_mut_ptr(), 0x123456789abcdef0, 5) };
        assert_eq!(&buf, &[0x123456789abcdef0u64; 5]);
    }
}
