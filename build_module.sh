#!/bin/bash
# build_module.sh - 辅助脚本处理模块构建的复杂逻辑
# 用途：将复杂的shell命令从Makefile中提取出来，使Makefile更清晰

set -e

MODULE_NAME="$1"
TARGET="$2"
MODULE_BUILD_DIR="$3"
BUILD_DIR="$4"
LD_COMMAND="$5"
AR="rust-ar"

# 验证参数
if [ -z "$MODULE_NAME" ] || [ -z "$TARGET" ] || [ -z "$MODULE_BUILD_DIR" ]; then
    echo "Usage: $0 <module_name> <target> <module_build_dir> <build_dir> <ld_command>"
    exit 1
fi

BUILD_DIR="${BUILD_DIR:=target}"
LD_COMMAND="${LD_COMMAND:=ld}"

# 第一步：查找静态库
lib_path="${MODULE_BUILD_DIR}/lib${MODULE_NAME}.a"
if [ ! -f "$lib_path" ]; then
    echo "Error: Library not found: $lib_path"
    exit 1
fi

lib_path_abs="$(cd "$(dirname "$lib_path")" && pwd)/$(basename "$lib_path")"

# 第二步：创建临时目录并提取对象文件
module_temp_dir="${BUILD_DIR}/${MODULE_NAME}"
mkdir -p "$module_temp_dir"

echo "Extracting object files from $lib_path_abs"
cd "$module_temp_dir"
$AR x "$lib_path_abs" || {
    echo "Failed to extract from static library"
    exit 1
}
cd - > /dev/null

# 第三步：计数对象文件
obj_count=$(find "$module_temp_dir" -maxdepth 1 -name "*.o" 2>/dev/null | wc -l)
if [ "$obj_count" -eq 0 ]; then
    echo "Error: No object files found in $module_temp_dir"
    exit 1
fi
echo "Found object files: $obj_count files"

# 第四步：获取所有对象文件
obj_files=$(find "$module_temp_dir" -maxdepth 1 -name "*.o" -type f | tr '\n' ' ')

# 第五步：链接生成.ko文件
output_ko="${BUILD_DIR}/${MODULE_NAME}/${MODULE_NAME}.ko"
mkdir -p "$(dirname "$output_ko")"

echo "Linking kernel module $MODULE_NAME"
echo "Linking with $obj_count object files"

# 执行链接命令
$LD_COMMAND -r -T linker.ld -o "$output_ko" $obj_files --strip-debug --build-id=none
if [ $? -ne 0 ]; then
    echo "Failed to create kernel module for $MODULE_NAME"
    exit 1
fi

# 第六步：验证生成的文件
if [ -f "$output_ko" ]; then
    size=$(stat -c%s "$output_ko" 2>/dev/null || stat -f%z "$output_ko" 2>/dev/null)
    echo "Successfully created kernel module: $output_ko"
    echo "Module size: $size bytes"
else
    echo "Error: Kernel module file was not created"
    exit 1
fi

# 第七步：清理临时对象文件
echo "Cleaning up temporary object files"
rm -f "${module_temp_dir}"/*.o

exit 0
