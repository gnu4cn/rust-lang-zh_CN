# 无惧并发

**Fearless Concurrency**

安全并高效地处理并发编程，是 Rust 的另一主要目标。所谓 *并发编程，concurrent programming*，是指其中程序的各部分独立地执行着，而 *并行编程，parallel programming*，则是指程序的不同部分于同一时间执行，随着越来越多的计算机利用了多处理器的优势，这两种编程范式变得日益重要起来。历史上，这两种情景下的编程，曾是有难度且容易出错的：Rust 就有望来改变这种局面。

早期阶段，Rust 团队曾认为确保内存安全与防止并发问题，属于要以不同方法来解决的两个单独挑战。随着时间的推移，团队发现所有权与类型系统，正是有助于管理内存安全，*及* 并发问题的一套强有力的工具！经由利用所有权与类型检查，许多的并发错误，就成了 Rust 中的编译时错误，而非运行时错误。因此，就不再是要咱们，在出现运行时并发错误时，花费大量时间尽力重现那些确切情形，而是那些不正确代码，将拒绝编译，并给出代码问题的错误提示。由此，咱们就可以在编写出错误代码时，而非潜在地于交付代码到生产之后，修复好这些代码。这里将 Rust 此方面的特性，亲切地取名为 *无惧并发，fearless concurrency*。无惧并发实现了编写出不带难以察觉错误的代码，且易于在不引入新代码错误之下，对代码加以重构。

> **注意**：为简化起见，这里将把许多的这些问题，指为 *并发，concurrency*，而非称作更准确的 *并发及/或并行，concurrency and/or parallel*。若本书是有关并发及/或并行编程的书，那么咱们就会更为具体。对于本章，请在任何提及 *并发* 之处，在内心里将其以 *并发及/或并行* 代换。

许多语言在他们所提供的，用于解决并发问题的方案上，都是机械教条主义的。比如，Erlang 有着消息传递方面并发的优雅功能，但在共用线程间状态方面，却只有一些晦涩难懂的的途径，for example, Erlang has elegant functionality for message-passing concurrency, but has only obscure ways to share state between threads。对于这类高级语言来讲，仅支持可行方案的子集，是说得通的一种策略，这是由于高级语言以放弃部分的掌控，而换取到抽象方面的收益。然而，那些底层语言，则被期望在各种情形下，都要提供最具性能的方案，进而在硬件上有着较少抽象。因此，Rust 便提供了用以适合于咱们自己不同情形与需求的各种方式，对问题加以建模的各种工具，therefore, Rust offers a variety of tools for modeling problems in whatever way is appropriate for your situtation and requirements。

以下即为本章咱们将涵盖的几个话题：

- 怎样创建出线程，来在同一时间运行代码的不同片段，how to create threads to run multiple pieces of code at the same time；

- *消息传递，message-passing* 方面的并发，其中有着于线程间发送消息的一些通道；

- *状态共用，shared-state* 方面的并发，其中多个线程均对某个数据加以访问；

- `Sync` 与 `Send` 特质，他们俩把 Rust 并发方面的保证，扩展到 Rust 使用者所定义的类型，以及由标准库所提供的那些类型。


（End）


