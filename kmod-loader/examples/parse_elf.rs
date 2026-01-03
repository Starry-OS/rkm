use std::env;
use std::path::Path;

use goblin::elf::Elf;
use std::string::{String, ToString};
use std::{collections::BTreeMap, format};

use kmod_loader::arch::{
    Aarch64RelocationType, Loongarch64RelocationType, Riscv64RelocationType, X86_64RelocationType,
};

pub struct ElfParser<'a> {
    elf: Elf<'a>,
    elf_data: &'a [u8],
}

impl<'a> ElfParser<'a> {
    pub fn new(elf_data: &'a [u8]) -> Result<Self, &'static str> {
        let elf = Elf::parse(elf_data).map_err(|_| "Failed to parse ELF data")?;
        if !elf.is_64 {
            return Err("Only 64-bit ELF files are supported");
        }
        Ok(ElfParser { elf, elf_data })
    }

    pub fn print_elf_header(&self) {
        println!("=== ELF header ===");
        println!("ELF Type: {}", self.get_elf_type());
        println!("Machine: {}", self.get_machine_type());
        println!("Version: {}", self.elf.header.e_version);
        println!("Entry point: 0x{:x}", self.elf.header.e_entry);
    }

    pub fn print_sections(&self) {
        println!("=== Sections ===");
        println!(
            "{:<4} {:<25} {:<12} {:<4} {:<10} {:<12}",
            "Index", "Name", "Type", "Flags", "Size", "Align"
        );
        println!("{}", "-".repeat(110));
        for (idx, section) in self.elf.section_headers.iter().enumerate() {
            let mut name = self
                .elf
                .shdr_strtab
                .get_at(section.sh_name)
                .unwrap_or("<unknown>")
                .to_string();
            if name.len() > 25 {
                name.truncate(22);
                name.push_str("...");
            }
            let type_str = self.get_section_type(section.sh_type);
            let flags_str = self.get_section_flags(section.sh_flags);

            println!(
                "{:<4} {:<25} {:<12} {:<4} 0x{:<10x} {:<12}",
                idx, name, type_str, flags_str, section.sh_size, section.sh_addralign
            );
        }
        println!("");
    }

    pub fn print_relocations(&self) {
        println!("=== Relocations ===");
        let mut has_relocs = false;

        for section in self.elf.section_headers.iter() {
            if section.sh_type == goblin::elf::section_header::SHT_REL {
                panic!("REL relocations are not supported in this parser");
            }
            if section.sh_type == goblin::elf::section_header::SHT_RELA {
                has_relocs = true;
                let section_name = self
                    .elf
                    .shdr_strtab
                    .get_at(section.sh_name)
                    .unwrap_or("<unknown>");
                println!("Section: {} (Type: RELA)", section_name);
                // println!(
                //     "{:<16} {:<35} {:<30} {:<16}",
                //     "Offset", "Type", "Symbol", "Addend"
                // );
                // println!("{}", "-".repeat(100));
                println!("{:<35} : Count", "Relocation Type");
                println!("{}", "-".repeat(50));
                self.parse_and_print_rela_relocs(section);
            }
        }

        if !has_relocs {
            println!("No relocation sections found\n");
        } else {
            println!("");
        }
    }

    fn parse_and_print_rela_relocs(&self, section: &goblin::elf::section_header::SectionHeader) {
        let offset = section.sh_offset as usize;

        // Size of Elf64_Rela
        debug_assert!(section.sh_entsize == 24);
        let data = self.elf_data;

        let data_buf = &data[offset..offset + section.sh_size as usize];

        let rela_list = unsafe {
            goblin::elf64::reloc::from_raw_rela(data_buf.as_ptr() as _, section.sh_size as usize)
        };

        let mut rela_ty_list = BTreeMap::<String, usize>::new();

        let mut example = None;
        for rela in rela_list {
            let rel_offset = rela.r_offset;
            let r_info = rela.r_info;
            let addend = rela.r_addend;
            let rel_type = (r_info & 0xffffffff) as u32;
            let sym_idx = (r_info >> 32) as usize;

            let rel_type = self.get_rel_type(rel_type);
            let sym_name = self.get_symbol_name(sym_idx).unwrap_or("unknow");

            if example.is_none() {
                let fmt = format!(
                    "0x{:<14x} {:<35} {:<30} 0x{:x}",
                    rel_offset, rel_type, sym_name, addend
                );
                example = Some(fmt);
            }

            rela_ty_list.entry(rel_type.clone()).or_insert_with(|| 0);
            *rela_ty_list.get_mut(&rel_type).unwrap() += 1;
        }
        for (rel_type, count) in rela_ty_list {
            println!("{:<35} : {}", rel_type, count);
        }

        if let Some(example) = example {
            println!("Example Relocation Entry Format:");
            println!(
                "{:<16} {:<35} {:<30} {:<16}",
                "Offset", "Type", "Symbol", "Addend"
            );
            println!("{}", example);
        }
    }

    fn get_rel_type(&self, rel_type: u32) -> String {
        let ty = match self.get_machine_type() {
            "x86-64" => X86_64RelocationType::try_from(rel_type).map(|ty| format!("{ty:?}")),
            "RISC-V" => Riscv64RelocationType::try_from(rel_type).map(|ty| format!("{ty:?}")),
            "LoongArch" => {
                Loongarch64RelocationType::try_from(rel_type).map(|ty| format!("{ty:?}"))
            }
            "AArch64" => Aarch64RelocationType::try_from(rel_type).map(|ty| format!("{ty:?}")),
            ty => unimplemented!(
                "Relocation type parsing not implemented for machine type: {}",
                ty
            ),
        };
        ty.unwrap_or_else(|_| format!("Unknown({})", rel_type))
    }

    fn get_elf_type(&self) -> &'static str {
        match self.elf.header.e_type {
            goblin::elf::header::ET_NONE => "None (ET_NONE)",
            goblin::elf::header::ET_REL => "Relocatable (ET_REL)",
            goblin::elf::header::ET_EXEC => "Executable (ET_EXEC)",
            goblin::elf::header::ET_DYN => "Shared Object (ET_DYN)",
            goblin::elf::header::ET_CORE => "Core (ET_CORE)",
            _ => "Unknown",
        }
    }

    fn get_section_flags(&self, sh_flags: u64) -> String {
        let mut flags = String::new();
        if sh_flags & goblin::elf::section_header::SHF_WRITE as u64 != 0 {
            flags.push('W');
        }
        if sh_flags & goblin::elf::section_header::SHF_ALLOC as u64 != 0 {
            flags.push('A');
        }
        if sh_flags & goblin::elf::section_header::SHF_EXECINSTR as u64 != 0 {
            flags.push('X');
        }
        flags
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

    fn get_section_type(&self, sh_type: u32) -> &'static str {
        match sh_type {
            goblin::elf::section_header::SHT_NULL => "NULL",
            goblin::elf::section_header::SHT_PROGBITS => "PROGBITS",
            goblin::elf::section_header::SHT_SYMTAB => "SYMTAB",
            goblin::elf::section_header::SHT_STRTAB => "STRTAB",
            goblin::elf::section_header::SHT_RELA => "RELA",
            goblin::elf::section_header::SHT_HASH => "HASH",
            goblin::elf::section_header::SHT_DYNAMIC => "DYNAMIC",
            goblin::elf::section_header::SHT_NOTE => "NOTE",
            goblin::elf::section_header::SHT_NOBITS => "NOBITS",
            goblin::elf::section_header::SHT_REL => "REL",
            goblin::elf::section_header::SHT_SHLIB => "SHLIB",
            goblin::elf::section_header::SHT_DYNSYM => "DYNSYM",
            _ => "Other",
        }
    }

    fn get_symbol_name(&self, sym_idx: usize) -> Result<&'a str, ()> {
        if sym_idx < self.elf.syms.len() {
            if let Some(sym) = self.elf.syms.get(sym_idx) {
                return Ok(self.elf.strtab.get_at(sym.st_name).unwrap_or("<unknown>"));
            }
        }
        Err(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .format_timestamp(None)
        .init();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <ELF file path>", args[0]);
        std::process::exit(1);
    }

    let file_path = Path::new(&args[1]);

    let abs_file_path = if file_path.is_absolute() {
        file_path.to_path_buf()
    } else {
        env::current_dir()?.join(file_path)
    };

    println!("Parsing ELF file: {}", abs_file_path.display());

    let data = std::fs::read(file_path).expect("Failed to read file");
    let data_box = data.into_boxed_slice();

    match ElfParser::new(&data_box) {
        Ok(parser) => {
            parser.print_elf_header();
            parser.print_sections();
            parser.print_relocations();
        }
        Err(e) => {
            eprintln!("Error: Failed to parse ELF file: {}", e);
            std::process::exit(1);
        }
    }
    Ok(())
}
