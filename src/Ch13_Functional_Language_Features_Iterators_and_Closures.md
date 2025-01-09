# 函数式编程语言特性：迭代器与闭包

Rust 的设计曾受到许多现有的语言和技术的启发，而一个显著的影响，便是 *函数式编程，functional programming*。以函数式风格编程，通常包括了通过把函数传入到参数中，或从其他函数返回函数，及将函数赋值给变量以便稍后执行等等，而将函数当作值使用，programming in a functional style often includes using functions as values by using functions as values by passing them in arguments, returning them from another functions, assigning them to variables for later execution, and so forth。

本章中，咱们不会讨论函数式编程是什么或不是什么的问题，而将讨论与许多通常被指为函数式编程语言中特性类似的 Rust 特性。

更具体地说，咱们将讲到：

- *闭包，closures*，可存储在变量中、类似函数的结构体;
- *迭代器，iterators*，处理元素序列的方式，a way of processing a series of elements;
- 如何使用闭包与迭代器，来改进第 12 章中的那个 I/O 项目；
- 闭包与迭代器的性能问题（剧透警告：他们比咱们可能想的要快！）。

咱们已经讲到过其他的一些 Rust 特性，诸如模式匹配与枚举等，也是受函数式编程影响的。由于掌握闭包与迭代器，是编写惯用、快速 Rust 代码的重要方面，因此咱们将把这整章，都用来讲解他们。


（End）


