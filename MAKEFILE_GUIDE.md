# Makefile - 内核模块构建系统

## 概述

用于编译和链接内核可加载模块(LKM)。

## 主要特性

✅ **多架构支持** - 支持x86-64、RISC-V、ARM等目标架构  
✅ **自动化构建** - 一键编译、提取和链接所有模块  
✅ **灵活配置** - 通过变量自定义目标架构和路径  
✅ **验证机制** - 自动验证生成的.ko文件  
✅ **清理工具** - 完整的清理和重建支持  

## 构建流程

```
编译模块 (cargo build)
    ↓
提取静态库中的对象文件 (rust-ar)
    ↓
链接成可重定位ELF (ld -r)
    ↓
验证 .ko 文件
    ↓
清理临时文件
```

## 使用方法

### 基本命令

```bash
# 编译所有模块
make

# 编译特定模块
make hello

# 列出可用的模块
make list-modules

# 显示帮助
make help

# 显示配置
make show-config

# 清理构建产物
make clean

# 完整重建
make rebuild
```

### 高级用法

```bash
# 为不同架构构建
make TARGET=riscv64gc-unknown-none-elf

# 使用自定义模块路径
make MODULE_PATHS=custom_modules

# 指定自定义链接脚本
make LINKER_SCRIPT=custom.ld

# 组合参数
make hello TARGET=riscv64gc-unknown-none-elf LINKER_SCRIPT=custom.ld
```

## 变量说明

| 变量            | 默认值                         | 说明                 |
| --------------- | ------------------------------ | -------------------- |
| `TARGET`        | `x86_64-unknown-none`          | Rust编译目标三元组   |
| `MODULE_PATHS`  | `modules`                      | 模块源代码所在目录   |
| `LINKER_SCRIPT` | `linker.ld`                    | 链接脚本路径         |
| `LD_COMMAND`    | `ld` 或 `riscv64-linux-gnu-ld` | 链接器命令(自动选择) |
| `BUILD_DIR`     | `target`                       | 构建输出目录         |

## 目标(Targets)说明

### 主要目标

- **all** - 编译所有模块(默认)
- **modules** - 编译所有可用模块
- **<module_name>** - 编译指定模块

### 信息目标

- **list-modules** - 列出可用的模块
- **show-config** - 显示当前配置
- **help** - 显示帮助信息

### 清理目标

- **clean** - 删除所有构建产物
- **rebuild** - 完整重建(clean + all)

### 内部目标(不直接调用)

- **process-module-library** - 处理模块库(提取对象文件)
- **create-kernel-module** - 创建.ko文件(链接对象文件)
- **verify-kernel-module** - 验证.ko文件

## 架构支持

### 自动链接器选择

| 目标架构 | 链接器命令             |
| -------- | ---------------------- |
| x86_64   | `ld`                   |
| RISC-V   | `riscv64-linux-gnu-ld` |
| ARM      | `ld`                   |
| 其他     | `ld`                   |

## 输出示例

```
$ make hello
Building module: hello
   Compiling hello v0.1.0
    Finished `release` profile [optimized] target(s) in 2.34s
make[1]: Entering directory '/path/to/kmod'
Processing module library: hello
Extracting object files from /path/to/libhello.a
Found object files: 34 files
make[2]: Entering directory '/path/to/kmod'
Linking kernel module hello
Linking with 34 object files
Successfully created kernel module: target/hello/hello.ko
Module size: 616760 bytes
```

## 文件结构

生成的.ko文件位置：
```
target/
└── <module_name>/
    └── <module_name>.ko
```

构建过程使用的临时目录：
```
target/
└── <module_name>/
    ├── *.o  (构建过程中产生，完成后删除)
    └── <module_name>.ko  (最终产物)
```

## 与Rust builder的对应关系

| 功能     | Rust builder               | Makefile                             |
| -------- | -------------------------- | ------------------------------------ |
| 列出模块 | `list_modules()`           | `list-modules` 目标 + shell globbing |
| 构建模块 | `build_modules()`          | `$(MODULES)` 目标                    |
| 处理库   | `process_module_library()` | `process-module-library` 目标        |
| 创建.ko  | `create_kernel_module()`   | `create-kernel-module` 目标          |
| 验证     | `verify_kernel_module()`   | `verify-kernel-module` 目标          |
| 架构检测 | `target_ld()`              | `ifeq` 条件判断                      |

## 常见问题

### Q: 如何只构建特定模块而不构建所有模块？
A: 使用 `make <module_name>`，例如 `make hello`

### Q: 如何修改输出目录？
A: 修改Makefile中的 `BUILD_DIR` 变量，或在命令行中指定：`make BUILD_DIR=custom_output`

### Q: 如何使用自定义的链接脚本？
A: 使用 `LINKER_SCRIPT` 变量：`make LINKER_SCRIPT=my_linker.ld`

### Q: 为什么构建RISC-V模块失败？
A: 确保安装了 `riscv64-linux-gnu-ld` 链接器，或修改Makefile中的链接器选择逻辑

## 与ELF解析器集成

生成的.ko文件可以使用kmod-loader中的ELF解析器验证：

```bash
# 解析并显示.ko文件的详细信息
cargo run --example parse_elf -- target/hello/hello.ko
```

## 注意事项

1. **cargo必须安装** - Makefile依赖cargo编译模块
2. **rust-ar工具** - 用于从Rust生成的.a文件提取对象文件
3. **链接器** - 根据目标架构可能需要特定的链接器
4. **linker.ld文件** - 链接脚本必须存在于项目根目录
5. **权限** - 确保对target目录有读写权限

## 后续改进
- [ ] 添加更多架构自动检测
