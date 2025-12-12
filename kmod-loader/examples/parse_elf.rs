use kmod_loader::loader::{KernelModuleHelper, ModuleLoader};
use kmod_loader::ElfParser;
use std::env;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
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

    let loader = ModuleLoader::<FakeHelper>::new(&data_box)?;
    let owner = loader.load_module()?;

    Ok(())
}

struct FakeHelper;

impl KernelModuleHelper for FakeHelper {
    fn vmalloc(size: usize, perms: kmod_loader::loader::SectionPerm) -> *mut u8 {
        assert!(size % 4096 == 0);
        let layout = std::alloc::Layout::from_size_align(size, 4096).unwrap();
        unsafe { std::alloc::alloc_zeroed(layout) }
    }
    fn vfree(ptr: *mut u8, size: usize) {
        assert!(size % 4096 == 0);
        let layout = std::alloc::Layout::from_size_align(size, 4096).unwrap();
        unsafe { std::alloc::dealloc(ptr, layout) }
    }

    fn resolve_symbol(name: &str) -> Option<usize> {
        // println!("Resolving symbol: {}", name);
        Some(0)
    }
}
