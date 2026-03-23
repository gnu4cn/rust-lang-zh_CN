# 以生命周期验证引用

生命周期属于另一种我们已在使用的泛型。与确保类型有着我们所期望的行为不同，生命周期确保引用在咱们需要他们的整个期间内都有效。

我们在第 4 章中的 [引用与借用](../ownership/references_and_borrowing.md) 小节中未曾讨论的一个细节是，Rust 中的每个引用都有一个生命周期，即该引用有效的作用域。大多数时候，生命周期都是隐式的和推导出的，就像大多数时候类型是推导出的一样。只有当存在多种可能的类型时，我们才需要注解类型。以类似方式，当引用的生命周期可以数种方式关联时，我们必须注解生命周期。Rust 要求我们使用泛型声明周期参数注解关系，以确保运行时使用的实际引用将肯定有效。

注解生命周期甚至都不是绝大多数其他编程语言所有的概念，因此这会让人感到陌生。尽管我们在这一章中不会介绍生命周期的全部，但咱们将讨论咱们可能会遇到生命周期语法的常见方式，以便咱们可以适应这个概念。


## 悬空引用

生命周期的主要目的是防止悬空引用，dangling references，若允许他们存在，将导致程序引用与其预期不同的数据。请考虑下面清单 10-16 中的程序，他有一个外层作用域和一个内层作用域。


<a name="listing_10-16"></a>
```rust
fn main() {
    let r;

    {
        let x = 5;
        r = &x;
    }

    println! {"r: {r}"};
}
```

**清单 10-16**：尝试使用其值已超出作用域的引用

> **注意**：清单 10-16、10-17 及 10-23 中的示例均在没有赋予变量初始值下声明了变量，因此变量名字存在于外层作用域中。乍一看，这似乎与 Rust 没有空值相冲突。然而，当我们在赋值前尝试使用变量，我们将得到一个编译器时错误，这表示 Rust 确实不允许空值。

外层作用域声明了个名为 `r` 的变量，没有初始值，内层作用域声明了个名为 `x` 的变量，有着初始值 `5`。在内层作用域内，咱们尝试将 `r` 的值设置为到 `x` 的引用。然后，内层作用域结束，我们尝试打印 `r` 中的值。这段代码将不编译，因为在我们尝试使用 `r` 所引用的值前，他已超出了作用域。下面是错误消息：

```console
$ cargo run
   Compiling dangling_ref_demo v0.1.0 (/home/hector/rust-lang-zh_CN/projects/dangling_ref_demo)
error[E0597]: `x` does not live long enough
 --> src/main.rs:6:13
  |
5 |         let x = 5;
  |             - binding `x` declared here
6 |         r = &x;
  |             ^^ borrowed value does not live long enough
7 |     }
  |     - `x` dropped here while still borrowed
8 |
9 |     println! {"r: {r}"};
  |                    - borrow later used here

For more information about this error, try `rustc --explain E0597`.
error: could not compile `dangling_ref_demo` (bin "dangling_ref_demo") due to 1 previous error
```

错误消息指出，变量 `x` “does not live long enough. （未存活足够长时间）” 原因是 `x` 将在第 7 行处内层作用域结束时超出作用域。但 `r` 对于外层作用域仍有效；由于其作用域更大，我们说他 “存活时间更长”。若 Rust 允许这段代码工作，`r` 将引用 `x` 超出作用域时解除分配的内存，而尝试对 `x` 执行的任何操作都将不会正确地工作。那么，Rust 是怎样判断这段代码无效的呢？他使用借用检查器。


> **译注**：Rust 使作用域成为可定量计算、可参数化。


## 借用检查器

Rust 编译器有个 *借用检查器，borrow checker*，会比较作用域以确定所有借用是否有效。下面清单 10-17 显示了与清单 10-16 相同的代码，但带有展示变量生命周期的注解。

<a name="listing_10-17"></a>
```rust
fn main() {
    let r;              // ---------+-- 'a
                        //          |
    {                   //          |
        let x = 5;      // -+-- 'b  |
        r = &x;         //  |       |
    }                   // -+       |
                        //          |
    println!("r: {r}"); //          |
}                       // ---------+
```

**清单 10-17**：`r` 与 `x` 的生命周期注解，分别命名为 `'a` 和 `'b`

在这里，咱们以 `'a` 注解了 `r` 的声明周期，以 `'b` 注解了 `x` 的生命周期。正如咱们所见，相比外层的 `'a` 声明周期块，内层的 `'b` 块小得多。在编译时，Rust 会比较这两个生命周期的大小，发现 `r` 有着 `'a` 的生命周期，但他引用了有着 `'b` 生命周期的内存。这个程序被拒绝，因为 `'b` 短于 `'a`：引用主体存活时间短于引用。

下面清单 10-18 修复了该代码，使其没有悬空引用进而其会在没有任何错误下编译。


<a name="listing_10-18"></a>
```rust
fn main() {
    let x = 5;          // ----------+-- 'b
                        //           |
    let r = &x;         // --+-- 'a  |
                        //   |       |
    println!("r: {r}"); //   |       |
                        // --+       |
}                       // ----------+
```

**清单 10-18**：有效引用，因为数据有着比引用更长的生命周期

这里，`x` 有着生命周期 `'b`，在这一情形下大于 `'a`。这意味着 `r` 可以应用 `x`，因为 Rust 知道当 `x` 有效时，`r` 中的引用将始终有效。

现在咱们知道引用的生命周期在何处以及 Rust 怎样分析生命周期以确保引用将始终有效，下面我们来探讨函数参数与返回值中的泛型生命周期。


## 函数中的泛型生命周期

我们将编写一个函数，返回两个字符串切片中较长的那个。这个函数将取两个字符串切片，并返回单个字符串切片。在我们实现 `longest` 函数后，下面清单 10-19 中的代码应打印 `最长的字符串为 abcd`。


<a name="listing_10-19"></a>
文件名：`src/main.rs`

```rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println! ("最长的字符串为 {result}");
}
```

**清单 10-19**：调用 `longest` 函数来找出两个字符串切片中较长的那个的 `main` 函数

请注意，我们希望这个函数取字符串切片，他们属于引用而不是字符串，因为我们不希望 `longest` 函数取得其参数的所有权。请参阅第 4 章中的 [字符串切片作为参数](../ownership/the_slice_type.md#字符串切片作为参数) 小节，了解更多有关为何我们在清单 10-19 中使用的参数，正是我们想要的讨论。

当咱们尝试实现如下面清单 10-20 中所示的 `longest` 函数时，其将不编译。


<a name="listing_10-20"></a>
文件名：`src/main.rs`

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}
```

**清单 10-20**：一种 `longest` 函数的实现，返回两个字符串切片中较长的那个，但尚不会编译


相反，咱们会得到以下提到生命周期的报错：


```console
$ cargo run
   Compiling generic_lifetimes_demo v0.1.0 (/home/hector/rust-lang-zh_CN/projects/generic_lifetimes_demo)
error[E0106]: missing lifetime specifier
 --> src/main.rs:9:33
  |
9 | fn longest(x: &str, y: &str) -> &str {
  |               ----     ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
help: consider introducing a named lifetime parameter
  |
9 | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  |           ++++     ++          ++          ++

For more information about this error, try `rustc --explain E0106`.
error: could not compile `generic_lifetimes_demo` (bin "generic_lifetimes_demo") due to 1 previous error
```

帮助文本揭示，返回类型需要其上的泛型生命周期，因为 Rust 无法区分返回的引用指向 `x` 还是 `y`。实际上，我们也不知道，因为这个函数主体中的`if` 代码块返回一个到参数 `x` 的引用，而 `else` 代码块返回一个到 `y` 的引用！

当咱们定义这个函数时，我们不知道将传入这个函数的具体值，因此我们不知道`if` 情况将执行，还是 `else` 情况将执行。我们也不知道将传入的引用的具体生命周期，因此我们无法像我们在 [清单 10-17](#listing_10-17) 与 [清单 10-18](#listing_10-18) 中那样，查看作用域来确定我们返回的引用将是否始终有效。借用检查器也无法确定这点，因为他不知道 `x` 与 `y` 的生命周期如何与返回值的生命周期关联。为了修复这个错误，我们将添加定义引用之间关系的泛型生命周期参数，以便借用检查器可以执行其分析。


## 生命周期注解语法

生命周期注解不会改变任何引用的存活时间。相反，他们在不影响生命周期下，描述了多个引用的生命周期相互之间的关系。正如在签名指定了泛型参数后函数可以接受任何类型一样，函数可以通过指定泛型生命周期参数接受带有任何生命周期的引用。

生命周期注解有着些许不同寻常的语法：生命周期参数的名字必须以撇号（单引号，`'`）开头，通常都是小写，并且像泛型一样非常短。大多数人使用名字 `'a` 作为第一个生命周期注解。我们将生命周期注解放在引用的 `&` 之后，使用空格将这种注解与引用的类型分开。

下面是一些示例 -- 到没有生命周期参数的 `i32` 的引用、到有着名为 `'a` 的生命周期参数的 `i32` 的引用，以及到同样有着生命周期 `'a` 的 `i32` 的可变引用。

```rust
&i32        // 引用
&'a i32     // 带有显式生命周期的引用
&'a mut i32 // 带有显式生命周期的可变引用
```

一个生命周期租借本身没有太大意义，因为注解的目的是告诉 Rust，多个引用的泛型生命周期参数相互之间是如何关联的。咱们来在 `largest` 函数的上下文中，检查生命周期注解相互之间的关联方式。


## 函数签名方面

要在函数签名中使用生命周期注解，我们需要在函数名字与参数列表之间的尖括号内声明泛型生命周期参数，就像我们对泛型类型参数所做的那样。

我们希望签名表达以下约束：返回的引用将在两个参数都有效期间有效。这就是参数的生命周期与返回值的之间的关系。咱们将命名生命周期为 `'a`，然后添加他到每个引用，如下清单 10-21 中所示。

<a name="listing_10-21"></a>
文件名：`src/main.rs`

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

**清单 10-21**：`longest` 函数的定义，指定签名中的所有引用都必须有着同一生命周期 `'a`


当我们与清单 10-19 的 `main` 函数一起使用这段代码时，他应编译并产生我们想要的结果。

函数签名现在告诉 Rust，针对某一生命周期 `'a`，该函数取两个参数，他们都是字符串切片，存活时间至少与生命周期 `'a` 一样长。这个函数签名还告诉 Rust，从该函数返回的字符串切片将存活至少与生命周期 `'a` 一样长。实际上，这意味着由 `longest` 函数返回的引用的生命周期，与由函数参数引用的值的生命周期中较小的相同。这些关系是我们希望 Rust 在分析这段代码时要使用的。

请记住，当咱们在这个函数签名中指定生命周期参数时，咱们没有改变任何传入值或返回值的生命周期。相反，咱们是在指定借用检查器应该拒绝任何未遵守这些约束的值。请注意，`longest` 函数不需要确切地知道 `x` 与 `y` 将存活多长时间，只要有某一作用域可替代 `'a` 即将满足这一签名。

> **译注**：这里原文难以理解：
>
> "Remember, when we specify the lifetime parameters in this function signature, we’re not changing the lifetimes of any values passed in or returned. Rather, we’re specifying that the borrow checker should reject any values that don’t adhere to these constraints. Note that the longest function doesn’t need to know exactly how long x and y will live, only that some scope can be substituted for 'a that will satisfy this signature."

在函数中注解生命周期时，注解位于函数签名中，而非函数主体中。生命周期注解成为函数合约的一部分，就像是签名中的类型一样。让函数签名包含生命周期合约，意味着 Rust 编译器执行的分析会更简单。当函数注解的方式或调用的方式存在问题时，编译器报错可以更精准地指向我们的代码部分和约束。相反，当 Rust 编译器对我们想要的生命周期关系做出更多推断时，那么编译器或许只能够指出在问题原因处许多步之外的咱们代码的某一用法。

当咱们传递具体引用给 `longest` 时，替换 `'a` 的具体生命周期的即为 `x` 的作用域与 `y` 的作用域重叠的部分。换句话说，泛型生命周期 `'a` 将获得与 `x` 与 `y` 的生命周期中较小者相等的具体生命周期。因为我们已通过同一生命周期参数 `'a` 注解了返回的引用，所以返回的引用也将在 `x` 与 `y` 的生命周期中较小者的长度内有效。

我们来看看生命周期注解怎样通过传入有着不同具体生命周期的引用限制 `longest` 函数。下面清单 10-22 是个直观的示例。

<a name="listing_10-22"></a>
文件名：`src/main.rs`

```rust
fn main() {
    let string1 = String::from("长字符串就是长");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println! ("最长字符串为 {result}");
    }
}
```

**清单 10-22**：以到具有不同具体生命周期的 `String` 类型值的一些引用，使用 `longest` 函数

在这个示例中，`string1` 在外层作用域结束之前有效，`string2` 在内层作用域结束之前有效，而 `result` 引用在内层作用域结束之前有效的内容。运行这段代码，咱们将看到借用检查器批准；他将编译并打印 `最长字符串为 长字符串就是长`。

接下来，我们来尝试一个实例，展示 `result` 中的引用的生命周期必须是两个参数中较小的生命周期。我们将把 `result` 变量的声明移出内层作用域，而把对 `result` 变量的赋值留在有着 `string2` 变量的作用域内。然后，我们将把使用 `result` 的 `println!` 语句移出内层作用域，在内层作用域结束之后。下面清单 10-23 中的代码将不编译。


<a name="listing_10-23"></a>
文件名：`src/main.rs`

```rust
fn main() {
    let string1 = String::from("长字符串就是长");
    let result;

    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println! ("最长字符串为 {result}");
}
```

**清单 10-23**：在 `string2` 已超出作用域后尝试使用 `result`


当我们尝试编译这段代码时，我们会得到下面这个报错：


```console
$ cargo run
   Compiling generic_lifetimes_demo v0.1.0 (/home/hector/rust-lang-zh_CN/projects/generic_lifetimes_demo)
error[E0597]: `string2` does not live long enough
 --> src/main.rs:7:44
  |
6 |         let string2 = String::from("xyz");
  |             ------- binding `string2` declared here
7 |         result = longest(string1.as_str(), string2.as_str());
  |                                            ^^^^^^^ borrowed value does not live long enough
8 |     }
  |     - `string2` dropped here while still borrowed
9 |     println! ("最长字符串为 {result}");
  |                              ------ borrow later used here

For more information about this error, try `rustc --explain E0597`.
error: could not compile `generic_lifetimes_demo` (bin "generic_lifetimes_demo") due to 1 previous error
```

这一报错表明，要让 `result` 对 `println!` 语句有效，`string2` 将需要在外层作用域结束前有效。Rust 之所以知道这点，是因为我们使用同一生命周期参数 `'a` 注解了该函数的参数和返回值。

作为人类，我们可以查看这段代码，发现 `string1` 长于 `string2`，因此 `result` 将包含到 `string1` 的引用。因为 `string1` 尚未超出作用域，所以到 `string1` 的引用对于 `println!` 语句仍将有效。但是，编译器无法看出看出这一情形下该引用是有效的。我们已经告诉 Rust，由 `longest` 函数返回的引用的生命周期与传入的引用的生命周期中较小的那个相同。因此，借用检查器未放行清单 10-23 中的代码，因为可能有无效引用。

请尝试设计更多实验，让传入 `longest` 函数的引用的值和生命周期，以及返回的引用的使用方式上各不相同。在咱们编译前，要就咱们的试验是否会通过借用检查器提出假设；然后，检查一下咱们是否正确！


## 关于关系

我们需要以何种方式指定生命周期参数，取决于咱们的函数正在做什么。例如，当我们修改 `longest` 函数的实现为始终返回第一个参数，而非最长的字符串切片，那么我们将不需要在参数 `y` 上指定生命周期。以下代码将编译：

文件名：`src/main.rs`

```rust
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```

咱们已对参数 `x` 与返回类型指定了生命周期参数 `'a`，而未对参数 `y` 指定，因为 `y` 的生命周期与 `x` 或返回值的生命周期没有任何关系。

在从函数返回引用时，返回类型的生命周期参数需要与参数之一的生命周期参数一致。当返回的引用 *未* 指向参数之一时，其必定会指向这个函数内部创建的某个值。但是，这将是个悬空引用，因为该值将在函数结束处超出作用域。请考虑下面这个将不编译的 `longest` 函数的尝试实现：

文件名：`src/main.rs`

```rust
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("真正长的字符串");
    result.as_str()
}
```

在这里，尽管咱们已对返回类型指定了生命周期参数 `'a`，这一实现仍将无法编译，因为返回值的生命周期与参数的生命周期完全无关。下面是我们得到的错误消息：

```console
$ cargo run
   Compiling generic_lifetimes_demo v0.1.0 (/home/hector/rust-lang-zh_CN/projects/generic_lifetimes_demo)
error[E0515]: cannot return value referencing local variable `result`
  --> src/main.rs:14:5
   |
14 |     result.as_str()
   |     ------^^^^^^^^^
   |     |
   |     returns a value referencing data owned by the current function
   |     `result` is borrowed here

For more information about this error, try `rustc --explain E0515`.
error: could not compile `generic_lifetimes_demo` (bin "generic_lifetimes_demo") due to 1 previous error
```

问题在于 `result` 会在 `longest` 函数结束处超出作用域而被清理掉。我们还试图从该函数返回到 `result` 的引用。咱们无法指定任何能改变悬空引用的生命周期参数，而 Rust 将不会让我们创建悬空引用。在这种情况下，最佳的修复方案将是返回一个自有的数据类型而非引用（译注：这样看来引用是没有所有权的），以便调用函数随后负责清理该值。

最终，生命周期语法是关于连接函数的不同参数与返回值的生命周期。一旦他们联系起来，Rust 就有足够信息来实现内存安全的操作，并禁止会创建悬空指针或以其他方式违反内存安全的操作。


## 结构体定义方面

到目前为止，我们已定义的结构体都存储的是自有类型。我们可以定义结构体为存储引用，但在这种情况下，我们就需要在结构体定义中的每个引用上添加生命周期注解。下面清单 10-24 有个名为 `ImportedExcerpt` 的结构体，存储了一个字符串切片。

<a name="listing_10-24"></a>
文件名：`src/main.rs`

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("请叫我 Ishmael。数年前.....");
    let first_sentence = novel.split('。').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
```

**清单 10-24**：存储引用的结构体，需要生命周期注解

此结构体拥有保存着是个引用的字符串切片的单一字段 `part`。与通用数据类型（泛型），generic data types，下一样，咱们在结构他名字后的尖括号里，声明了通用声明周期参数，进而就可以在结构体定义代码体中，使用那个生命周期参数。这个注解表示，`ImportantExcerpt` 的实例，无法存活超过其在 `part` 字段中所保存的那个引用，this annotation means an instance of `ImportedExcerpt` can't outlive the reference it holds in its `part` field。

这里的 `main` 函数会创建出 `ImportantExcerpt` 结构体的，保存着到由变量 `novel` 拥有的 `String` 的第一个句子引用的一个示例。`novel` 中的数据在这个 `ImportantExcerpt` 实例被创建之前就存在了。此外，在这个 `ImportantExcerpt` 超出作用域之前，`novel` 不会超出作用域，因此这个 `ImportantExcerpt` 实例中的引用是有效的。


## 生命周期的省略

咱们已经了解到每个引用都有生命周期，以及需要给使用了引用的函数与结构体，指明生命周期参数。不过，在第 4 章中的清单 4-9 中，咱们曾有一个不带生命周期注解也被编译了的函数，在下面清单 10-25 中再次予以展示。


文件名：`src/main.rs`

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
```

*清单 10-25：咱们曾在清单 4-9 中定义的一个即使其中的参数与返回值均为引用变量，而不带生命周期注解还仍编译了的函数*


这个函数不带生命周期注解仍会编译的原因，是历史遗留的：在 Rust 早期版本（`pre-1.0`）中，由于每个引用都需要显式生命周期，因此该代码就不会编译。那个时候，该函数签名会被写成下面这样：

```rust
fn first_word<'a>(s: &'a str) -> &'a str {
```

在编写许多 Rust 代码后，Rust 团队发现，Rust 程序员们在某些特定情形下，会一次又一次地敲入许多同样的生命周期注解。而这些特定情形，是可被预测的，并遵循了少数几种确定性模式，a few deterministic patterns。Rust 开发者们便把这些模式，编程进了编译器的代码，于是借用检查器就可以推断出这些情形下的生命周期，而无需显式的注解。

由于存在合并更多确定性模式，并将他们到编译器的可能，因此讲 Rust 的这段历史是有必要的。今后，或许就只要求更少甚至没有生命周期注解。

编程到 Rust 引用分析中的那些确定性模式，被称为 *生命周期省略规则，lifetime elision rules*。这些规则并非 Rust 程序员要遵循的；他们是编译器将考虑的一套特殊情形，并在咱们的代码符合这些情形时，咱们就不需要显式地写出生命周期。

这些省略规则并不提供完全的推断。在 Rust 明确地应用了这些规则，但仍存在引用具有什么样的生命周期方面的模糊性时，编译器将不会就其余引用变量应有什么样的生命周期，加以猜测。相比于猜测，编译器将给到咱们，可通过添加生命周期注解而解决的一个报错。

函数或方法参数上的生命周期，被称为 *输入生命周期，input lifetimes*，而在返回值上的生命周期，则被称为 *输出生命周期，output lifetimes*。

在没有显式注解时，编译器会运用三条规则，来计算出引用的生命周期。首条规则适用于输入生命周期，而第二及第三条规则，则都适用于输出生命周期。若编译器到了这三条规则的结束处，仍有其未能计算出生命周期的引用，那么编译器就会以报错而停止。这三条规则适用于 `fn` 定义及 `impl` 代码块都适用。

首条规则即为，编译器会指派生命周期参数给是引用的各个参数。也就是说，有着一个参数的函数，会获得一个生命周期参数：`fn foo<'a>(x: &'a i32)`；而有着两个参数的函数，就会得到两个单独生命周期参数：`fn foo<'a, 'b>(x: &'a i32, &'b i32)`；如此等等。

第二条规则，则是当确切地只有一个输入生命周期参数时，那个生命周期，就被指派给全部输出生命周期参数：`fn foo<'a>(x: &'a i32) -> &'a i32`。

第三条规则，当有多个输入生命周期参数，而由于这是个方法（这条规则是针对结构体上的方法），其中之一为 `&self` 或 `&mut self` 时，那么 `self` 的生命周期，便会被指派给全部输出生命周期参数。由于只有较少必要符号，因此这第三条规则，就会令到方法阅读与编写起来体验更佳。

下面咱们就来充当编译器。咱们将应用这些规则，来计算出清单 10-25 中，`first_word` 函数签名里各个引用的生命周期。函数签名以不带与其中引用关联的任何生命周期开始：

```rust
fn first_word(s: &str) -> &str {
```

随后编译器便应用首条规则，其指出了各个参数会获取到各自的生命周期。与平常一样，咱们将把该生命周期叫做 `'a`，那么现在函数签名就是这样的：

```rust
fn first_word<'a>(s: &'a str) -> &str {
```

由于这里只有一个输入生命周期，因此第二条规则便会适用。第二条规则指出，一个的输入参数生命周期，会被指派给输出生命周期，那么现在这个函数签名就是这样的：

```rust
fn first_word<'a>(s: &'a str) -> &'a str {
```

现在这个函数签名中的全部引用，都有了生命周期，进而编译器就可以在无需程序员注解函数签名中的生命周期的情况下，继续其分析了。

接下来就要看看另一个示例，这次要使用清单 10-20 中，一开始编写时没有生命周期参数的那个 `longest` 函数：

```rust
fn longest(x: &str, y: &str) -> &str {
```

首先来应用第一条规则：各个参数都得到自己的生命周期。这次不是一个而是两个参数，那么这里就有两个生命周期：

```rust
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
```

咱们可以看出，由于有多于一个的输入生命周期，因此第二条规则便不适用。因为 `longest` 是个函数而非方法，参数中没有一个是 `self`，因此第三条规则也不适用。在历经全部三条规则后，咱们仍未计算出返回值类型的生命周期为何。这就是咱们在尝试编译清单 10-20 中代码时，收到错误的原因：编译器历经这些生命周期省略规则，而仍未计算出函数签名中引用的全部生命周期。

由于第三条规则实际上只适用于方法签名，咱们接下来就要看看在方法上下文中的生命周期，以发现为何第三条规则，就意味着咱们不必经常注解方法签名中的生命周期。


## 方法定义方面

当咱们在结构体上实现带有生命周期的方法时，咱们会使用与清单 10-11 中所展示的泛型参数同样语法。其中咱们会根据其是否与结构体字段，或方法参数及返回值相关，而声明出并用到生命周期参数。

由于结构体字段的生命周期是结构体类型的一部分，因此他们总是需要声明在 `impl` 关键字之后，且随后会在结构体名字之后用到。

在 `impl` 代码块里的方法签名中，一些引用可能与结构体字段中的引用生命周期相关，也可能他们是独立的。此外，生命周期省略规则通常会发挥作用，从而在方法签名中，生命周期注解就不是必要的。咱们来看看一些使用咱们曾于清单 10-24 中定义的名为 `ImportantExcerpt` 结构体的示例。

首先，咱们将使用一个名为 `level` 的方法，其唯一参数是个到 `self` 引用，且返回值为非到任何东西引用的 `i32`：

```rust
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
```


`impl` 后的生命周期参数声明，与类型名字后其使用，都是必须的，但由于第一条省略规则的原因，咱们未被要求注解其中到 `self` 引用的生命周期。

下面是个其中第三条生命周期省略规则适用的示例：

```rust
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println! ("请注意：{}", announcement);
        self.part
    }
}
```

这里有两个输入生命周期，那么 Rust 就会适用首条生命周期省略规则，而赋予 `&self` 与 `announcement` 其各自的生命周期。随后，由于其中一个参数是 `&self`，那么返回值类型就会得到 `&self` 的生命周期，进而全部生命周期都得到了计算。


## 静态生命周期

咱们需要讨论的一种特殊生命周期是 `'static`，其表示受其影响的引用，*可以* 在程序整个持续时间内存活。所有字符串字面值，都有着 `'static` 的生命周期，咱们可将其注解为下面这样：

```rust
let s: &'static str = "我有静态的生命周期。";
```

此字符串的文本，被直接存储在该程序二进制数据中，而这是一直可用的。由此，所有字符串字面值的生命周期便是 `'static`。

在一些错误消息中，咱们或许会看到使用 `'static` 生命周期的建议。不过在给引用指定 `'static` 生命周期之前，请考虑一下手头的这个引用，是否会存活到整个程序的生命周期，以及咱们是否想要他存活到整个程序的生命周期。多数时候，建议 `'static` 生命周期的错误消息，都是由尝试创建悬空引用，或可用生命周期不匹配导致。在这些情况下，解决办法是修复这些问题，而非指定出 `'static` 生命周期。


# 泛型类型参数、特质边界与生命周期

咱们来简要地看看，在一个函数中，一起指定出泛型参数、特质边界与生命周期的语法！

```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println! ("通知！{}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

这便是清单 10-21 中，返回两个字符串切片中较长者的 `longest` 函数。不过现在他有了个泛型 `T` 名为 `ann` 的额外参数，泛型 `T` 可以实现了由 `where` 子句所指定的 `Display` 特质的任何类型填入。这个额外参数，将被使用 `{}` 打印出来，这便是为何 `Display` 特质为必要的原因。由于生命周期是泛型的一种，因此其中的生命周期参数 `'a` 与泛型参数 `T`，便处于函数名称后尖括号内的同一清单里。


# 本章小结

在这一章中，咱们谈到了很多东西！现在咱们清楚了泛型参数、特质与特质边界，与泛型生命周期参数，那么就准备好编写在许多不同情况下工作，不带有重复的代码了。泛型参数实现了将代码应用于不同类型。特质与特质边界确保即使类型是通用的，他们仍将有着代码所需的行为。咱们了解了如何使用生命周期注解，来保证这种灵活代码不会有任何悬空引用。且所有分析，都发生在编译时，其不会影响到运行时性能！

不论相信与否，关于咱们在本章讨论的这些话题，要掌握的东西远不止这些：第 17 章会讨论特质对象，trait objects，其是运用特质的另一种方式。还有更多咱们只会在极复杂场景下，才需要的涉及到更复杂场景的生命周期注解；要了解那些生命周期注解，咱们应阅读 [Rust 指南](https://doc.rust-lang.org/reference/index.html)。不过接下来，咱们将了解怎样编写 Rust 中的测试，从而就可以确保所咱们的代码，以其应有的方式工作。


（End）


