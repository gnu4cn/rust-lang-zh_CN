# 函数

函数在 Rust 代码中普遍存在。咱们已经看到这门语言中最重要函数之一：`main` 函数，他是许多程序的入口点。咱们还看到了 `fn` 这个关键字，他允许咱们声明新的函数。

Rust 代码使用 *蛇形命名法，snake case* 作为函数和变量名字的约定样式，其中所有字母都是小写，并以下划线分隔单词。下面是个包含示例函数定义的程序：


文件名：`src/main.rs`

```rust
fn main() {
    println! ("Hello, world!");

    another_function();
}

fn another_function() {
    println! ("另一函数。");
}
```

我们通过输入 `fn`，后跟函数名和一组括号定义 Rust 中的某个函数。花括号告诉编译器，函数体于何处开始及结束。

通过输入函数名字后跟一组括号，我们便可调用我们已定义的任何函数。因为 `another_function` 定义在这个程序中，其就可从 `main` 函数内部调用。请注意，我们是在源代码中 `main` 函数 *之后* 定义的 `another_function`；我们原本也可以在之前定义他。Rust 不关心咱们于何处定义咱们的函数，而只关心他们定义在调用者可以看到的作用域中的某处。

我们来启动一个名为 `functions` 的新二进制项目，进一步探讨函数。请将这个 `another_function` 示例放入 `src/main.rs` 并运行他。咱们应看到以下输出：


```console
$ cargo run
   Compiling functions v0.1.0 (/home/hector/rust-lang-zh_CN/projects/functions)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/functions`
Hello, world!
另一函数。
```

这些代码行会以他们在 `main` 函数中的出现顺序执行。首先打印 `"Hello, world!"` 这条消息，然后 `another_function` 被调用并打印其消息。


## 参数

我们可将函数定义为有着一些 *参数，parameters*，他们是属于函数签名一部分的一些特殊变量。当某个函数有着参数时，咱们就可以提供给他这些参数的具体值。技术上讲，具体值称为 *实参，arguments*，但在日常交谈中，针对函数定义中的变量，或咱们调用函数时传入的具体值，人们倾向于可互换使用 *形参，parameter* 和 *实参，argument* 这两个词。

在下面这个版本的 `another_function` 中，我们添加了个参数：

文件名：`src/main.rs`

```rust
fn main() {
    another_function(-5);
}

fn another_function(x: i32) {
    println! ("x 的值为：{}", x);
}
```


尝试运行这个程序；咱们应得到以下输出：


```console
$ cargo run
   Compiling functions v0.1.0 (/home/hector/rust-lang-zh_CN/projects/functions)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running `target/debug/functions`
x 的值为：-5
```


`another_function` 的声明有个名为 `x` 的参数。`x` 的类型被指定为 `i32`。当我们将 `-5` 传递进 `another_function` 时，`println!` 这个宏就会将 `5` 在格式字符串中，原先包含着 `x` 的一对大括号处。

在函数签名中，我们 *必须* 声明每个参数的类型。这是 Rust 设计中的一个深思熟虑的决定：在函数定义中要求类型注解，意味着编译器几乎永远不需要咱们在代码的其他地方，使用类型注解确定出咱们所指的是何种类型。当编译器清楚函数所期望的类型时，他还能够给到更有用的错误消息。

在定义多个参数时，要以逗号分隔这些参数声明，如下所示：


文件名：`src/main.rs`

```rust
fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println! ("度量值为：{value}{unit_label}");
}
```

这个示例创建了个名为 `print_labeled_measurement`，有着两个参数的函数。其中第一个参数名为 `value`，是个 `i32`。第二个参数名为 `unit_label`，为 `char` 类型。这个函数随后会打印同时包含 `value` 和 `unit_label` 的文本。

我们来尝试运行这段代码。请以前面的示例替换当前咱们 `functions` 项目的 `src/main.rs` 文件中的程序，并使用 `cargo run` 运行他：

```console
$ cargo run
   Compiling functions v0.1.0 (/home/hector/rust-lang-zh_CN/projects/functions)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/functions`
度量值为：5h
```

因为我们以 `5` 作为 `value` 的值、`'h'` 作为 `unit_label` 的值调用了该函数，所以这个程序输出会包含这两个值。


## 语句及表达式

函数体由一系列可选地以一个表达式结束的语句组成。到目前为止，我们介绍的函数都还未包含一个结束表达式，an ending expression，但我们已经看到，作为语句一部分的表达式。因为 Rust 是门基于表达式的语言，an expression-based language，所以这是个要掌握的重要区别。别的语言没有这相同区别，因此我们来看看，什么是语句和表达式，以及他们的区别如何影响函数的主体。

- 所谓 *语句，statements*，属于一些会执行某些操作，但不返回值的指令；
- 所谓 *表达式，expressions*，会计算为某个结果值。

咱们来看一些示例。

我们实际上已经用到了语句和表达式。以 `let` 关键字创建某个变量并赋值给他就是一条语句。在下清单 3-1 中，`let y = 6;` 就是条语句。


<a name="listing_3-1"></a>
文件名：`src/main.rs`

```rust
fn main() {
    let y = 6;
}
```

**清单 3-1**：包含一条语句的 `main` 函数声明


函数定义也属于语句；整个上面的示例本身就是条语句。

语句不会返回值。因此，咱们不能将某个 `let` 语句赋值给另一变量，就像以下代码试图做的那样；咱们将得到一个报错：


文件名：`src/main.rs`

```rust
fn main() {
    let x = (let y = 6);
}
```

当咱们运行这个程序时，咱们将得到的报错会看起来如下：

```console
$ cargo run
   Compiling functions v0.1.0 (/home/hector/rust-lang-zh_CN/projects/functions)
error: expected expression, found `let` statement
 --> src/main.rs:2:14
  |
2 |     let x = (let y = 6);
  |              ^^^
  |
  = note: only supported directly in conditions of `if` and `while` expressions

warning: unnecessary parentheses around assigned value
 --> src/main.rs:2:13
  |
2 |     let x = (let y = 6);
  |             ^         ^
  |
  = note: `#[warn(unused_parens)]` (part of `#[warn(unused)]`) on by default
help: remove these parentheses
  |
2 -     let x = (let y = 6);
2 +     let x = let y = 6 ;
  |

warning: `functions` (bin "functions") generated 1 warning
error: could not compile `functions` (bin "functions") due to 1 previous error; 1 warning emitted
```

其中 `let y = 6` 语句不会返回值，因此没有 `x` 要绑定任何东西。这与别的语言（如 C 和 Ruby）中发生的情况，在其他语言中，赋值会返回这一赋值的值。在这些语言中，咱们可以写下 `x = y = 6`，而让 `x` 和 `y` 都有着 `6` 这个值； Rust 中的情况并非如此。

表达式会计算为某个值，并构成咱们将以 Rust 编写的其余大部分代码。请设想某一数学运算，例如 `5 + 6`，其就是个会求值为 `11` 的表达式。表达式可以是语句的一部分：在清单 3-1 中，语句 `let y = 6;` 中的 `6` 就是个会求值为 `6` 的表达式；调用函数属于表达式；调用宏属于表达式；以花括号创建出的新作用域块属于表达式，例如：


文件名：`src/main.rs`

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println! ("y 的值为：{}", y);
}
```

其中这一表达式：

 ```rust
    {
        let x = 3;
        x + 1
    }
```


在这种情况下就是个会求值为 `4` 的代码块。该值会作为其中 `let` 语句的一部分被绑定到 `y`。请注意那个结束处没有分号的 `x + 1` 行，这与咱们到目前为止看到的大多数行都不同。表达式不会包含结束分号。当咱们添加一个分号到表达式末尾时，咱们便将其转换为了语句，而他将不返回值。在接下来咱们探讨函数的返回值与表达式时，要牢记这点。


> **译注**：若在上面代码块中的 `x + 1` 后面加上分号，那么 `y` 的值将为 *单元值，unit* `()` 这一特殊值。进而在接下来的 `println!` 语句中导致出错。
>
>
> ```console
> $ cargo run
>    Compiling expressions v0.1.0 (/home/hector/rust-lang-zh_CN/projects/expressions)
> error[E0277]: `()` doesn't implement `std::fmt::Display`
>  --> src/main.rs:7:27
>   |
> 4 |         x + 1;
>   |              - help: remove this semicolon
> ...
> 7 |     println! ("y 的值为：{}", y);
>   |                          --   ^ `()` cannot be formatted with the default formatter
>   |                          |
>   |                          required by this formatting parameter
>   |
>   = help: the trait `std::fmt::Display` is not implemented for `()`
>   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
>   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
>
> For more information about this error, try `rustc --explain E0277`.
> error: could not compile `expressions` (bin "expressions") due to 1 previous error
> ```
>
> 修改为下面这样即可输出变量 `y` 中包含的单元值：
>
> ```rust
> fn main() {
>     let y = {
>         let x = 3;
>         x + 1;
>     };
>
>     println! ("y 的值为：{:?}", y);
> }
> ```


## 有返回值的函数


函数可以返回值给调用他们的代码。我们不会命名返回值，但我们必须在一个箭头 (`->`) 后声明他们的类型。在 Rust 中，函数的返回值与函数体代码块中最后一个表达式的值同义。咱们可通过使用 `return` 关键字并指定某个值在函数中提前返回，但大多数函数都会隐式地返回最后一个表达式。下面是个返回了一个值的函数示例：


> **译注**：关键字 `return` 的使用，标志着函数体的结束。`return` 语句之后的代码，将不再执行。


文件名：`src/main.rs`

```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println! ("x 的值为：{x}");
}
```


在函数 `five` 中没有函数调用、宏，甚至没有 `let` 语句 -- 只有数字 `5` 本身。这在 Rust 中是个完全有效的函数。请注意，这个函数的返回类型也被指定了，为 `-> i32`。请尝试运行这段代码，输出应看起来像下面这样：


```console
$ cargo run
   Compiling functions v0.1.0 (/home/hector/rust-lang-zh_CN/projects/functions)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running `target/debug/functions`
x 的值为：5
```


`five` 中的 `5` 即为该函数的返回值，这就是返回类型为 `i32` 的原因。我们来更详细的研究一下这点。这里有两个重点：首先，`let x = five();` 这行表明，我们正使用某个函数的返回值来初始化一个变量。因为函数 `five` 会返回值一个 `5`，所以这行以下的行相同：

```rust
let x = 5;
```

其次，`five` 这个函数没有参数并定义了返回值的类型，但该函数的主体是个没有分号的孤零零的 `5`，因为他是个我们打算返回其值的表达式。


咱们来看看另一示例：


文件名：`src/main.rs`

```rust
fn main() {
    let x = plus_one(5);

    println! ("x 的值为：{x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```

运行这段代码将打印 `x 的值为：6`。但当我们在包含 `x + 1` 那行的结束处加个分号，将其从表达式改为语句时，会发生什么呢？


文件名：`src/main.rs`

```rust
fn main() {
    let x = plus_one(5);

    println! ("x 的值为：{x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1;
}
```


编译这段代码会产生一个报错，如下所示：


```console
$ cargo run
   Compiling functions v0.1.0 (/home/hector/rust-lang-zh_CN/projects/functions)
error[E0308]: mismatched types
 --> src/main.rs:7:24
  |
7 | fn plus_one(x: i32) -> i32 {
  |    --------            ^^^ expected `i32`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
8 |     x + 1;
  |          - help: remove this semicolon to return this value

For more information about this error, try `rustc --explain E0308`.
error: could not compile `functions` (bin "functions") due to 1 previous error
```


其中主要报错消息，`mismatched types`，揭示了这段代码的核心问题。函数 `plus_one` 的定义表明他将返回一个 `i32` 值，但语句不会计算为某个值，而是由 `()` 表示的单元类型，the unit type。因此，没有返回任何值，这与该函数的定义相矛盾而导致错误。在这一输出中，Rust 提供了一条消息，可能有助于纠正此问题：他建议移除分号，这将会修复错误。


（End）


