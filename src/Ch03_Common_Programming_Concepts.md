# 常见编程概念

本章涵盖了出现在几乎所有编程语言中的一些概念，以及这些概念在 Rust 中运作方式。众多编程语言，在他们各自核心，都有着许多共同的东西。出现在本章中的这些概念，没有一个是 Rust 独有的，然而这里是要就他们在 Rust 语境下进行讨论，并对使用这些概念的相关约定进行解释。

具体来讲，本章将要掌握到变量、基本类型、函数、注释及控制流等概念。这些基本概念将出现在全部 Rust 程序中，而早点掌握他们，就会给到一个强大的核心起点。

> **关键字（keywords）**
>
> Rust 语言有着一套 *关键字*，他们是保留的，仅由语言使用，在这点上与其他语言没有什么不同。牢记是不可以将这些词汇，用作变量或函数的名称的。多数关键字都有着特殊意义，而会使用他们来完成 Rust 程序中不同任务；其中有少数几个当前并没有关联功能，他们被保留用于未来将添加到 Rust 的功能。在 [附录 A](Ch99_Appendix_A.md) 就能看到关键字清单。


## 变量与可变性

**Variables and Mutability**

就如在之前的 ["用变量保存值"](Ch02_Programming_a_Guessing_Game.md#storing-values-with-variables) 小节中所讲的那样，默认变量是不可变的。这是 Rust 所提供的，推动利用 Rust 赋予的安全和易于并发代码编写方式的众多措施之一（by default variables are immutable, this is one of many nudges Rust gives you to write your code in a way that takes advantage of the safety and easy concurrency that Rust offers）。尽管如此，还是有将变量作为可变的选项。下面就来搞清楚，为何 Rust 会鼓励偏向不可变，以及为何有时会希望不接受 Rust 的建议。

在变量为不可变时，一旦值被绑定到某个名字，那么就无法修改那个值了。为对此进行演示，就来通过使用 `cargo new variables` 在 `projects` 目录中生成一个新的名为 `variables` 的项目。

然后，在那个新的 `variables` 目录中，打开 `src/main.rs` 并将其代码替换为下面的代码。此代码当然不会被编译，这里首先要对不可变错误加以检视。

```rust
fn main() {
    let x = 5;
    println! ("x 的值为：{}", x);

    x = 6;
    println! ("x 的值为：{}", x);
}
```

保存并使用 `cargo run` 运行这个程序。就会受到错误消息，如下面这个输出：

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

此示例显示了编译器如何帮助发现程序中的错误。编译器错误可能令人沮丧，但这些编译器错误真的意味着，程序未有安全地执行要程序做的事情；编译器错误并不表示你不是一名良好的程序员！即使那些经验丰富的 Rust 公民，也会收到编译器错误。

该错误消息表示错误原因为 `cannot assing twice to immutable variable 'x'`，是因为有尝试将第二个值赋给那个不可变的 `x` 变量。

在尝试修改某个被指定为不可变的值时，由于这种情况会导致程序错误，因此这个时候收到编译时错误尤为重要。代码一部分的运作，是建立在值将绝不会改变这种假定上，而代码另一部分却修改了那个值，那么就有可能代码的第一部分未有完成他预计要完成的工作了。此类程序错误的原因，就难于追踪到真相，尤其是在代码第二部分仅 *有的时候* 才对那个值进行修改时。Rust 编译器保证了在表明某个值不会变化时，那么那个值就真的不会变化，如此就不必亲自去紧盯着那个变量了。代码也由此而更易于推演。

然而可变则可能会非常有用，并能令到代码更便于编写。变量仅在默认情况下是不可变的；就如同在第 2 章中所做的那样，可通过在变量名字前添加 `mut` 关键字，让变量成为可变。`mut` 的添加，也向将来代码的读者传达了某种意图，表示代码的其他部分，会对这个变量的值进行修改。

比如，来将 `src/main.rs` 修改为下面这样：

文件名：`src/main.rs`

```rust
fn main() {
    let mut x = 5;
    println! ("x 的值为：{}", x);

    x = 6;
    println! ("x 的值为：{}", x);
}
```

在此时运行这个程序时，就会得到这样的输出：

```rust
$ cargo run                                                       101 ✘
   Compiling variables v0.1.0 (/home/peng/rust-lang/projects/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 0.46s
     Running `target/debug/variables`
x 的值为：5
x 的值为：6
```

在使用了 `mut` 关键字时，就被允许将绑定到 `x` 的值从 `5` 修改到 `6` 了。除了防止程序错误之外，还要考虑多种权衡。比如，在使用着大型数据结构时，就地修改其的一个实例，就会比拷贝并返回新近分配的实例要快一些（for example, in cases where you're using large data structures, mutating an instance in place may be faster than copying and returning newly allocated instances）。而对于较小的数据结构，创建其新实例，并以更具函数式编程风格来编写代码，则可能更易于理解，那么由此带来的性能下降，相对所取得的思路清晰，也会是值得的。

## <a id="constants"></a> 常量

与不可变变量类似， *常量（constants）* 是一些绑定到名字且不允许修改的值，但常量与变量之间，有些差异。

首先，是不允许在常量上使用 `mut` 关键字的。常量不光是默认不可变的 -- 他们一直都是不可变的。常量的声明用的是 `const` 关键字，而不是 `let` 关键字，同时值的类型 *必须* 被注解（be annotated）。在下一小节，[数据类型](#data-types)，就会讲到类型和类型注解了，因此现在不要关心细节。只要明白必须始终对类型进行注解。

可在任何作用域，包括全局作用域中声明常量。而当在全局作用域中声明常量时，则会让那些代码中许多部分都需要知悉的值的常量，变得有用起来。

常量与不可变变量的最后一个区别，就是常量只能被设置到一个常量表达式，而不能被设置为只能在运行时计算出结果的值。

下面是一个常量声明的示例：

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

该常量的名字为 `THREE_HOURS_IN_SECONDS`，而他的值就被设置为了 `60` （即一分钟的秒数）乘以 `60` （即一小时的分钟数）乘以 `3` （此程序中要计数的小时数）。Rust 关于常量命名的约定，即为全部使用大写，在词汇之间用下划线隔开。编译器在运行时，能够执行一套受限的运算，这样就可以选择将常量值，以这种更易于理解和验证的方式写出来，而不是将该常量设置为值 `10,800`。请参阅 [Rust 参考手册有关常量求值的小节](https://doc.rust-lang.org/reference/const_eval.html)，了解更多有关在声明常量时可使用那些运算的信息。

常量在程序运行的全部时间、在其被声明的作用域内部，都是有效的。常量的这个属性，令到常量对于应用域内的那些、程序多个部分都需要知悉的值来说，变得有用起来，比如某个游戏全部玩家所允许赚到的最大点数，或光速常量。

对那些整个程序都要用到的、作为常量的硬编码值进行取名，对于向代码将来的维护者们传达那些值的意义，是相当有用的。对于未来需要更新硬编码值来说，对常量命名就让那些需要修改的代码只有一处要改，而对此带来帮助。

## 遮蔽（shadowing）

如同在第 2 章中的猜数游戏里看到的那样，可声明一个与先前变量同名的新变量。Rust 公民们表示，那第一个变量是被第二个给 *遮蔽* 掉了，这就意味着在用到这个变量是，程序所看到的，会是第二个变量的值。通过使用一样的变量名字，以及重复使用 `let` 关键字，就可对某个变量进行遮蔽，如下所示：

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

这个程序首先将 `x` 绑定到值 `5`。随后通过重复 `let x =`，取原来的值并加上 `1`，而对 `x` 做了遮蔽操作，因此 `x` 的值此时就为 `6` 了。之后，在一个内部作用域内，第三个 `let` 语句也对 `x` 进行了遮蔽，将先前的值乘以 `2`，就给到 `x` 一个值 `12`。在那个内部作用域完毕时，那个内部遮蔽就结束了，同时 `x` 回到仍为 `6`。在运行这个程序时，他将输出下面的内容：


```console
$ cargo run                                                        ✔
   Compiling variables v0.1.0 (/home/peng/rust-lang/projects/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 0.47s
     Running `target/debug/variables`
内部作用域中 x 的值为：12
x 的值为：6
```

由于在不小心而尝试在不带 `let` 关键字而重新赋值给该变量时，会收到编译时错误，因此遮蔽不同于构造一个`mut` 的变量。通过使用 `let` 关键字，就可以在值上执行少量的转换操作，而在这些转换操作完成后又将该变量置入到不可变。

`mut` 与遮蔽的另一不同之处，则是由于再次使用`let`关键字时，有效地创建出了一个新变量，因此就可以改变那个值的类型，而仍然重用那同样的变量名字。比如说程序要通过用户输入若干空格字符，来询问用户希望在一些文本之间留多少个空格，而此时又要将用户输入的若干个空格，保存为一个数字：

```rust
let spaces = "    ";
let spaces = spaces.len();
```

第一个 `spaces` 变量是字符串类型，而第二个 `spaces` 变量则是数字类型。遮蔽因此而免于不得不苦苦思索不同的变量名字，诸如 `spaces_str` 及 `spaces_num`；相反，是可以重新较简单的 `spaces` 名称。然而，若尝试对这个变量使用 `mut` 关键字，就会收到一个编译时错误，如下所示：

```rust
let mut spaces = "    ";
spaces = spaces.len();
```

错误是说不允许转变变量类型：

```console
$ cargo run                                                        ✔
   Compiling variables v0.1.0 (/home/peng/rust-lang/projects/variables)
error[E0308]: mismatched types
  --> src/main.rs:14:14
   |
13 |     let mut spaces = "    ";
   |                      ------ expected due to this value
14 |     spaces = spaces.len();
   |              ^^^^^^^^^^^^ expected `&str`, found `usize`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `variables` due to previous error
```

现在已经完成变量运行机制的探讨，接卸来就要看看这些变量可以有的那些其余数据类型了。


## <a id="data-types"></a> 数据类型

Rust 的所有值，都属于某种确切的 *数据类型（data type）*，数据类型告诉 Rust 所指定的是何种数据，进而 Rust 才知道该怎样使用那个数据。接下来会看看两个数据类型的子集：标量（scalar）类型与复合（compound）类型。

请牢记 Rust 是门 *静态类型（statically typed）* 语言，这就意味着在运行时，他必须清楚所有变量的类型。基于值与对变量的使用方式，编译器通常可以推断出希望变量使用何种类型来。在可能有许多中类型的情况下，就如同第 2 章 [将猜数与秘密数字比较](Ch02_Programming_a_Guessing_Game.md#compring-the-guess-to-the-secret-number) 小节中，使用 `parse` 把一个 `String` 转换成数字类型时，就必须添加一个类型注释，如下面这样：

```rust
let guess: u32 = "42".parse().expect("这不是个数字！");
```

若这里添加类型注解，那么 Rust 就会给出下面的错误，表示编译器需要更多信息来明白这里想要使用何种类型：

```console
$ cargo build                                                  101 ✘
   Compiling variables v0.1.0 (/home/peng/rust-lang/projects/variables)
error[E0282]: type annotations needed
  --> src/main.rs:19:9
   |
19 |     let guess = "42".parse().expect("那不是个数字！");
   |         ^^^^^ consider giving `guess` a type

For more information about this error, try `rustc --explain E0282`.
static HELLO_WORLD: &str = "你好，世界！";

fn main() {
    println! ("名字为：{}", HELLO_WORLD);
}rror: could not compile `variables` due to previous error
```

接下来就会见识到其他数据类型的类型注解。


## 标量类型（Scalar Types）

*标量* 类型，表示单个值。Rust 有着四个主要的标量类型：整数、浮点数、布尔值与字符。这些类型，其他语言也有。下面就深入看看他们在 Rust 中是怎样工作的。

### <a id="integer-types"></a> 整形

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
> 比方说有个类型为 `u8` 的、可保存 `0` 到 `255` 之间值的变量。在尝试将该变量修改为超出那个范围的某个值，比如 `256` 时，就会发生 *整型溢出（integer overflow）*，而整型溢出又可导致两种行为之一。在以调试模式进行程序编译时，Rust 就会包含整数溢出的检查，在发生了整数溢出时，就会导致程序进入 *错误（panic）* 状态。对于程序因错误而退出执行这种情况，Rust 使用了 猝死（paniking） 这个词语；在第 9 章中的 [带有 `panic!` 宏的不可恢复性错误](Ch09_Error_Handling.md#unrecoverable-errors-with-panic) 小节，将更深入地讨论到程序因错误而终止运行的情况。
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

主要通过条件判断，来使用布尔值，比如在 `if` 表达式中。在 [控制流（Control Flow）](#control-flow) 小节，会讲到 Rust 中 `if` 表达式的工作方式。

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

请注意，相比使用双引号来给字符串字面值进行值的指定，这里是以单引号来对这些 `char` 的字面值进行指定的。Rust 的 `char` 类型，有着四个字节的大小，而表示了 Unicode 的标量值，这就意味着他可以表示比仅仅 ASCII 要多得多的符号。像是重音字母（accented letters）；中文、日语、韩语等等；emoji 符号；以及那些零宽度空格等等，在 Rust 中都是有效的 `char` 取值。Unicode 标量值的范围，是从 `U+0000` 到 `U+D7FF`，及 `U+E000` 到 `U+10FFFF`，包含边界值。不过，“字符（character）” 并非 Unicode 中的概念，因此对 “字符” 为何物的主观认识，可能与 Rust 中 `char` 的本质有所差别。在第 8 章中的 [用字符串存储 UTF-8 编码的文本](Ch08_Strings.md#storing-utf-8-encoded-text-with-strings) 小节，将对此话题进行讨论。

## 复合类型

*复合类型（compound types）* 可将多个值组合成一个类型。Rust 有着两个原生的复合类型：元组与数组（tuples and arrays）。

### <a id="the-tuple-type"></a> 元组类型

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

在希望数据分配在栈而不是堆（在 [第 4 章](Ch04_Understanding_Ownership.md#what-is-ownership) 将进一步讨论栈与堆）上时，或希望确保一直有着固定数目的元素时，数组就派上用场了。然而，数组不如矢量类型灵活。矢量是标准库所提供的，一种类似的集合类型，其大小 *可以* 变大或缩小。在不确定要使用数组，还是要使用矢量类型时，那多半就应该使用矢量了。[第 8 章](Ch08_Common_Collections.md#vectors) 对矢量类型进行了更详细的讨论。

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

### 无效的数组元素访问

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

## 函数

函数遍布于 Rust 代码中。而那个这门语言中最重要函数之一：`main` 函数，也一早就见到过了，`main` 函数可是许多程序的入口点。通过那个还已见到过的 `fn` 关键字，就可以声明出新的函数来。

Rust 代码使用 *蛇形命名法（snake case）*，作为函数与变量命名的约定样式，以这种命名法，函数及变量名称中的全部字母都是小写的，同时用下划线来分隔单词。下面就是一个包含了示例函数定义的程序：

文件名：`src/main.rs`

```rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println! ("另一函数。");
}
```

这里通过敲入 `fn` 关键字，接着的是函数名字，以及一套圆括号（`()`），定义出了一个函数。而那一对花括弧（`{}`），则告诉编译器，函数体在哪里开始和结束。

通过敲入函数名字，接上一对圆括号（`()`），就可以对已定义好的函数进行调用。由于 `another_function` 在程序中定义过，因此就可以在 `main` 函数里头对其调用。请注意在源代码中，是在 `main` 函数 *之后* 定义的 `another_function`；原本也可以在 `main` 函数之前定义他。Rust 不会关心在何处定义函数，只要他们在某处有定义即可。

为进一步对 Rust 的函数加以探索，就来创建一个新的名为 `functions` 的二进制可执行项目。将这个 `another_function` 示例放在 `src/main.rs` 中并运行。就会看到如下的输出：

```console
$ cargo run
   Compiling functions v0.1.0 (/home/peng/rust-lang/projects/functions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32s
     Running `target/debug/functions`
Hello, world!
另一函数。
```

> 注：二进制项目（a binary project），是与库源代码项目相对应的，可生成二进制可执行程序的项目。

这些代码行，是以他们出现在 `main` 函数中的顺序执行的。首先打印出的是 `Hello, world!` 消息，而随后 `another_function` 就被调用了，同时他的消息被打印出来。

### 参数

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

### 语句及表达式

函数体是由一系列语句构成，这些语句可以是表达式结束的，也可以不是。到目前为止，所讲到的函数，都没有包含语句以表达式结束，不过有见到过表达式作为语句一部分的情况。由于 Rust 是基于表达式的语言，那么这一点就很重要，是要掌握的特征。其他语言并无这同样的特征，因此接下来就要看看语句和表达式究竟是何物，以及他们对函数体影响的不同。

*语句（statements）* 是一些完成某些操作而不返回值的指令。 *表达式（expressions）* 会求得一个结果值。来看看一些示例。

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


### 有返回值的函数

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

## 注释

所有程序员都会致力于让他们的代码易于理解，而有时是需要额外解释的。在这种情况下，程序员们就会在他们的源代码中，留下会被编译器忽略的 *注释（comments）*，而那些阅读到源代码的人会发现有用。

下面就是个简单的注释：

```rust
// hello, world
```
在 Rust 中，惯用的注释风格，是以双斜杠来开始一条注释，同时该注释会持续到那行的结束。对于那些超过一行的注释，在每行注释就都要包含 `//`，就像这样：

```rust
// 那么这里要编写一些复杂的注释，这注释长到要用多个行
// 才能写完！噢！还好，这条注释会解释接下来要做些什么。
```

注释也可以放在那些包含代码行的末尾：

文件名：`src/main.rs`

```rust
fn main() {
    let lucky_number = 7; // 今天我感受到了好运
}
```

不过更常见的则是以下面这种形式运用的注释，其中注释位处单独的、在其要注解代码之上的行：

文件名：`src/main.rs`

```rust
fn main() {
    // 今日感到幸运
    let lucky_number = 7;
}
```

Rust 还有另外一种注释，叫做文档注释，在第 14 章的 [将代码箱发布到 Crates.io](Ch14_More_about_Cargo_and_Crates.io.md#publishing-a-crate-tocrates-io) 中会对文档注释进行讨论。

## 控制流程（Control Flow）

根据条件是否为真，来运行某些代码，或者在条件为真时重复运行某些代码的能力，是绝大多数语言的根基。实现Rust代码执行流程控制最常见的结构，即是 `if` 表达式和循环。

### `if` 表达式

`if` 表达式实现了根据条件对代码进行分支。提供到一个条件，然后就表明，“在该条件满足时，运行这个代码块。在条件不满足时，就不要运行这个代码块。”

请在 `projects` 目录下，创建一个新的、名为 `branches` 的项目，来探索这个 `if` 表达式。在 `src/main.rs` 文件中，输入以下代码：

文件名：`src/main.rs`

```rust
fn main() {
    let number = 3;

    if number < 5 {
        println! ("条件为真");
    } else {
        println! ("条件为假");
    }
}
```

全部 `if` 表达式，都是以关键字 `if` 开头的，接着的是一个条件。在此示例中，那个条件就变量 `number` 是否小于 `5` 进行检查。是把要在条件为真时立即执行的代码块，放在条件之后、一对花括号里头。`if`表达式中与那些条件相关联的代码块，有时也叫做 *支臂（arms）*，这与在第 2 章的 [将猜数与秘密数字比较](Ch02_Programming_a_Guessing_Game.md#comparing-the-guess-to-the-secret-number) 小节中讨论过的 `match` 表达式中的支臂一样。

作为可选项，还可以包含一个 `else` 表达式，即这里做的那样，从而给到程序一个替代性的、将在条件求解结果为 `false` 时执行的代码块。在未提供`else`表达式，且条件为 `false` 时，程序将直接跳过那个 `if` 代码块，而前往接下来的代码处。

尝试运行此代码；将看到下面的输出：

```console
$ cargo run                                                            ✔
   Compiling branches v0.1.0 (/home/peng/rust-lang/projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/branches`
条件为真
```

下面来试着将 `number` 的值修改为一个令到该条件为 `false` 的值，看看会发生什么：

```rust
    let number = 7;
```

再运行这个程序，然后看看输出：

```console
$ cargo run                                                            1 ✘
   Compiling branches v0.1.0 (/home/peng/rust-lang/projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.45s
     Running `target/debug/branches`
条件为假
```

还值得注意的是，此代码中的条件 *必须* 是个 `bool` 类型。在条件不是 `bool` 类型时，就会收到错误。比如，尝试运行下面的代码：

文件名：`src/main.rs`

```rust
fn main() {
    let number = 3;

    if number {
        println! ("数字是 3");
    }
}
```

这次的 `if` 条件求解为一个 `3` 的值，进而 Rust 抛出一个错误：

```console
$ cargo run                     ✔
   Compiling branches v0.1.0 (/home/peng/rust-lang/projects/branches)
error[E0308]: mismatched types
 --> src/main.rs:4:8
  |
4 |     if number {
  |        ^^^^^^ expected `bool`, found integer

For more information about this error, try `rustc --explain E0308`.
error: could not compile `branches` due to previous error
```

该错误表明 Rust 期望得到一个 `bool` 值但得到的是个整数。与诸如 Ruby 和 JavaScript 那样的语言不同，Rust 不会自动将非布尔值转换为布尔值。必须显式地且一直提供给 `if` 一个布尔值作为其条件。比如希望那个 `if` 代码块，仅在某个数字不等于 `0` 的时候运行，那么就可以将这个 `if` 表达式修改为下面这样：

文件名：`src/main.rs`

```rust
fn main() {
    let number = 3;

    if number != 0 {
        println! ("数字为非零数");
    }
}
```

运行此代码，就会打印出 `数字为非零数`。


### 用 `else if` 来处理多个条件

通过在 `else if` 表达式中，结合 `if` 和 `else`，就可以使用多个条件。比如：

文件名：`src/main.rs`

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println! ("数字可被 4 整除");
    } else if number % 3 == 0 {
        println! ("数字可被 3 整除");
    } else if number % 2 == 0 {
        println! ("数字可被 2 整除");
    } else {
        println! ("数字不可被 4、3 或 2 整除");
    }
}
```

此程序有着其可接收的四个可能路径。在运行他时，就会看到下面的输出：

```console
$ cargo run                                                                           101 ✘
   Compiling branches v0.1.0 (/home/peng/rust-lang/projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.45s
     Running `target/debug/branches`
数字可被 3 整除
```

在该程序执行时，就会依次检查各个 `if` 表达式，并执行那第一个条件成立的代码体。请注意即便 `6` 是可被 `2` 整除的，却并未看到输出 `数字可被 2 整除`，也没看到那个 `else` 代码块的 `数字不能被 4、3 或 2 整除` 文字。这是由于 Rust 只执行了第一个为真条件下的代码块，而一旦他发现了一个，就在不会检查剩下的那些条件了。

使用太多的 `else if` 表达式，就会让代码杂乱无章，因此在有多于一个这样的表达式时，或许就应对代码进行重构了。第 6 章描述了针对这样情况的一种强大的 Rust 分支结构，名为`match` 模式匹配。

### 在 `let` 语句中使用 `if` 关键字

由于 `if` 是个表达式，那么就可以在 `let` 表达式的右边使用他，来将其结算结果，赋值给某个变量，如下面的清单 3-2 所示：

文件名：`src/main.rs`

```rust
fn main() {
    let condition = true;

    let number = if condition { 5 } else { 6 };

    println! ("number 的值为：{}", number);
}
```

*清单 3-2：将`if` 表达式的结果赋值给某个变量*

其中的 `number` 变量，就会被绑定到那个 `if` 表达式的计算结果上。运行此代码看看会发生什么：

```console
$ cargo run                                                                               ✔
   Compiling branches v0.1.0 (/home/peng/rust-lang/projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.45s
     Running `target/debug/branches`
number 的值为：5
```

请记住代码块会求解到其中最后一个表达式的值，且数字本身也就是表达式。在此示例中，整个 `if` 表达式的值，是取决于会执行到哪个代码块的。这就意味着那些该 `if` 表达式各个支臂的、具备作为 `if` 表达式运算结果的那些值，必须要是相同类型；在清单 3-2 中，`if` 支臂和 `else` 支臂的运算结果，就都是 `i32` 类型的整数。若这些类型不匹配，就如下面的示例中那样，则会收到错误：

文件名：`src/main.rs`

```rust
fn main() {
    let condition = true;

    let number = if condition { 5 } else { "six" };

    println! ("number 的值为：{}", number);
}
```

在尝试编译这段代码时，就会收到错误。其中的 `if` 与 `else` 支臂的值类型不兼容，同时 Rust 还准确标明了在程序中何处发现的该问题：

```console
$ cargo run                                                                               ✔
   Compiling branches v0.1.0 (/home/peng/rust-lang/projects/branches)
error[E0308]: `if` and `else` have incompatible types
 --> src/main.rs:4:44
  |
4 |     let number = if condition { 5 } else { "six" };
  |                                 -          ^^^^^ expected integer, found `&str`
  |                                 |
  |                                 expected because of this

For more information about this error, try `rustc --explain E0308`.
error: could not compile `branches` due to previous error
```

`if` 代码块中的表达式，求解为整数，而`else` 代码块中的表达式求解为了字符串。由于变量必须有着单一类型，且 Rust 需要知道在运行时变量 `number` 的类型是什么，那么显然这代码是不会工作的。清楚 `number` 的类型，就允许编译器在所有用到 `number` 的地方，验证其类型的有效性。而如果只有在运行时才确定出 `number` 的类型，那么 Rust 就无法做到这一点；若编译器务必要对全部变量的多个假定类型进行跟踪，那么编译器就会更为复杂，且做到更少代码保证。

## 循环下的重复

多次执行某个代码块常常是有用的。对于这类任务，Rust 提供了数种 *循环（loops）*，所谓循环，是指会贯通执行循环体里头的代码到结束，并随后立即回到开头开始执行。首先构造一个名为 `loops` 的新项目，来进行这些循环的实验。

Rust 有着三种循环：`loop`、`while` 及 `for`。接下来就要各个进行尝试。

### 用 `loop` 关键字对代码进行重复

`loop` 关键字告诉 Rust 去一直一遍又一遍执行代码块，抑或直到显式地告诉他停下来为止。

作为示例，将 `loops` 目录中的 `src/main.rs` 文件修改为下面这样：

文件名：`src/main.rs`

```rust
fn main() {
    loop {
        println! (”再次！“);
    }
}
```

在运行这个程序时，就会看到一遍又一遍地持续打印出 `再次！`，知道手动停止这个程序为止。大多数终端程序，都支持键盘快捷键 `ctrl-c` 来中断某个卡在无尽循环中的某个程序。来尝试一下：

```console
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/loops`
再次！
再次！
再次！
再次！
^C再次！
```

其中的符号 `^C` 表示按下 `ctrl-c` 的地方。在那个 `^C` 之后，可能会也可能不会看到 `再次！` 被打印出来，取决于程序接收到中断信号时，代码在循环中的何处。

幸运的是，Rust 还提供了一种运用代码来跳出循环的方式。可在循环中放置 `break` 关键字，而告诉程序在何时结束执行这个循环。还记得在第 2 章的 [猜对数字后退出程序](Ch02_Programming_a_Guessing_Game.md#quitting-after-a-correct-guess) 小节，就在那个猜数游戏中这样做了，在通过猜到正确数字而赢得游戏时退出那个程序。

在那个猜数游戏中，还使用了 `continue` 关键字，循环中的 `continue` 关键字，告诉程序去跳过循环本次迭代的其余全部代码，而前往下一次迭代。

在有着循环里头的循环时，那么 `break` 与 `continue` 就会应用在他们所在点位处的最内层循环（if you have loops within loops, `break` and `continue` apply to the innermost loop at that point）。可选择在某个循环上指定一个 *循环标签（loop label）*，这样就可以与 `break` 或 `continue` 结合使用，来指明这些关键字是要应用到打上标签的循环，而不再是那最里层的循环了。下面就是一个有着两个嵌套循环的示例：

```rust
fn main() {
    let mut count = 0;

    'counting_up: loop {
        println! ("计数 = {}", count);
        let mut remaining = 10;

        loop {
            println! ("剩余 = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println! ("最终计数 = {}", count);
}
```

其中的外层循环有着标签 `'counting_up`，同时他将从 `0` 计数到 `2`。而其中的内层循环不带标签，会从 `10` 计数到 `9`。其中的第一个未指定标签的 `break` 语句，将只会退出那个内部循环。而那个 `break 'counting_up;` 语句，则会将外层循环退出。此代码会打印出：

```console
$ cargo run                                                                           INT ✘
   Compiling loops v0.1.0 (/home/peng/rust-lang/projects/loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.18s
     Running `target/debug/loops`
计数 = 0
剩余 = 10
剩余 = 9
计数 = 1
剩余 = 10
剩余 = 9
计数 = 2
剩余 = 10
最终计数 = 2
```

### 自循环返回值

**Returning Values from Loops**

`loop` 的一个用途，即是对一个明知会失败的操作进行重试，比如检查某个线程是否已完成他的作业。还可能需要将那个操作的结果，从循环传出来给代码的其余部分。要实现这一点，可将想要返回的值，添加在用于停止该循环的 `break` 表达式之后；那个值就会被返回到该循环的外面，进而就可以用到那个值了，如下所示：

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println! ("结果为：{}", result);
}
```

在这个循环之前，这里声明了一个名为 `counter` 的变量，并将其初始化为 `0`。随后声明了一个名为 `result` 变量，来保存从该循环所返回的值。在该循环的每次迭代上，是要给 `counter` 变量加上 `1` 的，并随后检查那个计数器是否等于 `10`。在计数器等于 `10` 的时候，就使用有着值 `counter * 2` 的 `break` 关键字。在该循环之后，使用了一个分号来结束将值 `counter * 2` 赋值给 `result` 的那个语句。最后，这里打印出了在 `result` 里的值，即这个示例中的 `20`。

### 使用 `while` 的条件循环

程序经常会对循环里的条件进行求值。当条件为真时，该循环就运行。在条件不再为真时，程序就调用 `break`，把循环停下来。使用 `loop`、`if`、`else` 与 `break` 来实现与此相似的行为，是可能的；若愿意这样做，现在就可以在程序中尝试一下。不过由于这种模式如此常见，以至于 Rust 为此而有了一个内建的语言结构，那就是叫做 `while` 的循环。在下面的清单 3-3 中，就使用了 `while` 来将该程序循环三次，每次都倒计数，并随后在循环结束之后，打印出一条消息而退出。

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println! ("{}!", number);

        number -= 1;
    }

    println! ("点火！！");
}
```

*清单 3-3：使用 `while` 循环在条件保持为真期间运行代码*

此代码结构，消除了使用 `loop`、`if`、`else`、及 `break` 实现同样结构时，很多不可缺少的嵌套，且此结构更为清晰。在条件保持为真期间，代码就会运行；否则，他将退出循环。


### <a id="looping-through-a-collection-with-for"></a> 使用 `for` 对集合进行遍历

可选择使用 `while` 结构，来对集合，诸如数组，的那些元素进行循环。作为示例，下面清单 3-4 中的循环，将打印出数组 `a` 中的各个元素。

文件名：`src/main.rs`

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    let mut index = 0;

    while index < a.len() {
        println! ("值为：{}", a[index]);

        index += 1;
    }
}
```

*清单 3-4：使用 `while` 循环遍历集合的各个元素*

这个程序里，代码会根据那个数组中的元素，往上计数。是以索引 `0` 开始，然后循环，直到循环到了数组中最后的那个索引（即，在 `index < 5` 不再为 `true` 时）。运行此代码将打印出数组中的所有元素：

```console
$ cargo run                                                                                  ✔
   Compiling loops v0.1.0 (/home/peng/rust-lang/projects/loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/loops`
值为：10
值为：20
值为：30
值为：40
值为：50
```

全部五个数组值都会出现在终端里，跟预期一样。尽管 `index` 在某个时间点达到值 `5`，但该循环会在尝试从那个数组获取第六个值之前，就停止执行。

但这种方法易于出错；在索引值或测试条件不正确时，就会导致该程序出错。比如，若把数组 `a` 的定义修改为有四个元素，而忘记了将那个条件更新到 `while index < 4`，此代码就会出错。由于编译器增加了在那个循环过程中，每次迭代上用于执行对 `index` 是否在数组边界内的，条件检查时间，因此这种方法还慢。

作为一种位为简练的替代，就可使用 `for` 循环而对集合中的各个元素，执行一些代码。`for` 循环看起来就跟下面清单 3-5 中的代码一样：

文件名：`src/main.rs`

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println! ("值为：{}", element);
    }
}
```

*清单 3-5：使用 `for` 循环对结合的各个元素进行遍历*


在运行这段代码时，将看到与清单 3-4 中同样的输出。更重要的是，现在业已提升了代码的安全性，并消除了可能因超出那个数组末端，或因索引未足够触及而遗失掉一些数组项目，而导致的代码错误。

使用这个 `for` 循环，在更改了那个数组中值的个数时，就无需记得，像清单 3-4 中所使用的方式那样，去修改任何其他代码。

`for` 循环的安全与简洁，使得他们成为了 Rust 中最常用的循环结构。即使在那种要以确切次数来运行某些代码的情形下，如同清单 3-3 中用到 `while` 循环的倒计时示例，大多数 Rust 公民也将会使用 `for` 循环。要以确切次数运行某些代码，则要用到由标准库提供的 `Range` 特性了，`Range` 会依序生成自某个数字开始，并在另一数字之前结束，其间的全部数字来。

下面就是使用 `for` 循环，和另一个至今还未讲到的、用于逆转那个范围的 `rev` 方法，来实现那个倒计时的样子：

文件名：`src/main.rs`

```rust
fn main() {
    for number in (1..4).rev() {
        println! ("{}!", number);
    }

    println! ("发射！！");
}
```

此代码要更好一些，不是吗？


## 总结

咱们做到了！这第 3 章内容可真不少：在这里掌握了变量、标量与复合数据类型、函数、代码注释、`if`表达式，还有循环！请构建一些程序来完成下面这些事情，从而练习一下本章所讨论的那些概念：

- 对法式温度和摄氏温度之间互相转换；
- 生成第 n 个斐波拉基数；
- 利用圣诞颂歌 “The Twelve Days of Christmas” 中的重复，而打印出这首颂歌的歌词来；

在做好了继续新内容的学习后，就将要讨论到 Rust 中的一个在其他编程语言中并不多见的概念：所有权（ownership）。

## 练习答案


<details>
    <summary>“法式温度与摄氏温度的转换”</summary>

```rust
use std::io;
use std::process;

fn fah_to_cels(f: f32) -> f32 {
    return (f - 32.0) / 1.8;
}

fn cels_to_fah(c: f32) -> f32 {
    return c * 1.8 + 32.0;
}

fn main() {
    println! ("法式温度与摄氏温度之间的转换");

    loop {
        println! ("\n-----------------\n请选择：
            '1'-摄氏温度/'2'-法式温度/'Q'/\"quit\" 退出程序。
            '1'/'2'/'Q'/\"quit\"[1]：");

        let mut temp_type = String::new();

        io::stdin()
            .read_line(&mut temp_type)
            .expect("读取输入失败！");

        let temp_type = temp_type.trim();

        if temp_type.eq("Q") || temp_type.eq("quit") { process::exit(0); }

        if ! temp_type.eq("1") && ! temp_type.eq("2") && ! temp_type.eq("") {
            println! ("无效输入，请输入 '1'、'2'、'Q'、\"quit\"，或直接按下回车键");
            continue;
        }

        if temp_type.eq("1") || temp_type.eq("") {
            println! ("请输入要转换的摄氏温度：");
            let temp = get_temp_input();

            println! ("摄氏温度： {:.2}°C，约为法氏温度：{:.2}°F", temp, cels_to_fah(temp));
        }

        if temp_type.eq("2") {
            println! ("请输入要转换的法氏温度：");
            let temp = get_temp_input();

            println! ("法氏温度：{:.2}°F，约为摄氏温度：{:.2}°C", temp, fah_to_cels(temp));
        }
    }
}

fn get_temp_input() -> f32 {
    return loop {
        let mut tmp = String::new();

        io::stdin()
            .read_line(&mut tmp)
            .expect("读取输入失败");

        match tmp.trim().parse() {
            Ok(num) => { break num },
            Err(_) => {
                println! ("请输入一个浮点数，比如 -10.0, 15.6");
                continue
            }
        };
    };
}
```

</details>


<details>
    <summary>"生成第 n 个斐波拉基数"</summary>


```rust
use std::io;
use num_format::{Locale, ToFormattedString};
// use std::process;

fn nth_fibonacci(n: u64) -> u64 {

    if n == 0 || n == 1 {
        return n;
    } else {
        return nth_fibonacci(n - 1) + nth_fibonacci(n - 2);
    }
}

fn main() {
    println! ("找出第 n 个斐波拉基数");

    'main_loop: loop {
        println! ("请输入 n: （Q/quit 退出程序）");

        let n: u64 = loop {
            let mut tmp = String::new();

            io::stdin()
                .read_line(&mut tmp)
                .expect("读取输入失败！");

            let tmp = tmp.trim();

            if tmp.eq("Q") || tmp.eq("quit") {
                // process::exit(0);
                break 'main_loop;
            }

            match tmp.parse() {
                Ok(num) => { break num },
                Err(_) => {
                    println! ("请输入一个正整数！\n");
                    continue;
                },
            };
        };

        println! ("第 {} 个斐波拉基数为：{}",
            n,
            nth_fibonacci(n).to_formatted_string(&Locale::en));
    }
}
```

</details>


<details>
    <summary>"打印圣诞颂歌 ‘The Twelve Days of Christmas’ 歌词"</summary>

```rust
fn main() {
    let days = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "nineth",
        "tenth",
        "eleventh",
        "twelfth"
    ];
    let amounts = [
        "A",
        "Two",
        "Three",
        "Four",
        "Five",
        "Six",
        "Seven",
        "Eight",
        "Nine",
        "Ten",
        "Eleven",
        "Twelve"
    ];
    let things = [
        "partridge in a pear tree",
        "turtle doves",
        "French hens",
        "calling birds",
        "golden rings",
        "geese-a-laying",
        "swans-a-swimming",
        "maids-a-milking",
        "ladies dancing",
        "lords-a-leaping",
        "pipers piping",
        "drummers drumming",
    ];

    for num in 1..=12 {
        println! ("\nOn the {} day of Christmas,\nMy true love gave to me:",
            days[num-1]);
        for tmp in (0..num).rev() {
            if tmp == 0 && num == 1 {
                println! ("{} {}.", amounts[tmp], things[tmp]);
            }
            if tmp == 0 && num != 1 {
                println! ("And {} {}.", amounts[tmp].to_lowercase(), things[tmp]);
            }
            if tmp != 0 {
                println! ("{} {},", amounts[tmp], things[tmp]);
            }
        }
    }
}
```

</details>
