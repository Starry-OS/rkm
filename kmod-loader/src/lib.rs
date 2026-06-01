#![no_std]
#![allow(unsafe_op_in_unsafe_fn)]
mod arch;
mod loader;
mod module;
mod param;
extern crate alloc;
pub use arch::ArchRelocationType;
use ax_errno::{LinuxError, LinuxResult};
pub use loader::{KernelModuleHelper, ModuleLoader, ModuleOwner, SectionMemOps, SectionPerm};
#[doc(hidden)]
pub use paste;

type Result<T> = LinuxResult<T>;
type ModuleErr = LinuxError;
