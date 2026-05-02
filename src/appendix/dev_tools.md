# 附录 D：有用的开发工具

在这个附录中，我们讨论 Rust 项目提供的一些有用的开发工具。我们将探讨自动格式化、快速修复告警的方法、代码静态分析工具，以及与 IDE 的集成。


## 通过 `rustfmt` 自动格式化

`rustfmt` 工具会依据社区编码风格，重新格式化咱们的代码。许多协作项目都使用 `rustfmt`，以防止在编写 Rust 时因风格选择而产生争议：每个人都使用这个工具来格式化自己的代码。

Rust 的安装默认包含 `rustfmt`，因此咱们的系统上应该已经安装了 `rustfmt` 和 `cargo-fmt` 这两个程序。这两个命令与 `rustc` 和 `cargo` 类似：`rustfmt` 允许更细粒度的控制，而 `cargo-fmt` 则能理解使用 Cargo 项目的约定。要格式化任何 Cargo 项目，请输入以下命令：

```console
$ cargo fmt
```

运行这个命令将重新格式化当前代码箱中的所有 Rust 代码。这只应改变代码的格式，不会改变代码的语义。有关 `rustfmt` 的更多信息，请参阅 [其文档](https://github.com/rust-lang/rustfmt).


## 通过 `rustfix` 修复代码

`rustfix` 工具包含在 Rust 安装中，能够修复那些有明确修正问题的方法的一些编译器告警，大致是咱们希望的。咱们可能之前就已见到过编译器告警。例如，设想下面这段代码：

文件名：`src/main.rs`

```rust
fn main() {
    let mut x = 42;
    println!("{x}");
}
```

在这里，我们定义变量 `x` 为可变的，但我们实际上从未真正改变他。Rust 会就此发出告警：

```console
$ cargo build
   Compiling myprogram v0.1.0 (/home/hector/rust-lang-zh_CN/projects/myprogram)
warning: variable does not need to be mutable
 --> src/main.rs:2:9
  |
2 |     let mut x = 42;
  |         ----^
  |         |
  |         help: remove this `mut`
  |
  = note: `#[warn(unused_mut)]` (part of `#[warn(unused)]`) on by default

```

这个告警建议我们移除 `mut` 关键字。我们可以通过运行 `cargo fix` 命令，利用 `rustfix` 工具自动应用这一建议：

```console
$ cargo fix --allow-no-vcs
    Checking myprogram v0.1.0 (/home/hector/rust-lang-zh_CN/projects/myprogram)
       Fixed src/main.rs (1 fix)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
```

> **译注**：之所以加上 `--allow-no-vcs` 命令行开关，是因为在不加时会报出如下错误。
>
> ```console
> $ cargo fix
> error: the working directory of this package has uncommitted changes, and `cargo fix` can potentially perform destructive changes; if you'd like to suppress this error pass `--allow-dirty`, or commit the changes to these files:
>
>   * projects/myprogram/ (dirty)
>   * src/SUMMARY.md (dirty)
>   * src/appendix/dev_tools.md (dirty)
> ```

当我们再次查看 `src/main.rs` 时，将发现 `cargo fix` 已修改了代码：

文件名：`projects/myprogram/src/main.rs`

```rust
fn main() {
    let x = 42;
    println!("{x}");
}
```

变量 `x` 现在是不可变的，并且告警也不再出现。

咱们还可以使用 `cargo fix` 命令在不同的 Rust 版本之间转换代码。版本在 [附录 E](./editions.md) 中得以介绍。


## Clippy 下的更多代码静态分析

[Clippy 工具](https://github.com/rust-lang/rust-clippy) 是一个用于分析代码的静态分析工具的集合，以便咱们可以发现常见错误，仅而改进 Rust 代码。

要对任何 Cargo 项目运行 Clippy 的静态分析工具，请输入以下命令：

```console
$ cargo clippy
```

例如，假设咱们编写了个程序，使用某个数学常数，比如 π 的近似值，就像下面这个程序所做的那样：

文件名：`src/main.rs`

```rust
fn main() {
    let x = 3.1415;
    let r = 8.0;
    println!("圆的面积为 {}", x * r * r);
}
```

对这个项目上运行 `cargo clippy` 会得到下面这个报错：

```console
$ cargo clippy
    Checking clippy_demo v0.1.0 (/home/hector/rust-lang-zh_CN/projects/clippy_demo)
error: approximate value of `f{32, 64}::consts::PI` found
 --> src/main.rs:2:13
  |
2 |     let x = 3.1415;
  |             ^^^^^^
  |
  = help: consider using the constant directly
  = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.93.0/index.html#approx_constant
  = note: `#[deny(clippy::approx_constant)]` on by default

error: could not compile `clippy_demo` (bin "clippy_demo") due to 1 previous error
```

这个报错让咱们知道，Rust 已经定义了个更精确的 `PI` 常量，并且若咱们使用这个常量时，程序将更为正确。因此，咱们就要修改代码为使用 `PI` 常量。

以下代码不会导致来自 Clippy 的任何报错或告警：

文件名：`projects/clippy_demo/src/main.rs`

```rust
use std::f64::consts;

fn main() {
    let x = consts::PI;
    let r = 8.0;
    println!("圆的面积为 {}", x * r * r);
}
```

有关 Clippy 的更多信息，请参阅 [其文档](https://github.com/rust-lang/rust-clippy)。


## 使用 `rust-analyzer` 的 IDE 集成

为了帮助 IDE 集成，Rust 社区建议使用 [`rust-analyzer`](https://rust-analyzer.github.io/)。这个工具是一组以编译器为中心的实用程序，支持 [语言服务器协议，Language Server Protocol](http://langserver.org/) ，该协议是 IDE 和编程语言之间相互通信的规范。不同客户端均可以使用 `rust-analyzer`，比如 [Visual Studio Code 的 Rust 分析器插件](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)。

请访问 `rust-analyzer` 项目 [主页](https://rust-analyzer.github.io/) 查看安全说明，随后在咱们的特定 IDE 中安装语言服务器支持。咱们的 IDE 将获得自动补全、跳转到定义以及内联报错等能力。

## 译注补充：`cargo-binutils`

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


