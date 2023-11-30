# 使用迭代器处理条目系列

**Processing a Series of Items with Iterators**


迭代器模式，the iterator pattern, 实现了在条目序列上，依次执行某个任务。迭代器负责对各个条目遍历，及判断序列何时结束的逻辑。咱们运用了迭代器后，就不必自己再实现那个逻辑。

在 Rust 中，迭代器是 *惰性的，lazy*，这意味着在咱们调用消费该迭代器的方法，将其用完前，他们没有任何效果。例如，下面清单 13-10 中的代码，通过调用定义在 `Vec<T>` 上的 `iter` 方法，而在矢量 `v1` 中的项目上创建了一个迭代器。这段代码本身并不做任何有用的事情。

```rust
    let v1 = vec! [1, 2, 3];
    let v1_iter = v1.iter();
```

*清单 13-10：创建迭代器*

迭代器被存储在变量 `v1_iter` 中。一旦咱们已创建出迭代器，就能以多种方式使用他。在第 3 章中的清单 3-5 中，就曾使用了 `for` 循环，对数组进行迭代，而在该数组的各个条目上执行代码。在使用 `for` 循环表象下，便是隐式地创建出迭代器，并随后消费迭代器，但直到现在，我们都未提及其原理。

下面清单 13-11 中的示例里，咱们把迭代器的创建，与 `for` 循环中迭代器的使用分离开了。当使用 `v1_iter` 中的迭代器，调用 `for` 循环时，迭代器中的各个元素，就会在循环的每次迭代中被使用，这就打印出了各个值。

```rust
    let v1 = vec! [1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println! ("得到了：{}", val);
    }
```

*清单 13-11：于 `for` 循环中使用迭代器*

在不具备由其标准库提供的迭代器的编程语言中，咱们很可能通过从 `0` 开始开始一个索引变量，使用那个变量索引到矢量值中来获取到一个值，并在循环中对递增索引变量值，直到索引变量达到矢量条目总数为止，而编写出这个同样功能。

迭代器为你处理所有这些逻辑，减少了咱们可能会搞砸的重复性代码。不只咱们可以索引的数据结构，比如矢量值，对于许多不同类别的序列，迭代器都给了我们运用同样逻辑的更多灵活性。咱们来看看迭代器是如何做到的。


## `Iterator` 特质与 `next` 方法

**The `Iterator` Trait and the `next` Method**


所有迭代器都实现了在标准库中定义的名为 `Iterator` 的特质。该特质的定义看起来像这样：


```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // 这里省略了有着默认实现的方法
}
```

请注意此定义使用了一些新语法：`type Item` 与 `Self::Item`，他们定义着此特质下的一个 *关联类型，associated type*。在 19 章中，咱们将深入谈及关联类型。至于现在，咱们只需清楚这段代码表明，实现 `Iterator` 特质需要咱们同时定义一个 `Item` 类型，而这个 `Item` 类型会在 `next` 方法返回值类型中用到。也就是说，`Item` 类型将是迭代器返回的类型。

`Iterator` 特质只需要实现者，implementors，定义一个方法：即 `next` 方法，该方法会一次返回一个封装在 `Some` 中的迭代器条目，当迭代完毕时，就会返回 `None`。

咱们可以直接调用迭代器上的 `next` 方法；下面清单 13-12 演示了，在自矢量创建出的迭代器上，反复调用 `next` 方法，会返回的值。

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

*清单 13-12：调用迭代器上的 `next` 方法*


请注意咱们需将 `v1_iter` 构造为可变：调用迭代器上的 `next` 方法，会修改迭代器用来追踪其位于序列中何处的内部状态。换句话说，这段代码 *消费，consumes*，或用掉，use up，了迭代器。每次对 `next` 的调用，都会吃掉迭代器的一个条目。在咱们使用 `for` 循环时，之所以不需要将 `v1_iter` 构造为可变，是由于那个循环取得了 `v1_iter` 的所有权，而在幕后将其构造为了可变。

还要注意咱们从 `next` 的调用获取到值，都是到矢量中值的不可变引用。`iter` 方法会产生对不可变引用的迭代器。若咱们打算创建出取得 `v1` 所有权，并返回有所有权的数据时，咱们可以调用 `into_iter` 而非 `iter`。与此类似，若咱们打算对可变引用迭代，咱们可以调用 `iter_mut` 而非 `iter`。


## 消费迭代器的方法

**Methods that Consume the Iterator**


`Iterator` 特质有着数个不同的，带有由标准库提供默认实现的方法；通过查阅 `Iterator` 特质的标准库 API 文档，咱们便可找到这些方法。其中一些方法，在他们的定义中会调用 `next` 方法，这就是为什么在实现 `Iterator` 特质时需要实现 `next` 方法的原因。

调用 `next` 的方法称为 *消费适配器，consuming adaptors*，因为调用它们会耗尽迭代器。一个例子是 `sum` 方法，他会获取迭代器的所有权并通过重复调用 `next` 方法来迭代项目，从而消费迭代器。在迭代过程中，他会把每个条目，加到一个运行中的总和，并在遍历完成时返回总和。下面清单 13-13，有着说明 `sum` 方法运用的测试：

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

*清单 13-13：调用 `sum` 方法来获取迭代器中全部项目的总和*

由于 `sum` 取得了咱们于其上调用他的迭代器所有权，因此在 `sum` 的调用后，就不允许使用 `v1_iter` 了。


## 产生出其他迭代器的方法

**Iterators that Produce Other Iterators**


*迭代器适配器，iterator adaptors* 是定义在 `Iterator` 特质上，不会消费迭代器的方法。相反，他们会通过改变初始迭代器的某一方面，而产生出另一迭代器。

下面清单 13-17 给出了调用迭代器适配器方法 `map` 的示例，其会取迭代器条目被遍历时，在各个条目上调用的一个闭包。`map` 方法会返回产生出修改后条目的新迭代器。这里的闭包创建了一个新的迭代器，其中原矢量的各个条目都增加了 `1`：

文件名：`src/main.rs`

```rust
    let v1 = vec! [1, 2, 3];

    v1.iter().map(|x| x + 1);
```

*清单 13-14：调用迭代器适配器 `map` 来创建出新迭代器*

然而，此代码会产生一条告警：

```console
$ cargo run                                                                                      lennyp@vm-manjaro
   Compiling iterator_demo v0.1.0 (/home/lennyp/rust-lang/iterator_demo)
warning: unused `Map` that must be used
 --> src/main.rs:4:5
  |
4 |     v1.iter().map(|x| x + 1);
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_must_use)]` on by default
  = note: iterators are lazy and do nothing unless consumed

warning: `iterator_demo` (bin "iterator_demo") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.36s
     Running `target/debug/iterator_demo`
```

清单 13-14 中的代码没有做任何事情；咱们指定的闭包从未被调用。这个警告提醒了我们为什么：迭代器适配器是懒惰的，我们需要在这里消费迭代器。

为修正此告警并消费迭代器，咱们将使用 `collect` 方法，在第 12 章的清单 12-1 中，咱们曾对 `env::args` 用到过该方法。此方法会消费迭代器，并将结果值收集到一个集合数据类型中。

下面清单 13-15 中，咱们把对从到 `map` 调用，返回的迭代器遍历的结果，收集到一个矢量值中。这个矢量最终将包含原矢量中增加 `1` 后的每一项。

文件名：`src/main.rs`

```rust
    let v1 = vec! [1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq! (v2, vec! [2, 3, 4]);
```

*清单 13-15：调用 `map` 方法创建出新迭代器，并随后调用 `collect` 方法消费这个新的迭代器并创建出一个矢量值*

由于 `map` 取了一个闭包，因此咱们便可指定出，咱们想要对各个条目执行的任何操作。这是一个很好的例子，说明闭包如何让咱们在重用Iterator属性提供的迭代行为的同时，定制一些行为。

咱们可将多个调用，链接到迭代器适配器，来以能读懂方式执行复杂操作。但由于所有迭代器都是惰性的，因此咱们必须调用一个消费适配器方法，one of the consuming apdaptor methods，来获取调用迭代器适配器的结果。


## 使用捕获其环境的闭包

**Using Closures that Capture Their Environment**


许多迭代器适配器，都会取闭包作参数，且通常咱们指定给迭代器适配器的闭包，都将是捕获其环境的闭包。

咱们将使用取闭包的 `filter` 方法，作为这方面的示例。闭包从其所在迭代器获取到一个条目，并返回一个 `bool`。闭包返回 `true` 时，条目的值将被包含在由 `filter` 产生出的迭代中。在闭包返回 `false` 时，条目的值则不会被包含。

下面清单 13-16 中，咱们使用带有捕获其环境中 `shoe_size` 变量闭包的 `filter` 方法，来迭代 `Shoe` 结构体实例的集合。他将返回仅限特定尺码的鞋子。

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

*清单 13-16：使用有着捕获 `shoe_size` 闭包的 `filter` 方法*

其中的 `shoes_in_size` 函数取得了 `shoes` 矢量的所有权，并取了 `shoe_size` 作参数。他返回只包含特定尺码鞋子的矢量。

`shoes_in_size` 的函数主体中，咱们调用 `into_iter` 创建一个迭代器，以取得矢量的所有权。随后咱们调用 `filter` 来将迭代器调整为，只包含令闭包返回 `true` 元素的新迭代器。

闭包从环境中捕获到 `shoe_size` 参数，并将该值与每双鞋子的尺码比较，只保留特定尺码的鞋子。最后，调用 `collect` 方法，将调整出的迭代器返回的 `Shoe` 类型值，收集到一个矢量中，`shoes_in_size` 函数返回的，便是这个矢量值。

测试表明，当咱们调用 `shoes_in_size` 时，只得到了与咱们指定的值相同大小的鞋子。
