# 在哈希图中存储关联的键与值

最后一个常用集合，就是 *哈希图（hash map）* 了。类型 `HashMap<K, V>`，存储了使用确定如何将这些类型为 `K` 的键，与类型为 `V` 的值放置于内存中的 *散列函数（a hashing function）*，而建立的键与值映射关系。许多编程语言都支持这种数据结构，不过他们通常使用了别的名称，比如哈希、图、对象、哈希表、字典，或者关系数组，这里仅举几例。

在打算不使用如同矢量中那样的索引，而是通过使用可为任意类型的键，来查找数据时，哈希图就是有用的了。比如在某个游戏中，就可在各个键为战队名字，值为战队得分的哈希图中，保持对这些战队得分的追踪。在给到战队名字后，就可获取到他的得分。

本小节将审视哈希图集合数据结构的基本 API，不过有数不尽的哈希图好处，都是隐藏在由标准库所定义、`HashMap<K, V>` 上的那些函数里。与往常一样，请查看标准库文档，来了解更多信息。


## 创新一个新的哈希图

创建空哈希图的一种方式，即为使用 `new` 方法，与使用 `insert` 方法进行元素添加。在下面清单 8-20 中，就要对两个名字分别为 *蓝队（Blue）* 与 *黄队（Yellow）* 的战队得分，进行追踪。蓝队以 10 分开始，而黄队以 50 分开始。


```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("蓝队"), 10);
    scores.insert(String::from("红队"), 50);
```

*清单 8-20：创建一个新的哈希图并插入一些键与值*

请注意这里需要首先 `use` 这个来自标准库集合部分的 `HashMap`。三个常用集合中，这个是最少用到的，因此他就没有包含在那些 Rust 程序前奏（the prelude）中，自动带入的特性里。哈希图受标准库的支持也较少；比如标准库中就没有内建的构造哈希图的宏。

与矢量一样，哈希图是将他们的数据存储在内存堆上的。示例中的这个 `HashMap` 键的类型为 `String`，值的类型为 `i32`。与矢量类似，哈希图都是同质的（homogeneous）：所有键都必须有着同样类型，且所有值也必须有着同样类型。

另一种构造哈希图的方式，即为通过使用迭代器，与元组矢量上的 `collect` 方法，而元组矢量中各个元组，则是由一个键与其值组成。在 [第 13 章的 “使用迭代器处理一系列的条目” 小节](Ch13_Functional_Language_Features_Iterators_and_Closures.md#使用迭代器对条目系列进行处理)，就会深入到迭代器的有关细节及其相关方法。`collect` 方法会将数据收集到包括 `HashMap` 在内数种集合类型中。比如，在将战队名字与初始得分放在两个单独矢量中时，那么就可以使用 `zip` 方法，来创建一个元组的迭代器，其中 `Blue` 就会与 `10` 结对，并以此类推。随后就可以使用 `collect` 方法类将那个元组迭代器，转换到一个哈希图了，如下清单 8-21 中所示。


```rust
    use std::collections::HashMap;

    let teams = vec! [String::from("蓝队"), String::from("红队")];
    let initial_scores = vec! [10, 50];

    let mut scores: HashMap<_, _> = teams
        .into_iter()
        .zip(initial_scores.into_iter())
        .collect();
```

*清单 8-21：从战队清单与得分清单创建一个哈希图*

由于有可能 `collect` 进到许多不同数据结构，而除非有指明，那么 Rust 就不清楚所想要的是何种数据结构，因此这里的类型注解 `HashMap<_, _>` 是需要的。不过对于键与值类型的泛型参数，这里使用了下划线（`_`），而 Rust 可基于那两个矢量中数据的类型，而推断出该哈希图的类型。在上面清单 8-21 中，键的类型将是 `String`，而值类型将为 `i32`，就跟清单 8-20 中的一样。


## 访问哈希图中的值

通过将某个值的键提供给 `get` 方法，就可以从哈希图中获取到该值来，如下清单 8-23 中所示。

```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("蓝队"), 10);
    scores.insert(String::from("红队"), 50);

    let team_name = String::from("蓝队");
    let score = scores.get(&team_name);
```

*清单 8-23：对存储在哈希图中的蓝队得分进行访问*

这里，`score` 将具有与蓝队关联的取值，同时结果将为 `Some(&10)`。由于 `get` 方法返回的是 `Option<&V>` 类型，因此该结构是封装在 `Some` 中的；当在哈希图中没有那个键的值时，`get` 就会返回 `None`。程序就需要以在第 6 章中所讲到的那些方式之一，对这个返回的 `Option` 加以处理。

可以与对矢量进行迭代的类似方式，即使用 `for` 循环，对哈希图中的各个键/值对加以迭代：

```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("蓝队"), 10);
    scores.insert(String::from("红队"), 50);

    for (key, value) in &scores {
        println! ("{}, {}", key, value);
    }
```

此代码将以任意顺序，打印出各个键值对：

```console
蓝队, 10
红队, 50
```


## 哈希图与所有权


对于实现了 `Copy` 特质（the `Copy` trait） 的那些类型，比如 `i32`，那么他们的值就被拷贝到哈希图里。而对于像是 `String` 这样的被持有值，他们的所有值就会被迁移，进而哈希图会成为这些值的所有者，如同在下面清单 8-22 中所演示的那样。

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

*清单 8-25：对一旦被插入到哈希图，键与值就被哈希图持有的展示*

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

在对 `insert` 调用而导致 `field_name` 与 `field_value` 被迁移到那个哈希图中之后，这里就无法使用这两个变量了。

而在将到值的引用插入进哈希图时，这些值就不会被迁移进哈希图。对于这些引用所指向的值，则只要哈希图尚有效，那么他们便一直有效。在后面第 10 章的 [“以声明周期对引用有效性进行验证”](Ch10_Generic_Types_Traits_and_Lifetimes.md#使用生命周期对引用加以验证) 小节中，将进一步讲到这些问题。


## 更新哈希图


虽然键值对数目是可增长的，但每个键在某个时刻，只能有一个与其关联的值。在要修改哈希图中的数据时，就必须定下来怎样处理某个键已经指定了值的情形。可以将原有值替换为新值，而完全忽略原有值。可以保留原有值而忽视掉新值，而在键 *尚未* 有值时，仅将新值进行添加。或者可以将原有值与新值结合在一起。那就来看看怎样处理这些各种情况！


## 重写某个值

**Overwriting a Value**


在将一个键与一个值插入到哈希图，并在随后再插入同样键与一个不同值，那么与那个键关联的值就会被替换掉。尽管下面清单 8-24 中的代码调用了 `insert` 两次，由于这里两次都是插入 “蓝队” 的值，因此那个哈希图将只包含一个键/值对。

```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("蓝队"), 10);
    scores.insert(String::from("蓝队"), 25);

    println! ("{:?}", scores);
```

*清单 8-24：以特定键对存储的某个值进行替换*

此代码打印 `{"蓝队": 25}`。原先的值 `10` 已被重写。


### 仅在某个键不存在时添加该键与值

**Adding a Key and Value Only If a Key Isn't Present**


检查哈希图中是否已经存在某个特定键，然后采取以下措施：在散列映射中确实存在该键，则保持现有值不变。如果该键不存在，则插入该键及其值，这种做法是常见的。

哈希映射为此有着一个将要检查的键作为参数，名为 `entry` 的特殊 API。`entry` 方法的返回值，是个表示某个可能存在，也可能不存在值的，名为 `Entry` 的枚举。比方说，我们打算检查黄队这个键，是否有关联的值。如果没有，我们就插入值 50，对于蓝队也一样。使用 `entry` 这个 API，代码就会看起来像下面的清单 8-24。

```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("蓝队"), 10);

    scores.entry(String::from("黄队")).or_insert(50);
    scores.entry(String::from("蓝队")).or_insert(50);

    println! ("{:?}", scores);
```

*清单 8-25：使用 `entry` 方法仅在键尚无值时插入值*

`Entry` 类型上的 `or_insert` 方法，被定义为在键存在时，返回相应 `Entry` 键的值的可变应用，而若键不存在，那么就会将其参数作为该键的新值插入，并返回到该新值的可变引用。此技巧相比于咱们自己来编写该逻辑，要清楚得多，此外，在以借用规则检查器进行检查时，进行得也更好。

运行清单 8-25 中的代码，将打印出 `{"黄队": 50, "蓝队": 10}`。其中首次到 `entry` 的调用，由于黄队尚无值，因此就会插入黄队的键与值 `50`。而第二个到 `entry` 的调用，因为蓝队已经有了值 `10`，因此就不会修改这个哈希图。


### 根据其原有值更新某个值

**Updating a Value Based on the Old Value**


哈希图的另一个常见使用情形，即是查找某个键的值，并随后根据原有值对其更新。举例来说，下面清单 8-26 给出了对在一些文字中各个词出现次数进行计数的代码。这里使用了一个以词汇作为键的哈希图，并对值进行增加来追踪已见到那个词了多少次。而在首次见到某个词时，就会首先插入值 `0`。

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

*清单 8-26：使用存储词汇与计数的哈希图，对词汇出现次数进行计数*

此代码将打印 `{"wonderful": 1, "world": 2, "hello": 1}`。这个 `split_withespace` 方法会对 `text` 中的那个值的、以空格分隔的子切片进行迭代。而那个 `or_insert` 方法，返回的时到指定键的值的可变引用（`&mut V`）。这里是将那个可变引用存储在变量 `count` 中的，因此为了对那个值进行赋值，这里就必须首先使用星号（`*`）对 `count` 解引用。在那个 `for` 循环结尾，该可变引用就超出了作用域，因此所有这些修改都是安全的，同时为借用规则所允许。


## 散列函数

**Hashing Functions**


默认情况下，`HashMap` 使用了一个可提供抵抗哈希表有关的拒绝服务攻击的、名为 *`SipHash`* 的散列函数。<sup>注 1</sup>这并非可用的最快散列算法，不过这种为了更好安全性，而在性能上的舍弃，是值得的。在对自己代码进行推敲，而发现这个默认散列函数对于自己目的太慢时，是可以通过指定别的哈希器，切换到另一函数的。 *哈希器（a hasher）* 是一种实现了 `BuildHasher` 特质（the `BuildHasher` trait）的类型。在第 10 章中就会谈到特质及其如何实现。不必从头实现自己的哈希器；[crates.io](https://crates.io/) 就有由其他 Rust 使用者共享的、提供对许多常用散列算法进行实现的哈希器的库。

> **注 1**：参见：[https://en.wikipedia.org/wiki/SipHash](https://en.wikipedia.org/wiki/SipHash)。


# 本章小结

矢量、字符串与哈希图，在程序中需要存储、访问与修改数据时，就会提供大量必要功能。下面就是一些现在应有能力解决的练习：

- 给定一个整数清单，请使用矢量，并返回这些数的中位数（即在这些数排序后，位于中间位置的值）与众数（最常出现的那个值；此时哈希图将有帮助）；
- 将字符串（英语）转换为拉丁语式的结尾。每个词汇的第一个常量，会被迁移到该词汇的末尾，同时会加上 “ay”，那么 “first” 就变成了 “irst-fay” 了。以元音开头的词汇，则会将 “hay” 添加到词汇末尾（比如 “apple” 就成了 “apple-hay”）。请牢记有关 UTF-8 编码的那些细节！
- 运用哈希图与矢量，创建一个实现程序用户把员工名字添加到某公司里的某个部门的文本接口。比如，“添加 Sally 到工程部” 或 “添加 Amir 到销售部”。随后让用户获取到某个部门全体人员清单，或以部门字母排序的公司全体人员名单。

标准库 API 文档对矢量、字符串及哈希图有着的、对这些练习将有帮助的方法都有说明！

接下来就要进入到一些其中某些操作可能失败的程序，那么现在就是讨论错误处理的最佳时机。下一章就要来完成对错误处理的讨论了！
