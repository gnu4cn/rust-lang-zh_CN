# `Rc<T>`，引用计数灵巧指针

**`Rc<T>`, the Reference Counted Smart Pointer**


在大多数情况下，所有权是明确的：咱们确切地知道哪个变量拥有给定值。然而，在有些情况下，单个值可能有多个所有者。例如，在图数据结构中，多条边可能指向同一个节点，而该节点在概念上被所有指向他的边所拥有。一个节点不应该被清理，除非没有任何边指向他，因此没有了所有者。

咱们必须使用 Rust 类型 `Rc<T>` 显式启用多重所有权，`Rc<T>` 是 *引用计数，reference counting* 的缩写。`Rc<T>` 类型记录了对一个值的引用数量，以确定该值是否仍在使用。如果对某个值的引用为零，那么这个值就可以被清理掉，而不会有任何引用变得无效。

请将 `Rc<T>` 设想为客厅里的一台电视。在有人进来看电视时，他们会打开他。其他人可以进入客厅并观看电视。当最后一人离开客厅时，他们会关掉电视，因为他已不再被使用了。如果有人在其他人还在看电视的情况下关掉电视，剩下的看电视的人就会哗然！

当我们想在内存堆上分配一些数据给程序的多个部分读取，且无法在编译时确定哪个部分会最后使用完这些数据时，我们就会使用 `Rc<T>` 类型。如果我们知道哪个部分会最后完成，我们就可以让那个部分成为数据的所有者，而在编译时执行的正常所有权规则就会生效。

请注意 `Rc<T>` 仅适用于单线程场景，only for use in single-threaded scenarios。当咱们在第 16 章中讨论并发时，咱们将介绍如何在多线程程序中进行引用计数。


## 使用 `Rc<T>` 来共用数据

**Using `Rc<T>` to Share Data**


让我们回到清单 15-5 中咱们的构造列表示例。回想一下，我们用 `Box<T>` 定义了他。这一次，我们将创建两个列表，二者会公用第三个列表的所有权。从概念上看，这类似于下图 15-3：

![`b` 与 `c` 两个列表，共用了第三列表 `a` 的所有权](images/15-03.svg)

*图 15-03：两个列表`b` 与 `c`，共用第三个列表 `a` 的所有权*

咱们将创建一个包含 `5` 和 `10` 的列表。然后我们再做两个列表：以 `3` 开头的 `b` 和以 `4` 开头的 `c`，然后 `b` 和 `c` 的列表都会延续到第一个包含 `5` 和 `10` 的 `a` 列表。换句话说，这两个列表将共用第一个包含 `5` 和 `10` 的列表。

尝试使用带有 `Box<T>` 的 `List` 定义来实现这个场景是行不通的，如示例 15-17 所示：

文件名：`src/main.rs`

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a));
}
```

*清单 15-17：演示咱们不允许有两个使用 `Box<T>` 的列表，试图共用第三个列表的所有权*

当我们编译这段代码时，我们得到了这样的报错：

```console
$ cargo run
   Compiling sp_demos v0.1.0 (/home/peng/rust-lang/sp_demos)
error[E0382]: use of moved value: `a`
  --> src/main.rs:11:30
   |
9  |     let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
   |         - move occurs because `a` has type `List`, which does not implement the `Copy` trait
10 |     let b = Cons(3, Box::new(a));
   |                              - value moved here
11 |     let c = Cons(4, Box::new(a));
   |                              ^ value used here after move

For more information about this error, try `rustc --explain E0382`.
error: could not compile `sp_demos` due to previous error;
```

`Cons` 变种拥有他们持有的数据，所以当我们创建 `b` 列表时，`a` 被迁移到 `b` 中，`b` 拥有了 `a`。然后，当我们在创建 `c` 时试图再次使用 `a`，我们不被允许，因为 `a` 已经被迁移了。

咱们原本可以将 `Cons` 的定义修改为持有引用，但那样咱们就必须指定生命周期参数。通过指定生命周期参数，咱们将指定列表中的每个元素，都至少与整个列表的寿命一样长。清单 15-17 中的元素与列表就是这种情况，但并非在所有情况下都如此。

相反，我们将改变 `List` 的定义，使用 `Rc<T>` 来代替 `Box<T>`，如下清单 15-18 所示。现在每个 `Cons` 变种将持有一个值和一个指向 `List` 的 `Rc<T>`。当我们创建 `b` 时，我们将克隆 `a` 所持有的 `Rc<List>`，而不是取得 `a` 的所有权，从而将引用的数量从一个增加到两个，并让 `a` 和 `b` 共用该 `Rc<List>` 中数据的所有权。在创建 `c` 时，我们也将克隆 `a`，将引用的数量从两个增加到三个。每次我们调用 `Rc::clone`，`Rc<List>` 中数据的引用数就会增加，除非对他的引用为零，否则数据不会被清理掉。

文件名：`src/main.rs`

```rust
{{#rustdoc_include ../projects/rc_demo/src/main.rs}}
```

*清单 15-18：使用 `Rc<T>` 的 `List` 定义*

我们需要添加一个 `use` 语句来将 `Rc<T>` 引入作用域，因为他不在 Rust 前奏中。在 `main` 中，我们创建了包含 `5` 和 `10` 的列表，并将其存储在 `a` 中的新 `Rc<List>` 中。然后当我们创建 `b` 和 `c` 时，我们调用了 `Rc::clone` 函数，并将对 `a` 中的 `Rc<List>` 的引用作为参数传递。

我们本可以调用 `a.clone()` 而不是 `Rc::clone(&a)`，但是 Rust 的惯例是在这种情况下使用 `Rc::clone`。`Rc::clone` 的实现并不像大多数类型的 `clone` 实现那样对所有数据进行深度拷贝。对 `Rc::clone` 的调用只是增加引用计数，这不会花费很多时间。数据的深度拷贝会花费很多时间。通过使用 `Rc::clone` 进行引用计数，我们可以直观地区分出深拷贝的那种克隆和增加引用计数的那种克隆。当寻找代码中的性能问题时，我们只需要考虑深拷贝的克隆，而可以不考虑对 `Rc::clone` 的调用。

> 注：第 4 章 [变量与数据交互方式之二：克隆](Ch04_Understanding_Ownership.md#变量与数据交互方式之二克隆) 中，曾提到：“当咱们看到对 `clone` 的调用时，咱们就知道一些任意的代码正在被执行，而这些代码可能开销很大。这是表明正在发生一些不同寻常事情的明显标志。”


## 克隆 `Rc<T>` 会增加引用计数

**Cloning an `Rc<T>` Increases the Reference Count**


我们来修改示例 15-18 中的工作示例，以便我们可以看到在我们创建和删除对 `a` 中的 `Rc<List>` 的引用时，引用计数会发生变化。

在下面清单 15-19 中，我们将更改 `main`，使其具有围绕列表 `c` 的内部作用域；然后我们可以看到当 `c` 超出作用域时引用计数如何变化。

文件名：`src/main.rs`

```rust
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println! ("在创建出 a 后，引用计数为 {}", Rc::strong_count(&a));

    let b = Cons(3, Rc::clone(&a));
    println! ("在创建出 b 后，引用计数为 {}", Rc::strong_count(&a));

    {
        let c = Cons(4, Rc::clone(&a));
        println! ("在创建出 c 后，引用计数为 {}", Rc::strong_count(&a));
    }

    println! ("在 c 超出作用域后，引用计数为 {}", Rc::strong_count(&a));
}
```

*清单 15-19：打印出引用计数*

在程序中引用计数发生变化的每一点上，我们都会打印引用计数，我们通过调用 `Rc::strong_count` 函数得到这个计数。这个函数被命名为 `strong_count` 而不是 `count`，是因为 `Rc<T>` 类型也有一个 `weak_count`；我们将在 [“防止引用循环：将 `Rc<T>` 变成 `Weak<T>`”](#防止引用循环将-rct-转变为-weakt) 小节中看到 `weak_count` 的用途。

这段代码打印出以下内容：

```console
$ cargo run
   Compiling sp_demos v0.1.0 (/home/lennyp/rust-lang/sp_demos)
    Finished dev [unoptimized + debuginfo] target(s) in 0.40s
     Running `target/debug/sp_demos`
在创建出 a 后，引用计数为 1
在创建出 b 后，引用计数为 2
在创建出 c 后，引用计数为 3
在 c 超出作用域后，引用计数为 2
```

我们可以看到 `a` 中的 `Rc<List>` 的初始引用计数为 `1`；然后每次我们调用 `clone`，计数都会增加 `1`。当 `c` 超出作用域时，计数会减少 `1`。我们不必像调用 `Rc::clone` 增加引用计数那样调用一个函数来减少引用计数：当 `Rc<T>` 值超出作用域时，`Drop` 特质的实现会自动减少引用计数。

我们在这个例子中看不到的是，当 `b` 和 `a` 在 `main` 的末尾超出作用域时，计数为 `0`，并且 `Rc<List>` 会被完全清除。使用 `Rc<T>` 允许单个值拥有多个所有者，而计数确保只要任何所有者仍然存在，该值就保持有效。

通过不可变的引用，`Rc<T>` 允许咱们在程序的多个部分之间共用数据，仅供读取。如果 `Rc<T>` 还允许咱们有多个可变引用，咱们可能会违反第四章中讨论的一个借用规则：对同一个地方的多个可变借用，会导致数据竞赛和不一致。但是，能够修改数据是非常有用的！在下一节中，我们将讨论内部可变性模式和 `RefCell<T>` 类型，咱们可以将其与 `Rc<T>` 结合起来使用，以应对这种不可变性限制。


（End）


