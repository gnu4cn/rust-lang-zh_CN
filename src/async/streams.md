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


在 `get_messages` 中，我们对 `messages` 这个数组，使用了迭代器方法 `enumerate`，这样咱们就能获取到咱们正发送的每个项目索引及该项目本身。然后，我们对偶数索引的项目应用 100 毫秒的延迟，对奇数索引的项目应用 300 毫秒的延迟，模拟现实世界中我们可能见到的消息流不同延迟。由于我们的超时为 200 毫秒，这应该会影响到一半的消息。


要在 `get_messages` 函数中的消息间无阻塞地睡眠，我们需要用到异步。但是，我们不能让 `get_messages` 本身构造为一个异步函数，因为这样我们就会返回一个 `Future<Output = Stream<Item = String>>` 而不是 `Stream<Item=String>>`。调用者也将必须等待 `get_messages` 本身才能访问到该流。但请记住：给定未来值中的所有项目，都是线性发生的；并发会发生在未来值 *之间*。等待 `get_messages` 就将要求他，在返回这个接收器流前发送所有消息，包括每条消息间的睡眠延迟。因此，其中的超时将毫无用处。在该流本身中，不会有延迟；所有延迟都应在该流可用前发生。


相反，我们将 `get_messages` 作为一个返回流的常规函数，且咱们生成了个任务，处理其中异步的 `sleep` 调用。


> **注意**：以这种方式调用 `spawn_task` 会工作，是因为我们已经设置了咱们的运行时；若咱们未设置运行时，其将造成一个运行时不可恢复错误，a panic。其他（译注：异步运行时）实现，选择了不同折衷方法：他们可能会生成一个新的运行时而避免这种不可恢复错误，但最终会带来一些额外的开销；或者他们会在没有到运行时的引用时，简单地不提供生成任务的独立方法。请确保咱们清楚咱们的运行时，选择了哪种折衷方法，并据此编写代码！


现在，我们的代码有了更有趣的结果。在每对信息之间，都有个 `Problem: Elapsed(())` 报错。



```console
Message: 'a'
Problem: Elapsed(())
Message: 'b'
Message: 'c'
Problem: Elapsed(())
Message: 'd'
Message: 'e'
Problem: Elapsed(())
Message: 'f'
Message: 'g'
Problem: Elapsed(())
Message: 'h'
Message: 'i'
Problem: Elapsed(())
Message: 'j'
```


超时并未阻止信息最终到达。我们仍能收到全部原始消息，因为我们的通道 *不受限制*：内存能容纳多少消息，通道就能容纳多少消息。若超时前消息没有到达，我们的流处理程序将考虑到这点，但当他再次轮询该流时，信息就可能已经到达了。


在需要时，咱们可通过使用更通用的其他类型通道或其他类型流，获得不同行为。我们来通过将一个时间间隔流与这个消息流结合，看看其中的一种实际应用。



## 合并流

**Merging Streams**


首先，我们来创建另一个，在咱们让他直接运行时，他将每毫秒发出一个条目的流。为简单起见，我们可使用 `sleep` 函数，以一定延迟发送一条消息，并将其与咱们在 `get_messages` 中使用的自通道创建出流的同样方法结合。不同的是，这次我们将发回已经历的时间间隔计数，因此返回类型将是 `impl Stream<Item = u32>`，我们可调用函数 `get_intervals`（参见清单 17-36）。


文件名：`src/main.rs`


```rust
fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let mut count = 0;
        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;
            tx.send(count).unwrap();
        }
    });

    ReceiverStream::new(rx)
}
```

*清单 17-36：以一个每毫秒都将被发射的计数器，创建出一个流*


我们以在该任务中定义一个 `count` 开始。(我们也可在该任务外定义它，但限制任何给定变量的作用域会更加清晰。）然后，我们创建了个无限循环。该循环的每次迭代，都会异步地休眠一毫秒，递增那个计数，然后将其发送到通道上。由于这都封装在由 `spawn_task` 创建的任务中，因此包括无限循环在内的所有内容，都将与运行时一起被清理。


在异步的 Rust 中，这种只会在整个运行时被破坏时，才结束的无限循环相当常见：许多程序都需要无限期地运行下去。在异步下，只要在该循环的每次迭代中，至少有一个等待点，就不会阻塞其他任何事情。


现在，回到咱们主函数的异步代码块，我们可以尝试合并 `messages` 与 `intervals` 两个流，如下清单 17-37 所示。


文件名：`src/main.rs`


```rust
        let messages = get_messages().timeout(Duration::from_millis(200));
        let intervals = get_intervals();
        let merged = messages.merge(intervals);
```

*清单 17-37：尝试合并 `messages` 与 `intervals` 两个流*


我们以调用 `get_intervals` 开始。然后，我们以 `merge` 方法，合并 `messages` 与 `intervals` 两个流，该方法会将多个流合并为一个，在任一来源流中项目可用时，就会立即产生项目，而不会强加任何特定顺序。最后，我们会对合并后的流，而不再对 `messages` 循环。


此刻，`messages` 与 `intervals` 二者都不需要固定或可变，因为二者都将被合并为单一的 `merged` 流。然而，这个到 `merge` 的调用不会编译！(`while let` 循环中的 `next` 调用也不会编译，不过我们会再讨论这个问题。）这是因为两个流的类型不同。`messages` 流的类型是 `Timeout<impl Stream<Item = String>>`，其中 `Timeout` 是实现了 `Stream` 的某个 `timeout` 调用的类型。`intervals` 流的类型为 `impl Stream<Item = u32>`。要合并这两个流，我们需要转换其中一个，以匹配另一个。我们将重新设计间隔流，因为消息流已经是我们想要的基本格式，并且必须要处理超时错误（请参阅清单 17-38）。


文件名：`src/main.rs`


```rust
        let messages = get_messages().timeout(Duration::from_millis(200));
        let intervals = get_intervals()
            .map(|count| format!("Interval: {count}"))
            .timeout(Duration::from_secs(10));
        let merged = messages.merge(intervals);
        let mut stream = pin!(merged);
```


*清单 17-38：将 `intervals` 流的类型与 `messages` 流的类型对齐*

首先，我们可使用 `map` 这个辅助方法，将 `intervals` 转换为字符串。其次，我们需要与 `messages` 中的 `Timeout` 匹配。不过，由于我们实际上并不 *想要* `intervals` 的某个超时，因此我们可以创建一个比我们所使用的其他持续时间，更长的一个超时。在这里，我们使用 `Duration::from_secs(10)` 创建了一个 10 秒的超时。最后，我们需要将 `stream` 可变，这样 `while let` 循环的 `next` 调用，就可以遍历这个流，并将其固定，确保这样做是安全的。这样就 *差不多* 达到了我们需要的效果。其中一切都通过了类型检查。在咱们运行这个程序时，会有两个问题。首先，他永远不会停止！咱们需要用 `ctrl-c` 停止他。其次，英文字母的消息，会埋没在全部间隔计数器消息中间。


```console
--snip--
Interval: 38
Interval: 39
Interval: 40
Interval: 41
Interval: 42
Interval: 43
Interval: 44
Interval: 45
Interval: 46
Interval: 47
Message: 'a'
Interval: 48
Interval: 49
Interval: 50
--snip--
```


下面清单 17-39 给出了解决这最后两个问题的一种方式。


文件名：`src/main.rs`


```rust
        let messages = get_messages().timeout(Duration::from_millis(200));
        let intervals = get_intervals()
            .map(|count| format!("Interval: {count}"))
            .throttle(Duration::from_millis(100))
            .timeout(Duration::from_secs(10));
        let merged = messages.merge(intervals).take(20);
        let mut stream = pin!(merged);
```


*清单 17-39：使用 `throttle` 与 `take` 管理合并的两个流*


首先，我们在 `intervals` 上使用了 `throttle` 方法，这样他就不会淹没 `messages` 流。所谓 *节流*，是种限制函数调用速率的方式 -- 或者说，在本例中，就是限制对该流轮询的频率。每 100 毫秒轮询一次就可以了，因为我们的消息大概就是以这个频率到达的。


要限制我们从某个流中接受项目的数量，我们对 `merged` 流应用了 `take` 方法，因为我们要限制的是最终输出，而不仅仅是其中一个流或另一个流。


现在，当我们运行这个程序时，他会在从该流中拉取 20 个条目后停止，且间隔时间不会淹没消息。我们也不会得到 `Interval: 100` 或 `Interval: 200` 等消息，而是得到 `Interval: 1`、`Interval: 2` 等消息 -- 尽管我们的源流，可以每毫秒产生一个事件。这是因为其中的 `throttle` 调用，会产生一个封装了原始流的新流，这样原始流就只能以节流速率，而不是其自己 “原生” 速率被轮询。我们没有了一堆的我们选择忽略的那些未处理间隔消息。相反，我们从一开始，就不会产生出这些间隔消息！这是 Rust 未来值固有的 “懒惰” 再次发挥了作用，允许我们选择一些性能特性。


```console
Interval: 1
Message: 'a'
Interval: 2
Interval: 3
Problem: Elapsed(())
Interval: 4
Message: 'b'
Interval: 5
Message: 'c'
Interval: 6
Interval: 7
Problem: Elapsed(())
Interval: 8
Message: 'd'
Interval: 9
Message: 'e'
Interval: 10
Interval: 11
Problem: Elapsed(())
Interval: 12
```

我们还需要处理最后一件事：出错！对于这两个基于通道的流，当通道的一侧关闭时，`send` 调用可能会失败 -- 这只是运行时执行组成流的未来值方式的问题。到目前为止，通过调用 `unwrap`，我们忽略了这种可能性，但在行为完备的应用中，我们应该显式地处理这个出错，至少要结束循环，从而咱们不再尝试发送任何消息。下面清单 17-40 展示了一种简单的出错策略：打印出问题，然后从循环中 `break`。


```rust
fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];

        for (index, message) in messages.into_iter().enumerate() {
            let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;

            if let Err(send_error) = tx.send(format!("Message: '{message}'")) {
                eprintln!("Cannot send message '{message}': {send_error}");
                break;
            }
        }
    });

    ReceiverStream::new(rx)
}

fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let mut count = 0;
        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;

            if let Err(send_error) = tx.send(count) {
                eprintln!("Could not send interval {count}: {send_error}");
                break;
            };
        }
    });

    ReceiverStream::new(rx)
}
```

*清单 17-40：处理出错并关闭循环*


与往常一样，处理消息发送出错的正确方法各有不同；只要确保咱们有种策略就可以了。


既然我们已经看到了实践种的大量异步，那么我们来退后一步，深入探讨一下 Rust 用于实现异步的 `Future`、`Stream` 及其他关键特质的一些细节。


（End）


