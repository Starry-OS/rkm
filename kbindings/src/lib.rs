#![no_std]
#![allow(warnings)]

mod aarch64;
mod riscv64;
mod x86_64;

#[cfg(target_arch = "aarch64")]
pub use aarch64::mod_arch_specific;
#[cfg(target_arch = "riscv64")]
pub use riscv64::mod_arch_specific;
#[cfg(target_arch = "x86_64")]
pub use x86_64::mod_arch_specific;

pub type __s8 = core::ffi::c_schar;
pub type __u8 = core::ffi::c_uchar;
pub type __s16 = core::ffi::c_short;
pub type __u16 = core::ffi::c_ushort;
pub type __s32 = core::ffi::c_int;
pub type __u32 = core::ffi::c_uint;
pub type __s64 = core::ffi::c_longlong;
pub type __u64 = core::ffi::c_ulonglong;
pub type s8 = __s8;
pub type u8_ = __u8;
pub type s16 = __s16;
pub type u16_ = __u16;
pub type s32 = __s32;
pub type u32_ = __u32;
pub type s64 = __s64;
pub type u64_ = __u64;
pub const false_: _bindgen_ty_1 = 0;
pub const true_: _bindgen_ty_1 = 1;
pub type _bindgen_ty_1 = core::ffi::c_uint;

pub const module_state_MODULE_STATE_LIVE: module_state = 0;
pub const module_state_MODULE_STATE_COMING: module_state = 1;
pub const module_state_MODULE_STATE_GOING: module_state = 2;
pub const module_state_MODULE_STATE_UNFORMED: module_state = 3;
pub type module_state = core::ffi::c_uint;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}
impl Default for list_head {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kset {
    // TODO: fill fields
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct kobj_type {
    // TODO: fill fields
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kernfs_node {
    // TODO: fill fields
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct atomic_t {
    pub counter: core::ffi::c_int,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct refcount_struct {
    pub refs: atomic_t,
}
pub type refcount_t = refcount_struct;

pub type refcount_saturation_type = core::ffi::c_uint;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct kref {
    pub refcount: refcount_t,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}
impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct kobject {
    pub name: *const core::ffi::c_char,
    pub entry: list_head,
    pub parent: *mut kobject,
    pub kset: *mut kset,
    pub ktype: *const kobj_type,
    pub sd: *mut kernfs_node,
    pub kref: kref,
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize]>,
    pub __bindgen_padding_0: [u8; 3usize],
}

impl Default for kobject {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct module_param_attrs {
    pub _address: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct completion {
    // TODO: fill fields
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct module_kobject {
    pub kobj: kobject,
    pub mod_: *mut module,
    pub drivers_dir: *mut kobject,
    pub mp: *mut module_param_attrs,
    pub kobj_completion: *mut completion,
}

impl Default for module_kobject {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct module_attribute {
    // TODO: fill fields
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct kernel_symbol {
    pub _address: u8,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct optimistic_spin_queue {
    pub tail: atomic_t,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct atomic64_t {
    pub counter: s64,
}
pub type atomic_long_t = atomic64_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qspinlock {
    pub __bindgen_anon_1: qspinlock__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union qspinlock__bindgen_ty_1 {
    pub val: atomic_t,
    pub __bindgen_anon_1: qspinlock__bindgen_ty_1__bindgen_ty_1,
    pub __bindgen_anon_2: qspinlock__bindgen_ty_1__bindgen_ty_2,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct qspinlock__bindgen_ty_1__bindgen_ty_1 {
    pub locked: u8_,
    pub pending: u8_,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct qspinlock__bindgen_ty_1__bindgen_ty_2 {
    pub locked_pending: u16_,
    pub tail: u16_,
}
impl Default for qspinlock__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for qspinlock {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type arch_spinlock_t = qspinlock;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw_spinlock {
    pub raw_lock: arch_spinlock_t,
}
impl Default for raw_spinlock {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type raw_spinlock_t = raw_spinlock;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mutex {
    pub owner: atomic_long_t,
    pub wait_lock: raw_spinlock_t,
    pub osq: optimistic_spin_queue,
    pub wait_list: list_head,
}
impl Default for mutex {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}

pub type bool_ = bool;

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct exception_table_entry {
    pub insn: core::ffi::c_int,
    pub fixup: core::ffi::c_int,
    pub data: core::ffi::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rb_node {
    pub __rb_parent_color: core::ffi::c_ulong,
    pub rb_right: *mut rb_node,
    pub rb_left: *mut rb_node,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct latch_tree_node {
    pub node: [rb_node; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mod_tree_node {
    pub mod_: *mut module,
    pub node: latch_tree_node,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct module_memory {
    pub base: *mut core::ffi::c_void,
    pub size: core::ffi::c_uint,
    pub mtn: mod_tree_node,
}
impl Default for module_memory {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct bug_entry {
    pub bug_addr_disp: core::ffi::c_int,
    pub file_disp: core::ffi::c_int,
    pub line: core::ffi::c_ushort,
    pub flags: core::ffi::c_ushort,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct elf64_sym {
    // TODO: fill fields
}
pub type Elf64_Sym = elf64_sym;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mod_kallsyms {
    pub symtab: *mut Elf64_Sym,
    pub num_symtab: core::ffi::c_uint,
    pub strtab: *mut core::ffi::c_char,
    pub typetab: *mut core::ffi::c_char,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct module_sect_attrs {
    pub _address: u8,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct module_notes_attrs {
    pub _address: u8,
}
#[repr(C)]
#[repr(align(64))]
#[derive(Copy, Clone)]
pub struct srcu_data {
    // TODO: fill fields
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct lockdep_map {}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct srcu_usage {
    // TODO: fill fields
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct srcu_struct {
    pub srcu_idx: core::ffi::c_uint,
    pub sda: *mut srcu_data,
    pub dep_map: lockdep_map,
    pub srcu_sup: *mut srcu_usage,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tracepoint {
    // TODO: fill fields
}

#[repr(C)]
#[repr(align(32))]
#[derive(Debug, Copy, Clone)]
pub struct bpf_raw_event_map {
    pub tp: *mut tracepoint,
    pub bpf_func: *mut core::ffi::c_void,
    pub num_args: u32_,
    pub writable_size: u32_,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct jump_entry {
    pub code: s32,
    pub target: s32,
    pub key: core::ffi::c_long,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct trace_event_call {
    pub _address: u8,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct trace_eval_map {
    pub _address: u8,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct static_call_site {
    pub addr: s32,
    pub key: s32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct klp_modinfo {
    // TODO: fill fields
}

pub type ctor_fn_t = ::core::option::Option<unsafe extern "C" fn()>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _ddebug {
    // TODO: fill fields
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ddebug_class_map {
    // TODO: fill fields
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _ddebug_info {
    pub descs: *mut _ddebug,
    pub classes: *mut ddebug_class_map,
    pub num_descs: core::ffi::c_uint,
    pub num_classes: core::ffi::c_uint,
}

#[repr(C)]
#[repr(align(64))]
#[derive(Copy, Clone)]
pub struct module {
    pub state: module_state,
    pub list: list_head,
    pub name: [core::ffi::c_char; 56usize],

    pub mkobj: module_kobject,
    pub modinfo_attrs: *mut module_attribute,
    pub version: *const core::ffi::c_char,
    pub srcversion: *const core::ffi::c_char,
    pub holders_dir: *mut kobject,

    pub syms: *mut kernel_symbol,
    pub crcs: *const s32,
    pub num_syms: core::ffi::c_uint,

    pub param_lock: mutex,

    pub kp: *mut kernel_param,
    pub num_kp: core::ffi::c_uint,

    pub num_gpl_syms: core::ffi::c_uint,
    pub gpl_syms: *const kernel_symbol,
    pub gpl_crcs: *const s32,
    pub using_gplonly_symbols: bool_,

    // CONFIG_MODULE_SIG
    pub sig_ok: bool_,

    pub async_probe_requested: bool_,

    pub num_exentries: core::ffi::c_uint,
    pub extable: *mut exception_table_entry,

    pub init: ::core::option::Option<unsafe extern "C" fn() -> core::ffi::c_int>,

    pub mem: [module_memory; 7usize],

    pub arch: mod_arch_specific,
    pub taints: core::ffi::c_ulong,

    pub num_bugs: core::ffi::c_uint,
    pub bug_list: list_head,
    pub bug_table: *mut bug_entry,

    pub kallsyms: *mut mod_kallsyms,
    pub core_kallsyms: mod_kallsyms,

    pub sect_attrs: *mut module_sect_attrs,
    pub notes_attrs: *mut module_notes_attrs,

    pub args: *mut core::ffi::c_char,

    pub percpu: *mut core::ffi::c_void,
    pub percpu_size: core::ffi::c_uint,

    pub noinstr_text_start: *mut core::ffi::c_void,
    pub noinstr_text_size: core::ffi::c_uint,

    pub num_tracepoints: core::ffi::c_uint,
    pub tracepoints_ptrs: *const core::ffi::c_int,

    pub num_srcu_structs: core::ffi::c_uint,
    pub srcu_struct_ptrs: *mut *mut srcu_struct,

    pub num_bpf_raw_events: core::ffi::c_uint,
    pub bpf_raw_events: *mut bpf_raw_event_map,

    pub btf_data_size: core::ffi::c_uint,
    pub btf_base_data_size: core::ffi::c_uint,
    pub btf_data: *mut core::ffi::c_void,
    pub btf_base_data: *mut core::ffi::c_void,

    pub jump_entries: *mut jump_entry,
    pub num_jump_entries: core::ffi::c_uint,

    pub num_trace_bprintk_fmt: core::ffi::c_uint,
    pub trace_bprintk_fmt_start: *mut *const core::ffi::c_char,

    pub trace_events: *mut *mut trace_event_call,
    pub num_trace_events: core::ffi::c_uint,
    pub trace_evals: *mut *mut trace_eval_map,
    pub num_trace_evals: core::ffi::c_uint,

    pub num_ftrace_callsites: core::ffi::c_uint,
    pub ftrace_callsites: *mut core::ffi::c_ulong,

    pub kprobes_text_start: *mut core::ffi::c_void,
    pub kprobes_text_size: core::ffi::c_uint,
    pub kprobe_blacklist: *mut core::ffi::c_ulong,
    pub num_kprobe_blacklist: core::ffi::c_uint,

    // CONFIG_HAVE_STATIC_CALL_INLINE
    // pub num_static_call_sites: core::ffi::c_int,
    // pub static_call_sites: *mut static_call_site,

    // CONFIG_LIVEPATCH
    // pub klp: bool_,
    // pub klp_alive: bool_,
    // pub klp_info: *mut klp_modinfo,
    pub source_list: list_head,
    pub target_list: list_head,

    pub exit: ::core::option::Option<unsafe extern "C" fn()>,
    pub refcnt: atomic_t,
    // CONFIG_CONSTRUCTORS: disabled for most configurations
    // pub ctors: *mut ctor_fn_t,
    // pub num_ctors: core::ffi::c_uint,
    pub dyndbg_info: _ddebug_info,
}

impl Default for module {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct kernel_param_ops {
    pub flags: core::ffi::c_uint,
    pub set: ::core::option::Option<
        unsafe extern "C" fn(
            val: *const core::ffi::c_char,
            kp: *const kernel_param,
        ) -> core::ffi::c_int,
    >,
    pub get: ::core::option::Option<
        unsafe extern "C" fn(
            buffer: *mut core::ffi::c_char,
            kp: *const kernel_param,
        ) -> core::ffi::c_int,
    >,
    pub free: ::core::option::Option<unsafe extern "C" fn(arg: *mut core::ffi::c_void)>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct kparam_string {
    pub maxlen: core::ffi::c_uint,
    pub string: *mut core::ffi::c_char,
}
impl Default for kparam_string {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct kparam_array {
    pub max: core::ffi::c_uint,
    pub elemsize: core::ffi::c_uint,
    pub num: *mut core::ffi::c_uint,
    pub ops: *const kernel_param_ops,
    pub elem: *mut core::ffi::c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union kernel_param__bindgen_ty_1 {
    pub arg: *mut core::ffi::c_void,
    pub str_: *const kparam_string,
    pub arr: *const kparam_array,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kernel_param {
    pub name: *const core::ffi::c_char,
    pub mod_: *mut module,
    pub ops: *const kernel_param_ops,
    pub perm: u16_,
    pub level: s8,
    pub flags: u8_,
    pub __bindgen_anon_1: kernel_param__bindgen_ty_1,
}
