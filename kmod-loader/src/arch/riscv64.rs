use goblin::{elf::SectionHeader, strtab::Strtab};
use int_enum::IntEnum;

use crate::loader::{KernelModuleHelper, ModuleLoadInfo, ModuleOwner};
use crate::Result;

#[repr(u32)]
#[derive(Debug, Clone, Copy, IntEnum)]
#[allow(non_camel_case_types)]
/// See <https://github.com/gimli-rs/object/blob/af3ca8a2817c8119e9b6d801bd678a8f1880309d/crates/examples/src/readobj/elf.rs#L3124>
pub enum Riscv64RelocationType {
    R_RISCV_NONE,
    R_RISCV_32,
    R_RISCV_64,
    R_RISCV_RELATIVE,
    R_RISCV_COPY,
    R_RISCV_JUMP_SLOT,
    R_RISCV_TLS_DTPMOD32,
    R_RISCV_TLS_DTPMOD64,
    R_RISCV_TLS_DTPREL32,
    R_RISCV_TLS_DTPREL64,
    R_RISCV_TLS_TPREL32,
    R_RISCV_TLS_TPREL64,
    R_RISCV_TLSDESC,
    R_RISCV_BRANCH,
    R_RISCV_JAL,
    R_RISCV_CALL,
    R_RISCV_CALL_PLT,
    R_RISCV_GOT_HI20,
    R_RISCV_TLS_GOT_HI20,
    R_RISCV_TLS_GD_HI20,
    R_RISCV_PCREL_HI20,
    R_RISCV_PCREL_LO12_I,
    R_RISCV_PCREL_LO12_S,
    R_RISCV_HI20,
    R_RISCV_LO12_I,
    R_RISCV_LO12_S,
    R_RISCV_TPREL_HI20,
    R_RISCV_TPREL_LO12_I,
    R_RISCV_TPREL_LO12_S,
    R_RISCV_TPREL_ADD,
    R_RISCV_ADD8,
    R_RISCV_ADD16,
    R_RISCV_ADD32,
    R_RISCV_ADD64,
    R_RISCV_SUB8,
    R_RISCV_SUB16,
    R_RISCV_SUB32,
    R_RISCV_SUB64,
    R_RISCV_GOT32_PCREL,
    R_RISCV_ALIGN,
    R_RISCV_RVC_BRANCH,
    R_RISCV_RVC_JUMP,
    R_RISCV_RVC_LUI,
    R_RISCV_GPREL_I,
    R_RISCV_GPREL_S,
    R_RISCV_TPREL_I,
    R_RISCV_TPREL_S,
    R_RISCV_RELAX,
    R_RISCV_SUB6,
    R_RISCV_SET6,
    R_RISCV_SET8,
    R_RISCV_SET16,
    R_RISCV_SET32,
    R_RISCV_32_PCREL,
    R_RISCV_IRELATIVE,
    R_RISCV_PLT32,
    R_RISCV_SET_ULEB128,
    R_RISCV_SUB_ULEB128,
    R_RISCV_TLSDESC_HI20,
    R_RISCV_TLSDESC_LOAD_LO12,
    R_RISCV_TLSDESC_ADD_LO12,
    R_RISCV_TLSDESC_CALL,
}

impl Riscv64RelocationType {}

pub struct Riscv64ArchRelocate;

impl Riscv64ArchRelocate {
    /// See <https://elixir.bootlin.com/linux/v6.6/source/arch/riscv/kernel/module.c>
    /// See <https://elixir.bootlin.com/linux/v6.6/source/arch/riscv/kernel/module.c#L313>
    pub fn apply_relocate_add<H: KernelModuleHelper>(
        elf_data: &[u8],
        sechdrs: &[SectionHeader],
        strtab: &Strtab<'_>,
        load_info: &ModuleLoadInfo,
        relsec: usize,
        module: &ModuleOwner<H>,
    ) -> Result<()> {
        let rel_section = &sechdrs[relsec];
        let offset = rel_section.sh_offset as usize;

        // Size of Elf64_Rela
        debug_assert!(rel_section.sh_entsize == 24);
        let data = elf_data;

        let data_buf = &data[offset..offset + rel_section.sh_size as usize];
        let rela_list = unsafe {
            goblin::elf64::reloc::from_raw_rela(
                data_buf.as_ptr() as _,
                rel_section.sh_size as usize,
            )
        };
        for rela in rela_list {
            let rel_offset = rela.r_offset;
            let r_info = rela.r_info;
            let addend = rela.r_addend;
            let rel_type = (r_info & 0xffffffff) as u32;
            let sym_idx = (r_info >> 32) as usize;

            // This is where to make the change
            let location = sechdrs[rel_section.sh_info as usize]
                .sh_addr
                .wrapping_add(rel_offset);

            let sym = load_info.syms[sym_idx];

            let reloc_type = Riscv64RelocationType::try_from(rel_type).unwrap();
        }
        todo!("RISC-V relocation application not implemented yet");
    }
}
