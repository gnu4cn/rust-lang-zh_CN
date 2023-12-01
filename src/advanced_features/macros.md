# 关于宏

**Macros**


贯穿这本书，咱们业已用到像是 `println!` 这样的宏，但咱们并未完整地探讨过何为宏，以及其工作原理。 *宏，macro* 这个术语，指的是 Rust 中的一个特性家族：有着 `macro_rules!` 的 *声明式，declarative* 宏，与如下三种 *程序性，procedural* 宏：

- 指明一些在结构体及枚举上以 `derive` 属性添加代码的 **定制 `#[derive]` 的宏**，custome `#[derive]` macros that specify code added with the `derive` attribute used on structs and enums；
- 定义出一些可在任何项目上使用的一些定制属性的 **类属性宏**，attribute-like macros that define custom attributes usable on any item；
- 看起来像函数调用，但是在一些指定为其参数的令牌上操作的 **类函数宏**，function-like macros that look like function calls but operate on the tokens specified as their argument。

咱们将逐个讲到这每个的宏，但首先来看看，为何在已有函数的情况下，咱们还需要宏？


> **注**：宏似乎与 Java 及 Python 等语言中的装饰器类似？


## 宏与函数的区别

**The Difference Between Macros and Functions**


根本上讲，宏是一种编写其他代码的代码编写方式，这种方式被称作 *元编程，metaprogramming*。在附录 C 中，咱们会讨论那个 `derive` 属性，其会为咱们生成各种特质的实现。遍布这本书，咱们也已用到了 `println!` 与 `vec!` 两个宏。全部这些宏，都会 *展开，expand* 来产生相比于咱们手写代码更多的代码。

对于降低咱们所必须编写与维护代码量，元编程是有用的，这也是函数的角色之一。但是，宏有着函数所没有的一些额外能力。

函数签名必须要声明该函数所有的参数个数与类型。而另一方面的宏，则可以取数目不定的参数：咱们可以一个参数调用 `println! ("你好")`，或以两个参数调用 `println! ("你好 {}", name)`。同时，宏是在编译器对代码的意义加以解译之前展开的，因此宏就可以，比如在给到他的类型上实现某个特质。由于函数是在运行时被调用的，而特质需要在编译时被实现，故函数没办法做到这点。

实现宏而非函数的缺点，就是因为咱们是在编写那些编写出 Rust 代码的代码，所以宏定义要比函数定义更为复杂。由于这种间接性，相比于函数定义，宏定义一般都更难阅读、理解及维护。

宏与函数的另一重要区别，便是咱们必须于某个文件中调用宏 *之前*，定义好他们或将他们带入到作用域中，这一点与可在任何地方定义并在任何地方调用的函数相反。


## 用于通用元编程的带有 `macro_rules!` 的声明式宏

**Declarative Macros with `macro_rules!` for General Metaprogramming**


Rust 中使用最广泛的宏形式，就是 **声明式宏，declarative macro**。这些宏有时也被指为 “示例性宏，macros by example”，“`macro_rules!` 宏”，或仅被指为 “宏，macros”。声明式宏的核心，便是实现编写出类似于 Rust `match` 表达式的一些东西来。正如在第 6 章中曾讨论过的，`match` 表达式是取一个表达式、将该表达式计算结果值与一些模式比较，而在随后返回与匹配模式相关联代码的一些控制结构。宏也会把某个值与一些与特定代码相关的模式比较：在这种情形下，那个值便是传被递给宏的字面 Rust 源代码；一些模式就与那源代码比较；而与各个模式关联的代码，在匹配上时，就会替换传递给该宏的代码。这全部都是在编译器期间发生的。

要定义宏，就要用到 `macro_rules!` 结构体下面就通过看看 `vec!` 宏是如何定义的，来探讨一下怎样使用这个 `macro_rules!`。第 8 张曾涉及到咱们可以如何使用 `vec!` 宏，来创建出有着一些特定值的新矢量。比如，下面的红会创建出一个包含三个整数的新矢量值：

```rust
let v: Vec<u32> = vec! [1, 2, 3];
```

咱们也可以使用 `vec!` 宏，构造出两个整数的矢量值，或是五个字符串的矢量值。由于咱们预先不会知道值数目和类型，因此是无法使用函数完成这同样事情的。

下面清单 19-28 给出了稍微简化后的 `vec!` 宏的定义。

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

*清单 19-28：`vec!` 宏定义的简化版本*

> 注意：标准库中 `vec!` 宏的具体定义，包含了预先分配正确数量内存的代码。在这里咱们为了令到这个示例更为简单，而并未包含那些属于优化的代码。


其中的 `#[macro_export]` 注解，表明当这个宏被定义的代码箱，被带入到作用域的时候，这个宏就应成为可用。若没有这个注解，那么该宏就无法被带入到作用域。

随后咱们以 `macro_rules!` 及 *不带* 感叹号的咱们正定义宏的名字，开始该宏的定义。在此示例总，名字即为 `vec`，其后跟着表示宏定义代码体，the body of the macro definition, 的一对花括号。

`vec!` 宏代码体中的结构，与 `match` 表达式的结构类似。在这里咱们有着一个带有模式 `( $( $x:expr ),* )`，跟着 `=>` 及与这个模式关联代码块的支臂。在该模式匹配是，那个关联代码块将被运行，be emitted。鉴于这是这个宏中的唯一支臂，那么就只有一种要匹配有效方式；任何其他模式都将导致报错。那些更为复杂的宏，则将有着多于一个的支臂。

由于宏的那些模式，始于 Rust 代码结构而非一些值相匹配的，因此宏定义中有效的模式语法，不同于第 18 章中所涉及的模式语法。咱们来看看，清单 19-28 中各个模式片段，分别表示什么；对于宏的完整模式语法，请参见 [Rust 参考手册](https://doc.rust-lang.org/reference/macros-by-example.html)。

首选，咱们使用了一对圆括号，把整个模式包括起来。咱们使用一个美元符号（`$`），来声明出在宏系统中的，一个将要包含与这个模式匹配的 Rust 代码的变量，we use a dollar sign(`$`) to declare a variable in the macro system that will contain the Rust code matching the pattern。这个美元符号明确了这是个宏变量，而非一个常规 Rust 变量。接下来是捕获用于替换代码中的，与圆括号中模式匹配的那些值的一对圆括号，next comes a set of parentheses that captures values that match the pattern within the parentheses for use in the replacement code。在 `$()` 里的，为 `$x:expr`，这会与任意 Rust 表达式匹配，并把那个表达式命名为 `$x`。

`$()` 之后的逗号，表明在匹配 `$()` 中代码的代码之后，可选择性地出现一个字面的逗号分隔符。那个 `*` 指出了该模式会与零个或更多的 `*` 之前的东西匹配。

当咱们以 `vec! [1, 2, 3];` 调用这个宏时，`$x` 就会分别与表达式 `1`、`2` 与 `3` 匹配三次。

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
