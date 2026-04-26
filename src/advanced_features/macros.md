# 宏，macro

在这本书中，我们一直使用像 `println!` 这样的宏，但尚未全面探讨什么是宏，以及他的工作原理。 *宏，macro* 这个术语，指的是 Rust 中的一类特性 -- 通过 `macro_rules!` 的 *声明式宏，declarative macro*，以及如下三种 *程序式宏，procedural macro*：

- 自定义的 `#[derive]` 宏，他们通过 `derive` 属性，对结构体和枚举指定添加的代码（译注：这类似于 Python 中的 `@` 装饰语法）；
- 类属性的宏，定义可用于任何项目的自定义属性；
- 类函数的宏，看起来像函数调用，但对指定为其参数的标记（程序项目）执行操作。

我们将依次讨论其中的每一种，但首先，我们来看看既然有了函数，为什么我们还需要宏。


## 宏与函数的区别

从根本上说，宏属于一种编写生成其他代码的代码的方式，这被称为 *元编程，metaprogramming*。在 [附录 C](../appendix/derivable_traits.md) 中，我们会讨论 `derive` 属性，其会为咱们生成各种特质的实现。在这本书的各处，我们也已使用了 `println!` 与 `vec!` 两个宏。所有这些宏都会 *展开，expand*，从而生成比咱们手写的代码更多的代码。

元编程对于减少咱们必须编写和维护代码量很有用，这也是函数的作用之一。但是，宏有着函数不具备的一些额外能力。

函数签名必须声明函数具有的参数个数和类型。另一方面，宏可以取可变数量的参数：我们可以以一个参数调用 `println! ("你好")`，也可以以两个参数调用 `println! ("你好 {}", name)`。此外，宏会在编译器解析代码含义之前得以展开，因此宏就可以，比如对给定类型实现特质。函数则无法做到这点，因为函数是在运行时被调用的，而特质需要在编译时实现。

实现宏而非函数的缺点在于，宏的定义比函数的定义更为复杂，因为咱们要编写生成 Rust 代码的 Rust 代码。由于这种间接性，宏的定义通常比函数的定义更难阅读、理解和维护。

宏与函数之间的另一重要区别在于，在文件中调用宏 *之前*，咱们必须先定义宏或带入他们到作用域，这与咱们可以在任何地方定义和调用任何地方的函数相反。


## 用于通用元编程的声明式宏

Rust 中使用最广泛的宏形式属于 **声明式宏，declarative macro**。这些宏有时也被称为

- “示例宏，macros by example”，
- “`macro_rules!` 宏”，
- 或简称 “宏”。

从本质上讲，声明式宏允许咱们编写类似于 Rust `match` 表达式的代码。正如 [第 6 章](../enums_and_pattern_matching/match_control_flow.md) 中讨论过的，`match` 表达式属于一种控制结构，取一个表达式、将该表达式的结果值与模式比较，然后运行与匹配模式关联的代码。宏也会将某个值与关联特定代码的模式比较：在这种情形下，值就是传递给宏的字面量 Rust 源代码；模式会与该源代码的结构比较；当二者匹配时，就会替换传递给宏的代码。这一过程全都发生在编译器期间。

为了定义宏，咱们要使用 `macro_rules!` 结构体。我们通过分析 `vec!` 宏的定义方式，来了解怎样使用 `macro_rules!`。第 8 章介绍了怎样使用 `vec!` 宏创建带有特定值的新矢量值。例如，以下宏会创建一个包含三个整数的新矢量值：

```rust
let v: Vec<u32> = vec! [1, 2, 3];
```

我们也可以使用 `vec!` 宏构造一个包含两个整数的矢量值，或者一个包含五个字符串切片的矢量值。我们无法使用函数来执行同样的操作，因为我们事先不知道值的数量或类型。

下面清单 20-35 展示了 `vec!` 宏的略微简化的定义。

<a name="listing_20-35"></a>
文件名：`projects/declarative_macro/src/main.rs`

```rust
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
```

**清单 20-35**：`vec!` 宏定义的简化版本

> **注意**：标准库中 `vec!` 宏的实际定义，包含了预先分配正确数量内存的代码。该代码属于一种优化，为了使示例更简单，我们没有包含该代码。

其中 `#[macro_export]` 注解表明，只要定义该宏的代码箱被带入作用域，那么这个宏就应可用。若没有这个注解，该宏就无法被带入作用域。

然后我们以 `macro_rules!` 以及 *不带* 感叹号的我们正在定义的宏的名字，来开始宏的定义。在这一情形下名字即为 `vec`，后跟表示宏定义主体的花括号。

`vec!` 主体中的结构，类似于 `match` 表达式的结构。这里，我们有一个支臂，有着

- 模式 `( $( $x:expr ),* )`，
- 后跟 `=>`，
- 以及与该模式关联的代码块。

当模式匹配时，关联代码块将得以生成。鉴于这是这个宏中唯一的模式，因此只有一种有效的匹配方式；任何其他模式都将导致报错。更复杂的宏将有着多个支臂。

宏定义中有效的模式语法，与第 19 章中介绍的模式语法不同，因为宏的模式是与 Rust 代码结构匹配，而非与值匹配。我们来详细解析清单 20-35 中的模式片段的含义；有关完整的宏模式语法，请参阅 [Rust 参考手册](https://doc.rust-lang.org/stable/reference/macros-by-example.html)。

首选，我们使用一对圆括号来环绕整个模式。我们使用美元符号（`$`）声明宏系统中的一个变量，该变量将包含匹配模式的 Rust 代码。美元符号清楚地表明这是个宏变量，而非普通 Rust 变量。接下来的一对圆括号，捕获匹配其内模式的值，供替换代码中使用。在 `$()` 内的是 `$x:expr`，这会匹配任意 Rust 表达式，并给予表达式名字 `$x`。

`$()` 后的逗号表示，在匹配 `$()` 中代码的每个代码实例之间必须出现一个字面上的逗号分隔符。随后的 `*` 指定该模式会匹配 `*` 之前的内容零次或多次（译注：这一点与正则表达式类似）。

当我们通过 `vec! [1, 2, 3];` 调用这个宏时，`$x` 模式会与三个表达式 `1`、`2` 和 `3` 匹配三次。

现在我们来看看与这个支臂关联的代码主体中的模式：`$()*` 内的 `temp_vec.push()` 会针对匹配模式中的 `$()` 每个部分，而生成零次或多次，具体取决于该模式匹配的次数。其中 `$x` 以每个匹配的表达式替换。当我们以 `vec! [1, 2, 3];` 调用这个宏时，生成的替换这次宏调用的代码将如下：

```rust
{
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
}
```

我们定义了一个宏，他可以取任意数量、任意类型的参数，并且可以生成代码来创建一个包含指定元素的矢量。

要了解有关如何编写宏的更多信息，请查阅在线文档或其他资源，比如由 Daniel Keep 撰写，Lukas Wirth 接续编写的 [The Little Book of Rust Macros](https://veykril.github.io/tlborm/)。


## 用于根据属性生成代码的过程宏

宏的第二种形式是过程宏，其行为更像是函数（并且属于一种过程，a type of procedure）。*过程宏，procedural macros* 接受一些代码作为输入，对该代码操作，并生成一些代码作为输出，而非像声明式宏那样，与模式匹配并以其他代码替换代码。过程宏的三种类别分别是

- 自定义的 `derive` 宏、
- 类属性宏，attribute-like macros、
- 及类函数宏，function-like macros。

并且三种都以类似方式工作。

在创建过程宏时，其定义必须位于一个有着特殊代码箱类型的他们自己的代码箱中。这是出于一些复杂的技术原因，我们（Rust 开发团队）希望今后能消除这些原因。在下面清单 20-36 中，我们展示了怎样定义一个过程宏，其中 `some_attribute` 是使用特定宏变种的占位符。

<a name="listing_20-36"></a>
文件名：`src/lib.rs`

```rust
use proc_macro::TokenStream;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
}
```

**清单 20-36**： 定义过程宏的示例

定义过程宏的函数，会取一个 `TokenStream` 值作为输入，并生成一个 `TokenStream` 作为输出。`TokenStream` 类型由 Rust 附带的 `proc_macro` 代码箱定义，表示令牌序列，a sequence of tokens。这是这种宏的核心：宏所操作的源代码构成输入的 `TokenStream`，而宏生成的代码则是输出的 `TokenStream`。该函数附带了一个属性，指定我们正在创建何种类别的过程宏。我们可以在同一个代码箱中包含多种类别的过程宏。

我们来看看不同类别的过程宏。我们将从自定义 `derive` 宏开始，然后探讨使其他形式有所不同的细微差异。


## 自定义 `derive` 宏

我们来创建一个名为 `hello_macro` 的代码箱，他通过一个名为 `hello_macro` 的关联函数，定义了个名为 `HelloMacro` 的特质。与其让用户为他们的每个类型都实现 `HelloMacro` 特质，我们提供一个过程宏，以便用户可以通过 `[derive(HelloMacro)]` 来注解他们的类型，以获得 `hello_macro` 函数的默认实现。默认实现将打印 `你好，宏！我的名字是 TypeName!`，其中的 `TypeName` 是该特质被定义所在的类型的名字。换句话说，我们将编写一个代码箱，使其他程序员能够编写如下清单 20-37 中的代码。

<a name="listing_20-37"></a>
文件名：`projects/pancakes/src/main.rs`

```rust
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}
```

**清单 20-37**：我们的代码箱用户在使用我们的过程宏时，将能够编写的代码

当我们完成时，这段代码将打印 `你好，宏！我的名字叫 Pancakes！`。第一步是构造一个新的库代码箱，如下所示：

```console
$ cargo new hello_macro --lib
```

接下来，在清单 20-38 中，我们将定义 `HelloMacro` 特质及其关联函数。

<a name="listing_20-38"></a>
文件名：`projects/hello_macro/src/lib.rs`

```rust
pub trait HelloMacro {
    fn hello_macro();
}
```

**清单 20-38**：一个我们将与 `derive` 宏一起使用的简单特质

我们有一个特质及其函数。此时，我们的代码箱用户可以实现这个特质，以实现所需的功能，如下清单 20-39 中所示。

<a name="listing_20-39"></a>
文件名：`src/main.rs`

```rust
use hello_macro::HelloMacro;

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println! ("你好，宏！我的名字叫 Pancakes！");
    }
}

fn main() {
    Pancakes::hello_macro();
}
```

**清单 20-39**：当用户编写 `HelloMacro` 特质的手动实现时，会是什么样子

然而，他们需要针对他们打算与 `hello_macro` 一起使用的每种类型，都编写实现代码块；我们希望省去他们这部分工作。

此外，我们还无法为 `hello_macro` 函数提供将打印对其实现该特质的类型的名字的默认实现：Rust 不具备反射能力，因此无法在运行时查找类型的名字。我们需要一个宏在编译时生成代码。

> **译注**：关于反射能力/机制，请参考：
>
> - [Reflection in Java](https://www.geeksforgeeks.org/java/reflection-in-java/)
>
> - [Java中的反射](https://java.xfoss.com/Ch20_Appendix_B.html#java%E4%B8%AD%E7%9A%84%E5%8F%8D%E5%B0%84)

不过，用户们将需要为各种打算使用 `hello_macro` 特质的类型，编写那个实现的代码块；而咱们原本是要他们免于必须完成这项工作的。

此外，咱们尚不能提供，有着将打印特质被实现在其上类型名字的`hello_macro` 函数默认实现：Rust 没有反射能力，reflection capabilities，因此他无法在运行时查找处那个类型的名字。咱们需要一个宏，从而在编译时生成代码。

下一步是定义过程宏。在撰写本文时，过程宏需要位于自己的代码箱中。最终，这一限制可能会被取消。组织代码箱和组织宏代码箱方面的约定如下：对于名为 `foo` 的代码箱，则自定义的 `derive` 过程宏代码箱应名为 `foo_derive`。我们来在 `hello_macro` 项目内，启动一个名为 `hello_macro_derive` 的新代码箱：

```console
$ cargo new hello_macro_derive
```

这两个代码箱紧密相关，因此我们在 `hello_macro` 代码箱目录下创建这个过程宏代码箱。当我们修改 `hello_macro` 中的特质定义时，也必须修改 `hello_macro_derive` 中的过程宏。这两个代码箱需要单独发布，而使用这两个代码箱的程序员则需要添加他们为依赖项，并带入他们到作用域。我们也可以让 `hello_macro` 代码箱作为依赖项使用 `hello_macro_derive`，并重新导出过程宏的代码。然而，我们组织项目的方式，让程序员即使不想要 `derive` 的功能，也可以使用 `hello_macro`。

我们需要声明 `hello_macro_derive` 代码箱为一个过程宏的代码箱。稍后咱们就会看到，我们还需要 `syn` 和 `quote` 代码箱中的功能，因此我们需要添加他们为依赖。请添加以下内容到 `hello_macro_derive` 的 `Cargo.toml` 文件：

文件名：`projects/hello_macro/hello_macro_derive/Cargo.toml`

```toml
[lib]
proc-macro = true

[dependencies]
syn = "2.0"
quote = "1.0"
```

要开始定义这个过程宏，请要下面清单 20-40 中的代码放入 `hello_macro_derive` 代码箱的 `src/lib.rs` 文件中。请注意，在我们添加 `impl_hello_macro` 函数的定义之前，这段代码不会编译。


<a name="listing_20-40"></a>
文件名：`projects/hello_macro/hello_macro_derive/src/lib.rs`

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // 将 Rust 代码构造为我们可以操作的语法树形式
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // 构建特质实现
    // Build the trait implementation
    impl_hello_macro(&ast)
}
```

**清单 20-40**：大多数过程宏代码箱为了处理 Rust 代码都需要的代码

请注意，我们拆分了代码为两个函数：

- 负责解析 `TokenStream` 的 `hello_macro_derive` 函数，
- 和负责转换语法树的 `impl_hello_macro` 函数。

这样做让编写过程宏更加方便。外层函数（这一情形下的 `hello_macro_derive` ）中的这种代码，对于咱们所见或创建的几乎所有过程宏代码箱，都将是相同的。而咱们在内层函数（这一情形下的 `impl_hello_macro`）的主体指定的代码，将根据过程宏的用途而有所不同。

我们引入了三个新的代码箱：

- `proc_macro`、
- [`syn`](https://crates.io/crates/syn)、
- 和 [`quote`](https://crates.io/crates/quote)。


`proc_macro` 代码箱是 Rust 默认自带的，因此我们无需添加他到 `Cargo.toml` 中的依赖项。`proc_macro` 代码箱属于编译器的 API，允许我们在代码中读取和操作 Rust 代码。

`syn` 代码箱能将字符串形式的 Rust 代码，解析为我们可以对其执行操作的数据结构。`quote` 代码箱则将 `syn` 的数据结构重新转换回 Rust 代码。这两个代码箱大大简化了我们处理各类 Rust 代码的解析工作：编写针对 Rust 代码的完整解析器并非易事。

当我们的库用于在某种类型上指定 `#[derive(HelloMacro)]` 时，`hello_macro_derive` 函数就会被调用。这样做之所以可行，是因为我们在这里以 `proc_macro_derive` 注解了 `hello_macro_derive` 函数，并指定了名字 `HelloMacro`，其与我们的特质名字匹配；这是大多数过程宏遵循的约定。

`hello_macro_derive` 函数首选会将 `input` 从 `TokenStream` 转换为一种数据结构，我们随后对其解析并执行操作。这是 `syn` 发挥作用的地方。`syn` 中的 `parse` 函数会取一个 `TokenStream` 并返回一个 `DeriveInput` 结构体，解析后的 Rust 代码。下面清单 20-41 展示了我们从解析 `struct Pancakes;` 字符串，得到的 `DeriveInput` 结构体的相关部分。

<a name="listing_20-41"></a>
```rust
DeriveInput {
    // --跳过代码--

    ident: Ident {
        ident: "Pancakes",
        span: #0 bytes(95..103)
    },
    data: Struct(
        DataStruct {
            struct_token: Struct,
            fields: Unit,
            semi_token: Some(
                Semi
            )
        }
    )
}
```

**清单 20-41**：我们在解析有着清单 20-37 中宏属性的代码时，得到的 `DeriveInput` 实例

这个结构体的字段表明，我们解析的 Rust 代码是个单元值结构体，有着 `Pancakes` 的 `ident`（*identifier*，即名字）。这个结构体中还有更多字段，用于描述 Rust 的各个方面；请参阅 [ `DeriveInput` 的 `syn` 文档](https://docs.rs/syn/1.0/syn/struct.DeriveInput.html) 了解更多信息。

很快我们将定义 `impl_hello_macro` 函数，其中我们将构建想要包含的新 Rust 代码。但在我们开始之前，请注意 `derive` 宏的输出也是个 `TokenStream`。返回的 `TokenStream` 会被添加到我们的代码箱用户编写的代码中，因此当他们编译自己的代码箱时，他们将获得咱们在修改后的 `TokenStream` 中提供的额外功能。

咱们可能已经注意到，我们调用了 `unwrap`，以在这里的 `syn::parse` 函数调用失败时，引起 `hello_macro_derive` 函数终止运行。我们的过程宏有必要在出现错误时终止运行，因为 `proc_macro_derive` 函数必须返回 `TokenStream` 而不是 `Result`，以符合过程宏 API。我们通过使用 `unwrap` 简化了这个示例；在生产代码中，咱们应该通过使用 `panic!` 或 `expect`，提供有关出错原因的更具体的错误消息。

现在我们有了将注解后的 Rust 代码从 `TokenStream` 转换为 `DeriveInput` 实例的代码，接下来让我们生成对注解的类型实现 `HelloMacro` 特质的代码，如下清单 20-42 中所示。

<a name="listing_20-42"></a>
文件名：`projects/hello_macro/hello_macro_derive/src/lib.rs`

```rust
fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println! ("你好，宏！我的名字叫 {}！", stringify! (#name));
            }
        }
    };
    gen.into()
}
```

**清单 20-42**：使用解析后的 Rust 代码实现 `HelloMacro` 特质

我们使用 `ast.ident` 得到一个 `Ident` 结构体实例，包含被注解类型的名字（标识符）。清单 20-41 中的结构体表明，当我们对清单 20-37 中的代码运行 `impl_hello_macro` 函数时，我们得到的 `ident` 就将有着值为 `"Pancakes"` 的 `ident` 字段。因此，清单 20-42 中的 `name` 变量将包含一个 `Ident` 结构体实例，其在打印出时将是字符串 `"Pancakes"`，即清单 20-37 中结构体的名字。

其中 `quote!` 宏让我们可以定义打算返回的 Rust 代码。编译器期望的类型与 `quote!` 宏执行的直接结果不同，因此我们需要将其转换为 `TokenStream`。我们通过调用 `into` 方法完成这一操作，其会消费中间表示形式，并返回所需的 `TokenStream` 类型的值。

`quote!` 宏还提供了一些非常酷的模板机制：我们可以输入 `#name`，而 `quote!` 将以变量 `name` 中的值替换他。咱们甚至可以像常规宏那样，执行一些重复操作。请参考 [`quote` 代码箱的文档](https://docs.rs/quote) 了解完整的介绍。

我们希望我们的过程宏针对用户注解的类型生成我们的 `HelloMacro` 特质实现，我们可以通过使用 `#name` 获取到用户注解的类型。特质实现有个名为 `hello_macro` 的函数，其函数体包含我们希望提供的功能：打印 `你好，宏！我的名字叫 `，然后是注解的类型的名字。

这里使用的 `stringify!` 宏内置于 Rust。他取一个 Rust 表达式，比如 `1 + 2`，并在编译时转换该表达式为字符串字面值，比如 `"1 + 2"`。这与 `format!` 或 `println!` 不同，二者属于会求值表达式然后转换结果为 `String` 的宏。由于存在 `#name` 可能是个要原样打印的表达式的可能性，因此我们使用 `stringify!`。使用 `stringify!` 还能通过在编译时转换 `#name` 为字符串字面值，从而节省一次内存分配。

此时，在 `hello_macro` 和 `hello_macro_derive` 下的 `cargo build` 都应成功完成。我们来将这两个代码箱连接到清单 20-37 中的代码，看看过程宏的实际操作！使用 `cargo new pancakes`  在咱们的 *projects* 目录下创建一个新的二进制项目。我们需要在 `pancakes` 代码箱的 `Cargo.toml` 中，作为依赖项添加 `hello_macro` 及 `hello_macro_derive`。当咱们把咱们版本的 `hello_macro` 与 `hello_macro_derive` 发布在 [crates.io](https://crates.io/) 上时，他们就属于常规依赖项；而在没有发布时，咱们可以像下面这样指定他们为 `path` 依赖项：

文件名：`projects/pancakes/Cargo.toml`

```toml
hello_macro = { path = "../hello_macro" }
hello_macro_derive = { path = "./hello_macro/hello_macro_derive" }
```

将清单 20-37 中的代码放入 `src/main.rs` 中，并运行 `cargo run`：他应打印出 `你好，宏！我的名字叫 Pancakes！`。过程宏中的 `HelloMacro` 特质实现已被包含，而无需 `pancakes` 代码箱实现他；`#[derive(HelloMacro)]` 添加了该特质的实现。

接下来，我们来探讨其他类别的过程宏与自定义的 `derive` 宏有何不同。


## 类属性的宏

类属性宏与定制派生宏类似，不过与生成 `derive` 属性的代码不同，他们允许咱们创建出新的属性。他们还更灵活：`derive` 只对结构体和枚举生效；而属性则同时可应用到其他项目，比如函数等。下面就是一个使用类属性宏的示例：比方说咱们在运用某个 web 应用框架时，就有一个对函数加以注解的名为 `route` 的属性：

```rust
#[route(GET, "/")]
fn index() {
```

这个 `#[route]` 就将是由那个框架，定义的一个程序性宏。那个宏定义函数的签名，将看起来像下面这样：

```rust
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenSteam {
```

这里，咱们有两个类型 `TokenStream` 的参数。头一个是属性的内容：即 `GET, "/"` 部分。而第二个，则是该属性被附加到的那个项目的函数体：在这个示例中，便是 `fn index() {}` 及该函数的函数体其余部分。

除此之外，类属性宏与定制派生宏以同样方式运作：咱们要创建出一个有着 `proc-macro` 代码箱类型的代码箱，并实现一个生成咱们想要代码的函数！


## 类函数宏

**Function-link macros**


类函数宏定义了看起来像函数调用的宏。与 `macro_rules!` 宏类似，他们比函数更为灵活；比如，他们就可取未知数目的参数。然而，`macro_rules!` 宏只能使用咱们早先在 [用于通用元编程的带有 `macro_rules!` 的声明式宏](#用于通用元编程的带有-macro_rules-的声明式宏) 小节，曾讨论过的 match-like 语法。而类函数宏，则会取一个 `TokenStream` 参数，而这些宏的定义，就会使用 Rust 代码，如同另外两种程序性宏所做的那样，对那个 `TokenStream` 加以操纵。作为类函数宏的一个例子，便是将如下面调用的一个 `sql!` 宏：

```rust
let sql = sql! (SELECT * FROM posts WHERE id=1);
```

这个宏会解析其内部的 SQL 语句，并就其语法方面的正确性加以检查，相比 `macro_rules!` 宏所能完成的处理，这就要复杂多了。这个 `sql!` 宏将像下面这样定义：

```rust
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
```

此定义与定制派生宏的签名类似：咱们会接收圆括号内部的那些令牌，并返回咱们所要生成的代码。


# 本章小结

咦！现在咱们在工具箱中，便有了大概率不会经常用到的一些 Rust 特性，不过咱们会明白，在一些极为特别的情况下他们会是可用的。咱们业已引入几个复杂的主题，因此在咱们于一些错误消息建议，或其他人的代码中遇到他们时，咱们就能识别出这些概念和语法。请将这一章，当作引导咱们得到解决办法的一个参考。


接下来，咱们将把这正本书中曾讨论过的所有内容，投入到实践中，而完成另一个项目！


（End）


