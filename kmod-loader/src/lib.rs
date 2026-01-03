#![no_std]

pub mod arch;
mod loader;
mod module;

use alloc::string::String;
pub use loader::{KernelModuleHelper, ModuleLoader, ModuleOwner, SectionMemOps, SectionPerm};
extern crate alloc;

type Result<T> = core::result::Result<T, ModuleErr>;

#[derive(Debug)]
pub enum ModuleErr {
    InvalidElf,
    InvalidOperation,
    UnsupportedArch,
    RelocationFailed(String),
    MemoryAllocationFailed,
    UnsupportedFeature,
    UndefinedSymbol,
}

impl core::fmt::Display for ModuleErr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ModuleErr::InvalidElf => write!(f, "Invalid ELF file"),
            ModuleErr::InvalidOperation => write!(f, "Invalid operation"),
            ModuleErr::UnsupportedArch => write!(f, "Unsupported architecture"),
            ModuleErr::RelocationFailed(msg) => write!(f, "Relocation failed: {}", msg),
            ModuleErr::MemoryAllocationFailed => write!(f, "Memory allocation failed"),
            ModuleErr::UnsupportedFeature => write!(f, "Unsupported feature encountered"),
            ModuleErr::UndefinedSymbol => write!(f, "Undefined symbol encountered"),
        }
    }
}

impl core::error::Error for ModuleErr {}
