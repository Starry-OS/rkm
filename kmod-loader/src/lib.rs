#![no_std]
#![allow(unsafe_op_in_unsafe_fn)]
mod arch;
mod loader;
mod module;
mod param;
#[doc(hidden)]
pub use paste;

pub use arch::ArchRelocationType;
use axerrno::{LinuxError, LinuxResult};
pub use loader::{KernelModuleHelper, ModuleLoader, ModuleOwner, SectionMemOps, SectionPerm};

extern crate alloc;

type Result<T> = LinuxResult<T>;
type ModuleErr = LinuxError;
