# 使用结构体的一个示例程序

**An Example Program Using Structs**


为了搞明白什么情况下我们可能会打算使用结构体，我们来编写个计算矩形面积的程序。我们将首先使用单个变量，然后重构该程序，直到咱们使用结构体为止。

我们来使用 Cargo，构造一个名为 `rectangles`，将以像素为单位，指定出矩形的宽和高，并计算该矩形面积的二进制项目。下面清单 5-8 给出了一个简短的程序，其在咱们项目的 `src/main.rs` 中，有着完成这一点的方法。


文件名：`src/main.rs`

```rust
fn main() {
    let width1 = 30;
    let height1 = 50;

    println! (
        "该矩形的面积为 {} 平方像素。",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
```

*清单 5-8：计算由单独的宽度和高度变量指定的矩形面积*


现在，请使用 `cargo run` 运行这个程序：

```console
$ cargo run
   Compiling rectangles v0.1.0 (/home/peng/rust-lang/projects/rectangles)
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/rectangles`
该矩形的面积为 1500 平方像素。
```


这段代码通过以各个维度调用那个 `area` 函数，成功计算出了该矩形的面积，但我们还可以做得更多，使这段代码更加清晰易读。

这段代码的问题，在 `area` 的签名中很明显：


```rust
fn area(width: u32, height: u32) -> u32 {
```


`area` 函数本应计算一个矩形的面积，但我们编写的函数却有两个参数，而且咱们的程序中，也没有明确说明这两个参数是相关的。如果将宽度和高度组合在一起，会更易于阅读和管理。在第 3 章 [元组类型](../programming_concepts/data_types.md#元组类型) 小节，我们已经讨论过一种，我们可以实现这一点的方法：使用元组。


## 使用元组重构

**Refactoring with Tuples**


下面清单 5-9 给出了使用元组的咱们程序另一版本。


文件名：`src/main.rs`

```rust
fn main() {
    let rect1 = (30, 50);

    println! (
        "该矩形的面积为 {} 平方像素。",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```

*清单 5-9：使用一个元组指定矩形的宽度和高度*


从某种意义上说，这个程序更好。元组让我们增加了一点结构，同时我们现在只传递了一个参数。但从另一个角度看，这个版本就不那么清晰了：元组没有为其元素命名，因此我们必须索引到元组的各个部分，这使得我们的计算不那么直观。

混淆宽度和高度对于面积计算并不重要，但如果我们要在屏幕上绘制矩形，就会有影响！我们必须记住，`width` 是该元组的索引 `0`，而 `height` 是该元组的索引 `1`。如果其他人使用我们的代码，就更难搞清楚并牢记这一点了。因为我们没有在咱们的代码中，传达咱们数据的含义，所以现在更容易引入错误。


## 使用结构体重构：添加更多意义

**Refactoring with Structs: Adding More Meaning**


我们要使用结构体，通过标记数据来增加意义。如下清单 5-10 所示，我们可以将正使用的元组，转换为一个整体和各部分都有名字的结构体。


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
        "该矩形的面积为 {} 平方像素。",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

*清单 5-10：定义出 `Rectangle` 结构体*


这里我们定义了一个结构体，并将其命名为 `Rectangle`。在花括号中，我们将字段定义为 `width` 和 `height`，两个字段的类型都是 `u32`。然后，在 `main` 中，我们创建了个有着宽度为 `30`，高度为 `50` 的 `Rectangle` 特定实例。

现在，我们的 `area` 函数定义了一个参数，我们将其命名为 `rectangle`，其类型是对 `Rectangle` 结构体实例的不可变借用。正如第 4 章所述，我们希望借用这个结构体，而不是要取得其所有权。这样，`main` 就可以保留其所有权，并继续使用 `rect1`，这也是我们在那个函数签名中，以及调用改函数时，使用 `&` 的原因。

`area` 函数会访问 `Rectangle` 实例的 `width` 和 `height` 字段（注意，访问某个借用的结构体实例的字段，不会迁移字段值，这就是为什么我们经常会看到结构体的借用）。现在，我们的 `area` 函数签名，就准确表达了我们的意思：使用 `Rectangle` 的 `width` 和 `height` 字段，计算其面积。这表达了宽度和高度是相互关联的，并且为这些值提供了描述性的名称，而不是使用 `0` 和 `1` 的元组索引值。这是清晰度方面的胜利。


## 使用派生特质添加有用功能

**Adding Useful Functionality with Derived Traits**


如果能在咱们调试咱们的程序期间，打印出 `Rectangle` 实例并查看其所有字段的值，那将会非常有用。下面清单 5-11 尝试使用 `println!` 宏，就像我们在前几章中使用的那样。但这行不通。


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

    println! ("rect1 为：{}", rect1);
}
```

*清单 5-11：尝试打印某个 `Rectangle` 实例*


当我们编译这段代码时，会得到一个带有下面这条核心信息的报错：


```console
error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
```


`println!` 宏可以执行多种格式化，而默认情况下，其中的花括号会告诉 `println!`，使用被称为 `Display` 的格式：供最终用户直接使用的输出。我们目前看到的原生类型，默认都实现了 `Display`，因为只有一种咱们想要的方式，将 `1` 或其他原生类型展示给用户。但对于结构体，`println!` 应如何格式输出的方式，就不那么清晰了，因为有更多的显示可能性： 咱们要不要逗号？要不要打印花括号？是否所有字段都应被显示出来？由于这种模糊性，Rust 不会试图猜测我们想要什么，同时结构体没有提供某种 `Display` 实现，来与 `println!` 和 `{}` 占位符一起使用。

如果我们继续阅读那些报错信息，就会发现这条有用的说明：


```console
   = help: the trait `std::fmt::Display` is not implemented for `Rectangle`
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
```


我们来试试把！那个 `println!` 宏调用现在将看起来像 `println! ("rect1 is {:?}"，rect1);`。在大括号中加上 `:?` 说明符，就告诉 `println!`，我们打算使用一种名为 `Debug` 的输出格式。`Debug` 特质使我们能以一种，对开发人员有用的方式打印出咱们的结构体，这样我们在调试代码时，就能看到他的值。

编译有着此项修改的代码。糟糕！我们仍然得到一个报错：


```console
error[E0277]: `Rectangle` doesn't implement `Debug`
```


但编译器再次给了我们一个有用的提示：


```console
   = help: the trait `Debug` is not implemented for `Rectangle`
   = note: add `#[derive(Debug)]` to `Rectangle` or manually `impl Debug for Rectangle`
```


Rust *确实* 包含了打印调试信息的功能，但我们必须显式地选择，将该功能用于我们的结构体。为此，我们要在结构体定义之前，添加外层属性 `#[derive(Debug)]`，如下清单 5-12 所示。


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

*清单 5-12：添加属性以派生 `Debug` 特质，并使用调试格式打印 `Rectangle` 实例*


现在，当我们运行该程序时，我们不会收到任何报错，并且我们将看到以下输出：


```console
$ cargo run
   Compiling rectangles v0.1.0 (/home/peng/rust-lang/projects/rectangles)
    Finished dev [unoptimized + debuginfo] target(s) in 0.20s
     Running `target/debug/rectangles`
rect1 为：Rectangle { width: 30, height: 50 }
```


不错！这不是最漂亮的输出，但他显示了该实例所有字段的值，这在调试期间绝对有帮助。当我们有更大的结构体时，让输出更容易阅读会很有用；在这些情况下，我们可以在 `println!` 字符串中，使用 `{:#?}` 代替 `{:?}`。在本例中，使用 `{:#?}` 样式，将输出如下内容：


```console
cargo run
   Compiling rectangles v0.1.0 (/home/peng/rust-lang/projects/rectangles)
    Finished dev [unoptimized + debuginfo] target(s) in 0.18s
     Running `target/debug/rectangles`
rect1 为：Rectangle {
    width: 30,
    height: 50,
}
```


使用 `Debug` 格式打印出某个值的另一种方法，是使用 [`dbg!` 宏](https://doc.rust-lang.org/std/macro.dbg.html)，他会取得某个表达式的所有权（与取得引用的 `println!` 相反），打印咱们代码中，这个 `dbg!` 宏调用出现的文件与行号，以及表达式的结果值，并返回该值的所有权。


> 注意：调用 `dbg!` 宏会打印到标准错误控制台流 (`stderr`)，这与打印到标准输出控制台流 (`stdout`) 的 `println!` 相反。关于 `stderr` 和 `stdout`，我们将在第 12 章 [将错误信息写入标准错误而不是标准输出](../io_project/std_err.md) 小节中详细讨论。


下面是个，其中我们对分配给 `width` 字段的值，以及 `rect1` 中的整个结构体的值感兴趣的实例：


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


我们可以将 `dbg!` 放在表达式 `30 * scale` 的周围，由于 `dbg!` 会返回该表达式值的所有权，因此那个 `width` 字段，将获得与该处没有调用 `dbg!` 时相同的值。我们不打算让 `dbg!` 取得 `rect1` 的所有权，因此在下一次调用中，使用了对 `rect1` 的引用。下面是这个示例的输出结果：


```console
$ cargo run
   Compiling rectangles v0.1.0 (C:\tools\msys64\home\Lenny.Peng\rust-lang-zh_CN\projects\rectangles)
warning: fields `width` and `height` are never read
 --> src\main.rs:3:5
  |
2 | struct Rectangle {
  |        --------- fields in this struct
3 |     width: u32,
  |     ^^^^^
4 |     height: u32,
  |     ^^^^^^
  |
  = note: `Rectangle` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
  = note: `#[warn(dead_code)]` on by default

warning: `rectangles` (bin "rectangles") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.68s
     Running `target\debug\rectangles.exe`
[src\main.rs:11] 30 * scale = 60
[src\main.rs:15] &rect1 = Rectangle {
    width: 60,
    height: 50,
}
```


我们可以看到，输出的第一部分来自 `src/main.rs` 第 11 行，其中我们调试了 `30 * scale` 这个表达式，而其结果值是 `60`（对整数实现的 `Debug` 格式，就是只打印其值）。`src/main.rs` 第 15 行的 `dbg!` 调用，会输出 `&rect1` 的值，这正是那个 `Rectangle` 结构体。该输出使用了 `Rectangle` 这个类型的良好 `Debug` 格式。当你试图弄清咱们的代码在做什么时，`dbg!` 这个宏确实很有帮助！

除了这个 `Debug` 特质外，Rust 还为我们提供了许多与这个 `derive` 属性配合使用的特质，这些特质可以为我们的自定义类型，添加有用的行为。[附录 C](../appendix/derivable_traits.md) 列出了这些特质及其行为。我们将在第 10 章，介绍如何使用自定义行为实现这些特质，以及如何创建咱们自己的特质。除了 `derive` 之外，还有许多其他属性；有关详细信息，请参阅 [《Rust 参考》中的 "属性 "部分](https://doc.rust-lang.org/reference/attributes.html)。

我们的 `area` 函数，是非常专门的：他只会计算矩形的面积。如果能将这一行为，与咱们的 `Rectangle` 结构体更紧密地联系在一起，将会很有帮助，因为他无法与任何其他类型一起工作。我们来看看，咱们可以怎样通过将这个 `area` 函数，转化为定义在 `Rectangle` 类型上的 `area` 方法，来继续重构这段代码。


（End）


