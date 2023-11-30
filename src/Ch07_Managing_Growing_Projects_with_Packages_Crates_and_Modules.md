# 用代码包、代码箱与模组来对日益增长的项目进行管理

在编写大型程序时，由于在头脑里对整个程序保持追踪已成为不可能，因此代码组织就尤为重要。通过将相关功能分组，并以截然不同的特性而将代码加以分离，就会搞清楚在哪里去找到实现了某个特定特性的代码，以及在哪里去修改某项特性的运作方式。

到目前为止，这里所编写的程序，都是在一个模组的一个文件中的。而随着项目的增长，就可以通过将项目分解为多个模组及多个文件，来对代码加以组织。一个代码包，可以包含多个二进制的代码箱，并可有选择地包含一个库代码箱。本章会涵盖到所有的这些技巧。对于那些极为大型、有着一套互相关联而共同演化的项目，Cargo 工具提供了工作区（workspaces）概念，关于工作区，将在第 14 章的 [Cargo 工作区](Ch14_More_about_Cargo_and_Crates_io.md#cargo-工作区)中讲到。

除了实现功能上的分组（grouping functionality）外，对功能实现细节的封装，还实现了更高层次上的代码重用：一旦实现了某个操作，其他代码就可以在无需掌握其实现原理的情况下，通过该代码的公共接口，对该实现代码加以调用。编写代码的方式，就定义了哪些部分是公开给其他代码使用的，哪些部分是私有实现细节而对其修改权力有所保留。这是对那些必须保留在头脑中细节实现数量，而有所限制的另一种方式（in addition to grouping functionality, encapsulating implementation details lets you reuse code at a higher level: once you've implemented an operation, other code can call that code via the code's pulic interface without knowing how the implementation works. The way you write code defines which part are public for other code to use and which parts are private implementation details that you reserve the right to change. This is another way to limit the amount of detail you have to keep in your head）。

而与此相关的概念，便是作用域（scope）：代码被编写出处的嵌套上下文，有着定义在所谓 “在作用域中（in scope）” 的一套名字。在读、写及编译代码时，程序员与编译器，二者都需要掌握，在某个特定点位处的某个特定名字，是否是指向了某个变量、函数、结构体、枚举、模组、常量或别的项目，以及该名字所指向项目的意义。创建作用域，及将一些名字的在或不在某个作用域加以修改，都是可行的。在同一作用域中，不能有两个名字相同的项目；有一些工具，可用于找出名字冲突。

对于包括哪些细节被暴露、哪些细节为私有，以及程序中各个作用域中有哪些名字等的代码组织，Rust 有着数种特性实现对其的管理。Rust 的这些有关代码组织的特性，有时被统称为 *模组系统（module system）*，包括了：

- **代码包（packages）**：实现代码箱（crates）的构建、测试与分享的 Cargo 特性；
- **代码箱（crates）**：产生出库或可执行文件的模组树（a tree of modules that produces a library or executable）；
- **模组（modules）** 与 **`use`关键字**：实现对代码组织、作用域及路径私有的控制（let you control the organization, scope, and privacy of paths）；
- **路径（paths）**：对结构体、函数或模组等进行命名的方式（a way of naming an item, such as a struct, function, or module）。

在本章中，就要涉及到这些特性，讨论他们之间互动的原理，以及如何运用这些特性，来对作用域加以管理。在本章结束时，就会对 Rust 的模组系统有扎实掌握，并能够像专业 Rust 程序员那样，以作用域来编写程序！
