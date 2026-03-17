# 以字符串存储 UTF-8 编码的文本

我们在第 4 章中讨论过字符串，而现在我们将更深入研究他们。新手 Rustacean 们通常会因为以下三个原因的结合，而在字符串上陷入困境：

- Rust 暴露可能错误的倾向；
- 字符串属于一种比许多程序员所认为的更复杂的数据结构；
- 以及 UTF-8。

当咱们来从其他编程语言时，这些因素以一种看似困难的方式结合在一起。


我们之所以会在集合的上下文中讨论字符串，是因为字符串是作为字节的集合实现的，加上一些在这些字节被解释为文本时提供有用功能的方法。在这一小节中，我们将讨论 `String` 上的、每种集合类型都有的操作，比如创建、更新及读取等。我们还将讨论 `String` 与其他集合的不同之处，即由于人与计算机在解析 `String` 数据方式上的差异，而怎样导致对 `String` 的索引会变得复杂。


## 定义字符串

我们将首先定义 *字符串* 这一术语的含义。Rust 在核心语言中只有一种字符串类型，即通常以其借用形式 `&str` 看到的字符串切片 `str`。在第 4 章中，我们讨论过 [字符串切片](../ownership/the_slice_type.md#字符串切片)，他们属于对存储在别处的一些 UTF-8 编码字符串数据的引用。例如，字符串的字面值存储在程序的二进制文件中，而因此属于字符串切片。

所谓 `String` 类型，是由 Rust 的标准库提供而非编码到核心语言中，是一种可增长、可变、被拥有（所有权）、UTF-8 编码的字符串类型。当 Rustacean 们提及 Rust 中的 “字符串” 时，他们可能指的要么是 `String`，或者是字符串切片 `&str` 类型，而不单是这两种类型之一。虽然这一小节主要是有关 `String`，但这两种类型在 Rust 的标准库中都大量使用，并且 `String` 和字符串切片都是 UTF-8 编码的。


## 创建新的 `String`

由于 `String` 实际上是作为对字节矢量的封装器实现的，并有着一些额外的保证、限制及功能，因此可用于 `Vec<T>` 的许多相同操作也可用于 `String`。一个在 `Vec<T>` 和 `String` 上工作方式相同的函数示例，便是创建实例的 `new` 函数，如下清单 8-11 中所示。

<a name="listing_8-11"></a>
```rust
    let mut s = String::new();
```

**清单 8-11**：创建一个新的空 `String`


这行代码创建了个名为 `s` 的新空字符串，随后我们可以加载数据到其中。通常，我们将有一些我们打算以其开始字符串的初始数据。为此，我们会使用 `to_string` 方法，其在任何实现了 `Display` 特质的类型上都可用，字符串字面量便如此。下面清单 8-12 展示了两个示例。


<a name="listing_8-12"></a>
```rust
    let data = "初始内容";

    let s = data.to_string();

    // 该方法直接在字面值之上也工作
    let s = "初始内容".to_string();
```

**清单 8-12**：使用 `to_string` 方法从字符串字面值创建 `String`

这段代码会创建一个包含 `初始内容` 的字符串。

我们还可使用函数 `String::from` 从字符串字面量创建 `String`。下面清单 8-13 中的代码与清单 8-12 中使用 `to_string` 的代码等价。


<a name="listing_8-13"></a>
```rust
    let s = String::from("初始内容");
```

**清单 8-13**：使用 `String::from` 函数从字符串字面值创建 `String`


由于字符串的用途非常广泛，我们可以使用许多不同的字符串通用 API，从而提供给我们很多选择。其中一些 API 看似多余，但他们都有他们的用处！在这一情形下，`String::from` 和 `to_string` 完成同一件事，因此咱们选择哪个取决于样式与可读性。

请记住，字符串都是 UTF-8 编码，因此我们可以在他们中包含任何正确编码的数据，如下清单 8-14 中所示。

<a name="listing_8-14"></a>
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

**清单 8-14**：以字符串存储不同语言的问候语

所有这些都是有效的 `String` 值。


## 更新字符串

当咱们压入更多数据到 `String` 中时，其大小会增加且其内容会变化，就像 `Vec<T>` 的内容一样。此外，咱们可以方便地使用 `+` 运算符或 `format!` 宏连接 `String` 值。

### 以 `push_str` 和 `push` 追加内容

我们可通过使用 `push_str` 方法追加字符串切片来增长 `String`，如下清单 8-15 中所示。

<a name="listing_8-15"></a>
```rust
    let mut s = String::from("foo");
    s.push_str("bar");
```

**清单 8-15**：使用 `push_str` 方法追加字符串切片到 `String`

这两行后，`s` 将包含 `foobar`。`push_str` 方法取一个字符串切片，因为我们并不想要取得参数的所有权。例如，在下面清单 8-16 中的代码中，我们希望在追加 `s2` 的内容到 `s1` 后能够使用 `s2`。

<a name="listing_8-16"></a>
```rust
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println! ("s2 为 {s2}");
```

**清单 8-16**：在追加字符串切片的内容到 `String` 后使用他


若 `push_str` 方法取得了 `s2` 的所有权，我们将无法在最后一行打印他的值。然而，这段代码会如我们预期那样工作！

`push` 方法取单个字符作为参数并将其添加到 `String`。下面清单 8-17 使用 `push` 方法添加字母 `l` 到一个 `String`。

<a name="listing_8-17"></a>
```rust
    let mut s = String::from("lo");
    s.push('l');
```

**清单 8-17**：使用 `push` 添加一个字符到 `String`

作为结果，`s` 将包含 `lol`。


###  以 `+` 或 `format!` 连接字符串

通常，咱们将希望合并两个现有字符串。一种这样做的方法是使用 `+` 运算符，如下清单 8-18 中所示。

<a name="listing_8-18"></a>
```rust
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;  // 请注意 s1 在这里已被迁移，而不能再被使用
```

**清单 8-18**：使用 `+` 运算符合并两个 `String` 值为新的 `String` 值

字符串 `s3` 将包含 `Hello, world!`。`s1` 在加法运算后不再有效的原因，与我们使用到 `s2` 的引用的原因，都与我们使用 `+` 运算符时调用的方法的签名有关。`+` 运算符使用 `add` 方法，其签名大致如下：


```rust
fn add(self, s: &str) -> String
```

在标准库中，咱们将看到 `add` 是使用泛型与关联类型定义的。在这里，我们已替换以具体类型，这正是我们对 `String` 值调用此方法时会发生的情况。我们将在第 10 章中讨论 [泛型](../Ch10_Generic_Types_Traits_and_Lifetimes.md)。这个签名给到我们为了理解 `+` 运算符的棘手部分所需的线索。

首先，其中的 `s2` 有个 `&`，这意味着我们正在添加第二个字符串的引用到第一个字符串。这是因为 `add` 函数中的参数 `s`：我们只能添加字符串切片到 `String`；我们不能将两个 `String` 值加到一起。但是等等，`&s2` 的类型为 `&String`，而不是如同在 `add` 的第二个参数中指定的 `&str`。那么，为什么清单 8-18 会编译呢？

我们能够在对 `add` 的调用中使用 `&s2` 的原因，是因为编译器可强制转换 `&String` 参数为 `&str`。当我们调用 `add` 方法时，Rust 会使用解引用强制转换，a deref coercion，在这里其会将 `&s2` 转换为 `&s2[..]`。我们将在第 15 章中进一步讨论 [解引用强制转换](../smart_pointers/deref-t.md)。由于 `add` 方法未取得参数 `s` 的所有权，因此 `s2` 在这一操作后将仍是个有效的 `String` 值。

其次，我们可以在签名中看到，`add` 函数会取得 `self` 的所有权，因为 `self` *没有* `&`。这意味着清单 8-18 中的 `s1` 将被迁移到 `add` 调用中而将在该调用后不再有效。因此，尽管 `let s3 = s1 + &s2;` 看起来将同时拷贝两个字符串并创建一个新字符串，这条语句实际上会取得 `s1` 的所有权，追加`s2` 内容的副本，然后返回结果的所有权。换句话说，看起来他构造了很多拷贝，但事实并非如此；这种实现相比拷贝更为高效。

当我们需要连接多个字符串时，`+` 运算符的行为会变得笨拙：

```rust
    let s1 = String::from("tic");
    let s2 = String::from("toc");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
```

此时，`s` 将为 `tic-tac-toe`。在全部的 `+` 和 `"` 字符下，难于看出发生了什么。针对更复杂方式的合并字符串，我们可以改用 `format!` 宏：


```rust
    let s1 = String::from("tic");
    let s2 = String::from("toc");
    let s3 = String::from("toe");

    let s = format! ("{s1}-{s2}-{s3}");
```


这段代码也会设置 `s` 为 `tic-toc-toe`。`format!` 宏以与 `println!` 的类似方式工作，但不会打印输出到屏幕，而是返回一个包含内容的 `String`。使用 `format!` 的代码版本更易于阅读，且由 `format!` 宏生成的代码使用引用，从而这一调用不会取得其任何参数的所有权。


## 字符串内的索引

在许多别的编程语言中，通过按照索引来引用字符串中的单个字符访问他们，属于有效且常见的操作。然而，当咱们尝试使用 Rust 中的索引语法访问 `String` 的部分时，咱们将得到报错。请考虑下面列表 8-19 中的无效代码。


<a name="listing_8-18"></a>
```rust
    let s1 = String::from("hello");
    let h = s1[0];
```

**清单 8-19**：尝试对 `String` 值使用索引语法

这段代码将导致以下报错：

```console
$ cargo run
   Compiling string_demo v0.1.0 (/home/hector/rust-lang-zh_CN/projects/string_demo)
error[E0277]: the type `str` cannot be indexed by `{integer}`
 --> src/main.rs:3:16
  |
3 |     let h = s1[0];
  |                ^ string indices are ranges of `usize`
  |
  = help: the trait `SliceIndex<str>` is not implemented for `{integer}`
  = note: you can use `.chars().nth()` or `.bytes().nth()`
          for more information, see chapter 8 in The Book: <https://doc.rust-lang.org/book/ch08-02-strings.html#indexing-into-strings>
help: the following other types implement trait `SliceIndex<T>`
 --> /rustc/01f6ddf7588f42ae2d7eb0a2f21d44e8e96674cf/library/core/src/slice/index.rs:214:1
  |
  = note: `usize` implements `SliceIndex<[T]>`
 --> /rustc/01f6ddf7588f42ae2d7eb0a2f21d44e8e96674cf/library/core/src/bstr/traits.rs:203:1
  |
  = note: `usize` implements `SliceIndex<ByteStr>`
  = note: required for `String` to implement `Index<{integer}>`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `string_demo` (bin "string_demo") due to 1 previous error
```

其中的报错道出了所以然：Rust 字符串不支持索引。但为什么不支持呢？要回答这个问题，我们需要讨论 Rust 在内存中存储字符串的方式。


### 内部表示

`String` 是对 `Vec<u8>` 的封装器。我们来看看 [列表 8-14](#listing_8-14) 中的一些咱们的正确编码的 UTF-8 示例字符串。首先是这个：

```rust
let hello = String::from("Hola");
```

在这种情况下，`len` 将为 `4`，这意味着存储字符串 `"Hola"` 的矢量值为 4 字节长。当以 UTF-8 编码时，这些字母每个占用一字节。然而，下面这行会让咱们感到意外（请注意，这个字符串以大写的西里尔字母 *Ze* 开头，而非数字 3）：


```rust
    let hello = String::from("Здравствуйте");
```

当咱们被问及该字符串的长度时，咱们可能会说 12。事实上，Rust 的答案是 24：这是以 UTF-8 编码 “Здравствуйте” 所需的字节数，因为该字符串中的每个 Unicode 标量值都需要 2 个字节的存储。因此，对字符串字节中的索引并不总是与有效的 Unicode 标量值对应。为了演示这一点，请考虑下面这段无效 Rust 代码：


```rust
    let hello = String::from("Здравствуйте");
    let answer = &hello[0];
```

咱们已经知道 `answer` 将不是第一个字母 `З`。当以 UTF-8 编码时，`З` 的第一个字节是 `208`，第二个字节是 `151`，因此看起来 `answer` 实际上应是 `208`，但 `208` 本身并不是个有效的字符。当用户询问该字符串的首字母时，返回 `208` 显然并非用户想要结果；然而，这是 Rust 在字节索引 `0` 处有着的唯一数据。通常用户不希望字节值返回，即使字符串仅包含拉丁字母：若 `&"hi"[0]` 属于返回字节值的有效代码，他将返回 `104`，而非 `h`。

那么答案就是，为了避免返回非预期值，和避免造成可能无法立即发现的 bug，Rust 根本不会编译这种代码，并在开发过程的早期防止误解。


### 字节、标量值与字素簇


关于 UTF-8 的另一要点是，从 Rust 角度来看，如何看待字符串实际上有三种相关的方式：字节、标量值以及字素簇（最接近于我们所说的 *字母* 的概念）。

当我们看到以梵文，the Devanagari script，书写的印地语单词 “नमस्ते” 时，他是作为看起来像下面这样的 `u8` 的矢量值存储的：

```rust
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
```

这是 18 个字节，也是计算机最终存储这一数据的方式。当我们将其视为 Unicode 的标量值，即 Rust 的 `char` 类型时，这些字节看起来如下面这样：

```rust
['न', 'म', 'स', '्', 'त', 'े']
```

> **译注**：每个 Unicode 标量值占 3 个字节的存储。

这里有六个 `char` 值，但第四和第六个不属于字母：他们是本身没有意义的变音符号。最后，当我们将其视为字素簇时，我们就会得到人们会讲的构成这个印地语单词的四个字母：


```rust
["न", "म", "स्", "ते"]
```

Rust 提供了解析计算机存储的原始字符串数据的不同方式，以便每个程序都可以选择其所需的解释，而无论数据是以哪种人类语言。

Rust 不允许我们索引到字符串中以获取字符的最后一个原因是，索引操作预期总是耗时恒定（即 `O(1)` 复杂度）。但在 `String` 下要保证这种性能是不可能的，因为 Rust 将必须遍历从开头到索引位置的内容以确定有多少个有效字符。


## 切分字符串

在字符串中索引操作通常是个坏主意，因为字符串索引操作的返回类型为何不明确：是字节值、字符、字素簇，或者字符串切片。因此，当咱们确实需要使用索引来创建字符串切片时，Rust 会要求咱们更为明确。

与其在单个数字下使用 `[]` 索引，咱们可在某一范围下使用 `[]` 来创建包含特定字节的字符串切片：

```rust
    let hello = String::from("Здравствуйте");

    let s = &hello[0..4];
```

这里，`s` 将是个 `&str`，包含字符串前 4 个字节。早先，我们曾提到每个这些字符都是 2 个字节，这意味着 `s` 将是 `Зд`。

若我们尝试以像是 `&hello[0..1]` 的方式仅切下字符字节的一部分，Rust 将在运行时停止运行，以矢量值中无效索引被访问时的同一方式：


```console
$ cargo run
   Compiling string_demo v0.1.0 (/home/hector/rust-lang-zh_CN/projects/string_demo)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/string_demo`

thread 'main' (423382) panicked at src/main.rs:4:19:
byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

在以范围创建字符串切片时咱们应谨慎操作，因为这样做可能崩溃咱们的程序。


## 迭代字符串

对字符串片段操作的最佳方式是要明确咱们是需要字符还是字节。针对单个的 Unicode 标量值，就要使用 `chars` 方法。对 “Зд” 调用 `chars` 方法会分离并返回两个 `char` 类型的值，进而咱们可以遍历该结果以访问每个元素：


```rust
    for c in "Зд".chars() {
        println!("{c}");
    }
```

这段代码将打印以下内容：

```rust
З
д
```

或者，`bytes` 方法会返回每个原始字节，他们可能适合咱们的领域：

```rust
    for b in "Зд".bytes() {
        println!("{b}");
    }
```

这段代码将打印构成这个字符串的 4 个字节：

```console
208
151
208
180
```

但要务必记住，有效的 Unicode 标量值会由超过 1 个字节组成。

正如对梵文一样，获取字符串中的字素簇较为复杂，因此这一功能并未由标准库提供。若这是咱们需要的功能，可在 [crates.io](https://crates.io/) 上找到代码箱。


## 处理字符串的复杂性

总而言之，字符串是复杂的。不同编程语言对于如何向程序员呈现这种复杂性做出了不同的选择。Rust 选择将正确处理 `String` 数据，作为所有 Rust 程序的默认行为，这意味着程序员必须在前期就要投入更多思考到处理 UTF-8 数据中。这种权衡虽然比其他编程语言更直观地暴露了更多的字符串复杂性，但他防止了咱们必须在开发生命周期的后期处理涉及非 ASCII 字符的错误。

好消息是，标准库提供了大量基于 `String` 与 `&str` 类型构建的功能，以帮助正确处理这些复杂情况。请务必查阅文档以了解有用的方法，例如在字符串中检索的 `contains` 方法，以及以另一字符串替换字符串一部分的 `replace` 方法等。

我们来转向一个稍微简单一点的主题：哈希图！


（End）


