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


可将函数定义为有 *参数（parameters）*，所谓参数，就是作为函数签名一部分的一些特殊变量（which are special variables that are part of a function's signature）。在函数有着参数时，就可以提供到函数与这些参数对应的具体值。技术上讲，提供到函数的具体值叫做 *实参（arguments）*，不过在一般聊天中，人们会将 *形参（parameters）* 与 *实参（arguments）* 两个说法互换使用，既指函数定义中的变量，又表示调用函数时所传入的具体值。

在下面这个版本的 `another_function` 中，就要添加一个参数：

文件名：`src/main.rs`

```rust
fn main() {
    another_function(-5);
}

fn another_function(x: i32) {
    println! ("x 的值为：{}", x);
}
```

试着运行这个程序；就会得到以下输出：

```console
$ cargo run                                                        ✔
   Compiling functions v0.1.0 (/home/peng/rust-lang/projects/functions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/functions`
x 的值为：-5
```

`another_function` 的声明，有着一个名为 `x` 的参数。`x` 的类型被指定为 `i32`。在将 `-5` 传入到 `another_function` 时，那个 `println!` 的宏，就将 `-5` 放在那个格式化字符串中两个花括号所在的地方。

在函数签名中，*必须* 声明各个参数的类型。这是 Rust 设计中深思熟虑的决定：在函数定义中要求类型注解，就意味着编译器几近无需在代码中的什么地方使用那些函数的情况下，就能搞清楚是要何种类型（requiring type annotations in function definitions means that the compiler almost never needs you to use them elsewhere in the code to figure out what type you mean）。

在定义多个参数时，要用逗号（`,`）将那些参数声明分隔开，像下面这样：

文件名：`src/main.rs`

```rust
fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println! ("度量值为：{}{}", value, unit_label);
}
```

此示例创建了一个名为 `print_labeled_measurement`的、有两个参数的方法。第一个参数名为 `value`，且类型为 `i32`。第二个名为 `unit_label`，同时类型为 `char`。该函数随后会打印出同时包含 `value` 与 `unit_label` 的文本。

来尝试运行此代码。将`functions` 项目中的 `src/main.rs` 中的当前程序，用上面的示例进行替换，并使用 `cargo run` 运行当前程序：

```console
$ cargo run                                                        ✔
   Compiling functions v0.1.0 (/home/peng/rust-lang/projects/functions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.45s
     Running `target/debug/functions`
度量值为：5h
```

由于这里以 `5` 作为 `value` 的值，以 `h` 作为 `unit_label` 的值，调用了这个函数，因此该程序的输出，就包含了这些值。


## 语句及表达式

函数体是由一系列语句构成，这些语句可以是表达式结束的，也可以不是。到目前为止，所讲到的函数，都没有包含语句以表达式结束，不过有见到过表达式作为语句一部分的情况。由于 Rust 是基于表达式的语言，那么这一点就很重要，是要掌握的特征。其他语言并无这同样的特征，因此接下来就要看看语句和表达式究竟是何物，以及他们对函数体影响的不同。

- *语句（statements）* 是一些完成某些操作而不返回值的指令。

- *表达式（expressions）* 会求得一个结果值。来看看一些示例。

这里事实上已经用到了语句和表达式。创建一个变量，并以 `let` 关键字将一个值指派给他，就是一条语句。下面的清单 3-1 中，`let y = 6;` 就是一条语句。

文件名：`src/main.rs`

```rust
fn main() {
    let y = 6;
}
```

*清单 3-1：包含一条语句的一个 `main` 函数*

函数定义也是语句；上面的整个示例本身就是一条语句。

语句不会返回值。因此就无法将一条 `let` 语句，指派给另一变量了，就如同下面代码尝试完成的那样；这就会得到一条错误消息：

文件名：`src/main.rs`

```rust
fn main() {
    let x = (let y = 6);
}
```

当运行这个程序时，将收到的错误如下所示：

```console
$ cargo run                                                        ✔
   Compiling functions v0.1.0 (/home/peng/rust-lang/projects/functions)
error: expected expression, found statement (`let`)
 --> src/main.rs:2:14
  |
2 |     let x = (let y = 6);
  |              ^^^^^^^^^
  |
  = note: variable declaration using `let` is a statement

error[E0658]: `let` expressions in this position are unstable
 --> src/main.rs:2:14
  |
2 |     let x = (let y = 6);
  |              ^^^^^^^^^
  |
  = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information

warning: unnecessary parentheses around assigned value
 --> src/main.rs:2:13
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

For more information about this error, try `rustc --explain E0658`.
warning: `functions` (bin "functions") generated 1 warning
error: could not compile `functions` due to 2 previous errors; 1 warning emitted
```

其中的 `let y = 6` 语句不会返回值，因此这里就没有任何东西给 `x` 绑定。这不同于其他语言所发生的事情，譬如 C 和 Ruby 等，在其他语言中，赋值操作返回的是所赋的那个值。在那些语言中，就可以写下 `x = y = 6`，而让 `x` 与 `y` 同时有了值 `6`；但在 Rust 中却不是这样的。

表达式会求解为一个值，进而构成往后编写的 Rust 代码的绝大部分。设想一个数学运算，比如 `5 + 6`，这就是个将求值为值 `11` 的表达式。

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

其中的这个表达式：

 ```rust
{
    let x = 3;
    x + 1
}
```

在这个示例中，就是一个求值为 `4` 的表达式。其求得的值 `4` 会作为那条 `let` 语句的一部分，被绑定到 `y`。请注意那代码块最后的 `x + 1` 的代码行，并没有分号（`;`），而与到目前为止所见到的大多数代码行不同。表达式并不包含最后的分号。若将分号家到表达式末端，就会将其变成一条语句，进而就不再返回值了。在接下来对函数返回值与表达式的探索过程中，请牢记这一点。

> 注：若在上面代码块中的 `x + 1` 后面加上分号，那么 `y` 的值将为 `()` 这一特殊值（类似于 `null`）。进而在接下来的 `println!` 语句中导致出错。


## 有返回值的函数

函数可以将值返回给调用他们的代码。在函数有值要返回时，不会就这些返回值命名，但必须在箭头（`->`）后面，声明这些值的类型。在 Rust 中，函数的返回值，与函数体代码块的最后那个表达式的值，二者等价。通过使用 `return` 关键字并指定一个值，即可尽早地给函数返回值，不过大多数函数，都显式地返回最后的那个表达式。下面就是返回值的一个函数示例：

> 注：关键字 `return` 的使用，标志着函数体的结束，`return` 语句之后的代码，将不再执行。

文件名：`src/main.rs`

```rust
fn five() -> u32 {
    5
}

fn main() {
    let x = five();

    println! ("x 的值为：{}", x);
}
```

在那个 `five` 函数中，没有任何函数调用、宏、或者甚至 `let` 语句 -- 只是那个数字 `5` 自己。在 Rust 中这是个完全有效的函数。请注意该函数的返回值类型，也是以 `-> u32` 的形式指定了的。尝试运行此代码；输出应像下面这样：

```console
$ cargo run                                                        ✔
   Compiling functions v0.1.0 (/home/peng/rust-lang/projects/functions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.45s
     Running `target/debug/functions`
x 的值为：5
```

函数 `five` 中的 `5` 即是该函数的返回值，这就是为何返回值类型为 `u32` 的原因。下面来更深入地检视一下。其中有两个重点：首先，代码行`let x = five();` 表明这里使用了某个函数的返回值，来对一个变量进行初始化。由于函数 `five` 返回了一个 `5`，因此那行代码就跟下面的相同：

```rust
let x = 5;
```

其次，函数 `five` 没有参数，并定义了返回值类型，而其函数体只是个孤零零的、不带分号的 `5`，这是由于这个不带分号的 `5`，是个要将其值加以返回的表达式（注：若加上分号，那么就会变成一个语句，返回的将是特殊值 `()`，返回值类型将不再是 `u32`，从而导致编译时错误......）。

下面来看看另一个示例：

文件名：`src/main.rs`

```rust
fn main() {
    let x = plus_one(-1);

    println! ("x 的值为：{}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1;
}
```

对这段代码进行编译，会产生一条错误，如下所示：

```console
$ cargo run                                                        ✔
   Compiling functions v0.1.0 (/home/peng/rust-lang/projects/functions)
error[E0308]: mismatched types
 --> src/main.rs:7:24
  |
7 | fn plus_one(x: i32) -> i32 {
  |    --------            ^^^ expected `i32`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
8 |     x + 1;
  |          - help: consider removing this semicolon

For more information about this error, try `rustc --explain E0308`.
error: could not compile `functions` due to previous error
```

主要错误消息为，“mismatched types，”，该消息表明了此代码的核心问题。函数 `plus_one` 的定义是说他将返回一个 `i32`，然而函数体的语句并未求解到一个值来，求解到的是一个以 `()` 表示的单元类型（the unit type）。因此，就什么也没返回，这是与函数定义相矛盾的，进而导致了一个错误。在此输出中，Rust 提供了一条或许有助于纠正此问题的消息：他建议移除那个分号，那样就会修正该错误。


