# 一个文件系统 I/O 项目：构建一个命令行程序

这一章是对到目前为止所学到技能的一个回顾，又是对少数几个另外标准库特性的一个探索。这里将构建一个与文件及命令行输入输出进行交互的命令行工具，来练习咱们现在掌握到的一些 Rust 概念。

Rust 的速度、安全性、单一二进制可执行程序输出，还有跨平台支持，令其成为创建命令行工具的理想编程语言，那么对于这个项目，这里将构造自己版本的那个经典的命令行搜索工具 `grep` （**g**lobally search a **r**egular **e**xpression and **p**rint，正则表达式全局搜索及打印程序）。在最简单用例中，`grep` 会对某个指定文件，就某个指定字符串而加以搜索。为完成这个目的，`grep` 就会取一个文件路径与一个字符串作为其命令行参数。随后他会读取那个文件，找到那个文件中包含有该字符串参数的那些行，并打印出这些行。

在构造这个命令行程序的道路上，这里将展示如何让这个命令行工具，使用到其他命令行工具都会用到的一些终端特性（the terminal features）。这里将读取某个环境变量的值，来允许使用者对这个工具默认行为进行配置。这里还会将错误消息打印到标准错误控制台的流（the standard error console stream, `stderr`），而非打印到标准输出（`stdout`），如此一来，用户就可以将成功的输出重定向到某个文件，而仍能从屏幕上看到错误消息，并有着其他一些好处。

名为 Andrew Gallant 的一位 Rust 社区成员，就已经创建了一个特性完整、非常快版本的 `grep`，名叫 [`ripgrep`](https://github.com/BurntSushi/ripgrep)。相比之下，这个版本将相当简单，不过这一章将给到一些掌握诸如 `ripgrep` 这样的真实项目，所需的背景知识。

这个 `grep` 项目，将结合至今所掌握的下面几个到目前为止已掌握的概念：

- 对代码进行组织（使用 [第 7 章](Ch07_Managing_Growing_Projects_with_Packages_Crates_and_Modules.md) 中所掌握的有关模组的知识）
- 对矢量值与字符串的使用（集合，[第 8 章](Ch08_Common_Collections.md)）
- 对错误的处理（[第 9 章](Ch09_Error_Handling.md)）
- 在恰当之处使用特质与生命周期（[第 10 章](Ch10_Generic_Types_Traits_and_Lifetimes.md)）
- 编写测试（[第 11 章](Ch11_Writing_Automated_Tests.md)）

这里还会简要对闭包、迭代器及特质对象等，进行简要介绍，后面的 [第 13 章](Ch13_Functional_Languages_Features_Iterator_and_Closures.md) 与 [第 17 章](Object_Oriented_Programming_Features_of_Rust.md) 等章节，将详细讲解到这些特性。


