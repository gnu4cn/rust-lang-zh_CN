# `Sync` 与 `Send` 两个特质下的可扩展并发

**Extensible Concurrency with the `Sync` and `Send` Traits**


有趣的是，Rust 语言并发方面的特性 *非常* 少。本章中到目前为止咱们讲到过的每种并发特性，都已是标准库而非语言本身的一部分。用于处理并发问题的选项，并不局限于这门语言或标准库；咱们可以编写自己的并发特性，或可以使用由其他人编写的并发特性。

不过，在这门语言中，是嵌入了两个并发概念的：即 `std::marker` 特质 `Sync` 与 `Send`。


## 使用 `Send` 特质实现线程间所有权转移

**Allowing Transference of Ownership Between Threads with `Send`**


这个 `Send` 标识符特质，表示实现 `Send` 类型值的所有权，可以在线程间转移。几乎全部 Rust 类型都是 `Send` 类型，但有一些例外，包括 `Rc<T>`：由于在咱们克隆了某个 `Rc<T>` 并尝试将这份克隆的所有权，转移到另一线程时，两个现场可能在同一时间更新引用计数，因此 `Rc<T>` 就不能是 `Send` 类型。由于这个原因，`Rc<T>` 正是为其间咱们不打算付出线程安全方面性能开销的那些单线程情形，而实现的。

由此，Rust 的类型系统与特质边界，type system and trait bounds，就确保了咱们绝不会意外地将某个 `Rc<T>`，不安全地跨越线程发送。当咱们在清单 16-14 中尝试这样做时，咱们就曾得到编译器报错 `` the trait `Send` is not implemented for `Rc<Mutex<i32>>` ``。而在咱们切换到 `Arc<T>` 这种 `Send` 类型时，那段代码就编译了。

由全部 `Send` 类型所组成的类型，也会被自动标记为 `Send` 类型。除开那些原始指针，raw pointers 外，那么可以说几乎全部原生类型都是 `Send` 的，咱们将在第 19 章中，讲到那些原始指针。


## 使用 `Sync` 实现来自多个线程的访问

**Allowing Access from Multiple Threads with `Sync`**

`Sync` 标识符表示实现 `Sync` 特质的类型，其被从多个线程引用是安全的。换句话说，任何类型 `T` 在 `&T` （即到 `T` 的不可变引用） 为 `Send` 的时，那么其即为 `Sync` 的，表示该引用可以安全地发送到另一线程。与 `Send` 类似，原生类型均为 `Sync` 的，且由全部都是 `Sync` 的类型所组成的类型，也都是 `Sync` 的。

灵巧指针 `Rc<T>` 因为其不是 `Send` 的同样原因，其也不是 `Sync` 的。`RefCell<T>` 类型（咱们曾在第 15 章讲过）以及相关的 `Cell<T>` 类型家族，都不是 `Sync` 的。`RefCell<T>` 在运行时所完成的借用检查实现，不是线程安全的。灵巧指针 `Mutex<T>` 是 `Sync` 的，并正如咱们在 [于多个线程间共用 `Mutex<T>`](#在多个线程间共用-mutext) 小节中看到的，其可被用于多个线程下共用访问。


## 手动实现 `Send` 与 `Sync` 是不安全的

**Implementing `Send` and `Sync` Manually Is Unsafe**


由于 `Send` 与 `Sync` 特质构成的类型自动也是 `Send` 与 `Sync` 的，因此咱们大可不必手动实现这两个特质。而作为标记性特质，二者甚至都没有任何要实现的方法。他们只是在执行与并发性有关的不变性方面很有用。

手动实现这两个特质，涉及到实现一些不安全 Rust 代码，unsafe Rust code。在第 19 章咱们将讲到运用不安全 Rust 代码；至于现在，要点在于构造不是由一些 `Send` 与 `Sync` 部分组成的新并发类型，需要深思熟虑来维持那些安全保证。[The Rustonomicon](https://doc.rust-lang.org/nomicon/index.html) 有着这些保证的更多信息，以及维持这些保证的方式。


# 本章小节

这不会是你在本书中将见到并发的最后一章：第 20 张中的那个项目，就将在相比于这里所讨论过较小示例，而更具现实意义的情形下用到本章中的那些概念。

正如早先所提到的，由于只有极少量的 Rust 处理并发方式，属于这门语言的一部分，因此许多并发解决方案，都是作为代码箱实现的。这些方案相比标准库进化更为迅速，那么就要确保在线搜寻当前的、最前沿代码箱，来用于多线程情形中。

Rust 标准库提供了用于消息传递的信道，以及诸如 `Mutex<T>` 与 `Arc<T>` 等安全用于并发情景中的一些灵巧指针类型。类型系统与借用检查器，会确保应用了这些方案的代码，不会以数据竞争或无效引用结束。一旦让代码编译了，咱们就可以放下心来，代码将愉快地运行于多线程之上，而不会有在其他语言中常见的那些难于追踪的问题。并发编程自此不再是令人害怕的概念：去吧，让你的程序并发起来，无所畏惧！

接下来，咱们将讲到，随着咱们的 Rust 程序变得大了起来，建模问题与架构出方案的一些管用做法。此外，咱们将讨论 Rust 的一些习惯说法，这些说法可能与面向对象编程中所熟悉的有关。


（End）


