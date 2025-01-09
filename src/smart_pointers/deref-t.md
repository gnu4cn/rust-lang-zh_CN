# 使用 `Deref` 特质将智能指针视为常规引用

**Treating Smart Pointers Like Regular References with `Deref` Trait**


实现 `Deref` 特质允许咱们自定义 *解引用操作符, the dereference operator*️'*' （不要与乘法或 glob 运算符相混淆）的行为。通过实现 `Deref`，智能指针可以被当作普通的引用来对待，咱们便可编写对引用进行操作的代码，并将该代码也用于智能指针。

咱们首先来看看解除引用操作符是如何在常规引用中工作的。然后咱们将尝试定义一个行为类似于 `Box<T>` 的自定义类型，并看看为什么解除引用操作符在咱们新定义的类型上不像引用那样工作。咱们将探讨实现 `Deref` 特性如何使智能指针的工作方式与引用相似。然后咱们将看看 Rust 的 *解引用强制转换，deref coercion* 特性，以及其如何让咱们使用引用或智能指针工作的。

> 注意：咱们将要建立的 `MyBox<T>` 类型和真正的 `Box<T>` 之间有一个很大的区别：咱们的版本不会将其数据存储在堆中。咱们把这个例子的重点放在 `Deref` 上，所以数据实际存储在哪里并不重要，重要的是类似指针的行为。


## 顺着指针找到值

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


## 像引用一样使用 `Box<T>`

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

清单 15-7 和清单 15-6 之间的主要区别在于，这里我们将 `y` 设置为指向 `x` 的拷贝值的 `Box<T>` 实例，而不是指向 `x` 值的引用。在最后的断言中，我们可以使用解除引用操作符来跟随 `Box<T>` 的指针，就像我们在 `y` 是一个引用时一样。接下来，我们将探讨 `Box<T>` 有什么特别之处，使我们能够通过定义我们自己的类型来使用解引用操作符。


## 定义咱们自己的灵巧指针


咱们来建立一个类似于标准库提供的 `Box<T>` 类型的灵巧指针，以体验灵巧指针的行为与默认的引用有什么不同。然后咱们将看看如何增加使用解除引用操作符的能力。

`Box<T>` 最终被定义为了具有一个元素的元组结构体，a tuple struct，因此清单 15-8 以同样方式，定义了一个 `MyBox<T>` 类型。咱们还将定义一个 `new` 函数，来匹配在 `Box<T>` 上定义的 `new` 函数。

文件名：`src/main.rs`

```rust
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
```

*清单 15-8：定义 `MyBox<T>` 类型*

我们定义了一个名为 `MyBox` 的结构，并声明了一个通用参数 `T`，因为我们希望我们的类型可以容纳任何类型的值。`MyBox` 类型是一个元组结构，其中一个元素为 `T` 类型。`MyBox::new` 函数接收一个 `T` 类型的参数，并返回一个 `MyBox` 实例，该实例保存着传入的值。

我们来试着将清单 15-7 中的 `main` 函数添加到清单 15-8 中，并将其改为使用我们定义的 `MyBox<T>` 类型而不是 `Box<T>`。清单 15-9 中的代码不会被编译，因为 Rust 不知道如何解除对 `MyBox` 的引用。

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

下面就是产生的编译报错：

```console
$ cargo run
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


## 通过实现 Deref 特质将类型视为引用

**Treating a Type Like a Reference by Implementing the `Deref` Trait**


正如第 10 章 ["在类型上实现特质"](Ch10_Generic_Types_Traits_and_Lifetimes.md#在类型上实现某个特质) 小节中所讨论的，要实现某个特质，咱们需要为该特质的必要方法提供实现。由标准库提供的 `Deref` 特质，要求咱们实现一个名为 `deref` 的方法，该方法借用 `self` 并返回对内部数据的引用。下面清单 15-10 包含 `Deref` 的一个实现，来添加到 `MyBox` 的定义中：

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

`type Target = T;` 语法定义了一个关联类型，an associated type，供 `Deref` 特质使用。关联类型是声明泛型参数的一种些许不同的方式，但现在咱们无需担心他们；咱们将在第 19 章中更详细地介绍他们。

我们在 `deref` 方法的主体中填入 `&self.0`，这样 `deref` 就会返回一个我们想用 `*` 操作符访问的值的引用；回顾一下第五章 [“使用没有命名字段的元组结构体来创建不同的类型”](Ch05_Using_Structs_to_Structure_Related_Data.md#使用没有命名字段的元组结构体来创建不同的类型) 小节，`.0` 可以访问一个元组结构体中的第一个值。清单 15-9 中调用 `MyBox<T>` 值的 `main` 函数现在可以编译了，而且断言也通过了!

如果没有 `Deref` 特质，编译器只能对 `&` 引用进行解引用。`deref` 方法给了编译器这样的能力：取一个实现 `Deref` 的任何类型的值，并调用 `deref` 方法来获得一个他知道如何解除引用的 `&` 引用。

当我们在清单 `15-9` 中输入 `*y` 时，在幕后 Rust 实际上运行了下面这段代码：

```rust
*(y.deref())
```

Rust 将 `*` 运算符替换为对 `deref` 方法的调用，然后是普通的解引用，这样咱们就不必考虑是否需要调用 `deref` 方法。这个 Rust 特性让我们可以编写功能相同的代码，无论我们有一个常规引用还是一个实现了 `Deref` 的类型。

`deref` 方法返回一个值的引用，以及 `*(y.deref())` 中括号外的普通解引用仍然是必要的，其原因与所有权系统有关。如果 `deref` 方法直接返回值，而不是返回值的引用，值就会被移出 `self`。在这种情况下，或者在大多数使用解引用操作符的情况下，我们不希望取得 `MyBox<T>` 内部的值的所有权。

请注意，每次我们在代码中使用 `*` 时， `*` 运算符被替换为对 `deref` 方法的调用，然后仅调用一次 `*` 运算符。因为 `*` 运算符的替换不会无限递归，所以我们最终得到类型为 `i32` 的数据，他与清单 15-9 中 `assert_eq!` 中的 `5` 匹配。


## 函数与方法下的隐式解引用强制转换

**Implicit Deref Coercions with Functions and Methods**


*解引用强制转换，deref coercion* 将对实现了 `Deref` 特质的某种类型的引用，转换为对另一类型的引用。例如，解引用强制转换可以将 `&String` 转换为 `&str`，因为 `String` 实现了 `Deref`，所以他会返回 `&str`。解引用强制转换是 Rust 对函数和方法的参数执行的一种便利，只对实现 `Deref` 特质的类型起作用。当我们把对某一特定类型的值的引用作为参数传递给函数或方法时，它就会自动发生，而该参数与函数或方法定义中的参数类型并不匹配。一系列对 `Deref` 方法的调用将我们提供的类型转换为参数需要的类型。


> 注：在面向对象编程语言 Java 中，类似的特性叫 ["自动装箱"](https://java.xfoss.com/Ch10_Numbers_and_Statics_Numbers_Matter.html#自动装箱模糊原生值与对象之间的界线)。

解引用强制转换被添加到 Rust 中，这样编写函数和方法调用的程序员就不需要用 `&` 和 `*` 添加那么多显式引用和解引用。解引用强制转换特性，也让咱们写出更多既可以用于引用，也可以用于灵巧指针的代码。

为了看到解引用强制转换的作用，下面咱们来使用清单 15-8 中定义的 `MyBox<T>` 类型，以及清单 15-10 中添加的 `Deref` 的实现。清单 15-11 给出了一个有字符串切片参数的函数定义：

文件名：`src/main.rs`

```rust
fn hello(name: &str) {
    println! ("你好，{name}");
}
```

*清单 15-11：参数 `name` 为 `&str` 类型的 `hello` 函数*

我们可以用一个字符串切片作为参数来调用 `hello` 函数，例如 `hello("Rust");`。解引用强制转换使我们可以用对 `MyBox<String>` 类型值的引用来调用 `hello`，如清单 15-12 所示：

文件名：`src/main.rs`

```rust
fn main() {
    hello("Rust");

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
```

*清单 15-12：使用对 `MyBox<String>` 值的引用调用 `hello`，由于解引用强制转换，其可以工作*

这里我们用参数 `&m` 调用 `hello` 函数，他是对 `MyBox<String>` 值的一个引用。因为我们在清单 15-10 中对 `MyBox<T>` 实现了 `Deref` 特性，Rust 可以通过调用 `deref` 将 `&MyBox<String>` 变成 `&String`。标准库提供了一个 `String` 上的 `Deref` 的实现，其返回一个字符串片，这在 `Deref` 的 `API` 文档中。Rust 再次调用 `deref`，将 `&String` 变成 `&str`，这与 `hello` 函数的定义相符。

如果 Rust 没有实现解引用强制转换，为了用一个 `&MyBox<String>` 类型的值调用 `hello`，我们就必须写清单 15-13 中的代码，而不是清单 15-12 中的代码。

文件名：`src/main.rs`

```rust
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
}
```

*清单 15-13：如果 Rust 没有解引用强制转换，我们将不得不编写的代码*

`(*m)` 将 `MyBox<String>` 解引用为 `String`。然后 `&` 和 `[..]` 获取 `String` 等于整个字符串的一个字符串切片，以匹配 `hello` 的签名。由于涉及所有这些符号，这段没有解引用强制转换的代码更难阅读、编写和理解。 解引用强制转换允许 Rust 自动为我们处理这些转换。

在所涉及的类型定义了 `Deref` 特质时，Rust 将分析这些类型，并根据需要多次使用 `Deref::deref`，来获得与参数类型匹配的引用。所需插入 `Deref::deref` 次数，是在编译时就已确定，所以利用解引用强制转换的优势，没有运行时的代价！


## 解引用强制转换与可变性的互动方式

**How Deref Coercion Interacts with Mutability**


与使用 `Deref` 特质重写不可变引用上的 `*` 运算符类似，咱们可以使用 `DerefMut` 特质，重写可变引用上的 `*` 运算符。

在以下三种情形下找到类型与特质实现时，Rust 会执行解引用强制转换：

- 当 `T: Deref<Target=U>` 时，会从 `&T` 转换为 `&U`;
- 当 `T: DerefMut<Target=U>` 时，会从 `&mut T` 转换为 `&mut U`;
- 当 `T: Deref<Target=U>` 时，会从 `&mut T` 转换为 `&U`。

前两种情况彼此相同，只是第二种情况实现了可变性。第一种情况表明，如果咱们有一个 `&T`，并且 `T` 实现了对某种类型 `U` 的 `Deref`，咱们可以透明地得到一个 `&U`。第二种情况表明相同的解引用强制转换发生在可变引用上。

第三种情况比较棘手：Rust 还会将可变引用强制转换为不可变引用。但反过来是 *不* 可行的：不可变引用永远不会强制转换为可变引用。由于借用规则，如果咱们有一个可变引用，则该可变引用必须是对那个数据的唯一引用（否则，程序将无法编译）。将一个可变引用转换为一个不可变引用永远不会违反借用规则。将不可变引用转换为可变引用，则要求那个初始不可变引用是对那个数据的唯一不可变引用，但借用规则并不能保证这一点。因此，Rust 不能假设将不可变引用转换为可变引用是可行的。


（End）


