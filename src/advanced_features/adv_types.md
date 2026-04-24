# 高级类型

Rust 类型系统有着一些我们到目前为止提到过，但尚未讨论的特性。我们将

- 首先概述新型模式，并讨论他们作为类型为何有用；
- 然后，我们将继续讨论类型别名，一项与新型模式类似的特性，但语义略有不同；
- 我们还将讨论 `!` 类型，以及动态长度的类型。


## 类型安全与新型模式下的抽象


这一小节假设咱们已经阅读了前面小节 [通过新型模式实现外部特质](./adv_traits.md#通过新型模式实现外部特质) 。新型模式对于我们迄今为止讨论过的任务也很有用，包括静态地强制值永不混淆，以及标明值的单位。在 [清单 20-16](./adv_traits.md#listing_20-16) 中，咱们已经看到使用新型表示单位的示例：回顾一下，`Millimeters` 与 `Meters` 两个结构体都将 `u32` 值封装在新型中。若我们编写了一个带有 `Millimeters` 类型参数的函数，我们将无法编译一个意外尝试以类型 `Meters` 或普通 `u32` 值调用该函数的程序。

我们还可以使用新型模式来抽象出类型的一些实现细节：新类型可以暴露一个不同于私有的内层类型 API 的公开 API。

新型模式还可以隐藏内部实现。例如，我们可提供 `People` 类型来封装 `HashMap<i32, String>`，该 `HashMap` 存储与姓名关联的人员 ID。使用 `People` 的代码只会与我们提供的公开 API 交互，比如添加姓名字符串到 `People` 集合的方法；该代码无需知道我们在内部分配 `i32` 的 ID 给姓名。新型模式属于一种实现封装以隐藏实现细节的轻量级方法，我们在第 18 章中 [隐藏实现细节的封装](../oop/characteristics_oop.md#隐藏实现细节的封装) 小节中曾对此讨论过。


## 类型同义词和类型别名

Rust 提供了声明 *类型别名* 的能力，以便给予现有类型另一个名字。为此，我们使用 `type` 关键字。例如，我们可以像下面这样创建 `i32` 的别名 `Kilometers`：

```rust
type Kilometers = i32;
```

现在，别名 `Kilometers` 是 `i32` 的 *同义词*；与我们在 [清单 20-16](./adv_traits.md#listing_20-16) 中创建的 `Millimeters` 与 `Meters` 两个类型不同，`Kilometers` 并不是个单独的新类型。类型为 `Kilometers` 的值将被视为与类型 `i32` 的值相同：

文件名：`projects/type_aliases/src/main.rs`

```rust
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    assert_eq! (x, y);
```

由于 `Kilometers` 与 `i32` 属于同一类型，我们可以将这两种类型的值相加，并且可以传递 `Kilometers` 的值给取 `i32` 参数的函数。但是，使用这种方法，我们无法获得前面讨论过的新型模式中的类型检查优势。换句话说，当我们在某处混用了 `Kilometers` 与 `i32` 的值时，编译器也不会给予我们报错。

类型同义词的主要用例是减少重复。例如，我们可能遇到如下这种冗长的类型：

```rust
Box<dyn Fn() + Send + 'static>
```

在函数签名中，以及作为代码各处的类型注解，写下这种冗长的类型，可能会很烦人且容易出错。设想一下，当某个项目全是像下面清单 20-25 中的代码。

<a name="listing_20-25"></a>
```rust
    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println! ("嗨"));

    fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
        // --跳过代码--
    }

    fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
        // --跳过代码--
    }
```

**清单 20-25**：在多处使用冗长类型

类型别名通过减少重复，使这段代码更易于管理。在下面清单 20-26 中，我们针对该冗长类型引入了一个名为 `Thunk` 的别名，并可以用这个更短的别名 `Thunk`替换该类型的所有使用。

<a name="listing_20-26"></a>
```rust
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println! ("嗨"));

    fn takes_long_type(f: Thunk) {
        // --跳过代码--
    }

    fn returns_long_type() -> Thunk {
        // --跳过代码--
    }
```

**清单 20-26**：引入类型别名 `Thunk`，以减少重复

这段代码现在更容易月的和编写！为类型别名选择一个有意义的名字，也可以帮助传达咱们的意图（ *thunk* 是指将在稍后执行的代码，因此对于一个被存储的闭包来说，这是个恰当的名字）。

类型别名也经常与 `Result<T, E>` 一起使用，以减少重复。设想标准库中的 `std::io` 模组。I/O 操作通常返回 `Result<T, E>`，来处理操作失败的情况。这个库有个 `std::io::Error` 结构体，表示所有可能的 I/O 错误的。`std::io` 中的许多函数都会返回 `Result<T, E>`，其中 `E` 就是 `std::io::Error`，比如 `Write` 特质中的这些函数：

```rust
use std::fmt;
use std::io::Error;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn flush(&mut self) -> Result<(), Error>;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
}
```

其中 `Result<..., Error>` 多次重复。因此，`std::io` 便包含了以下的类型别名声明：

```rust
type Result<T> = std::result::Result<T, std::io::Error>;
```

由于这一声明位于 `std::io` 模组中，我们可以使用完全限定的别名 `std::io::Result<T>`；也就是说，这是个 `Result<T, E>`，其中 `E` 被填入为 `std::io::Error`。`Write` 特质的函数签名最终看起来像下面这样：

```rust
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}
```

类型别名有两种作用：他使代码更容易编写，*并* 给予我们对整个 `std::io` 的一致接口。由于他是个别名，因此他只是另一个 `Result<T, E>`，这意味着我们可以时与哦嗯任何适用于 `Result<T, E>` 的方法，以及诸如 `?` 运算符这样的特殊语法。


## 永不返回的 `never` 类型

Rust 有一种名为 `!` 的特殊类型，在类型论术语种称为 *空类型*，因为他没有值。我们更倾向于称其为 *`never` 类型*，因为当函数永远不返回时，他会代替返回类型。下面是个示例：

```rust
fn bar() -> ! {
    // --跳过代码--
}
```

这段代码读作 “函数 `bar` 返回 `never`。” 返回 `never` 函数被称为 *发散函数*。我们无法创建 `!` 类型的值，因此 `bar` 永远不可能返回。

但是，一种咱们永远无法针对其创建值的类型有什么用处呢？回顾 [清单 2-5](../Ch02_Programming_a_Guessing_Game.md#listing_2-5) 中的代码，那时猜数游戏的一部分；我们已在下面清单 20-27 中重现了其中一部分。

<a name="listing_20-27"></a>
```rust
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
```

**清单 20-27**：带有一个以 `continue` 结束的支臂的 `match` 表达式

当时，我们跳过了这段代码种的一些细节。在第 6 章中 [`match` 控制流结构](../enums_and_pattern_matching/match_control_flow.md) 小节中，我们讨论了 `match` 支臂必须返回相同的类型。因此，例如，以下代码不会正常运行：

```rust
    let guess = match guess.trim().parse() {
        Ok(_) => 5,
        Err(_) => "你好",
    }
```

这段代码中的 `guess` 类型将必定是整数和字符串，而 Rust 要求 `guess` 只能有一种类型。那么，`continue` 返回什么呢？在清单 20-27 中，我们为何被允许在一个支臂中返回 `i32`，并让另一个支臂以 `continue` 结束呢？

正如您可能已经猜到的，`continue` 有个 `!` 的值。也就是说，当 Rust 计算 `guess` 的类型时，他会同时查看两个匹配支臂，前者的值为 `u32`，后者的值为 `!` 值。由于 `!` 永远不会有值，Rust 因此确定 `guess` 的类型是 `u32`。

描述这种行为的正式说法是，类型为 `!` 的表达式可以被强制转换为任何其他类型。我们允许用以 `continue` 结束这个匹配支臂，因为 `continue` 不会返回值；相反，他会迁移控制权到该循环的顶部，因此在 `Err` 情形下下，我们永远不会指派值给 `guess`。

`never` 类型对 `panic! 宏` 也很有用。回顾我们对 `Option<T>` 值调用 `unwrap` 函数，以通过以下定义生成一个值或终止运行：

```rust
impl<T> Option<T> {
    pub fn unwrap(self) -> {
        match self {
            Some(val) => val,
            None => panic! ("called `Option::unwrap()` on a `None` value"),
        }
    }
}
```

在这段代码中，发生了与清单 20-27 中 `match` 表达式中相同的事情：Rust 发现 `val` 具有类型 `T`，而 `panic!` 有着类型 `!`，因此整个 `match` 表达式的结果是 `T`。这段代码之所以能正常运行，是因为 `panic!` 不会产生值；他会终止程序。在 `None` 情形下，我们不会从 `unwrap` 返回值，因此这段代码是有效的。

最后一个有着 `!` 类型的表达式是 `loop` 表达式：

文件名：`projects/never_type/src/main.rs`

```rust
    print! ("永永 ");

    loop {
        print! ("远远 ");
    }
```

在这里，循环永远不会结束，因此 `!` 就是该表达式的值。然后，若我们包含一个 `break`，情况就不同了，因为循环在遇到 `break` 时就会终止。

> **译注**：包含一个 `break` 时，该表达式的值为单元值 `()`。相关代码和打印输出如下。
>
> 文件名：`projects/never_type/src/main.rs`
>
> ```rust
>     print! ("永永 ");
>
>     let res = loop {
>         print! ("远远 ");
>         break
>     };
>
>     println! ("{res:?}");
> ```
>
> 打印输出：`永永 远远 ()`

## 动态大小类型与 `Sized` 特质

Rust 需要知道其类型的某些详细信息，比如为特定类型的值分配多少空间。这使得其类型系统的一个方面乍看之下有些令人困惑：*动态大小类型，dynamically sized types* 的概念。这些类型有时又被称为 DSTs 或 *未知大小类型，unsized types*，他们允许我们编写出，使用只有在运行时才知道其长度的值的代码。

我们来深入探讨一下名为 `str` 的动态长度类型，我们在这整本书一直在使用他。没错，不是 `&str`，而是 `str` 本身就属于 DST。在许多情况下，比如存储用户输入的文本时，我们无法在运行时之前知道字符串的长度。这意味着我们既不能创建 `str` 类型的变量，也不能取 `str` 类型的参数。请考虑以下代码，他无法正常运行：

```rust
    let s1: str = "致以问候！";
    let s2: str = "最近过得怎么样？";
```

Rust 需要知道为特定类型的任何值分配多少内存，且一种类型的所有值都必须使用同样数量的内存。若 Rust 允许我们编写这段代码，那么这两个 `str` 值将需要占用相同数量的空间。但他们有着不同长度：`s1` 需要 15 字节的存储，而 `s2` 需要 `24` 字节。这就是为什么无法创建保存动态长度类型值的变量的原因。

那么，我们该怎么办？在这种情况下，咱们已经知道答案了：我们构造 `s1` 和 `s2` 的类型为 `&str`，而不是 `str`。回顾第 4 章中 [字符串切片](../ownership/the_slice_type.md#字符串切片) 小节，切片数据结构仅存储切片的起始位置和长度。因此，尽管 `&T` 属于存储 `T` 所在内存地址的单个值，但字符串切片则是 *两个* 值：`str` 的地址与其长度。正因如此，我们可以在编译时知道字符串切片值的长度： 他是 `uszie` 长度的两倍。也就是说，我们始终知道字符串切片的长度，无论他引用的字符串有多长。一般来说，Rust 中动态长度类型的使用方式是这样的：他们有着额外的元数据，存储动态信息的长度。动态长度类型的黄金法则是，我们必须始终将动态长度类型的值放在某种指针之后。

我们可以将 `str` 与各种类别的指针结合：例如 `Box<str>` 或 `Rc<str>`。事实上，咱们之前已经见过这种情况，只不过以一种不同的动态长度类型：那便是特质。每个特质都属于动态长度类型，我们可以使用特质名字来引用他。在第 18 章中 [使用特质来抽象共用行为](../oop/trait_objects.md) 小节中，我们提到要将特质作为特质对象使用，就必须将其置于指针之后，比如 `&dyn Trait` 或 `Box<dyn Trait>` （`Rc<dyn Trait>` 也可以）。

为了使用 DST，Rust 提供了 `Sized` 特质，来确定某种类型的长度是否在编译时已知。对于所有在编译时大小已知的类型，都会自动实现该特质。此外，Rust 会隐式地将一个 `Sized` 的边界添加到每个泛型函数。也就是说，像下面这样的一个泛型函数：

```rust
fn generic<T>(t: T) {
    // --跳过代码--
}
```

实际上会被视为我们写了这样一段代码：

```rust
fn generic<T: Sized>(t: T) {
    // --跳过代码--
}
```

默认情况下，泛型函数仅适用于编译时已知大小的类型。但是，咱们可以使用以下特殊语法来放宽这一限制：

```rust
fn generic<T: ?Sized>(t: &T) {
    // --跳过代码--
}
```

`?Sized` 的特质边界，意味着 “`T` 可能属于 `Sized` 类型，也可能不属于”，这种写法覆盖了 “泛型类型必须在编译时具有已知大小” 的默认规则。具有这种含义的 `?Trait` 语法，仅适用于 `Sized`，不适用于其他任何特质。

另请注意，我们将参数 `t` 的类型从 `T` 改为 `&T`。由于该类型可能不是 `Sized`，我们需要在某种指针之后使用他。在这种情况下，我们选择了引用。

接下来，咱们将讨论函数和闭包！


（End）


