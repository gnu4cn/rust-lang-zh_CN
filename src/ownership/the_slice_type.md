# 切片类型

**The Slice Type**


*切片，slices* 允许咱们引用某个集合中，连续的元素序列，而不是整个集合。切片属于一种引用，因此他不具有所有权。

这里有一个编程小问题：要编写一个，取个以空格分隔单词的字符串，并返回其在该字符串中，找到的第一个单词的函数。如果该函数在字符串中，未找到空格，那么整个字符串必定是一个单词，所以这整个字符串就应被返回。

我们来看看，在不使用切片的情况下，咱们要如何编写这个函数的签名，以了解切片将解决什么问题：


```rust
fn first_word(s: &String) -> ?
```

函数 `first_word` 有着一个 `&String` 的参数。我们不需要所有权，所以这没有问题。但我们应返回什么呢？我们确实没有描述字符串的 *一部分* 的方法。不过，我们可以返回该单词，以空格表示的结束处的索引。我们来试试看，如下清单 4-7 所示。


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

*清单 4-7：返回 `&String` 参数中某个字节索引值的 `first_word` 函数*


因为我们需要逐个元素地遍历这个 `String`，并检查某个值是否为空格，所以我们将使用 `as_bytes` 方法，将咱们的 `String` 转换为一个字节数组。


```rust
    let bytes = s.as_bytes();
```


接下来，我们使用 `iter` 方法，在这个字节数组上，创建了一个迭代器：


```rust
    for (i, &item) in bytes.iter().enumerate() {
```


我们将在 [第 13 章](../functional_features/iterator.md) 详细讨论迭代器。现在，我们只需知道 `iter` 是个会返回，集合中每个元素的方法，而 `enumerate` 会封装 `iter` 的结果，而将每个元素作为元组的一部分返回。`enumerate` 返回元组的第一个元素是索引，第二个元素是指向集合元素的引用。这比我们自己计算索引，要方便一些。

因为 `enumerate` 方法返回了个元组，所以我们可以使用模式，来解构这个元组。我们将在 [第 6 章](../enums_and_pattern_matching/match_control_flow.md#绑定到值的模式) 详细讨论模式。在这个 `for` 循环中，我们指定了个其中 `i` 表示元组中的索引，`&item` 表示元组中单个字节的模式。因为我们从 `.iter().enumerate()` 中，得到的是个指向集合元素的引用，所以我们在模式中，使用了 `&`。

在这个 `for` 循环中，我们通过使用字节字面值语法，检索表示空格的字节。如果我们找到了空格，就返回其位置。否则，我们便通过使用 `s.len()`，返回该字符串的长度。


```rust
        if item == b' ' {
            return i;
        }
    }

    s.len()
```


我们现在有了已知找出字符串中，第一个单词末尾索引的方法，但有个问题。我们单独返回一个 `usize`，但他只有在那个 `&String` 的上下文中,才是个有意义的数字。换句话说，因为他是个独立于那个 `String` 的值，所以不能保证他在将来仍然有效。请看下面清单 4-8 中的那个程序，他使用了清单 4-7 中的 `first_word` 函数。


文件名：`src/main.rs`

```rust
fn main() {
    let mut s = String::from("The quick brown fox jumps over the lazy dog.");

    let word = first_word(&s);  // word 将得到值 5

    s.clear();  // 这会清空那个 String，令其等于 ""

    // 这里 word 仍有着值 5，但已没有咱们可将值 5
    // 有意义地运用的字符串了。word 现在完全无效！
}
```

*清单 4-8：存储调用 `first_word` 函数的解构，并随后修改那个 `String` 的内容*

此程序会不带任何报错地编译，即使在我们在调用 `s.clear()` 之后使用 `word` 也会如此。由于 `word` 完全未与 `s` 的状态联系起来，`word` 仍然包含值 `5`。我们本可以使用值 `5` 于变量 `s`，提取出第一个单词，但这将是个错误，因为自从我们将 `5` 保存在 `word` 中后，`s` 的内容已经发生了变化。

要担心 `word` 中的索引与 `s` 中的数据不同步，既繁琐又容易出错！如果我们要编写一个 `second_word` 函数，那么管理这些索引，就会变得更加棘手。其签名应该是这样的：


```rust
fn second_word(s: &String) -> (usize, usize) {
```


现在，我们要跟踪起始 *和* 终止索引，而且咱们还有更多的值，是根据特定状态下的数据计算得出的，但这些值又与该状态完全无关。我们有三个不相关的变量，需要保持同步。

幸运的是，Rust 有此问题的解决方法：字符串切片。


## 字符串切片

**String Slices**


所谓 *字符串切片*，是对字符串部分内容的引用，他看起来像这样：


```rust
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
```


与对整个 `String` 的引用不同，`hello` 是对这个 `String` 中，由额外的 `[0..5]` 代码，所指定的一部分的引用。我们通过指明 `[starting_index..ending_index]`，来使用括号内的范围创建出切片，其中 `starting_index` 是切片中的第一个位置，`ending_index` 比切片中最后一个位置多一。在内部，切片这种数据结构，存储了切片的起始位置和长度，即 `ending_index` 减去 `starting_index`。因此，在 `let world = &s[6..11];` 的情况下，`world` 将是个包含着指向 `s` 的索引 `6` 处字节指针，长度值为 `5` 的切片。


> **译注**：切片应是一种灵巧指针。


下图 4-6 以图表的形式，展示了这一点。


![指向一个 `String` 数据局部的字符串切片](images/Ch04_06.svg)

*图 4-6：指向某个 `String` 的字符串切片*


使用 Rust 的 `..` 范围语法，如果咱们打算从索引 `0` 处开始，咱们可以去掉两个句点前的值。换句话说，下面这两个值是相等的：


```rust
    let s = String::from("hello");

    let slice = &s[0..2];
    let slice = &s[..2];
```


对于同一个字符串令牌，如果咱们的片段包括该 `String` 的最后一个字节，则可以去掉两个句点后的尾数。这意味着下面两个值是相等的：


```rust
    let s = String::from("hello");

    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..];
```

要取用整个字符串时，还可以把开始与结束索引都舍弃掉。那么下面的语句就是等价的了：

```rust
    let s = String::from("hello");

    let len = s.len();

    let slice = &s[0..len];
    let slice = &s[..];
```

> **注意**：这些字符串切片的范围索引值，必须出现于有效的 UTF-8 字符边界处。若在 UTF-8 多字节字符中间，尝试创建字符串切片，那么程序就会以错误退出。这里只是为介绍字符串切片目的，而假定本小节中只使用 ASCII 字符；在第 8 章的 [“以 `String` 类型值存储 UTF-8 编码的文本”](Ch08_Common_Collections.md#使用-string-存储-utf-8-编码的文本) 小节，有着对 UTF-8 字符串的更全面讨论。


对这全部字符串切片的情况了然在胸，那么下面就来将 `first_word` 重写为返回切片。表示 “字符串切片” 的类型，写做 `&str`：

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

这里是以前面清单 4-7 中所做的同样方式，即查找首次出现的空格，而获取到该单词结束处的索引。在找到空格时，就运用该字符串的开头，与那个空格的索引，作为字符串切片开始与结束索引，而返回一个字符串切片。

现在当调用 `first_word` 函数时，取回的便是与所用 `String` 数据联系起来单个值。这个值是由到切片起点的引用，与切片中元素个数所组成。

这样返回切片，对于 `second_word` 函数，也是有效的：

```rust
fn second_word(s: &String) -> &str {
```

由于编译器将确保到那个 `String` 数据中引用保持有效，因此现在就有了一个简单的、相比之前那个不那么容易搞混的 API 了。还记得在清单 4-8 中那个程序里的错误吧，即那个在已经获取到首个单词结束位置的索引，而随后清除了那个字符串，因此得到的索引就不在有效的问题。那段代码虽然逻辑上不正确，但也不会立即给出什么错误来。若继续尝试使用空字符串上的首个单词结束索引，这些问题仍会出现。切片就令到这个代码错误不可能了，并实现了更快发现代码问题。使用切片版本的 `first_word` 函数，就会抛出一个编译时错误：

文件名：`src/main.rs`

```rust
fn main() {
    let mut s = String::from("The quick brown fox jumps over the lazy dog.");

    let word = first_word(&s);

    s.clear();

    println! ("首个单词为：{}", word);
}
```

下面就是那个编译器错误消息：

```console
$ cargo run                                                                      ✔
   Compiling ownership_demo v0.1.0 (/home/peng/rust-lang/projects/ownership_demo)
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:5
  |
4 |     let word = first_word(&s);
  |                           -- immutable borrow occurs here
5 |
6 |     s.clear();
  |     ^^^^^^^^^ mutable borrow occurs here
7 |
8 |     println! ("首个单词为：{}", word);
  |                                 ---- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `ownership_demo` due to previous error
```

回顾借用规则，在有着到某数据的不可变引用时，就不能同时有可变引用。由于 `clear` 方法需要清空那个 `String` 值，那么就需要得到一个可变引用。而在 `clear` 方法调用之后的 `println!`，用到了变量 `word` 里的引用，那么这个不可变引用于那个时刻，就必将仍是活跃的。Rust 不允许 `clear` 中的可变引用，与 `word` 中的不可变引用同时存在，进而编译失败。可以看出，Rust 不光令到这个 `first_word` 的 API 更易于使用，他还在运行时就消除了这一整类错误！


### 作为切片的字符串字面值

**String Literals as Slices**


还记得前面讲到过，那些硬编码的、存储在二进制可执行文件内部的字符串字面值吧。现在了解了切片，那么就可以很好理解字符串字面值了：

```rust
let s = "Hello, world!";
```

这里的变量 `s` 的类型，即为 `&str`：他是个指向到二进制文件特殊点位的一个切片。这也是为何字符串字面值为不可变的原因；`&str` 类型属于不可变引用。


### 作为参数的字符串切片

**String Slices as Parameters**


了解了咱们可在函数中，取字符串字面值的切片及 `String` 值，就引出了对 `first_word` 函数的又一项改进，而下面就是函数 `first_word` 的签名：

```rust
fn first_word(s: &String) -> &str {
```

更老道的 Rust 公民将把这个函数签名，写着像下面清单 4-9 中所展示的那样，这是因为下面这样写，就实现了在 `&String` 与 `&str` 两种类型值上，可使用同一个函数：

```rust
fn first_word(s: &str) -> &str {
```

*清单 4-9：通过对 `s` 参数的类型使用字符串切片，对 `first_word` 函数进行改进*

在咱们有着某个字符串切片时，那么就可以直接传递那个字符串切片。而在咱们有着一个 `String` 时，则可传递该 `String` 的切片，或到这个 `String` 的引用。这种灵活性，是利用了 *强制引用解除，deref coercions* 特性，在第 15 章的 [函数与方法下的隐式强制解引用](Ch05_Smart_Pointers.md#函数与方法下的隐式解引用强制转换) 小节，将讲到的一种特性。

这样定义出取字符串切片，而非到 `String` 值引用做参数的函数，令到这个 API 在不丢失任何功能的情况下，变得更为通用和有用：

文件名：`src/main.rs`

```rust
fn main() {
    let s = String::from("The quick brown fox jumps over the lazy dog.");

    // 函数 first_word 在 String 值的切片上有效，不管是部分还是全部的切片
    let word = first_word(&s[0..6]);
    println! ("{}", word);

    let word = first_word(&s[..]);
    println! ("{}", word);

    // 函数 first_word 还在 String 变量的引用上有效，而 String 变量的引用
    // 与 String 值的整个切片是等价的
    let word = first_word(&s);
    println! ("{}", word);

    let s_string_literal = "hello word";

    // 函数 first_word 在字符串字面值上有效，不论是部分还是整体
    let word = first_word(&s_string_literal[0..6]);
    println! ("{}", word);

    let word = first_word(&s_string_literal[..]);
    println! ("{}", word);

    // 由于字符串字面值已经 是 字符串切片，因此无需切片语法，这
    // 也是有效的!
    let word = first_word(s_string_literal);

    println! ("{}", word);
}
```

## 其他切片

或许已经想到，字符串切片是特定于字符串的。然而还有更多通用切片类型呢。请看下面这个数组：

```rust
let a = [1, 2, 3, 4, 5];
```

就跟要引用字符串的部分一样，也可能要引用数组的部分。那么就将像下面这样，来完成对数组一部分的引用：

```rust

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq! (slice, &[2, 3]);
```

这个切片变量 `slice` 的类型为 `&[i32]`。数组切片的原理与字符串切片一样，都是经由存储到首个元素的引用，和切片长度实现的。今后将对所有类别的其他集合，运用到这种切片。在第 8 章讲到各种矢量时，就会对这些集合加以讨论。


# 本章小结

所有权、借用及切片等概念，在编译时确保了 Rust 程序中的内存安全。Rust 语言所给到的对内存运用的掌控方式，与别的系统编程语言相同，但会让数据的所有者，在其超出作用域时，自动清理掉其数据，这就意味着咱们不必编写并调试额外代码，来实现这种控制。

所有权对 Rust 程序的许多其他部分都有影响，因此在本书其余部分，都将更进一步的涉及到这些所有权的概念。接下来就要移步第 5 章，而在结构体 `struct` 中，如何将小块数据组装起来。
