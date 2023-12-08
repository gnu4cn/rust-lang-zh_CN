#  数据类型

**Data Types**


Rust 中的每个值，都属于特定的 *数据类型，data type*，这会告诉 Rust，正被指定的是何种数据类型，以便他知道，如何处理这个数据。我们将研究两个数据类型的子集：标量类型和复合类型，scalar and compound。

请记住，Rust 是门 *静态类型，statically typed* 的语言，这意味着在编译时，他必须知道所有变量的类型。编译器通常可以根据值，以及咱们使用值的方式，推断出我们打算使用的类型。在可能存在多种类型的情况下，例如在第 2 章 [“将猜数与秘密数字进行比较”](../Ch02_Programming_a_Guessing_Game.md#将猜数与秘数相比较) 小节中，咱们曾使用 `parse` 将一个 `String` 转换为数字类型时，我们就必须添加一个类型注解，就像下面这样：


```rust
let guess: u32 = "42".parse().expect("这不是个数字！");
```


如果我们不添加上面代码中所示的 `: u32` 类型注解，Rust 将显示如下错误，这意味着编译器需要我们提供更多信息，才能知道我们打算使用哪种类型：


```console
$ cargo build
   Compiling data_types v0.1.0 (C:\tools\msys64\home\Lenny.Peng\rust-lang-zh_CN\projects\data_types)
error[E0282]: type annotations needed
 --> src\main.rs:2:9
  |
2 |     let guess = "42".parse().expect("这不是个数字！");
  |         ^^^^^
  |
help: consider giving `guess` an explicit type
  |
2 |     let guess: /* Type */ = "42".parse().expect("这不是个数字！");
  |              ++++++++++++

For more information about this error, try `rustc --explain E0282`.
error: could not compile `data_types` (bin "data_types") due to previous error
```


咱们将看到，其他数据类型的不同类型注解。


## 标量类型

**Scalar Types**


所谓 *标量，scalar* 类型，表示单个值。Rust 有四种主要的标量类型：整数、浮点数、布尔值和字符。咱们可能在其他编程语言中，见过这些类型。我们来了解一下，他们在 Rust 中是如何工作的。


### 整形

**Integer Types**


所谓 *整数，integer*，是没有小数部分的数字。我们在第 2 章中，曾使用了一种整数类型，即 `u32` 类型。该类型声明，表示与之关联的值，应该是个无符号整数（有符号整数类型以 `i`，而不是 `u` 开头），占用 32 位空间。下表 3-1 列出了 Rust 中，内置的整数类型。我们可以使用任何的这些变种，来声明某个整数值的类型。


*表 3-1：Rust 中的整数类型*


| 长度 | 有符号 | 无符号 |
| :-: | :- | :- |
| 8 位 | `i8` | `u8` |
| 16 位 | `i16` | `u16` |
| 32 位 | `i32` | `u32` |
| 64 位 | `i64` | `u64` |
| 128 位 | `i128` | `u128` |
| 架构（arch） | `isize` | `usize` |


其中每个变种，都可以是有符号或无符号的，并有明确的大小。*有符号，signed* 和 *无符号，unsigned*，指的是数字是否有可能是负数 -- 换句话说，数字是否需要带有符号（有符号），或者数字是否只能是正数，而因此可以不带符号表示（无符号）。这就像在纸上书写数字：当符号很重要时，数字会用加号或减号表示；然而，当可以肯定数字是正数时，数字就会不带符号给出。有符号的数字使用 [二进制补码表示法](https://en.wikipedia.org/wiki/Two%27s_complement) 存储。

每个带符号的变种，可以存储 -(2<sup>n - 1</sup>) 到 2<sup>n - 1</sup> - 1（含）的数字，其中 *n* 是该变种使用的比特数。因此，`i8` 可以存储-(2<sup>7</sup>) 到 2<sup>7</sup> - 1 的数字，相当于 -128 到 127。无符号变种可以存储 0 至 2<sup>n</sup> - 1 的数字，因此 `u8` 可以存储 0 至 2<sup>8</sup> - 1 的数字，相当于 0 至 255。

此外，`isize` 和 `usize` 这两种类型，取决于咱们程序运行所在的计算机体系结构，这在该表中，用 “架构（arch）” 表示：如果咱们在 64 位架构上，则为 64 位；如果是在 32 位架构上，则为 32 位。

咱们可以下表 3-2 所示的任何形式，写出整数的字面值。需要注意的是，可以是多种数值类型的数字字面值，则允许使用类型后缀，如 `57u8`，来指定类型。数字字面值，也可以使用 `_` 作为视觉分隔符，使数字更容易读取，例如 `1_000`，其将与咱们指定的 `1000` 相同。


*表 3-2：Rust 中的数字字面值*

| 数字字面值 | 示例 |
| :- | :- |
| 十进制（Decimal） | `98_222` |
| 十六进制（Hex） | `0xff` |
| 八进制（Octal） | `0o77` |
| 二进制（Binary） | `0b1111_0000` |
| 字节（仅限 `u8`，Byte(`u8` only)） | `b'A'` |


那么，咱们怎么知道，要使用哪种整数类型呢？在不确定时，Rust 默认选项，通常是个很好的开始：整数类型默认为 `i32`。而使用 `isize` 或 `usize` 的主要情况，是在索引某种集合时。


> 关于 **整数溢出**
>
> 假设咱们有个可保存 0 到 255 之间值的 `u8` 类型变量。在咱们尝试将该变量，更改为超出该范围的某个值，比如 256 时，就会发生 *整数溢出，integer overflow*，这可能导致两种行为之一。在咱们以调试模式编译时，Rust 包含了对那些发生时，会引起咱们程序 *中止，panic* 的整数溢出的检查。当某个程序以一个报错而退出时，Rust 便对此使用 *中止，panicking* 一词；我们将在第 9 章，[“使用 `panic!` 的无法恢复错误](../error_handling/panic.md) 小节中，更深入地讨论中止运行。
>
> 而使用 `--release` 命令行开关，在发布模式下编译时，Rust 就不会检查会导致中止运行的整数溢出。相反，如果发生溢出，Rust 会执行 *二的补码换行，two's complement wrapping*。简而言之，大于该类型所能容纳最大值的值，会 “折回，wrap around” 到该类型所能容纳的最小值。在 `u8` 的情况下，值 256 会变成 0，值 257 会变成 1，依此类推。程序不会中止运行，但变量的值可能不是咱们期望的值。依赖整数溢出的 wrapping 行为，被视为错误。
>
> 要显式地处理溢出的可能性，咱们可以使用标准库为原始数值类型，提供的下面这些方法系列：
>
> - 使用 `wrapping_*` 系列方法，如 `wrapping_add`，在所有模式下换行，wrap；
>
> - 使用 `checked_*` 系列方法，在出现溢出时，则返回 `None` 值；
>
> - 使用 `overflowing_*` 系列方法，返回一个值，和一个表明是否存在溢出的布尔值；
>
> - 使用 `saturating_*` 系列方法，对数值的最小值或最大值进行饱和处理。


### 浮点类型

**Floating-Point Types**


Rust 同样有 *浮点数，floating-point numbers*（带有小数点的数）的两种原始类型。Rust 的浮点类型是 `f32` 和 `f64`，大小分别为 `32` 位和 `64` 位。默认类型是 `f64`，因为在现代 CPU 上，其速度与 `f32` 大致相同，但精度更高。所有浮点类型都是带符号的。

下面是个展示浮点数实际操作的示例：

文件名：`src/main.rs`

```rust
fn main() {
    let x = 2.0;    // f64
    let y: f32 = 3.0;   // f32
}
```

浮点数根据 [IEEE-754 标准](https://standards.ieee.org/ieee/754/6210/) 表示。`f32` 类型属于单精度浮点数，`f64` 类型有着双精度。


### 数值运算

**Numeric Operations**


Rust 支持所有数字类型的基本数学运算：加法、减法、乘法、除法和余数，reminder（求模）。整数的除法，会向零截断到最接近的整数。下面的代码展示了，如何在 let 语句中使用每种数字运算：


文件名：`src/main.rs`

```rust
fn main() {
    // 加法
    let sum = 5 + 10;

    // 减法
    let difference = 95.5 - 4.3;

    // 乘法
    let product = 4 * 30;

    // 除法
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // 结果为 0

    // 余数
    let reminder = 43 % 5;

    println! ("
        5 + 10 = {sum},
        95.5 - 4.3 = {difference}
        4 * 30 = {product}
        56.7 / 32.2 = {quotient}
        2 / 3 = {floored}
        43 % 5 = {reminder}");
}
```


这些语句中的每个表达式，都使用了个数学运算符，并求值为一个单一值，然后绑定到某个变量。[附录 B](../appendix/ops_and_symbols.md) 列出了 Rust 提供的所有运算符。



### 布尔值类型

**The Boolean Type**


与大多数其他编程语言中一样，Rust 中的布尔类型，有两个可能的值：`true` 和 `false`。布尔值的大小为一个字节。Rust 中的布尔类型使用 `bool` 指定。例如:


文件名：`src/main.rs`

```rust
fn main() {
    let t = true;

    let f: bool = false; // 带有显式类型注解
}
```


使用布尔值的主要方式，是经由一些条件，例如某个 `if` 表达式。我们将在 [“控制流”](control_flow.md) 小节，介绍 `if` 表达式在 Rust 中的工作原理。


### 字符类型

**The Character Type**


Rust 的 `char` 类型，是这门语言的最原始字母类型。下面是声明出一些 `char` 值的示例：


文件名：`src/main.rs`

```rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // 带有显式的类型注解
    let heart_eyed_cat = '😻';

    println! ("c 为 {c}, z 为 {z}, 爱心猫: {heart_eyed_cat}");
}
```

请注意，我们使用单引号指定 `char` 字面值，而字符串字面值则使用双引号。Rust 的 `char` 类型大小为 4 个字节，表示某个 Unicode 的标量值，a Unicode Scalar Value，这意味着他可以表示的字符，不仅仅是 ASCII。在 Rust 中，重音字母、中日韩文字、表情符号和零宽度的空格等，都是有效的 `char` 值。Unicode 标量值的范围为 `U+0000` 至 `U+D7FF` 和 `U+E000` 至 `U+10FFFF`（含）。不过，所谓 “字符”，在 Unicode 中并不是个真正的概念，因此咱们对 “字符” 的直觉，可能与 Rust 中的字符不一致。我们将在第 8 章，[“使用 `String` 存储 UTF-8 编码的文本”](../common_collections/strings.md) 小节中，详细讨论这个问题。


## 复合类型

**Compound Types**


*复合类型，compound types*，可以将多个值编为一种类型。Rust 有两种原始的复合类型：元组和数组。


###  元组类型

**The Tuple Type**


元组是将不同类型的多个值，组合在一起成为一个复合类型的通用方法。元组有固定的长度：一旦声明了出来，其大小就不能增大或缩小。

我们通过在括号内写入一个以逗号分隔的值列表，来创建一个元组。元组中的每个位置都有一个类型，元组中不同值的类型不必相同。在下面这个示例中，我们添加了可选的类型注解：


文件名：`src/main.rs`

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```


该变量 `tup` 会绑定到整个元组，因为元组被视为单一的复合元素。而要从某个元组中获取单个值，我们可以使用模式匹配，来解构某个元组值，就像这样：


文件名：`src/main.rs`

```rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println! ("y 的值为：{y}");
}
```


这个程序首先创建了一个元组，并将其绑定到变量 `tup`。然后，程序使用一个带有 `let` 的模式，来取 `tup`，并将其变成三个独立的变量 `x`、`y` 和 `z`。这叫做 *解构，destructuring*，因为他将单个元组，分解为了三个部分。最后，该程序打印出 `y` 的值，即 `6.4`。

我们还可以通过使用后跟要访问值索引的句点（`.`），直接访问某个元组元素。例如：


文件名：`src/main.rs`

```rust
fn main() {
    let x = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println! ("x.0：{five_hundred}, x.1：{six_point_four}, x.2：{one}");
}
```


此程序创建了一个元组 `x`，然后使用其各自索引，访问了元组中的每个元素。与大多数编程语言一样，元组中的第一个索引为 0。

没有任何值的元组，有一个特殊的名称，*单元值，unit*。这个值及其对应的类型，都被写成 `()`，并表示某个空值，或空返回类型。如果表达式不返回任何别的值，则会隐式返回这个单元值。


### 数组类型

**The Array Type**


另一种拥有多个值集合的方法，是使用 *数组，array*。与元组不同，数组的每个元素，都必须有着同样类型。与其他一些语言中的数组不同，Rust 中的数组有着固定长度。

我们将数组中的值，写作方括号内以逗号分隔的列表：


文件：`src/main.rs`

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

当咱们希望咱们的数据，分配在栈上而不是堆上时（我们将在 [第 4 章](ownership/about_ownership.md) 详细讨论栈和堆），或者当咱们希望确保始终有着固定数量元素时，数组就非常有用。不过，数组不如矢量类型灵活。所谓 *矢量，vector*，是标准库提供的一种类似的集合类型，其大小 *可以* 增大或缩小。如果咱们不确定是使用数组还是矢量，那么很可能应该使用矢量。[第 8 章](../common_collections/vectors.md) 会详细讨论矢量。

不过，当我们清楚元素数量今后不会需要改变时，数组会更有用。例如，如果咱们要在程序中使用月份的那些名字时，咱们就可能会使用数组而不是矢量，因为咱们知道他将始终包含 12 个元素：


```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
```


咱们可以使用带有每个元素的类型、分号以及数组中元素的数量的方括号，来写出某个数组的类型，如下所示：


```rust
let a: [i32, 5] = [-1, 0, 1, 2, 3];
```


这里，`i32` 是每个元素的类型。分号后的数字 `5` 表示该数组包含五个元素。

咱们还可以通过在方括号中，指定初始值、分号和数组长度，来初始化某个数组，使每个元素都包含相同的值，如下所示：


```rust
let a = [3; 5];
```


名为 `a` 这个数组，将包含 `5` 个元素，都将被初始设置为 `3`。这与写下 `let a = [3, 3, 3, 3, 3]` 相同；但写法更简洁。


### 访问数组元素

**Accessing Array Elements**


数组是可以在堆栈上分配的，已知、固定大小的单块内存。咱们可以使用索引，访问数组中的元素，就像这样：


文件名：`src/main.rs`

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let last = a[a.len()-1];

    println! ("数组的第一个元素：{first}，最后一个元素：{last}");
}
```


在此示例中，名为 `first` 的变量将获得值 `1`，因为这是数组中索引 `[0]` 处的值。名为 `last` 的变量将从数组中的索引 `[4]` 处获取到值 `5`。

> **译注**：`println! ("a[0] 为：{a[0]}");` 这种写法，会报出错误。

```console
$ cargo build
   Compiling tuple_demo v0.1.0 (C:\tools\msys64\home\Lenny.Peng\rust-lang-zh_CN\projects\tuple_demo)
error: invalid format string: expected `'}'`, found `'['`
 --> src\main.rs:4:24
  |
4 |     println! ("a[0]: {a[0]}");
  |                      - ^ expected `'}'` in format string
  |                      |
  |                      because of this opening brace
  |
  = note: if you intended to print `{`, you can escape it using `{{`

error: could not compile `tuple_demo` (bin "tuple_demo") due to previous error
```


### 无效的数组元素访问

**Invalid Array Element Access**


我们来看看，如果咱们尝试访问数组中，超过数组末尾的某个元素，会发生什么。假设咱们要运行下面这段，类似于第 2 章中的猜数游戏，从用户处获取数组索引的代码：


文件名：`src/main.rs`

```rust
use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println! ("请输入一个数组索引。");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("读取行失败，failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("输入的所以并非一个数字");

    let element = a[index];

    println! ("位于索引 {index} 出的元素值为：{element}");
}
```


这段代码会编译成功。如果咱们使用 `cargo run` 运行这段代码，并输入 `0`、`1`、`2`、`3` 或 `4`，该程序将打印出数组中，该索引处的相应值。如果咱们输入的数字超过了数组的末尾，例如 `10`，咱们将看到如下输出：


```console
$ cargo run
   Compiling tuple_demo v0.1.0 (C:\tools\msys64\home\Lenny.Peng\rust-lang-zh_CN\projects\tuple_demo)
    Finished dev [unoptimized + debuginfo] target(s) in 1.10s
     Running `target\debug\tuple_demo.exe`
请输入一个数组索引。
10
thread 'main' panicked at src\main.rs:19:19:
index out of bounds: the len is 5 but the index is 10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\tuple_demo.exe` (exit code: 101)
```


该程序在索引操作中，使用某个无效值时，出现了 *运行时* 错误。程序以一条错误信息退出了，并且没有执行最后的那条 `println!` 语句。当咱们尝试使用索引访问某个元素时，Rust 会检查咱们指定的索引，是否小于数组长度。如果索引大于或等于长度，Rust 就会终止运行。这种检查必须在运行时进行，尤其是在这种情况下，因为编译器不可能知道，某名用户在他们稍后运行代码时，会输入什么值。

这是 Rust 内存安全原则，实际应用的一个例子。在许多底层语言中，并无这种检查，而当咱们提供了某个不正确的索引时，无效内存就会被访问到。Rust 通过立即退出，而不是允许这种内存访问并继续，保护咱们免受此类错误的影响。第 9 章将讨论更多的 Rust 错误处理，以及如何编写既不会终止运行，也不允许无效内存访问的，可读安全代码。
