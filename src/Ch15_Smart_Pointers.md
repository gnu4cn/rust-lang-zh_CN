# 灵巧指针

**Smart Pointers**

所谓 *指针，pointer*，是个一般的概念，指的是包含了内存中某个地址的变量。该地址引用，或者说 “指向” 另一数据。Rust 中最常见的指针类别，便是咱们在第 4 章中曾了解过的引用。引用由 `&` 符号表示，并借用他们指向的值。除了引用数据之外，他们没有任何特殊功能，并且没有开销。

另一方面，*灵巧指针，smart pointer* 是一种数据结构，其作用类似于指针，但还具有额外的元数据与能力。灵巧指针这个概念，并非 Rust 所独有的：灵巧指针起源于 C++，且在其他语言中也存在。Rust 在标准库中定义了各种智能指针，他们提供了超越引用所提供的功能。为了探讨一般概念，我们将看几个不同的智能指针示例，包括 *引用计数，reference counting* 智能指针类型。引用计数这种指针，通过追踪数据所有者的数目，实现了允许数据有着多个所有者，在没有所有者剩下时，就清除该数据。

在所有权和借用的概念下，Rust 在引用和智能指针之间还有一个区别：引用只借用数据，而在很多情况下，灵巧指针则 *拥有，own* 他们所指向的数据。

虽然当时咱们没有这样称呼他们，但在本书中我们已经遇到了一些智能指针，包括第 8 章中的 `String` 和 `Vec<T>`。这两种类型都算作灵巧指针，因为他们拥有一些内存，并允许咱们对其进行操作。他们也有元数据和额外的能力或保证。例如，`String` 将其容量存储为元数据，并有额外的能力来确保其数据将始终是有效的 UTF-8。

灵巧指针通常是使用结构体来实现的。与寻常结构体不同，灵巧指针实现了 `Deref` 与 `Drop` 特质。`Deref` 特质允许灵巧指针结构体实例像引用那样行事，如此咱们便可编写出处理引用或灵巧指针的代码。而 `Drop` 特质则允许咱们定制在灵巧指针超出作用域时要运行的代码。本章中，咱们将讨论这两种特质，并演示他们为何对灵巧指针很重要。

鉴于灵巧指针模式是 Rust 中频繁用到的一种通用设计模式，本章不会涵盖每个既有灵巧指针。许多库都有自己的灵巧指针，咱们甚至也可以编写自己的灵巧指针。咱们将介绍标准库中最常见的灵巧指针：

- 用于在内存堆上分配值的 `Box<T>`；
- `Rc<T>`，一个引用计数类型，可以实现多重所有权，`Rc<T>`, a reference counting type that enables multiple ownership；
- `Ref<T>` 和 `RefMut<T>`，通过 `RefCell<T>` 访问，该类型在运行时而不是编译时执行借用规则检查，`Ref<T>` and `RefMut<T>`, accessed through `RefCell<T>`, a type that enforces the borrowing rules at runtime instead of compile time。

此外，咱们还将讨论 *内部可变性，interior mutability* 模式，在这种模式下，不可变的类型会暴露出一个用于改变内部值的 API。我们还将讨论引用循环：他们如何泄漏内存以及如何防止他们。

下面就来切入正题吧！


## 使用 `Box<T>` 指向内存堆上的数据

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

### 用匣子数据结构得到递归类型

**Enabling Recursive Types with Boxes**

*递归类型，recursive type* 的值可以有另一个相同类型的值作为其自身的一部分。递归类型带来了一个问题，因为在编译时 Rust 需要知道一个类型占用了多少空间。然而，理论上递归类型的值的嵌套可以无限地继续下去，所以 Rust 无法知道值需要多少空间。因为盒子有一个已知的大小，我们可以通过在递归类型定义中插入一个盒子来得到递归类型。

作为一个递归类型的示例，咱们来探讨一下 *构造列表，cons list*（the *cons* tructs *list*）。这是函数式编程语言中常见的一种数据类型。除了其中的递归之外，咱们将定义的构造列表类型是简单明了的；因此，当咱们遇到涉及递归类型的更复杂情况时，我们将使用的例子中的概念会很有用。


#### 构造列表的更多信息

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

#### 计算非递归类型的大小

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


#### 使用 `Box<T>` 获得已知大小的递归类型


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


## 使用 `Deref` 特质将智能指针视为常规引用

**Treating Smart Pointers Like Regular References with `Deref` Trait**

实现 `Deref` 特质允许咱们自定义 *解引用操作符, the dereference operator*️'*' （不要与乘法或 glob 运算符相混淆）的行为。通过实现 `Deref`，智能指针可以被当作普通的引用来对待，咱们便可编写对引用进行操作的代码，并将该代码也用于智能指针。

咱们首先来看看解除引用操作符是如何在常规引用中工作的。然后咱们将尝试定义一个行为类似于 `Box<T>` 的自定义类型，并看看为什么解除引用操作符在咱们新定义的类型上不像引用那样工作。咱们将探讨实现 `Deref` 特性如何使智能指针的工作方式与引用相似。然后咱们将看看 Rust 的 *解引用强制转换，deref coercion* 特性，以及其如何让咱们使用引用或智能指针工作的。

> 注意：咱们将要建立的 `MyBox<T>` 类型和真正的 `Box<T>` 之间有一个很大的区别：咱们的版本不会将其数据存储在堆中。咱们把这个例子的重点放在 `Deref` 上，所以数据实际存储在哪里并不重要，重要的是类似指针的行为。


### 顺着指针找到值

**Following the Pointer to the Value**

常规引用是一种指针，而看待指针的一种方式，便是指向存储于别处值的一个箭头。在下面清单 15-6 种，咱们创建了一个对 `i32` 值的引用，然后使用解引用操作符，来跟随对该值的引用：

文件名：`src/main.rs`

```rust
fn main() {
    let x = 5;
    let y = &x;

    assert_eq! (5, x);
    assert_eq! (5, *y);
}
```

*清单 15-6：使用解引用操作符来跟随一个 `i32` 值的引用*

变量 `x` 保存着一个 `i32` 值 `5`。咱们将 `y` 设置为等于到 `x` 的引用。咱们可以断言 `x` 等于 `5`。然而，如果咱们想对 `y` 中的值进行断言，咱们必须使用 `*y` 来跟随其所指向的值的音乐（因此是 *解引用，dereference*），这样编译器才能比较具体值。一旦咱们解引用了 `y`，咱们就可以访问咱们可将其与 `5` 比较的 `y` 指向的整数值。

相反，如果咱们尝试编写 `assert_eq! (5, y);`，咱们便会得到下面这样的编译报错：

```console
$ cargo run                                                      ✔  
   Compiling sp_demos v0.1.0 (/home/peng/rust-lang/sp_demos)
error[E0277]: can't compare `{integer}` with `&{integer}`
 --> src/main.rs:6:5
  |
6 |     assert_eq! (5, y);
  |     ^^^^^^^^^^^^^^^^^ no implementation for `{integer} == &{integer}`
  |
  = help: the trait `PartialEq<&{integer}>` is not implemented for `{integer}`
  = help: the following other types implement trait `PartialEq<Rhs>`:
            f32
            f64
            i128
            i16
            i32
            i64
            i8
            isize
          and 6 others
  = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0277`.
error: could not compile `sp_demos` due to previous error
```

比较数字与对数字的引用是不允许的，因为他们属于不同的类型。咱们必须使用解引用操作符来跟随引用到他所指向的值。


### 像引用一样使用 `Box<T>`

**Using `Box<T>` Like a Reference**


咱们可将清单 15-6 中的代码，重写为使用 `Box<T>` 而不是引用；下面清单 15-7 中 `Box<T>` 上使用的解引用操作符，与清单 15-6 中引用上使用的解引用操作符功能相同：

文件名：`src/main.rs`

```rust
fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq! (5, x);
    assert_eq! (5, *y);
}
```

*清单 15-7：在 `Box<i32>` 上使用解引用操作符*

清单 15-7 与清单 15-6 之间的主要区别，就是这里将 `y` 设置为了指向 `x` 的一个拷贝值的匣子实例，而不是清单 15-6 中那样，指向 `x` 值的引用。在最后那个断言里，咱们就可以在 `y` 为引用时，曾做过的同样方式，使用解引用操作符来跟随这个匣子的指针。接下来，将通过定义咱们自己的匣子类型，来探讨到底 `Box<T>` 有何特别之处，来实现在其上使用解引用操作符的。


### 定义咱们自己的灵巧指针

接下来就要构建出一个，类似于由标准库所提供的 `Box<T>` 类型的灵巧指针，而感受一下灵巧指针默认情况下，是怎样不同于引用的。随后就将看看，如何添加这种使用解引用操作符的能力。

`Box<T>` 最终被定义为有着一个元素的元组结构体（a tuple struct），因此清单 15-8 就以同样方式，定义了一个 `MyBox<T>` 类型。这里还将定义一个 `new` 函数，来与定义在 `Box<T>` 上的那个 `new` 函数相匹配。

文件名：`src/main.rs`

```rust
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
```

*清单 15-8：定义出一个 `MyBox<T>` 类型*

这里定义了一个名为 `MyBox` 的结构体，并由于这里想要这个类型保存任意类型的值，而声明了一个泛型参数 `T`。该 `MyBox` 类型是个有着一个类型 `T` 元素的元组结构体。其中的 `MyBox::new` 函数会取一个类型 `T` 的参数，并返回保持着所传入值的一个 `MyBox` 实例。

接下来尝试把清单 15-7 中的 `main` 函数，添加到清单 15-8 并将其修改为，使用这个上面定义的 `MyBox<T>` 类型而非 `Box<T>`。由于 Rust 不清楚怎样解引用 `MyBox`，因此下面清单 15-9 中的代码不会编译。

文件名：`src/main.rs`

```rust
fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq! (5, x);
    assert_eq! (5, *y);
}
```

*清单 15-9：以使用引用及 `Box<T>` 同样方式，尝试使用 `MyBox<T>`*

下面就是那产生出的编译报错：

```console
$ cargo run                                                      ✔  
   Compiling sp_demos v0.1.0 (/home/peng/rust-lang/sp_demos)
error[E0614]: type `MyBox<{integer}>` cannot be dereferenced
  --> src/main.rs:14:20
   |
14 |     assert_eq! (5, *y);
   |                    ^^

For more information about this error, try `rustc --explain E0614`.
error: could not compile `sp_demos` due to previous error
```

由于咱们未曾在这个 `MyBox<T>` 类型上实现过其被解引用的能力，因此他无法被解引用。为实现使用 `*` 运算符的解引用，就要实现 `Deref` 特质。


### 通过实现 `Deref` 特质而像引用那样，对待某个类型

**Treating a Type Like a Reference by Implementing the `Deref` Trait**

正如第 10 章的 ["在类型上实现某个特质"](Ch10_Generic_Types_Traits_and_Lifetimes.md#在类型上实现某个特质) 小节中所讨论过的，这里需要提供到特质所要求的那些方法的实现。而这个由标准库提供的 `Deref` 特质，要求咱们实现一个会借用到 `self`，并会返回到其内部数据的引用的名为 `deref` 的方法。下面清单 15-10 包含了添加到 `MyBox` 定义的一个 `Deref` 实现：

文件名：`src/main.rs`

```rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```

*清单 15-10：在 `MyBox<T>` 上实现 `Deref`*

其中 `type Target = T;` 这种语法，定义出了 `Deref` 特质要用到的一个关联类型（an assiotiated type for the `Deref` trait to use）。关联类型属于与声明泛型参数有些许不同的声明方式，现在无需担心他们；在第 19 张中将更细致地讲到他们。

这里填入 `deref` 方法函数体的是 `&self.0`，从而 `deref` 就返回了到咱们打算用 `*` 运算符访问的那个值的一个引用；回顾第 5 章的 [运用不带命名字段的元组结构体来创建出不同类型](Ch05_Using_Structs_to_Structure_Related_Data.md#使用不带命名字段的元组结构体来创建不同类型) 小节，那个 `.0` 就是访问了结构体中的首个值。清单 15-9 中在其中 `MyBox<T>` 值上调用了 `*` 的 `main` 函数，现在就会编译了，同时那些断言将通过！

没有这个 `Deref` 特质，编译器就只能解引用那些 `&` 的引用。那个 `deref` 方法，给到了编译器取得实现了 `Deref` 特质的任意类型值的能力，而调用该特质的 `deref` 方法，就获得了其知道如何解引用的一个 `&` 引用。

在于清单 15-9 中敲入 `*y` 时，在幕后 Rust 实际上运行了下面的代码：

```rust
*(y.deref())
```

Rust 使用到 `deref` 方法的一个调用，以及接着一个普通的解引用，替换了那个 `*` 运算符，如此咱们就不必考虑，这里到底需不需要调用那个 `deref` 方法了。Rust 的这项特性，实现了不论对于常规引用，或是对实现了 `Deref` 特质的类型，以同样方式起作用代码的编写。

那个`deref` 返回到某个值的引用，以及 `*(y.deref())` 中括号外的那个普通解引用，二者都有其存在的必要原因，那就是配合了 Rust 的所有权系统。假如这个 `deref` 方法直接返回值，而不是到值的引用，那么该值就会被迁移出 `self`。咱们并未打算取得这个示例，或用到解引用操作符的其他绝大多数用例中，`MyBox<T>` 里头那个内层值的所有权。

请注意那个 `*` 运算符被替换为了到 `deref` 方法的一次调用，和随后到 `*` 运算符的一次调用，这替换只有一次，且咱们在代码中用到一次 `*` 运算符，这种替换就会进行一次。由于 `*` 运算符的这种替代不会无限递归，因此这里就会以类型 `i32` 的数据而结束，其正好与清单 15-9 中那个 `assert_eq!` 里的 `5` 匹配。


### 函数与方法下的隐式解引用强制转换

**Implicit `Deref` Coercions with Functions and Methods**

*解引用强制转换，deref coercion* 会将到实现了 `Deref` 特质的某种类型的引用，转换为到另一类型的引用。比如，由于 `String` 实现了 `Deref` 特质，因此对 `&String` 解引用强制转换，就会返回 `&str`，因此就可以把 `&String` 解引用强制转换为 `&str`。解引用强制转换，属于 Rust 在函数与方法的参数上，所执行的一项便利措施，并只在那些实现了 `Deref` 特质的类型上起作用。在将到特定类型值的引用，作为参数传递给某个函数或方法，而所传递的引用，并不与那个函数或方法定义中的参数类型想匹配时，这种解引用强制转换就会发生。这时到 `deref` 方法的一系列调用，就会把所提供的类型，转换为函数或方法定义中那些参数所需的类型。

> *注*：在面向对象编程语言 Java 中，类似的特性叫 ["自动装箱"](https://java.xfoss.com/Ch10_Numbers_and_Statics_Numbers_Matter.html#%E8%87%AA%E5%8A%A8%E8%A3%85%E7%AE%B1%E6%A8%A1%E7%B3%8A%E5%8E%9F%E7%94%9F%E5%80%BC%E4%B8%8E%E5%AF%B9%E8%B1%A1%E4%B9%8B%E9%97%B4%E7%9A%84%E7%95%8C%E7%BA%BF)。

为了程序员们在编写函数与方法调用时，无需使用 `&` 及 `*` 添加许多的那些显示引用和解引用，解引用强制转换特性就这样被添加到 Rust 了。这种解引用强制转换，还实现更多既可在引用，亦可在灵巧指针上起作用代码的编写。

下面就来使用定义在清单 15-8 中的这个 `MyBox<T>` 类型，以及在清单 15-10 中添加的那个 `Deref` 实现，来看看运作中的解引用强制转换。下面清单 15-11 给出了有着一个字符串切片参数的某个函数定义：

文件名：`src/main.rs`

```rust
fn hello(name: &str) {
    println! ("你好，{name}");
}
```

*清单 15-11：有着类型 `&str` 参数 `name` 的 `hello` 函数*

这里可使用一个字符串切片作为参数，调用这个 `hello` 函数，譬如 `hello("Rust");`。而解引用强制转换特性，就令到使用到类型 `MyBox<String>` 值的引用，来调用 `hello` 成为可能，如下清单 15-12 中所示：

文件名：`src/main.rs`

```rust
fn main() {
    hello("Rust");

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
```

*清单 15-12：使用到某个 `MyBox<String>` 值的引用调用 `hello`，因为有解引用强制转换，这样做是可行的*

这里使用参数 `&m`，即到某个 `MyBox<String>` 值的引用，调用的那个 `hello` 函数。由于这里曾在清单 15-10 中的 `MyBox<T>` 上实现过 `Deref` 特质，因此 Rust 就能通过调用 `deref`，将 `&MyBox<String>` 转换为 `&String`。标准库提供了在 `&String` 上，返回一个字符串切片的 `Deref` 实现，且这一点就在 `Deref` 的 API 文档中。Rust 就会再度调用 `deref`，来将这个 `&String` 转换为 `&str`，这就与 `hello` 函数定义想吻合了。

若 Rust 不曾实现解引用强制转换特性，那么这里就不得不编写出下面清单 15-13 中的代码，而不是清单 15-12 中的代码，来以某个类型 `&MyBox<String>` 值调用 `hello` 了：

文件名：`src/main.rs`

```rust
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
}
```

*清单 15-13：若 Rust 没有解引用强制转换特性，而将不得不编写的代码*

其中的 `(*m)` 将那个 `MyBox<String>` 解引用为了一个 `String`。随后的 `&` 与 `[..]`，则取了与 `hello` 函数签名相匹配，等于整个字符串的该 `String` 的字符串切片。这种不带有解引用强制转换的代码，因为涉及到全部的这些符号，而更难于阅读、编写与理解。正是解引用强制转换，实现了 Rust 为咱们自动处理这些转换。

在为这些涉及到的类型定义了 `Deref` 特质时，Rust 将分析这些类型，并运用必要次数的 `Deref::deref`，来获取到与函数或方法定义中参数类型相匹配的一个引用。所需插入的 `Deref::deref` 次数，是在编译时解算出来的，因此解引用强制转换优势的利用，并无运行时的代价。


### 解引用强制转换与可变性的互动方式

与使用 `Deref` 特质来覆写不可变引用上的 `*` 运算符的方式类似，咱们可以使用 `DerefMut` 特质，来覆写可变引用上的 `*` 运算符。

在以下三种情形下，Rust 会在他发现类型与特质的实现时，执行强制引用转换：

- 从 `&T` 强制转换为 `&U` 时，`T: Deref<Target=U>`
- 从 `&mut T` 转换为 `&mut U` 时，`T: DerefMut<Target=U>`
- 从 `&mut T` 转换为 `&U` 时，`T: Deref<Target=U>`

其中前两个情形，除了第二种实现了可变外，他们是同样的。第一种情形指出了在咱们有着一个 `&T`，且 `T` 对某种类型 `U` 实现了 `Deref` 特质，那么就显然能得到一个 `&U`。第二种情形则指出了对可变引用，同样会发生解引用强制转换。

那第三中情形就较为复杂了：Rust 还将把某个可变引用，强制转换为一个不可变引用。但反过来则是 *不* 可行的：不可变引用绝不会强制转换为可变引用。由于借用规则的存在，在有着某个可变引用时，那个可变引用必定只会是到那个数据的引用（否则，程序就不会编译）。将一个可变引用转换为一个不可变引用，是绝不会破坏借用规则的。而将不可变引用转换为可变引用，就会要求那个初始不可变引用，为到那个数据的唯一不可变引用，但借用规则却不会确保那一点。因此，Rust 就无法做出将不可变引用，转换为可变引用可行这一假定。


## 使用 `Drop` 特质在清理内存时运行代码

**Running Code on Cleanup with `Drop` Trait**

对于灵巧指针模式来讲，重要的第二个特质便是 `Drop` 了，他允许咱们在某个值即将超出作用域时，对要发生什么加以定制。在任何类型上，咱们都可以提供 `Drop` 特质的一个实现，而那些代码就可被用于释放诸如文件或网络连接等资源。

这里之所以在灵巧指针上下文中引入 `Drop` 特质，是由于 `Drop` 特质的功能，几乎总是用在实现某个灵巧指针的时候。比如，在某个 `Box<T>` 被弃用时，`Drop` 特质就会解除该匣子所指向的堆上的内存空间分配。

在一些语言中，对于某些类型，编程者就必须在他们每次结束使用这些类型的某个实例时，调用代码来释放内存或其他资源。这类示例包括了文件把手、套接字或一些锁等等（file handles, sockets, or locks）。若他们忘记了这点，那么系统就会变得过载并崩溃。而在 Rust 中，咱们就可以指定出，在每当有某个值超出作用域时，所运行的一些特定代码，而编译器就会自动插入这些代码。结果就是，咱们就不需要小心翼翼地，在程序里某种特定类型的某个示例结束使用的各处，放置那些清理代码了 -- 咱们仍不会泄露各种资源！

咱们是通过实现 `Drop` 特质，指定出在某个值超出作用域时所运行的那些代码的。`Drop` 特质要求咱们，要实现一个取到 `self` 的可变引用、名为 `drop` 的方法。现在就来实现一个有着数条 `println!` 语句的 `drop` 方法，以发现 Rust 于何时调用这个 `drop` 方法。

下面清单 15-14 给出了仅有着一项定制功能，即在其实例超出作用域时打印出 `正在弃用 CustomSmartPointer！` 的一个 `CumstomSmartPointer` 结构体，以展示出 Rust 在何时运行这个 `drop` 函数。

文件名：`src/main.rs`

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println! ("正在使用数据 `{}` 弃用 CustomSmartPointer！", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("我的事情"),
    };
    let d = CustomSmartPointer {
        data: String::from("其他事情"),
    };
    println! ("已创建出一些 CustomSmartPointer 实例");
}
```

*清单 15-14：实现了于其中放置咱们编写的清理代码的 `Drop` 特质的 `CustomSmartPointer` 结构体*

`Drop` 特质是包含在 Rust 序曲（the prelude）中的，因此这里就无需将其带入作用域。这里在 `CustomSmartPointer` 上实现了 `Drop` 特质，并提供了一个调用了 `println!` 宏的 `drop` 方法的实现。这个 `drop` 函数的函数体，即是咱们将要放置那些，在咱们类型的某个实例超出作用域时，打算运行的全部逻辑的地方。这里咱们就打印出一些文本，来直观地演示 Rust 何时会调用 `drop` 方法。

在 `main` 函数中，这里创建出了 `CustomSmartPointer` 的两个实例，并于随后打印了 `已创建出一些 CumstomSmartPointer 实例`。在 `main` 末尾处，这些 `CumstomSmartPointer` 的实例，就将超出作用域，同时 Rust 就会调用咱们放入到`drop` 方法中的那些代码，打印出咱们的最终消息。请注意咱们并不需要显式地调用这个 `drop` 方法。

在运行这个程序的时候，就会看到下面的输出：

```console
$ cargo run                                                                             lennyp@vm-manjaro
   Compiling sp_demos v0.1.0 (/home/lennyp/rust-lang/sp_demos)
    Finished dev [unoptimized + debuginfo] target(s) in 0.50s
     Running `target/debug/sp_demos`
已创建出一些 CustomSmartPointer 实例
正在使用数据 `其他事情` 弃用 CustomSmartPointer！
正在使用数据 `我的事情` 弃用 CustomSmartPointer！
```

在这些实例超出作用域时，Rust 就自动为咱们调用了 `drop`，进而调用了咱们指定出的那些代码。变量以与他们创建相反的顺序被弃用，因此其中的 `d` 先于 `c` 被启用。这个示例的目的，是要给到 `drop` 方法工作方式的直观说明；通常咱们会指定出咱们的类型所要运行的代码，而非一条打印出的消息。


### 使用 `std::mem::drop` 提前弃用某个值

**Drop a Value Early with `std::mem::drop`**

不幸的是，要关闭这种自动的 `drop` 功能，却并不那么简单。关闭 `drop` 并不常见；`Drop` 特质的全部意义，就在于他是自动的。然而在少数情况下，咱们就会想要提前清理掉某个值。一个这样的例子，便是在运用一些管理锁的灵巧指针时：咱们就可能希望强制运行那个释放锁的 `drop` 方法，从而同一作用域中的其他代码，就可以请求到该锁。Rust 是不允许咱们，手动调用 `Drop` 特质的 `drop` 方法的；相反，在想要在作用域结束之前，强制弃用某个值时，咱们可以调用由标准库提供的 `std::mem::drop` 函数。

若咱们通过修改清单 15-14 中那个 `main` 函数，而尝试手动调用 `Drop` 特质的 `drop` 方法，如下清单 15-15 中所示，就会得到一个编译器报错：

文件名：`src/main.rs`

```rust
fn main() {
    let c = CustomSmartPointer {
        data: String::from("一些数据"),
    };
    println! ("已创建出一个 CustomSmartPointer 实例。");
    c.drop();
    println! ("在 main 结束之前这个 CustomSmartPointer 已被弃用。")
}
```

*清单 15-15：尝试调用 `Drop` 特质中的 `drop` 方法来提前清理*


在尝试编译此代码时，就会得到下面这样的错误：

```console
$ cargo run                                                                             lennyp@vm-manjaro
   Compiling sp_demos v0.1.0 (/home/lennyp/rust-lang/sp_demos)
error[E0040]: explicit use of destructor method
  --> src/main.rs:17:7
   |
17 |     c.drop();
   |     --^^^^--
   |     | |
   |     | explicit destructor calls not allowed
   |     help: consider using `drop` function: `drop(c)`

For more information about this error, try `rustc --explain E0040`.
error: could not compile `sp_demos` due to previous error
```

此错误消息指出，这里是不允许显式调用 `drop` 方法的。该错误消息用到了术语 *解构器，descructor*，那正是清理掉某个实例的函数的通用编程术语（the general programming term）。*解构器，destructor* 类似于创建出实例的 *构造器，constructor*。Rust 中的 `drop` 函数，就是一个特别的解构器。

Rust 之所以不让咱们显式地调用 `drop`，是因为 Rust 仍将在 `main` 函数末尾，自动调用那个值上的 `drop`。由于 Rust 会两次尝试清理同一值，因此这就会导致 *双重释放，double free* 的错误。

咱们无法关闭这种在某个值超出作用域时的 `drop` 自动插入，同时又无法显式地调用 `drop` 方法。因此，在咱们需要强制某个值提前被清理掉时，就要使用 `std::mem::drop` 函数。

这个 `std::mem::drop` 函数不同于 `Drop` 特质中的那个 `drop` 方法。咱们是通过将要强制弃用的那个值作为参数传递，而调用他的。该函数位于 Rust 序曲中（in the prelude），因此这里就可以把清单 15-15 中的 `main` 函数，修改为如下清单 15-16 中所示的调用那个 `drop` 函数：

文件名：`src/main.rs`

```rust
fn main() {
    let c = CustomSmartPointer {
        data: String::from("我的事情"),
    };
    println! ("已创建出一个 CustomSmartPointer 实例。");
    drop(c);
    println! ("在 main 结束之前这个 CustomSmartPointer 已被弃用。")
}
```

*清单 15-16：调用 `std::mem::drop` 在某个值超出作用域之前，显式地弃用该值*

运行此代码就会打印出下面的输出：

```console
$ cargo run                                                                             lennyp@vm-manjaro
   Compiling sp_demos v0.1.0 (/home/lennyp/rust-lang/sp_demos)
    Finished dev [unoptimized + debuginfo] target(s) in 0.40s
     Running `target/debug/sp_demos`
已创建出一个 CustomSmartPointer 实例。
正在使用数据 `一些数据` 弃用 CustomSmartPointer!
在 main 结束之前这个 CustomSmartPointer 已被弃用。
```

于 `已创建出一个 CustomSmartPointer 实例。`，与 `在 main 结束之前这个 CustomSmartPointer 已被弃用。` 文本之间打印出的文本，`正在使用数据 `一些数据` 弃用 CustomSmartPointer!` ，显示在那个时间点，`Drop` 特质的 `drop` 方法被调用来弃用 `c`。

咱们可以许多种方式，使用 `Drop` 特质实现中所指明的代码，来令到资源清理变成方便且安全：比如就可以使用这种技巧，来创建出咱们自己的内存分配器！有了这个 `Drop` 特质及 Rust 的所有权系统，由于 Rust 会自动完成资源的清理，因此咱们就不必一定要记得清理资源了。

咱们还不必担心，意外清理仍在使用中的一些值而导致的问题：确保引用始终有效的所有权系统，会确保 `drop` 只会在该值不再会被使用时才被调用。

既然咱们已经检视了 `Box<T>` 及灵巧指针的一些特征，接下来就要看看定义在标准库中的个别其他灵巧指针了。


## `Rc<T>`，引用计数灵巧指针

大多数情况下，所有权都是明确的：咱们确切知道，是哪个变量拥有者某个给定值。然而，单个值可能有着多个所有者的情形，也是有的。比如，在图数据结构（graph data structures），多条边就可能指向同一节点，从概念上讲，而那个节点就是被所有指向他的边所拥有的。在已不再有任何边指向节点，进而该节点已无所有者之前，这个节点就不应被清理掉。

必须通过使用 Rust 的类型 `Rc<T>`，来显式地启用多重所有权，`Rc<T>` 即 *引用计数，reference counting* 的缩写。`Rc<T>` 类型会追踪某个值的引用数，从而判断出该值是否仍在使用中。在到某个值的引用数为零时，该值就可以在不会有任何引用变成无效的情况下，（安全地）被清理掉。

请将 `Rc<T>` 设想为客厅里的一台电视机。在有人进来看电视时，他们就会打开他。其他人是可以进来客厅而看电视的。在最后一人离开客厅时，由于电视已不再被使用，他们便关掉了电视机。而在其他人仍在看电视时，有人关了电视机，那么剩下的那些电视观众，就会哇哇叫的！

当咱们打算在内存堆上给咱们程序多个部分，分配用来读取的一些数据，且无法确定出，在编译时那些部分将最后用到这些数据时，咱们就会用到这个 `Rc<T>` 类型。若咱们清楚那个部分将最后结束，那么就可以只把那个部分，构造为该数据的所有者，同时在编译时强制用到的一般所有权规则，就能发挥作用。

请注意 `Rc<T>` 只适用于单线程的场景（only for use in single-threaded scenarios）。在第 16 章中讨论到并发时，就会讲到多线程程序中，怎样完成引用计数。


### 使用 `Rc<T>` 来共用数据

**Using `Rc<T>` to Share Data**


下面来回到清单 15-5 中那个构造列表的示例。回顾到咱们曾使用 `Box<T>` 定义出的那个构造列表。这次，咱们将创建出同时共用了第三个列表的两个列表。概念上讲，这看起来与下图 15-3 类似：

![`b` 与 `c` 两个列表，共用了第三列表 `a` 的所有权](images/15-03.svg)

*图 15-03：`b` 与 `c` 两个列表，共用了第三列表 `a` 的所有权*

这里将构造出包含 `5` 与其后 `10` 的列表 `a`。随后这里将构造两个另外的列表：以 `3` 开始的 `b` 和以 `4` 开始的列表 `c`。列表 `b` 与 `c` 都将接着延续到头一个包含着 `5` 及 `10` 的列表 `a`。换句话说，这两个列表将共用那包含了 `5` 与 `10` 的头一个列表。

使用之前有着 `Box<T>` 的 `List` 尝试实现这种场景，就不会工作，如下清单 15-17 中所示：

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

*清单 15-17：对不允许有着两个用到 `Box<T>` 的列表尝试共用第三列表进行演示*

在编译上面的代码时，就会得到下面的报错：

```console
$ cargo run                                              ✔  
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

那些 `Cons` 变种，拥有他们所保存的数据，因此在创建出那个 `b` 列表时，`a` 就被迁移进了 `b`，进而 `b` 就拥有了 `a`。随后，在创建出 `c` 而尝试再次使用 `a` 时，因为 `a` 已被迁移，因此这里就不再被允许了。

这里原本是可以将 `Cons` 的定义，修改为保存引用的，但随后就必须要指定生命周期参数。经由制定生命周期参数，这里就指出了列表中的每个元素，都将与整个列表有同样的存活时间。这正是清单 15-17 中元素与列表的情形，但并非是所有场景中的情形。

相反，这里将把 `List` 的定义，修改为在 `Box<T>` 处运用 `Rc<T>`，如下清单 15-18 中所示。这样各个 `Cons` 变种，现在就将保存一个值与一个指向某个 `List` 的 `Rc<T>` 了。在创建出 `b` 时，就不再是取得 `a` 的所有权，而是将克隆出 `a` 正保存的那个 `Rc<List>`，因此将引用的数据，从一个增加到了两个，实现了 `a` 与 `b` 共用那个 `Rc<List>` 中的数据。在创建出 `c` 时，这里也将克隆 `a`，从而将引用的数据，从两个增加到三个。每次调用 `Rc::clone` 时，到那个 `Rc<List>` 里头数据的引用计数，都将增加，同时除非到其引用为零，该数据便不会被清理掉。

文件名：`src/main.rs`

```rust
#[derive(Debug)]
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

    println! ("b 为: {:?}\nc 为： {:?}", b, c);
}
```

*清单 15-18：使用了 `Rc<T>` 的 `List` 定义*

由于 `Rc<T>` 不在 Rust 序曲中（in the prelude），因此就需要添加一条 `use` 语句将其带入到作用域中。在 `main` 函数里，这里创建了那个包含 `5` 与 `10` 的列表，并将其存储在 `a` 中的一个新 `Rc<List>` 里。随后在创建 `b` 与 `c` 时，这里调用了 `Rc::clone` 函数，并将到那个 `Rc<List>` 的引用作为参数加以传入。

这里本可以调用 `a.clone()` 而不是 `Rc::clone(&a)`，但在这种情况下，Rust 的约定就是使用 `Rc::clone`。`Rc::clone` 方法的实现，与绝大多数类型的 `clone` 实现方式不同，其并不会构造全部数据的深拷贝。到 `Rc::clone` 的调用，只会增加引用计数，这样做不耗费很多时间。而数据的一些深拷贝，则能耗费很多时间。通过使用 `Rc::clone` 来进行引用计数，咱们就可以直观地区别出深拷贝类别的那些克隆，与那些增加引用计数的克隆类别。在查找代码中的性能问题时，咱们只需要关注那些深拷贝的克隆，而可以不用管那些到 `Rc::clone` 的调用。

> **注**：第 4 章 [变量与数据交互方式之二：克隆](Ch04_Understanding_Ownership.md#变量与数据交互方式之二克隆) 中，曾提到：“当看到一个对 clone 方法的调用时，那么就明白正有一些任性代码在被执行，且那代码可能开销高昂。对此方法的调用，是某些不同寻常事情正在发生的明显标志。”。


### 对某个 `Rc<T>` 进行克隆，就会增加引用计数

**Cloning an `Rc<T>` Increases the Reference Count**

下面就拉修改清单 15-18 中的那个运作中的示例，从而可以发现在创建及弃用到 `a` 中那个 `Rc<T>` 的引用时，引用计数就会改变。

在下面清单 15-19 中，这里将修改 `main` 为其有着一个围绕列表 `c` 的内层作用域；随后就会看到在 `c` 超出作用域时，引用计数会怎样变化。


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

在程序中引用计数变化的各个点位，咱们都打印出了引用计数，其正是咱们经由调用 `Rc::strong_count` 函数得到的。该函数之所以名为 `strong_count`，而非 `count`，是由于这个 `Rc<T>` 类型，还有一个 `weak_count` 函数；在 [阻止引用的循环：将 `Rc<T>` 转换为 `Weak<T>`](#防止引用循环将-rct-转变为-weakt) 小节，就会看到 `weak_count` 的使用。

此代码会打印出下面的东西：


```console
$ cargo run                                                                                              lennyp@vm-manjaro
   Compiling sp_demos v0.1.0 (/home/lennyp/rust-lang/sp_demos)
    Finished dev [unoptimized + debuginfo] target(s) in 0.40s
     Running `target/debug/sp_demos`
在创建出 a 后，引用计数为 1
在创建出 b 后，引用计数为 2
在创建出 c 后，引用计数为 3
在 c 超出作用域后，引用计数为 2
```

可以看出，变量 `a` 中的 `Rc<List>` 有着初始的引用计数 `1`；随着在每次调用 `clone` 时，该计数就会上升 `1`。在变量 `c` 超出作用域时，该计数降低了 `1`。与必须调用 `Rc::clone` 来提升该引用计数不同，咱们不必调用某个函数，来降低引用计数：在某个 `Rc<T>` 值超出作用域时，`Drop` 特质实现会自动降低引用计数。

在这个示例中，咱们无法见到的是，在 `b` 及随后的 `a` 于 `main` 结束处超出作用域时，该计数就会是 `0`，同时这个 `Rc<List>` 就被完全清除掉。使用 `Rc<T>` 就实现了单个的值，有着多个所有者，同时这种计数确保了该值在任意这些所有者存在期间，保持有效。

通过不可变引用，`Rc<T>` 实现了程序的各个部分之间，只读地共用数据。在 `Rc<T>` 也实现了有着多个可变引用时，就会违反第 4 章中，曾讨论过的借用规则之一：到同一处所的多个可变借用，会导致数据竞争与不一致问题。然而能够修改数据，是非常有用的！那么在接下来的小节，就会讨论内部可变性模式（the interior mutability pattern），与那个可结合 `Rc<T>` 值，用来解决这种不可变限制问题的 `RefCell<T>` 类型（the `RefCell<T>` type that you can use in conjunction with an `Rc<T>` to work with this immutability restriction）。


## `RefCell<T>` 及内部可变性模式

**`RefCell<T>` and the Interior Mutability Pattern**

*内部可变性，interior mutability* 属于 Rust 中的一种设计模式，他实现了即使在有着到数据的一些不可变引用之下，对数据加以改变；一般情况下，这样的行为是借用规则所不允许的。为了改变数据，这种模式便运用了数据结构内部的一些 `unsafe` 代码，来改变了 Rust 监管可变性与借用的一些一般规则。这些不安全代码向编译器表明，咱们自己在手动检查那些规则，而非依赖于编译器为咱们检查那些规则；在第 19 章将进一步讨论这些不安全代码。

咱们可以只在能够确保借用规则在运行时将被遵循，而即使编译器无法保证这一点时，使用那些运用了内部可变性的类型。那么这个时候所涉及的那些 `unsafe` 代码，就会被封装在某个安全的 API 中，而外层的类型仍然是不可变的（we can use types that use the interior mutability pattern only when we can ensure that the borrowing rules will be followed at runtime, even though the compiler can't guarantee that. The `unsafe` code involved is then wrapped in a safe API, and the outer type is still immutable）。

接下来就要经由检视这个遵循内部可变性设计模式的 `RefCell<T>` 类型，探讨此概念。


### 使用 `RefCell<T>` 在运行时强制借用规则检查

**Enforcing Borrowing Rules at Runtime with `RefCell<T>`**

不同于 `Rc<T>`，这个 `RefCell<T>` 类型，表示其所保存数据上的单个所有权。那么到底是什么令到 `RefCell<T>` 不同于像 `Box<T>` 这样的类型呢？回顾在第 4 章中所掌握的那些借用规则：

- 在任何给定时间，咱们都可以有着 *要么* （而非同时） 一个的可变引用，要么任意数量的不可变引用；
- 引用必须始终是有效的。

在引用及 `Box<T>` 之下，这些借用规则的那些不变性，在编译时被强制检查（with references and `Box<T>`, the borrowing rules' invariants are enforced at compile time）。而在 `RefCell<T>` 之下，这些不变性是在 *运行时，runtime*，被强制检查的。对于引用，在破坏这些规则时，就会得到编译时错误。而对于 `RecCell<T>`，在破坏这些规则时，程序就会终止运行并退出。

在编译时检查借用规则的好处，就是那些错误会在开发过程中被及时捕获到，而因为全部代码分析都是提前完成的，因此在运行时性能上没有影响。由于这些原因，在编译时检查借用规则，即是大多数情形中的最佳实践，也正是 Rust 作为默认项的原因。

相反在运行时检查借用规则的优势，在于这个时候明确的内存安全场景是被允许的，这些场景中，他们原本是不被编译时借用规则检查所允许。一些静态分析，好比 Rust 的编译器，本质上是保守的。代码的一些属性，都是不可能通过分析代码侦测到的：其中最有名的示例，便是图灵停机问题，the Halting Problem，这个问题超出了本书的范围，但是个要研究的有趣话题。

由于某些分析不可能进行，因此在 Rust 编译器无法确定代码，在所有权规则下会编译时，他就会拒绝某个正确的程序；从这方面讲，他就是保守的了。假如 Rust 编译器接受不正确的程序，那么用户将无法信任 Rust 所做出的那些保证。然而，若 Rust 拒绝某个正确程序，那么编程者就将感到不便，却又不会发生什么灾难性的事情。在咱们确定咱们的代码遵循了借用规则，只不过编译器无法理解并确保那一点时，`RefCell<T>` 类型便是有用的了。

与 `Rc<T>` 类似，`RefCell<T>` 仅用于一些单线程场景中，并在咱们尝试将其用于多线程语境中时，将给到一个编译时报错。在第 16 章将讲到怎样在多线程的程序中，获得 `RefCell<T>` 的功能。

以下为因何原因而选择 `Box<T>`、`Rc<T>` 或 `RefCell<T>` 的总结：

- `Rc<T>` 实现了同一数据的多个所有者；`Box<T>` 与 `RefCell<T>` 都有着单个所有者；
- `Box<T>` 实现了编译时的可变或不可变借用检查；`Rc<T>` 仅实现了编译时的不可变借用检查；`RefCell<T>` 则实现了在运行时的可变及不可变借用检查；
- 由于 `RefCell<T>` 实现了运行时的可变借用检查，因此即是某个 `RefCell<T>` 是不可变的，咱们也可以其内部的值。

修改某个不可变值内部的值，即为 *内部可变性* 模式（the *interior mutability* pattern）。接下来就要看一个其中内部可变性有用的示例，并检视内部可变性是如何可行的。


### 内部可变性：到不可变值的可变借用

**Interior Mutability: A Mutable Borrow to an Immutable Value**

借用规则的一种后果，便是在有着某个不可变值时，是无法可变地借用他的。比如，下面的代码就不会编译：

```rust
fn main() {
    let x = 5;
    let y = &mut x;
}
```

在尝试编译此代码时，就会得到以下报错：

```console
$ cargo run                                                                                         lennyp@vm-manjaro
   Compiling sp_demos v0.1.0 (/home/lennyp/rust-lang/sp_demos)
error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
 --> src/main.rs:3:13
  |
2 |     let x = 5;
  |         - help: consider changing this to be mutable: `mut x`
3 |     let y = &mut x;
  |             ^^^^^^ cannot borrow as mutable

For more information about this error, try `rustc --explain E0596`.
error: could not compile `sp_demos` due to previous error;
```

然而，某个值在他的一些方法中对自身加以修改，而对别的代码表现出不可变，这种情况在一些场合是有用的。值那些方法外部的代码，将无法修改该值。使用 `RefCell<T>`，便是获得拥有内部可变性能力的一种途径，但 `RefCell<T>` 并不是完全绕开了借用规则：编译器中的借用检查，放行了这种内部可变性，而取而代之的是，借用规则在运行时得以检查。在违反这些规则时，就会得到一个 `panic!` 而非编译时错误。

接下来就要贯穿其中可使用 `RefCell<T>`，来修改某个不可变值，并发现为何这是有用的一个实际例子。


### 内部可变性的一个用例：模拟对象

**A Use Case for Interior Mutability: Mock Objects**

有的时候，在测试期间，编程者为了观察到特定行为，并断言该行为有被正确实现，就会在某个类型处使用另一类型。这样的占位类型，叫做 *测试替身，test double*。请将其设想为电影工业中的 “特技替身，stunt double”，即某人介入进来并代替某名演员完成特别棘手的一个场景。在测试时，测试替身代表了其他类型。所谓模拟对象，就是记录测试过程中，发生了些什么，如此咱们就可以确定出那些正确操作有发生的一些特定类型。

Rust 没有如同其他有着对象的语言，同样意义上的那些对象，且 Rust 没有如同一些其他语言那样，内建到标准库中的模拟对象功能。然而，咱们是绝对可以创建出，将起到与模拟对象相同的作用，这样的结构体的。

以下就是这里将测试的场景：这里将创建根据最大值而追踪某个值，并根据最大值与当前值的接近程度，发出一些消息的库。这样的库，比如就可被用于追踪用户的允许调用 API 次数配额。

这个库将提供对某个值接近最大值程度的追踪，以及在什么时刻发出什么消息的功能。使用这个库的应用，预期将提供发送消息的机制：应用可将某条消息放置于该应用中，或者发出一封电子邮件，或者发出一条手机短信，抑或别的什么。这个库则无需清楚那样的细节。他所需的全部，即是实现一个这里将提供的、名为 `Messenger` 的一个特质。下面清单 15-20 给出了该库的代码：


文件名：`src/lib.rs`

```rust
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("出错：你已超出你的配额！");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("紧急警告：你已用掉你配额的 90% ！");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("警告：你已用掉你配额的 75% ！");
        }
    }
}
```

*清单 15-20：就某个值与最大值接近程度加以追踪，并在该值处于不同水平时发出告警的一个库*

此代码的一个重要部分，即是有着一个取了到 `self` 的不可变引用与消息文本、名为 `send` 方法的那个 `Messenger` 特质。这个特质就是咱们的模拟对象所需实现的接口（the interface, 借鉴了 Java 语言的叫法，参见 [使用接口来拯救！](https://java.xfoss.com/ji-cheng-he-duo-tai-ji-zhi/ch08_interfaces_and_abstract_classes#interface_rescue)），从而这种模拟就可与真实对象的同样方式，而被使用。至于另一重要部分，则是这里打算测试 `LimitTracker` 上那个 `set_value` 方法的行为（注意：这里 `LimitTracker` 命名方式，同样借鉴了 Java 语言中类的命名约定）。这里可以改变所传入的那个 `value` 参数的值，但 `set_value` 不会返回任何咱们对其做出断言的东西。这里是要能够表达出，在咱们以实现了 `Messenger` 特质的某物，及 `max` 的某个特定值，而创建出一个 `LimitTracker` 下，当咱们传入不同数字的 `value` 时，那个信使方法，就被告知要发送一些恰当的消息。

这里所需的模拟对象，在调用 `send` 时，不是发送电子邮件或手机短信，而是将只追踪其被告知要发送的消息。这里可以创建出该模拟对象的一个新实例，然后创建一个用到这个模拟对象的 `LimitTracker`，接着调用 `LimitTracker` 上的那个 `set_value` 方法，并随后检查该模拟对象是否有着咱们期望的消息。下面清单 15-21 给出了实现一个模拟对象，来刚好完成这些步骤的一种尝试，但借用检查器不会放行这个尝试：

文件名：`src/lib.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        sent_messages: Vec<String>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: vec! [],
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_waring_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq! (mock_messenger.sent_messages.len(), 1);
    }
}
```

*清单 15-21：实现不被借用检查器允许的一个 `MockMessenger` 的尝试*


此测试代码定义了有着一个 `sent_messages` 字段的 `MockMessenger` 结构体，该字段有着用于跟踪其被告知要发送消息的一些 `String` 值的 `Vec` 类型变量。这里还定义了一个关联函数 `new`，来令到创建出以空消息清单开头的那些新 `MockMessenger` 类型值，便利起来。随后这里就为 `MockMessenger` 实现了那个 `Messenger` 特质，于是就可以将某个 `MockMessenger` 给到一个 `LimitTracker` 了。在那个 `send` 方法的定义中，这里将所传入的消息，取作了参数，并将其存储在 `MockMessenger` 的 `sent_messages` 清单中。

在那个测试中，所测试的是，当其中的 `LimitTracker` 被告知要将 `value`，设置为大于其中的 `max` 值的某个值时，会发生什么事情。首先，这里创建出了一个新的 `MockMessage`，他将以一个空的消息清单开始。随后这里创建了一个新的 `LimitTracker`，并给到他了到那个新 `MockMessenger` 的引用，以及 `100` 的 `max` 值。这里以值 `80` 调用了 `LitmitTracker` 上的 `set_value` 方法，而该值是大于 `75` 小于 `100` 的。随后这里断言了 `MockMessenger` 正追踪的那个消息清单，现在应有一条消息在其中。

然而，该测试有一个问题，如下所示：

```console
$ cargo test                                                                                   lennyp@vm-manjaro
   Compiling limit_tracker v0.1.0 (/home/lennyp/rust-lang/limit_tracker)
error[E0596]: cannot borrow `self.sent_messages` as mutable, as it is behind a `&` reference
  --> src/lib.rs:58:13
   |
2  |     fn send(&self, msg: &str);
   |             ----- help: consider changing that to be a mutable reference: `&mut self`
...
58 |             self.sent_messages.push(String::from(message));
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `self` is a `&` reference, so the data it refers to cannot be borrowed as mutable

For more information about this error, try `rustc --explain E0596`.
error: could not compile `limit_tracker` due to previous error
warning: build failed, waiting for other jobs to finish...
```

由于其中的 `send` 方法取了一个到 `self` 的不可变引用，因此这里是无法修改那个 `MockMessenger` 值来追踪到那些消息的。这里还不能采取报错文本中，使用 `&mut self` 取代的建议，这是由于随后 `send` 的签名，将不与 `Messenger` 特质定义中的函数签名相匹配（请尽情尝试，并观察会得到什么样的报错消息）。

这正是内部可变性可帮到忙的一种情形！下面就将把那个 `sent_messages` 存储于一个 `RefCell<T>` 内部，而接下来那个 `send` 方法，就将能够修改 `sent_messages`，以存储咱们曾见到过的那些消息了。下面清单 15-22 给出了那看起来的样子：

文件名：`src/lib.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec! []),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_waring_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq! (mock_messenger.sent_messages.borrow().len(), 1);
    }
}
```

*清单 15-22：在外层值被视为不可变的同时，使用 `RefCell<T>` 改变内层值*

那个 `sent_messages` 字段，现在就是类型 `RefCell<Vec<String>>`，而非 `Vec<String>` 的了。在其中的 `new` 函数里，这里围绕那个空矢量值，创建了一个新的 `RefCell<Vec<String>>` 实例。

而对于那个 `send` 方法的实现，首个参数认识 `self` 的一个不可变借用，这与那个特质定义是相匹配的。这里调用了 `self.sent_messages` 值中，那个 `RefCell<Vec<String>>` 类型实例上的 `borrow_mut` 方法，来获取了到这个 `RefCell<Vec<String>>` 实例内部值，亦即那个矢量值的一个可变引用。随后这里调用了到该矢量值可变引用上的 `push` 方法，来最终测试期间所发送的那些消息。

这里必须做出的最后一项修改，是在那个断言中：为了看到在那个内部矢量值中有多少的条目，这里就要调用那个 `RefCell<Vec<String>>` 上的 `borrow` 方法，来获取到其中矢量值的一个不可变引用。

既然咱们以及看到怎样使用 `RefCell<T>`，接下来就要探究其原理了！


### 使用 `RefCell<T>` 在运行时对借用进行追踪

**Keeping Track of Borrows at Runtime with `RefCell<T>`**

在创建不可变及可变引用时，咱们分别用到了 `&` 与 `&mut` 语法。而在 `RefCell<T>` 下，咱们使用的是 `borrow` 与 `borrow_mut` 两个方法，他们均为属于 `RefCell<T>` 那些安全 API 的一部分。其中 `borrow` 方法返回的是灵巧指针类型 `Ref<T>`（the smart pointer type `Ref<T>`），而 `borrow_mut` 则返回的是灵巧指针类型 `RefMut<T>`（the smart pointer type `RefMut<T>`）。这两种返回的类型，都实现了 `Deref` 特质，因此咱们就能向常规引用那样，对待他们。

`RefCell<T>` 追踪了有多少个当前活动的 `Ref<T>` 及 `RefMut<T>`。在每次于某个 `RefCell<T>` 值上调用 `borrow` 时，该 `RefCell<T>` 都会增加其有多少个活动不可变借用计数。而在某个 `Ref<T>` 值超出作用域时，该不可变借用计数，就会降低一个。跟编译时借用规则一样，`RefCell<T>` 允许在任何时刻，有着多个不可变借用或一个的可变借用。

在咱们尝试违反这些规则时，与之前在引用下得到编译器报错相反，`RefCell<T>` 的实现将在运行时终止运行。下面清单 15-23 就给出了清单 15-22 中那个 `send` 实现的修改版本。其中故意尝试创建出统一作用域的两个活动可变借用，来演示 `RefCell<T>` 会在运行时阻止咱们这样做。

文件名：`src/lib.rs`

```rust
    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            let mut borrow_one = self.sent_messages.borrow_mut();
            let mut borrow_two = self.sent_messages.borrow_mut();

            borrow_one.push(String::from(message));
            borrow_two.push(String::from(message));
        }
    }
```

*清单 15-23：创建出同一作用域中的两个可变引用，来发现 `RefCell<T>` 将终止运行*

这里创建了返回自 `borrow_mut` 的一个灵巧指针 `RefCell<T>` 的变量 `borrow_one`。随后这里以同样方式创建了变量 `borrow_two` 中的另一可变借用。这就在同一作用域中构造了两个可变引用，而这是不运行的。在咱们运行这个库的测试时，清单 15-23 中的代码将不带任何报错地编译，但测试将失败：

```console
$ cargo test                                                                    101 ✘  
   Compiling limit_tracker v0.1.0 (/home/peng/rust-lang/limit_tracker)
    Finished test [unoptimized + debuginfo] target(s) in 0.46s
     Running unittests src/lib.rs (target/debug/deps/limit_tracker-98d6159d1b15eb72)

running 1 test
test tests::it_sends_an_over_75_percent_waring_message ... FAILED

failures:

---- tests::it_sends_an_over_75_percent_waring_message stdout ----
thread 'tests::it_sends_an_over_75_percent_waring_message' panicked at 'already borrowed: BorrowMutError', src/lib.rs:60:53
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::it_sends_an_over_75_percent_waring_message

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`
```

请注意该代码是以消息 `already borrowed: BorrowMutError` 终止运行的。这正是 `RefCell<T>` 处理运行时违反借用规则的方式。

选择在运行时，而非编译时捕获借用报错，正如这里所做的那样，就意味着咱们潜在地会在程序开发过程中晚一点的时候，发现代码中的错误：那么就可能在直到代码部署到生产环境时，也没有发现这些错误。同时，咱们的代码还会因在运行时，而非编译时保持对那些借用的追踪，而遭受由此导致的性能代价。然而，`RefCell<T>` 的使用，令到在只允许使用一些不可变值的上下文中，编写出正使用着的，可对自身加以修改，从而跟踪其所见到的那些消息的模拟对象成为可能。咱们可在权衡了其弊端，及相交常规引用所能提供到的更多功能后，合理使用 `RefCell<T>` 这种灵巧指针。


### 通过结合 `Rc<T>` 与 `RefCell<T>`，实现可变数据的多个所有者

**Having Multiple Owners of Mutable Data by Combining `Rc<T>` and `RefCell<T>`**

使用 `RefCell<T>` 的一种常见方式，便是与 `Rc<T>` 结合运用。回顾 `Rc<T>` 实现了某个数据有着多个所有者，但其只给出到那个数据的不可变访问。在有着保存了一个 `RefCell<T>` 的 `Rc<T>` 时，咱们就可得到，一个可以有着多个所有者，*且* 咱们可以改变的值。

比如，回顾清单 15-18 中的那个构造列表示例，其中使用了 `Rc<T>` 来实现多个列表共用另一列表所有权。由于 `Rc<T>` 仅保存着一些不可变值，因此一旦咱们创建出了那些清单，就再不能修改其中的任何值。下面就要加入 `RefCell<T>`，来获得修改列表中那些值的能力。下面清单 15-24 给出了通过在那个 `Cons` 定义中，使用 `RefCell<T>`，咱们就可以修改存储在所有列表中值了。

文件名：`src/main.rs`

```rust
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println! ("之后的 a = {:?}", a);
    println! ("之后的 b = {:?}", b);
    println! ("之后的 c = {:?}", c);
}
```

*清单 15-24：运用 `Rc<RefCell<i32>>` 创建出可改变的 `List`*

这里创建了为 `Rc<RefCell<i32>>` 类型实例的一个值，并将其存储在名为 `value` 的一个变量中，如此咱们就可以在稍后直接访问他。接着这里在 `a` 中，创建了有着保存了 `value` 的 `Cons` 变种的一个 `List`。这里需要克隆 `value`，这样 `a` 与 `value` 都会有着那个内层值 `5` 的所有权，而非将所有权从 `value` 转移到 `a` 或让 `a` 从 `value` 借用。

这里把那个列表 `a`，封装在了一个 `Rc<T>` 中，进而在创建列表 `b` 与 `c` 时，二者都可以引用到 `a`，正如咱们在清单 15-18 中所做的那样。在这里已创建出 `a`、`b` 与 `c` 中的三个列表后，就打算把 `10` 加到 `value` 中的那个值。咱们是通过调用 `value` 上的 `borrow_mut` 方法做到这点的，这用到了第 5 章中曾讨论过的自动解引用特性（参见 [`->` 操作符去哪儿了？](Ch05_Using_Structs_to_Structure_Related_Data.md#--操作符the---operator哪去了呢)），来将这个 `Rc<T>` 解引用到内层的 `RefCell<T>` 值。这个 `borrow_mut` 方法返回的是一个 `RefMut<T>` 的灵巧指针，而咱们于其上使用了解引用操作符，并修改了那个内层值。

在打印 `a`、`b` 与 `c` 时，就可以看到他们都有了修改后的值 `15` 而非 `5`：

```console
$ cargo run                                                       lennyp@vm-manjaro
   Compiling cons_list_demo v0.1.0 (/home/lennyp/rust-lang/cons_list_demo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/cons_list_demo`
之后的 a = Cons(RefCell { value: 15 }, Nil)
之后的 b = Cons(RefCell { value: 3 }, Cons(RefCell { value: 15 }, Nil))
之后的 c = Cons(RefCell { value: 4 }, Cons(RefCell { value: 15 }, Nil))
```

这样的技巧是相当整洁的！通过使用 `RefCell<T>`，咱们就有了对外不可变的 `List` 值（an outwardly immutable `List` value）。而由于咱们可以使用 `RefCell<T>` 上提供了对其内部可变性访问的那些方法，因此就可以在需要的时候修改一些数据。运行时的借用规则检查，保护了咱们免于数据竞争，而有的时候为了数据结构中的此种灵活性，是值得拿运行速度来换取的。请注意 `RefCell<T>` 对于多线程代码是不生效的！`Mutex<T>` 即为线程安全版本的 `RefCell<T>`，而在第 16 章咱们就会讨论到 `Mutex<T>`。


## 引用循环会泄露内存

**Reference Cycles Can Leak Memory**

Rust 的内存安全，确保的是难于，但并非不可能，意外创建出绝不会被清理的内存（即所谓的 *内存泄露，memory leak*）。完全防止内存泄露，不是 Rust 那些保证之一，这就意味着在 Rust 中，内存泄露即为内存安全的（preventing memory leaks entirely is not one of Rust's gurantees, meaning memory leaks are memory safe in Rust）。通过使用 `Rc<T>` 及 `RefCell<T>` 就能发现，Rust 是允许内存泄露的：创建出其中以循环方式，指向对方的一些引用是有可能的。由于循环中各个引用条目的引用计数，将永远到不了 `0`，而这些值就永远不会被弃用，这就创造了内存泄露。


### 创建出循环引用

**Creaing a Reference Cycle**

下面以清单 15-25 中的 `List` 枚举及一个 `tail` 方法开始，来看看循环引用会怎样发生，以及怎样防止循环引用：


文件名：`src/main.rs`

```rust
use std::cell::RefCell;
use std::rc::Rc;
use crate::List::{Cons, Nil};

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

fn main() {}
```

*清单 15-25：保存着一个 `RefCell<T>`，从而可修改 `Cons` 变种指向何处的一个构造列表定义*

这里用的是清单 15-5 中那个 `List` 定义的另一变体。`Cons` 变种中的第二个元素，现在是 `RefCell<Rc<List>>`，表示这里不是要如同在清单 15-24 中所做的那样，具备修改那个 `i32` 值的能力，这里是要修改某个 `Cons` 变种所指向的那个 `List` 值。这里还添加了在有着某个 `Cons` 变种时，实现便于访问其第二个项目的 `tail` 方法。

在下面清单 15-26 中，咱们添加了用到清单 15-25 中那些定义的 `main` 函数。此代码创建了变量 `a` 中的一个清单，以及变量 `b` 中指向 `a` 中清单的一个清单。随后他将 `a` 中的清单指向了 `b`，这就创建了一个循环引用。其间有着一些 `println!` 语句，来显示此过程中不同点位的那些引用计数。


文件名：`src/main.rs`

```rust
fn maiN() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println! ("a 的初始 rc 计数 = {}", Rc::strong_count(&a));
    println! ("a 的下一条目 = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println! ("b 的创建后 a 的 rc 计数 = {}", Rc::strong_count(&a));
    println! ("b 的初始 rc 计数 = {}", Rc::strong_count(&b));
    println! ("b 的下一条目 = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println! ("在修改 a 之后 b 的 rc 计数 = {}", Rc::strong_count(&b));
    println! ("在修改 a 之后 a 的 rc 计数 = {}", Rc::strong_count(&a));

    // 取消下面这行注释，就可以看到这里有着循环引用；
    // 他将溢出堆栈（it will overflow the stack）
    // println! ("a 的下一条目 = {:?}", a.tail());
}
```

*清单 15-26：创建出相互指向的两个 `List` 的循环引用*


这里创建出了保存着变量 `a` 中，初始列表 `5, Nil` 的一个 `Rc<List>` 实例。随后这里又创建了保存着变量 `b` 中包含了值 `10` 并指向了 `a` 中清单的另一个 `Rc<List>` 实例。

这里修改了 `a` 从而其指向了 `b` 而非 `Nil`，于是创建了一个循环。咱们是通过是要那个 `tail` 方法，来获得到 `a` 中那个 `RefCell<Rc<List>>` 的引用，这里将其放入到了变量 `link` 中。随后这里使用了这个 `RefCell<Rc<List>>` 上的 `borrow_mut` 方法，来将保存着 `Nil` 的一个 `Rc<List>`，修改为保存到 `b` 中的那个 `Rc<List>`。

在保持那最后一个 `println!` 被注释掉，而运行此代码时，就会得到下面的输出：

```console
$ cargo run                                                                           lennyp@vm-manjaro
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

在将 `a` 中的列表修改为指向 `b` 后，与 `b` 中的两个 `Rc<List>` 实例引用计数均为 `2`。在 `main` 的结尾，Rust 会弃用掉变量 `b`，这会将那个 `b`  `Rc<List>` 实例的引用计数，从 `2` 降低到 `1`。因为他的引用计数为 `1` 而不是 `0`，因此那个 `Rc<List>` 在内存堆上的内存，在此刻就不会被弃用。随后 Rust 会弃用 `a`，这同样会将那个 `a` `Rc<List>` 实例的引用计数，从 `2` 降低到 `1`。由于另一 `Rc<List>` 实例仍指向着他，因此该实例的内存也无法被弃用。那么分配给该清单的内存，将永不会被收回。为形象表示这个引用循环，这里创建出了下图 15-4 中的图示。

![相互指向的列表 `a` 与 `b` 的一个循环引用](images/15-04.svg)

*图 15-04：相互指向的列表 `a` 与 `b` 的一个循环引用*

在取消注释掉其中最后一个 `println!` 而运行该程序时，Rust 就会尝试以 `a` 指向 `b` 指向 `a` 如此往复，打印出这个循环，直到他溢出内存栈为止。

相较于真实世界的程序，这个示例中创建出循环引用的那些后果并不算非常残酷：在这里创建出那个循环引用之后，这个程序就立马结束了。然而，在更为复杂的程序在某个循环中分配了大量内存，并在其上耗费较长时间时，那么这个程序就会用到相比于其所需要更多的内存，且可能会是系统不堪重负，造成系统耗尽可用内存。

循环引用的创建并非一蹴而就，但也并非是不可能的。在有着包含着 `Rc<T>` 值的一些 `RefCell<T>` 值，或类似的带有内部可变性及引用计数的嵌套类型组合时，咱们就必须确保不会创建出循环引用。咱们不能依靠 Rust 来捕获到循环引用。创建出循环引用来，是属于程序中的逻辑错误，咱们应运用自动化测试、代码审阅，及其他一些软件开发实践来消除。

避免循环引用的另一种方案，便是重组咱们的数据结构，从而实现一些引用表达所有权，而一些引用则不是。结果就是，咱们是可以有着由一些所有权关系，与一些非所有权关系构成的循环，而只有所有权关系会影响到某个值是否可被丢弃。在清单 15-25 中，咱们是一直要那些 `Cons` 变种，拥有他们清单的所有权，那么重组其中的数据结构就是不可行的。接下来要看到用到了由一些父节点与子节点组成的图数据结构（graphs made up of parent nodes and child nodes）的示例，来发现在什么时候，非所有权关系是防止循环引用的恰当方式。


### 防止引用循环：将 `Rc<T>` 转变为 `Weak<T>`

到目前为止，咱们已经证实了调用 `Rc::clone` 会增加某个 `Rc<T>` 示例的 `strong_count`，同时 `Rc<T>` 示例只会在其 `strong_count` 为 `0` 时被清理掉。咱们还可以通过调用 `Rc::downgrade` 并传入一个到某个 `Rc<T>` 的引用，而创建出到该 `Rc<T>` 实例中值的 *弱引用，weak reference*。强引用是咱们可共用某个 `Rc<T>` 实例的方式。弱引用并不表示某种所有权关系，且在某个 `Rc<T>` 实例被清理掉时，他们的计数不会受影响。由于在一旦所涉及到那些值的强引用计数为 `0` 时，涉及到弱引用的全部循环都将被破坏，因此弱引用就不会导致循环引用（weak references don't express an ownership relationship, and their count doesn't affect when an `Rc<T>` instance is cleaned up. They won't cause a reference cycle because any cycle involving some weak references will be broken once the strong reference count of values involved is `0`）。

在调用 `Rc::downgrade` 时，就会得到类型 `Weak<T>` 的灵巧指针。调用 `Rc::downgrade` 不是把 `Rc<T>` 实例中的 `strong_count` 加 `1`，而是把 `weak_count` 加 `1`。与 `strong_count` 类似，`Rc<T>` 类型使用 `weak_count` 来追踪存在多少个 `Weak<T>`。不同之处在于，对于 `Rc<T>` 的被清理，是无需 `weak_count` 为 `0` 的。

由于 `Weak<T>` 所引用的值，可能已被启用了，因此在以某个 `Weak<T>` 所指向值来完成任何事情时，咱们必须确保那个值仍是存在的。而要确保这一点，是通过在 `Weak<T>` 实例上调用 `upgrade` 方法实现的，该方法将返回一个 `Option<Rc<T>>` 值。在那个 `Rc<T>` 值尚未被弃用时，咱们就会得到一个 `Some` 的结果，而若那个 `Rc<T>` 值已被弃用，则就会得到 `None` 的结果。由于 `upgrade` 返回的是一个 `Option<Rc<T>>`，Rust 就将确保 `Some` 与 `None` 两种情形都被处理，进而就将不会有无效指针。

下面的示例，这里将创建其条目了解各自的子条目 *以及* 各自的父条目的一种树形数据结构，而非之前的其条目仅了解其下一条目的列表数据结构。


**创建一种树形数据结构：有着字节点的节点, Creating a Tree Data Structure: a Node with Child Nodes**

作为开头，这里将构建有着了解其子节点的一些节点。这里将创建出一个名为 `Node` 的结构体，保存着自身的 `i32` 值，以及到其子 `Node` 值的一些引用。

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

这里要的是某个 `Node` 拥有其子节点，并想要以一些变量，来共用那样的所有权，从而就可以直接访问树中的各个 `Node`（we want a `Node` to own its children, and we want to share that ownership with variables so we can access each `Node` in the tree directly）。为了完成这一点，这里把其中的那些 `Vec<T>` 条目，定义为了类型 `Rc<Node>` 的一些值。这里还打算修改哪些节点是另一节点的子节点，因此这里就有一个在 `children` 字段中，包裹着 `Vec<Rc<Node>>` 的 `RefCell<T>`。

接下来，这里就将使用这个结构体定义，并创建出有着值 `3` 而没有子节点的一个名为 `leaf` 的 `Node` 实例，以及另一个有着值 `5` 及将 `leaf` 作为其子节点的 `branch` 实例，如下清单 15-27 中所示：

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

*清单 15-27：创建出没有子节点的一个 `leaf` 节点及将 `leaf` 作为其一个子节点的 `branch` 节点*


这里克隆了 `leaf` 中的 `Rc<Node>` 并将其存储在了 `branch` 中，表示 `leaf` 中的 `Node` 现在有了两个所有者：`leaf` 与 `branch`。这里就可以经由 `branch.children`，从 `branch` 到达 `leaf`，但并无从 `leaf` 到 `branch` 的途径。原因就在于 `leaf` 没有到 `branch` 的引用，而就不知道他们是相关的。这里想要 `leaf` 明白，`branch` 是其父节点。接下来就要完成这一点。


**在子节点中添加到其父节点的引用，Adding a Reference from a Child to Its Parent**

要让那个字节点了解他的父节点，这里就需要添加一个 `parent` 字段到这里的 `Node` 结构体定义。麻烦在于确定出 `parent` 字段应为何种类型。咱们清楚他不能包含一个 `Rc<T>`，因为那样就会以 `leaf.parent` 指向 `branch` 且 `branch.children` 指向 `leaf`，而创建出循环引用，这将导致他们的 `strong_count` 值用不为零。

以另外一种方式，来设想这样的关系，父节点因拥有他的子节点：在父节点被弃用时，他的那些子节点也应被弃用。然而，子节点则不应拥有他的父节点：在咱们弃用某个子节点时，那个父节点应存在。这正是弱引用的情况！

因此这里将把 `parent` 字段的类型，构造为使用 `Weak<T>`，具体而言就是 `RefCell<Weak<Node>>`，而非 `Rc<T>`。现在这个 `Node` 结构体定义看起来像下面这样：

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

节点将能够引用他的副节点，而不拥有父节点。在下面清单 15-28 中，把 `main` 更新为使用这个新定义，进而 `leaf` 节点将有引用其父节点，`branch` 的一种途径：

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

*清单 15-28：带有到其父节点 `branch` 的弱引用的 `leaf` 节点*

这个 `leaf` 节点的创建，与清单 15-27 类似，除了其中的 `parent` 字段：`leaf` 以不带父节点开始，因此这里创建了一个新的、空 `Weak<Node>` 引用实例。

到这里，在咱们尝试通过使用 `upgrade` 方法，获取 `leaf` 的父节点的引用时，就会得到一个 `None` 值。在首个 `println!` 语句的输出中，就看到了这点：

```console
叶子节点的父节点 = None
```

在创建那个 `branch` 节点时，由于 `branch` 没有父节点，他也将有一个 `parent` 字段中的新 `Weak<Node>` 引用。这里仍将 `leaf` 作为 `branch` 的子节点之一。一旦咱们有了 `branch` 变量中的那个 `Node` 实例，就可以修改 `leaf`，来给到他一个到其父节点的 `Weak<Node>` 引用。这里使用了 `leaf` 的 `parent` 字段中，`RefCell<Weak<Node>>` 上的 `borrow_mut` 方法，并于随后使用了 `Rc::downgrade` 函数，来子 `branch` 变量中的那个 `Rc<Node>`，创建出到 `branch` 的 `Weak<Node>` 引用。

当咱们再度打印 `leaf` 的父节点时，这次就会得到保存着 `branch` 的一个 `Some` 变种：现在 `leaf` 就可以访问其父节点了！在打印 `leaf` 时，同样避免了清单 15-26 中曾有过的，最终以栈一出而告终的那个循环引用；其中的 `Weak<Node>` 引用，是作为 `(Weak)` 被打印出的：

```console
叶子节点的父节点 = Some(Node { value: 5, parent: RefCell { value: (Weak) }, children: RefCell { value: [Node { value: 3, parent: RefCell { value: (Weak) }, children: RefCell { value: [] } }] } })
```

没有了无限输出，就表示此代码并未创建出循环引用。咱们还可以通过查看从调用 `Rc::strong_count` 与 `Rc::weak_count` 得到的值，来说明这一点。


**`strong_count` 与 `weak_count` 变化的直观表示，Visualizing Changes to `strong_count` and `weak_count`**

下面来看看这些 `Rc<Node>` 实例，是怎样通过创建出新的内层作用域，并将 `branch` 定义迁移到那个作用域而变化的。通过这样做，咱们就可以看到在 `branch` 被创建出，及在其超出作用域而被弃用时，会发生什么。下面清单 15-29 中给出了这些修改：

文件名：`src/main.rs`

```rust
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec! []),
    });

    println! (
        "叶子节点的强引用计数：{}，弱引用计数：{}\n",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec! [Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println! (
            "枝干节点的强引用计数：{}，弱引用计数：{}\n",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );
        println! (
            "叶子节点的强引用计数：{}，弱引用计数：{}\n",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );

    }
    println! ("叶子节点的父节点 = {:?}\n", leaf.parent.borrow().upgrade());
    println! (
        "叶子节点的强引用计数：{}，弱引用计数：{}\n",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
```

*清单 15-29：在内层作用域中创建 `branch` 并对那些强弱引用计数进行检查*

在 `leaf` 节点被创建出了后，其 `Rc<Node>` 便有了强引用计数 `1` 及弱引用计数 `0`。在那个内层作用域中，这里创建了 `branch` 并将其与 `leaf` 关联，在打印两种计数的那个时间点，`branch` 中 `Rc<Node>` 将有着强计数 `1` 与弱计数 `1`（由于 `leaf.parent` 以一个 `Weak<Node>` 指向了 `branch`）。在打印 `leaf` 中的两个计数时，由于 `branch` 现在有着存储在 `branch.childeren` 中，`leaf` 的 `Rc<Node>` 的一份克隆，因此咱们就会看到他将有着强引用计数 `2`，而他仍将有着弱引用计数 `0`。

在那个内存作用域结束时，`brach` 就超出了作用域，而那个 `Rc<Node>` 的强引用计数就会降低到 `0`，因此他的 `Node` 就被丢弃了。源自 `leaf.parent` 的弱引用计数 `1`，与这个 `Node` 是否被弃用无关，因此这里就不会得到任何内存泄露！

在那个内层作用域结束之后，若咱们尝试访问 `leaf` 的父节点，就将再度得到 `None`。在该程序结束处，由于变量 `leaf` 此时又仅是到那个 `Rc<Node>` 的唯一引用，因此他里面的 `Rc<Node>`，将有着强引用计数 `1` 与弱引用计数 `0`。

管理这两种计数与值的弃用的全部逻辑，都被内建到了 `Rc<T>` 与 `Weak<T>`，以及二者的 `Drop` 特质实现中。通过在 `Node` 定义中，指明某个子节点到其父节点的关系，应为 `Weak<T>` 的引用，咱们就能够在不创建出循环引用与内存泄露之下，让父节点指向子节点，并反过来让子节点也指向父节点。


## 本章小节

本章涵盖了怎样运用灵巧指针，来做出相比与 Rust 默认在常规引用下，所做出的不同保证及权衡（this chapter covered how to use smart pointers to make different gurantees and trade-offs from those Rust makes by default with regular references）。其中的 `Box<T>` 类型，有着已知大小，并指向分配在内存堆上的数据。而 `Rc<T>` 类型，则对到内存堆上数据的引用数量加以追踪，因此那个数据变可以有多个所有者。`RefCell<T>` 类型，以其内部可变性，而给到在需要一种不可变类型，却又需要修改那种类型的内层值时，咱们可用的一种类型；这种类型还强制要求在运行时，而非编译时的借用规则检查。

本章还讨论了 `Deref` 与 `Drop` 两个特质，他们实现了灵巧指针的很多功能。这里探讨了可导致内存泄露的循环引用，以及怎样运用 `Weak<T>` 来防止他们。

若这一章激发了你的兴趣，而打算实现自己的灵巧指针，那么请查看 [The Rustonomicon](https://doc.rust-lang.org/nomicon/index.html) 了解更多有用信息。

接下来，咱们就将谈谈 Rust 中的并发问题了。咱们将了解到少数几个新的灵巧指针。
