# 使用 `Box<T>` 指向内存堆上的数据

最直接了当的灵巧指针，便是 *匣子，box* ，其类型写为 `Box<T>`。匣子允许咱们将数据存储在堆上，而非在栈上。留在栈上的指向堆数据的指针。请参考第 4 章，回顾一下栈与堆的区别。

除了在内存堆而不是栈上存储数据外，匣子数据结构并无性能方面的开销。但他们也没有很多额外能力。在下面这些情况下，咱们将经常用到他们：

- 当咱们有个在编译时无法知道其大小的类型，并且咱们想在需要精确大小的上下文中使用该类型的值时，when you have a type whose size can't be known at compile time and you want to use a value of that type in a context that requires an exact size;
- 当咱们有着大量数据，而想转移所有权，但要确保转移时数据不会被拷贝时，when you have a large amount of data and you want to transfer ownership but ensure the data won't be copied when you do so；
- 当咱们想拥有某个值，却只关心他是个实现了特定特质的类型，而非某个特定类型时，when you want to own a value and you care only that it's a type that implements a particular trait rather than being of a specific type。

我们将在 [“用匣子数据结构得到递归类型”](#用匣子数据结构得到递归类型) 小节演示第一种情况。在第二种情形下，转移大量数据的所有权可能需要很长的时间，因为数据在堆栈上被拷过来拷过去。为改进这种情况下的性能，咱们可以将内存堆上的大量数据，存储在一个匣子中。尔后，在栈上便只有少量的指针数据被拷来拷去了，同时其所引用的数据，则会保持在堆上的一处。第三种情况被称为 *特质对象，trait object*，第 17 章用了一整节，[“使用允许不同类型值的特质对象”](Ch17_Object_Oriented_Programming_Features_of_Rust.md#使用允许不同类型值的特质对象)，来讨论这个话题。因此，咱们在这里学到的东西，将在第 17 章中再度得到应用！

### 使用 `Box<T>` 在内存堆上存储数据

在咱们讨论 `Box<T>` 的内存堆存储用例之前，咱们将介绍其语法与怎样与存储在 `Box<T>` 中的值交互。

下面清单 15-1 展示了如何使用匣子，在内存堆上存储一个 `i32` 的值：

文件名：`src/main.rs`

```rust
fn main() {
    let b = Box::new(5);
    println! ("b = {}", b);
}
```

*清单 15-1：使用匣子在内存堆上存储一个 `i32` 值*

我们将变量 `b` 定义为具有指向值 `5` 的 `Box` 的值，该值分配在堆上。此程序将打印出 `b = 5`；在这种情况下，咱们可以像访问栈上的数据一样，访问匣子中的数据。就像任何拥有的值一样，当某个匣子超出作用域时，如同 `b` 在 `main` 结尾处那样，他将被解除内存分配。解除内存分配，同时发生在匣子（存储在栈上）和他指向的数据（存储在内存堆上）。


## 用匣子数据结构得到递归类型

**Enabling Recursive Types with Boxes**


*递归类型，recursive type* 的值可以有另一个相同类型的值作为其自身的一部分。递归类型带来了一个问题，因为在编译时 Rust 需要知道一个类型占用了多少空间。然而，理论上递归类型的值的嵌套可以无限地继续下去，所以 Rust 无法知道值需要多少空间。因为盒子有一个已知的大小，我们可以通过在递归类型定义中插入一个盒子来得到递归类型。

作为一个递归类型的示例，咱们来探讨一下 *构造列表，cons list*（the *cons* tructs *list*）。这是函数式编程语言中常见的一种数据类型。除了其中的递归之外，咱们将定义的构造列表类型是简单明了的；因此，当咱们遇到涉及递归类型的更复杂情况时，我们将使用的例子中的概念会很有用。


### 构造列表的更多信息

所谓 *构造列表，cons list*，是来自 Lisp 编程语言及其方言的一种数据结构，由嵌套对组成，是 Lisp 版本的链表，is made up of nested pairs, and is the Lisp version of a linked list。其名称来自于 Lisp 的 `cons` 函数（是构造函数，construct function，的简称），该函数从其两个参数，构造出一个新嵌套对。通过对由一个值与另一嵌套对组成的嵌套对上调用 `cons`，咱们便可构造出由递归嵌套对组成的构造链表。

例如，下面是一个包含列表 1、2、3 的构造列表的伪代码表示，每个嵌套对都在括号里：

```lisp
(1, (2, (3, Nil)))
```

构造列表中的每个条目都包含了两个元素：当前条目的值与下一条目。列表中最后条目，只包含名为 `Nil` 的值，而没有下一条目。构造列表是由递归调用 `cons` 函数产生的。表示递归基础的规范名称是 `Nil`，the canonical name to denote the base case of the recursion is `Nil`。请注意，这与第 6 章中的 “null” 或 “nil” 概念不同，后者是一个无效或不存在的值。

在 Rust 中，构造列表并不是一种常用的数据结构。大多数时候，当咱们在 Rust 中有一个条目清单时，`Vec<T>` 是一个更好的选择。那么别的时候的各种情况下，更复杂的递归数据类型，*则都是* 有用的，而在本章中以构造列表开始，咱们便可专心探讨匣子数据结构如何让我们定义出递归数据类型。

下面清单 15-2 包含了构造列表的一种枚举定义。请注意，这段代码还不能编译，因为 List 类型没有一个已知的大小，我们将证明这一点。

文件名：`src/main.rs`

```rust
enum List {
    Cons(i32, List),
    Nil,
}
```

*清单 15-2：第一次尝试定义一个枚举来表示 `i32` 值的构造列表数据结构*

> 注意：出于这个示例的目的，咱们正在实现一个仅包含 `i32` 值的构造列表。咱们本可以使用泛型来实现他，就像咱们在第 10 章中讨论的那样，定义出一个可存储任何类型值的构造列表。

使用 `List` 类型来存储列表 `1, 2, 3`，看起来就会像下面清单 15-3 中的代码：

文件名：`src/main.rs`

```rust
use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
```

*清单 15-3：使用 `List` 枚举来存储列表 `1, 2, 3`*

其中第一个 `Cons` 保存着 `1` 与另一 `List` 值。这个 `List` 值是另一个 `Cons` 值，保存了 `2` 与另一 `List`。这个 `List` 值则是又一个 `Cons` 值，其保存了 `3` 与一个为 `Nil` 的最后 `List` 值，这个非递归变种表示列表的结束。

如果我们尝试编译清单 15-3 中的代码，我们会得到下面清单 15-4 中的报错：

```console
$ cargo run
   Compiling sp_demos v0.1.0 (/home/lennyp/rust-lang/sp_demos)
error[E0072]: recursive type `List` has infinite size
 --> src/main.rs:1:1
  |
1 | enum List {
  | ^^^^^^^^^ recursive type has infinite size
2 |     Cons(i32, List),
  |               ---- recursive without indirection
  |
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to make `List` representable
  |
2 |     Cons(i32, Box<List>),
  |               ++++    +

For more information about this error, try `rustc --explain E0072`.
error: could not compile `sp_demos` due to previous error
```

*清单 15-4：在尝试定义递归枚举时，咱们得到的报错*

报错显示此类型“具有无限大小，has infinite size”。原因是咱们一个递归的变种定义了 `List`：其直接持有自己的的另一个值。因此，Rust 无法计算出他需要多少空间来存储一个 `List` 值。咱们来分析一下为什么咱们会得到这个报错。首先，咱们来看看 Rust 如何确定出他需要多少内存空间来存储某个非递归类型的值。


### 计算非递归类型的大小

回顾咱们在第 6 章讨论枚举定义时，在清单 6-2 中定义的 `Message` 枚举：

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

为了确定出给一个 `Message` 值分配多少内存空间，Rust 会检查每个变种，找出哪个变种需要最多的空间。Rust 发现 `Message::Quit` 不需要任何空间，`Message::Move` 需要存储两个 `i32` 值的足够空间，以此类推。由于只会用到一个变种，因此某个 `Message` 值所需的最大内存空间，便是其存储最大变种的空间。

将此与 Rust 尝试确定递归类型（如清单 15-2 中的 `List` 枚举）需要多少空间时发生的情况进行对比。编译器从查看 `Cons` 变种开始，其持有一个 `i32` 类型的值和一个 `List` 类型的值。因此，`Cons` 需要的空间量等于 `i32` 的大小加上 `List` 的大小。为了计算出 `List` 类型需要多少内存，编译器会从 `Cons` 变体开始查看变种。 `Cons` 变种保存了一个 `i32` 类型的值和一个 `List` 类型的值，这个过程会无限继续下去，如图 15-1 所示。

![由无限的 `Cons` 变种组成的一个无限 `List`](images/15-01.svg)

*图 15-01：由无限的 `Cons` 变种组成的一个无限 `List`*


### 使用 `Box<T>` 获得已知大小的递归类型


因为 Rust 无法计算出要为以递归方式定义出的类型分配多少空间，所以编译器会给出带有下面这个有用的建议的报错：

```console
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to make `List` representable
  |
2 |     Cons(i32, Box<List>),
  |               ++++    +
```

在此建议中，“间接，indirection” 意味着我们不应直接存储一个值，而应该改变数据结构，通过存储一个指向该值的指针，间接存储该值。

由于 `Box<T>` 是个指针，Rust 总是知道 `Box<T>` 需要多少内存空间：指针的大小不会根据他指向的数据量而变化。这意味着咱们可以在 `Cons` 变种里放入一个 `Box<T>`，而不是直接放入另一个 `List` 值。`Box<T>` 将指向下一个 `List` 值，他将在内存堆上而不是在 `Cons` 变种内。从概念上讲，咱们仍然有一个列表，用持有其他列表的列表来创建，但现在这种实现更像是把列表项目放在彼此的旁边，而不是放在彼此的里面。

咱们可以把清单 15-2 中 `List` 枚举的定义和清单 15-3 中 `List` 的用法，改为下面清单 15-5 中的代码，这样就可以编译了：

文件名：`src/main.rs`

```rust
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println! ("list: {:?}", list);
}
```

*清单 15-5：使用 `Box<T>` 的 `List` 的定义，以便有已知的大小*

`Cons` 变种需要一个 `i32` 的大小，加上存储匣子指针数据的内存空间。`Nil` 变种不存储存储任何值，所以他需要的空间比 `Cons` 变种少。咱们现在知道，任何 `List` 值都会占用一个 `i32` 的大小，加上一个匣子的指针数据的大小。通过使用匣子，咱们已经破解了无限的递归链，因此编译器可以计算出存储 `List` 值所需的内存大小。下图 15-2 显示了 `Cons` 变种现在的样子：

![由于 `Cons` 保存了一个 `Box` 而不在是无限大小的 `List`](images/15-02.svg)

*图 15-02：不在是无限大小的 `List`，因为 `Cons` 持有着一个 `Box`*

匣子仅提供这种间接与内存堆的内存分配；他们不具备任何像咱们在其他灵巧指针类型中，将看到的其他特别能力。他们也没有这些特殊能力所带来的性能开销，所以在像构造列表这样的情况下，他们就能很有用，因为间接性是我们唯一需要的功能。在第 17 章，咱们还会看一下匣子的更多用例。

`Box<T>` 类型是一个智能指针，因为他实现了 `Deref` 特质，他允许 `Box<T>` 值被当作引用。当 `Box<T>` 值超出作用域时，由于 `Drop` 特质的实现，匣子所指向的内存堆数据也会被清理。这两个特质对于咱们在本章后面将讨论的其他灵巧指针所提供的功能，将更加重要。咱们来更深入地探讨这两个特质。


（End）


