# I/O 项目：构建命令行程序

这一章是对咱们迄今为止学到的许多技能的回顾，并是对更多的一些标准库特性的探索。我们将构建一个与文件及命令行输入输出交互的命令行工具，以练习咱们目前已经掌握的一些 Rust 概念。

Rust 的速度、安全性、单一二进制输出和跨平台支持，使其成为创建命令行工具的理想语言，因此对于我们的项目，我们将构造自己版本的经典命令行搜索工具 `grep` （**g**lobally search a **r**egular **e**xpression and **p**rint，全局检索正则表达式并打印）。在最简单的用例下，`grep` 会检索指定文件中的指定字符串。为此，`grep` 会取文件路径和字符串作为其参数。然后，他读取该文件，找到该文件中包含字符串参数的行，并打印这些行。

在此过程中，我们将展示如何使我们的命令行工具，使用许多其他命令行工具都用到的终端特性。我们将读取环境变量的值，以允许用户配置我们工具的行为。我们将打印错误消息到标准错误控制台流，the standard error console stream, `stderr`，而不是标准输出（`stdout`），以便用户可以重定向成功的输出到文件，同时仍然在屏幕上看到错误消息。

Rust 社区成员 Andrew Gallant 已经创建了个特性完整、速度非常快的 `grep` 版本，称为 [`ripgrep`](https://github.com/BurntSushi/ripgrep)。相比之下，我们的版本将相当简单，但这一章将给予咱们理解比如 `ripgrep` 这样的真实项目所需的背景知识。

我们的 `grep` 项目将结合咱们迄今为止学到的数个概念：

- 组织代码（[第 7 章](./Ch07_Managing_Growing_Projects_with_Packages_Crates_and_Modules.md)）
- 使用矢量值与字符串（[第 8 章](./Ch08_Common_Collections.md)）
- 处理错误（[第 9 章](./Ch09_Error_Handling.md)）
- 在适当情况下使用特质和生命周期（[第 10 章](./Ch10_Generic_Types_Traits_and_Lifetimes.md)）
- 编写测试（[第 11 章](./Ch11_Writing_Automated_Tests.md)）

我们还将简要介绍闭包、迭代器与特质对象等，[第 13 章](./Ch13_Functional_Language_Features_Iterators_and_Closures.md) 与 [第 18 章](./Ch17_Object_Oriented_Programming_Features_of_Rust.md) 将详细介绍他们。


