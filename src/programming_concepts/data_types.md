#  数据类型

Rust 中的每个值都属于某种特定的 *数据类型，data type*，这会告诉 Rust 正被指定的是何种类别的数据，以便他知道如何处理该数据。我们将研究两个数据类型子集：标量类型及复合类型。

请记住，Rust 属于一门 *静态类型，statically typed* 的语言，这意味着他必须在编译时就知道，所有变量的类型。编译器通常可以根据值及咱们使用值的方式，推断出我们打算使用的类型。在可能有多种类型，比如在第 2 章 [“比较猜数与秘密数”](../Ch02_Programming_a_Guessing_Game.md#比较猜数与秘密数) 小节中，咱们曾使用 `parse` 将 `String` 转换为数字类型时，我们必须添加类型注解，就像下面这样：


```rust
let guess: u32 = "42".parse().expect("这不是个数字！");
```

当我们不添加前面代码中所示的 `: u32` 类型注解时，Rust 将显示以下错误，这意味着编译器需要我们提供更多信息，才能知道我们打算使用何种类型：


```console
$ cargo build
   Compiling type_annotation v0.1.0 (/home/hector/rust-lang-zh_CN/projects/type_annotation)
error[E0284]: type annotations needed
 --> src/main.rs:2:9
  |
2 |     let guess = "42".parse().expect("Not a number!");
  |         ^^^^^        ----- type must be known at this point
  |
  = note: cannot satisfy `<_ as FromStr>::Err == _`
help: consider giving `guess` an explicit type
  |
2 |     let guess: /* Type */ = "42".parse().expect("Not a number!");
  |              ++++++++++++

For more information about this error, try `rustc --explain E0284`.
error: could not compile `type_annotation` (bin "type_annotation") due to 1 previous error
```

咱们将看到其他数据类型的不同类型注解。


## 标量类型

所谓 *标量，scalar* 类型，表示单个值。Rust 有四种主要的标量类型：

- 整数
- 浮点数
- 布尔值
- 和字符

咱们可能在别的编程语言中见过这些类型。我们来了解一下他们在 Rust 中的工作原理。


### 整型

所谓 *整数，integer*，属于不带小数部分的数字。我们曾在第 2 章中用到一种整数类型，即 `u32` 类型。这种类型声明表明，与之关联的值应是个占用 32 位空间的无符号整数（有符号整数类型以 `i` 而不是 `u` 开头）。下表 3-1 列出了 Rust 中内置的整数类型。我们可使用任何的这些变种，声明某个整数值的类型。


<a name="table_3-1"></a>
**表 3-1**：Rust 中的整数类型


| 长度 | 有符号 | 无符号 |
| :-: | :- | :- |
| 8 位 | `i8` | `u8` |
| 16 位 | `i16` | `u16` |
| 32 位 | `i32` | `u32` |
| 64 位 | `i64` | `u64` |
| 128 位 | `i128` | `u128` |
| 依赖于架构（arch） | `isize` | `usize` |


每个变种都可以是有符号或无符号的，并有着明确大小（译注：位数）。*有符号，signed* 与 *无符号，unsigned*，指的是数字是否可能是负数 -- 换句话说，数字是否需要带有符号（`signed`），或者数字是否将只会是正数而因此可在不带符号下表示（`unsigned`）。这就像在纸上书写数字：当符号很重要时，数字会以一个加号或减号表示；然而，当可以安全地假设该数字是正数时，他就会以无符号给出。有符号数字是使用 [二进制补码](https://en.wikipedia.org/wiki/Two%27s_complement) 表示法存储的。

每个有符号变种都可以存储 -(2<sup>n - 1</sup>) 到 2<sup>n - 1</sup> - 1（含）的数字，其中 *n* 是该变种用到的比特数。因此，`i8` 可以存储 -(2<sup>7</sup>) 到 2<sup>7</sup> - 1 的数字，相当于 -128 到 127。无符号变种可存储 0 至 2<sup>n</sup> - 1 的数字，因此 `u8` 可存储 0 至 2<sup>8</sup> - 1 的数字，相当于 0 至 255。

此外，`isize` 与 `usize` 两种类型，取决于咱们程序运行所在计算机的体系结构：当咱们是在某种 64 位架构上时就是 64 位，当咱们是在某种 32 位架构上时则为 32 位。

咱们可以下表 3-2 所示的任何形式，书写整数的字面值。要注意可能是多种数值类型的数字字面值，则允许使用类型后缀来指定类型，比如 `57u8`。数字字面值还可使用 `_` 作为视觉分隔符，使数字更容易读取，例如 `1_000`，其将有着与咱们指定 `1000` 时相同的值。


<a name="table_3-2"></a>
**表 3-2**：Rust 中的数字字面值

| 数字字面值 | 示例 |
| :- | :- |
| 十进制，Decimal | `98_222` |
| 十六进制，Hex | `0xff` |
| 八进制，Octal | `0o77` |
| 二进制，Binary | `0b1111_0000` |
| 字节（仅限 `u8`），Byte(`u8` only) | `b'A'` |


那么，咱们怎么知道要使用何种整数类型呢？当咱们不确定时，Rust 默认选项通常是个很好的开始：整数类型默认为 `i32`。咱们会用到 `isize` 或 `usize` 的主要情形，是在索引某种集合时。


> 关于 **整数溢出**
>
> 假设咱们有个可保存 0 到 255 间值的 `u8` 类型的变量。当咱们尝试将该变量修改为某个超出该范围的值，比如 256 时，那么 *整数溢出，integer overflow* 就将发生，其会导致两种行为之一。当咱们以调试模式编译时，Rust 会包含对整数溢出的检查，当这种行为发生时，会导致咱们的程序 *中止，panic*。当程序以某种错误退出时，Rust 会使用 *中止，panicking* 一词；我们将在第 9 章的 [“`panic!` 下的不可恢复错误](../error_handling/panic.md) 小节中，更深入讨论中止运行。
>
> 当咱们以 `--release` 命令行开关在发布模式下编译时，Rust 就 *不* 会包含对导致中止运行的整数溢出的检查。相反，当溢出发生时，Rust 会执行 *二进制补码回绕，two's complement wrapping*。简言之，大于该类型可容纳最大值的值，会 “折回，wrap around” 到该类型可容纳的最小值。在 `u8` 的情况下，值 256 会变成 0，值 257 会变成 1，依此类推。程序将不中止运行，但变量将有着某个可能不是咱们期望他要有的值。依赖于整数溢出的折回行为被视为一种错误。
>
> 要显式地处理溢出的可能性，咱们可以使用由标准库针对原始数值类型所提供的下面这些方法族：
>
> - 以 `wrapping_*` 方法，比如 `wrapping_add`，在所有模式下折回；
> - 以 `checked_*` 方法，在溢出时返回 `None` 值；
> - 以 `overflowing_*` 方法，返回一个值，与一个表明是否溢出的布尔值；
> - 以 `saturating_*` 方法，于最小值或最大值处饱和。


### 浮点类型

Rust 同样有两种 *浮点数，floating-point numbers* 的两种原始类型，他们属于带有小数点的数字。Rust 的浮点类型为 `f32` 和 `f64`，大小分别是 32 和 64 位。默认类型是 `f64`，因为在现代 CPU 上，其速度与 `f32` 大致相同，但能提供更高精度。两种浮点类型都是有符号的。

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

Rust 支持咱们所期望的对所有数字类型的基本数学运算：

- 加法
- 减法
- 乘法
- 除法
- 及余数（求模）


整数的除法，会向零方向截断到最接近的整数。以下代码展示了咱们应如何在 let 语句中，使用每种数字运算：


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
    let truncated = -5 / 3; // 结果为 -1

    // 余数
    let remainder = 43 % 5;

    println! ("
        5 + 10 = {sum},
        95.5 - 4.3 = {difference}
        4 * 30 = {product}
        56.7 / 32.2 = {quotient}
        -5 / 3 = {truncated}
        43 % 5 = {remainder}");
}
```


这些语句中的每个表达式，都使用了个数学运算符并求值为单个值，然后其被绑定到某个变量。[附录 B](../appendix/ops_and_symbols.md) 包含 Rust 所提供的全部运算符清单。


### 布尔值类型

与大多数别的编程语言中一样，Rust 中的布尔类型有两个可能值：`true` 与 `false`。布尔值的大小为一个字节。Rust 中的布尔类型使用 `bool` 指定。例如:


文件名：`src/main.rs`

```rust
fn main() {
    let t = true;

    let f: bool = false; // 带有显式类型注解
}
```

使用布尔值的主要方式是经由一些条件，例如某个 `if` 表达式。我们将在 [“控制流”](control_flow.md) 小节，介绍 Rust 中 `if` 表达式的工作原理。


### 字符类型

Rust 的 `char` 类型，是这门语言的最原始字母类型。下面是声明 `char` 值的一些示例：

文件名：`src/main.rs`

```rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // 带有显式的类型注解
    let heart_eyed_cat = '😻';

    println! ("
        c 为 {c}
        z 为 {z}
        爱心猫: {heart_eyed_cat}
    ");
}
```

要注意，我们以单引号指定 `char` 字面值，与使用双引号的字符串字面值相反。Rust 的 `char` 类型大小为 4 字节，表示一个 Unicode 标量值，a Unicode scalar value，这意味着他可表示远超过 ASCII 的字符。重音字母、中日韩文字、表情符号与零宽度空格等等，都属于 Rust 中的有效 `char` 值。Unicode 标量值范围从 `U+0000` 至 `U+D7FF`，及 `U+00E000` 至 `U+10FFFF`（含）。不过，所谓 “字符” 并非真正是个 Unicode 中的概念，因此咱们对 “字符” 的直觉，可能与 Rust 中的 `char` 不一致。我们将在第 8 章的 [“以字符串存储 UTF-8 编码的文本”](../common_collections/strings.md) 小节中，详细讨论这个主题。


## 复合类型

所谓 *复合类型，compound types*，可将多个值编组为一种类型。Rust 有两种原始复合类型：

- 元组
- 数组


###  元组类型

所谓元组，是将不同类型的多个值组合在一起，成为一种复合类型的通用方式。元组有着固定的长度：一旦声明，其大小就无法增大或缩小。

通过在括号内写下一个逗号分隔的值列表，我们就创建出了个元组。元组中的每个位置都有一种类型，元组中不同值的类型不必相同。在下面这个示例中，我们已添加了可选的类型注解：


文件名：`src/main.rs`

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```


其中变量 `tup` 会绑定到整个元组，因为元组被视为单个复合元素。要获取元组中的单个值，我们可以使用模式匹配解构元组值，就像这样：


文件名：`src/main.rs`

```rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println! ("y 的值为：{y}");
}
```

这个程序首先创建了个元组，并将其绑定到变量 `tup`。其随后以 `let` 使用一个模式，取得 `tup` 并将其转换三个独立变量 `x`、`y` 与 `z`。这称为 *解构，destructuring*，因为他将单个元组分解为三部分。最后，这个程序打印 `y` 的值，即 `6.4`。


> **译注**：这种解构元组时用到的模式匹配，在 Erlang/OTP 中早已存在，且功能更为强大。Erlang/OTP 的模式匹配特性，不仅可以提取元组中的单个元素，甚至可以提取元组的其余部分而得到一个新的元组。
>
> 参考：[又来模式匹配](https://erlang.xfoss.com/part-ii/Ch03-basic_concepts.html#%E5%8F%88%E6%9D%A5%E6%A8%A1%E5%BC%8F%E5%8C%B9%E9%85%8D)

我们还可通过使用句点（`.`），后跟我们打算访问值的索引，直接访问元组元素。例如：


文件名：`src/main.rs`

```rust
fn main() {
    let x = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println! ("
        x: {:?}
        x.0：{five_hundred}
        x.1：{six_point_four}
        x.2：{one}
    ", x)
}
```


这个程序创建了个元组 `x` 并随后使用其各自索引，访问了该元组中的各个元素。与大多数编程语言一样，元组中的第一个索引为 0。

没有任何值的元组有个特殊名字，*单元值，unit*。这个值及其对应的类型都被写成 `()`，并表示一个空值或空的返回类型。当表达式不返回任何别的值时，就会隐式地返回单元值。


### 数组类型

拥有多个值集合的另一种方式，是在 *数组，array* 下。不同于元组，数组的每个元素都必须有着同一类型。与一些其他语言中的数组不同，Rust 中的数组有着固定长度。

我们要将数组中的值，写作方括号内以逗号分隔的列表：


文件：`src/main.rs`

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

当咱们希望咱们数据，如同我们迄今为止所看到的其他类型一样，分配在栈上而不是堆上时（我们将在 [第 4 章](ownership/about_ownership.md) 详细讨论栈和堆），或者当咱们希望确保始终有着固定数量元素时，数组非常有用。不过，数组不如矢量类型灵活。所谓 *矢量，vector*，是由标准库提供的一种类似集合类型，其 *是* 允许在大小（内存用量）上增大或缩小的，因为其内容位于内存堆上。当咱们不确定要使用数组还是矢量时，那么很可能咱们应使用矢量。[第 8 章](../common_collections/vectors.md) 会详细讨论矢量值。

不过，当我们清楚元素数量无需改变时，数组会更有用。例如，当咱们在程序中用到月份的那些名字时，咱们就可能会使用数组而不是矢量值，因为咱们知道他将始终包含 12 个元素：


```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
```


咱们会使用带有各个元素类型、分号以及数组中元素数量的方括号，编写数组类型，如下所示：


```rust
let a: [i32; 5] = [-1, 0, 1, 2, 3];
```


这里，`i32` 是每个元素的类型。分号后的数字 `5` 表示这个数组包含五个元素。

咱们还可通过在方括号中指定初始值、后跟一个分号及随后该数组的长度，初始化某个数组为每个元素都包含同一个值，如下所示：


```rust
let a = [3; 5];
```

名为 `a` 这个数组将包含 `5` 个都将被初始设置为 `3` 的元素。这与写下 `let a = [3, 3, 3, 3, 3]` 相同；不过以一种更简洁的方式。


### 数组元素的访问


数组是可在栈上分配的已知、固定大小的单个内存块。咱们可以使用索引访问数组的元素，就像这样：


文件名：`src/main.rs`

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    println! ("
        数组 a：{:?}
        数组的第一个元素：{first}
        第二个元素：{second}
    ", a);
}
```


在这个示例中，名为 `first` 的变量将得到值 `1`，因为这是该数组中索引 `[0]` 处的值。名为 `second` 的变量得到该数组中的索引 `[1]` 处的值 `2`。

> **译注**：`println! ("a[0]: {a[0]}");` 这种写法，会报出错误。

```console
$ cargo build
   Compiling array_demo v0.1.0 (/home/hector/rust-lang-zh_CN/projects/array_demo)
error: invalid format string: expected `}`, found `[`
  --> src/main.rs:11:17
   |
11 |         a[0]: {a[0]}
   |               - ^ expected `}` in format string
   |               |
   |               because of this opening brace
   |
   = note: if you intended to print `{`, you can escape it using `{{`

error: could not compile `array_demo` (bin "array_demo") due to 1 previous error
```

### 无效数组元素访问

我们来看看当咱们尝试访问某个超过数组末尾的元素时会发生什么。假设咱们运行下面这段，类似于第 2 章中猜数游戏的代码，从用户处获取一个数组索引：


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
        .expect("输入的索引并非一个数字");

    let element = a[index];

    println! ("位于索引 {index} 出的元素值为：{element}");
}
```

这段代码会编译成功。当咱们使用 `cargo run` 运行这段代码，并输入 `0`、`1`、`2`、`3` 或 `4` 时，程序将打印出数组中该索引处的相应值。当咱们输入一个超出数组末尾的数字，比如 `10` 时，咱们将看到如下的输出：

```console
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/invalid_array_element_access`
请输入一个数组索引。
10

thread 'main' (100331) panicked at src/main.rs:19:19:
index out of bounds: the len is 5 but the index is 10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

程序在索引操作中使用了某个无效值的时间点，引发了一次运行时错误，a runtime error。程序以一条错误信息退出，且未执行最后的 `println!` 语句。当咱们尝试使用索引访问某个元素时，Rust 将检查咱们指定的索引是否小于数组长度。当索引大于或等于这一长度时，Rust 将终止运行。这一检查必须在运行时进行，尤其在这种情况下，因为编译器不可能知道稍后用户运行代码时，将输入什么值。

这是 Rust 内存安全原则实际应用的一个例子。在许多底层语言中，这类检查都没有进行，当咱们提供了不正确索引时，无效内存就会被访问。Rust 通过立即退出，而不是允许这一内存访问并继续，保护咱们免受此类错误的影响。第 9 章详细讨论了 Rust 的错误处理，以及咱们怎样可以编写出既不会终止运行，也不允许无效内存访问的可读、安全代码。


（End）


