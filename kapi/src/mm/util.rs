use core::ffi::c_char;

use alloc::vec::Vec;
use kmod::capi_fn;

/// kstrndup - allocate space for and copy an existing string
///
/// # Arguments
/// - s: the string to duplicate
/// - max: read at most @max chars from @s
/// - gfp: the GFP mask used in the kmalloc() call when allocating memory
///
/// # Note
/// Use kmemdup_nul() instead if the size is known exactly.
///
/// # Returns
/// newly allocated copy of @s or %NULL in case of error
#[capi_fn]
pub unsafe extern "C" fn kstrndup(s: *const c_char, max: usize, _gfp: u32) -> *mut c_char {
    if s.is_null() {
        return core::ptr::null_mut();
    }
    let len = crate::string::strnlen(s, max);
    let buf: *mut c_char = Vec::with_capacity(len + 1).leak().as_mut_ptr();
    if !buf.is_null() {
        crate::string::memcpy(
            buf as *mut core::ffi::c_void,
            s as *const core::ffi::c_void,
            len,
        );
        *buf.add(len) = 0;
    }
    buf
}

/// kmemdup - duplicate region of memory
/// # Arguments
/// - src: memory region to duplicate
/// - len: memory region length
/// - gfp: GFP mask to use
/// # Returns
/// newly allocated copy of @src or %NULL in case of error,
/// result is physically contiguous. Use kfree() to free.
#[capi_fn]
pub unsafe extern "C" fn kmemdup(
    src: *const core::ffi::c_void,
    len: usize,
    _gfp: u32,
) -> *mut core::ffi::c_void {
    if src.is_null() {
        return core::ptr::null_mut();
    }
    let buf: *mut core::ffi::c_void = Vec::with_capacity(len).leak().as_mut_ptr();
    if !buf.is_null() {
        crate::string::memcpy(buf, src, len);
    }
    buf
}
