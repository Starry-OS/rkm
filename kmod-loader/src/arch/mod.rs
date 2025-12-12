mod aarch64;
mod loongarch64;
mod riscv64;
mod x86_64;

pub use aarch64::Aarch64RelocationType;
pub use loongarch64::Loongarch64RelocationType;
pub use riscv64::{Riscv64ArchRelocate, Riscv64RelocationType};
pub use x86_64::X86_64RelocationType;
