# 切片类型

**The Slice Type**


*切片，slices* 特性，实现了对集合中一个连续元素序列，而非对整个集合的引用。切片是引用的一种类别，因此他不会持有所有权。

这里有个小的编程问题：编写一个取得字符串，而返回在那个字符串中找到的第一个单词的函数。在函数在那个字符串中未找到空格时，那么这整个字符串就一定是一个单词，因此就要返回这整个字符串了。

下面就要在不使用切片特性的情况下，来看看该怎么编写这个函数的签名，从而搞明白切片要解决的问题：

```rust
fn first_word(s: &String) -> ?
```

这个 `first_word` 函数，有着一个作为参数的 `&String` 类型。这里不想要所有权，因此这是没问题的。不过应该返回什么呢？这里实在没有一种描述字符串 *局部（part）* 的方式。不过，这里可以返回那个单词的、以一个空格表示的结尾的索引。先来试试这个，如下面清单 4-7 所示：

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

*清单 4-7：返回那个 `&String` 参数中一个字节索引值的 `first_word` 函数*

因为这里需要对这个 `String` 值元素挨个遍历，进而挨个检查值是否是个空格，因此这里就将使用 `as_bytes` 方法，把这个 `String` 值转换为字节的数组：

```rust
let bytes = s.as_bytes();
```

接着，这里使用 `iter` 方法，创建了一个在该字节数组上的迭代器：

```rust
for (i, &item) in bytes.iter().enumerate() {
```

在第 13 章，将讨论到迭代器的更多细节。而现在，明白 `iter` 是个返回集合中各个元素的方法，而那个 `enumerate` 则会将 `iter` 的结果进行封装进而将各个元素作为一个元组的组成部分，进行返回即可。自 `enumerate` 返回的元组第一个元素就是索引值，而第二个元素，则是到 `iter` 返回元素的索引。相比由代码编写者自己计算索引，这就要方便一点。

由于 `enumerate` 方法返回了一个元组，因此这里就可以使用模式，来解构那个元组。在 [第 6 章](Ch06_Enums_and_Pattern_Matching.md#绑定到值的模式)，会对模式进行更多讨论。在那个 `for` 循环中，指定了一个有着用于那个元组中索引的 `i`，以及用于那个元组中单个字节的 `&item` 的模式。由于这里获得的是一个到从 `.iter().enumerate()` 获取元素的引用，因此在那个模式中使用了 `&` 运算符。

在那个 `for` 循环内部，这里通过使用字节字面值语法（the byte literal syntax），就表示空格的字节进行了搜索。在找到空格时，就返回空格的位置。否则就通过使用 `s.len()` 返回该字符串的长度。

```rust
        if item == b' ' {
            return i;
        }
    }

    s.len()
```

现在就有了一种找出字符串中第一个单词末尾索引的方法了，不过这里有个问题。这里所返回的只是个 `usize`，然而这个返回值只是在 `&String` 的语境下，才是个有意义的数字。也就是说，由于这个返回的 `usize` 类型值，是从那个 `String` 值获取到的孤立值，因此就没办法保证在以后仍然有效。关于这点，可考虑在清单 4-8 中、用到了清单 4-7 中 `first_word` 函数的这么一个程序。

文件名：`src/main.rs`

```rust
fn main() {
    let mut s = String::from("The quick brown fox jumps over the lazy dog.");

    let word = first_word(&s);  // 变量 word 将获得值 5

    s.clear();  // 这个语句会清空该字符串，令其等于 ""

    // 到这里变量 word 仍有着值 5，但已经不再有那个可将值 5 有意义的运用
    // 到的字符串了。变量 5 现在完全无用了！
}
```

*清单 4-8：将来自调用 `first_word` 函数的结果存储起来，并在随后修改那个 `String` 值的内容*

该程序会不带任何错误地编译，且同样会在调用了 `s.clear()`后使用变量 `word` 时，其仍会完成后续执行（this program compiles without any errors and would do so if we used `word` after calling `s.clear()`）。由于变量 `word` 完全未被连接到变量 `s` 的状态，因此变量 `word` 仍包含着值 `5`。这里仍可使用那个值 `5` 与变量 `s`，来尝试提取出第一个单词，但由于自将值 `5` 保存在 `word` 中以来，变量 `s` 的内容已被修改，因此这样做将是个程序错误（a bug）。

这种不可避免的担心变量 `word` 中的索引，失去与变量 `s` 中的数据同步，就会十分烦人且容易发生错误！而在要编写 `second_word` 函数时，对这些索引的管理，将更加脆弱。`second_word` 的函数签名，将务必看起来像下面这样：

```rust
fn second_word(s: &String) -> (usize, usize) {
```

现在就得对一个开始 *和* 结束索引保持跟踪，同时甚至还有更多的、要从特定状态中的数据计算出的值，而这些值又完全没有与那种状态联系起来。这样就有了三个无关的、需要同步保持的变量漂浮着。

幸运的是，Rust 有此问题的解决办法，那就是：字符串切片（string slices）。

## 字符串切片

字符串切片是到某个 `String` 类型值部分的引用，而看起来像下面这样：

```rust
    let s = String::from("The quick brown fox jumps over the lazy dog.");

    let the = &s[0..3];
    let quick = &s[4..9];
```

与到整个 `String` 值的引用 `&s` 不同，`the` 是到这个 `String` 的，在那个附加 `[0..3]` 中所指明的一部分的引用。通过指定 `[start_index..ending_index]`，而使用了在一对方括号里的一个范围，这里创建出了切片，其中的 `starting_index` 是切片中首个位置，而 `ending_index` 则是比切片中最后位置多一的位置索引。切片数据结构内部，存储着开始位置与该切片的长度，长度即 `ending_index` 减去 `starting_index`。那么在示例 `let quick = &s[4..9];` 中，`quick` 就会包含一个到变量 `s` 的索引 `4` 处字节的指针。

下图 4-6 展示对此进行了展示。

![指向一个 `String` 数据局部的字符串切片](images/Ch04_06.svg)

*图 4-6：指向一个 `String` 数据局部的字符串切片*

在 Rust 的 `..` 范围语法，the `..` range syntax 之下，在希望于索引为零处开始时，那么就可以舍弃那两个点之前的值。也就是说，写开始索引 `0` 与不写，是等价的：

```
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];
```

对同一个字符串令牌，在切片包含了那个 `String` 的最后字节时，那么就可以舍弃那结尾的数字。即意味着下面的语句是等价的：

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
