# 有用笔记

此处记录学习及应用 Rust 编程软件过程中，觉得有用的一些东西。


## `cargo-binutils`

[这个项目](https://github.com/rust-embedded/cargo-binutils) 是 Embbeded-Rust 项目的，而不是 Rust 官方的，但提供了有用的功能。比如查看构建出的二进制程序文件的那些头部：


```console
$ cargo readobj --bin clippy_demo -- --file-headers
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
ELF Header:
  Magic:   7f 45 4c 46 02 01 01 00 00 00 00 00 00 00 00 00
  Class:                             ELF64
  Data:                              2's complement, little endian
  Version:                           1 (current)
  OS/ABI:                            UNIX - System V
  ABI Version:                       0
  Type:                              DYN (Shared object file)
  Machine:                           Advanced Micro Devices X86-64
  Version:                           0x1
  Entry point address:               0x86D0
  Start of program headers:          64 (bytes into file)
  Start of section headers:          4305200 (bytes into file)
  Flags:                             0x0
  Size of this header:               64 (bytes)
  Size of program headers:           56 (bytes)
  Number of program headers:         12
  Size of section headers:           64 (bytes)
  Number of section headers:         42
  Section header string table index: 41
```

使用前需要进行如下安装：

```console
$ cargo install cargo-binutils
$ rustup component add llvm-tools-preview
```
