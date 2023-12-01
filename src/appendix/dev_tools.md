# 附录 D：一些有用开发工具

在此附录中，咱们会讲到 Rust 项目所提供的一些有用的开发工具。咱们将看看自动格式化、应用警告修复的一些快速方法、一种代码静态分析工具，a linter，以及与多种 IDE 的集成。


## 使用 `rustfmt` 的自动格式化

**Automatic Formatting with `rustfmt`**


`rustfmt` 工具会依据社区编码风格，重新格式化咱们的代码。许多协作项目，都使用了 `rustfmt` 来防止有关编写 Rust 时使用何种风格方面的争论：每个人都使用这个工具来格式化他们的代码。

要安装 `rustfmt`，请键入下面的命令：

```console
$ rustup component add rustfmt
```

如同 Rust 会同时给到 `rustc` 与 `cargo` 一样，此命令会给到咱们 `rustfmt` 与 `cargo-fmt`。要格式化任何 Cargo 项目，请敲入下面的命令：

```console
$ cargo fmt
```

运行此命令，会重新格式化当前代码箱中全部的 Rust 代码。这只会改变编码风格，而不会改变代码语义。关于 `rustfmt` 的更多信息，请参阅 [其文档](https://github.com/rust-lang/rustfmt).


## 使用 `rustfix` 修复咱们的代码

**Fix Your Code with `rustfix`**


`rustfix` 工具已被 Rust 安装所包含，并可大致以咱们想要的方式，修复那些有着明确纠正问题方法的一些编译器告警。咱们之前大概率已经见到过编译器告警了。比如，设想有下面这段代码：

文件名：`src/main.rs`

```rust
fn do_something() {}

fn main() {
    for i in 0..100 {
        do_something();
    }
}
```

此处，咱们正调用 `do_something` 函数 100 次，但咱们在 `for` 循环的代码体中，从未用到那个变量 `i`。Rust 就会就此对咱们发出告警：

```console
$ cargo build
   Compiling rustfix_demo v0.1.0 (/home/lenny.peng/rust-lang-zh_CN/rustfix_demo)
warning: unused variable: `i`
 --> src/main.rs:4:9
  |
4 |     for i in 0..100 {
  |         ^ help: if this is intentional, prefix it with an underscore: `_i`
  |
  = note: `#[warn(unused_variables)]` on by default

warning: `rustfix_demo` (bin "rustfix_demo") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.29s
```

这个告警建议咱们要使用 `_i` 做名字：其中的下划线表示咱们有意不使用这个变量。通过运行 `cargo fix` 命令，咱们就可以使用 `rustfix`，自动应用那项建议：

```console
$ cargo fix --allow-no-vcs
    Checking rustfix_demo v0.1.0 (/home/lenny.peng/rust-lang-zh_CN/rustfix_demo)
       Fixed src/main.rs (1 fix)
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
```

当咱们再次看到 `src/main.rs`，就将发现 `cargo fix` 已修改了这段代码：

文件名：`src/main.rs`

```rust
fn do_something() {}

fn main() {
    for _i in 0..100 {
        do_something();
    }
}
```

那个 `for` 循环变量，现在就被命名为了 `_i`，同时那条告警也不再出现了。

咱们还可使用 `cargo fix` 命令，将咱们的代码在不同 Rust 版本之间转换。有关这些 Rust 版本，在附录 E 中有讲到。


## 使用 Clippy 获得更多的代码静态分析

**More Lints with Clippy**

Clippy 工具是用于分析咱们代码，从而咱们可以捕获到一些常见错误，而改进咱们 Rust 代码的一套代码静态分析集合。

要安装 Clippy，请输入以下命令：

```console
$ rustup component add Clippy
```

在任何 Cargo 项目上要运行 Clippy 的静态分析，请输入以下命令：

```console
$ cargo clippy
```

比如说咱们编写了像下面这个程序这样，用到某个数学常量近似值，好比说 `pi`，的一个程序：

文件名：`src/main.rs`

```rust
fn main() {
    let x = 3.1415;
    let r = 8.0;
    println!("圆的面积为 {}", x * r * r);
}
```

在这个项目上运行 `cargo clippy` 就会得到下面的报错：

```console
$ cargo clippy
    Checking clippy_demo v0.1.0 (/home/lenny.peng/rust-lang-zh_CN/clippy_demo)
error: approximate value of `f{32, 64}::consts::PI` found
 --> src/main.rs:2:13
  |
2 |     let x = 3.1415;
  |             ^^^^^^
  |
  = help: consider using the constant directly
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#approx_constant
  = note: `#[deny(clippy::approx_constant)]` on by default

error: could not compile `clippy_demo` due to previous error
```

此报错让咱们明白，Rust 已经定义了一个更精确的 `PI` 常量，且当咱们使用这个常量时，咱们的程序将更为正确。那么咱们随后就应修改咱们代码为使用这个 `PI` 常量。下面的代码就捕获导致 Clippy 的任何错误或告警：

文件名：`src/main.rs`

```rust
fn main() {
    let x = std::f64::consts::PI;
    let r = 8.0;
    println!("圆的面积为 {}", x * r * r);
}
```

有关 Clippy 的更多信息，请参阅 [其文档](https://github.com/rust-lang/rust-clippy)。


## 用到 `rust-analyzer` 的 IDE 集成

**IDE Integration Using `rust-analyzer`**


为帮助 IDE 集成，Rust 社区建议使用 [`rust-analyzer`](https://rust-analyzer.github.io/)。此工具是一套以编译器为中心，操 [语言服务器协议，Language Server Protocol](http://langserver.org/) 的实用工具；而所谓语言服务器协议，则是用于各种 IDEs 和编程语言，二者相互之间通信的一种规格。有多种不同客户端可使用 `rust-analyzer`，比如 [Visual Studio Code 的 Rust 分析器插件](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)。

请访问 `rust-analyzer` 项目 [主页](https://rust-analyzer.github.io/)，了解其安全说明，随后在咱们的特定 IDE 中安装该语言的服务器支持。咱们的 IDE 就能获得诸如自动补全、跳至定义及行内报错等能力。
