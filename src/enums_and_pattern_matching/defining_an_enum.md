# 定义枚举

结构体给到咱们一种将相关字段及数据分组在一起的方法，比如有着其 `width` 和 `height` 的 `Rectangle`，而枚举则给到咱们一种表达某个值属于一组可能值之一的方式。例如，我们可能想表达 `Rectangle` 属于一组可能的形状之一，其还包括 `Circle` 和 `Triangle`。为此，Rust 允许我们将这些可能性编码为枚举。

我们来看看我们可能想在代码中表达的一种情况，看看为什么枚举在这种情况下要比结构体有用并更合适。假设我们需要处理 IP 地址。目前，有两种用于 IP 地址的主要标准：版本四和版本六。由于这些是我们的程序将遇到的 IP 地址唯一可能性，因此我们可以 *枚举* 所有可能的变种，这就是枚举名字的由来。

任何 IP 地址都会要么是版本四或者版本六的地址，但不会同时是这两种地址。IP 地址的这一属性使得枚举数据结构变得合适，因为枚举值只会是其变种之一。版本四和版本六的地址本质上都仍属于 IP 地址，因此当代码处理适用于任何类别 IP 地址的情况时，他们应被视为同一类型。

通过定义 `IpAddrKind` 枚举并列出 IP 地址会是的可能类别，即 `V4` 与 `V6`，我们就可以在代码中表达这一概念。下面是这个枚举的变种：


```rust
enum IpAddrKind {
    V4,
    V6,
}
```

`IpAddrKind` 现在是一种自定义数据类型，我们可以在我们代码的其他地方使用。


## 枚举值

我们可以像下面这样创建 `IpAddrKind` 两个变种的实例：


```rust
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
```

请注意，这个枚举的两个变种都被纳入其标识符下的命名空间中，我们使用双冒号分隔两者。这很有用，因为现在 `IpAddrKind::V4` 和 `IpAddrKind::V6` 两个值均属于同一类型：`IpAddrKind`。例如，我们随后可定义出一个取任意 `IpAddrKind` 的函数：


```rust
fn route(ip_kind: IpAddrKind) {}
```


而我们可以任一变种调用这个函数：


```rust
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
```


使用枚举还有更多优势。进一步思考我们的 IP 地址类型，目前我们没有存储具体 IP 地址 *数据* 的方法；我们只知道其为什么 *类别，kind*。鉴于咱们刚在第 5 章中了解了结构体，咱们可能会想以结构体来解决这个问题，如下清单 6-1 中所示。


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

<a name="listing_6-1"></a>
**清单 6-1**：使用一个 `struct` 存储 IP 地址的数据及 `IpAddrKind` 变种*

在这里，我们定义了个结构体 `IpAddr`，他有两个字段：一个 `IpAddrKind` 类型的 `kind` 字段（我们之前定义的枚举）和一个 `String` 类型的 `address` 字段。咱们有这一结构体的两个实例。第一个是 `home`，他以值 `IpAddrKind::V4` 作为其 `kind` 字段，与 `127.0.0.1` 的关联地址数据。第二个实例是 `loopback`。他以 `IpAddrKind` 的另一变种 `V6` 作为其 `kind` 值，并以地址 `::1` 于其关联。我们已使用了个结构体将 `kind` 和 `address` 两个值捆绑在一起，因此现在变种就与值关联了。

然而，只使用枚举表示相同的概念更为简洁：我们可将数据直接放入各个枚举变种中，而不是结构体内的枚举。下面这个 `IpAddr` 枚举的新定义，表达了 `V4` 和 `V6` 两个变种都将有着关联的 `String` 值：


```rust
enum IpAddr {
    V4(String),
    V6(String),
}

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
```


我们直接将数据附加到枚举的各个变种，因此不需要额外的结构体。在这里，还可以更容易地看到枚举工作原理的另一个细节：我们定义的各个枚举变种名字，也成为构造枚举实例的函数。也就是说，`IpAddr::V4()` 是个函数调用，他会取一个 `String` 参数并返回 `IpAddr` 类型的一个实例。由于枚举的定义，我们自动让这一构造函数得以定义。

使用枚举而非结构体还有另一优势：每个变种都可以有不同类型及数量的关联数据。版本四的 IP 地址将总是有着四个数字组件，他们将有着 0 到 255 之间的值。若我们打算将 `V4` 的地址存储为四个 `u8` 值，而仍将 `V6` 地址表示为一个 `String` 值时，在结构体下我们就无法做到。枚举会轻松处理这种情况：


```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
```


我们已展示数种不同的定义数据结构方式，以存储第四和第六版 IP 地址。然而，事实证明，希望存储 IP 地址并编码其属于何种类别进行编码是如此普遍，以致 [标准库就有我们可以使用的定义](https://doc.rust-lang.org/std/net/enum.IpAddr.html)！我们来看看标准库如何定义 `IpAddr`。他有着与我们已定义并使用的完全一致枚举及变种，但他以两个不同结构体的形式将地址数据嵌入到变种内，针对每个变种，两个结构体被分别定义：


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

这段代码说明咱们可以将任何类别的数据都放入枚举变种内：比如字符串、数字类型或结构体等。咱们甚至可以包含另一枚举！此外，标准库类型通常并不会比咱们可能想到的复杂多少。

请注意，即使标准库包含 `IpAddr` 的定义，我们仍然可以创建并使用咱们自己的定义而不会发生冲突，因为我们尚未将标准库的定义带入我们的作用域中。我们将在第 7 章中进一步讨论将类型带入作用域。

我们来看看下面清单 6-2 中枚举的另一示例：这个枚举在其变种中嵌入了多种类型。


```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

<a name="listing_6-2"></a>
**清单 6-2**：一个 `Message` 枚举，其变种各自都存储不同数量和类型的值


这个枚举有四个不同类型的变种：


- `Quit` 完全没有与其关联的数据；
- `Move` 有一些命名字段，就像结构体那样；
- `Write` 包含单个 `String`；
- `ChangeColor` 包含三个 `i32` 值。


以诸如清单 6-2 中的那些变种定义枚举，类似于定义不同类别的结构体定义，只不过枚举未使用 `struct` 关键字，并且所有变种都在 `Message` 类型分组在一起。以下结构体可保存与前面的枚举变种保存的相同数据：


```rust
struct QuitMessage; // 单元值结构体
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String);    // 元组结构体
struct ChangeColorMessage(i32, i32, i32);   // 元组结构体
```


但若我们使用不同的结构体，每个结构体都有自己的类型，我们就无法像以清单 6-2 中定义的 `Message` 枚举，其为一个单一类型，那样轻松地定义一个取任何的这些类别消息的函数。

枚举和结构体之间还有个相似之处：正如我们能够使用 `impl` 在结构体上定义方法一样，我们也能够在枚举上定义方法。下面是个名为 `call` 的方法，我们可以在 `Message` 枚举上他：


```rust
impl Message {
    fn call(&self) {
        // 方法体将在这里定义
    }
}


    let m = Message::Write(String::from("hello"));
    m.call();
```


方法的主体将使用 `self` 来获取我们与其上调用该方法的值。在这个示例中，我们创建了个变量 `m`，有着值 `Message::Write(String::from("hello"))`，而这便是 `m.call()` 运行时 `self` 在 `call` 方法主体中的内容。

我们来看看标准库中的另一枚举，其非常常见和有用：`Option`。


## 枚举 `Option`

这一节探讨 `Option` 的案例研究，他是由标准库定义的另一个枚举。`Option` 类型编码了非常常见的情形，其中某个值可能是某物，也可能什么也没有。

例如，当咱们请求某个非空列表中的第一个项目时，咱们将得到某个值。当咱们请求某个空列表中的第一个项目时，咱们将什么也得不到。以类型系统表达这一概念，意味着编译器可以检查咱们是否已处理了所有咱们应该处理的情况；这一功能可以防止其他编程语言中极为常见的一些 bug。

编程语言设计通常会从咱们要包含那些特性方面考虑，但咱们要排除哪些特性也很重要。Rust 没有许多其他语言都有的空值，`null`，功能。所谓 *空值*，是个表示该处没有值的值。在有着空值的语言中，变量会始终处于两种状态之一：空值或非空值。

空值的发明者 Tony Hoare 在他 2009 年的演讲 “空引用：10 亿美金的失误（Null Reference: The Billion Dollar Mistake）” 中，这样说道：


> 我称之为我的十亿美元失误。当时，我正在为一门面向对象语言中的引用，设计首个全面的类型系统。我的目标是确保在由编译器自动执行的检查下，所有对引用的使用都应绝对安全。但我未能抵制住加入空引用的诱惑，仅仅因为他太容易实现了。这已导致了数不清的错误、漏洞和系统崩溃，他们在过去的四十年里可能已造成数十亿美元的痛苦和损失。


空值的问题在于，当咱们尝试将空值用作非空值时，咱们将得到某种错误。由于这一空值或非空值的属性普遍存在，因此极易造成这种错误。

然而，空值试图表达的概念仍然是个有用的概念：空值属于一个当前无效，或出于某种原因不存在的值。

问题实际上不在于这个概念，而在于特定的实现。因此，Rust 没有空值，但他有个枚举，可以编码值存在或不存在这一概念。这个枚举就是 `Option<T>`，他由 [标准库定义](https://doc.rust-lang.org/std/option/enum.Option.html) 如下：

```rust
enum Option<T> {
    None,
    Some(T),
}
```

`Option<T>` 枚举非常有用，以致于他甚至包含在前奏中；咱们不需要显式地将他带入作用域。他的变种也包含在前奏中：咱们可直接使用 `Some` 和 `None` 而无需 `Option::` 前缀。`Option<T>` 枚举仍然只是个常规枚举，而 `Some(T)` 和 `None` 仍然都是 `Option<T>` 类型的变种。

`<T>` 语法是我们尚未讨论的一项 Rust 特性。他是个泛型参数，我们将在第 10 章中更深入介绍泛型。现在，咱们需要知道只是 `<T>` 表示 `Option` 枚举的 `Some` 变种可以保存任何类型的一条数据，并且用来代替 `T` 的每种具体类型都会使整个 `Option<T>` 类型成为不同的类型。下面是一些使用 `Option` 值保存数字类型与字符类型的示例：


```rust
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
```


其中 `some_number` 的类型是 `Option<i32>`。`some_char` 的类型是 `Option<char>`，这属于一种不同类型。Rust 可以推断这些类型，因为我们已在 `Some` 变种中指定了一个值。对于 `absent_number`，Rust 要求我们注解整个 `Option` 类型：编译器无法仅通过查看 `None` 值来推断相应 `Some` 变种将持有的类型。在这里，我们告诉 Rust 我们指的是 `absent_number` 属于 `Option<i32>` 类型。

当我们有个 `Some` 值时，我们知道某个值存在，并且该值保存在 `Some` 中。当我们有个 `None` 值时，那么在某种意义上他表示与空值相同的事物：我们没有有效值。那么，为什么有着 `Option<T>` 比有着空值更好呢？

简而言之，因为 `Option<T>` 与 `T`（其中 `T` 可以是任何类型）属于不同类型，编译器将不允许我们将 `Option<T>` 值当作绝对是个有效值使用。例如下面这段代码将不编译，因为他试图将 `i8` 与 `Option<i8>` 相加：


```rust
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;
```


当我们运行这段代码时，我们会收到如同下面这样的报错消息：


```console
$ cargo run
   Compiling option_enum_demo v0.1.0 (/home/hector/rust-lang-zh_CN/projects/option_enum_demo)
error[E0277]: cannot add `Option<i8>` to `i8`
 --> src/main.rs:5:17
  |
5 |     let sum = x + y;
  |                 ^ no implementation for `i8 + Option<i8>`
  |
  = help: the trait `Add<Option<i8>>` is not implemented for `i8`
help: the following other types implement trait `Add<Rhs>`
 --> /rustc/01f6ddf7588f42ae2d7eb0a2f21d44e8e96674cf/library/core/src/ops/arith.rs:114:1
  |
  = note: `&i8` implements `Add<i8>`
  |
  = note: `&i8` implements `Add`
  |
  = note: `i8` implements `Add<&i8>`
  |
  = note: `i8` implements `Add`
  = note: this error originates in the macro `add_impl` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0277`.
error: could not compile `option_enum_demo` (bin "option_enum_demo") due to 1 previous error
```


相当尖刻！实际上，这条报错消息是指 Rust 不明白如何将 `i8` 与 `Option<i8>` 相加，因为他们属于不同类型。当我们在 Rust 中有个像 `i8` 这样类型的值时，编译器将确保我们始终有个有效的值。我们可以放心地继续操作，而不必在使用该值前检查是否为空值。只有当我们有个 `Option<i8>`（或我们正在使用的任何其他值类型）时，我们才必须担心可能并没有值，而编译器将确保我们在使用该值前会处理这种情况。

换句话说，在咱们可对 `Option<T>` 执行 `T` 的运算前，我们必须将其转换为 `T`。一般来说，这有助于捕获空值下的最常见问题之一：在某物实际上是空值时，假设了其不是空值。

消除错误地假设非空值的风险，会帮助咱们对咱们的代码更有信心。为了获得可能为空的值，咱们必须显式地选择将该值的类型构造为 `Option<T>`。然后，当咱们使用该值时，咱们就会被要求显式地处理该值为空的情况。只要值的类型不是 `Option<T>`，咱们就 *可以* 安全地假设该值不是空值。这属于一项 Rust 的有意设计决定，以限制空值泛滥进而提高 Rust 代码的安全性。

那么，当咱们有个类型 `Option<T>` 的值时，怎样从 `Some` 变种中获取 `T` 值以便咱们可以使用该值呢？`Option<T>` 枚举有着大量在不同情形下有用的方法；咱们可在 [其文档](https://doc.rust-lang.org/std/option/enum.Option.html) 中查看这些方法。熟悉 `Option<T>` 上的方法将对咱们的 Rust 之旅大有裨益。

一般来说，为了使用 `Option<T>` 值，咱们会希望有着将处理各个变种的代码。咱们会想要一些将只在咱们有个 `Some(T)` 值时才运行的代码，而这些代码会被允许使用内层 `T`；咱们会希望一些别的只在咱们有个 `None` 值时运行的代码，而这些代码没有可用的 `T` 值。`match` 表达式属于一种控制流结构，在与枚举一起使用时恰好完成了这点：他将根据其有着枚举的哪个变种运行不同的代码，而这些代码可以使用匹配值内部的数据。


（End）


