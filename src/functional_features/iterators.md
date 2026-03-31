# 以迭代器处理一系列项目

迭代器模式允许咱们依次对一系列项目执行某些任务。所谓迭代器，负责遍历每个项目以及判断序列何时结束的逻辑。当咱们使用迭代器时，咱们不必自己重新实现该逻辑。

在 Rust 中，迭代器是 *懒惰的*，这意味着在咱们调用消费迭代器的方法将其用尽之前，他们不会产生任何效果。例如，下面清单 13-10 中的代码通过调用定义在 `Vec<T>` 上的 `iter` 方法，创建一个遍历矢量值 `v1` 中的项目的迭代器。这段代码本身并不会完成任何有用的事情。

<a name="listing_13-10"></a>
```rust
    let v1 = vec! [1, 2, 3];

    let v1_iter = v1.iter();
```

**清单 13-10**：创建迭代器

迭代器存储在变量 `v1_iter` 中。一旦我们创建了迭代器，就可以多种方式使用他。在 [清单 3-5](../programming_concepts/control_flow.md#listing_3-5) 中，我们使用 `for` 循环遍历了一个数组，以对其每个项目执行一些代码。在底层，这隐式地创建然后消费了一个迭代器，但直到现在，我们才详细探讨其工作原理。

下面清单 13-11 中的示例中，我们把迭代器的创建与 `for` 循环中的使用分离。当使用 `v1_iter` 中的迭代器调用 `for` 循环时，迭代器中的每个元素就会在循环的一次迭代中使用，从而打印出每个值。

<a name="listing_13-11"></a>
```rust
    let v1 = vec! [1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println! ("得到：{val}");
    }
```

**清单 13-11**：在 `for` 循环中使用迭代器

在没有标准库提供的迭代器的语言中，咱们通常需要通过开始一个索引 0 处的变量，使用这个变量索引进入矢量值中来获取一个值，然后在循环中递增该变量值，直到其达到矢量中的条目总数，来编写这一相同功能。

迭代器为咱们处理了所有这些逻辑，从而减少咱们可能搞砸的重复代码。迭代器给予咱们更大的灵活性，以对许多不同类别的序列使用这同一逻辑，而不仅仅像矢量值那样咱们可以索引的数据结构。我们来看看迭代器是如何做到这一点的。


## `Iterator` 特质与 `next` 方法

所有迭代器都实现了定义在标准库中的名为 `Iterator` 的特质。该特质的定义看起来像下面这样：


```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // 有着默认实现的方法已省略
}
```

请注意，这个定义使用了一些新语法：`type Item` 与 `Self::Item`，他们定义了这个特质下的 *关联类型*。我们将在第 20 章中深入探讨关联类型。目前咱们只需知道，这段代码表明实现 `Iterator` 特质需要咱们同时定义一个 `Item` 类型，而这个 `Item` 类型会用于 `next` 方法的返回类型中。换言之，`Item` 类型将是自迭代器返回的类型。

`Iterator` 特质仅要求实现者定义一个方法：`next` 方法，他会一次返回迭代器的一个项目，包装在 `Some` 中，当迭代完毕时，返回 `None`。

我们可以直接调用对迭代器调用 `next` 方法；下面清单 13-12 演示了对从矢量创建的迭代器反复调用 `next` 会返回哪些值。

<a name="listing_13-12"></a>
文件名：`src/lib.rs`

```rust
#[test]
fn iterator_demonstration() {
    let v1 = vec! [1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq! (v1_iter.next(), Some(&1));
    assert_eq! (v1_iter.next(), Some(&2));
    assert_eq! (v1_iter.next(), Some(&3));
    assert_eq! (v1_iter.next(), None);
}
```

**清单 13-12**：对迭代器调用 `next` 方法

请注意，我们需要构造 `v1_iter` 为可变：调用迭代器上的 `next` 方法会修改迭代器的内部状态，迭代器用于追踪其位于序列中何处。换句话说，这段代码 *消费* 或用完迭代器。每次调用 `next` 都会吃掉迭代器中的一个项目。当我们使用 `for` 循环时，我们不需要使 `v1_iter` 可变，因为循环取得了 `v1_iter` 的所有权并在幕后使其可变。

还要注意，我们从调用 `next` 获取的值，属于对矢量中的值的不可变引用。`iter` 方法会生成一个对不可变引用的迭代器。当我们打算创建一个会取得 `v1` 的所有权并返回自有的值时，我们可以调用 `into_iter` 而不是 `iter`。同样的，当我们打算遍历可变引用时，可以调用 `iter_mut` 而不是 `iter`。


## 消费迭代器的方法

`Iterator` 特质有着数个带有标准库提供的默认实现的不同方法；咱们可以通过查阅 `Iterator` 特质的标准库 API 文档，了解这些方法。其中一些方法在他们的定义中调用 `next` 方法，这就是为什么在实现 `Iterator` 特质时，咱们需要实现 `next` 方法。

调用 `next` 的方法称为 *消费适配器，consuming adaptors*，因为调用他们会耗尽迭代器。一个例子是 `sum` 方法，他会取得迭代器的所有权，并通过重复调用 `next` 方法来迭代项目，从而消费迭代器。在遍历过程中，他会把每个条目加到一个运行总和，并在迭代完成后返回总和。下面清单 13-13有个测试，演示了 `sum` 方法的用法：

<a name="listing_13-13"></a>
文件名：`src/lib.rs`

```rust
#[test]
fn iterator_sum() {
    let v1 = vec! [1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq! (total, 6);
}
```

**清单 13-13**：调用 `sum` 方法获取迭代器中所有项目的总和

我们不允许在调用 `sum` 之后再使用 `v1_iter`，因为 `sum` 取我们对其调用他的迭代器的所有权。


## 产生其他迭代器的方法

所谓 *迭代器适配器，iterator adaptors*，属于一些定义在 `Iterator` 特质中的方法，他们不消费迭代器。相反，他们通过修改原始迭代器的某些方面而生成不同的迭代器。

下面清单 13-14 展示了调用迭代器适配器方法 `map` 的一个示例，其取一个闭包，在遍历迭代器项目时对每个项目调用该闭包。`map` 方法返回一个新的迭代器，生成修改后的项目。这里的闭包创建了一个新的迭代器，其中矢量值的每个项目都将增加 1：


<a name="listing_13-14"></a>
文件名：`src/main.rs`

```rust
    let v1 = vec! [1, 2, 3];

    v1.iter().map(|x| x + 1);
```

**清单 13-14**：调用迭代器适配器 `map` 来创建新的迭代器

然而，这段代码产生了一条告警：

```console
$ cargo run
   Compiling iterators v0.1.0 (/home/hector/rust-lang-zh_CN/projects/iterators)
warning: unused `Map` that must be used
 --> src/main.rs:4:5
  |
4 |     v1.iter().map(|x| x + 1);
  |     ^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: iterators are lazy and do nothing unless consumed
  = note: `#[warn(unused_must_use)]` (part of `#[warn(unused)]`) on by default
help: use `let _ = ...` to ignore the resulting value
  |
4 |     let _ = v1.iter().map(|x| x + 1);
  |     +++++++

warning: `iterators` (bin "iterators") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/iterators`
```

清单 13-14 中的代码未执行任何操作；我们指定的闭包从未被调用。这一告警提醒了我们原因：迭代器适配器是懒惰的，而我们需要在这里消费迭代器。

为了解决这一告警并消费该迭代器，咱们将使用 `collect` 方法，我们在 [清单 12-1](../io_project/accepting_cli_arguments.md#listing_12-1) 中对 `env::args` 使用过该方法。这个方法会消费迭代器，并收集结果值到集合数据类型中。

在下面清单 13-15 中，我们收集了遍历从调用 `map` 返回的迭代器的结果到一个矢量值中。这个矢量最终将包含原矢量值中的每个项目，且都增加 1。

<a name="listing_13-15"></a>
文件名：`src/main.rs`

```rust
    let v1 = vec! [1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq! (v2, vec! [2, 3, 4]);
```

**清单 13-15**：调用 `map` 方法创建一个新的迭代器，然后调用 `collect` 方法消费新的迭代器并创建一个矢量值

由于 `map` 取一个闭包，因此我们可以指定对每个项目想要的任何操作。这是个很好的示例，展示了闭包如何让咱们在重用 `Iterator` 特质提供的迭代行为的同时，自定义某些行为。

咱们可以链接多个调用到迭代器适配器，以可读的方式执行复杂操作。但由于所有迭代器都是懒惰的，因此咱们必须调用消费适配器方法之一，来获取迭代器适配器（方法）调用的结果。


## 捕获环境的闭包

许多迭代器适配器都会取闭包作为参数，且我们将作为参数指定给迭代器适配器的闭包，往往都将是一些会捕获其环境的闭包。

针对这一示例，我们将使用取一个闭包的 `filter` 方法。这个闭包获取一个迭代器中的项目并返回一个 `bool` 值。当闭包返回 `true` 时，值将包含在 `filter` 产出的迭代中。当闭包返回 `false` 时，值不会被包含。

> **译注**：原文这里有拼写错误，"If the closure returns `true`, the value will be included in the iteration produced by `filter`." 译者已通过 [Spell problem in Ch13-2 #4705](https://github.com/rust-lang/book/issues/4705) 提交该问题。

在下面清单 13-16 中，我们以一个闭包使用 `filter`，会捕获其环境中的 `shoe_size` 变量，以遍历 `Shoe` 结构体实例的集合。他将仅返回指定尺码的鞋子。

<a name="listing_13-16"></a>
文件名：`src/lib.rs`

```rust
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn filter_by_size() {
        let shoes = vec! [
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq! (
            in_my_size,
            vec! [
                Shoe {
                    size: 10,
                    style: String::from("sneaker"),
                },
                Shoe {
                    size: 10,
                    style: String::from("boot"),
                },
            ]
        );
    }
}
```

**清单 13-16**：以一个捕获 `shoe_size` 的闭包使用 `filter` 方法

`shoes_in_size` 函数取得一个 `shoes` 矢量值和 `shoe_size` 作为参数的所有权。他返回一个仅包含指定尺码鞋子的矢量值。

在 `shoes_in_size` 的主体中，我们调用 `into_iter` 来创建一个迭代器以取得矢量值的所有权。然后，我们调用 `filter` 来调整该迭代器为一个新的迭代器，仅包含闭包针对其返回 `true` 的元素。

闭包捕获环境中的 `shoe_size` 参数，并与每双鞋子比较该值，仅保留指定尺码的鞋子。最后，调用 `collect` 方法收集将调整后的迭代器返回的值，到一个该函数返回的矢量中。

测试表明，当我们调用 `shoes_in_size` 时，返回的只有与指定的值相同尺码的鞋子。


（End）


