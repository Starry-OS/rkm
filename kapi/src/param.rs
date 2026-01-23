use crate::{ModuleErr, Result};
use core::ffi::{
    CStr, c_char, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ulonglong, c_ushort, c_void,
};
use kmod::capi_fn;
use kmod::cdata;
use paste::paste;
/// Flags available for kernel_param_ops
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum ParamOpsFlags {
    /// NOARG - the parameter allows for no argument (foo instead of foo=1)
    KERNEL_PARAM_OPS_FL_NOARG = 1 << 0,
}

pub trait KernelParamValue: Sized {
    fn parse(s: &str) -> Result<Self>;
    fn format(self, buf: *mut u8) -> Result<usize>;
}

fn parse_base<T>(s: &str) -> Result<T>
where
    T: TryFrom<i128>,
{
    let s = s.trim();

    let v = if s.starts_with("0x") || s.starts_with("0X") {
        i128::from_str_radix(&s[2..], 16)
    } else if s.starts_with('0') && s.len() > 1 {
        i128::from_str_radix(&s[1..], 8)
    } else {
        s.parse::<i128>()
    }
    .map_err(|_| ModuleErr::EINVAL)?;

    T::try_from(v).map_err(|_| ModuleErr::EINVAL)
}

fn common_parse<T: KernelParamValue>(val: *const c_char) -> Result<T> {
    let c_str = unsafe { CStr::from_ptr(val) };
    let s = c_str.to_str().map_err(|_| ModuleErr::EINVAL)?;
    let v = T::parse(s)?;
    Ok(v)
}

fn common_set<T: KernelParamValue>(val: *const c_char, kp: *const kmod::kernel_param) -> c_int {
    let v = match common_parse::<T>(val) {
        Ok(v) => v,
        Err(_) => return -(ModuleErr::EINVAL as c_int),
    };
    let arg_ptr = unsafe { kp.as_ref().unwrap().__bindgen_anon_1.arg };
    unsafe {
        *(arg_ptr as *mut T) = v;
    }
    0
}

/// Macro to define standard kernel parameter operations for a given type.
///
/// See <https://elixir.bootlin.com/linux/v6.6/source/kernel/params.c#L218>
macro_rules! impl_macro {
    ($name: ident, $type: ident, $format:expr) => {
        #[repr(transparent)]
        #[derive(Copy, Clone, Debug, PartialEq, Eq)]
        #[allow(non_camel_case_types)]
        struct $name($type);

        impl KernelParamValue for $name {
            fn parse(s: &str) -> Result<Self> {
                let v = parse_base::<$type>(s)?;
                Ok($name(v))
            }

            fn format(self, buf: *mut u8) -> Result<usize> {
                let s = alloc::format!($format, self.0);
                let bytes = s.as_bytes();
                unsafe {
                    core::ptr::copy_nonoverlapping(bytes.as_ptr(), buf, bytes.len());
                }
                Ok(bytes.len())
            }
        }
        paste! {
            #[capi_fn]
            unsafe extern "C" fn [<param_set_$name>](
                val: *const c_char,
                kp: *const kmod::kernel_param,
            ) -> c_int {
                common_set::<$name>(val, kp)
            }

            #[capi_fn]
            unsafe extern "C" fn [<param_get_$name>](
                buffer: *mut c_char,
                kp: *const kmod::kernel_param,
            ) -> c_int {
                let arg_ptr = unsafe { kp.as_ref().unwrap().__bindgen_anon_1.arg };
                let v = unsafe { *(arg_ptr as *const $name) };
                let len = v.format(buffer as *mut u8).unwrap_or(0);
                len as c_int
            }

            #[cdata]
            pub static [<param_ops_$name>]: kmod::kernel_param_ops = kmod::kernel_param_ops {
                set: Some([<param_set_$name>]),
                get: Some([<param_get_$name>]),
                flags: 0,
                free: None,
            };
        }
    };
}

// STANDARD_PARAM_DEF(byte,	unsigned char,		"%hhu",		kstrtou8);
// STANDARD_PARAM_DEF(short,	short,			"%hi",		kstrtos16);
// STANDARD_PARAM_DEF(ushort,	unsigned short,		"%hu",		kstrtou16);
// STANDARD_PARAM_DEF(int,		int,			"%i",		kstrtoint);
// STANDARD_PARAM_DEF(uint,	unsigned int,		"%u",		kstrtouint);
// STANDARD_PARAM_DEF(long,	long,			"%li",		kstrtol);
// STANDARD_PARAM_DEF(ulong,	unsigned long,		"%lu",		kstrtoul);
// STANDARD_PARAM_DEF(ullong,	unsigned long long,	"%llu",		kstrtoull);
// STANDARD_PARAM_DEF(hexint,	unsigned int,		"%#08x", 	kstrtouint);
impl_macro!(byte, c_uchar, "{}\n");
impl_macro!(short, c_short, "{}\n");
impl_macro!(ushort, c_ushort, "{}\n");
impl_macro!(int, c_int, "{}\n");
impl_macro!(uint, c_uint, "{}\n");
impl_macro!(long, c_long, "{}\n");
impl_macro!(ulong, c_ulong, "{}\n");
impl_macro!(ullong, c_ulonglong, "{}\n");
impl_macro!(hexint, c_uint, "{:#08x}\n");

fn maybe_kfree_parameter(arg: *mut c_char) {
    unsafe {
        if !arg.is_null() {
            let _ = alloc::ffi::CString::from_raw(arg);
        }
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
struct charp(*mut c_char);

impl PartialEq for charp {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let s1 = if self.0.is_null() {
                ""
            } else {
                CStr::from_ptr(self.0).to_str().unwrap_or("")
            };
            let s2 = if other.0.is_null() {
                ""
            } else {
                CStr::from_ptr(other.0).to_str().unwrap_or("")
            };
            s1 == s2
        }
    }
}

impl KernelParamValue for charp {
    fn parse(s: &str) -> Result<Self> {
        if s.len() > 1024 {
            return Err(ModuleErr::ENOSPC);
        }
        let c_string = alloc::ffi::CString::new(s).map_err(|_| ModuleErr::EINVAL)?;
        let ptr = c_string.into_raw();
        Ok(charp(ptr))
    }

    fn format(self, buf: *mut u8) -> Result<usize> {
        unsafe {
            let c_str = CStr::from_ptr(self.0);
            let s = alloc::format!("{}\n", c_str.to_str().map_err(|_| ModuleErr::EINVAL)?);
            let bytes = s.as_bytes();
            core::ptr::copy_nonoverlapping(bytes.as_ptr(), buf, bytes.len());
            Ok(bytes.len())
        }
    }
}

unsafe extern "C" fn param_set_charp(val: *const c_char, kp: *const kmod::kernel_param) -> c_int {
    let v = common_parse::<charp>(val);
    let v = match v {
        Ok(v) => v,
        Err(_) => return -(ModuleErr::EINVAL as c_int),
    };

    let arg_ptr = unsafe { kp.as_ref().unwrap().__bindgen_anon_1.arg };
    unsafe {
        // Free the old string if any
        let old_ptr = *(arg_ptr as *mut *mut c_char);
        if !old_ptr.is_null() {
            let old_str = alloc::ffi::CString::from_raw(old_ptr);
            drop(old_str);
        }
        *(arg_ptr as *mut charp) = v;
    }
    0
}

unsafe extern "C" fn param_get_charp(buffer: *mut c_char, kp: *const kmod::kernel_param) -> c_int {
    let arg_ptr = unsafe { kp.as_ref().unwrap().__bindgen_anon_1.arg };
    let v = unsafe { *(arg_ptr as *const charp) };
    let len = v.format(buffer as _).unwrap_or(0);
    len as c_int
}

unsafe extern "C" fn param_free_charp(arg: *mut c_void) {
    maybe_kfree_parameter(*(arg as *mut *mut c_char));
}

#[cdata]
pub static param_ops_charp: kmod::kernel_param_ops = kmod::kernel_param_ops {
    set: Some(param_set_charp),
    get: Some(param_get_charp),
    flags: 0,
    free: Some(param_free_charp),
};

impl KernelParamValue for bool {
    // One of =[yYnN01]
    fn parse(s: &str) -> Result<Self> {
        let s = s.trim();
        // No equals means "set"...
        match s {
            "y" | "Y" | "1" | "" => Ok(true),
            "n" | "N" | "0" => Ok(false),
            _ => Err(ModuleErr::EINVAL),
        }
    }

    fn format(self, buf: *mut u8) -> Result<usize> {
        let s = if self { b"1\n" } else { b"0\n" };
        let bytes = s;
        unsafe {
            core::ptr::copy_nonoverlapping(bytes.as_ptr(), buf, bytes.len());
        }
        Ok(bytes.len())
    }
}

unsafe extern "C" fn param_set_bool(val: *const c_char, kp: *const kmod::kernel_param) -> c_int {
    let val = if val.is_null() {
        c"".as_ptr() // No argument means "set"
    } else {
        val
    };
    common_set::<bool>(val, kp)
}

unsafe extern "C" fn param_get_bool(buffer: *mut c_char, kp: *const kmod::kernel_param) -> c_int {
    let arg_ptr = unsafe { kp.as_ref().unwrap().__bindgen_anon_1.arg };
    let v = unsafe { *(arg_ptr as *const bool) };
    let len = v.format(buffer as _).unwrap_or(0);
    len as c_int
}

#[cdata]
pub static param_ops_bool: kmod::kernel_param_ops = kmod::kernel_param_ops {
    set: Some(param_set_bool),
    get: Some(param_get_bool),
    flags: ParamOpsFlags::KERNEL_PARAM_OPS_FL_NOARG as u32,
    free: None,
};

#[cfg(test)]
mod tests {
    use super::*;

    fn test_param<V: KernelParamValue + core::fmt::Debug + PartialEq>(
        s: &str,
        expected: V,
        excepted_str: &str,
    ) {
        let parsed = V::parse(s).expect("Failed to parse");
        assert_eq!(parsed, expected);

        let mut buf = [0u8; 64];
        let len = parsed.format(buf.as_mut_ptr()).expect("Failed to format");
        let formatted = core::str::from_utf8(&buf[..len]).expect("Invalid UTF-8");
        assert_eq!(formatted, excepted_str);
    }

    #[test]
    fn test_byte_param() {
        test_param("255", byte(255), "255\n");
        test_param("0x7F", byte(127), "127\n");
        test_param("0377", byte(255), "255\n");
    }

    #[test]
    fn test_short_param() {
        test_param("32767", short(32767), "32767\n");
        test_param("-32768", short(-32768), "-32768\n");
        test_param("0x7FFF", short(32767), "32767\n");
        test_param("077777", short(32767), "32767\n");
    }
    #[test]
    fn test_ushort_param() {
        test_param("65535", ushort(65535), "65535\n");
        test_param("0xFFFF", ushort(65535), "65535\n");
        test_param("0177777", ushort(65535), "65535\n");
    }

    #[test]
    fn test_int_param() {
        test_param("2147483647", int(2147483647), "2147483647\n");
        test_param("-2147483648", int(-2147483648), "-2147483648\n");
        test_param("0x7FFFFFFF", int(2147483647), "2147483647\n");
        test_param("017777777777", int(2147483647), "2147483647\n");
    }

    #[test]
    fn test_uint_param() {
        test_param("4294967295", uint(4294967295), "4294967295\n");
        test_param("0xFFFFFFFF", uint(4294967295), "4294967295\n");
        test_param("037777777777", uint(4294967295), "4294967295\n");
    }

    #[test]
    fn test_long_param() {
        test_param(
            "9223372036854775807",
            long(9223372036854775807),
            "9223372036854775807\n",
        );
        test_param(
            "-9223372036854775808",
            long(-9223372036854775808),
            "-9223372036854775808\n",
        );
        test_param(
            "0x7FFFFFFFFFFFFFFF",
            long(9223372036854775807),
            "9223372036854775807\n",
        );
        test_param(
            "0777777777777777777777",
            long(9223372036854775807),
            "9223372036854775807\n",
        );
    }

    #[test]
    fn test_ulong_param() {
        test_param(
            "18446744073709551615",
            ulong(18446744073709551615),
            "18446744073709551615\n",
        );
        test_param(
            "0xFFFFFFFFFFFFFFFF",
            ulong(18446744073709551615),
            "18446744073709551615\n",
        );
        test_param(
            "01777777777777777777777",
            ulong(18446744073709551615),
            "18446744073709551615\n",
        );
    }
    #[test]
    fn test_ullong_param() {
        test_param(
            "18446744073709551615",
            ullong(18446744073709551615),
            "18446744073709551615\n",
        );
        test_param(
            "0xFFFFFFFFFFFFFFFF",
            ullong(18446744073709551615),
            "18446744073709551615\n",
        );
        test_param(
            "01777777777777777777777",
            ullong(18446744073709551615),
            "18446744073709551615\n",
        )
    }

    #[test]
    fn test_hexint_param() {
        test_param("0xDEADBEEF", hexint(0xDEADBEEF), "0xdeadbeef\n");
        test_param("0Xdeadbeef", hexint(0xDEADBEEF), "0xdeadbeef\n");
    }

    #[test]
    fn test_charp_param() {
        let original_str = "Hello, Kernel Param!";
        let expected = charp(alloc::ffi::CString::new(original_str).unwrap().into_raw());
        test_param(original_str, expected, "Hello, Kernel Param!\n");
    }

    #[test]
    fn test_bool_param() {
        test_param("y", true, "1\n");
        test_param("Y", true, "1\n");
        test_param("1", true, "1\n");
        test_param("", true, "1\n");
        test_param("n", false, "0\n");
        test_param("N", false, "0\n");
        test_param("0", false, "0\n");
    }
}
