# 参考信息

```c
typedef struct {
    Elf64_Word      st_name;
    unsigned char   st_info;
    unsigned char   st_other;
    Elf64_Half      st_shndx;
    Elf64_Addr      st_value;
    Elf64_Xword     st_size;
} Elf64_Sym;
```

- `st_name` - 符号名称在字符串表中的偏移
- `st_info` - 符号类型和绑定信息
  - bind（高4位）：决定链接时的处理方式
    - 0 - STB_LOCAL：局部符号，仅在定义它的文件中可见
    - 1 - STB_GLOBAL：全局符号，在所有文件中可见
    - 2 - STB_WEAK：弱符号，类似于全局符号，但可以被其他符号覆盖
  - type（低4位）：符号类型
    - STT_NOTYPE 未分类
    - STT_OBJECT 数据对象（变量）
    - STT_FUNC 函数
    - STT_SECTION 段符号
    - STT_FILE 文件符号
- `st_other` - 符号的其他信息(通常为0)
  - 低 2 位表示可见性
    - STV_DEFAULT：默认可见性
    - STV_HIDDEN：隐藏符号
    - STV_PROTECTED：受保护符号
  - 内核不会加载 STV_HIDDEN 的符号（它们不导出）
- `st_shndx` - 符号所在的节区索引
  - 特殊值：
    - SHN_UNDEF (0)：未定义符号
    - SHN_ABS (0xfff1)：绝对符号
    - SHN_COMMON (0xfff2)：公共符号,common 块（全局变量未初始化）
- `st_value` - 符号的值（地址）
  - 如果符号在某 section 中（定义符号），则为该符号在 section 中的偏移
  - 如果符号未定义，则为0
  - 如果是绝对符号（SHN_ABS），则为绝对地址
- `st_size` - 符号的大小（以字节为单位）用于调试、统计等目的