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
    - [通用数据类型](generic_types_traits_and_lifetimes/generics.md)
    - [特质：定义共用行为](generic_types_traits_and_lifetimes/traits.md)
    - [使用生命周期验证引用](generic_types_traits_and_lifetimes/lifetimes.md)


- [编写自动化测试](Ch11_Writing_Automated_Tests.md)
    - [怎样编写测试](automated_tests/howto.md)
    - [控制测试运行方式](automated_tests/how_tests_are_run.md)
    - [测试的组织](automated_tests/test_organization.md)

---


# 上篇总结 - 实操

- [一个文件系统 I/O 项目：构建一个命令行程序](Ch12_An_IO_Project_Building_a_Command_Line_Program.md)
    - [接收命令行参数](io_project/accepting_cli_arguments.md)
    - [读取文件](io_project/reading_a_file.md)
    - [重构以改进模块化和错误处理](io_project/refactoring.md)
    - [以测试驱动方法，开发这个库的功能](io_project/test_driven_dev.md)
    - [使用环境变量](io_project/env_variables.md)
    - [将错误消息写到标准错误，而非标准输出](io_project/std_err.md)


---

# 下篇

- [函数式语言特性：迭代器与闭包](Ch13_Functional_Language_Features_Iterators_and_Closures.md)
    - [闭包：会捕获其环境的匿名函数](functional_features/closures.md)
    - [使用迭代器处理条目序列](functional_features/iterator.md)
    - [改进咱们的 I/O 项目](functional_features/improving_io_project.md)
    - [性能比较：循环与迭代器](functional_features/performance.md)

- [Cargo 的其他方面及 Crates.io](Ch14_More_about_Cargo_and_Crates-io.md)
    - [使用发布配置文件自定义构建](crates-io/release_profiles.md)
    - [将代码箱发布到 Crates.io](crates-io/publishing.md)
    - [Cargo 工作区](crates-io/workspace.md)
    - [使用 `cargo install` 安装 Crates.io 上的二进制程序](crates-io/cargo_install.md)
    - [以定制命令扩展 Cargo](crates-io/custom_commands.md)

- [灵巧指针](Ch15_Smart_Pointers.md)
    - [使用 `Box<T>` 指向内存堆上的数据](smart_pointers/box-t.md)
    - [使用 `Deref` 特质将灵巧指针视为常规引用](smart_pointers/deref-t.md)
    - [使用 `Drop` 特质在内存清理时运行代码](smart_pointers/drop-t.md)
    - [引用有计数的灵巧指针 `Rc<T>`](smart_pointers/rc-t.md)
    - [`RefCell<T>` 与内部可变性模式](smart_pointers/refcell-t.md)
    - [引用环会泄露内存](smart_pointers/ref-cycles.md)

- [无惧并发](Ch16_Fearless_Concurrency.md)
    - [使用线程同步运行代码](concurrency/threads.md)
    - [使用消息传递再线程间传输数据](concurrency/message_passing.md)
    - [共用状态的并发](concurrency/shared-state.md)
    - [使用 `Sync` 与 `Send` 的可扩展并发](concurrency/extensible_concurrency.md)


- [Rust 的面向对象编程特性](Ch17_Object_Oriented_Programming_Features_of_Rust.md)
    - [面向对象语言的特征](oop/characteristics_oop.md)
    - [使用允许不同类型值的特质对象](oop/trait_objects.md)
    - [实现一种面向对象设计模式](oop/implementing.md)

- [模式与匹配](Ch18_Patterns_and_Matching.md)
    - [可使用模式的全部处所](patterns/all_places.md)
    - [可证伪性：某个模式是否会匹配失败](patterns/refutability.md)
    - [模式语法](patterns/syntax.md)

- [先进特性](Ch19_Advanced_Features.md)
    - [不安全的 Rust](advanced_features/unsafe.md)
    - [高级特质](advanced_features/adv_traits.md)
    - [高级类型](advanced_features/adv_types.md)
    - [高级函数与闭包](advanced_features/adv_fns_and_closures.md)
    - [关于宏](advanced_features/macros.md)

- [最后项目：构建一个多线程的 Web 服务器](Ch20_Final_Project_Building_a_Multithreaded_Web_Server.md)

- [附录](Ch21_Appendix.md)
