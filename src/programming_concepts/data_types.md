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


*标量* 类型，表示单个值。Rust 有着四个主要的标量类型：整数、浮点数、布尔值与字符。这些类型，其他语言也有。下面就深入看看他们在 Rust 中是怎样工作的。

###  整形

**Integer Types**

*整数* 是不带小数部分的数。在第 2 章中就已用到一种整数类型，即 `u32` 类型。这种类型声明表示变量关联的值，应是个无符号的、占据 32 个二进制位空间的整数（有符号整数以 `i` 而不是 `u` 开头）。下面的表 3-1 给出了 Rust 中内建的那些整数类型。可使用这些变种中的任何一个，取声明出某个整数值的类型。

*表 3-1：Rust 中的整数类型*


| 长度 | 有符号 | 无符号 |
| :-: | :- | :- |
| 8 位 | `i8` | `u8` |
| 16 位 | `i16` | `u16` |
| 32 位 | `i32` | `u32` |
| 64 位 | `i64` | `u64` |
| 128 位 | `i128` | `u128` |
| 架构决定 | `isize` | `usize` |

这每个变种，都可以是有符号或无符号的，同时有着显式的大小（二进制位数）。 *有符号* 与 *无符号* 是该数字是否可以是负数--也就是说，该数是否需带有符号（即有符号的），或者说他是否将只为正数，而因此仅能被不带符号的表示出来（即无符号）。这与在纸上写数字相像：在符号重要时，那么写下来的数字就会带有加号或减号；不过在可安全地假定数字为正数时，写下的数字就不带符号了。有符号数字，是采用 [二进制补码](https://en.wikipedia.org/wiki/Two%27s_complement) 表示法加以存储的。

每个有符号变种，都可存储自 `-(2^n-1)` 到 `2^n-1` 范围的那些数字（包括边界上的两个数字），其中的 `n` 即为变种用到的位数。那么一个 `i8` 就可以存储 从`-(2^7)` 到 `2^7-1` 的那些数字了，相当于 `-128` 到 `127`。

无符号变种则可以存储 `0` 到 `2^n - 1` 的数字，因此一个 `u8` 可以存储 `0` 到 `2^8 - 1` 的数字，相当于 `0` 到 `255`。

此外，其中的 `isize` 与 `usize` 类型，取决于程序所运行计算机的架构，这在上面的表格中，是以 `arch` 表示的：若在 `64-bit` 机器上那么就是 64 位，而若在 `32-bit` 机器上，那么就是 32 位。

可使用上面表 3-2 中的任何形式，来编写整数字面值（integer literals）。请注意数字字面值是可以将多种数字类型，作为类型后缀（a type suffix），而指定出该数字的类型的，比如 `57u8`。数字字面值中还可以使用 `_` 作为视觉分隔符，从而让数字更易于阅读，比如 `1_234_456_789_012`，这与指明 `123456789012` 有着同样的值。

*表 3-2：Rust 中的数字字面值*

| 数字字面值 | 示例 |
| :- | :- |
| 十进制（Decimal） | `98_222` |
| 十六进制（Hex） | `0xff` |
| 八进制（Octal） | `0o77` |
| 二进制（Binary） | `0b1111_0000` |
| 字节（仅限 `u8`，Byte(`u8` only)） | `b'A'` |

那么怎样知道，该用何种类型的整数呢？在不确定的时候，一般来说 Rust 默认的整数类型，即是不错的开场：整数类型默认为 `i32`。而要用到 `isize` 或 `usize` 的主要场合，则是在对一些类别的集合进行索引的时候（the primary situation in which you'd use `isize` or `usize` is when indexing some sort of collection）。

> 关于 **整数溢出**
>
> 比方说有个类型为 `u8` 的、可保存 `0` 到 `255` 之间值的变量。在尝试将该变量修改为超出那个范围的某个值，比如 `256` 时，就会发生 *整型溢出（integer overflow）*，而整型溢出又可导致两种行为之一。在以调试模式进行程序编译时，Rust 就会包含整数溢出的检查，在发生了整数溢出时，就会导致程序进入 *错误（panic）* 状态。对于程序因错误而退出执行这种情况，Rust 使用了 猝死（paniking） 这个词语；在第 9 章中的 [带有 `panic!` 宏的不可恢复性错误](Ch09_Error_Handling.md#带-panic-的不可恢复错误) 小节，将更深入地讨论到程序因错误而终止运行的情况。
>
> 在以 `--release` 开关进行发布模式的编译时，Rust 就不会包含对引起程序终止运行的整数溢出的检查。这时若发生了溢出，Rust 就会执行 *二进制补码封装（two's complement wrapping）*。简而言之，对于比那种类型能保存的最大值还要大的值，就会被“回卷（wrap around）”到那种类型所能保存的最小值。以 `u8` 为例，值 `256` 就变成了 `0`，值 `257` 就变成了 `1`，如此等等。这样程序就不会死掉，而那个变量则会有着一个或许不是所期望的值。对整数溢出的回卷行为的依赖，被视为一种错误（Relying on integer overflow's wrapping behavior is considered an error）。
>
> 要显式地处理可能的溢出，就要用到标准库为原生数字类型所提供的以下方法族（these families of methods provided by the standard library for primitive numeric types）：
>
> - 以 `wrapping_*` 这些方法的所有模式的封装，比如 `wrapping_add`（wrap in all modes with the `wrapping_*` methods, such as `wrapping_add`）；
> - 存在以 `checked_*` 方法的溢出时，返回 `None` 值（return the `None` value if there is overflow with the `checked_*` methods）；
> - 返回该值，以及一个表示是否存在带有 `overflowing_*` 方法的溢出的布尔值（return the value and a boolean indicating whether there was overflow with the `overflow_*` methods）；
> - 以 `saturating_*` 方法，实现该值的最小或最大值处饱和（saturate at the value's minimum or maximum values with `saturating_*` methods）。


### 浮点类型

Rust 同样有两种原生的 *浮点数* 类型，所谓浮点数，是带有小数点的数字。Rust 的浮点数类型为 `f32` 与 `f64`，分别为 32 位及 64 位大小。由于在现代 CPU 上 `f64` 与 `f32` 处理速度大致一样，不过前者具备了更高的精度，因此默认类型就定为了 `f64`。两种浮点数类型都是有符号的。

下面的示例展示了具体的浮点数：

文件名：`src/main.rs`

```rust
fn main() {
    let x = 2.0;    // f64
    let y: f32 = 3.0;   // f32
}
```

浮点数的表示，符合 [IEEE-754 标准](https://standards.ieee.org/ieee/754/6210/)。`f32` 类型是单精度浮点数，而 `f64` 则是双精度的。

### 数字运算

Rust 支持在所有数字类型上、所期望的那些基本数学运算：加法、减法、乘法、除法，及余数。整数除法会向下取到最接近的整数结果。下面的代码展示了在 `let` 语句中，如何运用各种数字运算：

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
        5 + 10 = {},
        95.5 - 4.3 = {}
        4 * 30 = {}
        56.7 / 32.2 = {}
        2 / 3 = {}
        43 % 5 = {}", sum, difference, product, quotient, floored, reminder);
}
```

这些语句中每个表达式都使用了一个数学运算符，并求到一个单值，该单值随后被绑定到变量。[附录 B](Ch99_Operators.md) 包含了 Rust 所提供的全部运算符。

### 布尔值类型

与多数其他编程语言中一样，Rust 中的布尔值类型也有两个可能的值：`true` 及 `false`。布尔值大小为一个字节。Rust 中的布尔值类型，指定使用 `bool` 关键字。比如：

文件名：`src/main.rs`

```rust
fn main() {
    let t = true;

    let f: bool = false; // 带有显式类型注解
}
```

主要通过条件判断，来使用布尔值，比如在 `if` 表达式中。在 [控制流（Control Flow）](#控制流程control-flow) 小节，会讲到 Rust 中 `if` 表达式的工作方式。

### 字符类型

Rust 的 `char` 类型，是这门语言最为原生的字母类型。下面就是一些声明 `char` 值的示例：

文件名：`src/main.rs`

```rust
fn main() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println! ("c 为 {}, z 为 {}, 爱心猫: {}", c, z, heart_eyed_cat);
}
```

请注意，相比使用双引号来给字符串字面值进行值的指定，这里是以单引号来对这些 `char` 的字面值进行指定的。Rust 的 `char` 类型，有着四个字节的大小，而表示了 Unicode 的标量值，这就意味着他可以表示比仅仅 ASCII 要多得多的符号。像是重音字母（accented letters）；中文、日语、韩语等等；emoji 符号；以及那些零宽度空格等等，在 Rust 中都是有效的 `char` 取值。Unicode 标量值的范围，是从 `U+0000` 到 `U+D7FF`，及 `U+E000` 到 `U+10FFFF`，包含边界值。不过，“字符（character）” 并非 Unicode 中的概念，因此对 “字符” 为何物的主观认识，可能与 Rust 中 `char` 的本质有所差别。在第 8 章中的 [用字符串存储 UTF-8 编码的文本](Ch08_Strings.md#使用-string-存储-utf-8-编码的文本) 小节，将对此话题进行讨论。

## 复合类型

*复合类型（compound types）* 可将多个值组合成一个类型。Rust 有着两个原生的复合类型：元组与数组（tuples and arrays）。

###  元组类型

元组是将数个不同类型的值，组合成一个复合类型的一般方式。元组是固定长度的：一旦被声明出来，他们的大小就无法扩大或缩小了。

通过编写放在圆括号里面的、逗号分隔的值清单，来创建元组。元组中每个位置都有着一种类型，同时元组中不同值的类型不必一致。下面的示例中，加上了那些可选的类型注解：

文件名：`src/main.rs`

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

由于元组被当作是单一复合元素，因此这里的变量 `tup` 绑定到了那整个的元组。要从元组获取到其单个值，就要使用模式匹配，来对元组值进行解构，就像下面这样：

文件名：`src/main.rs`

```rust
fn main() {
    let tup = (500, 6.4, 1, "元组的最后一个元素");

    let (x, y, z, a) = tup;

    println! ("a 的值为：{}", a);
}
```

这个程序首先创建了一个元组，并将其绑定到了变量 `tup`。随后以 `let` 关键字，使用了一个模式来取得 `tup`，并将其转换为四个独立变量，分别为 `x`、`y`、`z` 与 `a`。由于该操作将单个的元素，打散为了四个部分，因此称之为 *解构（destructuring）*。最后，程序打印出了 `a` 的值，即为 `元组的最后一个元素`。

还可以通过使用句点（`.`）后带上想要访问值的索引，还可直接对某个元组元素进行访问。比如：

文件名：`src/main.rs`

```rust
fn main() {
    let x = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println! ("x.0： {}, x.1：{}, x.2：{}", five_hundred, six_point_four, one);
}
```

此程序创建了元组 `x`，并随后通过使用各个元素的索引，而构造了他们的新变量。与绝大多数编程语言一样，元组的首个索引为 `0`。

没有任何值的元组，`()`，是种仅有一个值的特殊类型，同样写作 `()`。该类型叫做 *单元类型（unit type）*，同时这个值叫做 *单元值（unit value）*。那些不返回任何值的表达式，就会显式地返回这个单元值。


### 数组类型

*数组（array）* 是拥有多个值集合的另一种方式。不同于元组，数组中的每个元素，都必须是同一类型。与其他一些语言中的数组不同，Rust 中的数组是定长的。

以方括号（`[]`）中逗号分隔的方式，来编写数组中个那些值：

文件：`src/main.rs`

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

在希望数据分配在栈而不是堆（在 [第 4 章](Ch04_Understanding_Ownership.md#何谓所有权) 将进一步讨论栈与堆）上时，或希望确保一直有着固定数目的元素时，数组就派上用场了。然而，数组不如矢量类型灵活。矢量是标准库所提供的，一种类似的集合类型，其大小 *可以* 变大或缩小。在不确定要使用数组，还是要使用矢量类型时，那多半就应该使用矢量了。[第 8 章](Ch08_Common_Collections.md#使用矢量类型对值清单进行存储) 对矢量类型进行了更详细的讨论。

尽管如此，在了解了元素数目无需变化时，数组将更为有用。比如，在程序中正使用着各个月份名字时，由于是知道那将总是包含 12 个元素，因此就要使用数组而非矢量类型：

```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
```

数组类型的编写，是以方括弧（`[]`）括起来的，各个元素的类型，一个分号（`;`），和数组中元素个数的形式，像下面这样：

```rust
let a: [i32, 5] = [-1, 0, 1, 2, 3];
```

这里，`i32`就是各个元素的类型。在分号之后，数字 `5` 表示该数组包含五个元素。

还可以通过在方括弧（`[]`）中，先指定初始值，接着一个分号（`;`），及随后数组长度的方式，初始化出一个包含各个元素为同一个值的数组，如下所示：

```rust
let a = [3; 5];
```

名叫 `a` 这个这个数组，将包含 `5` 个元素都将被初始化设置为值 `3` 的元素。这与 `let a = [3, 3, 3, 3, 3];` 的写法是一样的，不过是一种更简洁的方式。

### 对数组元素的访问

一个数组，即是可分配在栈上的、已知及固定大小的一块内存。使用索引，就可以对数组的那些元素进行访问，比如下面这样：

文件名：`src/main.rs`

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let last = a[a.len()-1];

    println! ("数组的第一个元素：{}，最后一个元素：{}", first, last);
}
```

在这个示例中，由于值 `1` 为该数组中索引为 `[0]` 处的值，因此名为 `first` 的元素将获取到值 `1`。而名为 `last` 的变量，将从数组中的索引 `[4]` 获取到值 `5`。

**无效的数组元素访问**

下来来看看，在尝试访问超出数组末端的数组元素时，会发生什么。就是说在运行下面这个程序时，与第二章中的猜数游戏类似，要从用户那里获取到一个数组索引：

文件名：`src/main.rs`

```rust
use std::io;
use std::process;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println! ("请输入一个数组索引。");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("读取行失败");

    let index: usize = match index.trim()
        .parse() {
            Ok(num) => num,
            Err(_) => {
                println! ("输入的索引并非数字");
                process::exit(0);
            }
        };

    let element = a[index];

    println! (
        "位于索引 {} 处的元素值为：{}",
        index, element);
}
```

此代码会成功编译。而在使用 `cargo run` 运行此代码，并输入 `0`、`1`、`2`、`3` 或 `4` 时，程序将打印出该数组中对应与那个索引处的值。而若输入了一个超出数组末端的数字，比如 `10`，那么就会看到下面这样的输出：

```console
thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10', src/main.rs:24:19
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

当在索引操作中使用了无效值的时间点，该程序造成了一个 *运行时（runtime）* 错误。该程序以一条错误消息退出，而并未执行那最后的 `println!` 语句。在尝试使用索引访问某个元素时，Rust 会就所指定的索引，小于数组长度进行检查。若该索引大于或等于数组长度，Rust 就会出错。此项检查必须要在运行时进行，尤其是在此示例中，这是因为编译器几无可能知道在用户随后运行此程序时，会输入什么样的值。

这是 Rust 内存安全准则的一个活生生的示例。在许多底层语言中，此种检查都未实现，进而在提供了不正确的索引时，就会访问到无效的内存。Rust 通过立即退出而不是允许这种无效内存访问并继续运行，而保护免于此类错误。第 9 章将对 Rust 的错误处理进行过多的讨论。


