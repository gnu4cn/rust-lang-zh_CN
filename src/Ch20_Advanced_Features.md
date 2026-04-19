# 高级特性

**Advanced Features**

到此时，咱们业已学习了 Rust 编程语言的那些最为常用部分。在第 20 章中咱们完成另一个项目之前，将看看咱们可能会偶尔碰到，但却不会每天用到的这门语言的一些方面。咱们可将这一章，当作在今后遇到一些不明白之处时的一份参考。这里所涵盖的特性，在一些非常特定情形下是有用的。尽管咱们可能不会经常碰到这些情形，咱们还是希望，能掌握 Rust 所提供到的全部特性。

咱们将在这一章，涵盖以下内容：

- 不安全的 Rust, unsafe Rust: 怎样选择不使用 Rust 的一些保证，而由程序员亲自负责维持这些保证；

+ 一些高级特质，advanced traits:
    - 关联类型，associated types；
    - 默认类型参数，default type parameters；
    - 完全合格语法，fully qualified syntax；
    - 超特质，supertraits；
    - 及与特质相关的新型模式，the newtype pattern in relation to traits。

+ 一些高级类型：
    - 更多有关新型模式的内容；
    - 类型别名，type aliases；
    - 永恒类型，the never type；
    - 以及动态大小的类型，dynamically sized types。

- 高级函数与高级闭包：函数指针与返回的闭包，function pointers and returning closures；

- 宏，macros：那些在编译时定义了别的代码的代码定义方式，ways to define code that defines more code at compile time。


本章是给每个人应该了解的，一整套 Rust 特性！咱们就开始吧！


