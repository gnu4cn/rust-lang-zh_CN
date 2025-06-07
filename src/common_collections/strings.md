# 使用 `String` 存储 UTF-8 编码的文本

在第 4 章中,我们曾讨论过字符串，现在我们将对其进行更深入的研究。新的 Rust 公民通常卡在字符串上，原因有三：

- Rust 有暴露可能错误的倾向；
- 字符串是种比许多程序员认为的更复杂的数据结构；
- 以及 UTF-8。

当咱们从其他编程语言转至 Rust 时，这些因素就会以看起来很难的方式，纠缠在一起。


我们之所以会在集合语境下讨论字符串，是因为字符串是作为字节集合，加上一些在这些字节被解释为文本时，提供有用功能的方法实现的。在本节中，我们将讨论那些 `String` 上的、每种集合类型都有的操作，比如创建、更新及读取等。我们还将讨论 `String` 与其他集合的不同之处，即由于人与计算机在解析 `String` 数据时的差异，而对 `String` 的索引会变得如何复杂。


## 何为字符串？


我们将首先定义出所谓 *字符串* 的含义。Rust 的核心语言中，只有一种字符串类型，即通常以其借用形式 `&str` 出现的字符串切片 `str`。在第 4 章我们曾谈到过 [*字符串片切片*](../ownership/the_slice_type.md#字符串切片)，他们是对存储在别处的 UTF-8 编码字符串数据的一些引用。例如，字符串的字面值，就存储在程序的二进制文件中，而因此属于一些字符串的切片。


所谓 `String` 类型，则是由 Rust 的标准库提供，而未被编码到核心语言，是种可增长、可变、有所有权、UTF-8 编码的字符串类型。当 Rust 公民提及 Rust 中的 “字符串” 时，他们可能指的是 `String` 或字符串切片的 `&str` 类型，而不单是其中一种类型。虽然本节主要讨论 `String`，但这两种类型在 Rust 的标准库中都有大量使用，同时 `String` 和字符串切片均为 UTF-8 编码的。


## 创建一个新的 `String`

对 `Vec<T>` 的许多相同操作对 `String` 也可使用，因为 `String` 实际上是作为字节矢量的封装器实现的，不过带有一些额外的保证、限制及功能。`Vec<T>` 和 `String` 下以同一方式工作的函数示例，便是创建出一个实例的 `new` 函数，如下清单 8-11 所示。


```rust
    let mut s = String::new();
```

*清单 8-11：创建一个新的空 `String`*


这行代码会创建出一个名为 `s` 的新空字符串，然后我们就可以向其中加载数据。通常情况下，我们会使用一些咱们开始该字符串的初始数据。为此，我们会使用 `to_string` 方法，该方法适用于任何实现了 `Display` 特质的类型，正如字符串字面量那样。下面清单 8-12 展示了两个示例。


```rust
    let data = "初始内容";

    let s = data.to_string();

    // 该方法同样直接工作于字面值之上
    let s = "初始内容".to_string();
```

*清单 8-12：使用 `to_string` 方法从某个字符串字面值创建出 `String`*

这段代码会创建出一个包含 `初始内容` 的字符串。


我们还可以使用函数 `String::from`，从某个字符串字面量创建出字符串。下面清单 8-13 中的代码，等同于清单 8-12 中使用 `to_string` 的代码。


```rust
    let s = String::from("初始内容");
```

*清单 8-13：使用 `String::from` 函数，从某个字符串字面值创建出 `String`*


由于字符串的用途非常广泛，我们可以使用许多不同的字符串通用 API，从而为我们提供了大量选择。其中有些看起来是多余的，但他们都有自己的用武之地！在这个示例中，`String::from` 和 `to_string` 做的是同一件事，所以咱们选择哪个只是风格与可读性的问题。

请记住，字符串是 UTF-8 编码的，因此我们可以在其中包含任何正确编码的数据，如下清单 8-14 所示。

```rust
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
    let hello = String::from("👋");
```

*清单 8-14：以字符串存储不同语言的问候语*

所有这些都是有效的 `String` 值。


## 更新字符串

在咱们向某个 `String` 中压入更多数据时，其大小和内容都会发生变化，就像 `Vec<T>` 的内容一样。此外，咱们还可以方便地使用 `+` 运算符或 `format!` 宏，连接多个 `String` 值。



### 使用 `push_str` 与 `push` 追加内容到某个 `String`

如下清单 8-15 所示，通过使用 `push_str` 方法追加一个字符串切片，我们可以增长某个 `String`。

```rust
    let mut s = String::from("foo");
    s.push_str("bar");
    println! ("{}", s);
```

*清单 8-15：使用 `push_str` 方法将字符串切片追加到某个 `String`*


在这两行后，`s` 将包含 `foobar`。`push_str` 方法会取个字符串切片，因为我们并不会想要取得那个参数的所有权。例如，在下面清单 8-16 中的代码中，我们打算在将 `s2` 的内容追加到 `s1` 后，能够使用 `s2`。


```rust
    let mut s1 = String::from("foo");
    let s2 = "bar";

    s1.push_str(s2);

    println! ("{}", s2);
```

*清单 8-16：在将其内容追加到某个 `String` 后，使用该字符串切片*


如果 `push_str` 方法取得了 `s2` 的所有权，我们就无法在最后那行打印出他的值。然而，这段代码会如我们与其那样工作！

`push` 方法会取单个字符作为参数，并将其添加到 `String`。下面清单 8-17 使用 `push` 方法将字母 `l` 追加到某个 `String`。

```rust
    let mut s = String::from("lo");
    s.push('l');
```

*清单 8-17：使用 `push` 将一个字符添加到 `String`*

作为结果，`s` 将包含 `lol`。


###  使用 `+` 运算符或 `format!` 宏的字符串连接

**Concatenation with the `+` Operator or the `format!` Macro**

通常情况下，咱们会想要合并两个现有字符串。一种方法是使用 `+` 运算符，如下清单 8-18 所示。


```rust
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;  // 请注意这里 s1 已被迁移，而不能再被使用
```

*清单 8-18：使用 `+` 运算符合并两个 `String` 值为新的 `String` 值*


字符串 `s3` 将包含 `Hello, world!`。加法运算后，字符串 `s1` 不再有效的原因，以及我们使用到 `s2` 引用的原因，都与我们使用 `+` 运算符时调用的方法签名有关。`+` 运算符使用了 `add` 方法，该方法的签名如下所示：


```rust
fn add(self, s: &str) -> String
```


在标准库中，咱们会看到使用了泛型及关联类型定义的 `add`。在这里，我们代入了具体类型，这就是我们对 `String` 值调用此方法时的情况。我们将在第 10 章讨论 [泛型](../Ch10_Generic_Types_Traits_and_Lifetimes.md)。这个签名为我们提供了理解 `+` 这个运算符中，棘手部分所需的线索。

首先，`s2` 带有一个 `&`，这意味着我们是将第二个字符串的 *引用*，加到第一个字符串。这是因为 `add` 函数中的那个 `s` 参数：我们只能将一个 `&str` 加到某个 `String`；我们不能将两个 `String` 值加到一起。但是等等，`&s2` 的类型为 `&String`，而不是 `&str`，正如 `add` 的第二个参数所指定的那样。那么为什么清单 8-18 会编译呢？


我们能够在到 `add` 调用中使用 `&s2` 的原因是，编译器可以将 `&String` 参数，强制转换为 `&str`。当我们调用 `add` 方法时，Rust 会使用 *解引用强制转换*，在这里其会将 `&s2` 转换为 `&s2[..]`。我们将在第 15 章更详细地讨论 [解引用强制转换](../smart_pointers/deref-t.md)。由于 `add` 方法不会取得参数 `s` 的所有权，因此在这个操作后，`s2` 仍将是个有效的 `String` 值。

其次，我们可以在这个签名中看到，`add` 函数会取得 `self` 的所有权，因为 `self` *没有* `&`。这意味着列表 8-18 中的 `s1`，将被迁移到 `add` 这个调用中，并且在调用之后将不再有效。因此，尽管 `let s3 = s1 + &s2;` 看起来像是会拷贝两个字符串，并创建出一个新字符串，但实际上该语句会取得 `s1` 的所有权，将 `s2` 内容的一份副本追加到 `s1` 上，然后返回结果的所有权。换句话说，虽然看起来像是进行了多次拷贝，但实际上并非如此；该实现比拷贝更高效。

在我们需要连接多个字符串时，`+` 运算符的行为会变得笨重：

```rust
    let s1 = String::from("tic");
    let s2 = String::from("toc");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
```

此时，`s` 将是 `tic-tac-toe`。由于包含大量 `+` 和 `"` 字符，很难看清具体情况。对于更复杂的字符串合并，我们可以改用 `format!` 这个宏：


```rust
    let s1 = String::from("tic");
    let s2 = String::from("toc");
    let s3 = String::from("toe");

    let s = format! ("{}-{}-{}", s1, s2, s3);
```


这段代码也会将 `s` 设置为 `tic-toc-toe`。`format!` 宏与 `println!` 类似，但他不会将输出打印到屏幕上，而是返回一个包含内容的 `String`。使用 `format!` 的代码版本更加易于阅读，且由 `format!` 宏生成的代码使用了引用，因此这个调用不会取得其任何参数的所有权。


## 字符串的索引

**Indexing into Strings**

在许多其他编程语言中，通过索引引用字符串中的单个字符，是种有效且常见的操作。然而，如果咱们尝试在 Rust 中使用索引语法，访问某个 `String` 的一些部分，咱们将得到一个报错。请设想下面列表 8-19 中的无效代码。


```rust
    let s1 = String::from("hello");
    let h = s1[0];
```

*清单 8-19：尝试对某个 `String` 使用索引语法*

这段代码将导致一些报错：

```console
$ cargo run
   Compiling string_demo v0.1.0 (/home/peng/rust-lang/projects/string_demo)
error[E0277]: the type `String` cannot be indexed by `{integer}`
 --> src/main.rs:3:13
  |
3 |     let h = s1[0];
  |             ^^^^^ `String` cannot be indexed by `{integer}`
  |
  = help: the trait `Index<{integer}>` is not implemented for `String`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `string_demo` due to previous error
```

该报错与注释，揭示了问题所在：Rust 字符串不支持索引操作。但为什么不支持呢？要回答这个问题，我们需要讨论 Rust 是如何在内存中存储字符串的。


### 内部表示

**Internal Representation**


`String` 是对 `Vec<u8>` 的封装。我们来看看列表 8-14 中，咱们的那些正确编码的 UTF-8 示例字符串。首先，这个：

```rust
let hello = String::from("Hola");
```

在这种情况下，`len` 值将为 `4`，这意味着存储字符串 `"Hola"` 的矢量值长度为 4 字节。在以 UTF-8 编码时，这些字母每个占用一字节。然而，下面这行代码可能会让咱们感到意外（请注意，这个字符串以大写的西里尔字母 *Ze* 开头，而非数字 3）：


```rust
    let hello = String::from("Здравствуйте");
```

在咱们被问及该字符串的长度时，你可能会说 12。事实上，Rust 的答案是 24：这是以 UTF-8 编码 “Здравствуйте” 所需的字节数，因为该字符串中的每个 Unicode 标量值，都需要 2 个字节的存储空间。因此，对字符串字节的索引，并不总是对应于有效的 Unicode 标量值。为了演示，请考虑下面这段无效的 Rust 代码：


```rust
    let hello = String::from("Здравствуйте");
    let answer = &hello[0];
```

咱们已经知道 `answer` 不会是 `З`，即第一个字母。当以 UTF-8 编码时，`З` 的第一个字节是 `208`，第二个字节是 `151`，因此 `answer` 似乎应是 `208`，但 `208` 本身并不是个有效的字符。在用户询问该字符串的首字母时，返回 `208` 可能并非用户所期望的结果；然而，这是 Rust 在字节索引 `0` 处唯一拥有的数据。用户通常不想要返回的字节值，即使该字符串仅包含拉丁字母：若 `&"hi"[0]` 是返回字节值的有效代码，他将返回 `104`，而非 `h`。

因此，答案是：为避免返回非预期值，而引发可能不会立即被发现的错误，Rust 根本不会编译这段代码，从而在开发过程早期就防止误解。


### 字节、标量值与字素簇！我的天！

**Bytes and Scalar Values and Grapheme Clusters! Oh My!**


关于 UTF-8 的另一要点，是从 Rust 角度来看，实际上存在三种相关的字符串表示方式：

- 字节；
- 标量值
- 以及字素簇（与我们所说的 *字母* 最接近的概念）。

在我们看到以梵文，Devanagari，书写的印地语单词 “नमस्ते” 时，他会被存储为一个 `u8` 值的矢量，看起来像这样：

```rust
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
```

这是 18 个字节，也是计算机最终存储这个数据的方式。若我们将其视为 Unicode 的标量值，即 Rust 的 `char` 类型，那么这些字节看起来如下：

```rust
['न', 'म', 'स', '्', 'त', 'े']
```

这里有六个 `char` 值，但第四和第六个不属于字母：他们是独立存在时没有意义的音标符号。最后，在我们将其视为字素簇是，我们就会得到人类所说的，构成印地语单词的四个字母：


```rust
["न", "म", "स्", "ते"]
```

Rust 提供了解析计算机所存储原始字符串数据的多种方法，以便每个程序都能根据需要选择合适的解析方式，而无论数据使用的何种人类语言。


Rust 不允许我们通过索引访问字符串中某个字符的最后一个原因，是索引操作被认为始终会消耗常数时间的复杂度（即 `O(1)` ）。但对于 `String` 而言，无法保证这种性能，因为 Rust 必须从开头遍历到索引位置的内容，以确定其间有效字符的数量。


## 字符串切片

**Slicing Strings**

字符串索引操作通常是个糟糕的主意，因为字符串索引操作的返回类型并不明确：是字节值、字符、字素簇，还是个字符串切片。因此，在咱们确实需要使用索引，创建字符串切片时，Rust 会要求咱们更加明确。

与其将 `[]` 与单个数字一起使用，咱们可将 `[]` 与某个范围一起使用，创建出包含特定字节的字符串切片：

```rust
    let hello = String::from("Здравствуйте");

    let s = &hello[0..4];
```

这里，`s` 将是个包含着该字符串前四个字节的 `&str`。早先我们提到，这些字符是两个字节，这意味着 `s` 将是 `Зд`。

若我们尝试以类似 `&hello[0..1]` 的方式，只切取该某个字符字节部分时，Rust 将以访问某个矢量值中无效索引时的同一方式终止运行：


```console
$ cargo run
   Compiling string_demo v0.1.0 (/home/hector/rust-lang-zh_CN/projects/string_demo)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/string_demo`
thread 'main' panicked at src/main.rs:3:19:
byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`
```

在以范围创建字符串切片时，咱们应谨慎操作，因为此类操作可能导致咱们的程序崩溃。


## 迭代字符串的方法

**Methods for Iterating Over Strings**


对字符串片段进行操作的最佳方式，是明确指定咱们是要字符还是字节。对于单个的 Unicode 标量值，就要使用 `chars` 方法。对 “Зд” 调用 `chars` 方法，会将其拆分并返回两个 `char` 类型的值，咱们就可以遍历结果，访问每个元素：


```rust
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
```

这段代码将打印以下内容：

```rust
न
म
स

त

```

或者，`bytes` 方法会返回可能适合咱们领域的各个原始字节：

```rust
    for b in "Зд".bytes() {
        println!("{b}");
    }
```

该代码将打印出构成这个 `String` 的 18 个字节来：

```console
208
151
208
180
```

但请务必记住，有效的 Unicode 标量值可能由多个字节组成。


从字符串中获取字素簇，如上面对梵文那样，较为复杂，因此标准库未提供此功能。若咱们需要此功能，可在 [crates.io](https://crates.io/) 上找到相关代码箱。



## 字符串并不简单

**Strings Are Not So Simple**

总而言之，字符串是复杂的。不同编程语言在向程序员呈现这种复杂性方面，会做出不同的选择。Rust 选择将正确处理 `String` 数据，作为所有 Rust 程序的默认行为，这意味着程序员在开发初期，必须更多地考虑如何处理 UTF-8 数据。这种权衡虽然比其他编程语言暴露了更多字符串的复杂性，但他防止了咱们在开发生命周期的后期，不得不处理那些涉及非 ASCII 字符的错误。

好消息是，标准库提供了大量基于 `String` 与 `&str` 类型的功能，帮助正确处理这些复杂情况。建议查阅文档，了解一些有用方法，例如用于在字符串中检索的 `contains` 方法，以及用于将字符串的一部分替换为另一字符串的 `replace` 方法等。




让我们转向一个稍微简单一点的主题：哈希图！


（End）


