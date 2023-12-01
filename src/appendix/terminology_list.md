# 附录 I - 术语清单


- 命令行界面

Command-Line Interface，在终端里运行的应用，与在 GUI 窗口中应用不同。

- 模组系统

The module system，大型程序中，组织代码的方式。


- 迁移所有权

在闭包参数清单前，使用 `move` 关键字，让闭包取得其用到的所在环境中的值所有权。

- 关联类型

Associated type, 是通过 `type` 关键字，定义在特质下的类型。咱们知道方法即为关联函数，associated function，那么关联类型自然与关联函数有些类似。


- 消费适配器

Consuming adaptor, `Iterator` 特质上，会调用到迭代器 `next` 方法的一些方法，由于这些方法会耗尽迭代器，故他们被称为消费适配器。


- 迭代器适配器

Iterator adaptor，`Iterator` 特质上，通过改变原迭代器某些方面而产生出另一迭代器的一些方法。


- 零成本抽象

Zero-cost abstractions，相较于一般实现，语言提供的高级抽象在编译后生成的代码，与自己努力编写出的优化低级别代码类似。故使用高级抽象是没有运行时开销的。


- 展开优化

Unrolling，Rust 编译器在编译迭代器代码时，会把已知的历次迭代展开为重复代码，而实现性能优化。


- 文档注释

Documentation comment, 将产生出 HTML 的注释。


- 重导出程序项目

Re-export, 使用 `pub use` 重新导出程序项目。


- 语义版本控制规则

Semantic Versioning rules, 又大版本、小版本及补丁版本构成的，形如 `MAJOR.MINOR.PATCH` 的版本编号规则。参考：[semver.org](https://semver.org)。


- 工作区

Workspace，为有着多个库代码箱的大型项目组织的一项 Cargo 特性。


- 编译出的物件

The compiled artifacts


- 路径依赖

A path dependency


- 匣子类型（数据结构）

`Box<T>`，由存储在栈上的指针，与存储在堆上的数据，实现的一种数据结构。


- 间接

Indirection, 匣子类型的变量，通过保存指向数据在内存堆上的地址，而间接保存了数据。


- 解引用强制转换

Deref coercion，类似于其他语言的开箱操作。


- 元组结构体

A tuple struct, 形式为 `struct MyBox<T>(T)`，是保持着只有一个元素元组的结构体，`Box<T>` 的数据结构为元组结构体。


- 前奏

The Rust Prelude, `std::prelude` 模组。前奏是 Rust 自动导入到每个 Rust 程序中的东西的列表。他被保持在尽可能小的范围内，并且专注于几乎每个 Rust 程序都会用到的东西，特别是特质。参见：[`std::prelude`](https://doc.rust-lang.org/std/prelude/index.html)。


- 内部可变性模式

The interior mutability pattern, Rust 的一种设计模式，用于改变不可变值内部的某个值。


- 内存泄漏

Memory leak, 出现未清理内存的情况。


- 关联类型

An associated type, 通过 `type Target = t;` 这种语法声明出的类型，是声明泛型参数的一种稍微不同的方式。


- 单态化

所谓 *单态化，monomorphization*，是指即通过把在编译后用到的具体类型填入到泛型位置，而将通用代码转换为具体代码的过程。参考 [使用泛型代码的性能问题](Ch10_Generic_Types_Traits_and_Lifetimes.md#使用泛型参数代码的性能问题)。


- 内聚属性

a property called *coherence*，参见 [在类型上实现某个特质](Ch10_Generic_Types_Traits_and_Lifetimes.md#在类型上实现某个特质)。


- 孤儿规则

the orphan rule, 参见 [在类型上实现某个特质](Ch10_Generic_Types_Traits_and_Lifetimes.md#在类型上实现某个特质)。


- `impl Trait` 语法

`impl Trait` syntax, 在函数参数清单中，将特质用作参数类型注解的语法。参见：[作为参数的特质](Ch10_Generic_Types_Traits_and_Lifetimes.md#作为参数的特质)


- 特质边界语法

Trait bound syntax, 参见 [特质边界语法](Ch10_Generic_Types_Traits_and_Lifetimes.md#特质边界语法)


- 语法糖

Sugar syntax, 参见 [特质边界语法](Ch10_Generic_Types_Traits_and_Lifetimes.md#特质边界语法)


- 指明多个特质边界的 `+` 语法

The `+` syntax for specifying multiple trait bounds, 参见：[使用 + 语法，指定多个特质边界](Ch10_Generic_Types_Traits_and_Lifetimes.md#使用--语法指定多个特质边界)


- `where` 子句

`where` clauses, 参见 []()


- 生命周期省略规则

Lifetime elision rules, 编程到 Rust 引用分析中的一些确定性模式。


- 输入生命周期

Input lifetimes，函数或方法上的生命周期


- 输出生命周期

Output lifetimes, 返回值上的生命周期
