# 流：序列中的未来值

**Streams：Futures in Sequence**


本章到目前为止，我们主要着重于单个的未来值。唯一例外是我们曾用到的异步通道。回顾在本章前面的 [“消息传递”](concurrency_n_async.md#使用消息传递在两个任务上计数) 小节，我们使用咱们异步通道接收器的方式。那个异步 `recv` 方法，会随着时间推移产生一个条目序列。这是种所谓 *流* 的更通用模式的一个实例。


早在第 13 章 [“`Iterator` 特质与 `next` 方法”](../functional_features/iterators.md#iterator-特质与-next-方法) 小节，介绍 `Iterator` 这个特质时，我们就见到过条目序列，但迭代器与那个异步通道接收器，有两个不同点。第一个区别是时间：迭代器是同步的，而这个通道接收器是异步的。第二个区别是 API。在直接使用 `Iterator` 时，我们会调用其同步的 `next` 方法。而在这个特定的 `trpl::Receiver` 流下，我们调用了一个异步的 `recv` 方法。除此之外，这两个 API 感觉非常相似，而这种相似并非巧合。所谓流，就像迭代的一种异步形式。不过，尽管 `trpl::Receiver` 专门等待接收消息，单通用的流 API 则有着更广泛的范围：他以 `Iterator` 同样方式，提供下一项目，不过是异步地提供。


Rust 中迭代器与流之间的相似性，意味着我们实际上可以从任何迭代器，创建出一个流。与某个迭代器一样，通过调用某个流的 `next` 方法然后等待输出，我们就可以使用这个流，如下清单 17-30 所示。


文件名：`src/main.rs`

```rust
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = values.iter().map(|n| n * 2);
        let mut stream = trpl::stream_from_iter(iter);

        while let Some(value) = stream.next().await {
            println!("The value was: {value}");
        }
```


*清单 17-30：从某个迭代器创建出一个流，并打印其值*


我们从一个数字的数组开始，将其转换为一个迭代器，然后调用 `map` 将所有值加倍。然后我们使用 `trpl::stream_from_iter` 函数，将这个迭代器转换为一个流。接下来，我们以那个 `while let` 循环，在该流中项目到达时遍历他们。


不幸的是，当我们尝试运行该代码时，他不会编译，而是报告没有 `next` 方法可用：


```console
error[E0599]: no method named `next` found for struct `tokio_stream::iter::Iter` in the current scope
 --> src/main.rs:7:40
  |
7 |         while let Some(value) = stream.next().await {
  |                                        ^^^^
  |
  = help: items from traits can only be used if the trait is in scope
help: the following traits which provide `next` are implemented but not in scope; perhaps you want to import one of them
  |
1 + use futures_util::stream::stream::StreamExt;
  |
1 + use std::iter::Iterator;
  |
1 + use std::str::pattern::Searcher;
  |
1 + use trpl::StreamExt;
  |
help: there is a method `try_next` with a similar name
  |
7 |         while let Some(value) = stream.try_next().await {
  |                                        ~~~~~~~~

```

正如此输出所解释的，这条编译器报错的原因，是我们需要在作用域中的正确特质，才能使用这个 `next` 方法。根据我们到目前为止的讨论，咱们可能会合理地认为，这个特质是 `Stream`，但他实际上是 `StreamExt`。`Ext` 是 `extension` 的缩写，是 Rust 社区中用于表示以一个特质，扩展另一特质的常见模式。

在本章末尾，我们将对 `Stream` 与 `StreamExt` 两个特质进行更详细的解释，但现在咱们只需知道，`Stream` 特质定义了个可有效地结合 `Iterator` 与 `Future` 特质的底层接口。而 `StreamExt` 则在 `Stream` 之上，提供了一组更高级别的 API，包括 `next` 方法以及与由 `Iterator` 特质所提供的类似其他工具方法。`Stream` 与 `StreamExt` 还不是 Rust 标准库的一部分，但大多数生态代码箱，都使用了这同样的定义。


修复这个编译器报错的方法，就是添加一条 `trpl::StreamExt` 的 `use` 语句，如下清单 17-31 所示。


文件名：`src/main.rs`


```rust
use trpl::StreamExt;

fn main() {
    trpl::run( async {
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = values.iter().map(|n| n * 2);
        let mut stream = trpl::stream_from_iter(iter);

        while let Some(value) = stream.next().await {
            println!("The value was: {value}");
        }
    });
}
```

*清单 17-31：成功将某个迭代器，用作一个流的基础*


将所有这些部分放在一起，这段代码就可以按照我们想要的方式运行了！此外，既然咱们在作用域中有了 `StreamExt`，我们就可以像使用迭代器一样，使用 `StreamExt` 的所有工具方法。例如，在下面的清单 17-32 中，我们就使用了 `filter` 方法，过滤掉了除 3 与 5 倍数之外的所有内容。


文件名：`src/main.rs`


```rust
use trpl::StreamExt;

fn main() {
    trpl::run( async {
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = values.iter().map(|n| n * 2);
        let mut stream = trpl::stream_from_iter(iter);

        let mut filtered =
            stream.filter(|value| value % 3 == 0 || value % 5 == 0);

        while let Some(value) = filtered.next().await {
            println!("The value was: {value}");
        }
    });
}
```

*清单 17-32：以 `StreamExt:filter` 方法过滤某个流*


当然，这并不是很有趣，因为我们可以用一般迭代器完成同样的事情，而根本不需要任何的异步。我们来看看我们能做些什么 *流* 特有的事情。


## 合成流

**Composing Streams**


许多概念都可很自然地表示为流：某个队列中的项目成为可用；当整个数据集相对于内存来说太大时，从文件系统增量地拉取数据块；或者随着时间推移，数据经由网络到达等等。由于流是一些未来值，我们可以将他们与任何的其他类型未来值一起使用，并以有趣方式将他们结合。例如，我们可以批量处理事件，避免触发过多网络调用；对长期运行的操作序列设置超时；或为用户界面事件设置阈值，避免执行不必要的工作。


我们以创建一个小的消息流，作为咱们可能在 WebSocket 或其他实时通信协议中，所见到的数据流替身，如下清单 17-33 所示。


文件名：`src/main.rs`


```rust
use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    trpl::run(async {
        let mut messages = get_messages();

        while let Some(message) = messages.next().await {
            println!("{message}");
        }
    });
}

fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
    for message in messages {
        tx.send(format!("Message: '{message}'")).unwrap();
    }

    ReceiverStream::new(rx)
}
```


*清单 17-33：将 `rx` 接收器，用作一个 `ReceiverStream`*


首先，我们创建了个返回 `impl Stream<Item = String>`，名为 `get_messages` 的函数。对于其实现，我们创建了个异步通通道，遍历英语字母表的前 10 个字母，并将他们发送到该通道上。


我们还使用了一种将 `trpl::channel` 中的 `rx` 接收器，转换为有着 `next` 方法流的新类型 `ReceiverStream`。回到 `main`，我们使用一个 `while let` 循环，打印出该流中的所有消息。


当我们运行这段代码时，我们会得到正是我们所期望的结果：


```console
Message: 'a'
Message: 'b'
Message: 'c'
Message: 'd'
Message: 'e'
Message: 'f'
Message: 'g'
Message: 'h'
Message: 'i'
Message: 'j'
```


同样，我们可以使用常规的 `Receiver` API，或甚至常规的 `Iterator` API 完成这一点，不过，我们来添加一个需要流的功能：添加一个应用到流上每个项目的超时，以及一个我们发出项目上的延迟，如下清单 17-34 所示。


文件名：`src/main.rs`


```rust
use std::{pin::pin, time::Duration};
use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    trpl::run(async {
        let mut messages =
            pin!(get_messages().timeout(Duration::from_millis(200)));

        while let Some(result) = messages.next().await {
            match result {
                Ok(message) => println!("{message}"),
                Err(reason) => eprintln!("Problem: {reason:?}"),
            }
        }
    })
}
```


*清单 17-34：使用 `StreamExt::timeout` 方法，在流中项目上设置一个时间限制*


我们以使用来自 `StreamExt` 特质的 `timeout` 方法，为流添加超时开始。然后，我们更新那个 `while let` 循环的主体，因为该流现在会返回一个 `Result`。其中的 `Ok` 变种表示有消息及时到达了；`Err` 变种表示在有消息到达前超时了。我们对该结果进行 `match`，要么在成功接收消息时打印出该消息，否则打印一条超时的通知。最后，请注意在对消息应用超时后，我们会对其进行了固定，因为这个超时助手会产生一个需要被固定才能轮询的流。


不过，由于消息之间没有延迟，这个超时没有改变该程序的行为。我们来为咱们发送的信息，添加一个可变延迟，如下清单 17-35 所示。


文件名：`src/main.rs`


```rust
fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
        for (index, message) in messages.into_iter().enumerate() {
            let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;

            tx.send(format!("Message: '{message}'")).unwrap();
        }
    });

    ReceiverStream::new(rx)
}
```


*清单 17-35：在不将 `get_messages` 函数构造为异步函数下，以一个异步的延迟通过 `tx` 发送消息*


在 `get_messages` 中，我们对 `messages` 这个数组，使用了迭代器方法 `enumerate`，这样咱们就能获取到咱们正发送的每个项目索引及该项目本身。然后，我们对偶数索引的项目，应用 100 毫秒的延迟，对奇数索引的项目，应用 300 毫秒的延迟，模拟现实世界中，我们可能见到的消息流不同延迟。由于我们的超时为 200 毫秒，这应该会影响到一半的消息。
