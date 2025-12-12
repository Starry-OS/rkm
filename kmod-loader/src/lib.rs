mod arch;
pub mod loader;
mod parser;
pub use parser::ElfParser;

type Result<T> = core::result::Result<T, ModuleLoadErr>;

#[derive(Debug)]
pub enum ModuleLoadErr {
    InvalidElf,
    UnsupportedArch,
    RelocationFailed,
    MemoryAllocationFailed,
    UnsupportedFeature,
    UndefinedSymbol,
}

impl core::fmt::Display for ModuleLoadErr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ModuleLoadErr::InvalidElf => write!(f, "Invalid ELF file"),
            ModuleLoadErr::UnsupportedArch => write!(f, "Unsupported architecture"),
            ModuleLoadErr::RelocationFailed => write!(f, "Relocation failed"),
            ModuleLoadErr::MemoryAllocationFailed => write!(f, "Memory allocation failed"),
            ModuleLoadErr::UnsupportedFeature => write!(f, "Unsupported feature encountered"),
            ModuleLoadErr::UndefinedSymbol => write!(f, "Undefined symbol encountered"),
        }
    }
}

impl core::error::Error for ModuleLoadErr {}
