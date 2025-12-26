#![no_std]
#![feature(linkage)]

mod module;
mod param;
pub use kmacro::{exit_fn, init_fn, module};
pub use module::Module;
pub use param::KernelParam;
