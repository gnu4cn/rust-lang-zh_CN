# 函数

**Functions**


函数在 Rust 代码中非常普遍。咱们已经看到，这门语言中最重要的函数之一：`main` 函数，他是许多程序的入口点。咱们还见过 `fn` 关键字，他允许咱们声明出新的函数。

Rust 代码使用 *蛇形命名法，snake case*，作为函数和变量名的习惯样式，其中所有字母都是小写，并以下划线分隔单词。下面是个包含了函数定义示例的程序：


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

我们通过输入后跟函数名和一组括号的 `fn`，来定义出 Rust 中的函数。花括号告诉编译器，函数体于何处开始及结束。

我们可以通过输入后跟一组括号的其名字，来调用我们已定义的任何函数。因为 `another_function` 是在该程序中定义的，所以可以从 `main` 函数内部调用他。请注意，我们在源代码中的 `main` 函数 *之后*，定义了 `another_function`；我们也可以在之前定义他。 Rust 并不关心咱们在何处定义函数，而只关心他们，是在调用者可以看到的作用域中某个地方定义的。

我们来启动一个名为 `functions` 的新二进制项目，以进一步探索函数。请将这个 `another_function` 示例，放入 `src/main.rs` 并运行。咱们会看到以下输出：


```console
$ cargo run
   Compiling functions v0.1.0 (/home/peng/rust-lang/projects/functions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32s
     Running `target/debug/functions`
Hello, world!
另一函数。
```

这些行会按照他们在主函数中出现的顺序执行。首先打印 "Hello, world!" 这条消息，然后 `another_function` 被调用，并打印其消息。


## 参数

**Parameters**


我们可以将函数定义有着一些 *参数，parameters*，他们是函数签名的一部分的一些特殊变量。当某个函数有参数时，咱们就可以为这些参数，提供一些具体值。技术上讲，这些具体值被称为 *实参，arguments*，但在日常交谈中，人们倾向于可互换使用 *形参，parameter* 和 *实参，argument* 这两个词，来指代函数定义中的变量，或调用函数时传入的具体值。

在下面这个版本的 `another_function` 中，我们添加了一个参数：

文件名：`src/main.rs`

```rust
fn main() {
    another_function(-5);
}

fn another_function(x: i32) {
    println! ("x 的值为：{}", x);
}
```


请尝试运行这个程序；咱们应得到以下输出：


```console
$ cargo run
   Compiling functions v0.1.0 (C:\tools\msys64\home\Lenny.Peng\rust-lang-zh_CN\projects\functions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.88s
     Running `target\debug\functions.exe`
x 的值为：-5
```


`another_function` 的声明，有个名为 `x` 的参数。`x` 的类型，被指定为了 `i32`。当我们将 `5` 传入到 `another_function` 时，`println!` 这个宏，会将 `5` 在格式字符串中，原先那个包含 `x` 的一对大括号处。

在函数签名中，我们 *必须* 声明出每个参数的类型。这是 Rust 设计中的一个深思熟虑的决定：要求在函数定义中进行类型注解，意味着编译器几乎永远不需要咱们，在代码的其他地方，使用类型注解来确定咱们所指的是何种类型。如果编译器知道函数所期望的类型，他还能给出更有用的错误信息。

定义多个参数时，请用逗号分隔参数声明，如下所示：


文件名：`src/main.rs`

```rust
fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println! ("度量值为：{value}{unit_label}");
}
```


这个示例创建了一个名为 `print_labeled_measurement`，有着两个参数的函数。其中第一个参数名为 `value`，是个 `i32`。第二个参数名为 `unit_label`，类型为 `char`。该函数随后会打印同时包含 `value` 和 `unit_label` 的文本。

我们来尝试运行这段代码。请用前面的示例，替换 `functions` 项目的 `src/main.rs` 文件中当前的程序，然后使用 `cargo run` 运行他：


```console
$ cargo run
   Compiling functions v0.1.0 (C:\tools\msys64\home\Lenny.Peng\rust-lang-zh_CN\projects\functions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.58s
     Running `target\debug\functions.exe`
度量值为：5h
```


因为我们使用 `5` 作为 `value` 的值、`'h'` 作为 `unit_label` 的值，调用了该函数，所以程序输出会包含这些值。


## 语句及表达式

**Statements and Expressions**


函数体由一系列语句组成，可选择以表达式结束。到目前为止，我们所涉及的函数，还未包含结束表达式，an ending expression，但我们已经看到，作为语句一部分的表达式。因为 Rust 是门基于表达式的语言，所以理解这一点很重要。其他语言没有这同一区别，因此我们来看看，什么是语句和表达式，以及他们的区别如何影响函数的主体。


- **语句，statements**，是执行某些操作，但不返回值的指令；

- **表达式，expressions**，会求得一个结果值。咱们来一些示例。


实际上，我们已经使用过语句和表达式。使用 `let` 关键字创建变量并赋个值给他，就是语句。在下清单 3-1 中，`let y = 6;` 就是条语句。


文件名：`src/main.rs`

```rust
fn main() {
    let y = 6;
}
```

*清单 3-1：包含一条语句的一个 `main` 函数*


函数定义也是语句；上面的整个示例本身就是一条语句。

语句不返回值。因此，咱们不能将 `let` 语句，赋值给另一个变量，就像下面的代码试图做的那样；咱们会得到一个报错：


文件名：`src/main.rs`

```rust
fn main() {
    let x = (let y = 6);
}
```


当咱们运行这个程序时，咱们将得到的报错，会看起来像下面这样：


```console
$ cargo run
   Compiling functions v0.1.0 (C:\tools\msys64\home\Lenny.Peng\rust-lang-zh_CN\projects\functions)
error: expected expression, found `let` statement
 --> src\main.rs:2:14
  |
2 |     let x = (let y = 6);
  |              ^^^
  |
  = note: only supported directly in conditions of `if` and `while` expressions

warning: unnecessary parentheses around assigned value
 --> src\main.rs:2:13
  |
2 |     let x = (let y = 6);
  |             ^         ^
  |
  = note: `#[warn(unused_parens)]` on by default
help: remove these parentheses
  |
2 -     let x = (let y = 6);
2 +     let x = let y = 6;
  |

warning: `functions` (bin "functions") generated 1 warning
error: could not compile `functions` (bin "functions") due to previous error; 1 warning emitted
```


其中的 `let y = 6` 语句不会返回值，因此 `x` 没有任何绑定对象。这与其他语言（如 C 和 Ruby）的情况不同，在其他语言中，赋值会返回赋值的值。在这些语言中，咱们可以写下 `x = y = 6`，而 `x` 和 `y`，都会有着值 `6`；而在 Rust 中，情况并非如此。

表达式会计算为一个值，并构成咱们在 Rust 中，将编写的其余代码的大部分。请设想某个数学运算，例如 `5 + 6`，这是个会求值为 `11` 的表达式。表达式可以是语句的一部分：在清单 3-1 中，语句 `let y = 6;` 中的 `6`，是个求值为 `6` 的表达式。调用某个函数，便是个表达式。调用某个宏，是个表达式。用花括号创建的某个新作用域块，是个表达式，比如：

表达式可作为语句的一部分：在清单 3-1 中，语句 `let y = 6;` 里的 `6` 就是一个求值到 `6` 的表达式。对函数的调用，同样是个表达式。对宏的调用，也是个表达式。以花括号创建出的新代码块，还是个表达式，比如：


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

其中这个表达式：

 ```rust
{
    let x = 3;
    x + 1
}
```


是个在本示例种，会求值为 `4` 的一个代码块。该值会作为那个 `let` 语句的一部分，被绑定到 `y`。请注意，`x + 1` 这行的结尾，没有分号，这与咱们目前看到的大多数行不同。表达式不会包括结尾的分号。如果咱们在表达式的末尾加上分号，咱们便将其转换为了语句，而他就将不返回值。在接下来咱们探讨函数返回值和表达式时，请牢记这一点。


> 注：若在上面代码块中的 `x + 1` 后面加上分号，那么 `y` 的值将为 *单元值，unit* `()` 这一特殊值。进而在接下来的 `println!` 语句中导致出错。


```console
$ cargo run
   Compiling functions v0.1.0 (C:\tools\msys64\home\Lenny.Peng\rust-lang-zh_CN\projects\functions)
error[E0277]: `()` doesn't implement `std::fmt::Display`
 --> src\main.rs:7:22
  |
7 |     println! ("y 的值为：{y}");
  |                          ^^^ `()` cannot be formatted with the default formatter
  |
  = help: the trait `std::fmt::Display` is not implemented for `()`
  = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
  = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly
 builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0277`.
error: could not compile `functions` (bin "functions") due to previous error
```


## 有返回值的函数

**Functions with Return Values**


函数可以将一些值，返回给调用他们的代码。我们不会给返回值命名，但必须在一个箭头 (`->`) 后，声明他们的类型。在 Rust 中，函数的返回值，与函数体代码块中，最后一个表达式的值同义。咱们可以通过使用 `return` 关键字，并指定某个值来提前从函数中返回，但大多数函数，都是隐式地返回最后一个表达式。下面是个会返回值函数的示例：


> 注：关键字 `return` 的使用，标志着函数体的结束，`return` 语句之后的代码，将不再执行。


文件名：`src/main.rs`

```rust
fn five() -> u32 {
    5
}

fn main() {
    let x = five();

    println! ("x 的值为：{x}");
}
```


在其中的函数 `five` 中，没有函数调用、宏，甚至没有 `let` 语句，只有数字 `5` 本身。这在 Rust 中，是个完全有效的函数。请注意，该函数的返回类型，还被指定为 `-> i32`。请尝试运行这段代码；输出应是下面这样：


```console
$ cargo run
   Compiling functions v0.1.0 (C:\tools\msys64\home\Lenny.Peng\rust-lang-zh_CN\projects\functions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.79s
     Running `target\debug\functions.exe`
x 的值为：5
```


`five` 中的 `5`，便是该函数的返回值，这就是返回类型为 `i32` 的原因。我们来详细研究一下。有两个要点：首先，`let x = five();` 这行表明，我们正使用某个函数的返回值，来初始化一个变量。因为函数 `five` 的返回值是 `5`，所以这行与下面的行相同：


```rust
let x = 5;
```


其次，`five` 这个函数没有参数，并定义了返回值的类型，但函数体是个没有分号的孤零零的 `5`，因为他是个我们要返回其值的表达式。


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


运行这段代码将打印出 `x 的值为：6`。但如果我们在包含 `x + 1` 那行结束处，加个分号，而将其从表达式改为语句，咱们将得到一个报错：


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


编译这段代码，会产生一条报错，如下所示：


```console
$ cargo run
   Compiling functions v0.1.0 (C:\tools\msys64\home\Lenny.Peng\rust-lang-zh_CN\projects\functions)
error[E0308]: mismatched types
 --> src\main.rs:7:24
  |
7 | fn plus_one(x: i32) -> i32 {
  |    --------            ^^^ expected `i32`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
8 |     x + 1;
  |          - help: remove this semicolon to return this value

For more information about this error, try `rustc --explain E0308`.
error: could not compile `functions` (bin "functions") due to previous error
```


其中的主要错误信息，`mismatched types`，揭示了这段代码的核心问题。函数 `plus_one` 的定义，说他将返回一个 `i32`，但语句不会计算为某个值，而是由 `()` 表示的单元值，the unit type。因此，没有返回任何值，这与函数定义相矛盾，而导致一个报错。在此输出中，Rust 提供了一条，可能有助于纠正此问题的信息：他建议删除分号，这就将会修复这个错误。


（End）


