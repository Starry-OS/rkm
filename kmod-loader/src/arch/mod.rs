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
