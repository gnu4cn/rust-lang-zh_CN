# 变量与可变性

如同 [“以变量存储值”](../Ch02_Programming_a_Guessing_Game.md#以变量存储值) 小节中所提到的，默认情况下，变量属于不可变的。这是 Rust 给到咱们的众多推动之一，允许咱们以利用 Rust 提供的安全性和简单并发性方式，编写咱们的代码。但是，咱们仍有着将咱们的变量构造为可变的选项。我们来探讨一下，Rust 怎样及为何鼓励咱们偏爱不可变性，以及为什么有时咱们可能需要放弃这点。

> **译注**：Erlang/OTP 语言中的变量，完全是不可变的。Rust 受到 Erlang/OTP 语言的影响。
>
> 参见：
>
> - [Erlang 变量不会变](https://erlang.xfoss.com/part-ii/Ch03-basic_concepts.html#erlang-%E5%8F%98%E9%87%8F%E4%B8%8D%E4%BC%9A%E5%8F%98)
>
> - [Rust (programming language)](https://en.wikipedia.org/wiki/Rust_(programming_language))

当变量不可变时，那么一旦某个值绑定到某个名字，咱们就无法更改那个值了。为演示这点，请通过使用 `cargo new variables`，在 `projects` 目录下生成一个名为 `variables` 的新项目。

然后在咱们的新 `variables` 目录下打开 `src/main.rs`，以以下代码替换其代码，这段代码还不会编译：


```rust
fn main() {
    let x = 5;
    println! ("x 的值为：{}", x);

    x = 6;
    println! ("x 的值为：{}", x);
}
```

保存并使用 `cargo run` 运行这个程序。咱们应会收到一条有关不变性错误的报错信息，如这个输出中所示：


```console
$ cargo run
   Compiling variables v0.1.0 (/home/hector/rust-lang-zh_CN/projects/variables)
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:5:5
  |
2 |     let x = 5;
  |         - first assignment to `x`
...
5 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
2 |     let mut x = 5;
  |         +++

For more information about this error, try `rustc --explain E0384`.
error: could not compile `variables` (bin "variables") due to 1 previous error
```

这个示例展示了编译器如何帮助咱们找到咱们程序中的错误。编译器报错可能令人沮丧，但其实他们只是意味着咱们的程序尚未安全地执行，咱们希望他要执行的操作；他们并 *不* 意味着咱们不是一名好的程序员！经验丰富的 Rustaceans 仍会遇到编译器报错。

咱们之所以收到了错误消息 `cannot assing twice to immutable variable 'x'`，是因为咱们试图将第二个值赋值给不可变的变量 `x`。

当我们试图更改某个被指定为不可变的值时，我们会收到编译时报错，compile-time errors, 这点很重要，因为这种特别情形可能导致程序 bug。当我们代码的一个部分运行于值永远将不改变这一假设之上，而咱们代码的另一部分却改变了该值，那么代码的第一部分就不会完成其被设计要完成的事情。这类 bug 的原因可能难于事后排查，尤其是当第二部分代码只是 *有时* 改变该值时。Rust 编译器保证，当咱们指出某个值将不改变时，他就真的不会改变，因此咱们不必自己跟踪他。咱们的代码因此就更容易推理。

但可变会非常有用，可以让代码更方便编写。虽然默认情况下变量是不可变的，但咱们可通过在变量名前面添加 `mut` 使其可变，就像在 [第 2 章](../Ch02_Programming_a_Guessing_Game.md#以变量存储值) 中所做的那样。添加 `mut` 还可通过表明代码的其他部分将改变这个变量的值，向未来的代码读者传达意图。

比如，咱们来将 `src/main.rs` 修改为以下：

文件名：`src/main.rs`

```rust
fn main() {
    let mut x = 5;
    println! ("x 的值为：{}", x);
    x = 6;
    println! ("x 的值为：{}", x);
}
```

在我们现在运行这个程序时，就会得到下面这样：

```rust
$ cargo run
   Compiling variables v0.1.0 (/home/hector/rust-lang-zh_CN/projects/variables)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/variables`
x 的值为：5
x 的值为：6
```

在使用 `mut` 后，我们就可以将绑定到 `x` 的值从 `5` 修改为 `6` 了。最终，决定是否要使用可变取决于咱们自己，取决于咱们认为在特定情形下哪种最清楚明了。


## 声明常量

与不可变的变量一样，*常量，constants* 属于一些绑定到名字且不允许更改的值，但常量和变量之间有些区别。

首先，不允许将 `mut` 与常量一起使用。常量不仅在默认情况下不可变 -- 他们始终是不可变的。咱们要使用 `const` 关键字而不是 `let` 关键字声明常量，并且值的类型 *必须* 被注解。我们将在下一小节 [“数据类型”](data_types.md) 中介绍类型和类型注解，所以现在不用担心细节。只要知道咱们必须始终注解这种类型。

常量可以在任何作用域，包括全局作用域中声明，这使得他们对于代码的多个部分都需要了解的值非常有用。

最后一个区别是，常量只能设置为常量表达式，而不能是某个只能在运行时计算出的值的结果，constants may be set only to a constant expression, not the result of a value that could only be computed at runtime。

> **译注**：这句话的意思是，常量的初始化赋值表达式，不能包含变量。如下面的代码：
>
> ```rust
> fn main() {
>     let a = 3;
>
>     const THREE_HOURS_IN_SECONDS: u32 = a * 60 * 60;
>
>     println! ("三个小时的秒数：{THREE_HOURS_IN_SECONDS}");
> }
> ```
>
> > 将报错：
>
>
> ```console
> $ cargo run
>    Compiling variables v0.1.0 (/home/hector/rust-lang-zh_CN/projects/variables)
> error[E0435]: attempt to use a non-constant value in a constant
>  --> src/main.rs:4:41
>   |
> 4 |     const THREE_HOURS_IN_SECONDS: u32 = a * 60 * 60;
>   |                                         ^ non-constant value
>   |
> help: consider using `let` instead of `const`
>   |
> 4 -     const THREE_HOURS_IN_SECONDS: u32 = a * 60 * 60;
> 4 +     let THREE_HOURS_IN_SECONDS: u32 = a * 60 * 60;
>   |
>
> warning: unused variable: `a`
>  --> src/main.rs:2:9
>   |
> 2 |     let a = 3;
>   |         ^ help: if this is intentional, prefix it with an underscore: `_a`
>   |
>   = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default
>
> For more information about this error, try `rustc --explain E0435`.
> warning: `variables` (bin "variables") generated 1 warning
> error: could not compile `variables` (bin "variables") due to 1 previous error; 1 warning emitted
> ```
>

下面是个常量声明的示例：

```rust
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

这个常量的名字是 `THREE_HOURS_IN_SECONDS`，其值被设为 60（一分钟的秒数）乘以 60（一小时的分钟数）再乘以 3（这个程序中要计算的小时数）的结果。Rust 的常量命名约定，是要使用全大写字母，单词之间带有下划线。编译器能够在编译时计算一个有限的运算集，这让我们可以选择以一种更容易理解和验证的方式写出这个值，而不是将这个常量设为值 10,800。有关在声明常量时可使用哪些运算的更多信息，请参阅 Rust 参考手册中，[有关常量求值的小节](https://doc.rust-lang.org/reference/const_eval.html)。

常量在程序运行的整个时间内，在他们被声明的范围内都有效。这一属性使得常量对于咱们程序域中，程序的多个部分可能需要了解的值非常有用，例如游戏中允许任何玩家获得的最大点数，或光速等。

给咱们程序整个范围中都要用到的一些硬编码值作为常量命名，有助于向代码的未来维护者传达该值的含义。此外，当硬编码值今后需要更新时，咱们需更修改的只有一处，这也很有帮助。


## 遮蔽

正如咱们曾在 [第 2 章](../Ch02_Programming_a_Guessing_Game.md#比较猜数与秘密数) 的猜数游戏教程中所看到的，咱们可以声明一个与某个先前变量同名的新变量。Rustaceans 会说，第一个变量是被第二个 *遮蔽了，shadowed*，这意味着当咱们使用这个变量的名字时，编译器将看到的是第二个变量。实际上，第二个变量掩盖了第一个，将这一变量名的任何用途都归为自身，直到他自己被遮蔽或作用域结束。通过使用相同变量的名字，并重复使用 `let` 关键字，咱们便可遮蔽某个变量，如下所示：


文件名：`src/main.rs`

```rust
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println! ("内部作用域中 x 的值为：{}", x);
    }

    println! ("x 的值为：{}", x);
}
```

> 注意：遮蔽特性的使用，不需要 `mut` 关键字。

这个程序首先会绑定 `x` 到一个 5 的值。然后，他通过重复 `let x =` 创建出一个新的变量 `x`，取原先值并加 `1`，因此 `x` 的值为 `6`。然后，在一个以大括号创建的内层作用域中，第三个 `let` 语句同样遮蔽了 `x` 而创建出一个新变量，将先前值乘以 `2` 使 `x` 的值为 `12`。当该作用域结束时，内部的遮蔽就结束了，`x` 回到为 `6`。当我们运行这个程序时，他将输出以下内容：


```console
$ cargo run
   Compiling shadowing v0.1.0 (/home/hector/rust-lang-zh_CN/projects/shadowing)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/shadowing`
内部作用域中 x 的值为：12
x 的值为：6
```
遮蔽不同于将变量构造为 `mut`，因为当我们不小心在没有使用 `let` 关键字下，尝试重新赋值到这个变量时，咱们将得到一个编译时报错，a compile-time error。而通过使用 `let` 关键字，我们可对某个值执行一些变换，而在这些变换完成后，让该变量不可变。

`mut` 与遮蔽的另一区别是，由于当我们再次使用 `let` 关键字时，我们实际上创建了个新的变量，因此我们可以改变这个值的类型，却重用了同一名字。例如，假设咱们的程序要求用户通过输入一些空格字符，给出他们想要一些文本间有多少个空格，然后我们打算将该输入存储为数字：


```rust
    let spaces = "    ";
    let spaces = spaces.len();
```

第一个 `spaces` 变量属于字符串类型，而第二个 `spaces` 变量则是数字类型。遮蔽因此而让我们不必想出不同的名称，比如 `spaces_str` 与 `spaces_num`；相反，我们可以重用这个更简单的 `spaces` 名字。然而，当我们尝试对此使用 `mut`，如下所示，咱们将得到一个编译时报错：


```rust
    let mut spaces = "    ";
    spaces = spaces.len();
```


该报错表明，我们未被允许改变变量的类型：


```console
$ cargo run
   Compiling shadowing v0.1.0 (/home/hector/rust-lang-zh_CN/projects/shadowing)
error[E0308]: mismatched types
 --> src/main.rs:3:14
  |
2 |     let mut spaces = "    ";
  |                      ------ expected due to this value
3 |     spaces = spaces.len();
  |              ^^^^^^^^^^^^ expected `&str`, found `usize`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `shadowing` (bin "shadowing") due to 1 previous error
```


现在我们已经探讨了变量的工作原理，我们来看看他们可以有的更多数据类型。


（End）


