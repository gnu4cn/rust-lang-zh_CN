# `Hello, World!`

既然已经安装好了 Rust, 那么就来编写第一个 Rust 程序吧。在掌握一门新语言时，传统就是要编写一个小的、打印出文字 `Hello, World!` 到屏幕上的程序，因此这里也会干这同样的事情！

> 注意：本书假定读者对命令行有着基本的熟悉。Rust 对代码在何处编辑和使用何种工具编辑没有特别要求，因此若优先选择某种集成开发环境，而非命令行，那么使用喜好的 IDE 即可。许多 IDE 都有某种程度的 Rust 支持；请查看 IDE 文档了解有关细节信息。近来，Rust 团队已着手启动良好的IDE支持，且此方面已取得极大进展！

## 创建一个项目目录

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

## 编写及运行 Rust 程序

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

而不论所在操作系统为何，字符串 `Hello, World!` 都应打印到终端。而若没有看到这个输出，那么请回到安装小节的 [“问题排除”](#问题排除) 部分获取帮助。

如确实打印出了 `Hello, World!`，那么恭喜你！你已正式编写除了一个 Rust 程序了。那就让你成为了一名 Rust 程序员了 -- 欢迎！

## Rust 程序解析

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


## 编译和运行是分开的步骤

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
