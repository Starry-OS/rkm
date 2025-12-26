/// The `Module` struct represents a kernel module.
///
/// See <https://elixir.bootlin.com/linux/v6.6/source/include/linux/module.h#L402>
#[repr(transparent)]
pub struct Module(kbindings::module);

impl Default for Module {
    fn default() -> Self {
        Self(kbindings::module::default())
    }
}

unsafe impl Send for Module {}
unsafe impl Sync for Module {}

impl Module {
    /// Creates a new `Module` instance with the given initialization and exit functions.
    pub const fn new(
        init_fn: Option<unsafe extern "C" fn() -> core::ffi::c_int>,
        exit_fn: Option<unsafe extern "C" fn()>,
    ) -> Self {
        let mut module = core::mem::MaybeUninit::<kbindings::module>::uninit();
        let mut module = unsafe {
            core::ptr::write_bytes(module.as_mut_ptr(), 0, 1);
            module.assume_init()
        };
        module.init = init_fn;
        module.exit = exit_fn;
        Module(module)
    }

    pub fn take_init_fn(&mut self) -> Option<unsafe extern "C" fn() -> core::ffi::c_int> {
        let init_fn = self.0.init.take();
        init_fn
    }

    pub fn take_exit_fn(&mut self) -> Option<unsafe extern "C" fn()> {
        let exit_fn = self.0.exit.take();
        exit_fn
    }
}
