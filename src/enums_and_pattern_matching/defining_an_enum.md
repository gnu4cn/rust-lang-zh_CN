# 定义一个枚举

**Defining an Enum**


结构体提供了一种将相关字段和数据，分组在一起的方法，比如有着 `width` 和 `height` 两个字段的 `Rectangle`，而枚举则提供了一种，表示一个值是一组可能值之一的方法。例如，我们可能想表达，`Rectangle` 是一组其中还包括 `Circle` 和 `Triangle` 等可能形状之一。为此，Rust 允许我们，将这些可能性编码为某个枚举。

我们来看看我们可能想在代码中表达的一种情况，看看为什么在这种情况下，枚举要比结构体更有用、更合适。假设我们需要处理 IP 地址。目前，有两种用于 IP 地址的主要标准：版本四和版本六。由于我们的程序只会遇到这两种可能的 IP 地址，因此我们可以 *枚举出* 所有可能的变种，这就是枚举名称的由来。

任何 IP 地址都可以是版本 4 或版本 6 的地址，但不能同时是这两种地址。IP 地址的这一属性，使得枚举这种数据结构非常合适，因为某个枚举值，只能是其变体之一。从根本上说，版本 4 和版本 6 地址，都仍是 IP 地址，因此当代码在处理适用于任何类别的 IP 地址的情况时，他们应被视为同一类型。

可以通过定义一个 `IpAddrKind` 枚举，并列出某个 IP 地址可以是的那些可能种类，即 `V4` 与 `V6`，我们就可以在咱们的代码中，表达这一概念。下面就是这个枚举的那些变种：


```rust
enum IpAddrKind {
    V4,
    V6,
}
```

`IpAddrKind` 现在就是一种，我们可以在我们代码的其他地方使用的自定义数据类型。


## 枚举值

**Enum Values**


我们可以像下面这样，创建出 `IpAddrKind` 两个变种的实例：


```rust
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
```

请注意，该枚举的两个变种，都是在其标识符下的命名空间中，而我们要使用双冒号，分隔两者。这很有用，因为现在 `IpAddrKind::V4` 和 `IpAddrKind::V6` 两个值，属于同一类型：`IpAddrKind`。例如，我们随后就可以定义出一个，取任意 `IpAddrKind` 的函数：


```rust
fn route(ip_kind: IpAddrKind) {}
```


并且我们可以两个变种，调用这个函数：


```rust
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
```


使用枚举还有更多优势。请再想想我们的 IP 地址类型，目前我们还没有存储具体 IP 地址 *数据* 的方法；我们只知道他是什么 *类别，kind*。鉴于咱们刚刚在第 5 章中，学习了结构体，咱们可能会想使用结构体，来解决这个问题，如下清单 6-1 所示。


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

*清单 6-1：使用 `struct` 存储 IP 地址的数据及 `IpAddrKind` 变种*

这里，我们定义了个有着两个字段的结构体 `IpAddr`：一个是 `IpAddrKind` 类型的 `kind` 字段（我们之前定义的枚举），另一个是 `String` 类型的 `address` 字段。咱们有着此结构体的两个实例。第一个实例是 `home`，其 `kind` 字段的值 `IpAddrKind::V4`，与其关联的地址数据为 `127.0.0.1`。第二个实例是 `loopback`。其 `kind` 字段值为 `IpAddrKind` 的另一变种 `V6`，关联的地址是 `::1`。我们已使用了一个结构体，将 `kind` 和 `address` 两个值捆绑在一起，因此现在这个变种，与值相关联了。

不过，只使用一个枚举，来表示这同样的概念，更为简洁：我们可以将数据直接放入各个枚举变种中，而不是将枚举放在结构体中。下面这个 `IpAddr` 枚举的新定义指出，`V4` 和 `V6` 两个变种，都将有着关联的 `String` 值：


```rust
enum IpAddr {
    V4(String),
    V6(String),
}

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
```


我们直接将数据，附加到枚举的各个变种，因此不需要一个额外结构体。在这里，我们还可以更容易地了解，枚举工作原理的另一细节：我们定义的每个枚举变种名字，还成为了用来构造枚举实例的一个函数。也就是说，`IpAddr::V4()` 是个取一个 `String` 参数，并返回 `IpAddr` 类型实例的函数调用。定义那个枚举时，我们就自动获得了这个构造函数。

使用枚举而非结构体，还有另一好处：每个变种都可以有不同类型和数量的关联数据。版本四的 IP 地址，总是有范围在 0 到 255 之间的四个数字部分。如果我们打算将 `V4` 地址，存储为四个 `u8` 值，但仍想要将 `V6` 地址，表示为一个 `String` 值，那么我们就无法使用结构体。枚举则可以轻松处理这种情况：


```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
```


我们已经展示了几种不同的，定义存储第四和第六版 IP 地址数据结构的方法。然而，事实证明，想要存储 IP 地址并对其进行编码，是如此普遍，以致 [标准库就有我们可以使用的定义](https://doc.rust-lang.org/std/net/enum.IpAddr.html)！我们来看看标准库是如何定义 `IpAddr` 的：他有着与我们定义和使用的完全同样的枚举和变种，但他将地址数据，以两个不同结构体的形式，嵌入到两个变种中，对于每个变种，表示地址数据的结构体定义都不同：


```rust
struct Ipv4Addr {
    // --跳过--
}

struct Ipv6Addr {
    // --跳过--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```


这段代码说明，咱们可以在枚举变种中，放入任何类别的数据：比如字符串、数字类型或结构体等。咱们甚至还可以包含另一枚举！此外，一些标准库类型，通常也不会比咱们接下来会构造出的类型复杂多少。

请注意，即使标准库包含了 `IpAddr` 的定义，我们仍然可以创建并使用咱们自己的定义，而不会发生冲突，因为我们还没有将标准库的定义，引入我们的作用域。我们将在第 7 章中，详细讨论将类型引入作用域的问题。

我们再来看看，下面清单 6-2 中枚举的另一示例：这个枚举的变种中，嵌入了多种类型。


```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write (String),
    ChangeColor(i32, i32, i32),
}
```

*清单 6-2：每个变种都存储了不同数量和类型值的 `Message` 枚举*


该枚举有着分别嵌入了不同类型的四个变种：


- `Quit` 完全没有与其关联的数据；

- `Move` 有着一些命名的字段，就像结构体那样；

- `Write` 包含单个 `String`；

- `ChangeColor` 包含了三个 `i32` 的值。


定义有着如清单 6-2 中的那些变种的枚举，类似于定义不同种类的结构体定义，只是枚举没有使用 `struct` 关键字，且全部变种都在 `Message` 这个类型下编组在一起。下面的结构体，可以保存与前面那些枚举变种，同样的数据：


```rust
struct QuitMessage; // 单元结构体
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String);    // 元组结构体
struct ChangeColorMessage(i32, i32, i32);   //  元组结构体
```


但是，如果我们使用不同结构体（每个结构体都有自己的类型），我们就无法像使用清单 6-2 中定义的 `Message` 枚举（这是一种单一类型）那样，轻松定义出取任何一种这些类别消息。

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


## 枚举 `Option` 及其超越空值的诸多优点

**The `Option` Enum and Its Advantages Over Null Values**

本小节会探讨 `Option` 的案例研究，`Option` 是由标准库定义的另一个枚举。`Option` 类型编码了某个值可能会是某个物件，或可能什么都不属于的这种甚为常见的场景（the `Option` type encodes the very common scenario in which a value could be something or it could be nothing）。比如在请求某个含有一些项目的清单的第一个项目时，就会得到一个值。而在请求某个空清单的第一个项目时，则会什么也得不到。以类型系统字眼，来表达这个概念，就表示编译器能够对，是否已处理了本应处理的全部情形，进行检查；此项功能可阻止那些在其他编程语言中极为常见的代码错误。

编程语言的设计，通常要考量包含哪些特性，而要排除哪些特性也至关重要。Rust 没有许多其他语言都有的空值特性。*空值（Null）* 是一个表示此处无值的值。在带有 `null` 的那些语言中，变量总是会处于下面两个状态之一：空值或非空值。

在 `null` 的发明人Tony Hoare 于 2009 年的演讲 “空值引用：10 亿美金代价失误（Null Reference: The Billion Dollar Mistake）” 中，就讲了这个问题：

> 我把他叫做我的 10 亿美元失误。那个时候我正在设计某门面向对象语言中的首个综合类型系统。目的是要在编译器自动执行的检查之下，确保全部引用使用，都应绝对安全。仅仅因为空值引用变量实现起来很容易，我当时就没能顶住诱惑，把他加入到特性集了。这个举动，业已造成了数不胜数的代码错误、漏洞及系统崩溃等等问题，在过去 40 余年里，这些问题可能已经造成大概 10 亿美金的痛苦和伤害。

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
