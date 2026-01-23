use crate::Result;
use alloc::ffi::CString;
use axerrno::LinuxError;
use core::ffi::CStr;
use kapi::param::ParamOpsFlags;
use kmod::KernelParam;

/// Parse a string to get a param value pair.
/// You can use " around spaces, but can't escape ".
/// Hyphens and underscores equivalent in parameter names.
fn next_arg(mut args: &mut [u8]) -> Result<(&CStr, Option<&CStr>, &mut [u8])> {
    let mut equals = None;
    let mut in_quote = false;
    let mut quoted = false;

    if args[0] == b'"' {
        args = &mut args[1..];
        in_quote = true;
        quoted = true;
    }

    let mut idx = 0;
    while args[idx] != b'\0' {
        let b = args[idx];
        if b.is_ascii_whitespace() && !in_quote {
            break;
        }
        if equals.is_none() && b == b'=' {
            equals = Some(idx);
        }
        if b == b'"' {
            in_quote = !in_quote;
        }
        idx += 1;
    }
    let param_start = args.as_ptr();
    let val_start = if let Some(equals_idx) = equals {
        // Split at equals
        args[equals_idx] = b'\0';
        let mut val_idx = equals_idx + 1;
        // Don't include quotes in value.
        if args[val_idx] == b'"' {
            val_idx += 1;
            if args[idx - 1] == b'"' {
                args[idx - 1] = b'\0';
            }
        }
        let val_start = unsafe { args.as_ptr().add(val_idx) };
        Some(val_start)
    } else {
        None
    };

    if quoted && idx > 0 && args[idx - 1] == b'"' {
        args[idx - 1] = b'\0';
    }
    if args[idx] != b'\0' {
        args[idx] = b'\0';
        args = &mut args[idx + 1..];
    } else {
        args = &mut args[idx..];
    }

    args = skip_spaces(args);

    let (param, val) = unsafe {
        let param = CStr::from_ptr(param_start as _);
        let val = val_start.map(|v| CStr::from_ptr(v as _));
        (param, val)
    };
    Ok((param, val, args))
}

fn skip_spaces(mut args: &mut [u8]) -> &mut [u8] {
    while let Some(&b) = args.first() {
        if b.is_ascii_whitespace() {
            args = &mut args[1..];
        } else {
            break;
        }
    }
    args
}

fn dash2underscore(c: u8) -> u8 {
    if c == b'-' { b'_' } else { c }
}

/// See <https://elixir.bootlin.com/linux/v6.6/source/kernel/params.c#L85>
fn parameqn(a: &CStr, b: &CStr, n: usize) -> bool {
    let a_bytes = a.to_bytes();
    let b_bytes = b.to_bytes();
    if a_bytes.len() < n || b_bytes.len() < n {
        return false;
    }

    for i in 0..n {
        if dash2underscore(a_bytes[i]) != dash2underscore(b_bytes[i]) {
            return false;
        }
    }
    true
}

fn parameq(a: &CStr, b: &CStr) -> bool {
    parameqn(a, b, a.to_bytes().len())
}

fn parse_one(
    param: &CStr,
    val: Option<&CStr>,
    doing: &str,
    params: &mut [KernelParam],
    min_level: i16,
    max_level: i16,
) -> Result<()> {
    for kp in params.iter_mut() {
        let name = kp.raw_name();
        if parameq(name, param) {
            if kp.level() < min_level || kp.level() > max_level {
                return Ok(());
            }
            let param_ops_flags = unsafe { kp.param_ops_flags() };
            // No one handled NULL, so do it here.
            if val.is_none()
                && param_ops_flags & (ParamOpsFlags::KERNEL_PARAM_OPS_FL_NOARG as u32) == 0
            {
                log::warn!(
                    "[{}] Parameter '{}' requires an argument",
                    doing,
                    name.to_str().unwrap(),
                );
                return Err(LinuxError::EINVAL);
            }
            log::debug!(
                "[{}] handling {} with {:?}\n",
                doing,
                param.to_str().unwrap(),
                kp.ops().set
            );
            let set = kp.ops().set.unwrap();
            let res = unsafe {
                set(
                    val.map_or(core::ptr::null(), |v| v.as_ptr()),
                    kp.raw_kernel_param(),
                )
            };
            if res < 0 {
                return Err(LinuxError::try_from(-res).unwrap());
            } else {
                return Ok(());
            }
        }
    }
    Err(LinuxError::ENOENT)
}

pub(crate) fn parse_args(
    doing: &str,
    args: CString,
    params: &mut [KernelParam],
    min_level: i16,
    max_level: i16,
) -> Result<CString> {
    log::error!("[{}]: parsing args '{:?}'", doing, args);
    let mut args = args.into_bytes_with_nul();
    let mut args = args.as_mut_slice();
    // skip spaces
    args = skip_spaces(args);
    if args.is_empty() {
        return Ok(CString::new("").unwrap());
    }

    while args[0] != b'\0' {
        let (param, val, new_args) = next_arg(args)?;
        args = new_args;
        // Stop at --
        if val.is_none() && param.to_bytes() == b"--" {
            // Remove the NUL terminator from the end of args before creating CString
            let args_without_nul = if args.last() == Some(&b'\0') {
                &args[..args.len() - 1]
            } else {
                args
            };
            return Ok(CString::new(args_without_nul).unwrap());
        }
        let res = parse_one(param, val, doing, params, min_level, max_level);
        match res {
            Err(LinuxError::ENOENT) => {
                log::error!(
                    "[{}]: Unknown parameter '{}'",
                    doing,
                    param.to_str().unwrap()
                );
                return Err(LinuxError::ENOENT);
            }
            Err(LinuxError::ENOSPC) => {
                log::error!(
                    "[{}]: '{:?}' too large for parameter '{}'",
                    doing,
                    val,
                    param.to_str().unwrap()
                );
                return Err(LinuxError::ENOSPC);
            }
            Err(e) => {
                log::error!(
                    "[{}]: '{:?}' invalid for parameter '{}'",
                    doing,
                    val,
                    param.to_str().unwrap()
                );
                return Err(e);
            }
            Ok(()) => { /* Parsed successfully */ }
        }
    }
    Ok(CString::new("").unwrap())
}

#[cfg(test)]
mod tests {
    use core::ffi::{c_char, c_int};

    use alloc::{borrow::ToOwned, boxed::Box};
    use kapi::param::param_ops_int;

    use super::*;

    #[test]
    fn test_parameq() {
        let a = CString::new("param-name").unwrap();
        let b = CString::new("param_name").unwrap();
        let c = CString::new("paramname").unwrap();
        assert!(parameq(&a, &b));
        assert!(!parameq(&a, &c));
    }

    #[test]
    fn test_next_arg() {
        let mut args = b"param1=val1 param2=\"val 2\" param3=val3\0".to_owned();
        let args_slice = args.as_mut_slice();
        let (param, val, rest) = next_arg(args_slice).expect("Failed to parse arg1");
        assert_eq!(param, c"param1");
        assert_eq!(val, Some(c"val1"));
        assert_eq!(rest, b"param2=\"val 2\" param3=val3\0");
        let (param, val, rest) = next_arg(rest).expect("Failed to parse arg2");
        assert_eq!(param, c"param2");
        assert_eq!(val, Some(c"val 2"));
        assert_eq!(rest, b"param3=val3\0");
        let (param, val, rest) = next_arg(rest).expect("Failed to parse arg3");
        assert_eq!(param, c"param3");
        assert_eq!(val, Some(c"val3"));
        assert_eq!(rest, b"\0");
    }

    #[test]
    fn test_next_arg_no_value() {
        let mut args = b"param1 param2=\"val 2\" -- param3=val3\0".to_owned();
        let args_slice = args.as_mut_slice();
        let (param, val, rest) = next_arg(args_slice).expect("Failed to parse arg1");
        assert_eq!(param, c"param1");
        assert_eq!(val, None);
        assert_eq!(rest, b"param2=\"val 2\" -- param3=val3\0");
        let (param, val, rest) = next_arg(rest).expect("Failed to parse arg2");
        assert_eq!(param, c"param2");
        assert_eq!(val, Some(c"val 2"));
        assert_eq!(rest, b"-- param3=val3\0");
    }

    // Helper function to create test kernel params
    // Note: This is a simplified approach that uses unsafe code to create mock KernelParam structures for testing
    fn create_test_param_int(name: &'static CStr, value_ptr: *mut c_int) -> KernelParam {
        // Use mem::transmute to bypass the type system for testing
        // This is safe in test context as we control all the types
        let param_raw: kmod::kernel_param = unsafe {
            let mut param = core::mem::MaybeUninit::<kmod::kernel_param>::zeroed();
            let p = param.as_mut_ptr();
            (*p).name = name.as_ptr() as *mut c_char;
            (*p).mod_ = core::ptr::null_mut();
            (*p).ops = &param_ops_int;
            (*p).perm = 0;
            (*p).level = 0;
            (*p).flags = 0;
            // Set the union field arg
            core::ptr::write(
                &mut (*p).__bindgen_anon_1 as *mut _ as *mut *mut core::ffi::c_void,
                value_ptr as *mut core::ffi::c_void,
            );
            param.assume_init()
        };

        KernelParam::from_raw(param_raw)
    }

    fn create_test_param_bool(name: &'static CStr, value_ptr: *mut bool) -> KernelParam {
        let param_raw: kmod::kernel_param = unsafe {
            let mut param = core::mem::MaybeUninit::<kmod::kernel_param>::zeroed();
            let p = param.as_mut_ptr();
            (*p).name = name.as_ptr() as *mut c_char;
            (*p).mod_ = core::ptr::null_mut();
            (*p).ops = &kapi::param::param_ops_bool;
            (*p).perm = 0;
            (*p).level = 0;
            (*p).flags = 0;
            core::ptr::write(
                &mut (*p).__bindgen_anon_1 as *mut _ as *mut *mut core::ffi::c_void,
                value_ptr as *mut core::ffi::c_void,
            );
            param.assume_init()
        };

        KernelParam::from_raw(param_raw)
    }

    fn create_test_param_charp(name: &'static CStr, value_ptr: *mut *mut c_char) -> KernelParam {
        let param_raw: kmod::kernel_param = unsafe {
            let mut param = core::mem::MaybeUninit::<kmod::kernel_param>::zeroed();
            let p = param.as_mut_ptr();
            (*p).name = name.as_ptr() as *mut c_char;
            (*p).mod_ = core::ptr::null_mut();
            (*p).ops = &kapi::param::param_ops_charp;
            (*p).perm = 0;
            (*p).level = 0;
            (*p).flags = 0;
            core::ptr::write(
                &mut (*p).__bindgen_anon_1 as *mut _ as *mut *mut core::ffi::c_void,
                value_ptr as *mut core::ffi::c_void,
            );
            param.assume_init()
        };

        KernelParam::from_raw(param_raw)
    }

    fn create_test_params() -> alloc::vec::Vec<KernelParam> {
        use core::ffi::c_char;

        // Create test variables to hold parameter values

        let test_int = Box::leak(Box::new(0 as c_int));
        let test_bool = Box::leak(Box::new(false as bool));
        let test_str = Box::leak(Box::new(core::ptr::null_mut() as *mut c_char));

        // Reset variables before each test
        unsafe {
            *test_int = 0;
            *test_bool = false;
            if !(*test_str).is_null() {
                let _ = CString::from_raw(*test_str);
                *test_str = core::ptr::null_mut();
            }

            let int_param = create_test_param_int(c"test_int", test_int);
            let bool_param = create_test_param_bool(c"test_bool", test_bool);
            let str_param = create_test_param_charp(c"test_str", test_str);

            alloc::vec![int_param, bool_param, str_param]
        }
    }

    #[test]
    fn test_parse_args_single_int() {
        let mut params = create_test_params();
        let args = CString::new("test_int=42").unwrap();
        let result = parse_args("test", args, &mut params, i16::MIN, i16::MAX);
        assert!(result.is_ok());

        // Verify the value was set
        let arg_ptr = unsafe { params[0].raw_kernel_param().__bindgen_anon_1.arg };
        let value = unsafe { *(arg_ptr as *const c_int) };
        assert_eq!(value, 42);
    }

    #[test]
    fn test_parse_args_multiple_params() {
        let mut params = create_test_params();
        let args = CString::new("test_int=123 test_bool=y test_str=hello").unwrap();
        let result = parse_args("test", args, &mut params, i16::MIN, i16::MAX);
        assert!(result.is_ok());

        // Verify int value
        let int_ptr = unsafe { params[0].raw_kernel_param().__bindgen_anon_1.arg };
        let int_val = unsafe { *(int_ptr as *const c_int) };
        assert_eq!(int_val, 123);

        // Verify bool value
        let bool_ptr = unsafe { params[1].raw_kernel_param().__bindgen_anon_1.arg };
        let bool_val = unsafe { *(bool_ptr as *const bool) };
        assert_eq!(bool_val, true);

        // Verify string value
        let str_ptr = unsafe { params[2].raw_kernel_param().__bindgen_anon_1.arg };
        let str_val = unsafe { *(str_ptr as *const *mut c_char) };
        assert!(!str_val.is_null());
        let c_str = unsafe { CStr::from_ptr(str_val) };
        assert_eq!(c_str.to_str().unwrap(), "hello");
    }

    #[test]
    fn test_parse_args_with_quotes() {
        let mut params = create_test_params();
        let args = CString::new("test_str=\"hello world\"").unwrap();
        let result = parse_args("test", args, &mut params, i16::MIN, i16::MAX);
        assert!(result.is_ok());

        let str_ptr = unsafe { params[2].raw_kernel_param().__bindgen_anon_1.arg };
        let str_val = unsafe { *(str_ptr as *const *mut c_char) };
        assert!(!str_val.is_null());
        let c_str = unsafe { CStr::from_ptr(str_val) };
        assert_eq!(c_str.to_str().unwrap(), "hello world");
    }

    #[test]
    fn test_parse_args_bool_no_value() {
        let mut params = create_test_params();
        let args = CString::new("test_bool").unwrap();
        let result = parse_args("test", args, &mut params, i16::MIN, i16::MAX);
        assert!(result.is_ok());

        let bool_ptr = unsafe { params[1].raw_kernel_param().__bindgen_anon_1.arg };
        let bool_val = unsafe { *(bool_ptr as *const bool) };
        assert_eq!(bool_val, true);
    }

    #[test]
    fn test_parse_args_double_dash() {
        let mut params = create_test_params();
        let args = CString::new("test_int=10 -- test_bool=y").unwrap();
        let result = parse_args("test", args, &mut params, i16::MIN, i16::MAX);
        assert!(result.is_ok());

        // Check that only test_int was processed
        let int_ptr = unsafe { params[0].raw_kernel_param().__bindgen_anon_1.arg };
        let int_val = unsafe { *(int_ptr as *const c_int) };
        assert_eq!(int_val, 10);

        // The remaining args should be returned (with leading space)
        let remaining = result.unwrap();
        assert_eq!(remaining.to_str().unwrap(), "test_bool=y");
    }

    #[test]
    fn test_parse_args_unknown_param() {
        let mut params = create_test_params();
        let args = CString::new("unknown_param=123").unwrap();
        let result = parse_args("test", args, &mut params, i16::MIN, i16::MAX);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), LinuxError::ENOENT);
    }

    #[test]
    fn test_parse_args_invalid_value() {
        let mut params = create_test_params();
        let args = CString::new("test_int=not_a_number").unwrap();
        let result = parse_args("test", args, &mut params, i16::MIN, i16::MAX);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_args_hyphen_underscore() {
        let mut params = create_test_params();
        // test-int should match test_int
        let args = CString::new("test-int=999").unwrap();
        let result = parse_args("test", args, &mut params, i16::MIN, i16::MAX);
        assert!(result.is_ok());

        let int_ptr = unsafe { params[0].raw_kernel_param().__bindgen_anon_1.arg };
        let int_val = unsafe { *(int_ptr as *const c_int) };
        assert_eq!(int_val, 999);
    }

    #[test]
    fn test_parse_args_hex_values() {
        let mut params = create_test_params();
        let args = CString::new("test_int=0xFF").unwrap();
        let result = parse_args("test", args, &mut params, i16::MIN, i16::MAX);
        assert!(result.is_ok());

        let int_ptr = unsafe { params[0].raw_kernel_param().__bindgen_anon_1.arg };
        let int_val = unsafe { *(int_ptr as *const c_int) };
        assert_eq!(int_val, 255);
    }

    #[test]
    fn test_parse_args_empty_string() {
        let mut params = create_test_params();
        let args = CString::new("").unwrap();
        let result = parse_args("test", args, &mut params, i16::MIN, i16::MAX);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_str().unwrap(), "");
    }

    #[test]
    fn test_parse_args_spaces() {
        let mut params = create_test_params();
        let args = CString::new("  test_int=50  test_bool=n  ").unwrap();
        let result = parse_args("test", args, &mut params, i16::MIN, i16::MAX);
        assert!(result.is_ok());

        let int_ptr = unsafe { params[0].raw_kernel_param().__bindgen_anon_1.arg };
        let int_val = unsafe { *(int_ptr as *const c_int) };
        assert_eq!(int_val, 50);

        let bool_ptr = unsafe { params[1].raw_kernel_param().__bindgen_anon_1.arg };
        let bool_val = unsafe { *(bool_ptr as *const bool) };
        assert_eq!(bool_val, false);
    }
}
