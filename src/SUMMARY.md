# Summary


[序和前言](Ch00_Forword_and_Introduction.md)

---

- [入门](Ch01_Getting_Started.md)
    - [安装](getting_started/installation.md)
    - [Hello, World!](getting_started/hello_world.md)
    - [你好，Cargo！](getting_started/hello_cargo.md)

- [编写猜数游戏](Ch02_Programming_a_Guessing_Game.md)

- [常见编程概念](Ch03_Common_Programming_Concepts.md)
    - [变量与可变性](programming_concepts/variables_and_mutability.md)
    - [数据类型](programming_concepts/data_types.md)
    - [函数](programming_concepts/functions.md)
    - [注释](programming_concepts/comments.md)
    - [控制流](programming_concepts/control_flow.md)

---

# 进阶

- [“掌握” 所有权](Ch04_Understanding_Ownership.md)
    - [何为所有权？](ownership/about_ownership.md)
    - [引用与借用](ownership/references_and_borrowing.md)
    - [切片类型](ownership/the_slice_type.md)

- [使用结构体来对相关数据进行架构](Ch05_Using_Structs_to_Structure_Related_Data.md)
    - [定义并初始化结构体](structs/defining_and_instantiating.md)
    - [运用结构体的一个示例程序](structs/example_program.md)
    - [方法语法](structs/method_syntax.md)

- [枚举与模式匹配](Ch06_Enums_and_Pattern_Matching.md)
    - [定义一个枚举](enums_and_pattern_matching/defining_an_enum.md)
    - [`match` 控制流结构](enums_and_pattern_matching/match_control_flow.md)
    - [使用 `if let` 的简洁控制流](enums_and_pattern_matching/if-let_contorl_flow.md)

- [使用包、代码箱与模组对日趋增长的项目进行管理](Ch07_Managing_Growing_Projects_with_Packages_Crates_and_Modules.md)
    - [代码包与代码箱](packages_crates_and_modules/packages_and_crates.md)
    - [定义用于控制作用域及隐私性的模组](packages_crates_and_modules/defining_modules.md)
    - [用于引用模组树中某个项目的路径](packages_crates_and_modules/paths.md)
    - [使用 `use` 关键字将路径带入作用域](packages_crates_and_modules/the_use_keyword.md)
    - [将模组分成不同文件](packages_crates_and_modules/separating_modules.md)

- [常用集合数据结构](Ch08_Common_Collections.md)
    - [使用矢量值存储值的清单](common_collections/vectors.md)
    - [使用字符串存储 UTF-8 编码的文本](common_collections/strings.md)
    - [在哈希图种存储带有关联值的键](common_collections/hash_maps.md)

- [错误的处理](Ch09_Error_Handling.md)
    - [使用 `panic!` 宏的不可恢复错误](error_handling/panic.md)
    - [使用 `Result` 的可恢复错误](error_handling/result.md)
    - [要 `panic!` 还是不要 `panic!`](error_handling/panic_or_not.md)

---

# 深入掌握

- [泛型、特质与生命周期](Ch10_Generic_Types_Traits_and_Lifetimes.md)

- [编写自动化测试](Ch11_Writing_Automated_Tests.md)

---


# 上篇总结 - 实操

- [一个文件系统 I/O 项目：构建一个命令行程序](Ch12_An_IO_Project_Building_a_Command_Line_Program.md)


---

# 下篇

- [函数式编程语言特性：迭代器与闭包](Ch13_Functional_Language_Features_Iterators_and_Closures.md)

- [Cargo 的其他方面及 Crates.io](Ch14_More_about_Cargo_and_Crates-io.md)

- [灵巧指针](Ch15_Smart_Pointers.md)

- [无惧并发](Ch16_Fearless_Concurrency.md)

- [Rust 的面向对象编程特性](Ch17_Object_Oriented_Programming_Features_of_Rust.md)

- [模式与匹配](Ch18_Patterns_and_Matching.md)

- [先进特性](Ch19_Advanced_Features.md)

- [最后项目：构建一个多线程的 Web 服务器](Ch20_Final_Project_Building_a_Multithreaded_Web_Server.md)

- [附录](Ch21_Appendix.md)
