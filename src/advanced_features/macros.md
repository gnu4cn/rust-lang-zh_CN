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
文件名：`src/lib.rs`

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

现在来看看与这个支臂关联的代码体中的模式：对于匹配了模式中 `$()` 的各个部分，根据该模式匹配的次数，`$()*` 里的 `temp_vec.push()` 会被零次或更多次生成。其中的 `$x` 会被各个匹配的表达式替换。当咱们以 `vec! [1, 2, 3];` 调用这个宏时，所生成的替换这个宏的代码，将是下面这样：

```rust
{
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
}
```

咱们就已定义了可取任意数目、任意类型参数，并能生成创建出包含这些特定元素矢量的一个宏了。


要了解更多有关如何编写宏的知识，请参考在线文档或其他资源，比如由 Daniel Keep 起头，Lukas Wirth 续写的 [“Rust 宏小册子”](https://veykril.github.io/tlborm/)。


## 用于从属性生成代码的程序性宏

**Procedural Macros for Generating Code from Attributes**


宏的第二种形式，便是 *程序性宏，procedural macro*，其行事更像函数（而是程序的一种类型，a type of procedure）。程序性宏接收一些代码作为输入，在那些代码上加以操作，并产生作为输出的一些代码，而如同非声明式宏所做的那样，与一些模式匹配并以别的代码替换那些代码。程序性宏的三种类别分别是定制派生宏，custom derive、类属性宏，attribute-like 及类函数宏，function-like，且这三种类别的程序性宏，都以类似方式运作。

在创建程序性宏时，那些定义务必要位处有着特别代码箱名字的他们自己的代码箱中。这是由于咱们（Rust 开发团队）希望在今后消除的一些复杂技术原因。在下面清单 19-29 中，咱们给出了如何定义一个程序性宏的方式，其中 `some_attribute` 是为使用某个特定宏变种的一个占位符。

文件名：`src/lib.rs`

```rust
use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
}
```

*清单 19-29： 定义某个程序性宏的示例*


这个定义了某个宏的函数，会取一个 `TokenStream` 值作为输入，并产生出一个 `TokenStream` 作为输出。`TokenStream` 类型是由 Rust 所包含的 `proc_macro` 代码箱定义，且表示的是一个令牌序列，a sequence of tokens。这个宏的核心如此：该宏在其上操作的源代码，构成了那个输入的 `TokenStream`，而该宏产生的代码，便是那个输出的 `TokenStream`。该函数还有一个附加给他的属性，指出咱们正在创建的是何种的程序性宏。在同一代码箱中，咱们可以有着多种类别的程序性宏。

下面就来看看各种不同类别的程序性宏。咱们将以一个定制的派生宏开始，并于随后探讨令到其他那些宏形式有所区别的一些小差异。


## 怎样编写出定制的 `derive` 宏

**How to Write a Custom `derive` Macro**


咱们就来创建一个名为 `hello_macro` 的宏，这个宏定义了一个名为 `HelloMacro`，有着名为 `hello_macro` 的关联函数的特质。与让咱们的用户为他们的各个类型实现这个 `HelloMacro` 特质不同，咱们将提供一个程序性宏，如此用户就可以 `[derive(HelloMacro)]` 注解他们的类型，从而得到那个 `hello_macro` 函数的默认实现。默认实现将打印出 `你好，宏！我的名字是 TypeName!`，其中的 `TypeName` 是这个特质被定义所在类型的名字。也就是说，咱们将编写一些实现其他编程者编写如下清单 19-30 中用到咱们代码箱的代码。

文件名：`src/main.rs`

```rust
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}
```

当我们完成编写时，此代码将打印 `你好，宏！我的名字叫 Pancakes！`。第一步是要构造一个新的库代码箱，像下面这样：

```console
$ cargo new hello_macro --lib --vcs none
```

接下来，咱们将定义那个 `HelloMacro` 特质及其关联函数：

文件名：`src/lib.rs`

```rust
pub trait HelloMacro {
    fn hello_macro();
}
```

咱们就有了一个特质及其函数。到这里，咱们代码箱的用户就可以实现这个特质来达成所需功能，像下面这样：

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

不过，用户们将需要为各种打算使用 `hello_macro` 特质的类型，编写那个实现的代码块；而咱们原本是要他们免于必须完成这项工作的。

此外，咱们尚不能提供，有着将打印特质被实现在其上类型名字的`hello_macro` 函数默认实现：Rust 没有反射能力，reflection capabilities，因此他无法在运行时查找处那个类型的名字。咱们需要一个宏，从而在编译时生成代码。

下一步就是要定义这个程序性宏。在编写这个小节的时候，程序性宏是需要在他们自己的代码箱中的。最终这个限制可能会被消除。代码箱的结构组织与宏代码箱方面的约定如下：对于名为 `foo` 的代码箱，那么定制派生程序性宏代码箱就会叫做 `foo_derive`。下面就在咱们的 `hello_macro` 项目内，开启一个名为 `hello_macro_derive` 的新代码箱：

```console
$ cargo new hello_macro_derive --lib --vcs none
```

咱们的这两个代码箱是密切相关的，因此咱们是在咱们的 `hello_macro` 代码箱目录下，创建的这个程序性宏代码箱。而若咱们修改了 `hello_macro` 中的特质定义，咱们就将不得不也要修改 `hello_macro_derive` 中那个程序性宏。两个代码箱将需要单独发布，且使用这两个代码箱的程序员，将需要将二者都添加为依赖，并同时把他们都带入到作用域。相反，咱们可以让 `hello_macro` 代码箱，将 `hello_macro_derive` 作为依赖使用，并重导出这些程序性宏的代码。然而，咱们阻止结构该项目的这种方式，会让那些不想要 `derive` 功能的程序员，也可以使用 `hello_macro`。

咱们需要将 `hello_macro_derive` 代码箱，声明为程序性宏的代码箱。如同马上就会看到的那样，咱们还需要来自 `syn` 与 `quote` 代码箱的功能，，因此咱们就需要将他们添加为依赖。请将下面的配置，添加到 `hello_macro_derive` 的 `Cargo.toml` 文件：

```toml
[lib]
proc-macro = true

[dependencies]
syn = "1.0"
quote = "1.0"
```

要开始定义这个程序性宏，就要将下面清单 19-31 中的代码，放置于 `hello_macro_derive` 代码箱的 `src/lib.rs` 文件中。请注意在咱们添加了 `impl_hello_macro` 函数定义前，此代码不会编译。


文件名：`hello_macro_derive/src/lib.rs`

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // 以语法树形式，构建出咱们可操作 Rust 代码的表示
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // 构造出这个特质实现
    impl_hello_macro(&ast)
}
```

*清单 19-31：多数程序性宏为处理 Rust 代码而都需要的代码*

请注意咱们已经代码分解到 `hello_macro_derive` 函数中，由其负责解析那个 `TokenStream`，而其中的 `impl_hello_macro` 函数，则负责转换那个语法树：这样做令到编写程序性宏更为方便。对于几乎每个咱们所见到的或创建的程序性宏，外层函数（此示例中的 `hello_macro_derive`）中的代码将是一致的。而咱们在那个内层函数（此示例中的 `impl_hello_macro`）中指定的代码，将依据咱们程序性宏目的而有所不同。

咱们引入了三个新的代码箱：`proc_macro`、[`syn`](https://crates.io/crates/syn) 与 [`quote`](https://crates.io/crates/quote)。`proc_macro` 代码箱是 Rust 自带的，因此咱们无需将其添加到 `Cargo.toml` 的依赖。`proc_macro` 代码箱，是实现从咱们的代码读取及操作 Rust 代码的编译器 API。

`syn` 代码箱会从一个字符串将 Rust 代码解析为咱们可在其上执行操作的一种数据结构。而 `quote` 代码箱，则会将 `syn` 数据结构，转换回 Rust 代码。这些代码箱令到解析任何一种咱们打算处理的 Rust 代码更为容易：编写出 Rust 代码的完整解析器，并非易事。

这个 `hello_macro_derive` 函数，将在咱们的库用户，于某个类型上指明 `#[derive(HelloMacro)]` 时被调用。这样做之所以可行，是由于咱们已使用 `proc_macro_derive` 注解了这里的 `hello_macro_derive` 函数，并指定了于咱们的特质名字相符的名字 `HelloMacro`；而这正是多数程序性宏所遵循的约定。

这个 `hello_macro_derive` 函数首选会将那个 `input`，从一个 `TokenStream` 转换为咱们随后可以解读并于其上操作的一种数据结构。这正是 `syn` 发挥作用之处。`syn` 中的 `parse` 函数，会取一个 `TokenStream` 并返回一个表示解析出 Rust 代码的 `DeriveInput` 数据结构。下面清单 19-32 给出了咱们对 `struct Pancakes;` 字符串进行解析而得到的 `DeriveInput` 数据结构的有关部分：

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

*清单 19-32：在对清单 19-30 中有着该宏属性的代码进行解析时咱们所得到的 `DeriveInput` 实例*


这个结构体的那些字段显示，咱们所解析的 Rust 是个有着 `Pancakes` 的 `ident`（标识符，意为名字）的一个单元结构体，a unit struct。此结构体上还有一些用于描述 Rust 各个方面的其他字段；请参阅 [有关 `DeriveInput` 的 `syn` 文档](https://docs.rs/syn/1.0/syn/struct.DeriveInput.html) 了解更多信息。

很快咱们就将实现那个 `impl_hello_macro` 函数，其中咱们将构建出咱们所打算包含的新 Rust 代码。但在咱们实现之前，请注意咱们的派生宏输出，同样是个 `TokenStream`。这个返回的 `TokenStream` 会添加到咱们代码箱用户编写的代码，因此当他们编译他们的代码箱时，他们将获得咱们在这个修改的 `TokenStream` 中所提供的额外功能。

咱们或许已经留意到，咱们调用了 `unwrap`，来在这里的到 `syn::parse` 函数调用失败时，造成那个 `hello_macro_derive` 函数终止运行。由于 `proc_macro_derive` 函数必须返回 `TokenStream`，而非 `Result` 来顺应程序性宏的 API，因此咱们的程序性宏就要在出错时终止运行。咱们已通过使用 `unwrap` 简化了这个示例；在生产代码中，咱们应通过运用 `panic!` 或 `expect`，提供有关那些东西出错的更具体的错误消息。

既然咱们有了将经注解的 Rust 代码，从一个 `TokenStream` 转换为一个 `DeriveInput` 实例的代码，那么就要生成在被注解类型上实现这个 `HelloMacro` 特质的代码，如下清单 19-33 中所示。

文件名：`hello_macro_derive/src/lib.rs`

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

*清单 19-33：是要解析出的 Rust 代码，实现这个 `HelloMacro` 特质*

通过使用 `ast.ident`，咱们得到了一个包含着受注解类型名字（标识符）的 `Ident` 结构体实例。清单 19-32 中的代码结构，显示当咱们在清单 19-30 中的代码上运行这个 `impl_hello_macro` 函数时，咱们得到的这个 `ident` 就将有着值为有一个 `"Pancakes"` 值的 `ident` 字段。因此，清单 19-33 中的 `name` 变量，就将包含一个在被打印出时，将为字符串 `"Pancakes"`，即清单 19-30 中那个结构体名字的 `Ident` 结构体。

其中的 `quote!` 宏，允许咱们定义出咱们打算返回的 Rust 代码。编译器会期望得到不同于这个 `quote!` 宏直接执行结果的东西，因此咱们就要将其转换为一个 `TokenStream`。咱们是通过调用的那个消费这个中间表示，并返回所需的 `TokenStream` 类型的一个值的 `into` 方法，完成这一点的。

`quote!` 宏还提供了一些非常酷的模板机制：咱们可以敲入 `#name`，而 `quote!` 就将使用变量 `name` 中的值，替换掉他。咱们甚至可以与宏工作类似方式，完成一些重复操作。请参考 [`quote` 代码箱文档](https://docs.rs/quote) 了解完整信息。

咱们是要这个程序性宏，在用户注解的类型上，生成咱们的 `HelloMacro` 特质实现，而咱们可通过使用 `#name` 做到这点。这个特质实现，有着一个名为 `hello_macro` 的函数，其函数体包含了咱们打算提供的功能：打印 `你好，宏！我的名字叫` 以及随后的那个受注解类型的名字。

这里用到的那个 `stringify!` 宏，是内建于 Rust 中的。他会取一个 Rust 表达式，比如 `1 + 2`，并在编译时将这个表达式转换为字符串字面值，比如 `"1 + 2"`。这与 `format!` 或 `println!` 这样的会执行表达式并随后将结果转换为一个 `String` 的宏不同。由于存在着那个 `#name` 输入，为一个要打印出字面值的表达式的可能，因此咱们便使用了 `stringify!`。使用 `stringify!` 还通过在编译时将 `#name` 转换为字符串字面值，而节省了一次内存分配。


到这里，在 `hello_macro` 与 `hello_macro_derive` 中，`cargo build` 都应完全成功。让我们来将这两个代码箱，连接到清单 19-30 中的代码，来看看行动中的程序性宏！在咱们的 `projects` 目录下，使用 `cargo new derive_macro_comsumer --vcs none` 创建一个新的二进制项目。咱们需要在这个 `derive_macro_comsumer` 代码箱的 `Cargo.toml` 中，把 `hello_macro` 及 `hello_macro_derive` 添加为依赖项。若咱们把咱们版本的 `hello_macro` 与 `hello_macro_derive` 发布在了 [crates.io](https://crates.io/)，那么他们将为一些常规依赖；而在没有发布时，咱们可以像下面这样，将他们指定为 `path` 的依赖：

```toml
hello_macro = { path = "../hello_macro" }
hello_macro_derive = { path = "./hello_macro/hello_macro_derive" }
```

请将清单 19-30 中的代码，放入到 `src/main.rs` 中，并运行 `cargo run`：其应打印出 `你好，宏！我的名字叫 Pancakes！` 在这个 `derive_macro_comsumer` 代码箱无需实现那个程序性宏中的 `HelloMacro` 特质下，该特质的实现就已被包含了；正是 `#[derive(HelloMacro)]` 添加了这个特质实现。

接下来，咱们要探讨其他类别的程序性宏，与定制派生宏有怎样的不同。


## 类属性宏

**Attribute-like macros**


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


