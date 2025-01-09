# 变量与可变性

**Variables and Mutability**


如同 [“使用变量存储值”](../Ch02_Programming_a_Guessing_Game.md#使用变量存储值) 小节所提到的，默认情况下，变量是不可变的。这是 Rust 给到咱们，以充分利用 Rust 所提供的安全性和易并发性的方式，编写咱们代码的措施之一。不过，咱们仍然可以选择让变量可变。我们来探讨一下，Rust 如何以及为什么鼓励咱们偏爱不可变性，以及为什么有时咱们可能会想要选择放弃。

当某个变量是不可变的时，那么一旦某个值绑定到某个名字，咱们就无法更改那个值了。为了说明这一点，请使用 `cargo new variables`，在 `projects` 目录中生成一个名为 `variables` 的新项目。

然后，在咱们的新 `variables` 目录下，打开 `src/main.rs`，用下面的代码替换其代码，这段代码现在还不会编译：


```rust
fn main() {
    let x = 5;
    println! ("x 的值为：{}", x);

    x = 6;
    println! ("x 的值为：{}", x);
}
```

保存并使用 `cargo run` 运行这个程序。咱们应该会收到一条，有关不变性错误的错误信息，如下面这个输出所示：


```console
$ cargo run                                                    ✔
   Compiling variables v0.1.0 (/home/peng/rust-lang/projects/variables)
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:5:5
  |
2 |     let x = 5;
  |         -
  |         |
  |         first assignment to `x`
  |         help: consider making this binding mutable: `mut x`
...
5 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable

For more information about this error, try `rustc --explain E0384`.
error: could not compile `variables` due to previous error
```

这个示例展示了，编译器如何帮助咱们发现咱们程序中的错误。编译器报错可能会令人沮丧，但实际上他们只意味着咱们的程序，还不能安全地完成咱们想要他做的事情；他们并 *不* 意味着咱们不是一名优秀的程序员！经验丰富的 Rustaceans 仍然会遇到编译器报错。

咱们之所以收到了错误消息 `cannot assing twice to immutable variable 'x'`，是因为咱们试图将第二个值，赋值给那个不可变的变量 `x`。

重要的是，当我们试图更改某个被指定为不可变的值时，会出现编译时错误，因为这种情况可能会导致错误。如果我们代码的一部分假设某个值永远不会改变，而代码的另一部分却改变了该值，那么代码的前一部分就有可能无法完成其设计目标。这种错误的原因可能很难事后追踪，尤其是当第二部分代码只是 *偶尔* 改变值时。Rust 编译器保证，当咱们声明了某个值不会改变时，他确实不会改变，因此咱们不必自己跟踪它。这样，咱们的代码就更容易推理了。

但可变会非常有用，可以让代码编写更方便。虽然变量在默认情况下是不可变的，但也可以像 [第 2 章](../Ch02_Programming_a_Guessing_Game.md#使用变量存储值) 中咱们曾做的那样，在变量名前添加 `mut`，使其成为可变变量。添加 `mut` 还可以向代码的未来读者，传达某种意图，即代码的其他部分，将改变该变量的值。

比如，咱们来将 `src/main.rs` 修改为下面这样：

文件名：`src/main.rs`

```rust
fn main() {
    let mut x = 5;
    println! ("x 的值为：{}", x);

    x = 6;
    println! ("x 的值为：{}", x);
}
```


当我们现在运行这个程序时，会得到这样的结果：


```rust
$ cargo run
   Compiling variables v0.1.0 (C:\tools\msys64\home\Lenny.Peng\rust-lang-zh_CN\projects\variables)
    Finished dev [unoptimized + debuginfo] target(s) in 1.26s
     Running `target\debug\variables.exe`
x 的值为：5
x 的值为：6
```

在使用了 `mut` 时，我们就可以将绑定到 `x` 的值，从 `5` 改为 `6`。最终，是否使用可变，取决于咱们自己，取决于咱们认为在特定情况下，什么是最清晰的。


## 常量

**Constants**


与不可变的变量一样，*常量，constants* 也是绑定到名字，且不允许更改的值，但常量和变量之间有些区别。

首先，不允许将 `mut` 与常量一起使用。常量不仅默认是不可变的，而且始终不可变。咱们要使用 `const` 关键字，而不是 `let` 关键字，声明常量，而且 *必须* 注解值的类型。我们将在下一小节 [“数据类型”](data_types.md) 中，介绍类型和类型注解，所以现在不用担心这些细节。只需知道咱们必须始终注解类型即可。

常量可以在任何作用域（包括全局作用域）中声明，因此对于代码的多个部分都需要知道的那些值来说，常量非常有用。

常量与不可变变量的最后一个区别，是常量只能设置为一个常量表达式，而不是只能在运行时计算出的值的结果，constants may be set only to a constant expression, not the result of a value that could only be computed at runtime。

> **译注**：这句话的意思，常量的初始化赋值表达式不能包含变量。如下面的代码：

```rust
fn main() {
    let a = 3;

    const THREE_HOURS_IN_SECONDS: u32 = a * 60 * 60;

    println! ("三个小时的秒数：{THREE_HOURS_IN_SECONDS}");
}
```

> 将报错：


```console
$ cargo run
   Compiling variables v0.1.0 (C:\tools\msys64\home\Lenny.Peng\rust-lang-zh_CN\projects\variables)
error[E0435]: attempt to use a non-constant value in a constant
 --> src\main.rs:4:41
  |
4 |     const THREE_HOURS_IN_SECONDS: u32 = a * 60 * 60;
  |     ----------------------------        ^ non-constant value
  |     |
  |     help: consider using `let` instead of `const`: `let THREE_HOURS_IN_SECONDS`

For more information about this error, try `rustc --explain E0435`.
error: could not compile `variables` (bin "variables") due to previous error
```


下面是个常量声明的示例：

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

这个常量的名字是 `THREE_HOURS_IN_SECONDS`，其值被设置为 60（一分钟的秒数）乘以 60（一小时的分钟数）再乘以 3（本程序中要计算的小时数）的结果。Rust 的常量命名约定，是使用全大写字母，单词之间使用下划线。编译器可以在编译时，计算一个有限的运算集，这让我们可以选择，以一种更容易理解和验证的方式写出这个值，而不是将这个常量设置为值 10,800。有关在声明常量时，可以使用哪些运算的更多信息，请参阅 [Rust 参考手册中，关于常量求值的小节](https://doc.rust-lang.org/reference/const_eval.html)。

常量在程序运行的整个过程中，在其声明的范围内都有效。这一属性使得常量对于程序域中，多个部分可能需要了解的值，例如游戏中允许任何玩家获得的最大点数或光速，会非常有用。

将整个程序中要用到的一些值，命名为常量，有助于向未来的代码维护者，传达该值的含义。此外，如果将来需要更新硬编码值，只需更改咱们代码中的一处即可。


## 遮蔽

**Shadowing**


如同咱们曾在 [第 2 章](../Ch02_Programming_a_Guessing_Game.md#将猜数与秘数相比较) 的猜数游戏教程中，所看到的，咱们可以声明出一个，与先前的某个变量同名的新变量。Rustaceans 会说，第一个变量是被第二个 *遮蔽了，shadowd*，这意味着当你使用这个变量的名字时，编译器将看到的，是第二个变量。实际上，第二个变量覆盖了第一个，将变量名的任何使用，都带到自己身上，直到他自己被遮蔽，或作用域结束。像下面这样，通过使用相同的变量名，和重复使用 `let` 关键字，咱们便可以遮蔽某个变量：


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


```console
内部作用域中 x 的值为：12
x 的值为：6
```

> 注意：遮蔽特性的使用，不需要 `mut` 关键字。

这个程序首先将 `x`，绑定到 5 的值。然后通过重复 `let x =`，创建出一个新变量 `x`，将原来的值加上 `1`，这样 `x` 的值就是 `6`。然后，在用大括号创建的一个内层作用域中，第三个 `let` 语句也对 `x` 进行了遮蔽处理，并创建了一个新变量，将先前的值乘以 `2`，使 `x` 的值为 `12`。当这个作用域结束时，内部的遮蔽结束，`x` 返回到 `6`。当我们运行这个程序时，他将输出如下内容：


```console
$ cargo run
   Compiling variables v0.1.0 (C:\tools\msys64\home\Lenny.Peng\rust-lang-zh_CN\projects\variables)
    Finished dev [unoptimized + debuginfo] target(s) in 1.03s
     Running `target\debug\variables.exe`
内部作用域中 x 的值为：12
x 的值为：6
```
遮蔽操作不同于将变量构造为 `mut`，因为如果我们不小心在没有使用 `let` 关键字的情况下，将变量重新赋值，咱们就会得到一个编译时报错。而通过使用 `let` 关键字，我们可以对某个值执行一些变换，但在这些变换完成后，该变量将不可变。

`mut` 与遮蔽的另一个区别是，当我们再次使用 `let` 关键字时，我们实际上是创建了一个新变量，因此我们可以改变该值的类型，而可以重复使用这个同样的名字。例如，假设咱们的程序要求用户，通过输入一些空格字符，来给出他们想要的某些文本之间多少个空格，然后我们打算将这个输入，存储为一个数字：


```rust
let spaces = "    ";
let spaces = spaces.len();
```


第一个 `spaces` 变量，属于字符串类型，而第二个 `spaces` 变量，则是数字类型。遮蔽就这样，让我们不必使用不同的名称，如 `spaces_str` 与 `spaces_num`；相反，我们可以重复使用这个更简单的 `spaces` 名称。然而，如果我们尝试使用 `mut` 来实现这一点，如下所示，咱们就会收到一个编译时报错：


```rust
let mut spaces = "    ";
spaces = spaces.len();
```


该报错表明，我们不允许改变变量的类型：


```console
$ cargo run
   Compiling variables v0.1.0 (C:\tools\msys64\home\Lenny.Peng\rust-lang-zh_CN\projects\variables)
error[E0308]: mismatched types
 --> src\main.rs:3:14
  |
2 |     let mut spaces = "    ";
  |                      ------ expected due to this value
3 |     spaces = spaces.len();
  |              ^^^^^^^^^^^^ expected `&str`, found `usize`
  |
help: try removing the method call
  |
3 -     spaces = spaces.len();
3 +     spaces = spaces;
  |

For more information about this error, try `rustc --explain E0308`.
error: could not compile `variables` (bin "variables") due to previous error
```


现在我们已经探讨了变量的工作原理，我们来看看，他们可以拥有的更多数据类型。


（End）


