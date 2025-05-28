# 在哈希图中存储关联的键与值

咱们最后一个常用集合，就是 *哈希图，hash map*。`HashMap<K, V>` 这种类型，使用 *哈希函数，hashing function*，存储类型为 `K` 的键，到类型为 `V` 的值的映射，哈希函数确定了其将这些值放置于内存中的方式。许多编程语言都支持这种数据结构，不过他们通常使用别的名称，比如 *哈希*、*映射*、*对象*、*哈希表*、*字典*，或者 *关联数组* 等，这里仅举几例。


在咱们不像在矢量值中，以索引查找数据，而以可是任何类型的键查找数据时，哈希图就非常有用。例如，在某场比赛中，咱们可在一个哈希图中记录每支球队的得分，其中每个键都是某支球队的名称，而值则是每支球队的得分。在给定某个队名时，咱们就可以获取到其得分。

我们将在本节中介绍哈希图的基本 API，但标准库为 `HashMap<K, V>` 定义的函数中，还隐藏了更多精彩内容。请一如既往地查看标准库文档，了解更多信息。


## 创建一个新哈希图


创建空哈希图的一种方法是使用 `new`，并以 `insert` 添加元素。在下面清单 8-20 中，我们记录了名为 `Blue` 和 `Yellow` 两支球队的得分。蓝队从 10 分开始，黄队从 50 分开始。


```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("蓝队"), 10);
    scores.insert(String::from("红队"), 50);
```

*清单 8-20：创建一个新的哈希图并插入一些键与值*


请注意，我们需要首先 `use` 标准库中集合部分的 `HashMap`。在我们常用的三种集合中，哈希图是最不常用的，因此他并未包含在前奏中自动带入作用域的那些特性里。哈希图在标准库中的支持也较少；例如，就没有构造哈希图的内置宏。

与矢量一样，哈希图也会将其数据存储在堆上。上面这个 `HashMap` 的键为 `String` 类型，值为 `i32` 类型。与矢量一样，哈希图也是同构的 <sup>1</sup>：所有键的类型必须相同，且所有值的类型也必须相同。


> <sup>1</sup>：Homogeneous, 参见：[Difference Between Heterogeneous and Homogeneous Data Structures](https://codeskiller.codingblocks.com/library/articles/difference-between-heterogeneous-and-homogeneous-data-structures)


另一种构造哈希图的方式为通过使用迭代器，及元组矢量上的 `collect` 方法，而元组矢量中各个元组由某个键与其值组成。在 [第 13 章的 “使用迭代器处理一系列的条目” 小节](Ch13_Functional_Language_Features_Iterators_and_Closures.md#使用迭代器对条目系列进行处理)，就会深入到迭代器的细节及其相关方法。`collect` 方法会将数据收集到包括 `HashMap` 在内的各种集合类型中。比如，在将球队名字与初始得分放在两个单独矢量中时，就可使用 `zip` 方法，创建出一个元组的迭代器，其中 `Blue` 会与 `10` 结对，并以此类推。随后就可使用 `collect` 方法，将那个元组迭代器转换为一个哈希图，如下清单 8-21 中所示。


```rust
    use std::collections::HashMap;

    let teams = vec! [String::from("蓝队"), String::from("红队")];
    let initial_scores = vec! [10, 50];

    let mut scores: HashMap<_, _> = teams
        .into_iter()
        .zip(initial_scores.into_iter())
        .collect();
```

*清单 8-21：从球队清单与得分清单创建出一个哈希图*

由于有可能 `collect` 到许多不同数据结构，因此除非有指明，那么 Rust 就不清楚其所想要的是何种数据结构，因此这里的类型注解 `HashMap<_, _>` 是需要的。对于键与值类型的泛型参数，这里使用了下划线（`_`），而 Rust 可基于那两个矢量中数据的类型，推断出该哈希图的类型。在上面清单 8-21 中，键的类型将是 `String`，值类型将是 `i32`，就跟清单 8-20 中的一样。


## 访问哈希图中的值

通过将某个值的键提供给 `get` 方法，咱们可从哈希图中提取该值，如下清单 8-23 中所示。


```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
```

*清单 8-23：访问该哈希图中蓝队的得分*


这里，`score` 将具有与蓝队关联的值，结果将是 `10`。`get` 方法会返回一个 `Option<&V>`；在哈希图中没有该键的值时，g`et` 将返回 `None`。这个程序处理 `Option` 的方法，是调用 `copied` 获得 `Option<i32>` 而不是 `Option<&i32>`，然后调用 `unwrap_or`，在 `scores` 中没有该键的条目时，将 `score` 设为 `0`。

就像处理向量一样，我们可使用 `for` 循环的方式，遍历哈希图中的每个键值对：

```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("蓝队"), 10);
    scores.insert(String::from("红队"), 50);

    for (key, value) in &scores {
        println! ("{}, {}", key, value);
    }
```

这段代码将以任意顺序打印出各键值对：

```console
蓝队, 10
红队, 50
```


## 哈希图与所有权

对于像是 `i32` 这样实现了 `Copy` 特质的类型值，会被复制到哈希图中。对于 `String` 这样的自有值，则会被迁移，同时哈希图将成为这些值的所有者，如下清单 8-22 所示。

```rust
    use std::collections::HashMap;

    let field_name = String::from("喜好颜色");
    let field_value = String::from("蓝色");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    println! ("{}, {}", field_name, field_value);
    // 到这里 field_name 与 field_value 就无效了，请尝试对
    // 他们进行使用，并看看会收到什么样的编译器错误！
```

*清单 8-25：显示一旦被插入到哈希图，键与值就被哈希图所有*

```console
$ cargo run                                                                ✔
   Compiling hashmap_demo v0.1.0 (/home/peng/rust-lang/hashmap_demo)
error[E0382]: borrow of moved value: `field_name`
  --> src/main.rs:10:25
   |
4  |     let field_name = String::from("喜好颜色");
   |         ---------- move occurs because `field_name` has type `String`, which does not implement the `Copy` trait
...
8  |     map.insert(field_name, field_value);
   |                ---------- value moved here
9  |
10 |     println! ("{}, {}", field_name, field_value);
   |                         ^^^^^^^^^^ value borrowed here after move
   |
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0382]: borrow of moved value: `field_value`
  --> src/main.rs:10:37
   |
5  |     let field_value = String::from("蓝色");
   |         ----------- move occurs because `field_value` has type `String`, which does not implement the `Copy` trait
...
8  |     map.insert(field_name, field_value);
   |                            ----------- value moved here
9  |
10 |     println! ("{}, {}", field_name, field_value);
   |                                     ^^^^^^^^^^^ value borrowed here after move
   |
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0382`.
error: could not compile `hashmap_demo` due to 2 previous errors
```

在调用 `insert` 将 `field_name` 和 `field_value` 迁移到哈希图中后，我们就无法使用这两个变量了。

若我们将到这两个值的引用，插入到哈希图中，那么这两个值就不会被迁移到哈希图中。引用所指向的值，必须至少在哈希图有效期内有效。我们将在第 10 章中的 [“使用生命周期验证引用”](../generic_types_traits_and_lifetimes/lifetimes.md) 中，详细讨论这些问题。


## 更新哈希图

虽然键值对的数量可以增长，但同一时间每个唯一键只能有一个与之关联的值（反之则不然：例如，蓝队和黄队都可以在 `scores` 这个哈希图中 `10`）。


当咱们打算更改某个哈希图中的数据时，咱们必须决定如何处理键已赋值的情形。

- 咱们可在完全忽略原有值下，用新值替换旧值；
- 咱们可以保留旧值而忽略新值，只有在键还 *没有* 值的情况下才添加新值；
- 或者，咱们也可以将旧值和新值结合。


我们来分别看看怎样完成这些操作！



### 重写某个值

**Overwriting a Value**


若我们将一个键与值插入到某个哈希图中，然后以不同值插入这个相同键，那么与该键关联的值就会被替换。即使下面清单 8-23 中的代码调用了两次 `insert`，该哈希图也只将包含一个键值对，因为我们两次插入的都是蓝队键的值。

```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("蓝队"), 10);
    scores.insert(String::from("蓝队"), 25);

    println! ("{:?}", scores);
```

*清单 8-24：以特定键替换某个存储的值*

这段代码将打印 `{"蓝队": 25}`。原先的值 `10` 已被重写。


### 仅在键不存在时添加键与值

**Adding a Key and Value Only If a Key Isn't Present**


以一个值检查哈希图中某个特定键是否已存在，并随后采取以下措施：若该键确实存在于哈希图中，则现有值应不变；若该键不存在，则插入该键及他的一个值，这种做法很常见。

哈希图为此提供了一个名为 `entry` 的特殊 API，他会取咱们打算检查的键作为参数。这个 `entry` 方法的返回值，是个名为 `Entry` 的枚举，表示某个可能存在也可能不存在的值。比方说，我们想要检查黄队这个键是否有个与其关联的值。在没有时，我们就插入值 `50`，对于蓝队也一样。使用这个 `entry` API，代码看起来如同下面清单 8-24。

```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("蓝队"), 10);

    scores.entry(String::from("黄队")).or_insert(50);
    scores.entry(String::from("蓝队")).or_insert(50);

    println! ("{:?}", scores);
```

*清单 8-25：使用 `entry` 方法，只有在键还没有值时插入*


`Entry` 上的 `or_insert` 方法，被定义为在键存在时，返回一个到这个对应 `Entry` 键值的可变引用；而若不存在，他就会将参数作为该键的新值插入，并返回一个到这个新值的可变引用。这种技巧比我们自己编写逻辑要简洁得多，而且与借用检查器的配合也更好。

运行清单 8-24 中的代码将打印出 `{"黄色"： 50, "Blue": 10}`。到 `entry` 的第一次调用，将插入黄队的键与值 `50`，因为黄队还没有值。到 `entry` 的第二次调用不会更改这个哈希图，因为蓝队已有了值 `10`。


### 根据原有值更新某个值

**Updating a Value Based on the Old Value**


哈希图的另一个常见用例，是查找某个键的值，然后根据其原有值更新他。例如，下面清单 8-25 给出了计算某个文本中每个单词出现次数的代码。我们使用了一个以各个单词为键的哈希图，并递增值来记录我们看到该单词的次数。在我们首次看到某个单词时，我们将首先插入值 `0`。

```rust
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println! ("{:?}", map);
```

*清单 8-26：使用存储了单词与计数的哈希图，统计单词出现次数*


这段代码将打印 `{"world": 2, "hello": 1, "wonderful": 1}`。咱们可能会看到这些同样的键值对，以不同的顺序打印出来：回顾在 [“访问哈希图中的值”](#访问哈希图中的值) 中，对哈希图的迭代就是以任意顺序出现的。


`split_whitespace` 这个方法会返回 `text` 中值以空白分隔的子切片的迭代器。`or_insert` 方法会返回指定键值的可变引用（`&mut V`）。这里，我们将该可变引用存储在 `count` 变量中，因此为了赋值给该值，我们必须首先使用星号（`*`），解引用 `count`。在这个 `for` 循环结束时，这个可变引用就会超出作用域，因此所有这些更改都是安全的，也是借用规则所允许的。


## 哈希函数

**Hashing Functions**

默认情况下，`HashMap` 使用了一个名为 `SipHash`，可抵抗涉及哈希表拒绝服务（DoS）攻击的散列函数 <sup>2</sup>。虽然这不是最快的散列算法，但以降低性能来换取更好的安全性是值得的。如果咱们在分析咱们的代码时，发现这个默认散列函数的速度对于咱们目的太慢，那么咱们可通过指定不同的散列器，切换到其他函数。所谓 *散列器，hasher*，是某种实现了 `BuildHasher` 特质的类型。我们将在 [第 10 章](../generic_types_traits_and_lifetimes/traits.md) 讨论特质以及如何实现特质。咱们不一定要从头实现咱们自己的散列器；[`crates.io`](https://crates.io/) 上有一些由其他 Rust 用户共享，提供实现了许多常见散列算法的散列器库。

> <sup>2</sup>：参见：[https://en.wikipedia.org/wiki/SipHash](https://en.wikipedia.org/wiki/SipHash)。


# 本章小结

在咱们需要存储、访问及修改数据时，矢量值、字符串于哈希图，提供了程序中大量必要的功能。下面是一些咱们现在应有能力解决的练习：


1. 给定一个整数列表，请使用一个矢量并返回该列表的中位数（排序时，位于中间位置的值）与模式（出现频率最高的值；哈希图在这里会很有用）；

2. 将字符串（英语）转换为拉丁文的结尾。每个单词的第一个辅音，要被移到该单词词末尾并加上 *ay*，因此 *first* 就会变成 *first-fay*。以元音开头的单词，则要在该词末尾加上 *hay*（ *apple* 就会变成 *apple-hay* ）。请记住有关 UTF-8 编码的细节！

3. 使用哈希图和矢量值，创建一个允许用户将员工姓名，添加到公司的某个部门的界面；例如，“将 Sally 添加到工程部” 或 “将 Amir 添加到销售部”。然后允许用户按字母顺序，获取某个部门所有人的请单，或按部门获取公司所有人的名单。

标准库 API 文档介绍了矢量值、字符串于哈希图的方法，这些方法对这些练习很有帮助！


我们正进入到一些更复杂程序中，其中的操作可能会失败，所以现在是讨论错误处理的最佳时机。接下来我们将讨论这个问题！


（End）


