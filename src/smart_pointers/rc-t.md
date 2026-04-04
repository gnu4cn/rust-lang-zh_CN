# `Rc<T>`，引用计数的灵巧指针

在大多数情况下，所有权是明确的：咱们确切地知道哪个变量拥有给定值。然而，存在单个值可能有多个所有者的情形。例如，在图数据结构下，多条边可能指向同一节点，进而这个节点在概念上由所有指向他的边所拥有。节点不应该被清理，除非没有任何边指向他而因此没有了所有者。

咱们必须显式地通过使用 Rust 类型 `Rc<T>` 启用多重所有权，`Rc<T>` 是 *引用计数，reference counting* 的缩写。`Rc<T>` 类型会跟踪对值的引用数量，以确定该值是否仍在使用。当对值的引用为零时，则该值可被清理，而不会有没有任何引用变得无效。

请将 `Rc<T>` 设想为客厅里的电视。当一个人进来看电视时，就会打开电视。其他人可以进入客厅看电视。当最后一个人离开客厅时，就会关掉电视，因为电视不再被使用。当有人在其他人还在看电视时关掉电视，剩下的电视观众就会哗然！

当我们打算在堆上分配某一数据供程序的多个部分读取，且无法在编译时确定哪个部分最后结束使用该数据时，我们就要使用 `Rc<T>` 类型。若我们知道哪个部分会最后结束使用，就可以直接让那个部分成为该数据的所有者，并且编译时执行的正常所有权规则就会生效。

请注意，`Rc<T>` 仅适用于单线程的场景。当我们在第 16 章中讨论并发时，我们将介绍怎样在多线程的程序中实现引用计数。


## 共用数据

我们来回到 [清单 15-5](./box-t.md#listing_15-5) 中的构造列表示例。回想一下，我们使用 `Box<T>` 定义了他。这一次，我们将创建两个列表，他们共用第三个列表的所有权。从概念上讲，这类似于下图 15-3。

<a name="f_15-3"></a>
![`b` 与 `c` 两个列表，共用了第三列表 `a` 的所有权](../images/15-03.svg)

**图 15-03**：两个列表`b` 与 `c`，共用第三个列表 `a` 的所有权

我们将创建包含 `5` 和 `10` 的列表 `a`。然后，我们将构造两个列表：以 `3` 开头的 `b` 和以 `4` 开头的 `c`。然后 `b` 和 `c` 两个列表都将接续到包含 `5` 和 `10` 的第一个列表 `a`。换句话说，这两个列表将共用包含 `5` 和 `10` 的第一个列表。

如清单 15-17 所示，尝试使用我们带有 `Box<T>` 的 `List` 定义来实现这一场景是行不通的。

<a name="listing_15-17"></a>
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

**清单 15-17**：演示我们不允许让两个使用 `Box<T>` 的列表，试图共用第三个列表的所有权

当我们编译这段代码时，我们会得到下面这个报错：

```console
$ cargo run
   Compiling cons-list v0.1.0 (/home/hector/rust-lang-zh_CN/projects/cons-list)
error[E0382]: use of moved value: `a`
  --> src/main.rs:11:30
   |
 9 |     let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
   |         - move occurs because `a` has type `List`, which does not implement the `Copy` trait
10 |     let b = Cons(3, Box::new(a));
   |                              - value moved here
11 |     let c = Cons(4, Box::new(a));
   |                              ^ value used here after move
   |
note: if `List` implemented `Clone`, you could clone the value
  --> src/main.rs:1:1
   |
 1 | enum List {
   | ^^^^^^^^^ consider implementing `Clone` for this type
...
10 |     let b = Cons(3, Box::new(a));
   |                              - you could clone this value

For more information about this error, try `rustc --explain E0382`.
error: could not compile `cons-list` (bin "cons-list") due to 1 previous error
```

`Cons` 变种拥有他们包含的数据，因此当我们创建列表 `b` 时，`a` 会被迁移到 `b` 中进而 `b` 拥有 `a`。然后，当我们在创建 `c` 时试图再次使用 `a` 时，我们不被允许，因为 `a` 已被迁移。

我们原本可以修改 `Cons` 的定义为包含引用，但随后我们将必须指定生命周期参数。通过指定生命周期参数，我们实际上是在指定列表中的每个元素，都将至少存活整个列表的寿命一样长。清单 15-17 中的元素与列表就是这种情况，但并非在所有情况下都如此。

相反，我们将修改 `List` 的定义为在 `Box<T>` 处使用 `Rc<T>`，如下清单 15-18 所示。现在每个 `Cons` 变种将包含一个值和一个指向 `List` 的 `Rc<T>`。当我们创建 `b` 时，我们将克隆 `a` 包含的 `Rc<List>`，而不是取得 `a` 的所有权，从而将引用计数从一增加到而，并让 `a` 和 `b` 共用该 `Rc<List>` 中的数据的所有权。在创建 `c` 时，我们也将克隆 `a`，将引用计数从二增加到三。每次我们调用 `Rc::clone` 时，到 `Rc<List>` 内数据的引用计数都将增加，并且除非对他的引用为零，否则数据不会被清理。

<a name="listing_15-18"></a>
文件名：`src/main.rs`

```rust
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}
```

**清单 15-18**：使用 `Rc<T>` 的 `List` 定义

我们需要添加一个 `use` 语句来带入 `Rc<T>` 到作用域，因为他不在前奏中。在 `main` 中，我们创建了包含 `5` 和 `10` 的列表并存储在 `a` 中的新 `Rc<List>` 中。然后，当我们创建 `b` 和 `c` 时，我们调用 `Rc::clone` 函数，并作为参数传递对 `a` 中的 `Rc<List>` 的引用。

我们本可以调用 `a.clone()` 而不是 `Rc::clone(&a)`，但是 Rust 的约定是在这种情况下要使用 `Rc::clone`。`Rc::clone` 的实现并不像大多数类型的 `clone` 实现那样，会构造所有数据的深度拷贝。对 `Rc::clone` 的调用只会增加引用计数，这不会花费太多时间。数据的深度拷贝会花费很多时间。通过使用 `Rc::clone` 进行引用计数，我们可以直观地区分深拷贝的克隆类别和增加引用计数的克隆类别。在查找代码中的性能问题时，我们只需考虑深拷贝的克隆，而可以忽略对 `Rc::clone` 的调用。

> **译注**：第 4 章的 [变量与数据相互作用：克隆](../ownership/about_ownership.md#变量与数据相互作用克隆) 小节中，我们曾提到：“当咱们看到对 clone 的调用时，咱们就知道一些任意代码正被执行，而这些代码可能开销高昂。这是一些不同寻常事情正在发生的直观指示器。”


## 通过克隆增加引用计数

我们来修改示例 15-18 中的工作的示例，以便可以看到在创建和删除对 `a` 中的 `Rc<List>` 的引用时，引用计数的变化。

在下面清单 15-19 中，我们将更改 `main`，使其具有一个围绕列表 `c` 的内层作用域；然后，我们可以看到当 `c` 超出作用域时，引用计数如何变化。

<a name="listing_15-19"></a>
文件名：`src/main.rs`

```rust
-- 跳过代码 --

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println! ("创建 a 后的计数 = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println! ("创建 b 后的计数 = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println! ("创建 c 后的计数 = {}", Rc::strong_count(&a));
    }

    println! ("c 超出作用域后的计数 = {}", Rc::strong_count(&a));
}
```

**清单 15-19**：打印引用计数

在程序中引用计数变化的每个点位，我们都打印引用计数，我们通过调用 `Rc::strong_count` 函数得到引用计数。这个函数之所以名为 `strong_count` 而不是 `count`，是因为 `Rc<T>` 类型还有个 `weak_count`；我们将在 [使用 `Weak<T>` 防止引用循环](./ref-cycles.md#使用-weakt-防止引用循环) 小节中看到 `weak_count` 的用途。

这段代码会打印以下内容：

```console
$ cargo run
   Compiling rc_demo v0.1.0 (/home/hector/rust-lang-zh_CN/projects/rc_demo)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/rc_demo`
创建 a 后的计数 = 1
创建 b 后的计数 = 2
创建 c 后的计数 = 3
c 超出作用域后的计数 = 2
```

我们可以看到，`a` 中的 `Rc<List>` 有着 1 的初始引用计数；然后，每次我们调用 `clone` 时，计数都会增加 1。当 `c` 超出作用域时，计数会减少 1。我们不必像必须调用 `Rc::clone` 来增加引用计数那样，调用函数来减少引用计数：`Drop` 特质的实现会在某个 `Rc<T>` 值超出作用域时，自动减少引用计数。

我们在这个例子中看不到的是，当 `b` 和 `a` 在 `main` 结束处超出作用域时，计数为 0，且 `Rc<List>` 会被彻底清理。使用 `Rc<T>` 允许单个值可以有多个所有者，而引用计数确保只要任何一个所有者仍然存在，该值就保持有效。

通过不可变引用，`Rc<T>` 允许咱们在程序的多个部分之间共用仅供读取的数据。若 `Rc<T>` 也允许咱们有着多个可变引用，咱们就会违反第 4 章中讨论的借用规则之一：对同一处的多个可变借用，会导致数据竞争和不一致。但能够修改数据是非常有用的！在下一小节中，我们将讨论内部可变性模式和 `RefCell<T>` 类型，咱们可以与 `Rc<T>` 结合使用该类型，以处理这一不可变性限制。


（End）


