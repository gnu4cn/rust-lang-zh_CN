# 使用 `cargo install` 安装二进制代码箱

**Installing Binaries with `cargo install`**


`cargo install` 命令允许咱们在本地安装和使用二进制的代码箱。这并不是要取代系统包，system packages；它的目的是为 Rust 开发者提供一种方便的方式来安装别人在 [crates.io](https://crates.io) 上分享的工具。请注意咱们只能安装有二进制目标的包，packages that have binary targets。所谓 *二进制目标，binary target*，即与本身为非可运行，而适合于在其他程序中包含的库目标，a libary target，相反，因为代码箱有着一个 `src/main.rs` 文件，或有着被指定为二进制的另一文件时，而创建出的可以运行的程序。通常，代码箱会在 `README` 文件中，有着关于其是否为库代码箱，还是有着二进制目标，或二者皆具方面的信息。

使用 `cargo install` 安装的全部二进制程序文件，都被存储在安装根的 `bin` 文件中，in the installation root's `bin` folder。在使用 `rustup.rs` 安装 Rust，且没做任何定制配置时，这个目录将是 `$HOME/.cargo/bin`。为能运行咱们使用 `cargo install` 安装的程序，就要确保那个目录在 `$PATH` 中。

> 注：可在任意位置运行 `cargo install` 命令，来安装 Crates.io 上的 Rust 二进制程序，这些程序都将被安装在 `$HOME/.cargo/bin` 下。若已安装了某个 Rust 程序后再安装他，那么就会有如下输出：

```console
$ cargo install ripgrep                                                                                              1m 4s lennyp@vm-manjaro
    Updating crates.io index
     Ignored package `ripgrep v13.0.0` is already installed, use --force to override
```

比如，咱们曾在第 12 章中提到，有个用于搜索文件，`grep` 工具的 Rust 实现 `ripgrep`。要安装 `ripgrep`，咱们可运行如下命令：

```console
$ cargo install ripgrep
    Updating crates.io index
  Installing ripgrep v13.0.0
   Compiling memchr v2.5.0
   Compiling cfg-if v1.0.0
   Compiling libc v0.2.137
   Compiling log v0.4.17
   Compiling proc-macro2 v1.0.47
   Compiling lazy_static v1.4.0
   Compiling regex-automata v0.1.10
   Compiling quote v1.0.21
   Compiling unicode-ident v1.0.5
   Compiling bstr v0.2.17
   Compiling syn v1.0.103
   Compiling aho-corasick v0.7.20
   Compiling regex-syntax v0.6.28
   Compiling serde_derive v1.0.147
   Compiling encoding_rs v0.8.31
   Compiling serde v1.0.147
   Compiling regex v1.7.0
   Compiling grep-matcher v0.1.5
   Compiling serde_json v1.0.89
   Compiling unicode-width v0.1.10
   Compiling fnv v1.0.7
   Compiling same-file v1.0.6
   Compiling once_cell v1.16.0
   Compiling thread_local v1.1.4
   Compiling globset v0.4.9
   Compiling textwrap v0.11.0
   Compiling encoding_rs_io v0.1.7
   Compiling memmap2 v0.5.8
   Compiling bitflags v1.3.2
   Compiling crossbeam-utils v0.8.14
   Compiling bytecount v0.6.3
   Compiling itoa v1.0.4
   Compiling ryu v1.0.11
   Compiling strsim v0.8.0
   Compiling termcolor v1.1.3
   Compiling clap v2.34.0
   Compiling grep-searcher v0.1.10
   Compiling atty v0.2.14
   Compiling base64 v0.13.1
   Compiling grep-printer v0.1.6
   Compiling grep-cli v0.1.6
   Compiling grep-regex v0.1.10
   Compiling ripgrep v13.0.0
   Compiling walkdir v2.3.2
   Compiling ignore v0.4.18
   Compiling grep v0.2.10
   Compiling num_cpus v1.14.0
    Finished release [optimized + debuginfo] target(s) in 1m 09s
  Installing /home/lennyp/.cargo/bin/rg
   Installed package `ripgrep v13.0.0` (executable `rg`)
```

输出的倒数第二行显示出已安装二进制程序的位置与名字，在这个示例中名字便是 `rg`。正如前面提到的，只要安装目录是在 `$PATH` 中，随后咱们就可以运行 `rg --help`，并启动一个用于检索文件的更快、更具 Rust 风格的工具了！
