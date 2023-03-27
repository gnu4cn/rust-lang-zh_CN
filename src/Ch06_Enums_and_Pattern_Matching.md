# 枚举与模式匹配

**Enums and Pattern Matching**

在本章，将会对 *枚举（enumerations）* 进行审视，枚举也被当作 *enums*。枚举实现了通过列举出类型可能的 *变种（variants）*，来定义出一个类型。这里会首先定义并使用一个枚举，来展示枚举能如何将意义和数据编码起来。接下来，就会探索一个特别有用、名为 `Option` 的枚举，该枚举表示了某个值既可以是某事物，也可以什么也不是。随后就会看看在 `match` 表达式中的模式匹配，是怎样令到根据枚举中不同的值，而运行各异的代码容易起来的。最后，将会讲到 `if let` 结构是怎样成为另一种处理代码中枚举值的、便利而简洁的习惯用法的。

许多语言都有枚举这一特性，不过在各个语言中的枚举能力是不同的。Rust 的枚举与那些函数式语言，诸如 F#、OCaml 及 Haskell 等中的 *代数数据类型（algebraic data types）* 最为相似。

## 定义一个枚举

枚举是不同于结构体的第二种定义定制数据类型的方式。下面就来看看一种在代码中可能表达的情形，并见识一下为何在此情形下，相比于结构体，枚举是有用且更恰当的。假设说这里需要对 IP 地址进行处理。目前仅有两种用于 IP 地址的标准：版本四和版本六。由于这两个标准是程序将遇到的 IP 地址仅有的可能性，因此就可以 *列举出（enumerate）* 全部可能的变种，这正是枚举（enumeration） 名字得来之处。

任何 IP 地址都只能是版本四或版本六的地址，而不会同时两个都是。由于枚举值只能是枚举变种之一，那么 IP 地址的这个属性，令到枚举数据结构（the enum data structure）恰当起来。而版本四和版本六两种地址，从根本上说都是 IP 地址，那么在代码对适用于任意类别 IP 地址的情形加以处理时，版本四和版本六地址都应当作同一类型对待。

在代码中，可通过定义一个 `IpAddrKind` 枚举，并列出 IP 地址可能的类别，即 `V4` 和 `V6`，来表达这个概念。下面就是该枚举的变种：

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

现在 `IpAddrKind` 就是一个可在代码中别的地方使用的定制数据类型了。


### <a id="enum-values"></a>枚举取值

可像下面这样，创建出 `IpAddrKind` 两个变种的实例来：

```rust
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
```

请注意，该枚举的两个变种，是在其标识符的命名空间之下的，且这里使用了双冒号将标识符和变种分隔开。由于现在这两个值 `IpAddrKind::V4` 与 `IpAddrKind::V6` 都是这同一类型：`IpAddrKind`，因此这就变得有用了。随后就可以，比如，定义一个取任意 `IpAddrKind` 类型值的函数：

```rust
fn route(ip_kind: IpAddrKind) {}
```

进而就能以这两个变种对这个函数进行调用了：

```rust
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
```

枚举的使用甚至还有更多好处。在还没有一种存储具体 IP 地址 *数据（data）* 的时候，就要进一步思考一下这里的 IP 地址类型；这是只知道 IP 地址数据为什么 *类别（king）*。根据在第 5 章中掌握的结构体知识，那么可能很想用下面清单 6-1 中的结构体来解决这个问题。

```rust
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}
```

*清单 6-1：使用结构体 `struct` 来存储 IP 地址的数据与 `IpAddrKind` 变种*

这里已定义了有着两个字段的结构体 `IpAddr`：一个类型为 `IpAddrKind` （即先前定义的那个枚举）的 `kind` 字段，以及一个类型为 `String` 的 `address` 字段。这里有该结构体的两个实例。第一个是 `home`，而他有着与地址数据 `127.0.0.1` 关联的 `IpAddrKind::V4` 作为其 `kind` 的值。第二个实例为 `loopback`。这个实例则有不同的 `IpAddrKind` 变种作为其 `kind` 的值，即 `V6`，与 `kind` 关联的是地址 `::1`。由于这里使用了结构体将 `kind`  与 `address` 值捆绑在一起，因此现在这个 `IpAddrKind` 的变种就与那个 `String` 值关联起来了。

不过，仅使用一个枚举来表示这同一概念，就会更加简练：与其将枚举放在结构体内部，可将数据直接放在各个枚举变种里头。那么这新的 `IpAddr` 枚举定义，就是说 `V4` 与 `V6` 两个变种，将同时有着关联的 `String` 值：

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
}
```

这里把数据直接附加到枚举的各个变种上，因此就无需额外的结构体了。这里还更易于发现枚举工作原理的另一细节：所定义的各个枚举变种的名字，还成为了构造该枚举实例的函数。那就是说，`IpAddr::V4()` 现在是个取 `String` 参数并返回该 `IpAddr` 类型实例的函数调用了。作为定义枚举的结果，这里让这个构造函数自动就定义好了。

这里还有另一个使用枚举而非结构体的好处：各个变种可以有不同类型及数量的关联数据。版本四类型的 IP 地址，将始终有着四个会有着 `0` 到 `255` 之间值的数字部分。在希望将 `V4` 地址存储为四个 `u8` 值，而仍然将 `V6` 地址表示为一个 `String` 值时，那就没法用结构体了，而枚举则能轻易处理这样的情况：

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
}
```

到这里，就已经给出了好几种定义用于存储版本四和版本六 IP 地址的数据结构了。然而事实表明，想要存储 IP 地址，及对这些 IP 地址所属类别进行编码是如此普遍，以致 [标准库就有一个可加以使用的定义](https://doc.rust-lang.org/std/net/enum.IpAddr.html)！下面就来看看，标准库是怎样定义 `IpAddr` 的：他有着与这里曾定义和使用过的相同枚举和变种，不过标准库是将地址数据，以两个不同结构体的形式，嵌入到变种里的，对两个枚举变种，定义了不同的结构体。

```rust
struct Ipv4Addr {
    // --跳过--
}

struct Ipv4Addr {
    // --跳过--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

这段代码说明可将任何类别的数据放在枚举变种里面：比如字符串、数字类型，或结构体等等。甚至可以包含另一枚举！还说明了，标准库类型，通常也并不比咱们自己编写的代码复杂多少。

请注意，由于这里不曾将标准库的 `IpAddr` 定义带入到这里的作用域，因此即使标准库包含了一个 `IpAddr` 的定义，这里也仍然可以毫无冲突地创建与使用自己的 `IpAddr` 定义。在第 7 章就会讲到有关带入类型到作用域的问题。

来看看下面清单 6-2 中另一个枚举的示例：这个枚举有着嵌入到其各个变种中的种类繁多的类型。

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write (String),
    ChangeColor(i32, i32, i32),
}
```

*清单 6-2：每个变种都存储了不同数量和类型值的 `Message` 枚举*

这个枚举有着四个带有不同类型数据的变种：

- `Quit` 变种完全没有与其关联的数据；
- `Move` 变种像结构体一样，有着两个命名的字段；
- `Write` 变种包含了单个 `String`；
- `ChangeColor` 编程包含了三个 `i32` 的值。

定义一个有着一些如上面清单 6-2 中变种的枚举，与定义不同种类的结构体定义类似，不同在于枚举未使用关键字 `struct`，且所有变种在 `Message` 类型下组织在了一起。下面这些结构体，就可保存之前各个枚举变种所保存的那些同样数据：

```rust
struct QuitMessage; // 单元结构体
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String);    // 元组结构体
struct ChangeColorMessage(i32, i32, i32);   //  元组结构体
```

不过假如这里使用了不同的、有着各自类型的结构体，那么就无法轻易地定义出一个接收原本在清单 6-2 中定义的、单一类型的  `Message` 枚举那样的，接收全部这些类别消息的函数了。

枚举与结构体之间，还有另外一个相似点：正如在结构体上使用 `impl` 关键字定义出一些方法，在枚举上定义方法也是可以的。下面就是一个可定义在这里的 `Message` 枚举上、名为 `call` 的方法：

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write (String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // 方法体将定义在这里
    }
}

fn main() {

    let m = Message::Write(String::from("hello"));
    m.call();
}
```

方法体将使用 `self`，来获取方法调用所在变种实例值。在此示例中，已创建了一个有着值 `Message::Write(String::from("hello"))` 的变量 `m`，而那就是在 `m.call()` 运行时，`call` 方法体中的那个 `self`。

下面就来看看标准库中另一个甚为常见和有用的枚举：`Option`。


### `Option` 枚举及其超越空值的诸多优点

**The `Option` Enum and Its Advantages Over Null Values**

本小节会探讨 `Option` 的案例研究，`Option` 是由标准库定义的另一个枚举。`Option` 类型编码了某个值可能会是某个物件，或可能什么都不属于的这种甚为常见的场景（the `Option` type encodes the very common scenario in which a value could be something or it could be nothing）。比如在请求某个含有一些项目的清单的第一个项目时，就会得到一个值。而在请求某个空清单的第一个项目时，则会什么也得不到。以类型系统字眼，来表达这个概念，就表示编译器能够对，是否已处理了本应处理的全部情形，进行检查；此项功能可阻止那些在其他编程语言中极为常见的代码错误。

编程语言的设计，通常要考量包含哪些特性，而要排除哪些特性也至关重要。Rust 没有许多其他语言都有的空值特性。*空值（Null）* 是一个表示此处无值的值。在带有 `null` 的那些语言中，变量总是会处于下面两个状态之一：空值或非空值。

在 `null` 的发明人Tony Hoare 于 2009 年的演讲 “空值引用：10 亿美金代价失误（Null Reference: The Billion Dollar Mistake）” 中，就讲了这个问题：

> 我把他叫做我的10亿美元失误。那个时候我正在设计某门面向对象语言中的首个综合类型系统。目的是要在编译器自动执行的检查之下，确保全部引用使用，都应绝对安全。仅仅因为空值引用变量实现起来很容易，我当时就没能顶住诱惑，把他加入到特性集了。这个举动，业已造成了数不胜数的代码错误、漏洞及系统崩溃等等问题，在过去 40 余年里，这些问题可能已经造成大概 10 亿美金的痛苦和伤害。

`null` 值的问题在于，当尝试将 `null` 值用作非 `null` 值时，就会得到某种错误。由于这种 `null` 或非 `null` 的属性遍布各处，因此极容易犯下此类错误。

但 `null` 试图表达的概念，还是有用的：`null` 是个因某些原因，而当前为无效或空缺的值。

问题不是真的在于这个概念，而在于针对性的实现。由于这些原因，Rust 就没有空值，但他确实有一个可对值存在或空缺这个概念，进行编码的枚举。这个枚举就是 `Option<T>`，而这个枚举 [由标准库定义](https://doc.rust-lang.org/std/option/enum.Option.html) 为下面这样：

```rust
enum Option<T> {
    None,
    Some(T),
}
```

这个 `Option<T>` 是如此重要，以至于在 Rust 序曲（the prelude）中甚至都包含了；是不需要显式地将其带入到作用域的（注：*原生类型、这里的 `Option<T>`，以及前面的 `String` 类型等等，就是这样的包含在序曲中的类型，无需显式地带入到作用域，就可以直接使用*）。该枚举的变种也已包含在 Rust 序曲中：可直接在不带前缀 `Option::` 的情况下直接使用 `Some` 与 `None`（注：*那么 `Some` 与 `None` 就被列为了 Rust 关键字了*）。`Option<T>` 仍然只是常规枚举，而 `Some<T>` 与 `None` 仍然是类型 `Option<T>` 的变种。

这里的 `<T>` 语法，是个到目前为止还未讲到的 Rust 特性。他是个泛型参数，而在第 10 章将更详细的涉及到泛型。至于现在，只需明白 `<T>` 表示 `Option` 枚举的 `Some` 变种，可保存任意类型的一条数据，而在 `T` 位置处用到的各个具体类型，会让整个 `Option<T>` 类型成为各异的类型（for now, all you need to know is that `<T>` means the `Some` variant of the `Option` enum can hold one piece of data of any type, and that each concrete type that gets used in place of `T` makes the overall `Option<T>` type a different type）。以下是使用 `Option` 来保存数字与字符串类型的一些示例：

```rust
    let some_numer = Some(5);
    let some_string = Some("一个字符串");

    let absent_number: Option<i32> = None;
```

`some_number` 的类型为 `Option<i32>`。`some_string` 的类型为 `Option<&str>`，是个不同的类型。由于这里已在 `Some` 变种里面指定了值，因此 Rust 可推导出这些类型来。而对于 `absent_number`，Rust 就要求注释整个 `Option` 类型：编译器无法通过仅查看一个 `None` 值，而推导出相应的 `Some` 变种的类型来。这里告诉了 Rust，这里计划的是 `absent_number` 为类型 `Option<i32>`。

在有着一个 `Some` 值时，就知道存在着一个值，且该值是保存在 `Some` 内部的。而在有个 `None` 值时，某种意义上讲，这表示了与空值同样的情况：没有一个有效值。那么究竟为什么有着 `Option<T>` 就是要比有着空值 `null` 好呢？

简而言之，由于 `Option<T>` 和 `T` (其中的 `T` 可以是任意类型) 为不同类型，因此编译器就不会允许将一个 `Option<T>` 值，当作一个必然的有效值来使用。比如，由于下面这段代码是在尝试将一个 `i8` 值，添加到某个 `Option<i8>` 上，因此这段代码不会编译：

```rust
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;
```

在运行这段代码时，就会得到下面这样的错误消息：

```console
$ cargo run                                                                                     ✔
   Compiling enum_demo v0.1.0 (/home/peng/rust-lang/projects/enum_demo)
error[E0277]: cannot add `Option<i8>` to `i8`
  --> src/main.rs:24:17
   |
24 |     let sum = x + y;
   |                 ^ no implementation for `i8 + Option<i8>`
   |
   = help: the trait `Add<Option<i8>>` is not implemented for `i8`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `enum_demo` due to previous error
```

强悍！事实上，这个错误消息表示，由于 `i8` 与 `Option<i8>` 属于不同类型，因此 Rust 不知道怎样将一个 `i8` 值与一个 `Option<i8>` 值相加。当在 Rust 中有着一个类型好比 `i8` 这样的值时，编译器就会保证始终有个有效值。在对那个值进行使用前，可不必检查他是不是 `null`，而可放心地加以处理。仅当有个 `Option<i8>` 类型（或其他任何正在使用的`Option<T>` 枚举类型值）的变量时，才真地必须关心可能并无值，同时编译器将确保在使用该值前，显式地处理无值的情况。

也就是说，在对 `Option<T` 执行类型参数 `T` 的那些操作前，必须将 `Option<T>` 类型转换为 `T` 类型。通常，这样做有助于捕获到 `null` 最常见问题之一：在某个东西实际上是 `null` 时，错误地将其设想为了他不是 `null`。

这种消除了不正确的假定某个非 `null` 值的做法，有助于增强代码自信。为了使用一个可能为 `null` 的值，就必须显式地通过将那个值的构造为 `Option<T>`，来带入这个值。在某个类型不为 `Option<T>` 值出现的每个地方，就都可以假定该值不是 `null`。这是 Rust 有意的设计决定，用以限制 `null` 的无处不在，及提升 Rust 代码的安全性。

那么在有一个类型为 `Option<T>` 值的时候，该怎么从 `Some` 变种获取到 `T` 这个值，从而就可以用上那个值呢？枚举 `Option<T>` 有着大量的、在不同情形下有用的方法；在 [`Option<T>` 文档](https://doc.rust-lang.org/std/option/enum.Option.html) 便可查看到这些方法。熟悉 `Option<T>` 上的这些方法，将对 Rust 编程生涯极为有用。

总的来说，为了使用某个 `Option<T>` 值，就要有将会处理各个变种的代码。要有一些只会在有着一个 `Some<T>` 的值时运行的代码，而此情况下就会允许这代码使用那个内部的 `T` 类型变量。在有着 `None` 值时，则还要有别的代码来允许了，而这代码就没有可用的 `T` 类型值了。在与枚举一起使用的时候，`match` 表达式正是实现此特性的控制流结构：`match` 表达式将依据枚举有着哪些变种，而运行相应的不同代码，以及哪些代码可使用匹配值内部的数据。

## <a id="the-match-control-flow-construct"></a>`match` 控制流结构

Rust 有值一种即为强大的、名为 `match` 的控制流结构，此控制流结构实现了将某个值与一系列模式的比较，并根据所匹配模式而执行相应的代码。模式可由字面值、变量名字、通配符及其他事物等构成；第 18 章会涵盖到全部不同种类的模式及其所完成的事情。`match` 的强大来自模式的表达能力，以及编译器对全部可能情形都被处理进行确认这一事实。

请将 `match` 表达式设想为一种类似硬币分选机这样的东西：硬币随一个滑道滚下，沿着这滑道有不同尺寸的洞，那么每个硬币都会在他碰到的第一个大小合适的洞那里掉落。同样道理，所有值都会历经 `match` 表达式中的各个模式，而在值 “适合” 的第一个模式处，那个值就会掉入到相关代码块，而在执行过程中被使用到。既然讲到了硬币，那么下面就来将其用作一个用到 `match` 表达式的示例！这里可以编写一个接收未知硬币，并以与点数机类似方式，判断出该硬币是何硬币而返回以分计的值来的函数，如下面清单 6-3 中所示。

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

*清单 6-3：一个枚举与一个将该枚举的那些变种作为其模式的 `match` 表达式*

这里就来把在 `value_in_cents` 函数中的那个 `match` 拆开讲讲。首先，这里是将 `match` 关键字后面跟上一个表达式，这里也就是 `coin` 后，列出来的。这就跟与 `if` 关键字一起使用的表达式极为相似，然而有个大的区别：在 `if` 之下，表达式需要返回一个布尔值，而在这里，该表达式可返回任意类型。此示例中 `coin` 的类型，即是这里在第一行上所定义的枚举 `Coin`。

接下来就是这个 `match` 表达式的各个支臂了。一个支臂有着两个部分：一个模式与一些代码。这里的第一个支臂，有着值为 `Coin::Penny` 的模式，同时其中的箭头运算符 `=>` 将模式与要运行的代码分隔开来。此情形下的代码，就只是值 `1`。各个支臂则是以逗号接着分开的。

在这个 `match` 表达式执行时，他就会依序将结果值与各个支臂的模式加以比较。在某个模式与该值匹配时，与那个模式关联的代码就被执行。而在那个模式不与该值匹配时，执行就会继续到下一支臂，就跟硬币分选机是一样的。这里需要多少支臂，就可以有多少支臂：在清单 6-3 中的 `match` 表达式，就有四个支臂。

与各个支臂关联的代码，是个表达式，而在匹配支臂中的表达式返回值，就是整个 `match` 表达式所返回的值。

正如清单 6-3 中，每个支臂只是返回一个值那样，在 `match` 表达式支臂代码，为简短代码时，就通常不会使用花括号。而在要于某个 `match` 支臂中运行多行代码时，就必须使用花括号。比如下面的代码，在该方法每次以 `Coin::Penny` 被调用时，都会打印出 “幸运便士！”，不过仍会返回该代码块的最后值，`1`：

```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println! ("幸运便士！");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

### 绑定到值的模式

`match` 支臂的另一有用特性，便是这些支臂可绑定到值与模式进行匹配值的多个部分（another useful feature of `match` arms is that they can bind to the parts of the values that match the pattern）。这就是从枚举变种提取出值的原理。

作为一个示例，下面就来将这里的枚举变种之一，修改为其内部保存数据。自 1999 年到 2008 年，美国在 25 美分硬币的一面，铸造上 50 个州不同的设计。别的硬币则没有这样的州份设计，因此只有这些 25 美分硬币才有这额外价值。那么就可以通过修改这个 `Quarter` 变种为内部包含一个 `UsState` 值，来将此信息添加到这里的 `enum` 类型，就如同下面清单 6-4 中所做的。

```rust
#[derive(Debug)]    // 这样就可以很快对州份进行检查
enum UsState {
    Alabama,
    Alaska,
    // --跳过--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
```

来设想一下有个朋友是在尝试收集全部 50 个州份的 25 美分硬币。在按照硬币类型对零钱进行分类的同时，还将叫出与每个 25 美分硬币关联的州份名字，如此就可以在发现那个 25 美分硬币，是那位朋友还没有的时候，就可以把那个硬币添加到收藏。

而在这个代码的 `match` 表达式中，就要添加一个名为 `state` 的变量到匹配变种 `Coin::Quarter` 的那些值。在有 `Coin::Quarter` 匹配时，这个 `state` 变量就会绑定到那个 25 美分硬币的状态值。随后就可以在那个支臂的代码里，使用 `state` 变量了，如同下面这样：

```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state: UsState) => {
            println! ("来自 {:?} 州份的 25 美分硬币！", state);
            25
        }
    }
}
```

这时在调用了 `value_in_cents(Coin::Quarter(UsState::Alaska))` 后，`coin` 就将是 `Coin::Quarter(UsState::Alaska)`。在将该值与各支臂进行比较时，在到达 `Coin::Quarter(state: UsState)` 支臂之前，是不会有任何支臂于其匹配的。而在 `Coin::Quarter(state: UsState)` 支臂处，`state` 所绑定的，将是值 `UsState::Alaska`。这时就可以使用那个 `println!` 表达式中的绑定，进而就从 `Quarter` 的 `Coin` 枚举变种，获取到那个内部 `state` 值了。


### `Option<T>` 下的模式匹配

在前一小节，那里是想要在运用 `Option<T>` 时，从 `Some` 情形中获取到那个内部的 `T` 值；像前面对 `Coin` 枚举所做的那样，也可以这样来对 `Option<T>` 加以处理！比较的不再是那些硬币，而是将比较 `Option<T>` 的两个变种，不过那个 `match` 表达式的原理还是一样的。

下面就假设说要编写一个取 `Option<i32>` 类型值的函数，同时当 `Option<i32>` 里面有个值时，就将 `1` 加到那个值上。在 `Option<i32>` 里没有值时，该函数则会返回 `None` 值，并不会尝试执行任何运算。

归功于 `match` 表达式，这个函数写起来很容易，他将看起来像下面清单 6-5 这样。

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(n) => Some(n + 1),
    }
}

fn main() {
    let five = Some(5);
    let none = None;
    println! ("{:?}, {:?}", plus_one(five), plus_one(none));
}
```

*清单 6-5：在 `Option<i32>` 类型上运用了 `match` 表达式的一个函数*

下面就来详细检视一下 `plus_one` 函数的首次执行。在调用 `plus_one(five)` 时，`plus_one` 函数体中的变量 `x` 将有着值 `Some(5)`。之后就会与 `match` 表达式的各个支臂进行比较。

```rust
        None => None,
```

该 `Some(5)` 值不与模式 `None` 匹配，因此就会继续到下一支臂。

```rust
        Some(n) => Some(n + 1),
```

`Some(5)` 与 `Some(n)` 匹配吗？当然是匹配的！这里有着同样的变种。这个 `n` 绑定的是包含在 `Some` 中的那个值，因此 `n` 就会取到值 `5`。随后该 `match` 支臂中的代码就会被执行，从而就会将 `1` 加到 `n` 的值，并创建出一个新的、内部有着这里的和 `6` 的 `Some` 值来。

现在来看看清单 6-5 中第二个 `plus_one` 的调用，其中 `x` 则是 `None` 了。这里进入到那个 `match` 表达式，并与第一个支臂进行比较。

```rust
        None => None,
```

他是匹配的！就没有要加的值了，因此程序就停下来并返回 `=>` 右侧上的那个 `None` 值。由于第一个支臂已经匹配，因此就不会再比较其他支臂了。

在许多场合，将 `match` 表达式与枚举结合都是有用的。在 Rust 代码中将会看到很多这样的模式：对某个枚举的 `match` 操作，将某个变量绑定到内部数据，并随后据此执行代码（`match` against an enum, bind a variable to the data inside, and then execute code based on it）。在刚开始的时候这显得有些难以琢磨，而一旦熟悉了这种模式，就会希望在全部语言中都有这样的模式。这样的模式一直是编程者的最爱。


### 匹配要彻底（Matches Are Exhaustive）

这里有个需要讨论到的 `match` 表达式的另一方面。想想这个有着代码错误而不会编译的 `plus_one` 版本：

```rust
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(n) => Some(n + 1),
        }
    }
```

这里没有对 `None` 情形加以处理，因此该代码就会引起错误。幸运的是，那是个 Rust 知道怎样取捕获的代码错误。在尝试编译此代码时，就会得到这样的错误：

```console
$ cargo run
   Compiling enum_demo v0.1.0 (/home/peng/rust-lang/projects/enum_demo)
error[E0004]: non-exhaustive patterns: `None` not covered
   --> src/main.rs:2:11
    |
2   |     match x {
    |           ^ pattern `None` not covered
    |
note: `Option<i32>` defined here
    = note: the matched value is of type `Option<i32>`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
    |
3   ~         Some(n) => Some(n + 1),
4   ~         None => todo!(),
    |

For more information about this error, try `rustc --explain E0004`.
error: could not compile `enum_demo` due to previous error
```

Rust 是知道这里未曾覆盖到每种可能情形，并甚至清楚这里忘记了那个模式！ Rust 中的 `match` 表达式要是 *彻底的（exhaustive）*：为了让代码有效，就必须穷尽所有的可能性。尤其是在 `Option<T>` 这个示例中，在 Rust 阻止这里忘记显式地处理 `None` 这个情形时，在这里可能会有个 `null` 值时，他就保护了避免有个值的错误假设，进而让那个先前讨论到的十亿美金错误成为不可能了。

### 捕获所有模式与 `_` 占位符（Catch-all Patterns and the `_` Placeholder）

运用枚举，还可以对少数特定值采取特别动作，而对所有其他值采取一种默认动作。设想正在实现某个游戏，其中在投中了骰子上的 3 点时，游戏角色就不会移动，而是会收到一顶新的帽子道具。而在投中 7 点时，游戏角色会失去一定道具帽子。对于其他所有点数值，游戏角色都会在游戏板上移动相应数目的格子。下面就是个实现了该逻辑的 `match` 表达式，其中的骰子点数结果，是硬编码而非随机值，至于其他由不带函数体的函数所表示的逻辑，则是由于实现这些函数超出了本示例的范围：

```rust
let dice_roll = 9;

match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player() {}
```

对于前两个支臂，模式为字面值 `3` 和 `7`。而那最后的最比，则涵盖了所有其他可能的值，该模式为这里以选为命名为 `other` 的那个变量。为该 `other` 支臂所运行的代码，通过将这个 `other` 变量传递给 `move_player` 函数，而用到了这个变量。

由于那最后的模式将匹配到未特别列出的全部值，因此尽管这里并未列出 `u8` 类型变量有的全部可能值，这段代码仍会编译。这种捕获全部的模式，满足了 `match` 表达式务必彻底的要求。请注意由于这些模式是求值的，因此这里必须将那个捕获全部支臂放在最后。若在捕获全部之后，添加了其他支臂，那么 Rust 就会告警，这是由于这些在捕获全部之后的支臂根本不会匹配到！

Rust 还有一种在不愿使用捕获全部模式中的值时，可使用的一种模式：`_`，这是一种特别的、未与该值绑定的其他所有值。这种模式告诉 Rust 这里将不会使用该值，因此 Rust 就不会发出有关某个未用到变量的告警了（Rust also has a pattern we can use when we don't want to use the value in the catch-all pattern: `_`, which is a special pattern that matches any value and doen't not bind to that value. This tells Rust we aren't going to use the value, so Rust won't warn us about an unused varialbe）。

下面就来将那个游戏的规则修改为，在投中骰子的三点和七点之外别的点数时，就必须再投一次骰子。那么这里就不需要用到那个点数值了，因此就可以将这里的代码修改为使用 `_` 而不是那个名为 `other` 的变量：

```rust
    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {}
```

由于这里在最后的支臂中，显式地忽略了全部其他值，因此该示例也是满足 `match` 表达式的穷尽要求的；这里并未忘记掉任何东西。

若再一次修改此游戏的规则，修改为在抛出即非三点也非七点的其他点数时，什么也不会发生，那么就可以通过使用单元值（即在 [元组类型](Ch03_Common_Programming_Concepts.md#the-tuple-type) 小节中讲到的那个空元组类型）作为该 `_` 支臂后的代码，来表达这样的游戏规则：

```rust
let dice_roll = 9;

match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => (),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
```

这里就显式地告诉 Rust，这里将不使用那些不与先前支臂匹配的全部其他值，且在此情形下这里不要运行任何代码。

在 [第 18 章](Ch18_Patterns_and_Matching.md) 将涉及到更多有关模式与匹配的内容。而现在就要移步到 `if let` 语法，在那些使用 `match` 表达式显得多余的情形下，`if let` 语法就会有用。

## `if let` 下的简洁控制流

`if let` 语法，实现了将 `if` 与 `let` 关键字，结合为不那么冗长的，处理与一个模式相匹配，而忽略其余模式的一些值的处理方式（the `if let` syntax lets you combine `if` and `let` into a less verbose way to handle that match one pattern while ignoring the rest）。设想下面清单 6-6 中的这个程序，该程序是要对 `config_max` 变量中的 `Option<u8>` 值进行匹配，而只打算在该值为 `Some` 变种时，才执行代码。

```rust
    let config_max = Some(3u8);

    match config_max {
        Some(max) => println! ("极大值被配置为了 {}"， max);
        _ => ();
    }
```

*清单 6-6：一个仅在乎当值为 `Some` 时运行代码的 `match` 表达式*

在该值为 `Some` 时，这里就通过将那个 `Some` 变种中的值，绑定到这个模式中的变量 `max`，而打印出该值来。这里并不想要对那个 `None` 值做什么操作。为满足 `match` 表达式的要求，这里必须在处理仅仅一个变种之后，添加 `_ => ()`，这就是要添加的恼人样板代码。

相反，可使用 `if let` 语法，以较简短方式写出来。下面的代码与清单 6-6 中的 `match` 表达式表现一致：

```rust
    let config_max = Some(3u8);

    if let Some(max) = config_max {
        println! ("极大值被设置为了 {}", max);
    }
```

`if let` 语法会接收由等号分隔的一个模式与一个表达式。他与 `match` 原理相同，其中的表达式被给到 `match` 表达式，而其中的模式就是 `match` 表达式的第一支臂。在此示例中，模式即为 `Some(max)`，而这个 `max` 就绑定到了 `Some` 里面的那个值。由此，这里随后就可以与在相应的 `match` 支臂中使用 `max` 的同样方式，在后面的那个 `if let` 代码块中对 `max` 进行使用。而在该值 `config_max` 不与该模式匹配时，那个 `if let` 代码块中的代码，就不会运行。

> ***注***：`if let` 实际上是两部分，其中 `let Some(max) = config_max` 是个 scrutinee expression。

使用 `if let` 语法，就意味着较少输入、较少的缩进，以及更少的样板代码。不过会损失 `match` 表达式强制要求的穷尽检查。是根据特定情形下，手头正在做的事情，在 `match` 表达式与 `if let` 语法之间加以选择的，以及考量为收获到简洁，而是否值得损失穷尽性检查。

也就是说，可将 `if let` 语法当作，在值与某个模式匹配时运行代码，并在之后忽略所有其他值的 `match` 表达式的语法糖（in other words, you can think of `if let` as syntax sugar for a `match` that runs code when the value matches one pattern and then ignores all other values）。

这里可以在 `if let` 之下，包含一个 `else` 关键字。`else` 所带的代码块，与在和 `if let` 及 `else` 等价的 `match` 表达式中， `_` 情形所带代码块相同。回想起清单 6-4 中的那个 `Coin` 枚举定义，其中的 `Quarter` 变种还有一个 `UsState` 值。在要通告出那些 25 美分硬币的州份的同时，还要清点出找到的全部非 25 美分数目，那么就可以使用下面这样的 `match` 表达式：

```rust
let mut count = 0;

match coin {
    Coin::Quarter(state) => println! ("这是来自州份 {:?} 的 25 美分硬币！", state),
    _ => count += 1,
}
```

或者这里还可以使用一个像下面这样的 `if let` 与 `else` 的表达式：

```rust
let mut count = 0;

if let Coin::Quarter(state) = coin {
    println! ("这是来自州份 {:?} 的 25 美分硬币！", state);
} else {
    count += 1;
}
```

在遇到程序中使用 `match` 显得太过繁复的逻辑这样情形时，就要记住在 Rust 工具箱中还有 `if let`语法呢。



## 总结

本章已经讲过，怎样运用枚举，来创建可作为一套一一列出数值之一的定制类型。这里给出了标准库的 `Option<T>` 类型，是怎样在运用该类型下，防止代码错误的原理。在枚举值有着内部值时，根据所要处理的多少种情况，而可使用 `match` 表达式或 `if let` 语法，来提取并使用这些值。

现在的 Rust 程序，就可以使用结构体与枚举，对所在领域的那些概念加以表达了。通过在自己构建的 API 使用的定制类型，而确保了类型安全：Rust 编译器将令到 API 中的那些函数，只获取到这些函数所期望类型的那些值。

而为了将可直接使用上的、组织良好的 API 提供到用户，并只暴露 API 的用户所需要部分，那么就要了解一下 Rust 的模组特性了。
