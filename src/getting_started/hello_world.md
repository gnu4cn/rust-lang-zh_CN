# `Hello, World!`

现在咱们已经安装了 Rust，那么就是时候编写第一个 Rust 程序了。学习一门新语言的传统做法，是编写一个在屏幕上打印出 `Hello, world!` 的小程序，所以咱们在这里，也要做同样的事情。

> **注意**：本书假定读者基本熟悉命令行。Rust 对编辑、工具或代码的存放位置，没有特别的要求，因此如果咱们喜欢使用集成开发环境（IDE），而不是命令行，请随意使用咱们喜欢的集成开发环境。许多集成开发环境，现在都在一定程度上有着 Rust 的支持；详情请查看集成开发环境的文档。Rust 团队一直致力于通过 `rust-analyzer`，实现对集成开发环境的强大支持。详情请参见 [附录 D](../appendix/dev_tools.md)。


## 创建项目的目录

**Creating a Project Directory**


首先，咱们将创建一个存储咱们 Rust 代码的目录。对于 Rust 来说，代码存放在哪里并不重要，但对于本书中的练习和项目，我们建议在咱们的主目录下，创建一个 `projects` 目录，并将咱们的所有项目，都放在那里。

这里是以构造一个保存 Rust 代码的目录开始的。对于 Rust 来说，代码位居何处并不重要，不过对于本书中的练习与项目，是建议在主目录下构造一个 `projects` 目录，并把全部项目放在那里的。

请打开某个终端，并输入以下命令，创建 `projects` 目录，并在 `projects` 目录下，创建出这个 "Hello, world!" 的项目目录。

对于 Linux、macOS 和 Windows 上的 `PowerShell`, 请输入：

```console
$ mkdir ~/projects
$ cd ~/projects
$ mkdir hello_world
$ cd hello_world
```

而对于 Windows 的 CMD， 请输入：

```console
> mkdir "%USERPROFILE%\projects"
> cd /d "%USERPROFILE%\projects"
> mkdir hello_world
> cd hello_world
```

## 编写并运行一个 Rust 程序


接下来，请创建一个新的源文件，并将其命名为 `main.rs`。Rust 文件总是以 `.rs` 扩展名结尾。如果咱们在文件名中，使用了多个单词，惯例是要使用下划线，分隔这些单词。例如，请使用 `hello_world.rs`，而不是 `helloworld.rs`。

现在请打开咱们刚刚创建的 `main.rs` 文件，输入下面清单 1-1 中的代码。

文件名：`main.rs`

```rust
fn main() {
    println!("Hello, World!");
}
```

*清单 1-1：打印出`Hello, World!` 的一个程序*

请保存该文件，并返回到 `~/projects/hello_world` 目录下，咱们的终端窗口。在 Linux 或 macOS 上，请输入以下命令，编译并运行该文件：

```console
$ rustc main.rs
$ ./main
Hello, World!
```

在 Windows 上，请输入命令 `.\main.exe` 而不是 `./main`：

```console
> rustc main.rs
> .\main.exe
Hello, World!
```

> **注意**：在 Windows 上使用 ZSH 的 MSYS2 环境下，上述命令的输出：

```console
➜  hello_word rustc main.rs
➜  hello_word ls
main.exe  main.pdb  main.rs
➜  hello_word ./main.exe
Hello, World!
```

不论咱们的操作系统如何，`Hello, world!` 这个字符串都应打印到终端。如果咱们看不到此输出，请参阅安装小节的 [故障排除](installation.md#问题排除) 部分，了解获得帮助的方法。

如果 `Hello, World!` 打印出来了，那么恭喜！咱们已经正式编写了一个 Rust 程序。这使咱们成为了一名 Rust 程序员 —— 欢迎！


## Rust 程序剖析

**Anatomy of a Rust Program**


咱们来详细回顾一下这个 "Hello, World!" 程序。下面是谜团的第一部分：

```rust
fn main() {

}
```

这些行定义了一个名为 `main` 的函数。`main` 函数很特别：他总是每个可执行 Rust 程序中，运行的第一段代码。这里，其中第一行，声明了一个没有参数，也不返回任何内容的名为 `main` 的函数。如果有参数，则会放在括号 `()` 内。

函数体被包裹在 `{}` 中。Rust 要求，在所有函数体周围使用大括号。将开头的大括号，与函数声明放在同一行，在二者之间添加一个空格，是一种良好的编码风格。

> **注意**：如果咱们想在不同的 Rust 项目间，坚持使用某种标准的风格，咱们可以使用一个名为 `rustfmt` 的自动格式化工具，以某种特定风格，格式化咱们的代码（关于 `rustfmt` 的更多信息，请参见 [附录 D](../appendix/dev_tools.md)）。如同 `rustc` 一样，Rust 团队已将这个工具，包含在了标准 Rust 发布中，因此他应该已安装在咱们的电脑上了！

`main` 函数的主体，包含以下代码：

```rust
println!("Hello, World!");
```

这行代码，完成了这个小型程序的所有工作：将文本打印到屏幕上。这里有四个重要细节要注意。

首先，Rust 的编码风格，是缩进四个空格，而非一个制表符；


其次，`println!` 调用了一个 Rust 宏。如果他调用的是个函数，就会以 `println` 的形式输入（不带 `!`）。 我们将在第 19 章，详细讨论 Rust 的宏。现在，咱们只需知道，使用 `!` 表示咱们调用的是宏，而不是普通函数，而且宏并不总会遵循，与函数相同的规则。

第三，咱们会看到那个 "Hello, world!" 字符串。我们将该字符串作为参数，传递给了 `println!`；

最后，我们以表示此表达式结束，下个表达式准备开始的分号（`;`），结束了这行。Rust 代码的大多数行，都会以分号结束。


## 编译和运行是两个独立的步骤

**Compiling and Running Are Separate Steps**


咱们刚刚运行了一个新进创建出的程序，那么就来监视一下，这个过程的每个步骤。

运行某个 Rust 程序之前，咱们必须通过输入 `rustc` 命令，并将源文件的名字传给他，而使用 Rust 的编译器编译他，就像这样：

```console
$ rustc main.rs
```

如果你有 C 或 C++ 背景，就会发现这与 `gcc` 或 `clang` 类似。编译成功后，Rust 会输出二进制可执行文件。

在 Linux、macOS 以及 Windows 的 PowerShell 中，咱们可以在咱们的 shell 中，输入 `ls` 命令看到这个可执行文件：


```console
$ ls
main    main.rs
```

在 Linux 和 macOS 上，咱们将看到两个文件。而在 Windows 上的 PowerShell 中，咱们将看到与使用 CMD 相同的三个文件。在 Windows 上的 CMD 中，咱们可以输入以下内容：

```console
> dir /B %= 这里的 /B 选项表示只显示文件名 =%
main.exe
main.pdb
main.rs
```

这显示了扩展名为 `.rs` 的源代码文件、可执行文件（Windows 上为 `main.exe`，而在所有其他平台上为 `main`），以及使用 Windows 时，一个扩展名为 `.pdb`，包含着调试信息的文件。从这里，咱们运行那个 `main` 或 `main.exe` 文件，如下所示：


```console
$ ./main # 或在 Windows 上的 .\main.exe
```
如果咱们的 `main.rs`，是那个 "Hello, world!" 程序，那么这行就会将 `Hello, world!` 打印到终端。

如果咱们更熟悉 Ruby、Python 或 JavaScript 等某门动态语言，则可能不习惯于将编译和运行某个程序，作为单独的步骤。Rust 属于一门 *提前编译，ahead-of-time compiled* 语言，这意味着，你能够编译某个程序，并把可执行文件交给其他人，而即使他们没有安装 Rust，也可以运行这个可执行程序。而如果咱们给别人某个 `.rb`、`.py` 或 `.js` 文件，他们就需要分别安装 Ruby、Python 或 JavaScript 的某种实现。不过在这些语言中，咱们只需要一条命令，来编译并运行咱们的程序。在语言设计中，一切都需要权衡利弊，everything is a trade-off in language design。

对于简单程序，使用 `rustc` 编译就可以了，但随着项目的发展，咱们会希望管理所有选项，以及并方便共用咱们的代码。接下来，我们将介绍，将会帮助咱们编写真实世界 Rust 程序的 Cargo 工具。


（End）


