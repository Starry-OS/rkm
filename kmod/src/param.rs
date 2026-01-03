/// The `KernelParam` struct represents a kernel module parameter.
///
/// See <https://elixir.bootlin.com/linux/v6.6/source/include/linux/moduleparam.h#L69>
pub struct KernelParam(kbindings::kernel_param);

impl KernelParam {
    pub fn name(&self) -> &str {
        unsafe {
            let c_str = core::ffi::CStr::from_ptr(self.0.name);
            c_str.to_str().unwrap_or_default()
        }
    }
}
