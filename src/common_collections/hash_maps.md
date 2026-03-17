# 在哈希图中存储键与关联值

最后一个我们的常用集合是哈希图，hash map。`HashMap<K, V>` 类型使用 *哈希函数，hashing function* 存储类型 `K` 的键到类型 `V` 的值的映射，该函数决定了如何将这些键和值放入内存中。许多编程语言都支持这种数据结构，但他们通常使用不同的名称，比如 *哈希，hash*、*映射，map*、*对象，object*、*哈希表，hash table*、*字典，dictionary* 或 *关联数组，associative array* 等，仅举几例。

当咱们希望不通过使用索引，如同咱们可对矢量值那样，而通过可以是任何类型的键查找数据时，哈希图就非常有用。例如，在比赛中，咱们可以在哈希图中跟踪每支队伍的得分，其中每个键都是队伍的名字，而值为每支队伍的得分。给定队伍名字，咱们便可获取其得分。

我们将在这一小节中介绍哈希图的基本 API，但更多好东西隐藏在由标准库在 `HashMap<K, V>` 上定义的函数中。与往常一样，要查看标准库文档获取更多信息。


## 创建新的哈希图

创建空哈希图的一种方式是使用 `new` 并以 `insert` 添加元素。在下面清单 8-20 中，我们正在跟踪两只队伍的得分，他们的名字分别为 `Blue` 和 `Yellow`。蓝队以 10 分开始，黄队以 50 分开始。


<a name="listing_8-20"></a>
```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);
```

**清单 8-20**：创建一个新的哈希图并插入一些键和值

请注意，我们需要首先 `use` 标准库集合部分中的 `HashMap`。在我们的三种常用集合中，这种集合属于最不常用的，因此他未包含在前奏中自动带入作用域的特性里。哈希图在标准库中的支持也较少；例如，没有构造他们的内置宏。

与矢量一样，哈希图存储他们的数据于堆上。这个 `HashMap` 有着 `String` 类型的键及 `i32` 类型的值。与矢量值一样，哈希图属于同构的 <sup>1</sup>：所有的键都必须有着同一类型，且所有的值也必须有着同一类型。


> <sup>1</sup>：Homogeneous, 参见：[Difference Between Heterogeneous and Homogeneous Data Structures](https://codeskiller.codingblocks.com/library/articles/difference-between-heterogeneous-and-homogeneous-data-structures)

> **译注**：下面构造哈希图的另一种方式已从新版原文中移除。

构造哈希图的另一种方式是通过使用迭代器与元组矢量上的 `collect` 方法，元组矢量中各个元组由键及其值组成。我们将在第 13 章的 [在迭代器下处理一系列项目](../functional_features/iterators.md) 小节深入迭代器的细节及其相关方法。`collect` 方法会收集数据到数种集合类型中，包括 `HashMap`。比如，若我们将队伍名字和初始得分分别放在两个单独矢量中，我们就可使用 `zip` 方法创建一个元组的迭代器，其中 `Blue` 与 `10` 配对，并以此类推。随后我们可以使用 `collect` 方法将这个元组迭代器转换为一个哈希图，如下清单 8-21 中所示。


<a name="listing_8-21"></a>
```rust
    use std::collections::HashMap;

    let teams = vec! [String::from("Blue"), String::from("Red")];
    let initial_scores = vec! [10, 50];

    let mut scores: HashMap<_, _> = teams
        .into_iter()
        .zip(initial_scores.into_iter())
        .collect();
```

**清单 8-21**：从队伍列表和得分列表创建哈希图*

这里类型注解 `HashMap<_, _>` 是需要的，因为 `collect` 可能收集到许多不同的数据结构中，进而除非咱们指明，否则 Rust 就不清楚咱们想要哪种数据结构。但针对键与值类型的泛型参数，我们使用下划线（`_`），而 Rust 可以根据两个矢量中的数据的类型推断哈希图包含的类型。在清单 8-21 中，键类型将是 `String`，值类型将是 `i32`，就像清单 8-20 中的类型一样。


## 访问哈希图中的值

我们可以通过提供某个值的键给 `get` 方法从哈希图中获取该值，如下清单 8-23 中所示。

<a name="listing_8-22"></a>
```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
```

**清单 8-23**：访问存储在哈希图中的蓝队得分


这里，`score` 将有着与蓝队关联的值，结果将为 `10`。`get` 方法返回一个 `Option<&V>`；当哈希图中没有该键的值时，`get` 将返回 `None`。这个程序通过调用 `copied` 来获取一个 `Option<i32>` 而不是一个 `Option<&i32>` 处理该 `Option`，然后在 `scores` 没有该键的条目时调用 `unwrap_or` 来设置 `score` 为 `0`。

我们可以与我们对矢量所做的那样，使用 `for` 循环迭代哈希图中的每个键值对：

```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println! ("{key}: {value}");
    }
```

这段代码将以任意顺序打印各键值对：

```console
Yellow: 50
Blue: 10
```


## 管理哈希图中的所有权

对于实现了 `Copy` 特质的类型，如 `i32`，他们的值会被拷贝到哈希图中。对于像 `String` 这样的自有值，这些值将被迁移且哈希图将成为这些值的所有者，如下清单 8-23 中所示。

<a name="listing_8-23"></a>
```rust
    use std::collections::HashMap;

    let field_name = String::from("喜好颜色");
    let field_value = String::from("蓝色");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // 此时 field_name 与 field_value 都会无效，请尝试
    // 使用他们，看看会咱们会得到什么编译器错误！
```

**清单 8-23**：显示键和值一旦被插入哈希图，便由哈希图所有

```console
$ cargo run
   Compiling hashmap_demo v0.1.0 (/home/hector/rust-lang-zh_CN/projects/hashmap_demo)
error[E0382]: borrow of moved value: `field_name`
  --> src/main.rs:12:17
   |
 4 |     let field_name = String::from("喜好颜色");
   |         ---------- move occurs because `field_name` has type `String`, which does not implement the `Copy` trait
...
 8 |     map.insert(field_name, field_value);
   |                ---------- value moved here
...
12 |     println! ("{field_name}, {field_value}");
   |                 ^^^^^^^^^^ value borrowed here after move
   |
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning the value if the performance cost is acceptable
   |
 8 |     map.insert(field_name.clone(), field_value);
   |                          ++++++++

error[E0382]: borrow of moved value: `field_value`
  --> src/main.rs:12:31
   |
 5 |     let field_value = String::from("蓝色");
   |         ----------- move occurs because `field_value` has type `String`, which does not implement the `Copy` trait
...
 8 |     map.insert(field_name, field_value);
   |                            ----------- value moved here
...
12 |     println! ("{field_name}, {field_value}");
   |                               ^^^^^^^^^^^ value borrowed here after move
   |
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in
 Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning the value if the performance cost is acceptable
   |
 8 |     map.insert(field_name, field_value.clone());
   |                                       ++++++++

For more information about this error, try `rustc --explain E0382`.
error: could not compile `hashmap_demo` (bin "hashmap_demo") due to 2 previous errors
```

在 `field_name` 和 `field_value` 被调用 `insert` 迁移到哈希图中后，我们就无法使用这两个变量了。

当我们插入值的引用到哈希图中时，值将不会被迁移到哈希图中。引用指向的值必须至少在哈希图有效期间有效。我们将在第 10 章中的 [以生命周期验证引用](../generic_types_traits_and_lifetimes/lifetimes.md) 中进一步讨论这些问题。


## 更新哈希图

尽管键值对的数量可以增长，但每个唯一键在同一时间只能有一个与之关联的值（但反之则不然：例如，蓝队和黄队都可以在 `scores` 哈希图中存储值 `10`）。

当咱们打算更改哈希图中的数据时，咱们必须决定如何处理键已指派值的情形。

- 咱们可以新值替换旧值，完全忽略原有值；
- 咱们可以保留旧值而忽略新值，只在键还 *没有* 值时才添加新值；
- 或者咱们可以结合旧值与新值。


我们来分别看看怎样完成这些操作！


### 重写值

当如我们插入一个键和一个值到哈希图中，然后以不同值插入这同一个键时，则与该键关联的值将被替换。即使下面清单 8-23 中的代码调用了 `insert` 两次，哈希图也将只包含一个键值对，因为我们两次插入的都是蓝队的键的值。

<a name="listing_8-24"></a>
```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println! ("{scores:?}");
```

**清单 8-24**：以特定键替换某个存储的值

这段代码将打印 `{"Blue": 25}`。原先的值 `10` 已被重写。


### 仅当键不存在时才添加键和值

以某个值检查特定键是否已存在于哈希图中，然后采取以下操作：当该键确实存在于哈希图中，则现有值应保持原样；当该键不存在时，则插入他及他的值，这种做法很常见。

哈希图为此有着一个名为 `entry` 的特殊 API，他取咱们打算检查的键作为参数。`entry` 方法的返回值是个名为 `Entry` 的枚举，表示一个可能存在也可能不存在的值。假设我们打算检查黄队的键是否有个与之关联的值。当没有时，我们希望插入值 `50`，对于蓝队也一样。使用 `entry` API，代码看起来如下面清单 8-24 这样。

<a name="listing_8-25"></a>
```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println! ("{scores:?}");
```

**清单 8-25**：使用 `entry` 方法仅在键还没有值时插入

`Entry` （枚举类型）上的 `or_insert` 方法被定义为当该键存在时，返回到对应 `Entry` 键的值的可变引用；或当该键不存在时，他便将参数作为这个键的新值插入，并返回到这个新值的可变引用。这一技巧比我们自己编写逻辑要简洁得多，并且与借用检查器配合得也更好。

运行清单 8-25 中的代码将打印 `{"Yellow"： 50, "Blue": 10}`。对 `entry` 的第一次调用将以值 `50`插入黄队的键，因为黄队还没有值。到 `entry` 的第二次调用将不修改这个哈希图，因为蓝队已有值 `10`。


### 根据原有值更新值

哈希图的另一个常见用例是查找键的值，然后根据原有值予以更新。例如，下面清单 8-26 展示了计算每个单词在一些文本中出现次数的代码。我们使用以单词为键的哈希图，并递增值来追踪我们看到该单词的次数。当其为我们第一次看到某个单词时，我们将首先插入值 `0`。

<a name="listing_8-26"></a>
```rust
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println! ("{map:?}");
```

**清单 8-26**：使用存储单词与计数的哈希图来统计单词出现次数

这段代码将打印 `{"world": 2, "hello": 1, "wonderful": 1}`。咱们可能会看到以不同顺序打印的同样键值对：回顾 [访问哈希图中的值](#访问哈希图中的值) 中，对哈希图的迭代会以任意顺序发生。

`split_whitespace` 方法返回 `text` 中的值的，以空格分隔的子切片的迭代器。`or_insert` 方法返回到指定键的值的可变引用（`&mut V`）。在这里，我们存储该可变引用在变量 `count` 中，因此为了指派到该值，我们必须首先使用星号（`*`）解引用 `count`。这个可变引用在 `for` 循环结束时超出作用域，因此所有这些更改都是安全的并且是借用规则允许的。


## 关于哈希函数

默认情况下，`HashMap` 使用名为 *SipHash* 的哈希函数，该函数可抵抗涉及哈希数据表的拒绝服务（DoS）攻击 <sup>2</sup>。这不是可用的最快哈希算法，但为了更好的安全性而带来的性能下降的这种权衡是值得的。当咱们对咱们的代码进行性能分析，发现默认哈希函数对于咱们的目的而言太慢时，咱们可通过指定别的散列器切换到另一函数。所谓 *散列器，hasher*，是某种实现了 `BuildHasher` 特质的类型。我们将在 [第 10 章](../generic_types_traits_and_lifetimes/traits.md) 中讨论特质以及如何实现他们。咱们不必非要从头实现咱们自己的散列器；[`crates.io`](https://crates.io/) 有着由其他 Rust 用户共享的库，提供了实现许多常见哈希算法的哈希器。

> <sup>2</sup>：参见：[https://en.wikipedia.org/wiki/SipHash](https://en.wikipedia.org/wiki/SipHash)。


# 本章小结

当咱们需要存储、访问及修改数据时，矢量值、字符串与哈希图将提供程序中所需的大量功能。下面是咱们现在应该能够解决的一些练习：

1. 给定一个整数列表，请使用矢量并返回该列表的中位数（排序后，中间位置处的值）与模式（最常出现的值；哈希图在这里将很有帮助）；
2. 将字符串（英语）转换为拉丁文的结尾。每个单词的第一个辅音要移到该单词末尾并加上 *ay*，因此 *first* 变成 *first-fay*。以元音开头的单词要在末尾加上 *hay*（ *apple* 变成 *apple-hay* ）。请记住有关 UTF-8 编码的细节！
3. 使用哈希图和矢量值，创建一个文本界面，允许用户添加员工姓名到公司的某个部门；例如，“添加 Sally 到工程部” 或 “添加 Amir 到销售部”。然后，让用户获取某个部门中所有人员，或按部门的公司全体人员的字母顺序的列表。

标准库 API 文档描述了矢量值、字符串与哈希图有着的方法，这些方法对这些练习很有帮助！

我们正涉足于更复杂的程序，其中操作可能会失败，因此现在是讨论错误处理的最佳时机。接下来我们将讨论这个！


（End）


