#![no_std]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(clippy::missing_safety_doc)]
extern crate alloc;

use axerrno::{LinuxError, LinuxResult};

#[allow(dead_code)]
type Result<T> = LinuxResult<T>;
#[allow(dead_code)]
type ModuleErr = LinuxError;

#[cfg(feature = "kstr")]
pub mod kstrtox;
#[cfg(feature = "kmem")]
pub mod mm;
#[cfg(feature = "kparameter")]
pub mod param;
#[cfg(feature = "kstr")]
pub mod string;
#[cfg(feature = "kstr")]
pub mod string_helper;
