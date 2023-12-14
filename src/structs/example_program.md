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


如果能在调试程序期间打印出 `Rectangle` 的实例，并查看到所有字段的值，那就会派上用场。下面的清单 5-11 尝试了使用之前各章已经用到 [`println!` 宏](https://doc.rust-lang.org/std/macro.println.html)。不过这段代码不会工作。

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

*清单 5-11：尝试打印出一个 `Rectangle` 实例*

在编译这段代码时，会得到有着以下核心消息的错误：

```console
error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
```

`println!` 宏可完成许多种类的格式化，而默认情况下，那对花括号告诉 `println!` 的是，要使用名为 `Display` 的格式化操作：即用于最终用户直接消费的输出（the `println!` macro can do many kinds of formatting, and by default, the curly brackets tell `println!` to use formatting known as `Display`: output intended for direct end user consumption）。因为在要将一个 `1` 或其他任何原生类型，展示给用户时，都只有唯一的一种方式，因此，对于至今为止已见到过的那些原生类型来说，默认都是实现了 `Display` 的。而对于结构体来说，由于存在更多的显示可能：是要逗号还是不要？要打印出那对花括号吗？所有字段都要展示出来吗？因此 `println!` 对输出进行格式化的方式，就不那么清楚了。正是因为这种模棱两可，Rust 于是就不尝试猜测代码编写者想要的样子，而结构体也就没有一个事先提供的、与 `println!` 和 `{}` 占位符一起使用的 `Display` 实现了。

在继续阅读该错误消息时，就会发现下面这个有用注解：

```console
   = help: the trait `std::fmt::Display` is not implemented for `Rectangle`
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
```

来试一下！这个 `println!` 的宏调用，现在看起来是这样 `println! ("rect1 为 {:?}", rect1);`。将说明符 `:?` 放在那对花括号里头，就会告诉 `println!`，这里是要使用一个名为 `Debug` 的输出。而 `Debug` 特质就令到这里可将那个结构体，以对开发者有用的方式打印出来，如此就可以在对代码进行调试时，看到那个结构体的值了。

在此改变下，对该代码进行编译。见鬼！还是得到个错误：

```console
error[E0277]: `Rectangle` doesn't implement `Debug`
```

不过编译器再度给到一个帮助性注释：

```console
   = help: the trait `Debug` is not implemented for `Rectangle`
   = note: add `#[derive(Debug)]` to `Rectangle` or manually `impl Debug for Rectangle`
```

Rust *确实* 带有打印输出调试信息的功能，不过这里必须显式地选择上那功能，从而使得那功能对这个结构体可用。而要实现这个目的，就要在紧接着结构体定义之前，加上外层属性 `#[derive(Debug)]`（the outer attribute `#[derive(Debug)`），如下面的清单 5-12 所示。

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

*清单 5-12：加入派生 `Debug` 特质的属性，进而运用调试格式化将那个 `Rectangle` 实例打印出来*

此时在运行这个程序时，就不会收到任何错误了，且会看到下面的输出：

```console
$ cargo run
   Compiling rectangles v0.1.0 (/home/peng/rust-lang/projects/rectangles)
    Finished dev [unoptimized + debuginfo] target(s) in 0.20s
     Running `target/debug/rectangles`
rect1 为：Rectangle { width: 30, height: 50 }
```

很棒！这虽不是最漂亮的输出，但他给出了该实例全部字段的值，这无疑在调试期间会有帮助。在有着较大的结构体时，让输出更容易阅读一点就会有用；对于那些更大结构体的情形，就可在 `println!` 中使用 `{:#?}` 而非 `{:?}`。而在这个示例中，使用 `{:#?}` 样式将输出：

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

使用 `Debug` 格式化将某个值打印出来的另一种方式，就是使用 [`dbg!` 宏](https://doc.rust-lang.org/std/macro.dbg.html)，这个 `dbg!` 宏会占据某个表达式的所有权，而将那个 `dbg!` 宏调用出现在代码中所在的文件与行号，与那个表达式的结果值一并打印出来，同时返回结果值的所有权（another way to print out a value using the [`dbg!` macro](https://doc.rust-lang.org/std/macro.dbg.html), which takes ownership of an expression, prints the file and line number of where that `dbg!` macro call occurs in your code along with the resulting value of that expression, and returns ownership of the value）。

> 注意：对 `dbg!` 宏的调用，会打印到标准错误控制台流（the standard error console stream, `stderr`），这与 `println!` 宏打印到标准输出控制台流（the standard output console stream, `stdout`）相反。在第 12 章中的 [将错误消息写到标准错误而非标准输出](Ch12_An_I_O_Project_Building_a_Command_Line_Program.md#把错误消息写到标准错误而非标准输出) 小节，将讲到更多有关 `stderr` 与 `stdout` 的内容。

以下是个其中对赋值给 `width` 字段，以及在变量 `rect1` 中的整个结构体的值感兴趣的示例：

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

这里可将 `dbg!` 放在表达式 `30 * scale` 附近，同时由于 `dbg!` 返回了该表达式值的所有权，因此 `width` 字段将获取到与不在此处调用 `dbg!` 同样的值。由于这里不想要 `dbg!` 取得 `rect1` 的所有权，因此在下一个对 `dbg!` 的调用中，使用到到 `rect1` 的引用。下面就是这个示例输出的样子：

```console
cargo run
   Compiling rectangles v0.1.0 (/home/peng/rust-lang/projects/rectangles)
    Finished dev [unoptimized + debuginfo] target(s) in 0.22s
     Running `target/debug/rectangles`
[src/main.rs:11] 30 * scale = 60
[src/main.rs:15] &rect1 = Rectangle {
    width: 60,
    height: 50,
}
```

这里就可以看到，输出的第一部分来自 `src/main.rs` 文件的第 10 行，正是对表达式 `30 * scale` 进行调式的地方，而该表达式的结果值即为 `60`（在整数原生值上实现的 `Debug` 格式化只打印他们的值）。在 `src/main.rs` 第 14 行上的 `dbg!` 调用，输出了 `rect1`，即那个 `Rectangle` 结构体的值。这个输出使用了 `Rectangle` 类型的良好 `Debug` 格式化。在尝试搞清楚代码在做什么时，这个 `dbg!` 宏真的会相当有用！

除 `Debug` 特质外，Rust 业已提供了数个与 `derive` 属性一起使用的其他特质，这些特质把有用的行为表现，添加到那些定制类型。Rust 提供的那些特质及其行为，在 [附录 C](Ch21_Appendix.md#附录-c派生特质) 小节中有列出。在第 10 章中，就会涉及到怎样去实现这些有着定制行为的特质，以及怎样创建自己的特质。除了 `derive` 之外，同样还有许多别的属性；有关属性的更多信息，请参阅 [Rust 参考手册的 “属性” 小节](https://doc.rust-lang.org/reference/attributes.html)。

这里的 `area` 函数，是相当专用的：他只会计算矩形的面积。由于 `area` 方法不会在其他任何类型上工作，因此将此行为与这里的 `Rectangle` 结构体更紧密的联系起来，就会变得有帮助。接下来就要看看，怎样通过将这个 `area` 函数，转变成一个定义在这里的 `Rectangle` 类型上的方法，而继续重构这段代码。



