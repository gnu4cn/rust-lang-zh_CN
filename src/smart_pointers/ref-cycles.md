# 引用环会泄露内存

Rust 的内存安全保证，使得意外创建出从不会被清理的内存（即 *内存泄漏，memory leak*）很难，但并非不可能。完全防止内存泄漏，并非 Rust 的保证之一，这意味着内存泄漏在 Rust 中也属于内存安全的。我们可以通过使用 `Rc<T>` 和 `RefCell<T>`，看到 Rust 允许内存泄漏：创建出一些其中项目以循环方式相互引用的引用是可能的。这会造成内存泄漏，因为循环中的各个项目的引用计数永远不会达到 0，进而值永远不会被弃用。


## 创建引用环

咱们从下面清单 15-25 中的 `List` 枚举和 `tail` 方法开始，看看引用环会怎样发生，以及如何防止他。

<a name="listing_15-25"></a>
文件名：`src/main.rs`

```rust
use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}
```

**清单 15-25**：一种包含 `RefCell<T>` 的构造列表定义，以便我们可以修改 `Cons` 变种引用的内容

我们正在使用 [清单 `15-5`](./box-t.md#listing_15-5) 中 `List` 定义中的另一种变种。 `Cons` 变种中的第二个元素，现在是 `RefCell<Rc<List>>`，这意味着我们不打算再像在示例 `15-24` 中那样，具备修改 `i32` 值的能力，而打算修改 `Cons` 变种指向的 `List` 值。我们还添加了个 `tail` 方法，以便在我们有个 `Cons` 变种时，方面我们访问第二个项目。

在下面清单 15-26 中，我们添加了个使用清单 15-25 中的定义的 `main` 函数。这段代码创建了一个变量 `a` 中的列表，以及一个变量 `b` 中的指向 `a` 中列表的列表。然后，他修改 `a` 中的列表指向了 `b`，从而创建一个引用环。其间有一些 `println!` 语句，显示这一过程中不同点的引用计数。


<a name="listing_15-26"></a>
文件名：`src/main.rs`

```rust
fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println! ("a 的初始 rc 计数 = {}", Rc::strong_count(&a));
    println! ("a 的下一条目 = {:?}", a.tail().unwrap());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println! ("b 创建后 a 的 rc 计数 = {}", Rc::strong_count(&a));
    println! ("b 的初始 rc 计数 = {}", Rc::strong_count(&b));
    println! ("b 的下一条目 = {:?}", b.tail().unwrap());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println! ("修改 a 之后 b 的 rc 计数 = {}", Rc::strong_count(&b));
    println! ("修改 a 之后 a 的 rc 计数 = {}", Rc::strong_count(&a));

    // 取消注释下一行，就可以看到我们有个循环；
    // 他将导致栈溢出。
    // println! ("a 的下一条目 = {:?}", a.tail());
}
```

**清单 15-26**：创建包含两个相互指向的 `List` 的引用环

> **译注**：其中 `a` 和 `b` 的下一条目应分别为 `a.tail().unwrap()` 和 `b.tail().unwrap()`。

我们以初始列表 `5, Nil` 在变量 `a` 中，创建包含 `List` 值的 `Rc<List>` 实例。然后，我们在变量 `b` 中创建一个包含另一 `List` 值的 `Rc<List>` 实例，该 `List` 值包含值 `10` 并指向 `a` 中的列表。

我们修改 `a` 使其指向 `b` 而不是 `Nil`，从而创建一个环。我们通过使用 `tail` 方法，获取对 `a` 中的 `RefCell<Rc<List>>` 的引用，并放置该引用于变量 `link` 中。然后，我们对 `RefCell<Rc<List>>` 使用 `borrow_mut` 方法，将内部的值从一个包含 `Nil` 值的 `Rc<List>`，修改为 `b` 中的 `Rc<List>`。

当我们运行这段代码时，暂时保留最后一个 `println!` 注释掉，我们将得到以下输出：

```console
$ cargo run
   Compiling cons-list v0.1.0 (/home/hector/rust-lang-zh_CN/projects/cons-list)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/cons-list`
a 的初始 rc 计数 = 1
a 的下一条目 = RefCell { value: Nil }
b 创建后 a 的 rc 计数 = 2
b 的初始 rc 计数 = 1
b 的下一条目 = RefCell { value: Cons(5, RefCell { value: Nil }) }
修改 a 之后 b 的 rc 计数 = 2
修改 a 之后 a 的 rc 计数 = 2
```

当我们修改 `a` 中的列表为指向 `b` 后，`a` 和 `b` 中的 `Rc<List>` 实例的引用计数均为 2。在 `main` 结束处，Rust 弃用了变量 `b`，这会将 `b` 中的 `Rc<List>` 实例的引用计数从 2 减少到 1。`Rc<List>` 在堆上的内存此时不会被弃用，因为他的引用计数为 1 而不是 0。然后，Rust 弃用 `a`，这会将 `a` 中的 `Rc<List>` 实例的引用计数从 2 减少到 1。这个实例的内存也无法弃用，因为另一 `Rc<List>` 实例仍指向他。分配给该列表的内存，将永远保持未回收状态。为了直观地展示这种引用环，我们创建了下图 15-4 中的图表。

<a name="f_15-4"></a>
![相互指向的列表 `a` 与 `b` 的一个循环引用](../images/15-04.svg)

**图 15-4**：列表 `a` 和 `b` 相互指向的引用环

当咱们取消注释最后一个 `println！` 并运行程序时，Rust 尝试打印这个循环，其中 `a` 指向 `b` 指向 `a`，如此循环反复，直至栈溢出。

与现实世界的程序相比，这个示例中的创建引用环的后果并不可怕：在我们创建引用环之后，程序就结束了。然而，当某个更复杂的程序在循环中分配了大量的内存并长期占用这些内存时，程序将使用比所需的更多内存，而可能使系统不堪重负，导致其耗尽可用内存。

创建引用环并不容易，但也不是不可能。当咱们有着包含 `Rc<T>` 值的 `RefCell<T>` 值，或有着类似的带有内部可变性和引用计数的嵌套组合时，就必须确保没有创建循环；咱们不能指望 Rust 来捕获他们。创建引用环属于程序中的逻辑错误，咱们应使用自动化测试、代码审查即其他软件开发实践，来最大程度地减少这种错误。

避免引用环的另一种解决方案是重新组织数据结构，从而使一些引用表达所有权，而另一些引用不表达所有权。因此，咱们可以有着由一些所有权关系，和一些非所有权关系组成的循环，而只有所有权关系会影响某个值是否可以被弃用。在 [清单 15-25](#listing_15-25) 中，我们总是想要 `Cons` 变种拥有他们的列表，所以重新组织数据结构是不可行的。我们来看一个使用由父节点和子节点组成的图的示例，以了解什么时候非所有权关系，是防止引用环的合适方式。


## 使用 `Weak<T>` 防止引用环

到目前为止，我们已经演示了调用 `Rc::clone` 会增加 `Rc<T>` 实例的 `strong_count`，而 `Rc<T>` 实例只有在其 `strong_count` 为 0 时才会被清理。咱们还可以通过调用 `Rc::downgrade` 并传递对 `Rc<T>` 的引用，创建对 `Rc<T>` 实例中值的 *弱引用，weak reference*。*强引用* 属于咱们共用 `Rc<T>` 实例所有权的方式。*弱引用* 则不表达所有权关系，进而他们的计数不影响 `Rc<T>` 实例何时被清理。他们不会导致引用环，因为任何涉及弱引用的循环，都将在所涉及的值的强引用计数为 0 时被打破。

当咱们调用 `Rc::downgrade` 时，咱们会得到一个 `Weak<T>` 类型的灵巧指针。调用 `Rc::downgrade` 不是将 `Rc<T>` 实例中的 `strong_count` 增加 1，而会将 `weak_count` 增加 1。与 `strong_count` 类似，`Rc<T>` 类型使用 `weak_count` 来跟踪存在多少个 `Weak<T>` 引用。区别在于，在 `Rc<T>` 实例的 `weak_count` 无需为 `0` 即可被清理。

由于 `Weak<T>` 引用的值可能已被弃用，因此要对 `Weak<T>` 指向的值执行任何操作，咱们都必须确保该值仍然存在。咱们可通过调用 `Weak<T>` 实例上的 `upgrade` 方法来做到这点，其将返回一个 `Option<Rc<T>>`。当 `Rc<T>` 值尚未被弃用时，咱们将得到 `Some` 的结果；当 `Rc<T>` 值已被弃用时，咱们将得到 `None` 的结果。因为 `upgrade` 返回 `Option<Rc<T>>`，Rust 将确保 `Some` 情形和 `None` 情形都得到处理，进而将不存在无效指针。

举个例子，我们将不再使用其项目仅了解下一个项目的列表，而将创建一棵树，他的项目了解其子项目 *和* 其父项目。


### 创建树形数据结构

首先，我们将构建一棵树，其中的节点了解他们的子节点。我们将创建一个名为 `Node` 的结构体，保存自己的 `i32` 值以及到其子 `Node` 值的引用：

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

我们希望 `Node` 拥有他的子节点，并且我们希望与变量共用这一所有权，以便我们可以直接访问树中的每个 `Node`。为此，我们定义 `Vec<T>` 中的项目为 `Rc<Node>` 类型的值。我们还希望修改哪些节点是另一节点的子节点，因此我们在 `children` 字段中有个围绕 `Vec<Rc<Node>>` 的  `RefCell<T>`。

接下来，我们将使用我们的结构体定义，并以值 `3` 及没有子节点，创建一个名为 `leaf` 的 `Node` 实例；并以值 `5` 和 `leaf` 作为其子节点，创建另一个名为 `branch` 的实例，如下清单 15-27 中所示。

<a name="listing_15-27"></a>
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

**清单 15-27**：创建一个没有子节点的 `leaf` 节点，以及一个以 `leaf` 作为其子节点之一的 `branch` 节点

我们克隆 `leaf` 中的 `Rc<Node>` 并存储在 `branch` 中，这意味着 `leaf` 中的 `Node` 现在有两个所有者：`leaf` 和 `branch`（译注：可调用 `Rc::strong_count(&leaf)` 查看 `leaf` 的引用计数）。我们可以通过 `branch.children`，从 `branch` 到达 `leaf`，但是没有办法从 `leaf` 到达 `branch`。原因是 `leaf` 没有到 `branch` 的引用，而不知道他们是相关的。我们希望 `leaf` 知道 `branch` 是他的父节点。接下来我们将实现这点。


### 在子节点中添加到父节点的引用

为了让子节点知道他的父节点，我们需要添加一个 `parent` 字段到我们的 `Node` 结构体定义。难点在于确定 `parent` 应为何种类型。我们知道他不能包含 `Rc<T>`，因为这将以 `leaf.parent` 指向 `branch`，而 `branch.children` 指向 `leaf` 创建一个引用环，这将导致他们的 `strong_count` 值永远不会为 0。

从另一角度考虑这些关系，父节点应拥有他的子节点：当父节点被弃用时，他的子节点也应该被弃用。然而，子节点不应拥有他的父节点：当我们弃用子节点时，父节点应该仍然存在。这正是弱引用的情形！

因此，我们将使用 `Weak<T>` 而不是 `Rc<T>` 构造 `parent` 的类型，具体来说是 `RefCell<Weak<Node>>`。现在我们的 `Node` 结构体定义看起来像下面这样：


文件名：`src/main.rs`

```rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
```

节点将能够引用其父节点，但不会拥有其父节点。在下面清单 15-28 中，我们更新 `main` 为使用这个新定义，以便 `leaf` 节点将有一种引用其父节点 `branch` 的方式。

<a name="listing_15-28"></a>
文件名：`src/main.rs`

```rust
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec! []),
    });

    println! ("leaf 的父节点 = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec! [Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println! ("leaf 的父节点 = {:?}", leaf.parent.borrow().upgrade());
}
```

**清单 15-28**：有着对其父节点 `branch` 的弱引用的 `leaf` 节点

创建 `leaf` 节点看起来与清单 15-27 相似，除了 `parent` 段外：`leaf` 一开始没有父节点，因此我们创建一个新的、空 `Weak<Node>` 引用实例。

此时，当我们尝试通过使用 `upgrade` 方法获取对 `leaf` 的父节点的引用时，我们得到 `None` 值。我们在第一个 `println!` 语句的输出中，看到了这点：


```console
leaf 的父节点 = None
```

当我们创建 `branch` 节点时，他在 `parent` 字段中也将有个新的 `Weak<Node>` 引用，因为 `branch` 没有父节点。我们仍然将 `leaf` 作为 `branch` 的子节点之一。一旦我们有了 `branch` 中的 `Node` 实例，我们就可以修改 `leaf`，以给予他一个到其父节点的 `Weak<Node>` 引用。我们对 `leaf` 的 `parent` 字段中的 `RefCell<Weak<Node>` 使用 `borrow_mut` 方法，然后使用 `Rc::downgrade` 函数，从 `branch` 中的 `Rc<Node>` 创建对 `branch` 的 `Weak<Node>` 引用。

当我们再次打印 `leaf` 的父节点时，这次我们将得到一个包含 `branch` 的 `Some` 变种：现在 `leaf` 可以访问他的父节点了! 当我们打印 `leaf` 时，我们还避免了我们在清单 15-26 中遇到的那样，最终以栈溢出结束的循环；`Weak<Node>` 的引用会被打印为 `(Weak)`：


```console
leaf 的父节点 = Some(Node { value: 5, parent: RefCell { value: (Weak) }, children: RefCell { value: [Node { value: 3, parent: RefCell { value: (Weak) }, children: RefCell { value: [] } }] } })
```

没有无限输出表明，这段代码没有创建引用环。我们也可以通过查看调用 `Rc::strong_count` 和 `Rc::weak_count` 得到的值来判断这点。


### 可视化 `strong_count` 与 `weak_count` 的变化

咱们来通过创建一个新的内层作用域，并迁移 `branch` 的创建到该作用域中，看看 `Rc<Node>` 实例的 `strong_count` 和 `weak_count` 值会如何变化。通过这样做，我们可以看到在 `branch` 被创建时，以及当他超出作用域而被弃用时，分别会发生什么。相关修改如下清单 15-29 中所示。

<a name="listing_15-29"></a>
文件名：`src/main.rs`

```rust
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec! []),
    });

    println! ("leaf 的强引用计数 = {}，弱引用计数 = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec! [Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println! ("branch 的强引用计数 = {}，弱引用计数 = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );

        println! ("leaf 的强引用计数 = {}，弱引用计数 = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
    }

    println! ("leaf 的父节点 = {:?}", leaf.parent.borrow().upgrade());

    println! ("leaf 的强引用计数 = {}，弱引用计数 = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
}
```

**清单 15-29**：在内层作用域中创建 `branch`，并检查强引用计数和弱引用计数

`leaf` 创建后，他的 `Rc<Node>` 有着 1 的 `strong count`，0 的 `weak count`。在内层作用域中，我们创建 `branch` 并将其与 `leaf` 关联，在我们打印计数处，`branch` 中的 `Rc<Node>` 将有着 1 的强引用计数和 1 的弱引用计数（因为 `leaf.parent` 以一个 `Weak<Node>` 指向 `branch`）。当我们打印 `leaf` 中的计数时，我们将看到他有着 2 的强引用计数，因为 `branch` 现在有着存储在 `branch.children` 字段中 `leaf` 的 `Rc<Node>` 的克隆，但 `leaf` 仍将有着 0 的弱引用计数。

在内层作用域结束后，`branch` 超出作用域，进而 `Rc<Node>` 的强引用计数降为 0，所以他的 `Node` 被弃用。来自 `leaf.parent` 的弱引用计数 1 并不影响 `Node` 是否被弃用，因此我们没有任何内存泄露！

当我们尝试在这个作用域结束后访问 `leaf` 的父节点时，我们将再次得到 `None`。在程序的结束处，`leaf` 中的 `Rc<Node>` 有着 1 的强引用计数，弱计数为 `0`，因为现在变量 `leaf` 再次成为对这个 `Rc<Node>` 的唯一引用。

所有管理引用计数与值弃用的逻辑，都内置于 `Rc<T>` 和 `Weak<T>` 及其 `Drop` 特质的实现中。通过在 `Node` 的定义中指定子节点到其父节点的关系应为 `Weak<T>` 引用，咱们可以在不会创建引用环和内存泄漏下，让父节点指向子节点，反之亦然。


# 本章小结

这一章介绍了如何使用灵巧指针，来做出与 Rust 在默认情况下以普通引用所做出的不同保证和权衡。`Box<T>` 类型有着已知大小，并指向堆上分配的数据。`Rc<T>` 类型记录堆上数据的引用的数量，以便数据可以有多个所有者。`RefCell<T>` 类型通过其内部可变性，给予我们一种可以在需要不可变类型，但又需要改变该类型的内部值时使用的类型；他还会在运行时而不是在编译时强制执行借用规则检查。

此外，这一章还讨论了 `Deref` 和 `Drop` 两个特质，他们实现了灵巧指针的大部分功能。我们探讨了可能导致内存泄露的引用环，以及怎样使用 `Weak<T>` 来防止他们。

若这一章引起了咱们的兴趣，并且咱们打算实现自己的灵巧指针，请查看 ["The Rustonomicon"](https://doc.rust-lang.org/nomicon/index.html) ，以获取更多有用的信息。

接下来，我们将讨论 Rust 中的并发问题。咱们甚至还将了解到一些新的灵巧指针。


（End）


