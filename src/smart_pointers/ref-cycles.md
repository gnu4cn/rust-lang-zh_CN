# 引用循环会泄露内存

**Reference Cycles Can Leak Memory**


Rust 的内存安全保证，使得意外创建出从未清理过的内存（称为 *内存泄漏，memory leak*）很难，但并非不可能。完全防止内存泄漏不是 Rust 的保证之一，这意味着内存泄漏在 Rust 中是内存安全的。通过使用 `Rc<T>` 和 `RefCell<T>`，我们可以看到 Rust 允许内存泄漏：创建出其中项目在循环中相互指向的引用是可能的。这会造成内存泄漏，因为循环中各个项目的引用计数永远不会达到 0，而值永远不会被弃用。


## 创建引用循环

**Creaing a Reference Cycle**


咱们以清单 15-25 中的 `List` 枚举和 `tail` 方法开始，来看看循环引用是如何发生的，以及怎样防止他：


文件名：`src/main.rs`

```rust
{{#include ../../projects/ref_cycle_demo/src/main.rs::18}}
```

*清单 15-25：包含 `RefCell<T>` 的构造列表定义，因此我们可以修改 `Cons` 变种指向的内容*

我们正在使用清单 `15-5` 中 `List` 定义的另一种变体。 `Cons` 变种中的第二个元素现在是 `RefCell<Rc<List>>`，这意味着我们不像在示例 `15-24` 中那样能够修改 `i32` 值，我们打算修改 `Cons` 变种指向的 `List` 值。我们还添加了一个 `tail` 方法，以便在我们有 `Cons` 变种时方便地访问第二个项目。

在下面清单 15-26 中，咱们添加了用到清单 15-25 中那些定义的 `main` 函数。此代码创建了变量 `a` 中的一个清单，以及变量 `b` 中指向 `a` 中清单的一个清单。随后他将 `a` 中的清单指向了 `b`，这就创建了一个循环引用。其间有着一些 `println!` 语句，来显示此过程中不同点位的那些引用计数。


文件名：`src/main.rs`

```rust
{{#include ../../projects/ref_cycle_demo/src/main.rs:20:}}
```

*清单 15-26：创建出相互指向的两个 `List` 的循环引用*

我们创建一个 `Rc<List>` 实例，在变量 `a` 中持有一个 `List` 值，初始列表为 `5， Nil`。然后我们创建一个 `Rc<List>` 实例，在变量 `b` 中保存另一个 `List` 值，其中包含值 `10`，并指向 `a` 中的列表。

我们修改 `a` 使其指向 `b` 而不是 Nil，从而创建一个循环。为此，我们使用 `tail` 方法获取对 `a` 中 `RefCell<Rc<List>>` 的引用，我们将其放入变量 `link` 中。然后我们使用 `RefCell<Rc<List>>` 上的 `borrow_mut` 方法，将里面的值从一个持有 `Nil` 值的 `Rc<List>` 更改为 `b` 中的 `Rc<List>`。

咱们暂时保持最后的 `println!` 注释掉，而运行此代码时，咱们将得到下面的输出：

```console
$ cargo run
   Compiling ref_cycle_demo v0.1.0 (/home/lennyp/rust-lang/ref_cycle_demo)
    Finished dev [unoptimized + debuginfo] target(s) in 1.20s
     Running `target/debug/ref_cycle_demo`
a 的初始 rc 计数 = 1
a 的下一条目 = Some(RefCell { value: Nil })
b 的创建后 a 的 rc 计数 = 2
b 的初始 rc 计数 = 1
b 的下一条目 = Some(RefCell { value: Cons(5, RefCell { value: Nil }) })
在修改 a 之后 b 的 rc 计数 = 2
在修改 a 之后 a 的 rc 计数 = 2
```

在咱们将 `a` 中的列表改为指向 `b` 后，`a` 和 `b` 中的 `Rc<List>` 实例的引用计数均为 `2`。在 `main` 的最后，Rust 弃用了变量 `b`，这使得 `b` 中的 `Rc<List>` 实例的引用计数从 `2` 减少到 `1`。`Rc<List>` 在内存堆中的内存此时不会被弃用，因为其引用计数为 `1` 而不是 `0`。然后 Rust 弃用 `a`，将 `a` 中的 `Rc<List>` 实例的引用计数从 `2` 减少到 `1`。由于另一 `Rc<List>` 实例仍指向他，因此该实例的内存也不能被弃用。分配给列表内存将永远保持未被收集的状态。为直观地表示这个引用循环，咱们创建了下图 15-4 中的图表。

![相互指向的列表 `a` 与 `b` 的一个循环引用](images/15-04.svg)

*图 15-04：列表 `a` 和 `b` 相互指向的引用循环*

如果咱们取消对最后一个 `println！` 的注释并运行程序，Rust 会尝试打印这个循环，`a` 指向 `b` 指向 `a`，如此反复，直到程序溢出栈，overflows the stack。

与现实世界的程序相比，在这个例子中创建一个引用循环的后果并不可怕：在我们创建引用循环之后，程序就结束了。然而，如果某个更复杂的程序在一个循环中分配了大量的内存，并长期占用这些内存，那么这个程序将使用比其需要的更多的内存，并可能使系统不堪重负，导致其可用内存耗尽。

创建引用循环不容易做到，但也不是不可能。如果咱们有着其中包含 `Rc<T>` 值的 `RefCell<T>` 值，或类似的带有内部可变性与引用计数类型的嵌套组合，咱们必须确保不创建循环；咱们不能依靠 Rust 来捕获他们。创建引用循环将是咱们程序中的逻辑错误，咱们应该使用自动测试、代码审查和其他软件开发实践来减少这种错误。

另一避免引用循环的办法是重新组织咱们的数据结构，使一些引用表达所有权，而一些引用不表达。由此，咱们可以有由一些所有权关系和一些非所有权关系组成的循环，而只有所有权关系会影响一个值是否可以被丢弃。在清单 15-25 中，我们总是希望 `Cons` 变体拥有他们的列表，所以重新组织数据结构是不可能的。咱们来看一个使用由父节点和子节点组成的图的示例，看看什么时候非所有权关系是防止引用循环的合适方式。


## 防止引用循环：将 `Rc<T>` 变为 `Weak<T>`

**Preventing Referencing Cycles: Turning an `Rc<T>` into `Weak<T>`**


到目前为止，我们已经证明了调用 `Rc::clone` 会增加 `Rc<T>` 实例的 `strong_count`，而 `Rc<T>` 实例只有在其 `strong_count` 为 `0` 时才会被清理掉。咱们还可以通过调用 `Rc::downgrade` 并传递对 `Rc<T>` 的引用，来创建对 `Rc<T>` 实例中值的 *弱引用，weak reference*。强引用是咱们共用 `Rc<T>` 实例所有权的方式。弱引用不表达所有权关系，他们的计数不会影响 `Rc<T>` 实例被清理的时间。他们不会引起引用循环，因为任何涉及弱引用的循环，都会在所涉及的值的强引用计数为 `0` 时被打破。

当咱们调用 `Rc::downgrade` 时，咱们会得到一个 `Weak<T>` 类型的灵巧指针。调用 `Rc::downgrade` 不是将 `Rc<T>` 实例中的 `strong_count` 增加 `1`，而是将 `weak_count` 增加 `1`。与 `strong_count` 类似，`Rc<T>` 类型使用 `weak_count` 来记录存在多少个 `Weak<T>` 引用。不同的是，在 `Rc<T>` 实例被清理时，`weak_count` 不需要为 `0`。

由于 `Weak<T>` 所引用的值可能已被弃用，因此要对 `Weak<T>` 所指向的值执行任何操作，咱们都必须确保该值仍然存在。通过在 `Weak<T>` 实例上调用 `upgrade` 方法来做到这一点，他将返回一个 `Option<Rc<T>>`。如果 `Rc<T>` 的值还没有被弃用，咱们将得到一个 `Some` 的结果；如果 `Rc<T>` 的值已被弃用，咱们将得到一个 `None` 的结果。因为 `upgrade` 返回的是 `Option<Rc<T>>`，Rust会确保 `Some` 和 `None` 的情况都得到处理，而且不会出现无效的指针。

作为一个例子，我们不是使用其项目只知道下一项目的列表数据结构，而是将创建一个其项目了解其子项目 *及* 其父项目的树。


### 创建一个树形数据结构：带有子节点的节点

**Creating a Tree Data Structure: a Node with Child Nodes**

首先，我们将构建一棵树，其中的节点知道他们的子节点。我们将创建一个名为 `Node` 的结构体，他拥有自己的 `i32` 值以及对其子 `Node` 值的引用：

文件名：`src/main.rs`

```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}
```

我们希望 `Node` 拥有他的子节点，并且我们希望与变量共用该所有权，以便咱们可以直接访问树中的每个 `Node`。为此，我们将 `Vec<T>` 项定义为 `Rc<Node>` 类型的值。我们还打算修改那些是另一节点的子节点的节点，因此我们在 `Vec<Rc<Node>>` 周围、`children` 字段中有一个 `RefCell<T>`。

接下来，我们将使用我们的结构体定义，创建一个名为 `leaf` 的 `Node` 实例，其值为 `3`，没有子节点；另一个名为 `branch` 的实例，其值为 `5`，`leaf` 是其子节点之一，如下清单 15-27 所示：

文件名：`src/main.rs`

```rust
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec! []),
    });

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec! [Rc::clone(&leaf)]),
    });
}
```

*清单 15-27：创建一个没有子节点的 `leaf` 节点和一个以 `leaf` 作为其子节点之一的 `branch` 节点*

我们克隆了 `leaf` 中的 `Rc<Node>` 并将其存储在 `branch` 中，这意味着 `leaf` 中的 `Node` 现在有两个所有者：`leaf` 和 `branch`。我们可以通过 `branch.children` 从 `branch` 获取到 `leaf`，但是没有办法从 `leaf` 获取到 `branch`。原因是 `leaf` 没有对 `branch` 的引用，不知道他们之间的关系。我们想让 `leaf` 知道 `branch` 是他的父节点。下一步我们将这样做。


### 在子节点中添加到其父节点的引用

**Adding a Reference from a Child to Its Parent**


为了让子节点知道他的父节点，我们需要在我们的 `Node` 结构体定义中添加一个父节点字段。问题在于确定出父节点的类型是什么。我们知道他不能包含一个 `Rc<T>`，因为这将创建一个引用循环，即 `leaf.parent` 指向 `branch`，而 `branch.children` 指向 `leaf`，这将导致他们的 `strong_count` 值永远为 0。

以另一种方式思考这些关系，一个父节点应该拥有他的子节点：如果一个父节点被弃用，他的子节点也应该被弃用。然而，一个子节点不应该拥有他的父节点：如果我们弃用某个子节点，父节点应该仍然存在。这就是弱引用的情况！

因此，我们将使用 `Weak<T>` 代替 `Rc<T>`，具体来说是 `RefCell<Weak<Node>>`。现在我们的节点结构定义如下所示：


文件名：`src/main.rs`

```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
```

节点将能够引用其父节点但不拥有其父节点。在下面清单 15-28 中，我们更新了 `main` 以使用这个新定义，这样 `leaf` 节点将有办法引用其父节点 `branch`：

文件名：`src/main.rs`

```rust
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec! []),
    });

    println! ("叶子节点的父节点 = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec! [Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println! ("叶子节点的父节点 = {:?}", leaf.parent.borrow().upgrade());
}
```

*清单 15-28：对其父节点 `branch` 有弱引用的 `leaf` 节点*

创建 `leaf` 节点看起来与清单 15-27 相似，除了父字段：`leaf` 开始时没有父节点，所以我们创建一个新的、空的 `Weak<Node>` 引用实例。

此时，当我们试图通过使用 `upgrade` 方法来获得对 `leaf` 的父节点的引用时，我们得到的是一个 `None` 值。我们在第一个 `println!` 语句的输出中看到了这一点：


```console
叶子节点的父节点 = None
```

当我们创建 `branch` 节点时，他在 `parent` 字段中也会有一个新的 `Weak<Node>` 引用，因为 `branch` 没有父节点。我们仍然将 `leaf` 作为 `branch` 的子节点之一。一旦我们在 `branch` 中有了 `Node` 实例，我们就可以修改 `leaf` 来给他一个到其父节点的 `Weak<Node>` 引用。我们在 `leaf` 的 `parent` 字段中的 `RefCell<Weak<Node>` 上使用 `borrow_mut` 方法，然后我们使用 `Rc::downgrade` 函数从 `branch` 中的 `Rc<Node>` 创建一个对 `branch` 的 `Weak<Node>` 引用。

当我们再次打印 `leaf` 的父节点时，这次我们会得到一个持有 `branch` 的 `Some` 变体：现在 `leaf` 可以访问他的父节点了! 当我们打印 `leaf` 时，我们也避免了像清单 15-26 中那样最终以栈溢出结束的循环；`Weak<Node>` 引用被打印为 `（Weak）`：


```console
叶子节点的父节点 = Some(Node { value: 5, parent: RefCell { value: (Weak) },
children: RefCell { value: [Node { value: 3, parent: RefCell { value: (Weak) },
children: RefCell { value: [] } }] } })
```

没有无限的输出表明这段代码没有创建引用循环。我们也可以通过查看调用 `Rc::strong_count` 和 `Rc::weak_count` 得到的值来判断这一点。


### 可视化 `strong_count` 和 `weak_count` 的变化

**Visualizing Changes to `strong_count` and `weak_count`**


咱们来看看通过创建一个新的内部作用域并将 `branch` 的创建移到该作用域中，`Rc<Node>` 实例的 `strong_count` 和 `weak_count` 值如何变化。通过这样做，我们可以看到在 `branch` 被创建后，当他离开作用域时，会发生什么。修改部分如下清单 15-29 所示：

文件名：`src/main.rs`

```rust
{{#include ../../projects/tree_demo/src/main.rs:11:}}
```

*清单 15-29：在内层作用域中创建 `branch` 并检查强引用和弱引用计数*

`leaf` 创建后，其 `Rc<Node>` 的 `strong count` 为 `1`，`weak count` 为 `0`。

在内层作用域中，我们创建了 `branch` 并将其与 `leaf` 关联，此时当我们打印计数时， `branch` 中的 `Rc<Node>` 将有 `1` 的强计数和 `1` 的弱计数（因为 `leaf.parent` 指向 `branch` 的是 `Weak<Node>`）。当我们打印 `leaf` 中的计数时，我们将看到他的强计数为 `2`，因为 `branch` 现在有一个保存在 `branch.children` 中的 `leaf` 的 `Rc<Node>` 的克隆，但仍然会有一个弱计数为 `0`。

当内层作用域结束时，`branch` 超出作用域，`Rc<Node>` 的强计数减少到 `0`，所以他的 `Node` 被弃用。来自 `leaf.parent` 的弱计数 `1` 对 `Node` 是否被弃用没有影响，所以我们没有任何内存泄露！

如果我们在作用域结束后试图访问 `leaf` 的父节点，我们会再次得到 `None`。在程序结束时，`leaf` 中的 `Rc<Node>` 的强计数为 `1`，弱计数为 `0`，因为现在变量 `leaf` 又是对 `Rc<Node>` 的唯一引用。

所有管理计数和值弃用的逻辑，都内置于 `Rc<T>` 和 `Weak<T>` 及他们的 `Drop` 特质实现中。通过在 `Node` 的定义中指定子节点与其父节点的关系应是 `Weak<T>` 引用，咱们可以让父节点指向子节点，反之亦然，而不会产生引用循环和内存泄漏。


# 本章小结

本章介绍了如何使用灵巧指针来进行与 Rust 默认的普通引用不同的保证和取舍。`Box<T>` 类型有一个已知的大小，指向在内存堆上分配的数据。`Rc<T>` 类型记录了对内存堆上数据的引用数量，因此数据可以有多个所有者。`RefCell<T>` 类型及其内部可变性为我们提供了一种类型，当我们需要不可变类型但需要改变该类型的内部值时，我们可以使用这种类型；他还在运行时而不是在编译时强制执行借用规则。

我们还讨论了 `Deref` 和 `Drop` 特质，这两个特质实现了灵巧指针的很多功能。我们探讨了可能导致内存泄露的引用循环以及如何使用 `Weak<T>` 来防止它们。

如果本章引起了你的兴趣，并且你想实现你自己的智能指针，请查看 ["The Rustonomicon"](https://doc.rust-lang.org/nomicon/index.html) ，以获得更多有用的信息。

接下来，我们将讨论 Rust 中的并发问题。咱们甚至会了解到一些新的灵巧指针。


（End）


