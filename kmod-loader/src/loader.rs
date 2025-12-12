use core::fmt::Display;

use crate::{ModuleLoadErr, Result};
use bitflags::bitflags;
use goblin::elf::Elf;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct SectionPerm: u8 {
        const READ = 0b001;
        const WRITE = 0b010;
        const EXECUTE = 0b100;
    }
}

impl Display for SectionPerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut perms = String::new();
        if self.contains(SectionPerm::READ) {
            perms.push('R');
        }
        if self.contains(SectionPerm::WRITE) {
            perms.push('W');
        }
        if self.contains(SectionPerm::EXECUTE) {
            perms.push('X');
        }
        write!(f, "{}", perms)
    }
}

impl SectionPerm {
    /// Create ModuleSectionPermissions from ELF section flags
    pub fn from_elf_flags(sh_flags: u64) -> Self {
        let mut perms = SectionPerm::empty();
        if (sh_flags & goblin::elf::section_header::SHF_ALLOC as u64) != 0 {
            perms |= SectionPerm::READ;
        }
        if (sh_flags & goblin::elf::section_header::SHF_WRITE as u64) != 0 {
            perms |= SectionPerm::WRITE;
        }
        if (sh_flags & goblin::elf::section_header::SHF_EXECINSTR as u64) != 0 {
            perms |= SectionPerm::EXECUTE;
        }
        perms
    }
}

/// Trait for kernel module helper functions
pub trait KernelModuleHelper {
    /// Allocate virtual memory for module section
    fn vmalloc(size: usize, perms: SectionPerm) -> *mut u8;
    /// Free virtual memory allocated for module section
    fn vfree(ptr: *mut u8, size: usize);
    /// Resolve symbol name to address
    fn resolve_symbol(name: &str) -> Option<usize>;
}

pub struct ModuleLoader<'a, H: KernelModuleHelper> {
    elf: Elf<'a>,
    elf_data: &'a [u8],
    module_name: Option<&'a str>,
    __helper: core::marker::PhantomData<H>,
}

struct SectionPages {
    name: String,
    addr: *mut u8,
    size: usize,
    perms: SectionPerm,
}

pub struct ModuleOwner<H: KernelModuleHelper> {
    name: Option<String>,
    pages: Vec<SectionPages>,
    _helper: core::marker::PhantomData<H>,
}

impl<H: KernelModuleHelper> Drop for ModuleOwner<H> {
    fn drop(&mut self) {
        for page in &self.pages {
            H::vfree(page.addr, page.size);
        }
    }
}

const fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

const fn align_down(addr: usize, align: usize) -> usize {
    addr & !(align - 1)
}

pub struct ModuleLoadInfo {
    pub(crate) syms: Vec<goblin::elf::sym::Sym>,
}

impl<'a, H: KernelModuleHelper> ModuleLoader<'a, H> {
    /// create a new ELF loader
    pub fn new(elf_data: &'a [u8]) -> Result<Self> {
        let elf = Elf::parse(elf_data).map_err(|_| ModuleLoadErr::InvalidElf)?;
        if !elf.is_64 {
            return Err(ModuleLoadErr::UnsupportedArch);
        }
        let module_name = elf.shdr_strtab.get_at(elf.header.e_shstrndx as usize);
        Ok(ModuleLoader {
            elf,
            elf_data,
            module_name,
            __helper: core::marker::PhantomData,
        })
    }

    /// Load the module into kernel space
    pub fn load_module(mut self) -> Result<ModuleOwner<H>> {
        let owner = self.layout_and_allocate()?;
        let load_info = self.simplify_symbols()?;
        self.apply_relocations(load_info, &owner)?;
        unimplemented!()
    }

    /// Layout sections and allocate memory
    /// See <https://elixir.bootlin.com/linux/v6.6/source/kernel/module/main.c#L2363>
    fn layout_and_allocate(&mut self) -> Result<ModuleOwner<H>> {
        let mut owner = ModuleOwner {
            name: self.module_name.map(|s| s.to_string()),
            pages: Vec::new(),
            _helper: core::marker::PhantomData,
        };

        for shdr in &mut self.elf.section_headers {
            let sec_name = self
                .elf
                .shdr_strtab
                .get_at(shdr.sh_name)
                .unwrap_or("<unknown>");

            // Skip non-allocatable sections
            if (shdr.sh_flags & goblin::elf::section_header::SHF_ALLOC as u64) == 0 {
                continue;
            }

            let perms = SectionPerm::from_elf_flags(shdr.sh_flags);
            let size = shdr.sh_size as usize;

            if size == 0 {
                println!("Skipping zero-size section '{}'", sec_name);
                continue;
            }

            let aligned_size = align_up(size, 4096);

            // Allocate memory for the section
            let addr = H::vmalloc(aligned_size, perms);
            if addr.is_null() {
                return Err(ModuleLoadErr::MemoryAllocationFailed);
            }

            // Copy section data from ELF to allocated memory
            let file_offset = shdr.sh_offset as usize;
            let section_data = &self.elf_data[file_offset..file_offset + size];
            unsafe {
                core::ptr::copy_nonoverlapping(section_data.as_ptr(), addr, size);
            }

            // Store the allocated page info
            owner.pages.push(SectionPages {
                name: sec_name.to_string(),
                addr,
                size: aligned_size,
                perms,
            });

            // update section address
            // Note: In a real loader, we would update the section header's sh_addr field
            // to reflect the new virtual address.
            shdr.sh_addr = addr as u64;
        }

        for page in &owner.pages {
            println!(
                "Allocated section '{:>16}' at {:p} [{}] ({:8<#x})",
                page.name, page.addr, page.perms, page.size
            );
        }

        Ok(owner)
    }

    /// Change all symbols so that st_value encodes the pointer directly.
    ///
    /// See <https://elixir.bootlin.com/linux/v6.6/source/kernel/module/main.c#L1367>
    fn simplify_symbols(&self) -> Result<ModuleLoadInfo> {
        let mut loadinfo = ModuleLoadInfo { syms: Vec::new() };

        for sym in self.elf.syms.iter() {
            let sym_name = self.elf.strtab.get_at(sym.st_name).unwrap_or("<unknown>");

            let sym_name = format!("{:#}", rustc_demangle::demangle(sym_name));
            let sym_value = sym.st_value;
            let sym_size = sym.st_size;

            // For debugging purposes, print symbol info
            log::info!(
                "Symbol: ('{}') [{}] Value: 0x{:016x} Size: {}",
                sym_name,
                sym_section_to_str(sym.st_shndx as _),
                sym_value,
                sym_size
            );

            loadinfo.syms.push(sym.clone());

            match sym.st_shndx as _ {
                goblin::elf::section_header::SHN_UNDEF => {
                    // Undefined symbol
                    let sym_address = H::resolve_symbol(&sym_name);
                    // Ok if resolved.
                    if let Some(addr) = sym_address {
                        log::trace!(
                            "  -> Resolved undefined symbol '{}' ({}) to address 0x{:016x}",
                            sym_name,
                            sym_bind_to_str(sym.st_bind()),
                            addr
                        );
                        // we would update the symbol table entry's st_value
                        // to reflect the resolved address.
                        loadinfo.syms.last_mut().unwrap().st_value = addr as u64;
                    } else {
                        // Ok if weak or ignored.
                        if sym.st_bind() == goblin::elf::sym::STB_WEAK {
                            log::warn!(
                                "  -> Unresolved weak symbol '{}' ({})",
                                sym_name,
                                sym_bind_to_str(sym.st_bind())
                            );
                        } else {
                            log::warn!(
                                "  -> Unresolved symbol '{}' ({})",
                                sym_name,
                                sym_bind_to_str(sym.st_bind())
                            );
                        }
                    }
                }
                goblin::elf::section_header::SHN_ABS => {
                    // Don't need to do anything
                    log::debug!("Absolute symbol: {} 0x{:x}", sym_name, sym_value);
                }
                goblin::elf::section_header::SHN_COMMON => {
                    // Ignore common symbols
                    // We compiled with -fno-common. These are not supposed to happen.
                    log::debug!("Common symbol: {}", sym_name);
                    log::warn!(
                        "{}: please compile with -fno-common",
                        self.module_name.unwrap_or("<unknown>")
                    );
                    return Err(ModuleLoadErr::UnsupportedFeature);
                }
                ty => {
                    /* Divert to percpu allocation if a percpu var. */
                    // if (sym[i].st_shndx == info->index.pcpu)
                    //     secbase = (unsigned long)mod_percpu(mod);
                    // else
                    //     secbase = info->sechdrs[sym[i].st_shndx].sh_addr;
                    // sym[i].st_value += secbase;
                    // break;

                    // TODO: Handle special sections like percpu
                    // Normal symbol defined in a section
                    loadinfo.syms.last_mut().unwrap().st_value +=
                        self.elf.section_headers[ty as usize].sh_addr;
                    log::trace!(
                        "  -> Defined symbol '{}' in section {} at address 0x{:016x}",
                        sym_name,
                        ty,
                        loadinfo.syms.last().unwrap().st_value
                    );
                }
            }
        }

        Ok(loadinfo)
    }

    /// See <https://elixir.bootlin.com/linux/v6.6/source/kernel/module/main.c#L1438>
    fn apply_relocations(&self, load_info: ModuleLoadInfo, owner: &ModuleOwner<H>) -> Result<()> {
        for (idx, shdr) in self.elf.section_headers.iter().enumerate() {
            let infosec = shdr.sh_info;

            let sec_name = self
                .elf
                .shdr_strtab
                .get_at(shdr.sh_name)
                .unwrap_or("<unknown>");

            // Not a valid relocation section?
            if infosec >= self.elf.section_headers.len() as u32 {
                continue;
            }
            // Don't bother with non-allocated sections
            if self.elf.section_headers[infosec as usize].sh_flags
                & goblin::elf::section_header::SHF_ALLOC as u64
                == 0
            {
                continue;
            }

            // Skip non-relocation sections
            if shdr.sh_type != goblin::elf::section_header::SHT_RELA {
                continue;
            }

            let to_section = &self.elf.section_headers[infosec as usize];
            let to_sec_name = self
                .elf
                .shdr_strtab
                .get_at(to_section.sh_name)
                .unwrap_or("<unknown>");

            println!(
                "Applying relocations for section '{}' to '{}'",
                sec_name, to_sec_name
            );

            match self.get_machine_type() {
                "RISC-V" => {
                    crate::arch::Riscv64ArchRelocate::apply_relocate_add(
                        self.elf_data,
                        &self.elf.section_headers,
                        &self.elf.strtab,
                        &load_info,
                        idx,
                        owner,
                    )?;
                }
                arch => {
                    panic!("Relocations for architecture '{}' not supported", arch);
                }
            }
        }
        Ok(())
    }

    fn get_machine_type(&self) -> &'static str {
        match self.elf.header.e_machine {
            goblin::elf::header::EM_X86_64 => "x86-64",
            goblin::elf::header::EM_AARCH64 => "AArch64",
            goblin::elf::header::EM_RISCV => "RISC-V",
            goblin::elf::header::EM_LOONGARCH => "LoongArch",
            _ => "unknown",
        }
    }
}

const fn sym_bind_to_str(bind: u8) -> &'static str {
    match bind {
        goblin::elf::sym::STB_LOCAL => "LOCAL",
        goblin::elf::sym::STB_GLOBAL => "GLOBAL",
        goblin::elf::sym::STB_WEAK => "WEAK",
        _ => "UNKNOWN",
    }
}

const fn sym_section_to_str(shndx: u32) -> &'static str {
    match shndx {
        goblin::elf::section_header::SHN_UNDEF => "UNDEF(0)",
        goblin::elf::section_header::SHN_LORESERVE => "LORESERVE(0xff00)",
        // goblin::elf::section_header::SHN_LOPROC => "LOPROC(0xff00)",
        goblin::elf::section_header::SHN_HIPROC => "HIPROC(0xff1f)",
        goblin::elf::section_header::SHN_ABS => "ABS(0xfff1)",
        goblin::elf::section_header::SHN_COMMON => "COMMON(0xfff2)",
        goblin::elf::section_header::SHN_HIRESERVE => "HIRESERVE(0xffff)",
        _ => "OTHER",
    }
}

// #define SHN_LIVEPATCH	0xff20
