#![allow(unused)]

cfg_if::cfg_if! {
    if #[cfg(target_arch = "aarch64")] {
        mod aarch64;
        pub use aarch64::*;
    } else if #[cfg(target_arch = "loongarch64")] {
        mod loongarch64;
        pub use loongarch64::*;
    } else if #[cfg(target_arch = "riscv64")] {
        mod riscv64;
        pub use riscv64::*;
    } else if #[cfg(target_arch = "x86_64")] {
        mod x86_64;
        pub use x86_64::*;
    } else {
        compile_error!("Unsupported architecture");
    }
}

const SZ_128M: u64 = 0x08000000;
const SZ_512K: u64 = 0x00080000;
const SZ_128K: u64 = 0x00020000;
const SZ_2K: u64 = 0x00000800;

/**
 * sign_extend64 - sign extend a 64-bit value using specified bit as sign-bit
 * @value: value to sign extend
 * @index: 0 based bit index (0<=index<64) to sign bit
 */
pub const fn sign_extend64(value: u64, index: u32) -> i64 {
    let shift = 63 - index;
    ((value << shift) as i64) >> shift
}

/// Extracts the relocation type from the r_info field of an Elf64_Rela
const fn get_rela_type(r_info: u64) -> u32 {
    (r_info & 0xffffffff) as u32
}

/// Extracts the symbol index from the r_info field of an Elf64_Rela
const fn get_rela_sym_idx(r_info: u64) -> usize {
    (r_info >> 32) as usize
}

#[derive(Debug, Clone, Copy)]
struct Ptr(u64);
impl Ptr {
    fn as_ptr<T>(&self) -> *mut T {
        self.0 as *mut T
    }

    /// Writes a value of type T to the pointer location
    pub fn write<T>(&self, value: T) {
        unsafe {
            let ptr = self.as_ptr::<T>();
            ptr.write(value);
        }
    }

    pub fn read<T>(&self) -> T {
        unsafe {
            let ptr = self.as_ptr::<T>();
            ptr.read()
        }
    }

    pub fn add(&self, offset: usize) -> Ptr {
        Ptr(self.0 + offset as u64)
    }

    pub fn as_slice<T>(&self, len: usize) -> &[T] {
        unsafe {
            let ptr = self.as_ptr::<T>();
            core::slice::from_raw_parts(ptr, len)
        }
    }
}

#[macro_export]
macro_rules! BIT {
    ($nr:expr) => {
        (1u32 << $nr)
    };
}

#[macro_export]
macro_rules! BIT_U64 {
    ($nr:expr) => {
        (1u64 << $nr)
    };
}

#[cfg(any(target_arch = "loongarch64", target_arch = "riscv64"))]
pub use common::*;

#[cfg(any(target_arch = "loongarch64", target_arch = "riscv64"))]
mod common {
    use goblin::elf::{Elf, Reloc, RelocSection, SectionHeaders};

    use crate::{KernelModuleHelper, ModuleErr, ModuleOwner, Result, arch::PltEntry};
    #[derive(Debug, Clone, Copy, Default)]
    #[repr(C)]
    pub struct ModuleArchSpecific {
        got: ModSection,
        plt: ModSection,
        plt_idx: ModSection,
    }

    #[derive(Debug, Clone, Copy, Default)]
    #[repr(C)]
    pub struct ModSection {
        shndx: usize,
        num_entries: usize,
        max_entries: usize,
    }

    #[derive(Debug, Clone, Copy)]
    #[repr(C)]
    pub struct GotEntry {
        symbol_addr: u64,
    }

    #[derive(Debug, Clone, Copy)]
    #[repr(C)]
    pub struct PltIdxEntry {
        symbol_addr: u64,
    }

    pub fn duplicate_rela(rela_sec: &RelocSection, idx: usize) -> bool {
        let rela_now = rela_sec.get(idx).expect("Invalid relocation index");
        for i in 0..idx {
            let rela_prev = rela_sec.get(i).expect("Invalid relocation index");
            if is_rela_equal(&rela_now, &rela_prev) {
                return true;
            }
        }
        false
    }

    fn is_rela_equal(rela1: &Reloc, rela2: &Reloc) -> bool {
        rela1.r_addend == rela2.r_addend
            && rela1.r_type == rela2.r_type
            && rela1.r_sym == rela2.r_sym
    }

    fn get_got_entry(
        address: u64,
        sechdrs: &SectionHeaders,
        sec: &ModSection,
    ) -> Option<&'static mut GotEntry> {
        let got_entries_addr = sechdrs[sec.shndx].sh_addr;
        let got_entries = unsafe {
            core::slice::from_raw_parts_mut(
                got_entries_addr as *mut GotEntry,
                sec.max_entries as usize,
            )
        };

        got_entries[0..sec.num_entries as usize]
            .iter_mut()
            .find(|entry| entry.symbol_addr == address)
    }

    fn get_plt_idx(address: u64, sechdrs: &SectionHeaders, sec: &ModSection) -> Option<usize> {
        let plt_idx_addr = sechdrs[sec.shndx].sh_addr;
        let plt_idx_entries = unsafe {
            core::slice::from_raw_parts_mut(
                plt_idx_addr as *mut PltIdxEntry,
                sec.max_entries as usize,
            )
        };
        plt_idx_entries[0..sec.num_entries as usize]
            .iter()
            .position(|entry| entry.symbol_addr == address)
    }

    fn get_plt_entry(
        address: u64,
        sechdrs: &SectionHeaders,
        plt_sec: &ModSection,
        plt_idx_sec: &ModSection,
    ) -> Option<&'static mut PltEntry> {
        let plt_idx = get_plt_idx(address, sechdrs, plt_idx_sec);
        if plt_idx.is_none() {
            return None;
        }
        let plt_idx = plt_idx.unwrap();

        let plt_entries_addr = sechdrs[plt_sec.shndx].sh_addr;
        let plt_entries = unsafe {
            core::slice::from_raw_parts_mut(
                plt_entries_addr as *mut PltEntry,
                plt_sec.max_entries as usize,
            )
        };
        Some(&mut plt_entries[plt_idx])
    }

    fn emit_got_entry(address: u64) -> GotEntry {
        GotEntry {
            symbol_addr: address,
        }
    }

    fn emit_plt_idx_entry(address: u64) -> PltIdxEntry {
        PltIdxEntry {
            symbol_addr: address,
        }
    }

    pub fn common_module_emit_got_entry(
        module: &mut ModuleOwner<impl KernelModuleHelper>,
        sechdrs: &SectionHeaders,
        address: u64,
    ) -> Option<&'static mut GotEntry> {
        let got_sec = &mut module.arch.got;
        let idx = got_sec.num_entries;
        let got = get_got_entry(address, sechdrs, got_sec);
        if got.is_some() {
            return got;
        }
        // There is no GOT entry for val yet, create a new one.
        let got_entries_addr = sechdrs[got_sec.shndx].sh_addr;
        let got_entries = unsafe {
            core::slice::from_raw_parts_mut(
                got_entries_addr as *mut GotEntry,
                got_sec.max_entries as usize,
            )
        };
        got_entries[idx as usize] = emit_got_entry(address);
        got_sec.num_entries += 1;
        if got_sec.num_entries > got_sec.max_entries {
            panic!("{}: GOT entries exceed the maximum limit", module.name());
        }
        return Some(&mut got_entries[idx as usize]);
    }

    type ArchEmitPltEntryFunc =
        fn(address: u64, plt_entry_addr: u64, plt_idx_entry_addr: u64) -> PltEntry;

    pub fn common_module_emit_plt_entry(
        module: &mut ModuleOwner<impl KernelModuleHelper>,
        sechdrs: &SectionHeaders,
        address: u64,
        arch_emit_plt_entry_func: ArchEmitPltEntryFunc,
    ) -> Option<&'static mut PltEntry> {
        let plt_sec = &mut module.arch.plt;
        let plt_idx_sec = &mut module.arch.plt_idx;
        let plt = get_plt_entry(address, sechdrs, plt_sec, plt_idx_sec);
        if plt.is_some() {
            return plt;
        }
        let nr = plt_sec.num_entries;
        // There is no duplicate entry, create a new one
        let plt_idx_addr = sechdrs[plt_idx_sec.shndx].sh_addr;
        let plt_idx_entries = unsafe {
            core::slice::from_raw_parts_mut(
                plt_idx_addr as *mut PltIdxEntry,
                plt_idx_sec.max_entries as usize,
            )
        };
        // write the PLT.IDX(loongarch64)/GOT.PLT(riscv64) entry
        plt_idx_entries[nr] = emit_plt_idx_entry(address);

        let plt_entries_addr = sechdrs[plt_sec.shndx].sh_addr;
        let plt_entries = unsafe {
            core::slice::from_raw_parts_mut(
                plt_entries_addr as *mut PltEntry,
                plt_sec.max_entries as usize,
            )
        };
        let plt_entry_addr = &plt_entries[nr] as *const PltEntry as u64;
        let plt_idx_entry_addr = &plt_idx_entries[nr] as *const PltIdxEntry as u64;

        // write the PLT entry
        plt_entries[nr] = arch_emit_plt_entry_func(address, plt_entry_addr, plt_idx_entry_addr);

        plt_sec.num_entries += 1;
        plt_idx_sec.num_entries += 1;

        if plt_sec.num_entries > plt_sec.max_entries {
            panic!("{}: too many PLT entries", module.name());
        }

        return Some(&mut plt_entries[nr]);
    }

    pub type ArchGotPltCounterFunc = fn(rela_sec: &RelocSection) -> (usize, usize);

    fn check_got_plt<H: KernelModuleHelper>(
        elf: &mut Elf,
        owner: &mut ModuleOwner<H>,
        plt_idx_name: &str,
    ) -> Result<()> {
        let mut got_section_idx = None;
        let mut plt_section_idx = None;
        let mut plt_idx_section_idx = None;
        // Find the empty .plt sections.
        for (idx, shdr) in elf.section_headers.iter_mut().enumerate() {
            let sec_name = elf.shdr_strtab.get_at(shdr.sh_name).unwrap_or("<unknown>");
            if sec_name == ".got" {
                got_section_idx = Some(idx);
            } else if sec_name == ".plt" {
                plt_section_idx = Some(idx);
            } else if sec_name == plt_idx_name {
                plt_idx_section_idx = Some(idx);
            }
        }
        if got_section_idx.is_none() {
            log::error!("{:?}: module .GOT section(s) missing", owner.name());
            return Err(ModuleErr::ENOEXEC);
        }
        if plt_section_idx.is_none() {
            log::error!("{:?}: module .PLT section(s) missing", owner.name());
            return Err(ModuleErr::ENOEXEC);
        }
        if plt_idx_section_idx.is_none() {
            log::error!(
                "{:?}: module {} section(s) missing",
                owner.name(),
                plt_idx_name.to_uppercase()
            );
            return Err(ModuleErr::ENOEXEC);
        }

        owner.arch.got.shndx = got_section_idx.unwrap();
        owner.arch.plt.shndx = plt_section_idx.unwrap();
        owner.arch.plt_idx.shndx = plt_idx_section_idx.unwrap();

        Ok(())
    }

    pub fn common_module_frob_arch_sections<H: KernelModuleHelper>(
        elf: &mut Elf,
        owner: &mut ModuleOwner<H>,
        got_plt_counter_func: ArchGotPltCounterFunc,
        plt_idx_name: &str,
    ) -> Result<()> {
        let mut num_plts = 0;
        let mut num_gots = 0;
        // Calculate the maxinum number of entries
        for (idx, rela_sec) in elf.shdr_relocs.iter() {
            let shdr = &elf.section_headers[*idx];
            if shdr.sh_type != goblin::elf::section_header::SHT_RELA {
                continue;
            }
            let infosec = shdr.sh_info;
            let to_section = &elf.section_headers[infosec as usize];
            // ignore relocations that operate on non-exec sections
            if to_section.sh_flags & goblin::elf::section_header::SHF_EXECINSTR as u64 == 0 {
                continue;
            }
            let (plt_entries, got_entries) = got_plt_counter_func(rela_sec);
            num_plts += plt_entries;
            num_gots += got_entries;
        }

        log::info!(
            "[{:?}]: Need {} PLT entries and {} GOT entries",
            owner.name(),
            num_plts,
            num_gots
        );
        check_got_plt(elf, owner, plt_idx_name)?;

        let got_section_idx = owner.arch.got.shndx;
        let plt_section_idx = owner.arch.plt.shndx;
        let plt_idx_section_idx = owner.arch.plt_idx.shndx;

        {
            let got_sec = &mut elf.section_headers[got_section_idx];
            got_sec.sh_type = goblin::elf::section_header::SHT_NOBITS;
            got_sec.sh_flags = goblin::elf::section_header::SHF_ALLOC as u64;
            got_sec.sh_addralign = 64; // TODO: L1_CACHE_BYTES
            got_sec.sh_size = (num_gots as u64 + 1) * size_of::<GotEntry>() as u64;
            owner.arch.got.num_entries = 0;
            owner.arch.got.max_entries = num_gots;
        }

        {
            let plt_sec = &mut elf.section_headers[plt_section_idx];
            plt_sec.sh_type = goblin::elf::section_header::SHT_PROGBITS;
            plt_sec.sh_flags = (goblin::elf::section_header::SHF_ALLOC
                | goblin::elf::section_header::SHF_EXECINSTR) as u64;
            plt_sec.sh_addralign = 64;
            plt_sec.sh_size = (num_plts as u64 + 1) * size_of::<PltEntry>() as u64;
            owner.arch.plt.num_entries = 0;
            owner.arch.plt.max_entries = num_plts;
        }

        {
            let plt_idx_sec = &mut elf.section_headers[plt_idx_section_idx];
            plt_idx_sec.sh_type = goblin::elf::section_header::SHT_PROGBITS;
            plt_idx_sec.sh_flags = goblin::elf::section_header::SHF_ALLOC as u64;
            plt_idx_sec.sh_addralign = 64;
            plt_idx_sec.sh_size = (num_plts as u64 + 1) * size_of::<PltIdxEntry>() as u64;
            owner.arch.plt_idx.num_entries = 0;
            owner.arch.plt_idx.max_entries = num_plts;
        }
        Ok(())
    }
}
