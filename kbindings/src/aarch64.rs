/// Linux localhost 6.12.57+deb13-arm64 #1 SMP Debian 6.12.57-1 (2025-11-05) aarch64 GNU/Linux

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct mod_arch_specific {
    pub core: mod_plt_sec,
    pub init: mod_plt_sec,
    // CONFIG_DYNAMIC_FTRACE
    pub ftrace_trampolines: *mut plt_entry,
    pub init_ftrace_trampolines: *mut plt_entry,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct mod_plt_sec {
    pub plt_shndx: core::ffi::c_int,
    pub plt_num_entries: core::ffi::c_int,
    pub plt_max_entries: core::ffi::c_int,
}

/*
 * A program that conforms to the AArch64 Procedure Call Standard
 * (AAPCS64) must assume that a veneer that alters IP0 (x16) and/or
 * IP1 (x17) may be inserted at any branch instruction that is
 * exposed to a relocation that supports long branches. Since that
 * is exactly what we are dealing with here, we are free to use x16
 * as a scratch register in the PLT veneers.
 */
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
struct plt_entry {
    /// adrp x16, ....
    adrp: u32,
    /// add x16, x16, #0x....
    add: u32,
    /// br x16
    br: u32,
}
