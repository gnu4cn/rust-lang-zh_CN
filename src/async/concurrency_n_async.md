# 以异步实现并发

在这一小节中，我们将应用异步到我们在第 16 章中，以线程解决的一些相同并发挑战。由于我们已经在那里讨论过许多的关键思想，因此在这一小节中我们将重点探讨线程和未来值之间的区别。

在许多情形下，使用异步处理并发的 API，与使用线程的非常相似。而在其他情形下，二者最终会截然不同。即使 API 在线程与异步之间 *看起来* 相似，他们往往有着不同的行为 -- 而且他们几乎总是有着不同的性能特征。


## 以 `spawn_task` 创建新任务

我们在第 16 章中 [以 `spawn` 创建新线程](../concurrency/threads.md#以-spawn-创建新线程) 小节中解决的第一个操作是在两个单独线程上计数。我们来使用异步执行同样的操作。`trpl` 代码箱提供了一个看起来与 `thread::spawn` API 非常相似的 `spawn_task` 函数，以及作为 `thread::sleep` API 的异步版本的 `sleep` 函数。我们可以一起使用这两个函数实现计数示例，如下清单 17-6 中所示。

<a name="listing_17-6"></a>
文件名：`src/main.rs`

```rust
use std::time::Duration;

fn main() {
    trpl::block_on( async {
        trpl::spawn_task( async {
            for i in 1..10 {
                println!("hi 来自第一个任务的数字 {i} !");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("hi 来自第二个任务的数字 {i} !");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    });
}
```

**清单 17-6**：创建一个新任务来打印一项内容，同时主任务打印其他内容

作为我们的起点，我们以 `trpl::block_on` 设置主函数，以便我们的顶级函数可以是异步的。

> **注意**：从本章的这里开始，每个示例都将在 `main` 中包含这种带有 `trpl::block_on` 的完全相同的封装代码，因此我们通常会像对待 `main` 一样跳过他。请记住要在咱们的代码中包含他！


然后我们在该代码块中编写两个循环，每个循环都包含一个 `trpl::sleep` 调用，其会在发送下一条消息前等待半秒（500 毫秒）。我们将一个循环放在 `trpl::spawn_task` 的主体中，另一个放在顶级 `for` 循环中。我们还在 `sleep` 调用之后添加了 `await` 这个后缀关键字。

这段代码的行为与基于线程的实现类似 -- 包括当咱们运行他时，可能看到消息以不同的顺序出现：


```console
hi 来自第二个任务的数字 1 !
hi 来自第一个任务的数字 1 !
hi 来自第一个任务的数字 2 !
hi 来自第二个任务的数字 2 !
hi 来自第一个任务的数字 3 !
hi 来自第二个任务的数字 3 !
hi 来自第一个任务的数字 4 !
hi 来自第二个任务的数字 4 !
hi 来自第一个任务的数字 5 !
```


这个版本会在主异步代码块的主体中的 `for` 循环结束时立即停止，因为当 `main` 函数结束时，`spawn_task` 生成的任务会被关闭。若咱们希望程序一直运行到该任务完成，则将需要使用一个联合句柄，a join handle，来等待第一个任务完成。在线程下，我们曾使用 `join` 方法来“阻塞” 直到线程运行完毕。在下面的清单 17-7 中，我们可以使用 `await` 完成同样的事情，因为任务句柄，the task handle，本身就是个未来值。他的 `Output` 类型是个 `Result`，因此我们也可以在等待他后对其解包。

<a name="listing_17-7"></a>
文件名：`src/main.rs`

```rust
        let handle = trpl::spawn_task( async {
            for i in 1..10 {
                println!("hi 来自第一个任务的数字 {i} !");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("hi 来自第二个任务的数字 {i} !");
            trpl::sleep(Duration::from_millis(500)).await;
        };

        handle.await.unwrap();
```

**清单 17-7**：对联合句柄使用 `await` 来运行任务至完成

这一更新后的版本会一直运行，直到 *两个* 循环都结束。

```console
hi 来自第二个任务的数字 1 !
hi 来自第一个任务的数字 1 !
hi 来自第一个任务的数字 2 !
hi 来自第二个任务的数字 2 !
hi 来自第一个任务的数字 3 !
hi 来自第二个任务的数字 3 !
hi 来自第一个任务的数字 4 !
hi 来自第二个任务的数字 4 !
hi 来自第一个任务的数字 5 !
hi 来自第一个任务的数字 6 !
hi 来自第一个任务的数字 7 !
hi 来自第一个任务的数字 8 !
hi 来自第一个任务的数字 9 !
```


到目前为止，看起来异步和线程给予了我们类似的结果，只是语法不同：对联合句柄使用 `await` 而不是调用 `join`，以及等待 `sleep` 调用。

更大的区别在于，我们无需为了实现这一目的而生成另一个操作系统线程。事实上，我们甚至不需要在这里生成一个任务。由于异步代码块会编译为匿名未来值，我们可以防止两个循环于一个异步代码块中，然后使用 `trpl::join` 函数让运行时同时运行他们至完成。

在第 16 章中的 [等待所有线程结束](../concurrency/threads.md#等待所有线程结束) 小节，我们展示了怎样对咱们调用 `std::thread::spawn` 时返回的 `JoinHandle` 类型使用 `join` 方法。`trpl::join` 函数类似，不过针对未来值。当咱们给予他两个未来值时，他会生成单个新的未来值，其输出是个元组，包含着咱们传入的两个未来值在 *全部* 完成后各自的输出。因此，在下面清单 17-8 中，我们使用 `trpl::join` 等待 `fut1` 和 `fut2` 都完成。我们 *并未* 等待 `fut1` 和 `fut2`，而是等待 `trpl::join` 生成的新的未来值。我们忽略了输出，因为他只是包含了两个单元值的元组。

<a name="listing_17-8"></a>
文件名：`src/main.rs`

```rust
        let fut1 = async {
            for i in 1..10 {
                println!("hi 来自第一个任务的数字 {i} !");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut2 = async {
            for i in 1..5 {
                println!("hi 来自第二个任务的数字 {i} !");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        trpl::join(fut1, fut2).await;
```

**清单 17-8**：使用 `trpl::join` 等待两个匿名未来值

当我们运行这段代码时，我们会看到两个未来值都会运行至完成：


```console
hi 来自第一个任务的数字 1 !
hi 来自第二个任务的数字 1 !
hi 来自第一个任务的数字 2 !
hi 来自第二个任务的数字 2 !
hi 来自第一个任务的数字 3 !
hi 来自第二个任务的数字 3 !
hi 来自第一个任务的数字 4 !
hi 来自第二个任务的数字 4 !
hi 来自第一个任务的数字 5 !
hi 来自第一个任务的数字 6 !
hi 来自第一个任务的数字 7 !
hi 来自第一个任务的数字 8 !
hi 来自第一个任务的数字 9 !
```

现在，咱们每次都将看到完全相同的顺序，这与我们在线程下以及清单 17-7 中 `trpl::spawn_task` 下看到的情况大不不同。这是因为 `trpl::join` 函数是 *公平的*，这意味着他会以相等频率检查各个未来值，在二者之间交替进行，并且绝不会让其中一个抢先，即使另一个已经就绪。在线程下，操作系统决定要检查哪个线程以及让他运行多长时间。在异步 Rust 下，运行时决定检查哪个任务。(实际上，细节会变得复杂，因为异步运行时可能在底层使用操作系统的线程，作为其管理并发的一部分，因此保证公平性对于运行时来说可能需要更多工作 -- 但这仍然是可行的！）运行时不必保证任何给定操作的公平性，他们通常提供不同的 API，来让咱们选择是否需要公平性。

请对等待未来值尝试以下这些变化，并看看他们有什么作用：

- 移除任一循环，或同时两个循环中的异步代码块；
- 在定义出每个异步代码块后，立即等待他们；
- 仅将第一个循环封装在异步代码块中，并在第二个循环的主体之后等待生成的未来值。

作为额外挑战，看看咱们是否能在运行代码 *前*，得出每种情况下的输出结果！


## 使用消息传递在两个任务之间发送

在未来值之间共用数据也将很常见：我们将再次使用消息传递，但这次是在异步版本的类型和函数下。我们将采取与第 16 章中 [通过消息传递在线程间传输数据](../concurrency/message_passing.md) 小节中的略有不同的路径，以演示基于线程和基于未来值的并发之间的一些关键区别。在下面清单 17-9 中，我们将仅从单个异步代码块开始 -- 而 *不是* 像我们生成单个线程那样生成单个任务。

<a name="listing_17-9"></a>
文件名：`src/main.rs`

```rust
        let (tx, mut rx) = trpl::channel();

        let val = String::from("hi");
        tx.send(val).unwrap();

        let received = rx.recv().await.unwrap();
        println!("收到 {received}");
```

**请单 17-9**：创建异步信道，并指派两端给 `tx` 与 `rx`

在这里，我们使用 `trpl::channel`，这是我们在第 16 章中与线程一起使用的多生产者、单消费者信道 API 的异步版本。这一 API 的异步版本与基于线程的版本只有细微差别：他使用可变接收器 `rx` 而不是不可变的，并且他的 `recv` 方法产生一个我们需要等待的未来值，而不是直接生成值。现在，我们可以从发送方发送消息到接受方。请注意，我们不必生成单独的线程甚至任务；我们只需等待 `rx.recv` 调用。


`std::mpsc::channel` 中的同步 `Receiver::recv` 方法会在接收到消息前一直阻塞。而 `trpl::Receiver::recv` 方法则不会，因为他属于异步的。他不会阻塞，而是交还控制权给运行时，直到收到消息或信道的发送侧关闭。相比之下，我们不会等待 `send` 调用，因为他不会阻塞。他之所以无需阻塞，因为我们发入消息的信道是不受限的 <sup>1</sup>。

> **译注**：
>
> <sup>1</sup>：the channel we're sending it into is unbounded.一个有效的、不发散的程序所能占用的空间和时间并没有先验的固定限制。
>
> 参考：[Unbounded nondeterminism](https://en.wikipedia.org/wiki/Unbounded_nondeterminism)



> **注意**：由于所有这些异步代码都在 `trpl::block_on` 调用的异步代码块中运行，因此其中的所有代码都可避免阻塞。但是，异步代码块 *外部* 的代码将在 `block_on` 函数运行时阻塞。这正是 `trpl::block_on` 函数的核心意义：他让咱们可以 *选择* 于何处阻塞某段异步代码，从而选择了于何处在同步代码和异步代码之间切换。


请注意这个示例中的两点。首先，消息将立即到达。其次，尽管我们在这里使用了未来值，但还并没有并发。清单中的所有操作都是按顺序发生的，就像不涉及未来值一样。

我们来通过发送一系列消息并在每次发送之间休眠，解决第一部分，如下清单 17-10 中所示。

<a name="listing_17-10"></a>
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
            println!("收到 '{value}'");
        }
```

**请单 17-10**：通过异步信道发送并接收多条消息，并在每条消息之间通过 `await` 休眠

除了发送消息外，我们还需要接收他们。在这一情形下，由于我们知道有多少条消息传入，因此可以通过手动调用 `rx.recv().await` 四次完成接收。但在现实世界中，我们通常将等待 *未知* 数量的消息，因此需要持续等待，直到确定不再有信息为止。

在 [清单 16-10](../concurrency/message_passing.md#listing_16-10) 中，我们使用 `for` 循环来处理从同步通道接收到的所有项目。然而，Rust 还没有对 *异步生成的* 项目序列使用 `for` 循环的方法，因此我们需要使用一种以前从未见过的循环：`while let` 条件循环。这是我们在第 6 章中 [`if let` 与 `let else` 下的简明控制流](../enums_and_pattern_matching/if-let_control_flow.md) 小节中，见过的 `if let` 结构的循环版本。只要其指定的模式继续与值匹配，该循环就会继续执行。

其中的 `rx.recv` 调用会生成一个未来值，我们等待该未来值。运行时将暂停该未来值，直到他准备就绪。一旦消息到达，该未来值就将解析为 `Some(message)`，且解析次数与消息到达次数相同。当信道关闭时，无论是否 *有* 消息到达，该未来值都将解析为 `None`，以表示不再有值，因此我们应该停止轮询 -- 即停止等待（未来值）。

`while let` 循环将所有这一切整合在一起。当调用 `rx.recv().await` 的结果是 `Some(message)` 时，我们得到对消息的访问，并可以在循环体中使用他，就像在 `if let` 下一样。当结果为 `None` 时，该循环结束。循环每次完成时，他都会再次遇到等待点，因此运行时会再次暂停他，直到另一条消息到达。

该代码现在成功地发送并接收所有消息。遗憾的是，仍然存在一些问题。其中之一便是，消息没有以半秒为间隔到达。他们会在我们启动程序 2 秒（2000 毫秒）后一次性全部到达。另外，这个程序还永远不会退出！相反，他会无限期等待新消息。咱们将需要用时 `Ctrl-c` 关闭他。


### 同一个异步代码块内的代码会线性地执行

我们以检查为什么消息会在全部延迟后一次性传入，而不是每条消息之间带有延迟的传入。在给定的异步代码块内，`await` 关键字在代码中出现的顺序，也是他们在程序运行时被执行的顺序。

清单 17-10 中只有一个异步代码块，因此其中的所有操作是线性运行的。这里仍然不存在并发。所有 `tx.send` 调用都会发生，其中穿插着所有 `trpl::sleep` 调用及其相关的等待点。在这之后，`while let` 循环才会开始处理 `recv` 调用上的任何等待点。

为了获得我们想要的行为，即每条消息之间睡眠延迟得以发生，我们需要放置 `tx` 和 `rx` 操作于他们自己的异步代码块中，如下清单 17-11 中所示。然后，运行时就可以使用 `trpl::join` 单独执行他们，就像在 [清单 17-8](#listing_17-8) 中那样。再次，我们等待调用 `trpl::join` 的结果，而不是单个未来值。如果我们按顺序等待各个未来值，最终只会回到顺序执行流程 -- 这正是我们试图 *不* 要实现的。

<a name="listing_17-11"></a>
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
                println!("收到 '{value}'");
            }
        };

        trpl::join(tx_fut, rx_fut).await;
```

**清单 17-11**：将 `send` 和 `recv` 分离到他们自己的异步代码块中，并等待这两个代码块的未来值

在清单 17-11 中更新后的代码下，消息会以 500 毫秒的间隔打印，而不是在 2 秒后一次性全部打印出来。


### 迁移所有权到异步代码块中

不过，由于 `while let` 循环与 `trpl::join` 交互方式的原因，程序仍然永远不会退出：

- 只有在传给 `trpl::joing` 的两个未来值 *都* 完成后，其返回的未来值才会完成；
- `tx_fut` 未来值在发送完 `vals` 中最后一条消息并完成休眠后就会完成；
- 在 `while let` 循环结束前，`rx_fut` 这个未来值不会完成；
- 在等待 `rx` 产生 `None` 之前，`while let` 循环不会结束；
- 只有在信道另一端关闭后，等待 `rx.recv` 才会返回 `None`；
- 只有在我们调用 `rx.close` 时，或发送侧 `tx` 被弃用（译注：超出作用域被内存回收）时，信道才会关闭；
- 我们并未在任何地方调用 `rx.close`， 且在传递给 `trpl::run` 的最外层异步代码块结束前，`tx` 不会被弃用；
- 该代码块无法结束，因为他被阻塞在 `trpl::join` 的完成中，这又将我们带回到代码清单的顶部。

目前，我们发送消息的异步代码块仅 *借用* `tx`，因为发送消息不需要所有权，但如果我们可以 *迁移* `tx` 到该异步代码块中，他就会在该代码块结束时被弃用。在第 13 章中 [捕获引用抑或迁移所有权](../functional_features/closures.md#捕获引用抑或迁移所有权) 小节中，咱们学习了如何在闭包中使用 `move` 关键字，并且正如第 16 章中 [对线程使用 `move` 的闭包](../concurrency/threads.md#对线程使用-move-的闭包) 小节中讨论的，在使用线程时，我们经常需要迁移数据到闭包中。同样的动因也适用于异步代码块，因此 `move` 关键字适用于异步代码块，就像他适用于闭包一样。

在下面清单 17-12 中，我们将用于发送消息的代码块从 `async` 修改为 `async move`。

<a name="listing_17-12"></a>
文件名：`src/main.rs`

```rust
        let (tx, mut rx) = trpl::channel();

        let tx_fut = async move {
            // -- 跳过代码 --
```

**清单 17-12**：对清单 17-11 中代码的修订，可在完成时正确关闭

当我们运行 *这一* 版本的代码时，他就会在最后一条消息发送和接收后优雅地关闭。接下来，我们来看看为了从多个未来值发送数据，将需要修改些什么。


### 通过 `join!` 宏合并/联合多个未来值

这个异步信道也是个多生产者信道，因此当我们打算从多个未来值发送消息时，我们可以对 `tx` 调用 `clone`，如下清单 17-13 中所示。


<a name="listing_17-13"></a>
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
                println!("收到 '{value}'");
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

        trpl::join!(tx1_fut, tx_fut, rx_fut).await;
```

**清单 17-13**：对异步代码块使用多生产者

首先，我们克隆 `tx`，在第一个异步代码块外创建 `tx1`。我们迁移 `tx1` 到该代码块中，就像之前对 `tx` 所做的那样。随后，我们迁移原始的 `tx` 迁移到一个 *新的* 异步代码块中，那里我们以稍慢的延迟发送更多消息。我们恰好把这个新的异步代码块，放在接收消息的异步代码块之后，但他也可以其之前。关键在于这些未来值等待的顺序，而不是他们创建的顺序。

发送信息的两个异步代码块都需要是 `async move` 的代码块，这样当这两个代码块完成时，`tx` 和 `tx1` 都会被弃用。否则，我们将最终回到最初陷入的哪个无限循环中。

最后，我们从 `trpl::join` 切换为 `trpl::join!` 来处理额外的未来值：`join!` 宏会等待任意数量的未来值，其中我们在编译时直到未来值的数量。我们将在这一章后面讨论等待未知数量的未来值的集合。

现在，我们会看到来自两个发送未来值的所有消息，并且由于两个发送未来值在发送后，使用略有不同的延迟，因此消息也会以不同的时间间隔接收。


```console
收到 'hi'
收到 'more'
收到 'from'
收到 'the'
收到 'messages'
收到 'future'
收到 'for'
收到 'you'
```

我们已经探讨了

- 如何使用消息传递，在未来值之间发送数据、
- 异步代码块内的代码如何顺序执行、
- 怎样迁移所有权到异步代码块中、
- 以及怎样合并多个未来值。


接下来，我们来讨论怎样以及为何要告知运行时，他可以切换到另一个任务。

（End）


