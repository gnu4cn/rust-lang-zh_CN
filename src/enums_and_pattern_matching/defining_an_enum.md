# 定义枚举

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


但是，如果我们使用不同结构体（每个结构体都有自己的类型），我们就无法像使用清单 6-2 中定义的 `Message` 枚举（是个单一类型）那样，轻松定义出取任何一种这些类别消息的函数。

枚举和结构体之间，还有个相似之处：正如我们可以使用 `impl` 在结构体上定义方法一样，我们也可以在枚举上定义方法。下面是我们可以在 `Message` 枚举上，定义的一个名为 `call` 的方法：


```rust
impl Message {
    fn call(&self) {
        // 方法体将定义在这里
    }
}


    let m = Message::Write(String::from("hello"));
    m.call();
```


这个方法的主体，将使用 `self` 来获取我们在其上调用该方法的值。在本例中，我们创建了一个值为 `Message::Write(String::from("hello"))` 的变量 `m`，当 `m.call()` 运行时，这就是出现在 `call` 方法主体中 `self` 的内容。

我们来看看标准库中，另一个非常常见和有用的枚举：`Option`。


## `Option` 枚举及其相对于空值的优势

**The `Option` Enum and Its Advantages Over Null Values**


本节会探讨 `Option` 的一个案例研究，这是标准库定义的另一枚举。`Option` 类型编码了一种非常常见的情况，即某个值可能是某物，也可能什么都不是。

例如，如果咱们请求了某个非空列表中的首个项目，咱们将得到某个值。如果咱们请求某个空列表中的第一项，则什么也得不到。用类型系统来表达这一概念，意味着编译器可以检查，咱们是否处理了所有应处理的情况；这一功能可以防止其他编程语言中，极为常见的编程错误。

编程语言的设计，通常会考虑包含哪些功能，但排除哪些功能，也很重要。Rust 没有许多其他语言所具有的空值 null 功能。*Null* 是个表示没有值的值。在有控制的语言中，变量总是处于两种状态之一：空值或非空值。

空值的发明者 Tony Hoare 于 2009 年的演讲 “空值引用：10 亿美金代价失误（Null Reference: The Billion Dollar Mistake）” 中，就讲了这个问题：


> 我称他为我的 "十亿美元错误"。当时，我正在为面向对象语言中的引用，设计第一个全面的类型系统。我的目标是确保所有引用的使用，都绝对安全，并由编译器自动进行检查。但是，我无法抵制加入空引用的诱惑，只是因为这太容易实现了。这导致了无数的错误、漏洞和系统崩溃，在过去的四十年里，这些错误和漏洞可能造成了数十亿美元的损失。


空值的问题在于，如果试图将空值用作非空值，就会出现某种错误。由于这种空值或非空值属性普遍存在，因此极易出现这种错误。

然而，空值试图表达的这种概念，仍然是有用的：空值是指由于某种原因，当前无效或不存在的值。

问题其实不在于概念，而在于特定的实现。因此，Rust 没有空值，但有个枚举可以编码值存在或不存在的概念。这个枚举就是 `Option<T>`，[标准库对其定义](https://doc.rust-lang.org/std/option/enum.Option.html) 如下：


```rust
enum Option<T> {
    None,
    Some(T),
}
```


`Option<T>` 枚举非常有用，以致他甚至被包含在 Rust 前奏中；咱们不需要显式地将他引入作用域。他的变种也包含在前奏中：咱们可以直接使用 `Some` 和 `None`，而无需 `Option::` 这个前缀。`Option<T>` 枚举仍然只是个普通的枚举，而 `Some(T)` 和 `None`，也仍然是 `Option<T>` 类型的变种。

`<T>` 这种语法，是我们尚未讨论过的一项 Rust 特性。他是个泛型参数，我们将在第 10 章，详细介绍泛型。现在，咱们只需知道 `<T>` 表示 `Option` 枚举的 `Some` 变种，可以容纳任意类型的数据，而每个用来代替 `T` 的具体类型，都会使整个 `Option<T>` 类型，成为不同的类型。下面是一些使用 `Option` 值，保存数字类型和字符串类型的示例：


```rust
    let some_numer = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
```


其中 `some_number` 的类型是 `Option<i32>`。`some_char` 的类型是 `Option<char>`，这是一种不同的类型。Rust 可以推断出这些类型，因为我们在 `Some` 变种中，指定了某个值。对于 `absent_number`，Rust 要求我们注解整个 `Option` 类型：编译器无法仅通过查看 `None` 值，来推断相应 `Some` 变种将持有的类型。在这里，我们告诉 Rust，我们的意思是 `absent_number` 属于 `Option<i32>` 类型。

当我们有某个 `Some` 值时，我们知道有个值存在，并且该值被保存在 `Some` 中。当我们有个 `None` 值时，从某种意义上说，他的含义与空值相同：我们没有一个有效值。那么，为什么 `Option<T>` 比空值更好呢？

简而言之，由于 `Option<T>` 和 `T`（`T` 可以是任何类型）属于不同的类型，编译器不会让我们，将某个 `Option<T>` 值用作其肯定是个有效值。例如，下面这段代码将不会编译，因为他试图将一个 `i8`，与一个 `Option<i8>` 相加：


```rust
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;
```


如果我们运行这段代码，我们会收到如下报错：


```console
$ cargo run
   Compiling option_demo v0.1.0 (C:\tools\msys64\home\Lenny.Peng\rust-lang-zh_CN\projects\option_demo)
error[E0277]: cannot add `Option<i8>` to `i8`
 --> src\main.rs:5:17
  |
5 |     let sum = x + y;
  |                 ^ no implementation for `i8 + Option<i8>`
  |
  = help: the trait `Add<Option<i8>>` is not implemented for `i8`
  = help: the following other types implement trait `Add<Rhs>`:
            <i8 as Add>
            <i8 as Add<&i8>>
            <&'a i8 as Add<i8>>
            <&i8 as Add<&i8>>

For more information about this error, try `rustc --explain E0277`.
error: could not compile `option_demo` (bin "option_demo") due to previous error
```


强悍如此！实际上，这条报错信息，表示 Rust 不理解如何将某个 `i8` 与某个 `Option<i8>` 相加，因为他们属于不同的类型。在 Rust 中，当我们有个 `i8` 类型的值时，编译器会确保我们，始终有个有效的值。在使用该值之前，我们无需检查是否为空值，就可以放心地继续。只有当我们有个 `Option<i8>`（或其他任何类型的值）时，我们才必须担心可能并没有值，编译器会确保我们，在使用该值前处理这种情况。

换句话说，在对 `Option<T>` 执行 `T` 的运算之前，我们必须先将其转换为 `T`。一般来说，这有助于捕捉空值最常见的问题之一：假设了某个项目不是空值，而实际上他却是空值。

消除错误地假定某个非空值的风险，可以让咱们对咱们代码更有信心。为了获得某个可能为空的值，咱们必须显式地选择，将该值的类型设为 `Option<T>`。然后，在使用该值时，咱们必须显式地处理，该值为空的情况。只要值的类型不是 `Option<T>`，咱们就可以放心地认为，该值不是空值。这是 Rust 为限制空值泛滥，和提高 Rust 代码的安全性，而特意做出的设计决定。

那么，当咱们有着某个 `Option<T>` 类型的值时，该怎样从 `Some` 变种中获取到那个 `T` 值，以便使用该值呢？`Option<T>` 这个枚举，有着大量在不同场景下，都有用的方法；咱们可以在 [其文档](https://doc.rust-lang.org/std/option/enum.Option.html) 中，查看这些方法。熟悉 `Option<T>` 的方法，将对咱们的 Rust 之旅大有裨益。

一般来说，为了使用某个 `Option<T>` 值，咱们需要编写处理每个变种的代码。咱们会想要一些，仅在咱们有个 `Some(T)` 值时才会运行的代码，而这些代码，就可以使用内部的 `T` 值；咱们会想要另一些，只有在咱们有个 `None` 值时才会运行的代码，而这些代码就没有可用的 `T` 值。与枚举一起使用的 `match` 表达式，便是一种正好完成这个目的的控制流结构：他会根据枚举有着哪一个变种，而运行不同的代码，而这些代码，就可以使用匹配值内部的数据。


（End）


