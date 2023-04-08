# 入门

现在就开始 Rust 之旅！有很多要掌握的东西，不过千里之行，始于足下。本章将讨论：

- 在 Linux、macOS 及 Windows 上安装 Rust;
- 编写一个打印出 `Hello, world!` 的程序来；
- Rust 的包管理器和构建系统 Cargo 的使用。


## <a id="installation"></a> 安装

第一步即是安装 Rust。这里将通过 `rustup` 这个用于管理 Rust 版本及相关工具的命令行工具，来下载 Rust。要下载 Rust，就需要互联网连接。

> 注意：若由于某些原因而不愿使用 `rustup`，那么请参考 [其他 Rust 安装方式页面](https://forge.rust-lang.org/infra/other-installation-methods.html) 了解更多选项。

接下来就是要按照最新的稳定版 Rust 编译器。Rust 的稳定性保证了本书中所有示例都将在较新的 Rust 版本下可持续编译。由于 Rust 经常会改进错误消息和告警，因此在不同版本之间，输出可能会略有不同。也就是说，任何使用以下步骤所安装的较新、稳定版 Rust，都将如本书内容中所期望的那样工作。

> 关于**命令行注释**
> 在本章及全书中，都会给出一些在终端中用到的命令。他们是一些应在以 `$` 开始的终端中输入的行。至于这个 `$` 字符，是无需输入的；这个字符表示每条命令的开头。那些不以 `$` 开头的行，通常给出的是上一命令的输出。此外，那些特定于 `PowerShell` 的示例中，将使用 `>` 而不是 `$`。


### 在 Linux 与 macOS 上安装 `rustup`

若使用的是 Linux 或 macOS，那么请打开一个终端，然后输入下面的命令：

```console
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

此命令会下载一个脚本并开始 `rustup` 工具的安装，而 `rustup` 将安装最新的稳定版 Rust。可能会提示输入 `sudo` 密码。在安装成功后，就会出现下面这行！

```console
Rust is isntalled now. Great!
```

这里还将需要一个连接器（linker），这是个Rust要用来将其编译好的输出，组合起来形成一个文件的程序。似乎你的电脑上以及有了一个这样的连接器了。若收到连接器错误信息，那么就应安装一个 C 语言编译器，C 编译器通常会包含着连接器的。由于一些常用 Rust 包对 C 代码有依赖且需要 C 编译器，因此 C 编译器也是有用的。

在 macOS 上，可通过运行下面的命令，获取到一个 C 编译器：

```console
$ xcode-select --install
```

Linux 用户一般都会安装 GCC 或 Clang，至于具体哪种 C 编译器，则是依据他们所用 Linux 分发版本的文档可以确定。比如若使用的是 Ubuntu，那么就可以安装 `build-essential` 软件包。


### 在 Windows 上安装 `rustup`

在 Windows 上，请前往 [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) 页面，并按照安装 Rust 的指令进行安装。在安装过程的某个时刻，将收到为何需要 Visual Studio 2013 或更新版本的 C++ 构建工具的说明。而最简单的获取到构建工具的方法，则是安装 [Visual Studio 2019 构建工具](https://visualstudio.microsoft.com/visual-cpp-build-tools/)。在询问将要安装何种工作负载（workloads）时，请确保 `C++ build tolls` 被选中，还要确保包含 Windows 10 SDK 及英语语言包。

本书接下来用到的命令，在 `cmd.exe` 与 `PowerShell` 中都可工作。若其中有特定区别，本书将会解释要用哪个。

## 更新与卸载

在通过 `rustup` 安装了 Rust 后，更新到最新版本就容易了。在 `shell` 中运行下面的更新脚本：

```console
$ rustup update
```

而要卸载 Rust 和 `rustup`，只需在 `shell` 中运行下面的卸载脚本：

```java
$ rustup self uninstall
```

## <a id="troubleshooting"></a> 故障排除

要检查当前是否安装了 Rust, 请开启一个 `shell` 并敲入这行命令：

```console
$ rustc --version
```

就会看到版本编号、合并哈希（`commit` hash），以及已发布的该最新稳定版本合并日期，以下面这种格式：

```console
rustc x.y.z (abcabcadc yyyy-mm-dd)
```

若看到这个信息，那么就已成功安装了 Rust！若看不到这个信息，且是在 Windows 上，那么就请在 `%PATH%` 系统变量中检查一下 Rust 在不在里面。若那一点问题都没有而 Rust 仍就不工作，那么可在数个地方需求帮助。其中最便利的就是 [Rust 官方 Discord](https://discord.gg/rust-lang) 上的 `#beginners` 频道了。在那里可与其他 Rust 公民（一种无厘头的自我称呼）聊天，他们可以帮助到你。其他不错的资源包括 [用户论坛](https://users.rust-lang.org/) 和 [Stack Overflow](https://stackoverflow.com/questions/tagged/rust)。

## 本地文档

Rust 的安装，也包含了一份本地文档，因此可离线阅读到这本地文档。运行 `rustup doc` 即可在浏览器中打开这本地文档。

在任何时候遇到标准库所提供的类型或函数，而又确定他做些什么或该怎样使用这类型或函数时，就可以使用 API 文档来搞明白他是怎么回事！

## `Hello, World!`

既然已经安装好了 Rust, 那么就来编写第一个 Rust 程序吧。在掌握一门新语言时，传统就是要编写一个小的、打印出文字 `Hello, World!` 到屏幕上的程序，因此这里也会干这同样的事情！

> 注意：本书假定读者对命令行有着基本的熟悉。Rust 对代码在何处编辑和使用何种工具编辑没有特别要求，因此若优先选择某种集成开发环境，而非命令行，那么使用喜好的 IDE 即可。许多 IDE 都有某种程度的 Rust 支持；请查看 IDE 文档了解有关细节信息。近来，Rust 团队已着手启动良好的IDE支持，且此方面已取得极大进展！

### 创建一个项目目录

这里是以构造一个保存 Rust 代码的目录开始的。对于 Rust 来说，代码位居何处并不重要，不过对于本书中的练习与项目，是建议在主目录下构造一个 `projects` 目录，并把全部项目放在那里的。

请打开一个终端，并输入下面的这些命令来构造一个 `projects` 的目录，和一个在 `projects` 下用于 "Hello, World!" 项目的目录。

对于 Linux、macOS 和 Windows 上的 `PowerShell`, 请输入：

```console
$ mkdir ~/rust-lang/projects
$ cd ~/rust-lang/projects
$ mkdir hello_world
$ cd hello_world
```

而对于 Windows 的 CMD， 请输入：

```console
> mkdir "%USERPROFILE%\rust-lang\projects"
> cd /d "%USERPROFILE%\rust-lang\projects"
> mkdir hello_world
> cd hello_world
```

### <a id="writing-and-running-a-rust-program"></a> 编写及运行 Rust 程序

接下来，就要构造一个源代码文件，并命名为 `main.rs`。Rust 文件总是以 `.rs` 扩展名结束。若要在文件名中是一多个单词，那么请使用下划线来将这些单词隔开。比如，请使用 `hello_world.rs` 而不是 `helloworld.rs`。

现在就要打开这个刚创建出的 `main.rs` 文件，并敲入清单 1-1 中的代码。

文件名：`main.rs`

```rust
fn main() {
    println!("Hello, World!");
}
```

*清单 1-1：打印`Hello, World!` 的程序*

保存这个文件并回到终端窗口。在 Linux 或 macOS 上，请输入下面的命令来编译和运行这个文件：

```console
$ rustc main.rs
$ ./main
Hello, World!
```

在 Windows 上，就要输入命令 `.\main.exe` 而不是 `./main`：

```console
> rustc main.rs
> .\main.exe
Hello, World!
```

而不论所在操作系统为何，字符串 `Hello, World!` 都应打印到终端。而若没有看到这个输出，那么请回到安装小节的 [“故障排除”](#troubleshooting) 部分获取帮助。

如确实打印出了 `Hello, World!`，那么恭喜你！你已正式编写除了一个 Rust 程序了。那就让你成为了一名 Rust 程序员了 -- 欢迎！

### Rust 程序解析

来仔细回顾一下刚才在 “Hello World！” 程序中发生了什么。这是谜团中第一部分：

```rust
fn main() {

}
```

这些行定义了 Rust 中的一个函数。这个 `main` 函数比较特殊：在每个可执行的 Rust 程序中，他总是第一个开始运行的代码。这第一行声明了一个名为 `main` 的、没有参数且不返回任何值的参数。若函数有参数，那么参数就应位处圆括号`()`内部。

还有就是，请注意函数体是包裹在花括号`{}`中的。Rust 要求将全部函数体都用花括号包裹起来。将开头的花括号与函数声明放在同一行，并在二者之间加上一个空格，是良好的代码风格。

若想要在多个 Rust 项目之间保持一种标准的编码风格，那么就可以使用一个名为 `rustfmt` 的自动格式化工具，来以一种特定样式对代码进行格式化。与 `rustc` 一样，Rust 团队已将此工具包含在标准的 Rust 发布中，因此在你的电脑上就应该已经有了这个格式化工具了！请查看在线文档了解更多详情。

在这个`main` 函数里头，是下面的代码：

```rust
println!("Hello, World!");
```

这行代码完成了此小程序的全部工作：他将文字打印到屏幕。这里有四个需要注意的重要细节。

首先，Rust 编码风格是缩进四个空格，而非一个制表符；

其次，`println!` 调用了一个 Rust 的宏（a Rust macro）。若他调用的是个函数，那么就应输入 `println` （是不带 `!` 的）。在后续的第 19 章，将详细讨论 Rust 的宏。而现在，则只需知道 `!` 的使用表示是在调用某个宏而不是普通函数，同时宏不会总是遵循与函数同样的规则；

第三，就是看到的 `Hello, World!` 这个字符串了。这里时将此字符串作为参数，传递给 `println!` 的，且这个字符串是被打印到屏幕上的；

最后，这行语句是以分号（`;`）结束的，这表示该表达式结束，同时下一表达式已准备好开始。Rust 代码的多数行，都是以分号结束的。


### 编译和运行是分开的步骤

这里刚刚运行了一个新近创建出的程序，那么来检视一下该过程的每个步骤。

在运行某个 Rust 程序之前，必须要通过敲入 `rustc` 命令并将源代码文件名字，作为`rustc`的参数加以传入，这样来使用 Rust 编译器对其进行编译，像下面这样：

```console
$ rustc main.rs
```

若你有 C 或 C++ 的背景知识，那么就会注意到这与 `gcc` 或 `clang` 类似。在成功编译后，Rust 就会输出一个二进制可执行文件。

在 Linux、macOS 和 Windows 上的 PowerShell 之上，就可以通过在 `shell` 中敲入 `ls` 看到这个可执行文件。在 Linux 与 macOS 上，将看到下面这两个文件。而在 Windows 上的 PowerShell 中，则会看到与使用 CMD 一样的以下三个文件。

```console
$ ls
main    main.rs
```

在 Windows 的 CMD 中，就应输入下面的东西：

```console
> dir /B %= 这里的 /B 选项表示只显示文件名 =%
main.exe
main.pdb
main.rs
```

这显示了带有 `.rs` 扩展名的源代码文件、那个可执行文件（Windows 上的 `main.exe`，对于其他平台则是 `main`），以及，在使用 Windows 时，一个包含了调试信息的、带有 `.pdb` 扩展名的文件。从此处，就像下面这样来运行这里的 `main` 或 `main.exe`：

```console
$ ./main # 或在 Windows 上的 .\main.exe
```

若这里的 `main.rs` 就是那个 “Hello, World!” 程序，那么这行命令就会将 `Hello, World!` 打印到你的终端了。

若你对某门动态语言，诸如 Ruby、Python 或者 JavaScript 更为熟悉，那么可能就不习惯于将编译和运行某个程序作为分开的步骤。Rust 是门 *提前编译* 语言（an *ahead-of-time compiled* language），这意味着可对程序进行编译，而将可执行文件交给他人，他们可在未安装 Rust 的情况下运行编译好的可执行文件。而若将某个 `.rb`、`.py`，或者 `.js` 文件交给某人时，他们就需要安装好相应的 Ruby、Python 或 JavaScript 实现。不过在这些语言中，仅需一个命令来编译和运行他们的程序。在编程语言设计中，每件事都有所取舍。

对于简单的程序来说，用 `rustc` 编译就足够了，但随着项目的成长，就希望对所有选项进行管理，并令到代码分享更为简便。接下来，就要介绍 Cargo 工具了，这工具将帮助我们编写出实用的 Rust 程序。


## 你好，Cargo！

Cargo 是 Rust 的构建系统和包管理器。由于 Cargo 处理了很多任务，诸如构建代码、下载代码所依赖的库，以及这些库的构建等等，因此绝大多数 Rust 公民都使用这个工具，来管理他们的 Rust 项目。（我们把这些库叫做代码需要依赖（we call the libraries that your code needs *dependencies*）。）

对于最简单的那些 Rust 程序，比如才写的那个，是没有任何依赖的。因此若使用 Cargo 来构建这个 `Hello, World!` 项目，那么就只会用到 Cargo 处理代码构建的部分。而随着更为复杂 Rust 程序的编写，就会添加依赖，而在开始一个用到 Cargo 的项目时，完成依赖添加就会容易得多。

由于广大 Rust 项目都用到了 Cargo，本书其余部分就假定了也使用 Cargo。若使用了在 [安装](#installation) 小节中提到的官方安装器进行的 Rust 安装，那么Cargo就已与 Rust 一起安装好了。而若是以其他方式安装的 Rust，那么就要通过在终端中敲入下面的命令，来检查 Cargo 是否已安装妥当：

```console
$ cargo --version
```

若能看到版本号，那么就有了这个工具！而若看到错误，诸如 `command not found`，就请查看你的安装方式的文档，找到怎样单独安装 Cargo 的方法。


### 使用 Cargo 创建项目

下面来使用 Cargo 创建一个新项目，并看看与原先的 “Hello, World!” 项目有何不同。现在导航至 `projects` 目录（或确定下来的保存代码的其他地方）。然后不论在那个操作系统之上，运行下面的命令：

```console
$ cargo new hello_cargo
$ cd hello_cargo
```

这第一个命令创建出了一个新的名为 `hello_cargo` 目录。这里就已将项目命名为了 `hello_cargo`，然后 Cargo 将其文件创建在了同名的目录里面。

进入到 `hello_cargo` 目录并列出那些文件。就会看到 Cargo 已经为我们生成了两个文件和一个目录：一个 `Cargo.toml`文件与一个里头有着 `main.rs` 文件的 `src` 目录。

`cargo new` 还初始化了一个新的、带有 `.gitignore` 文件的 Git 代码仓库。若是在一个既有的 Git 代码仓库运行的 `cargo new`，那么就不会生成那些 Git 文件；通过运用 `cargo new --vcs=git` 可重写此行为。

> 注意：Git 是种常用的版本控制系统。可通过上面的 `--vcs` 命令行参数，让 `cargo new` 使用其他版本控制系统或不使用版本控制系统。请运行 `cargo new --help`命令来查看所有可用选项。


文件名：`Cargo.toml`

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = '2021'

[dependencies]
```

*清单 1-2：由 `cargo new` 所生成的 `Cargo.toml` 的内容*

该文件是 [TOML](https://toml.io/) （ *Tom's Obvious, Minimal Language* ） 格式的，这是 Cargo 的配置格式。

该文件的第一行， `[package]`，是个小节标题，表示接下来的语句是在对一个包进行配置。随着往这个文件添加越来越多的信息，就会添加其他小节。

接下来的三行，对 Cargo 用于编译程序所需的信息进行了配置：项目名称、版本号及要使用的 Rust 版本。在 [附录 E](Appendix_E.md) 中会讲到这个 `edition` 关键字。

`Cargo.toml` 的最后一行，`[dependencies]`，是要列出项目全部依赖小节开始的地方。在 Rust 中，代码包被称为 *包裹（crates）*。此项目无需任何其他包裹，在第 2 章中的头一个项目，就会用到依赖包裹，因此在那时就会用到这个依赖小节。

现在打开 `src/main.rs` 然后看看：

文件名：`src/main.rs`

```rust
fn main() {
    println! ("Hello, World!");
}
```

Cargo 以及为我们生成了一个 “Hello, World!” 的程序，这个自动生成的程序就跟之前在清单 1-1 中的一样！到现在，先前的项目与这个 Cargo 生成的项目的不同之处，就是 Cargo 是将代码放在那个 `src` 目录中的，同时在顶层目录还有了一个 `Cargo.toml` 配置文件。

Cargo 希望那些源代码文件，存留在 `src` 目录里头。而顶层的项目目录，只用于 `README` 文件、许可证信息、配置文件及其他与代码无关的东西。使用 Cargo 有助于对项目的组织。一切都有了个地方，且一切都在各自的地方（there's a place for everything, and everything is in its place）。

若没有使用 Cargo 来开始项目，就如同先前在 “Hello, World!” 项目中所做那样，那么仍旧可使用 Cargo 将其转换为一个项目。将项目代码移入到 `src` 目录并创建出一个适当的 `Cargo.toml` 文件来:

```console
$ cd hello_world
$ mkdir src
$ mv main.rs src/
$ cargo init
```

### 构建和运行一个 Cargo 项目

现在来看看在使用 Cargo 来构建和运行那个 “Hello, World!” 程序有什么不同之处！在 `hello_cargo` 目录，通过敲入下面的命令，来构建该项目：

```console
$ cargo build                                                                   ✔ 
   Compiling hello_cargo v0.1.0 (/home/peng/rust-lang/projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.45s
```

此命令创建出在 `target/debug/hello_cargo`（或 Windows 上的`target\debug\hello_cargo.exe`）中，而非当前目录下的一个可执行文件。可使用下面这个命令运行那个可执行程序：

```console
$ ./target/debug/hello_cargo # 或者在 Windows 上的 .\target\debug\hello_cargo.exe
Hello, world!
```

若一切顺利，那么 `Hello, World!` 就会被打印到终端。首次运行 `cargo build`，还会造成 Cargo 在顶层目录创建一个新文件：`Cargo.lock`。该文件会跟踪项目中各个依赖的精确版本。由于这个项目没有依赖，因此该文件有些稀疏。绝无必要手动修改此文件；Cargo 会为我们管理他的内容。

文件名：`Cargo.lock`

```toml
# This file is automatically @generated by Cargo.
# It is not intended for manual editing.
version = 3

[[package]]
name = "hello_cargo"
version = "0.1.0"
```

*清单 1-3, `Cargo.lock`*

这里刚刚使用 `cargo build` 构建了一个项目，并用 `./target/debug/hello_cargo` 运行了这个项目，不过这里还可以将代码编译和运行编译结果，全部在一个命令，`cargo run`，中完成：

```console
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, World!
```

请注意这次并未见到表示 Cargo 曾在编译 `hello_cargo` 的输出。Cargo 发现这些文件并未发生改变，因此他就运行了那个二进制文件。若曾修改过源代码，那么 Cargo 就会在运行这个项目之前，重新构建该项目，从而会看到这样的输出：

```console
$ cargo run                                                                                        ✔ 
   Compiling hello_cargo v0.1.0 (/home/peng/rust-lang/projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/hello_cargo`
Hello, Cargo!
```

Cargo 还提供了一个叫做 `cargo check` 的命令。此命令会对代码进行快速检查，以确保代码可被编译，但该命令不会产生出可执行程序：

```console
$ cargo check                                                                                      ✔ 
   Checking hello_cargo v0.1.0 (/home/peng/rust-lang/projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.35s
```

这里为何不要一个可执行文件呢？通常，由于 `cargo check` 跳过了产生出可执行程序的步骤，因此他要比 `cargo build` 快得多。但在编写代码时，要持续检查已完成的工作时，那么 `cargo check` 的使用，就会加速工作流程！由于这个原因，许多的 Rust 公民，都会在编写他们的程序时，定期运行 `cargo check`，来确保程序通过编译。而在准备好使用可执行文件的时候，在运行 `cargo build`。

来概括一下到现在，已经掌握的有关 Cargo 的内容：

- 使用 `cargo new` 就可以创建出项目；
- 使用 `cargo build` 就可以构建出项目；
- 使用 `cargo run` 就可以一步完成项目的构建和运行；
- 使用 `cargo check`就可以在不产生出二进制程序的情况下，对项目加以构建以进行错误检查；
- Cargo 是将构建结果保存在 `target/debug` 目录，而不是保存在与源代码同样的目录。

使用 Cargo 的一个额外优势，就是不论是在何种操作系统上工作，那些命令都是同样的。基于这个原因，本书后续就不再提供针对 Linux 与 macOS，以及Windows 的特别说明了。

### 发布目的的构建

在项目最终准备好发布时，就可以使用 `cargo build --release` 来带优化地对其进行编译了。该命令将创建出一个位于 `target/release`，而非 `target/debug` 中的可执行文件。其中的那些优化，会令到项目的 Rust 代码运行得更快，不过开启这些优化，将增加程序编译的时间。这就是为什么有两种不同配置文件的原因：一个配置是为开发目的，在希望快速且频繁地对项目进行重新构建时使用的配置，而另一个，则是为构建要给到用户的、不会反复重新构建的、将尽可能快速运行的最终程序所用到的配置。在要对程序进行性能测试时，就一定要运行 `cargo build --release`，并对 `target/release` 中的可执行程序进行性能测试。

### 约定俗成的 Cargo

对于那些简单项目，相比于使用 `rustc`，Cargo 并未提供到很多价值，然而在程序变得愈加错综复杂时，他就会证明他的价值了。对于那些由多个代码箱（crates） 构成的复杂项目，让 Cargo 来对构建进行协调，就要容易得多。

即使这个`hello_cargo` 项目如此，此刻也用到了将在接下来的 Rust 编程生涯中会用到的真正工具。事实上，对于在任何既有的 Rust 项目，都应使用下面这些命令，使用 Git 来检出代码，然后前往到项目目录，进而加以构建：

```console
$ git clone example.org/someproject
$ cd someproject
$ cargo build
```

更多有关 Cargo 的信息，请查看看[Cargo 文档](https://doc.rust-lang.org/cargo/)。
