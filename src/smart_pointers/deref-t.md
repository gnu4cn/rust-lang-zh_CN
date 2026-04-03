# 将灵巧指针视为普通引用

实现 `Deref` 特质允许咱们定制 *解引用运算符* `*`（请不要与乘法或通配符混淆）的行为。通过以这种方式实现 `Deref`，灵巧指针可被视为普通引用，咱们可以编写对引用进行操作的代码，并也可以对灵巧指针使用该代码。

咱们首先来看看解除引用运算符对常规引用的工作原理。然后，我们将尝试定义一个行为类似于 `Box<T>` 的自定义类型，并了解为什么解引用运算符没有像我们新定义的类型上的引用那样工作。我们将探讨实现 `Deref` 特质，是如何使灵巧指针以类似于引用的方式工作成为可能的。然后，我们将研究 Rust 的解引用强制转换特性，以及他是如何让咱们既可以使用引用，又可以使用灵巧指针的。


## 沿着引用前往值

普通引用属于一种指针，而看待指针的一种方式是，指向存储于别处的值的箭头。在下面清单 15-6 中，我们创建了一个到 `i32` 值的引用，然后使用解引用运算符来跟随该引用前往值。

<a name="listing_15-6"></a>
文件名：`src/main.rs`

```rust
fn main() {
    let x = 5;
    let y = &x;

    assert_eq! (5, x);
    assert_eq! (5, *y);
}
```

**清单 15-6**：使用解引用运算符来跟随引用前往 `i32` 值

变量 `x` 保存着 `i32` 值 `5`。我们设置 `y` 等于到 `x` 的引用。我们可以断言 `x` 等于 `5`。但是，当我们打算对 `y` 中的值进行断言时，我们必须使用 `*y` 来跟随这个引用前往他指向的值（因此，叫做 *解引用*），以便编译器可以比较实际值。一旦解引用了 `y`，我们就有了我们可与 `5` 比较的， `y` 指向的整数值的访问权限。

相反，若我们尝试写下 `assert_eq! (5, y);`，我们就会得到下面这个编译报错：

```console
$ cargo run
   Compiling deref-example v0.1.0 (/home/hector/rust-lang-zh_CN/projects/deref-example)
error[E0277]: can't compare `{integer}` with `&{integer}`
 --> src/main.rs:6:5
  |
6 |     assert_eq! (5, y);
  |     ^^^^^^^^^^^^^^^^^ no implementation for `{integer} == &{integer}`
  |
  = help: the trait `PartialEq<&{integer}>` is not implemented for `{integer}`
  = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0277`.
error: could not compile `deref-example` (bin "deref-example") due to 1 previous error
```

比较数字与对数字的引用是不允许的，因为他们属于不同的类型。我们必须使用解引用运算符来跟随引用前往其所指向的值。


## 像引用一样使用 `Box<T>`

我们可以重写清单 15-6 中的代码为使用 `Box<T>` 而不是引用；下面清单 15-7 中对 `Box<T>` 使用的解引用运算符，与清单 15-6 中对引用使用的作用方式相同：

<a name="listing_15-7"></a>
文件名：`src/main.rs`

```rust
fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq! (5, x);
    assert_eq! (5, *y);
}
```

**清单 15-7**：对 `Box<i32>` 使用解引用运算符

清单 15-7 和清单 15-6 之间的主要区别在于，这里我们设置 `y` 为指向 `x` 的拷贝值的匣子类型的实例，而不是指向 `x` 值的引用。在最后的断言中，我们可以像 `y` 仍然是个引用那样， 使用解除引用运算符来跟随匣子的指针。接下来，我们将通过定义我们自己的匣子类型，探讨 `Box<T>` 有什么特别之处，使我们能够使用解引用运算符。


## 定义我们自己的灵巧指针

我们来构建一个类似于标准库提供的 `Box<T>` 类型的灵巧指针，以了解默认情况下灵巧指针类型与引用的行为方式有何不同。然后，我们将探讨怎样添加使用解除引用运算符的能力。

> **注意**：咱们即将构建的 `MyBox<T>` 类型与真正的 `Box<T>` 之间有个很大的区别：我们的版本不会存储数据在堆上。我们把这个示例的重点放在 `Deref` 上，因此相比类似指针的行为，数据实际存储在何处并不重要。

`Box<T>` 最终最终被定义为带有一个元素的元组结构体，因此清单 15-8 以同样方式定义了 `MyBox<T>` 类型。我们还将定义一个 `new` 函数，以与定义在 `Box<T>` 的 `new` 函数保持一致。

<a name="listing_15-8"></a>
文件名：`src/main.rs`

```rust
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
```

**清单 15-8**：定义 `MyBox<T>` 类型

我们定义了个名为 `MyBox` 的结构体，并声明了个泛型参数 `T`，因为我们希望我们的类型可以容纳任何类型的值。`MyBox` 类型是个元组结构体，带有一个类型为 `T` 的元素。`MyBox::new` 函数取一个类型 `T` 的参数，并返回一个包含传入值的 `MyBox` 的实例。

我们来试着添加清单 15-7 中的 `main` 函数到清单 15-8 中，并修改他为使用我们定义的 `MyBox<T>` 类型，而不是 `Box<T>`。清单 15-9 中的代码不会编译，因为 Rust 不知道怎样解引用 `MyBox`。

<a name="listing_15-9"></a>
文件名：`src/main.rs`

```rust
fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq! (5, x);
    assert_eq! (5, *y);
}
```

*清单 15-9：试图以咱们使用引用和 `Box<T>` 的方式使用 `MyBox<T>`*

下面是产生的编译报错：

```console
$ cargo run
   Compiling deref-example v0.1.0 (/home/hector/rust-lang-zh_CN/projects/deref-example)
error[E0614]: type `MyBox<{integer}>` cannot be dereferenced
  --> src/main.rs:14:20
   |
14 |     assert_eq! (5, *y);
   |                    ^^ can't be dereferenced

For more information about this error, try `rustc --explain E0614`.
error: could not compile `deref-example` (bin "deref-example") due to 1 previous error
```

我们的 `MyBox<T>` 类型无法被解引用，因为我们尚未对我们的类型实现这一能力。为了使 `*` 运算符下的解引用可行，我们就要实现 `Deref` 特质。


## 实现 Deref 特质

正如在第 10 章 中 [对类型实现特质](../generic_types_traits_and_lifetimes/traits.md#对类型实现特质) 小节中讨论的，要实现某个特质，我们需要提供该特质的必需方法的实现。标准库提供的 `Deref` 特质要求我们实现一个名为 `deref` 的方法，该方法会借用 `self` 并返回对内部数据的引用。下面清单 15-10 包含要添加到 `MyBox` 的定义的 `Deref` 的实现。

<a name="listing_15-10"></a>
文件名：`src/main.rs`

```rust
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```

**清单 15-10**：对 `MyBox<T>` 实现 `Deref`

`type Target = T;` 语法定义了个供 `Deref` 特质使用的关联类型。关联类型属于声明泛型参数的一种略有不同的方式，但现在咱们无需担心他们；我们将在第 20 章中更详细地介绍他们。

我们以 `&self.0` 填入 `deref` 方法的主体，以便 `deref` 返回一个到我们打算以 `*` 运算符访问的值的引用；回顾第 5 章中 [以元组结构体创建不同类型](../structs/defining_and_instantiating.md#以元组结构体创建不同类型) 中，`.0` 可以访问元组结构体中的第一个值。清单 15-9 中对 `MyBox<T>` 值调用 `*` 的 `main` 函数现在会编译，且两个断言都会通过!

在没有 `Deref` 特质下，编译器就只能解引用 `&` 的引用。`deref` 方法给予编译器取实现 `Deref` 的任何类型的值，并调用 `deref` 方法来获取一个他知道怎样解引用的引用的能力。

当我们在清单 `15-9` 中输入 `*y` 时，Rust 在幕后实际上运行了下面这行代码：

```rust
*(y.deref())
```

Rust 以对 `deref` 方法的调用替换 `*` 运算符，然后进行普通解引用，这样我们就不必考虑是否需要调用 `deref` 方法。这一 Rust 特性让我们可以编写作用一致的代码，无论我们有个普通引用，还是有个实现 `Deref` 的类型。

`deref` 方法返回对值的引用，以及 `*(y.deref())` 中括号外的普通解引用仍然是必要的原因，与所有权系统有关。若 `deref` 方法直接返回值，而不是对值的引用时，那么值就会从 `self` 中迁出。在这种情况下，或者在大多数使用解引用运算符的情况下，我们都不希望取得 `MyBox<T>` 内的内部值的所有权。

请注意，每次我们在代码中使用 `*` 时， `*` 运算符都会被替换以对 `deref` 方法的调用，然后仅一次对 `*` 运算符的调用。因为 `*` 运算符的替换不会无限递归，所以我们会以类型 `i32` 的数据结束，其与清单 15-9 中 `assert_eq!` 中的 `5` 一致。


## 在函数与方法中使用解引用强制转换

*解引用强制转换* 会转换实现 `Deref` 特质的类型的引用为另一类型的引用。例如，解引用强制转换可以转换 `&String` 为 `&str`，因为 `String` 实现了 `Deref`，因此他返回 `&str`。解引用强制转换是 Rust 对函数和方法的参数执行的一项便利操作，且仅对实现 `Deref` 特质的类型起作用。当我们作为参数传递特定类型的值的引用给函数或方法，而该引用与函数或方法定义中的参数类型不一致时，他会自动发生。对 `Deref` 方法的调用序列，会转换我们提供的类型为参数需要的类型。


> **译注**：
>
> 在面向对象编程语言 Java 中，类似的特性叫 ["自动装箱"](https://java.xfoss.com/Ch10_Numbers_and_Statics_Numbers_Matter.html#%E8%87%AA%E5%8A%A8%E8%A3%85%E7%AE%B1%E6%A8%A1%E7%B3%8A%E5%8E%9F%E7%94%9F%E5%80%BC%E4%B8%8E%E5%AF%B9%E8%B1%A1%E4%B9%8B%E9%97%B4%E7%9A%84%E7%95%8C%E7%BA%BF)。

解引用强制转换特性之所以添加到 Rust 中，是为编写函数和方法调用的程序员，无需以 `&` 和 `*` 添加过多的显式引用和解引用。解引用强制转换特性，还让我们可以写出更多既可以用于引用，也可以用于灵巧指针的代码。

为了看到解引用强制转换的实际效果，我们来使用我们在清单 15-8 中定义的 `MyBox<T>` 类型，以及我们在清单 15-10 中添加的 `Deref` 的实现。下面清单 15-11 展示了一个函数定义，其有着一个字符串切片的参数。

<a name="listing_15-11"></a>
文件名：`src/main.rs`

```rust
fn hello(name: &str) {
    println! ("你好，{name}！");
}
```

**清单 15-11**：`hello` 函数，有着 `&str` 类型的参数 `name`

我们可以一个字符串切片作为参数调用 `hello` 函数，例如 `hello("Rust");`。解引用强制转换使得以对 `MyBox<String>` 类型的值的引用，调用 `hello` 可行，如下清单 15-12 中所示：

<a name="listing_15-12"></a>
文件名：`src/main.rs`

```rust
fn main() {
    hello("Rust");

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
```

**清单 15-12**：以对 `MyBox<String>` 值的引用调用 `hello`，由于解引用强制转换这可以正常工作

在这里，我们以参数 `&m` 调用 `hello` 函数，这是个对 `MyBox<String>` 值的引用。因为我们在清单 15-10 中对 `MyBox<T>` 实现了 `Deref` 特质，Rust 可以通过调用 `deref` 转换 `&MyBox<String>` 为 `&String`。标准库提供了 `String` 上的 `Deref` 的实现，其返回一个字符串片，这位于 `Deref` 的 API 文档中。Rust 会再次调用 `deref` 来转换 `&String` 为 `&str`，这与 `hello` 函数的定义一致。

若 Rust 没有实现解引用强制转换，为了以 `&MyBox<String>` 类型的值调用 `hello`，我们就必须编写下面清单 15-13 中的代码，而不是清单 15-12 中的代码。

<a name="listing_15-13"></a>
文件名：`src/main.rs`

```rust
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
}
```

**清单 15-13**：若 Rust 没有解引用强制转换，我们将必须编写的代码

其中的 `(*m)` 解引用 `MyBox<String>` 为 `String`。然后，`&` 和 `[..]` 取与整个字符串相等的该 `String` 的字符串切片，以与 `hello` 的签名一致。这段不带解引用强制转换的代码，在涉及所有这些符号下，更难阅读、编写和理解。解引用强制转换允许 Rust 自动为我们处理这些转换。

当所涉及的类型定义了 `Deref` 特质时，Rust 将分析这些类型，并根据需要多次使用 `Deref::deref`，来获得与参数类型一致的引用。需要插入 `Deref::deref` 的次数会在编译时确定，因此利用解引用强制转换没有运行时损失！


## 处理可变引用下的解引用强制转换

与咱们使用 `Deref` 特质重写不可变引用上的 `*` 运算符类似，咱们可以使用 `DerefMut` 特质重写可变引用上的 `*` 运算符。

Rust 会在发现以下三种情形下的类型与特质实现时，执行解引用强制转换：

1. 当 `T: Deref<Target=U>` 时，会从 `&T` 强制转换为 `&U`;
2. 当 `T: DerefMut<Target=U>` 时，会从 `&mut T` 强制转换为 `&mut U`;
3. 当 `T: Deref<Target=U>` 时，会从 `&mut T` 强制转换为 `&U`。

前两种情况相同，只是第二种情况实现了可变性。第一种情况表明，当咱们有个 `&T`，并且 `T` 对某种类型 `U` 实现了 `Deref` 时，咱们可以透明地获得 `&U`。第二种情况表明，对于可变引用，也会发生相同的解引用强制转换。

第三种情况比较棘手：Rust 还会强制转换可变引用为不可变引用。但反过来是 *不* 可行的：不可变引用永远不会强制转换为可变引用。由于借用规则，当咱们有个可变引用时，该可变引用必须是对该数据的唯一引用（否则，程序将不编译）。转换一个可变引用为一个不可变引用，永远不会违反借用规则。而转换不可变引用为可变引用，将要求初始不可变引用是对该数据的唯一不可变引用，但借用规则不能保证这一点。因此，Rust 无法做出转换不可变引用为可变引用是可行的假设。


（End）


