# 切片类型

所谓 *切片，slices*，允许咱们引用集合中连续的元素序列。切片属于一种引用，因此他不拥有所有权。

这里有个小的编程问题：编写一个函数，该函数取由空格分隔单词的字符串，并返回其在该字符串中找到的第一个单词。若函数在字符串中未找到空格，则整个字符串必定是个单词，因此整个字符串应被返回。

> **注意**：出于介绍切片的目的，我们在这一小节中仅假设 ASCII；对 UTF-8 处理的更全面讨论，是在第 8 章的 [以字符串存储 UTF-8 编码的文本](../common_collections/strings.md) 小节处。

我们来看看，在不使用切片下咱们要怎样编写这个函数的签名，以了解切片将解决的问题：


```rust
fn first_word(s: &String) -> ?
```

函数 `first_word` 有个 `&String` 类型的参数。我们不需要所有权，所以这很好。（在惯用的 Rust 下，除非需要，否则函数不会取得他们参数的所有权，随着我们继续，其原因将变得清晰。）但我们应返回什么呢？我们确实没有一种描述字符串的 *一部分* 的方法。不过，我们可以返回以空格表示的单词结尾的索引。我们来尝试一下，如下清单 4-7 中所示。


<a name="listing_4-7"></a>
文件名：`src/main.rs`

```rust
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
```

**清单 4-7**：返回 `String` 参数中字节索引值的 `first_word` 函数

因为我们需要逐个元素地遍历 `String` 并检查值是否为空格，所以我们将使用 `as_bytes` 方法将 `String` 转换为字节数组。


```rust
    let bytes = s.as_bytes();
```


接下来，我们使用 `iter` 方法在字节数组上创建一个迭代器：


```rust
    for (i, &item) in bytes.iter().enumerate() {
```


我们将在 [第 13 章](../functional_features/iterator.md) 中更详细地讨论迭代器。现在只需了解，`iter` 是个返回集合中每个元素的方法，而 `enumerate` 封装 `iter` 的结果并将每个元素作为元组的一部分返回。`enumerate` 返回的元组的第一个元素是索引，第二个元素是指向元素的引用。这比我们自己计算索引更方便一点。

因为 `enumerate` 方法返回个元组，所以我们可以使用模式来解构该元组。我们将在 [第 6 章](../enums_and_pattern_matching/match_control_flow.md#绑定到值的模式) 中更多地详细讨论模式。在 `for` 循环中，我们指定了一种模式，其中 `i` 表示元组中的索引，`&item` 表示元组中单个字节。因为我们得到一个对 `.iter().enumerate()` 中元素的引用，所以我们在模式中使用 `&`。

在 `for` 循环内，我们通过使用字节字面值语法检索表示空格的字节。当我们找到一个空格时，我们就返回其位置。否则，我们通过使用 `s.len()` 返回字符串的长度。


```rust
        if item == b' ' {
            return i;
        }
    }

    s.len()
```


我们现在有一种办法，找出字符串中第一个单词末尾索引的方法，但有个问题。我们单独返回一个 `usize`，但他只在 `&String` 的上下文中才是个有意义的数字。换句话说，因为他是个独立于 `String` 的值，所以不能保证他在将来仍然有效。请看下面清单 4-8 中的程序，他使用清单 4-7 中的 `first_word` 函数。


文件名：`src/main.rs`

```rust
fn main() {
    let mut s = String::from("The quick brown fox jumps over the lazy dog.");

    let word = first_word(&s);  // word 将得到值 5

    s.clear();  // 这会清空那个 String，令其等于 ""

    // word 在这里仍有着值 5，但 s 已不再有咱们可对值 5
    // 有意义地运用的任何内容。因此 word 现在完全无效！
}
```

*清单 4-8：存储调用 `first_word` 函数的结果，然后修改 `String` 的内容*

这个程序可在没有任何报错下编译，并将在我们在调用 `s.clear()` 后使用 `word` 如此行事。因为 `word` 完全没有连接 `s` 的状态，所以 `word` 仍然包含值 `5`。我们本可将值 `5` 与变量 `s` 一起使用来提取出第一个单词，但这将是个 bug，因为自从我们在 `word` 中保存 `5` 以来，`s` 的内容已经发生了变化。

不得不担心 `word` 中的索引与 `s` 中的数据不同步，既乏味又容易出错！若我们还要编写 `second_word` 函数，管理这些索引就会更加脆弱。其签名就必须如下：


```rust
fn second_word(s: &String) -> (usize, usize) {
```

现在，我们正在跟踪起始索引 *和* 结束索引，而且咱们还有更多的值是根据特定状态下的数据计算得出的，却完全与该状态无关。我们有三个需要保持同步的无关变量在游离。

幸运的是，Rust 有一种针对这个问题的解决方法：字符串切片。


## 字符串切片

所谓 *字符串切片*，是对 `String` 中连续元素序列的引用，他看起来像这样：


```rust
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
```


与对整个 `String` 的引用不同，`hello` 是对 `String` 中一部分的引用，在额外的 `[0..5]` 部分中指定。我们通过指定 `[starting_index..ending_index]`，使用括号内的范围创建切片，其中 `starting_index` 是切片中的第一个位置，`ending_index` 比切片中最后一个位置多一个。在内部，切片数据结构存储了切片的起始位置和长度，对应于 `ending_index` 减去 `starting_index`。因此，在 `let world = &s[6..11];` 的情况下，`world` 将是个包含着指向 `s` 的索引 `6` 处字节的指针，与 `5` 的长度值。


> **译注**：切片应是一种灵巧指针。


图 4-7 以图示展示了这点。


![指向一个 `String` 数据局部的字符串切片](../images/Ch04_06.svg)

*图 4-6：指向 `String` 的一部分的字符串切片*


在 Rust 的 `..` 范围语法下，当咱们打算从索引 `0` 处开始时，咱们可去掉两个句点前的值。换句话说，下面两个值是相等的：


```rust
    let s = String::from("hello");

    let slice = &s[0..2];
    let slice = &s[..2];
```


同样的道理，当咱们的切片包含 `String` 的最后一个字节时，咱们可去掉尾随数字。这意味着下面两个值是相等的：


```rust
    let s = String::from("hello");

    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..];
```

咱们还可同时去掉这两个值来取整个字符串的切片。因此，下面两个值是相等的：

```rust
    let s = String::from("hello");

    let len = s.len();

    let slice = &s[0..len];
    let slice = &s[..];
```

> **注意**：字符串切片范围索引必须出现在有效的 UTF-8 字符边界处。当咱们尝试在多字节字符的中间创建字符串切片时，咱们的程序将以错误退出。


考虑到所有这些信息，我们来重写 `first_word` 为返回一个切片。表示 “字符串切片” 的类型写为 `&str`：


文件名：`src/main.rs`

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
```


我们以与清单 4-7 中相同的方式获取单词结尾的索引，即查找第一次出现的空格。当我们找到一个空格时，我们使用该字符串的开头与空格的索引，作为开始和结束索引返回一个字符串切片。

现在，当我们调用 `first_word` 时，我们会返回一个与所采用数据相关的值。该值由到切片起点的引用，和切片中元素的数量组成。

对于 `second_word` 函数来说，返回切片也是可行的：


```rust
fn second_word(s: &String) -> &str {
```

现在，我们有了一个简单明了的 API，而且更难出错，因为编译器会确保对那个 `String` 的引用保持有效。还记得清单 4-8 中程序的错误吗？当时我们得到了第一个单词末尾的索引，但随后又清除了那个字符串，因此咱们索引就无效来了。那段代码在逻辑上是错误的，但并没有立即给出任何错误。如果我们继续尝试对某个清空的字符串，使用第一个单词的索引，那么该问题就会在稍后出现。而切片则不会出现这种错误，并能让我们更早地知道，咱们代码出现了问题。使用切片版本的 `first_word` 会抛出一个编译时报错：


文件名：`src/main.rs`

```rust
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // error!

    println!("the first word is: {}", word);
}
```


下面是那个编译器报错：

```console
$ cargo run
   Compiling slices v0.1.0 (C:\tools\msys64\home\Lenny.Peng\rust-lang-zh_CN\projects\slices)
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> src\main.rs:6:5
  |
4 |     let word = first_word(&s);
  |                           -- immutable borrow occurs here
5 |
6 |     s.clear(); // error!
  |     ^^^^^^^^^ mutable borrow occurs here
7 |
8 |     println!("the first word is: {}", word);
  |                                       ---- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `slices` (bin "slices") due to previous error
```

回顾一下借用规则，如果我们有个不可变的引用，我们就不能同时取得一个可变的引用。因为 `clear` 需要截断这个 `String`，所以他需要得到一个可变引用。`clear` 调用后的那个 `println!`，使用了 `word` 中的引用，因此这个不可变引用，在此时必须仍然有效。Rust 不允许 `clear` 中的可变引用，和 `word` 中的不可变引用同时存在，因此编译会失败。Rust 不仅让我们的 API 更易于使用，还消除了编译时的一整类报错！


### 作为切片的字符串字面值

**String Literals as Slices**


回想一下，我们曾讲到过的存储在二进制文件中的字符串字面值。现在我们知道了分片，我们就能正确理解字符串字面值了：


```rust
let s = "Hello, world!";
```


这里 `s` 的类型，就是 `&str`：他是一个指向二进制中特定点的切片。这也是字符串字面值不可变的原因；`&str` 是个不可变引用。


### 作为参数的字符串切片

**String Slices as Parameters**


明白咱们可以取字符串字面值和 `String` 值的切片后，我们就可以对 `first_word` 进行另一项改进，那就是他的签名：


```rust
fn first_word(s: &String) -> &str {
```


更有经验的 Rustacean，会写下下面清单 4-9 中的签名，因为他允许我们，对 `&String` 值和 `&str` 值，使用同一个函数。


```rust
fn first_word(s: &str) -> &str {
```

*清单 4-9：通过对 `s` 参数的类型使用字符串切片，改进这个 `first_word` 函数*


如果我们有个字符串切片，我们可以直接传递他。如果我们有个 `String`，我们可以这个 `String` 切片，或到这个 `String` 的某个引用。这种灵活性，利用了我们将在第 15 章 [函数和方法中的隐式解引用强制转换](../smart_pointers/deref-t.md#函数与方法下的隐式解引用强制转换) 小节中,介绍的 *解引用强制转换* 特性。

定义一个取字符串切片，而非到某个 `String` 的引用的函数，使得我们的 API 更为通用和实用，而不会丢失任何功能：


文件名：`src/main.rs`

```rust
fn main() {
    let s = String::from("The quick brown fox jumps over the lazy dog.");

    // `first_word` 会在 String 的切片上工作，不管是部分还是整个 String
    let word = first_word(&s[0..6]);
    let word = first_word(&s[..]);

    // `first_word` 还对 String 的引用有效，这与 String 的整个切片等价
    let word = first_word(&s);

    let s_string_literal = "hello word";

    // `first_word` 在字符串字面值上有效，不论部分还是整体
    let word = first_word(&s_string_literal[0..6]);
    let word = first_word(&s_string_literal[..]);

    // 由于字符串字面值已经 *是* 字符串切片，
    // 因此无需切片语法，这也会工作。
    let word = first_word(s_string_literal);
}
```


## 其他切片

**Other Slices**


如同咱们可能想象的那样，字符串切片是专门针对字符串的。但还有一种更通用的切片类型。请看这个数组：


```rust
    let a = [1, 2, 3, 4, 5];
```

正如我们可能要引用字符串的一部分，我们也可能想引用数组的一部分。我们可以这样做：


```rust
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq! (slice, &[2, 3]);
```


这个切片的类型为 `&[i32]`。其工作方式与字符串切片相同，都是存储了到第一个元素的引用和长度。在其他各种集合中，咱们都将用到这种切片。我们将在第 8 章讨论矢量时，详细讨论这些集合。


# 本章小结


所有权、借用和切片的概念，确保了 Rust 程序在编译时的内存安全。Rust 给到了咱们，与其他系统编程语言同样方式的，对咱们内存使用的掌控，但在数据的所有者超出作用域时，会让数据的所有者，自动清理该数据，这意味着咱们不必编写和调试额外代码，来获得这种控制。

所有权会影响 Rust 许多其他部分的工作方式，因此我们将在本书的其余部分，进一步讨论这些概念。我们来继续阅读第 5 章，看看如何在 `struct` 中，对数据块进行分组。


（End）


