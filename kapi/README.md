# kapi

`kapi` is a `no_std` Rust crate that provides a small Linux kernel compatibility layer for this workspace.

It implements a subset of commonly used kernel-style C APIs in Rust so loadable kernel modules can resolve expected symbols at load time.

## What It Provides

- String and memory helpers such as `strlen`, `strcmp`, `memcpy`, and `memmove`
- String-to-number conversion helpers such as `kstrtoull`, `kstrtoint`, and `kstrtobool`
- Kernel parameter operations such as `param_ops_int`, `param_ops_bool`, and `param_ops_charp`

## Feature Flags

- `kstr`: string, memory, and parsing helpers
- `kmem`: memory duplication helpers
- `kparameter`: kernel parameter operation tables and handlers

## In This Project

`kapi` is part of the `rkm` workspace and is used as a compatibility/symbol-provider crate for kernel module loading and runtime support.

## Status

The crate is intentionally small and focused. It currently covers the APIs needed by the surrounding loader and module infrastructure, with room to grow as compatibility needs expand.
