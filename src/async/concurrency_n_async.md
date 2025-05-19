# 应用带有异步的并发

**Applying Concurrency with Async**


在本节中，我们将把异步应用到第 16 章中，咱们在线程上所面临的一些并发挑战。由于我们已经在第 16 章中，讨论过很多关键思想，因此本节我们将重点讨论线程与未来值之间的不同之处。


在许多情形下，使用异步处理并发的 API，与使用线程处理并发的 API 非常相似。而在其他情形下，二者最终会截然不同。即使线程与异步的 API *看起来* 很相似，他们也往往有着不同行为 -- 而且他们几乎总是有着不同的性能特征。


## 使用 `spawn_task` 创建新任务


在 [使用 Spawn 创建新线程](../concurrency/threads.md#使用-spawn-函数创建新线程) 小节中，我们解决的首项操作，是在两个独立线程上计数。现在我们来使用异步，完成同样的事情。`trpl` 代码箱提供了个与 `thread::spawn` 这个 API 非常相似的 `spawn_task` 函数，以及一个 `thread::sleep` API 异步版本的 `sleep` 函数。我们可以一并使用这两个函数，实现那个计数示例，如清单 17-6 所示。


文件名：`src/main.rs`


```rust
use std::time::Duration;

fn main() {
    trpl::run( async {
        trpl::spawn_task( async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    });
}
```

*清单 17-6：创建出在主任务打印其他内容时，打印一个东西的任务*


作为咱们的起点，我们以 `trpl::run` 设置了咱们的主函数，这样我们的顶层函数就可以是异步的了。


> **注意**：从本章的这里开始，每个示例都将在 `main` 中，包含以 `trpl::run` 封装的完全相同代码，因此我们通常将跳过 `trpl::run`，就像跳过 `main` 一样。请不要忘记在咱们的代码中加入他！


然后我们在该代码块中写了两个循环，每个循环都包含了个 `trpl::sleep` 调用，这会在发送下一条消息前等待半秒（500 毫秒）。我们将一个循环放在 `trpl::spawn_task` 的主体中，另一个放在一个顶层的 `for` 循环中。在 `sleep` 调用后，我们还添加了个 `await`。


这段代码的行为与基于线程的实现类似 -- 包括当咱们运行这段代码时，在咱们自己终端中可能看到消息以不同顺序出现这一情况：


```console
hi number 1 from the second task!
hi number 1 from the first task!
hi number 2 from the first task!
hi number 2 from the second task!
hi number 3 from the first task!
hi number 3 from the second task!
hi number 4 from the first task!
hi number 4 from the second task!
hi number 5 from the first task!
```


这个版本会在主异步代码块主体中的 `for` 循环结束时立即停止，因为由 `spawn_task` 生成的任务，在 `main` 函数结束时会被关闭。若咱们想要他一直运行到任务完成，就将需要使用一个联合句柄，a join handle，等待第一个任务完成。在线程下，我们曾使用 `join` 方法，在线程运行完毕前予以 “阻塞”。在下面的清单 17-7 中，我们可使用 `await` 完成同样的事情，因为任务句柄，the task handle，本身就是个未来值。他的 `Output` 类型是个 `Result`，因此我们也可以在等待他后，对其解封装。


文件名：`src/main.rs`


```rust
        let handle = trpl::spawn_task( async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }

        handle.await.unwrap();
```

*清单 17-7：使用 `await` 与联合句柄，运行任务到完成*


这个更新后的版本，会运行到 *两个循环* 都结束为止。


```console
hi number 1 from the second task!
hi number 1 from the first task!
hi number 2 from the first task!
hi number 2 from the second task!
hi number 3 from the first task!
hi number 3 from the second task!
hi number 4 from the first task!
hi number 4 from the second task!
hi number 5 from the first task!
hi number 6 from the first task!
hi number 7 from the first task!
hi number 8 from the first task!
hi number 9 from the first task!
```


目前看来，异步与线程给到了我们同样的基本结果，只是语法不同：使用 `await` 而不是在联合句柄上调用 `join`，以及等待那个 `sleep` 调用。


更大的区别在于，我们无需启动另一个操作系统线程，来完成这项工作。事实上，我们甚至不需要在这里生成一个任务。由于异步代码块会编译为匿名的未来值，因此我们可将各个循环，放在一个异步代码块中，然后使用 `trpl::join` 函数，让运行时将他们运行完成。


在 [“使用 `join` 句柄等待所有线程结束”](../concurrency/threads.md#使用-join-句柄等待全部线程结束) 小节中，我们展示了在调用 `std::thread::spawn` 时返回的 `JoinHandle` 类型上，如何使用 `join` 方法。这个 `trpl::join` 函数与之类似，不过用于未来值。当咱们给到他两个未来值时，他会产生出一个其输出为元组的新未来值，该元组中包含着咱们所传入的各个未来值完成后的输出。因此，在清单 17-8 中，我们使用了 `trpl::join`，等待 `fut1` 和 `fut2` 结束。我们等待的 *不是* `fut1` 和 `fut2`，而是由 `trpl::join` 生成的那个新未来值。我们会忽略输出，因为他只是个包含了两个单元值的元组。


文件名：`src/main.rs`


```rust
        let fut1 = async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut2 = async {
            for i in 1..5 {
                println!("hi number {i} from the second task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        trpl::join(fut1, fut2).await;
```

<a name="listing-17-8"></a> *清单 17-8：使用 `trpl::join` 等待两个匿名未来值*

在运行此代码时，我们会看到两个未来值都会运行至完成：


```console
hi number 1 from the first task!
hi number 1 from the second task!
hi number 2 from the first task!
hi number 2 from the second task!
hi number 3 from the first task!
hi number 3 from the second task!
hi number 4 from the first task!
hi number 4 from the second task!
hi number 5 from the first task!
hi number 6 from the first task!
hi number 7 from the first task!
hi number 8 from the first task!
hi number 9 from the first task!
```


现在，咱们将看到每次都完全相同的顺序，这与我们在线程下看到的情况截然不同。这是因为 `trpl::join` 函数是 *公平的*，意味着他会以相同频率检查各个未来值，在二者之间交替进行，并绝不会在一个已就绪时，让另一个超前。在线程下，是由操作系统决定要检查哪个线程，以及让他运行多久。而在异步的 Rust 下，是由运行时决定要检查哪个任务。(在实践中，细节会变得复杂，因为异步运行时可能会在表象之下，使用操作系统的线程，作为其管理并发性的一部分，所以对运行时来说，保证公平性可能会更费事 -- 但这仍然是可行的！）运行时不必保证对任何给定操作的公平性，他们通常提供不同的 API，让咱们选择是否需要公平性。

请在等待未来值上，尝试以下这些变化，看看他们能做些什么：


- 移除任一循环，或同时两个循环的异步代码块；
- 在定义出各个异步代码块后，立即等待他们；
- 只将第一个循环封装在异步代码块中，并在第二个循环的主体后，等待所得到的未来值。


作为额外挑战，请在运行代码 *前*，看看咱们能否得出每种情况下的输出结果！



## 使用消息传递在两个任务上计数


在未来值间共用数据也将很常见：再次我们将使用消息传递，但这次将使用异步版本的类型与函数。我们将采用与咱们在 [使用消息传递在线程间传输数据](../concurrency/message_passing.md) 小节中略微不同的路径，说明基于线程与基于未来值的并发间的一些关键区别。在清单 17-9 中，我们将从单个的异步代码块开始 -- 而 *不是* 像咱们曾生成单个线程时，生成单个任务。


文件名：`src/main.rs`


```rust
        let (tx, mut rx) = trpl::channel();

        let val = String::from("hi");
        tx.send(val).unwrap();

        let received = rx.recv().await.unwrap();
        println!("Got: {received}");
```

*请单 17-9：创建出一个异步通道并将其中两半分别赋值给 `tx` 与 `rx`*

这里，我们使用了我们在第 16 章中，与线程一起使用的多生产者、单消费者通道 API 的一个异步版本 `trpl::channel`。该 API 的异步版本，与基于线程的版本只有一点不同：他使用了可变的接收器 `rx`，而不是不可变接收器 `rx`，而且他的 `recv` 方法会产生一个我们需要等待的未来值，而不是直接产生值。现在，我们可以从发送方往接收方发送消息了。请注意，我们不必生成单独线程，甚至不需要生成任务；我们只需等待这个 `rx.recv` 调用。


`std::mpsc::channel` 中的同步 `Receiver::recv` 方法，会在收到消息前一直阻塞。而 `trpl::Receiver::recv` 这个方法不会，因为他是异步的。他不会阻塞，而是在消息被接收或通道的发送侧关闭前，会将控制权交还给运行时。相比之下，我们不会等待 `send` 调用，因为他不会阻塞。他之所以无需等待，是因为我们将消息发入的通道，是不受限的 <sup>1</sup>。


> **译注**：
>
> <sup>1</sup>：the channel we're sending it into is unbounded.一个有效的、不发散的程序所能占用的空间和时间并没有先验的固定限制。
>
> 参考：[Unbounded nondeterminism](https://en.wikipedia.org/wiki/Unbounded_nondeterminism)


> **注意**：由于所有这些异步代码，都在一个 `trpl::run` 调用的异步代码块中运行，因此其中的所有代码，都可避免阻塞。但是，当 `run` 函数返回时，异步代码块 *外部* 的代码会阻塞。这正是 `trpl::run` 函数的意义所在：他可以让咱们 *选择*，在哪些地方阻塞某些异步代码，从而在哪些地方切换同步代码和异步代码。在大多数异步运行时中，`run` 其实被命名为 `block_on`，正是出于这个原因。


请注意这个例子的两点。首先，消息将立即到达。其次，虽然我们在这里使用了一个未来值，但这里还没有并发。该列表中的所有事情，都是按顺序发生的，就像没有涉及到未来值一样。


我们来通过发送一系列消息并在中间休眠，解决第一部分问题，如清单 17-10 所示。


文件名：`src/main.rs`


```rust
        let (tx, mut rx) = trpl::channel();

        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("future"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }

        while let Some(value) = rx.recv().await {
            println!("received '{value}'");
        }
```

*请单 17-10：通过异步通道发送及接收多条消息，并在每条消息之间对 `await` 休眠*

除发送消息外，我们还需要接收他们。在本例中，由于我们知道有多少条消息进来，因此我们可通过调用 `rx.recv().await` 四次，手动完成接收。但在真实世界中，我们一般会等待 *未知* 数量的消息，因此我们需要一直等待，直到确定没有更多信息为止。


在 [清单 16-10](../concurrency/message_passing.md#listing-16-10) 中，我们使用了个 `for` 循环，处理从同步通道接收到的所有项目。然而，Rust 还没有一种，为 *异步的* 序列项目编写 `for` 循环的方法，因此我们需要使用一种以前从未见过的循环：`while let` 条件循环。这是我们曾在 [“使用 `if let` 与 `let else` 的简明控制流”](../enums_and_pattern_matching/if-let_control_flow.md) 小节中，看到的 `if let` 结构的循环版本。只要循环所指定的模式继续匹配值，其就会继续执行。


其中的 `rx.recv` 调用，会产生一个我们所等待的未来值。在其准备好前，运行时将暂停该未来值。一旦有消息到达，这个未来值将解析为 `Some(message)`，解析次数与消息到达次数相同。在通道关闭时，无论 *有多少* 消息到达，该未来值都会解析为表示不再有值的 `None`，并因此我们就应停止轮询 -- 即停止等待。


那个 `while let` 循环，将所有这一切联系在了一起。在调用 `rx.recv().await` 的结果为 `Some(message)` 时，我们就可以访问到消息，并在循环体中使用他，就跟使用 `if...let` 一样。在结果为 `None` 时，则该循环结束。每次循环完毕时，其都会再次遇到等待点，因此运行时会再度将其暂停，直到另一条消息到达。


代码现在可以成功发送并接收所有信息。遗憾的是，其间仍然存在一些问题。首先，消息不是以半秒为间隔到达的。他们会在我们启动程序 2 秒（2000 毫秒）后，一次性到达。另外，这个程序永远不会退出！相反，他会一直等待新信息。咱们需要以 `Ctrl-c` 关闭他。


我们先来看看，为什么消息会在全部延迟后一次性发送，而不是每条消息之间都有延迟。在某个给定异步代码块中，`await` 关键字在代码中出现的顺序，就是他们在程序运行时，被执行的顺序。


清单 17-10 中只有一个异步代码块，因此其中的所有代码都是线性运行的。其中仍然没有并发。所有的 `tx.send` 调用，都是在所有 `trpl::sleep` 调用及其关联的等待点之间发生的。在那之后，`while let` 循环才进入 `recv` 调用上的任何等待点。


要实现我们所期望的行为，即在每条消息之间发生睡眠延迟，我们需要将 `tx` 和 `rx` 操作，放在他们各自的异步代码块中，如下清单 17-11 所示。然后，运行时可使用 `trpl::join`，分别执行这两个操作，就像 [计数示例](#listing-17-8) 中那样。再一次，我们等待的是调用 `trpl::join` 的结果，而不是各个未来值。如果我们按顺序等待单个未来值，我们就会回到一种顺序流程中 -- 这正是我们 *不* 想做的。


文件名：`src/main.rs`


```rust
        let (tx, mut rx) = trpl::channel();

        let tx_fut = async {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        trpl::join(tx_fut, rx_fut).await;
```

*清单 17-11：通过异步通道发送及接收多条消息，并在每条消息之间以一个 `await` 休眠*


有了清单 17-11 中更新后的代码，消息将以 500 毫秒的间隔打印，而不是在 2 秒后匆忙全部打印出来。

不过，由于 `while let` 循环与 `trpl::join` 交互方式的原因，该程序仍然不会退出：


- 自 `trpl::join` 返回的未来值，只有在传给他的两个未来值 *都* 完成后才会完成；
- `tx` 这个未来值，在其发送完 `vals` 中最后一条消息，结束休眠后会立即完成；
- 在 `while let` 循环结束后，`rx` 这个未来值才会完成；
- 在等待 `rx.recv` 产生 `None` 前，`while let` 这个循环不会结束；
- 只有在通道另一端关闭后，等待 `rx.recv` 才会返回 `None`；
- 只有当我们调用 `rx.close` 时，或发送端 `tx` 被弃用（译注：超出作用域被内存回收）时，通道才会关闭；
- 我们未在任何地方调用 `rx.close`，同时在传递给 `trpl::run` 的外层异步代码块结束时，才会弃用 `tx`；
- 该代码块无法结束，因为他被阻塞于 `trpl::join` 的完成中，这让我们回到了该代码清单的顶部。



我们可通过在某处调用 `rx.close` 手动关闭 `rx`，但这样做意义不大。在处理了任意数量的消息后停止，会使这个程序关闭，但我们可能会错过消息。我们需要其他方法，确保 `tx` 在该函数结束 *前* 被弃用。


现在，我们于其中发送消息的异步代码块，只借用了 `tx`，因为发送消息不需要所有权，但如果我们能将 `tx` 迁移到该异步代码块中，那么一旦那个代码块结束，他就会被弃用。在第 13 章 [“捕获引用抑或迁移所有权”](../functional_features/closures.md#捕获引用抑或迁移所有权) 小节中，我们学习了如何在闭包中使用 `move` 关键字，而正如第 16 章 [“对线程使用 `move` 闭包”](../concurrency/threads.md#对线程使用-move-闭包) 小节中所讨论的，在使用线程时，我们经常需要将数据迁移到闭包中。同样的动因，也适用于异步代码块，因此 `move` 这个关键字在异步代码块中的作用，与在闭包中一样。


在下面的清单 17-12 中，我们将用于发送消息的代码块，从 `async` 改为了 `async move`。当我们运行 *这个* 版本的代码时，他就会在发送及接收完最后一条消息后，优雅地关闭。


文件名：`src/main.rs`


```rust
        let (tx, mut rx) = trpl::channel();

        let tx_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        trpl::join(tx_fut, rx_fut).await;

```

*清单 17-12：完成后正确关闭的清单 17-11 中代码的修订版本*


这个异步通道还是个多生产者的通道，因此在打算自多个未来值发送消息时，我们可以调用 `tx` 上的 `clone`，如下清单 17-13 所示。


文件名：`src/main.rs`


```rust
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        let tx1_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        let tx_fut = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };

        trpl::join3(tx1_fut, tx_fut, rx_fut).await;
```

*清单 17-13：对异步代码块使用多生产者*


首先，我们克隆了 `tx`，在第一个异步代码块外创建出 `tx1`。我们将 `tx1` 迁移到该代码块中，就跟之前对 `tx` 所做的那样。然后，我们将原始的 `tx` 迁移到一个 *新的* 异步代码块中，并在其中以稍慢的延迟发送更多消息。我们碰巧把这个新的异步代码块，放在接收消息的异步代码块后，但他也可以在接收信息的异步代码块前。关键在于等待未来值的顺序，而不是他们被创建出的顺序。


发送信息的两个异步代码块，都需要是 `async move` 的代码块，这样当这两个代码块完成时，`tx` 和 `tx1` 都会被弃用。否则，我们就会回到一开始的无限循环中。最后，我们将 `trpl::join` 切换为了 `trpl::join3`，以处理额外的未来值。



现在，我们可以看到来自两个发送未来值的所有消息，由于两个发送未来值在发送后，使用了略微不同的延迟，消息也是在这些不同的间隔内收到的。


```console
received 'hi'
received 'more'
received 'from'
received 'the'
received 'messages'
received 'future'
received 'for'
received 'you'
```

这是一个良好开端，但他将我们限制在少数几个未来值中：两个的 `join` 或三个的 `join3`。我们来看看，如何使用更多的未来值。


（End）


