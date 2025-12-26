/// The `KernelParam` struct represents a kernel module parameter.
///
/// See <https://elixir.bootlin.com/linux/v6.6/source/include/linux/moduleparam.h#L69>
pub struct KernelParam(kbindings::kernel_param);
