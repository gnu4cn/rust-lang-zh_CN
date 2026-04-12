# 使用任意数量的未来值

## 将控制权交换给运行时

回顾 [我们的第一个异步程序](./futures.md#我们的第一个异步程序)，在每个等待点，Rust 都会给予运行时一个机会来暂定任务，并在等待的未来值尚未准备好时切换到另一任务。反之亦然：Rust 在等待点处 *仅* 暂停异步代码块，并交还控制权给运行时。等待点之间的一切都是同步的。

这意味着，当咱们在不带等待点的异步代码块中执行大量工作时，该未来值将阻塞任何其他未来值。咱们有时会听说这种情况被称为 “一个未来值 *饿死* 其他未来值”。在某些情况下，这可能无关紧要。然而，但咱们正在进行某种开销很高的设置或长时间运行的工作，或者有个未来值将无限期地执行某项特定任务时，咱们就需要考虑何时，以及在何处交还控制权给运行时。

我们来模拟一个长时间运行的操作，来演示 “饥饿” 问题，然后探讨怎样解决他。下面清单 17-14 引入了一个 `slow` 函数。

<a name="listing_17-14"></a>
```rust
fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' 运行了 {ms}ms");
}
```

**清单 17-14**：使用 `thread::sleep` 模拟慢速操作

这段代码使用 `std::thread::sleep` 而不是 `trpl::sleep`，以便调用 `slow` 将阻塞当前线程数毫秒。我们可以使用 `slow` 代替现实世界中即长时间运行又具有阻塞性的操作。

在下面清单 17-15 中，我们使用 `slow` 来模拟在一对未来值中，执行此类 CPU 密集型的工作。

<a name="listing_17-15"></a>
```rust
        let a = async {
            println!("'a' 已启动。");
            slow("a", 30);
            slow("a", 10);
            slow("a", 20);
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'a' 已结束。");
        };

        let b = async {
            println!("'b' 已启动。");
            slow("b", 75);
            slow("b", 10);
            slow("b", 15);
            slow("b", 350);
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'b' 已结束。");
        };

        trpl::select(a, b).await;

```

**清单 17-15**：调用 `slow` 函数来模拟慢速操作

每个未来值都 *只* 会在执行完一系列慢速操作后，交还控制权给运行时。当咱们运行这段代码时，将看到以下输出：


```console
'a' 已启动。
'a' 运行了 30ms
'a' 运行了 10ms
'a' 运行了 20ms
'b' 已启动。
'b' 运行了 75ms
'b' 运行了 10ms
'b' 运行了 15ms
'b' 运行了 350ms
'a' 已结束。
```

正如在 [清单 17-5](./futures.md#listing_17-5) 中，我们使用 `trpl::select` 让两个未来值在获取两个 URL 上竞争，`select` 仍会在 `a` 完成后立即结束。不过，这两个未来值中对 `slow` 的调用之间不存在交错执行。未来值 `a` 会执行他的所有工作，直到 `trpl::sleep` 调用被等待；随后未来值 `b` 执行所有工作，直到他自己的 `trpl::sleep` 调用被等待；最后未来值 `a` 完成。为了让两个未来值都能在他们的慢速任务之间取得进展，我们需要等待点，以便我们可以将控制权给运行时。这意味着我们需要某种可以等待的东西！

我们在清单 17-15 中已经可以看到这种控制权交接的发生：当我们移除未来值 `a` 末尾的 `trpl::sleep` 时，那么他将在未来值 `b` *完全* 未运行的情况下就完成。我们来尝试使用 `trpl::sleep` 函数作为让操作轮流取得进展的起点，如下列表 17-16 中所示。

<a name="listing_17-16"></a>
```rust
        let one_ms = Duration::from_millis(1);

        let a = async {
            println!("'a' 已启动。");
            slow("a", 30);
            trpl::sleep(one_ms).await;
            slow("a", 10);
            trpl::sleep(one_ms).await;
            slow("a", 20);
            trpl::sleep(one_ms).await;
            println!("'a' 已结束。");
        };

        let b = async {
            println!("'b' 已启动。");
            slow("b", 75);
            trpl::sleep(one_ms).await;
            slow("b", 10);
            trpl::sleep(one_ms).await;
            slow("b", 15);
            trpl::sleep(one_ms).await;
            slow("b", 350);
            trpl::sleep(one_ms).await;
            println!("'b' 已结束。");
        };
```

**清单 17-16**：使用 `trpl::sleep` 让操作轮流取得进展

我们已在每个 `slow` 调用之间添加了带有等待点的 `trpl::sleep` 调用。现在两个值的工作是交错的：

```console
'a' 已启动。
'a' 运行了 30ms
'b' 已启动。
'b' 运行了 75ms
'a' 运行了 10ms
'b' 运行了 10ms
'a' 运行了 20ms
'b' 运行了 15ms
'a' 已结束。
```

在未来值 `a` 移交控制权给 `b` 之前，他仍然会运行一段时间，因为他在调用 `trpl::sleep` 之前先调用了 `slow`；但此后，每当其中一个未来值遇到等待点时，他们就会相互交替执行。在这一情形下，我们在每次调用 `slow` 之后都会这样做，但我们也可以对我们最合理的方式拆分工作。

不过，我们其实并不希望在这里 *休眠*：我们希望尽可能快地取得进展。我们只需要交还控制权给运行时即可。我们可以使用 `trpl::yield_now` 函数，直接做到这点。在下面清单 17-17 中，我们将所有 `trpl::sleep` 调用替换为 `trpl::yield_now`。

<a name="listing_17-17"></a>
```rust
        let a = async {
            println!("'a' 已启动。");
            slow("a", 30);
            trpl::yield_now().await;
            slow("a", 10);
            trpl::yield_now().await;
            slow("a", 20);
            trpl::yield_now().await;
            println!("'a' 已结束。");
        };

        let b = async {
            println!("'b' 已启动。");
            slow("b", 75);
            trpl::yield_now().await;
            slow("b", 10);
            trpl::yield_now().await;
            slow("b", 15);
            trpl::yield_now().await;
            slow("b", 350);
            trpl::yield_now().await;
            println!("'b' 已结束。");
        };
```

**清单 17-17**：使用 `yield_now` 让操作轮流取得进展

这段代码不仅更清晰地表达了实际意图，而且比使用 `sleep` 快得多，因为诸如 `sleep` 使用的定时器，通常对其粒度有限制。例如，即使我们传递给他一个 1 纳秒的 `Duration`，我们使用的 `sleep` 版本也将最少休眠一毫秒。再说一次，现代计算机运行速度 *极快*：他们可以在一毫秒内执行大量操作！

这意味着，具体取决于咱们的程序正在执行的其他操作，异步甚至对于计算密集型任务也很有用，因为他为组织程序不同部分之间的关系，提供了一项有用的工具（但代价是异步状态机的开销）。这属于一种 *协作式多任务处理* 模式，每个未来值有权通过等待点，决定何时交出控制权。因此，每个未来值也有责任避免阻塞太久。在一些基于 Rust 的嵌入式操作系统中，这是 *唯一* 的多任务处理方式！

当然，在现实世界的代码中，咱们通常不会在每一行上都交替使用函数调用和等待点。虽然以这种方式让出控制权相对 “便宜”，但并不是毫无代价。在许多情况下，试图分解计算密集型任务，会使其速度显著变慢，因此有时简单地让操作阻塞，对 *整体* 性能会更好。请务必进行性能测试，以确定代码中实际的性能瓶颈所在。不过，当咱们大量咱们预期会并发进行的工作却在串行进行时，就要务必牢记这一底层机制！


## 构建我们自己的异步抽象

我们还可以组合未来值，创建新的模式。例如，我们可以我们已有的异步构建块，构建一个 `timeout` 函数。在我们完成后，结果将是另一个的构建块，我们可以用于创建更多的异步抽象。

下面清单 17-18 展示了我们将期望这个 `timeout` 怎样处理某个慢速未来值。


<a name="listing_17-18"></a>
```rust
        let slow = async {
            trpl::sleep(Duration::from_secs(5)).await;
            "最终完成"
        };

        match timeout(slow, Duration::from_secs(2)).await {
            Ok(message) => println!("在 '{message}' 下成功"),
            Err(duration) => {
                println!("在 {} 秒后失败", duration.as_secs())
            }
        }
```

**清单 17-18**：使用我们设想的 `timeout` 在有限时间内运行慢速操作


我们来实现这个吧！首先，我们来思考一下 `timeout` 的 API：

- 他本身需要是个异步函数，这样我们才可以等待他；
- 他的第一个参数应该是个要运行的未来值。我们可以构造其为泛型，以允许他适用于任何未来值；
- 他的第二个参数将是要等待的最大时间。若我们使用一个 `Duration`，那将使其可以轻松地传递给 `trpl::sleep`；
- 他应返回一个 `Result`。当未来值成功完成时，`Result` 将是个带有未来值产生的值的 `Ok`。当超时值先发生时，`Result` 将为带有超时等待的时长的 `Err`。


下面清单 17-19 展示了这一声明。

<a name="listing_17-19"></a>
```rust
async fn timeout<F: Future>(
    future_to_try: F,
    max_time: Duration,
) -> Result<F::Output, Duration> {
    // 我们的实现将放在这里！
}
```

**清单 17-19**：定义 `timeout` 的签名

这满足了我们对类型的要求。现在我们来考虑一下我们需要的 *行为*：我们希望将传入的未来值和时长竞争。我们可以使用 `trpl::sleep` 转换时长为定时器未来值，并使用 `trpl::select` 与调用者传入的值一起运行该定时器。

在下面清单 17-20 中，我们通过匹配等待 `trpl::select` 的结果实现 `timeout`。

<a name="listing_17-20"></a>
```rust
// -- 跳过代码 --

async fn timeout<F: Future>(
    future_to_try: F,
    max_time: Duration,
) -> Result<F::Output, Duration> {
    match trpl::select(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
    }
}
```

**清单 17-20**：通过 `select` 和 `sleep` 定义 `timeout`

`trpl::select` 的实现是不公平的：他总是按照传递顺序轮询参数（其他 `select` 实现会随机选择先轮询哪个参数）。因此，我们先传递 `future_to_try` 给 `select`，这样即使 `max_time` 是个非常短的时长，他也有机会完成。当 `future_to_try` 先完成时，`select` 将返回带有 `future_to_try` 输出的 `Left`。当定时器先完成时，`select` 将返回带有 `()` 定时器输出的 `Right`。

当 `future_to_try` 成功且我们得到 `Left(output)` 时，我们返回 `Ok(output)`。相反当睡眠定时器超时且我们得到 `Right(())` 时，我们使用 `_` 忽略 `()` 并返回 `Err(max_time)`。

这样，我们就有了个从另外两个异步辅助函数构建的可用 `timeout`。当我们运行代码时，他将打印超时后的故障模式：


```console
在 2 秒后失败
```

由于未来值由其他未来值构成，因此咱们可以使用较小的异步构建块构建相当强大的工具。例如，咱们可以使用同样的方法结合超时与重试，进而对网络调用等操作使用这些工具（如 [清单 17-5](./futures.md#listing_17-5) 所示）。

在实践中，咱们通常将直接使用 `async` 和 `await`，其次使用 `select` 等函数以及 `join!` 等宏，来控制最外层未来值的执行方式。

我们已经看到了同时使用多个未来值的数种方式。接下来，我们将探讨如何在流下，按时间顺序处理多个未来值。


