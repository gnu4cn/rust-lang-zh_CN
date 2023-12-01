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
