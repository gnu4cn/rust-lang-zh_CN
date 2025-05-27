# 使用任意数量的未来值


在上一小节中，当我们从使用两个未来值，转换为使用三个未来值时，我们也不得不从使用 `join` 转换为使用 `join3`。如果每次改变我们要连接的未来值数量时，都要调用不同函数，那就太麻烦了。幸运的是，我们有个宏形式的 `join`，使用他我们就可以传递任意数量的参数。他还能自己处理等待未来值。因此，我们可以将清单 17-13 中的代码，重写为使用 `join!` 而非 `join3`，如下清单 17-14 中所示。


文件名：`src/main.rs`


```rust
        trpl::join! (tx1_fut, tx_fut, rx_fut);
```


*清单 17-14：使用 `trpl::join!` 等待多个未来值*


与 `join`、`join3` 和 `join4` 等之间的切换相比，这无疑是一种进步！不过，即使是这种宏形式，也只有在我们提前知道未来值数量的情况下才有效。但在真实世界的 Rust 中，将未来值压入某个集合，然后等待其中的部分或全部未来值完成是种常见的模式。


要检查某个集合中的全部未来值，我们将需要遍历并连接 *全部* 的未来值。`trpl::join_all` 函数接受任何实现了 `Iterator` 特质的类型，我们在第 13 章 [`Iterator` 特质与 `next` 方法](../functional_features/iterators.md#iterator-特质与-next-方法) 小节中，已经了解过迭代器特质，因此他似乎正是我们需要的。我们来尝试将我们的未来值放入一个矢量中，并用 `join_all` 代替 `join！`，如下清单 17-15 所示。


```rust
        let futures = vec! [tx1_fut, tx_fut, rx_fut];

        trpl::join_all(futures).await ;
```

*清单 17-15：将匿名未来值存储在一个矢量值中并调用 `join_all`*


不幸的是，这段代码不会编译。相反，我们会得到如下报错：


```console
error[E0308]: mismatched types
  --> src/main.rs:42:38
   |
8  |         let tx1_fut = async move {
   |                       ---------- the expected `async` block
...
28 |         let tx_fut = async move {
   |                      ---------- the found `async` block
...
42 |         let futures = vec! [tx1_fut, tx_fut, rx_fut];
   |                                      ^^^^^^ expected `async` block, found a different `async` block
   |
   = note: expected `async` block `{async block@src/main.rs:8:23: 8:33}`
              found `async` block `{async block@src/main.rs:28:22: 28:32}`
   = note: no two async blocks, even if identical, have the same type
   = help: consider pinning your async block and casting it to a trait object
```


这可能令人惊讶。毕竟，这个异步代码块没有一个会返回任何的内容，所以每个都会产生一个 `Future<Output = ()>`。但请记住，`Future` 是个特质，编译器会为这每个异步代码块，都创建一个唯一的枚举。咱们不能在一个 `Vec` 中，放入两个不同的手写结构体，同样的规则也适用于由编译器生成的不同枚举。


要令到这一点生效，我们就需要使用 *特质对象*，就像我们在第 12 章的 [“从 `run` 函数返回错误”](../io_project/refactoring.md#返回-run-函数中的错误) 小节中所做的那样。（我们将在第 18 章详细介绍特质对象）。使用特质对象，我们就可以将由这些类型产生的匿名未来值，视为同一类型，因为他们都实现了 `Future` 这个特质。


> **注意**：在第 8 章 [“使用枚举存储多个值”](../common_collections/vectors.md#运用枚举存储多种类型) 小节中，我们曾讨论了在 `Vec` 中包含多种类型的另一种方法：使用枚举来表示矢量值中可能出现的每种类型。但在这里我们不能这样做。首先，我们无法命名这些不同类型，因为他们都是匿名的。另外，我们之所以使用矢量和 `join_all`，首要考虑的时为了能够处理未来值的动态集合，我们只关心他们是否有着同一输出类型。


我们首先将 `vec!` 中的每个未来值，都封装在一个 `Box::new` 中，如下清单 17-16 中所示。

文件名：`src/main.rs`

```rust
        let futures =
            vec![Box::new(tx1_fut), Box::new(rx_fut), Box::new(tx_fut)];

        trpl::join_all(futures).await;
```

<a name="listing-17-16"></a> *清单 17-16：使用 `Box::new` 对齐某个 `Vec` 中的未来值类型*


不幸的是，这段代码仍不会编译。事实上，我们在第二和第三个 `Box::new` 调用处，都遇到了与之前同样的基本报错，同时还出现了指向 `Unpin` 特质的新报错。我们稍后再来看 `Unpin` 的报错。首先，我们来通过显式地注解 `futures` 这个变量的类型，修复 `Box::new` 调用上的类型错误（见清单 17-17）。


文件名：`src/main.rs`

```rust
        let futures: Vec<Box<dyn Future<Output = ()>>> =
            vec![Box::new(tx1_fut), Box::new(rx_fut), Box::new(tx_fut)];
```

<a name="listing-17-17"></a> *清单 17-17：通过显式类型生命，修复类型不匹配报错的其余部分*


这个类型声明有些重要，我们先来了解一下：


1. 最内层的类型就是那个未来值本身。通过写下 `Future<Output = ()>`，我们显式地指出，这个未来值的输出是单元值类型 `()`；
2. 然后，我们以 `dyn` 关键字注解了该特质，将其标记为动态特质；
3. 整个特质引用被封装在一个 `Box` 中；
4. 最后，我们显式地指明 `futures` 是个包含这些项目的 `Vec`。


这已经带来了很大的不同。现在，当我们运行编译器时，我们就只会得到提及 `Unpin` 的那些报错了。虽然有三个报错，但他们的内容非常相似。


```console
error[E0277]: `dyn Future<Output = ()>` cannot be unpinned
   --> src/main.rs:46:24
    |
46  |         trpl::join_all(futures).await;
    |         -------------- ^^^^^^^ the trait `Unpin` is not implemented for `dyn Future<Output = ()>`, which is required by `Box<dyn Future<Output = ()>>: F
uture`
    |         |
    |         required by a bound introduced by this call
    |
    = note: consider using the `pin!` macro
            consider using `Box::pin` if you need to access the pinned value outside of the current scope
    = note: required for `Box<dyn Future<Output = ()>>` to implement `Future`
note: required by a bound in `join_all`
   --> /home/hector/.cargo/registry/src/rsproxy.cn-8f6827c7555bfaf8/futures-util-0.3.31/src/future/join_all.rs:105:14
    |
102 | pub fn join_all<I>(iter: I) -> JoinAll<I::Item>
    |        -------- required by a bound in this function
...
105 |     I::Item: Future,
    |              ^^^^^^ required by this bound in `join_all`

error[E0277]: `dyn Future<Output = ()>` cannot be unpinned
  --> src/main.rs:46:9
   |
46 |         trpl::join_all(futures).await;
   |         ^^^^^^^^^^^^^^^^^^^^^^^ the trait `Unpin` is not implemented for `dyn Future<Output = ()>`, which is required by `Box<dyn Future<Output = ()>>: F
uture`
   |
   = note: consider using the `pin!` macro
           consider using `Box::pin` if you need to access the pinned value outside of the current scope
   = note: required for `Box<dyn Future<Output = ()>>` to implement `Future`
note: required by a bound in `futures_util::future::join_all::JoinAll`
  --> /home/hector/.cargo/registry/src/rsproxy.cn-8f6827c7555bfaf8/futures-util-0.3.31/src/future/join_all.rs:29:8
   |
27 | pub struct JoinAll<F>
   |            ------- required by a bound in this struct
28 | where
29 |     F: Future,
   |        ^^^^^^ required by this bound in `JoinAll`

error[E0277]: `dyn Future<Output = ()>` cannot be unpinned
  --> src/main.rs:46:33
   |
46 |         trpl::join_all(futures).await;
   |                                 ^^^^^ the trait `Unpin` is not implemented for `dyn Future<Output = ()>`, which is required by `Box<dyn Future<Output = (
)>>: Future`
   |
   = note: consider using the `pin!` macro
           consider using `Box::pin` if you need to access the pinned value outside of the current scope
   = note: required for `Box<dyn Future<Output = ()>>` to implement `Future`
note: required by a bound in `futures_util::future::join_all::JoinAll`
  --> /home/hector/.cargo/registry/src/rsproxy.cn-8f6827c7555bfaf8/futures-util-0.3.31/src/future/join_all.rs:29:8
   |
27 | pub struct JoinAll<F>
   |            ------- required by a bound in this struct
28 | where
29 |     F: Future,
   |        ^^^^^^ required by this bound in `JoinAll`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `hello-async` (bin "hello-async" test) due to 3 previous errors
```

这些报错要消化的东西 *太多* 了，我们来将其拆开看看。报错消息的第一部分告诉我们，第一个异步代码块（`src/main.rs:8:23: 20:10`）未实现 `Unpin` 这个，并建议使用 `pin!` 或 `Box::pin` 解决这个问题。本章稍后，我们将深入探讨有关 `Pin` 和 `Unpin` 的更多细节。不过现在，我们可按照编译器的建议解决这个问题。在清单 17-18 中，我们首先从 `std::pin` 导入 `Pin`。接下来，我们更新 `futures` 的类型注解，用 `Pin` 封装每个 `Box`。最后，我们使用 `Box::pin` 固定各个未来值。

文件名：`src/main.rs`

```rust
use std::pin::Pin;

// -- snip --

        let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> =
            vec![Box::pin(tx1_fut), Box::pin(rx_fut), Box::pin(tx_fut)];
```

*清单 17-18：使用 `Pin` 及 `Box::pin` 令到 `Vec` 的类型检查通过*

在我们编译并运行这段代码时，我们最终得到了我们所希望的输出：


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


哈！


这里还有一些要探讨的问题。其一，使用 `Pin<Box<T>>` 会增加因为我们需要将这些未来值，与 Box 一起放在内存堆上而带来的少量开销 -- 而这样做只是为了使类型保持一致（对齐）。毕竟，我们实际上 *不需要* 内存堆分配：这些未来值对于这个特定函数，属于一些局部的未来值。正如前面所提到的，`Pin` 本身就是一个封装类型，因此在无需进行内存堆分配下，我们就能获得在 `Vec` 中保存单一类型的好处 -- 这也是我们使用 `Box` 的初衷。使用 `std::pin::pin` 这个宏，我们可对各个未来值直接使用 `Pin`。


但是，我们仍然必须明确被固定引用的类型；否则，Rust 仍无法将这些引用解释为动态的特质对象，而这正是我们在 `Vec` 中所需要的。因此，我们将 `pin` 添加到 `std::pin` 导入的列表中。然后，在定义各个未来值时对其 `pin！`，并将 `futures` 定义为包含到动态未来值类型的固定可变引用的一个 `Vec`，如下清单 17-19 所示。

文件名：`src/main.rs`

```rust
use std::pin::{Pin, pin};

// -- snip --

        let tx1_fut = pin!(async move {
            // --snip--
        });

        let rx_fut = pin!(async {
            // --snip--
        });

        let tx_fut = pin!(async move {
            // --snip--
        });

        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> =
            vec![tx1_fut, rx_fut, tx_fut];
```


*清单 17-19：以 `pin!` 这个宏直接使用 `Pin`，避免一些不必要的内存堆分配*


我们之所以能做到这一点，是因为我们忽略了可能存在不同 `Output` 类型这一事实。例如，在下面的清单 17-20 中，`a` 的匿名未来值，实现了 `Future<Output = u32>`，`b` 的匿名未来值，则实现了 `Future<Output = &str>`，而 `c` 的匿名未来值，却实现了 `Future<Output = bool>`。


文件名：`src/main.rs`


```rust
        let a = async { 1u32 };
        let b = async { "Hello!" };
        let c = async { true };

        let (a_result, b_result, c_result) = trpl::join! (a, b, c);
        println!("{a_result}, {b_result}, {c_result}");
```

*清单 17-20：有着不同类型的三个未来值*


我们可以使用 `trpl::join!` 等待他们，因为他允许我们传入多个未来值类型，并生成这些类型的元组。我们 *不能* 使用 `trpl::join_all`，因为他要求传入的所有未来值，都要有同一类型。请记住，正是这个报错，让我们开始了 `Pin` 下的冒险之旅！

这是种基本的权衡：我们可以 `join_all` 处理动态数量的未来值，只要他们都具有同一类型；或者使用 `join` 函数或 `join!` 宏处理固定数量的未来值，即使他们有着不同类型。这与我们处理 Rust 种其他类型时，所面临的情况是一样的。未来值并不特殊，即便如此我们也有一些处理他们的良好语法，这是件好事。



## 未来值之间的竞争

**Racing Futures**


当我们以 `join` 系列函数和宏，“连接” 一些未来值时，我们要求 *全部* 未来值都完成后才能继续。但有时，我们只需要集合中的 *某个* 未来值完成，然后就可以继续 -- 这有点类似于一个未来值与另一个未来值赛跑。


在下面的清单 17-21 中，我们再次使用 `trpl::race`，运行 `slow` 与 `fast` 两个未来值。


文件名：`src/main.rs`


```rust
        let slow = async {
            println!("'slow' started.");
            trpl::sleep(Duration::from_millis(100)).await;
            println!("'slow' finished.");
        };

        let fast = async {
            println!("'fast' started.");
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'fast' finished.");
        };

        trpl::race(slow, fast).await;
```

*清单 17-21：使用 `race` 获得率先完成未来值的结果*


在其开始运行时，两个未来值都会打印一条消息，通过调用及等待 `sleep` 暂停一段时间，并在其结束时打印另一条消息。然后，我们将 `slow` 和 `fast` 都传递给 `trpl::race`，并等待其中一个完成。（这里的结果并不奇怪：`fast` 会赢得竞争。）与我们在 [“咱们的首个异步程序”](../async/futures.md#咱们的首个异步程序) 中，曾用到 `race` 时不同，这里我们简单地忽略了其返回的 `Either` 实例，因为所有感兴趣行为都发生在异步代码块的主体中。


请注意，若咱们将 `race` 的参数顺序颠倒一下，则尽管那个 `fast` 未来值总是先完成，“开始” 的信息顺序会改变。这是因为这个特殊的 `race` 函数实现并不公平。他总是按照参数传递的顺序，运行作为参数传入的未来值。别的实现 *是* 公平的，他们将随机选择要先轮询哪个未来值。不管我们使用的竞赛函数实现是否公平，在另一任务开始前，未来值 *之一* 都会运行到该竞赛函数主体中的首个 `await` 时刻。


回顾 [“咱们的首个异步程序”](../async/futures.md#咱们的首个异步程序)，在每个等待点处，Rust 都会给到运行时一个暂停任务的机会，并在正等待的未来值尚未准备好时，就会切换到另一任务。反之亦然： Rust *只会* 暂停异步代码块，并在等待点处将控制权交还给运行时。等待点之间的一切，都是同步的。


这意味着，若咱们在某个没有等待点的异步代码块中，执行大量工作，那么这个未来值将阻塞全部别的未来值取得进展。有时咱们可能会听到这样的说法：一个未来值会让其他未来值 *饿死*。在某些情况下，这可能不是什么大问题。但是，若咱们正在进行某种昂贵的设置，或执行长期运行的工作，或者在咱们有个将无限期地执行某项特定任务的未来值时，咱们就需要考虑，何时何处将控制权交还给运行时。


以同样说法，若咱们有些长期运行的阻塞操作，那么异步就会是种为程序的不同部分，提供相互关联方法的有用工具。


但在这种情况下，咱们要 *如何* 将控制权，交还给运行时呢？


## 将控制权交给运行时

**Yielding Control to the Runtime**


我们来模拟一项长时间运行的操作。下面清单 17-22 引入了一个 `slow` 函数。

文件名：`src/main.rs`


```rust
fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}
```


*清单 17-22：使用 `thread::sleep` 模拟慢速操作*


这段代码使用了 `std::thread::sleep` 而非 `trpl::sleep`，因此调用 `slow` 会阻塞当前线程若干毫秒。我们可以使用 `slow` 代替现实世界中，那些既要长时间运行又会阻塞的操作。


在下面的清单 17-23 中，我们使用了 `slow`，模拟两个未来值中的这种 CPU 密集的工作。

文件名：`src/main.rs`

```rust
        let a = async {
            println!("'a' started.");
            slow("a", 30);
            slow("a", 10);
            slow("a", 20);
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75);
            slow("b", 10);
            slow("b", 15);
            slow("b", 350);
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'b' finished.");
        };

        trpl::race(a, b).await;
```

*清单 17-23：使用 `thread:sleep` 模拟慢速操作*


首先，两个未来值都只会在执行了一系列慢速操作 *后*，才会将控制权交还给运行时。若咱们运行这段代码，咱们会看到这样的输出：


```console
'a' started.
'a' ran for 30ms
'a' ran for 10ms
'a' ran for 20ms
'b' started.
'b' ran for 75ms
'b' ran for 10ms
'b' ran for 15ms
'b' ran for 350ms
'a' finished.
```


正如咱们前面的示例那样，`race` 仍会在 `a` 完成后立即结束。不过，两个未来值间没有交错。在 `trpl::sleep` 调用被等待前，`a` 这个未来值会完成他的所有工作，然后在 `b` 自己的 `trpl::sleep` 调用被等待前，`b` 会完成他的所有工作，最后 `a` 这个未来值完成。为允许两个未来值在他们的慢速任务间都取得进展，我们就需要一些等待点，以便咱们将控制权交还给运行时。这意味着我们需要一些咱们可以等待的东西！


在清单 17-23 中，我们已经可以看到这种切换：若我们去掉 `a` 未来值结尾处的 `trpl::sleep`，他就会在 `b` 未来值 *完全* 未运行的情况下完成。我们来试着以 `sleep` 函数为起点，让两个这些操作在取得进展间切换，如下清单 17-24 中所示。


文件名：`src/main.rs`


```rust
        let one_ms = Duration::from_millis(1);

        let a = async {
            println!("'a' started.");
            slow("a", 30);
            trpl::sleep(one_ms).await;
            slow("a", 10);
            trpl::sleep(one_ms).await;
            slow("a", 20);
            trpl::sleep(one_ms).await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75);
            trpl::sleep(one_ms).await;
            slow("b", 10);
            trpl::sleep(one_ms).await;
            slow("b", 15);
            trpl::sleep(one_ms).await;
            slow("b", 350);
            trpl::sleep(one_ms).await;
            println!("'b' finished.");
        };
```


*清单 17-24：使用 `sleep` 让操作在取得进展间切换*


在清单 17-24 中，我们在每次调用 `slow` 间，以等待点添加了一些 `trpl::sleep` 调用。现在，两个未来值的工作是交替的了：


```console
'a' started.
'a' ran for 30ms
'b' started.
'b' ran for 75ms
'a' ran for 10ms
'b' ran for 10ms
'a' ran for 20ms
'b' ran for 15ms
'a' finished.
```

在将控制权移交给 `b` 前，`a` 未来值仍会运行一段时间，因为他在调用 `trpl::sleep` 前调用了 `slow`，但在此之后，每当其中一个未来值到达等待点时，他们就会来回交换。在这个示例中，我们在每次调用 `slow` 后都这样做了，但我们也可以任何对咱们最合理的方式，分解两个未来值的工作。


不过，我们并不真的打算在这里 *睡眠*：我们要的是尽快取得进展。我们只需将控制权交还给运行时。使用 `yield_now` 函数，我们就可以直接做到这点。在下面的清单 17-25 中，我们就以 `yield_now`，取代了所有的 `sleep` 调用。


文件名：`src/main.rs`


```rust
        let a = async {
            println!("'a' started.");
            slow("a", 30);
            trpl::yield_now().await;
            slow("a", 10);
            trpl::yield_now().await;
            slow("a", 20);
            trpl::yield_now().await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75);
            trpl::yield_now().await;
            slow("b", 10);
            trpl::yield_now().await;
            slow("b", 15);
            trpl::yield_now().await;
            slow("b", 350);
            trpl::yield_now().await;
            println!("'b' finished.");
        };
```


*清单 17-25：使用 `yield_now` 让操作在取得进展间切换*


这段代码既能更清楚地表达实际意图，又能比使用 `sleep` 快很多，因为 `sleep` 用到的这种定时器，通常有着他们所能及的粒度限制。例如，我们正使用的 `sleep` 版本，将总是会至少睡眠一毫秒，即便我们给他的 `Duration` 是一纳秒。同样，现代计算机的速度 *很快*：他们可在一毫秒中完成很多事情！


通过设置一个小的基准测试，诸如清单 17-26 中的那个，咱们就可以亲自发现这点。(这并不是一种特别严格的性能测试方法，但其足以在此显示差异。）


文件名：`src/main.rs`


```rust
        let one_ns = Duration::from_nanos(1);
        let start = Instant::now();
        async {
            for _ in 1..1000 {
                trpl::sleep(one_ns).await;
            }
        }
        .await;
        let time = Instant::now() - start;
        println!(
            "'sleep' version finished after {} seconds.",
            time.as_secs_f32()
        );

        let start = Instant::now();
        async {
            for _ in 1..1000 {
                trpl::yield_now().await;
            }
        }
        .await;
        let time = Instant::now() - start;
        println!(
            "'yield' version finished after {} seconds.",
            time.as_secs_f32()
        );
```

*清单 17-26：比较 `sleep` 与 `yield_now` 的性能*


这里，我们跳过了所有状态打印，将一个一纳秒的 `Duration` 传递给 `trpl::sleep`，并让每个未来值自行运行，两个未来值间没有切换。然后我们运行 `1000` 次迭代，看看使用 `trpl::sleep` 的未来值，与使用 `trpl::yield_now` 的未来值相比时间长了多少。


`yield_now` 的版本，是更快的 *方式*！


```console
'sleep' version finished after 1.0830415 seconds.
'yield' version finished after 0.000244098 seconds.
```


这意味着异步对那些计算密集任务也是有用的，这取决于咱们的程序在做什么，因为异步为构建程序不同部分之间的关系，提供了有用工具。这是一种 *合作式多任务处理*，其中每个未来值都有权决定，何时通过等待点移交控制权。因此，每个未来值也有责任避免阻塞过长时间。在一些基于 Rust 的嵌入式操作系统中，这是 *唯一* 的多任务处理方式！


当然，在实际代码中，你通常不会在每行代码上，以等待点在函数调用间交替。虽然以这种方式让渡控制权的代价相对较低，但并不免费。在很多情况下，试图中断某个计算密集的任务，可能令其明显变慢，因此有时让某项操作短暂阻塞，会更有利于 *整体* 性能。一定要测量咱们代码的具体性能瓶颈。不过，在咱们发现有很多工作是以串行方式进行的，而咱们预期的是以并发方式进行时，那么就必须注意底层动态了！


## 构建咱们自己的异步抽象


我们还可以将未来值组合在一起，创建处新的模式。例如，以咱们已有的异步构件，咱们可构建出一个 `timeout` 函数。当我们完成后，结果将是另一个我们可以用于创建出更多异步抽象的构建块（译注：异步构件）。


清单 17-27 显示了，这个 `timeout` 与某个慢速未来值一起使用时，咱们所期望的其工作方式。

文件名：`src/main.rs`


```rust
        let slow = async {
            trpl::sleep(Duration::from_millis(100)).await;
            "I finished!"
        };

        match timeout(slow, Duration::from_millis(10)).await {
            Ok(message) => println!("Succeeded with '{message}'"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs())
            }
        }
```


*清单 17-27：使用咱们设想的 `timeout`，在有限时间下运行某个慢速操作*


我们来实现这个异步构件！首先，我们来考虑一下 `timeout` 的 API：


- 他本身需要是个异步函数，这样我们才能等待他；
- 他的第一个参数应是个要运行的未来值。我们可将其构造为可与任何未来值一起使用的通用函数；
- 其第二个参数将是要等待的最长时间。若我们使用一个 `Duration`，就将很容易将其传递给 `trpl::sleep`；
- 他应返回一个 `Result`。在未来值成功完成时，这个 `Result` 将为带有该未来值所产生值的 `Ok`。在超时在先时，`Result` 将是带有超时等待时长的 `Err`。


下面清单 17-28 展示了这一声明。


文件名：`src/main.rs`


```rust
async fn timeout<F: Future> (
    future_to_try: F,
    max_time: Duration,
) -> Result<F::Output, Duration> {
    // Here is where our implementation will go!
}
```


*清单 17-28：定义出 `timeout` 的签名*


这就满足了我们的类型目标。现在，我们来考虑一下我们需要的 *行为*：我们是要让传入的未来值，与传入的持续时间赛跑。我们可使用 `trpl::sleep`，从传入的持续时间构造出一个定时器的未来值，并使用 `trpl::race` 与调用者所传入的未来值一起，运行这个定时器。


我们还指导这个 `race` 是不公平的，他会按照参数传递的顺序轮询参数。因此，我们先将 `future_to_try` 传递给 `race`，这样即使 `max_time` 很短，他也有机会完成。在 `future_to_try` 首先完成时，`race` 将返回 `Left` 与 `future_to_try` 的输出。在 `timer` 首先完成时，`race` 将返回 `Right` 与定时器的输出 `()`。


在下面的清单 17-29 中，我们匹配了等待 `trpl::race` 的结果。


文件名：`src/main.rs`


```rust
use trpl::Either;

// --snip--

fn main() {
    trpl::run(async {
        let slow = async {
            trpl::sleep(Duration::from_secs(5)).await;
            "Finally finished"
        };

        match timeout(slow, Duration::from_secs(2)).await {
            Ok(message) => println!("Succeeded with '{message}'"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs())
            }
        }
    });
}

async fn timeout<F: Future>(
    future_to_try: F,
    max_time: Duration,
) -> Result<F::Output, Duration> {
    match trpl::race(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
    }
```

*清单 17-29：使用 `race` 与 `sleep` 定义 `timeout`*


在 `future_to_try` 成功时，我们会得到 `Left(output)`，我们就会返回 `Ok(output)`。若那个睡眠定时器超时，我们就得到 `Right(())`，我们以 `_` 忽略那个 `()` 并返回 `Err(max_time)`。


这样，我们就从另外两个异步助手函数 `trpl::sleep` 与 `trpl::race`，创建出了个可工作的 `timeout`。在我们运行咱们的代码时，他会在超时后打印那个失败模式：


```console
Failed after 2 seconds
```


由于未来值可与其他未来值组合，因此咱们可以使用一些较小的异步构件，构建出真正强大的工具。例如，咱们可使用同样的方法，将超时与重试结合起来，进而将其用于诸如网络调用等的操作（本章开头的示例之一）。


在实践中，咱们通常会直接使用 `async` 与 `await`，其次是诸如 `join`、`join_all`、`race` 等函数与宏。现在咱们将只需偶尔用到 `pin`，就能在这些 API 中运用未来值。


现在，我们已经看到了同时处理多个未来值的数种方法。接下来，我们将了解如何使用 *流*，按时间顺序处理多个未来值。不过，咱们可能需要先考虑几件事：


- 我们使用了与 `join_all` 一起的一个 `Vec`，等待某个组中的所有未来值完成。那么咱们要如何使用一个 `Vec`，依次处理一组未来值呢？这样做有什么好处？
- 请查看 `futures` 代码箱中的 `futures::stream::FuturesUnordered` 类型。使用他与使用一个 `Vec` 有何不同？ （不用担心该类型是来自于这个代码箱板中的 `stream` 部分；他对任何的未来值集合都能正常工作。）


（End）


