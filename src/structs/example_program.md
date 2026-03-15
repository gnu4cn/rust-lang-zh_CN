# 使用结构体的示例程序

为立即何时我们可能打算使用结构体，我们来编写个计算矩形面积的程序。我们将以使用单个变量开始，然后重构程序，直到咱们转而使用结构体。

我们来以 Cargo 构造一个名为 `rectangles` 的新二进制项目，其将取以像素为单位指定的矩形的宽和高，并计算矩形面积。下面清单 5-8 展示了个简短程序，有着在咱们项目的 `src/main.rs` 中完成这点的一种方式。


<a name="listing_5-8"></a>
文件名：`src/main.rs`

```rust
fn main() {
    let width1 = 30;
    let height1 = 50;

    println! (
        "矩形的面积为 {} 平方像素。",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
```

**清单 5-8**：计算由单独的宽度和高度变量指定的矩形面积


现在，请使用 `cargo run` 运行这个程序：

```console
$ cargo run
   Compiling struct_example v0.1.0 (/home/hector/rust-lang-zh_CN/projects/struct_example)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/struct_example`
矩形的面积为 1500 平方像素。
```


这段代码通过以两个边长调用 `area` 函数成功计算出了矩形面积，但我们还可以做更多事情使这段代码清晰易读。

这段代码的问题在 `area` 的签名中很明显：


```rust
fn area(width: u32, height: u32) -> u32 {
```


`area` 函数本应计算一个矩形的面积，但我们编写的函数有两个参数，并且咱们程序中的任何地方这两个参数是相关的都不明确。将宽度和高度分组在一起，将更易于阅读和管理。我们已在第 3 章 的 [元组类型](../programming_concepts/data_types.md#元组类型) 小节处，讨论过一种我们可能实现这点的方法：使用元组。


## 以元组重构

下面清单 5-9 展示了咱们程序使用元组的另一版本。


<a name="listing_5-9"></a>
文件名：`src/main.rs`

```rust
fn main() {
    let rect1 = (30, 50);

    println! (
        "矩形的面积为 {} 平方像素。",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```

**清单 5-9**：以元组指定矩形的宽和高

从某种意义上说，这个程序更好。元组让我们增加了一点结构，进而我们现在只传递一个参数。但从另一方面来说，这个版本更不清楚：元组没有命名其元素，因此我们不得不索引进到元组的各个部分，使我们的计算不那么直观。

混淆宽度和高度对于面积计算并不重要，但当我们打算在屏幕上绘制矩形时，那就很重要了！我们将不得不记住 `width` 是元组的索引 `0`，`height` 是元组的索引 `1`。当其他人要使用我们的代码时，就更难搞清楚并记住这点。因为我们没有在咱们的代码中传达咱们数据的含义，所以现在更容易引入错误。


## 以结构体重构

我们使用结构体通过标记数据来添加含义。我们可将我们正使用的元组，转换为一个有着整体名字以及各部分名字的结构体，如下清单 5-10 中所示。


<a name="listing_5-10"></a>
文件名：`src/main.rs`

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println! (
        "矩形的面积为 {} 平方像素。",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

**清单 5-10**：定义一个 `Rectangle` 结构体


这里，我们已定义了个结构体并将其命名为 `Rectangle`。在花括号内，我们将字段定义为 `width` 和 `height`，两个字段都有着类型 `u32`。然后，在 `main` 中，我们创建了个 `Rectangle` 的特定实例，有着 `30` 的宽度及 `50` 的高度。

我们的 `area` 函数现在以一个参数定义，我们已将其命名为 `rectangle`，其类型是对 `Rectangle` 结构体实例的不可变借用。正如第 4 章中所提到的，我们打算借用结构体而不是取得其所有权。这样，`main` 会保留其所有权并可继续使用 `rect1`，这就是我们在函数签名中及调用该函数处使用 `&` 的原因。

`area` 函数访问 `Rectangle` 实例的 `width` 和 `height` 字段（请注意，访问借用的结构体实例的字段不会迁移字段值，这就是为何咱们经常会看到结构体借用的原因）。我们的 `area` 函数签名现在准确地表达了我们的意思：使用 `Rectangle` 的 `width` 和 `height` 字段计算其面积。这传达了宽度和高度是相互关联的，并且其给到了这些值的描述性名字，而不是使用 `0` 和 `1` 的元组索引值。这属于清晰度方面的一种胜利。


## 以派生特质添加功能

我们在调试咱们程序期间，能够打印 `Rectangle` 的实例进而看到其所有字段的值，那将很有用。下面清单 5-11 尝试使用 `println!` 这个宏，正如我们在前几章中曾使用的那样。但这行不通。


<a name="listing_5-11"></a>
文件名：`src/main.rs`

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println! ("rect1 为：{rect1}");
}
```

*清单 5-11：尝试打印 `Rectangle` 实例*


当我们编译这段代码时，会得到带有下面这条核心消息的报错：


```console
error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
```


`println!` 宏可执行多种格式化，默认情况下，花括号会告诉 `println!` 使用称为 `Display` 的格式化：意在供最终用户直接使用的输出。到目前为止我们已看到的那些原始类型默认都实现了 `Display`，因为只有一种咱们希望把 `1` 或任何其他原始类型展示给用户的方式。但对于结构体，`println!` 应格式化输出的方式就不那么清晰了，因为有更多的显示可能性： 咱们是否想要逗号？咱们要不要打印花括号？所有字段都应显示吗？由于这种模糊性，Rust 不会尝试猜测我们想要什么，进而结构体没有提供某种与 `println!` 和 `{}` 占位符一起使用的 `Display` 实现。

若我们继续阅读报错，我们将发现下面这条帮助性说明：


```console
12 |     println! ("rect1 为：{rect1:?}");
   |                          ^^^^^^^ `Rectangle` cannot be formatted with the default formatter
   |
```

我们来试试吧！`println!` 宏调用现在将看起来像 `println! ("rect1 is {rect1:?}");`。将 `:?` 这个说明符放在花括号内，告诉 `println!` 我们打算使用一种名为 `Debug` 的输出格式。`Debug` 特质使我们可以一种对开发者有用的方式打印出咱们的结构体，以便我们在调试咱们代码时可以看到其值。

在这一修改下编译代码。糟糕！我们仍得到一个报错：


```console
error[E0277]: `Rectangle` doesn't implement `Debug`
```


不过编译器再次给了我们一条帮助性说明：


```console
12 |     println! ("rect1 为：{rect1:?}");
   |                          ^^^^^^^^^ `Rectangle` cannot be formatted using `{:?}` because it doesn't implement `Debug`
   |
```

Rust *确实* 包含了打印出调试信息的功能，但我们必须显式地选择使该功能针对我们的结构体可用。要实现这一目的，我们就要在结构体定义之前添加外层属性 `#[derive(Debug)]`，如下清单 5-12 中所示。


<a name="listing_5-12"></a>
文件名：`src/main.rs`

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println! ("rect1 为：{:?}", rect1);
}
```

**清单 5-12**：添加派生出 `Debug` 特质的属性，并使用调试格式打印 `Rectangle` 的实例


现在当我们运行程序时，我们将不会收到任何报错，并且我们将看到以下输出：


```console
$ cargo run
   Compiling struct_n_derived_trait v0.1.0 (/home/hector/rust-lang-zh_CN/projects/struct_n_derived_trait)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/struct_n_derived_trait`
rect1 为：Rectangle { width: 30, height: 50 }
```


不错！他虽不是最漂亮的输出，但他显示了这一实例的所有字段的值，这在调试期间绝对会有帮助。当我们有一些更大的结构体时，有着更容易阅读的输出会很有用；在这些情况下，我们可在 `println!` 的（格式化）字符串中使用 `{:#?}` 而不是 `{:?}`。在这个示例中，使用 `{:#?}` 样式将输出如下内容：


```console
$ cargo run
   Compiling struct_n_derived_trait v0.1.0 (/home/hector/rust-lang-zh_CN/projects/struct_n_derived_trait)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/struct_n_derived_trait`
rect1 为：Rectangle {
    width: 30,
    height: 50,
}
```


使用 `Debug` 格式打印出某个值的另一种方法是使用 [`dbg!` 宏](https://doc.rust-lang.org/std/macro.dbg.html)，他会：

- 取得表达式的所有权（与 `println!` 相反，他会取得引用）；
- 打印 `dbg!` 宏出现于咱们代码中处的文件及行号；
- 以及该表达式的结果值，并返回该值的所有权。


> 注意：调用 `dbg!` 宏会打印到标准错误控制台流 (`stderr`)，与打印到标准输出控制台流 (`stdout`) 的 `println!` 相反。我们将在第 12 章的 [重定向错误到标准错误](../io_project/std_err.md) 小节中，进一步讨论 `stderr` 与 `stdout`。



下面是个示例，其中我们对指派给 `width` 字段的值，以及 `rect1` 中整个结构体的值：


```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;

    let rect1 = Rectangle {
        width: dbg! (30 * scale),
        height: 50,
    };

    dbg! (&rect1);
}
```


我们之所以可将 `dbg!` 放在表达式 `30 * scale` 的周围，是因为 `dbg!` 会返回表达式值的所有权，因此 `width` 字段将得到同一个值，就像我们在那里没有 `dbg!` 调用一样。我们不希望 `dbg!` 取得 `rect1` 的所有权，因此我们在下一次调用中使用了对 `rect1` 的引用。下面是这个示例的输出看起来的样子：


```console
$ cargo run
   Compiling struct_n_derived_trait v0.1.0 (/home/hector/rust-lang-zh_CN/projects/struct_n_derived_trait)
[src/main.rs:11:16] 30 * scale = 60
[src/main.rs:15:5] &rect1 = Rectangle {
    width: 60,
    height: 50,
}
```


我们可以看到输出的第一部分来自 `src/main.rs` 的第 11 行，我们正于该处调试表达式 `30 * scale`，而其结果值为 `60`（针对整数实现的 `Debug` 格式化是只打印他们的值）。`src/main.rs` 第 15 行的 `dbg!` 调用输出 `&rect1` 的值，这是 `Rectangle` 结构体。这一输出使用 `Rectangle` 类型的美化 `Debug` 格式化。`dgb!` 宏在咱们试图弄清楚咱们的代码在做什么时会很有帮助！

除了 `Debug` 特质外，Rust 还提供了数个特质供我们与 `derive` 属性一起使用，可添加添加有用行为到我们的自定义类型。这些特质及其行为列出在 [附录 C](../appendix/derivable_traits.md) 中。我们将在第 10 章中介绍如何以自定义行为实现这些特质以及如何创建咱们自己的特质。除了 `derive` 之外，还有许多属性；有关更多信息，请参阅 [“Rust 参考” 的 "属性" 部分](https://doc.rust-lang.org/reference/attributes.html)。

我们的 `area` 函数非常具体（专门）：他只计算矩形的面积。将这一行为与咱们的 `Rectangle` 结构体更紧密地联系起来会很有帮助，因为他将不会与任何别的类型一起工作。我们来看看咱们可以怎样通过将 `area` 函数转换为定义在 `Rectangle` 类型上的 `area` 方法，继续重构这段代码。


（End）


