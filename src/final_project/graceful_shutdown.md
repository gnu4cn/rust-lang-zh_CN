# 优雅关机与清理

[清单 21-20](./multithreaded.md#listing_21-20) 中的代码正如我们预期的那样，通过使用线程池以异步方式响应请求。我们会收到一些关于 `worker`、`id` 和 `thread` 等我们没有直接使用的字段的告警，这提醒我们没有清理任何东西。当我们使用不太优雅的 `Ctrl + c` 方法停止主线程时，所有其他线程也会立即停止，即使他们正处于服务请求状态。

> **译注**：收到的告警如下。
>
> ```console
> $ cargo run
>    Compiling hello v0.1.0 (/home/hector/rust-lang-zh_CN/projects/hello)
> warning: field `workers` is never read
>  --> src/lib.rs:7:5
>   |
> 6 | pub struct ThreadPool {
>   |            ---------- field in this struct
> 7 |     workers: Vec<Worker>,
>   |     ^^^^^^^
>   |
>   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default
>
> warning: fields `id` and `thread` are never read
>   --> src/lib.rs:48:5
>    |
> 47 | struct Worker {
>    |        ------ fields in this struct
> 48 |     id: usize,
>    |     ^^
> 49 |     thread: thread::JoinHandle<()>,
>    |     ^^^^^^
>
> warning: `hello` (lib) generated 2 warnings
>     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.27s
>      Running `target/debug/hello`
> ```

接下来，我们将实现 `Drop` 特质，对线程池中的每个线程调用 `join`，以便他们可以在关闭之前完成完成正在处理的请求。然后，我们将实现一种方法，告知线程停止接受新请求并关闭。为了查看这段代码的实际效果，我们将修改服务器为在有序关闭其线程池之前只接受两个请求。


## 对 `ThreadPool` 实现 `Drop` 特质

我们从对线程池实现 `Drop` 开始。当线程池被弃用时，我们的线程就都应归拢，以确保他们可以完成他们的工作。下面清单 21-22 展示了 `Drop` 实现的首次尝试；这段代码还无法正常工作。

<a name="listing_21-22"></a>
文件名：`projects/hello/src/lib.rs`

```rust
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println! ("关闭 worker {}", worker.id);

            worker.thread.join().unwrap();
        }
    }
}
```

**清单 21-22**：在线程池超出作用域时归拢各个线程

> **译注**：关于线程的 `join` 方法，请参考：
>
> - [Java Thread.join详解](https://zhuanlan.zhihu.com/p/57927767)
>
> - [Joining Threads in Java](https://www.geeksforgeeks.org/joining-threads-in-java/)。

首先，我们遍历线程池 `workers` 中的每个线程。我们为此使用 `&mut` 是因为 `self` 是个可变引用，并且我们还需要能够修改 `worker`。对于每个 `worker`，我们打印一条消息，表明该特定 `Worker` 正在关闭，然后我们对该 `Worker` 实例的线程调用 `join`。当 `join` 调用失败时，我们使用 `unwrap` 使 Rust 终止运行，从而进入非正常关闭状态。

下面是我们编译这代码时得到的报错信息：

```console
$ cargo check
    Checking hello v0.1.0 (/home/hector/rust-lang-zh_CN/projects/hello)
error[E0507]: cannot move out of `worker.thread` which is behind a mutable reference
  --> src/lib.rs:52:13
   |
52 |             worker.thread.join().unwrap();
   |             ^^^^^^^^^^^^^ ------ `worker.thread` moved due to this method call
   |             |
   |             move occurs because `worker.thread` has type `JoinHandle<()>`, which does not implement the `Copy` trait
   |
note: `JoinHandle::<T>::join` takes ownership of the receiver `self`, which moves `worker.thread`
  --> /rustc/01f6ddf7588f42ae2d7eb0a2f21d44e8e96674cf/library/std/src/thread/join_handle.rs:148:17

For more information about this error, try `rustc --explain E0507`.
error: could not compile `hello` (lib) due to 1 previous error
```

这个报错告诉我们，我们不能调用 `join`，因为只有每个 `worker` 的可变借用，而 `join` 会取得其参数的所有权。为了解决这个问题，我们需要从拥有 `thread` 的 `Worker` 实例中迁出线程，以便 `join` 可以消费线程。实现这一目的的一种方法是，采取我们在 [清单 18-15](../oop/implementing.md#listing_18-15) 中使用的同样方法。若 `Worker` 包含一个 `Option<thread::JoinHandle<()>>`，我们就可以对 `Option` 调用 `take` 方法，来从 `Some` 变种中迁出值，并原处留下 `None`。换句话说，正在运行的 `Worker` 将在 `thread` 中包含一个 `Some` 变种，而当我们想要清理一个 `Worker` 时，我们会以 `None` 替换 `Some`，这样那个 `Worker` 就没有要运行的线程了。

在这种情况下，存在更好的替代方案：`Vec::drain` 方法。他接受一个范围参数，来指定要矢量值中要移除的元素，并返回这些元素的迭代器。传递 `..` 范围语法将移除矢量中的所有值。

因此，我们需要像下面这样更新 `ThreadPool` 的 `drop` 实现：


文件名：`projects/hello/src/lib.rs`

```rust
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers.drain(..) {
            println! ("关闭 worker {}", worker.id);

            worker.thread.join().unwrap();
        }
    }
}
```

这解决了编译器报错，且无需对代码的任何其他修改。请注意，由于 `drop` 会在终止运行时被调用，`unwrap` 操作也会引发终止运行，从而导致双重终止运行，这会立即崩溃程序并终止任何正在进行的清理。对于示例程序来说这没有问题，但不建议用于生产代码。

## 通知线程停止监听作业

通过我们已进行的所有修改，我们的代码可以在无任何告警下编译。然而，坏消息是这段代码还没有以我们想要的方式运行。关键在于 `Worker` 实例的线程所运行的闭包中的逻辑：目前，我们调用了 `join` 方法，但这并不会关闭线程，因为他们会永远 `loop` 查找作业。当我们尝试通过我们当前的 `drop` 实现弃用 `ThreadPool` 时，主线程将永远阻塞，等待第一个线程完成。

要解决这个问题，我们需要修改 `ThreadPool` 的 `drop` 实现，然后修改 `Worker` 循环。

首先，我们将修改 `ThreadPool` 的 `drop` 实现，以在等待线程完成之前显式弃用 `sender`。下面清单 21-23 展示了对 ThreadPool 的修改，以显式弃用 `sender`。与线程不同，这里我们 *确实* 需要使用 `Option`，才能通过 `Option::take` 从 `ThreadPool` 中迁出 `sender`。

<a name="listing_21-23"></a>
文件名：`projects/hello/src/lib.rs`

```rust
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}
// -- 跳过代码 --
impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        // -- 跳过代码 --

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in self.workers.drain(..) {
            println! ("关闭 worker {}", worker.id);

            worker.thread.join().unwrap();
        }
    }
}
```

**清单 21-23**：在归拢 `Worker` 线程之前显式弃用 `sender`

弃用 `sender` 会关闭信道，这表明将不再发送消息。发生这种情况时，`Worker` 实例在无限循环中执行的所有 `recv` 调用都将返回错误。在下面清单 21-24 中，我们修改了 `Worker` 的循环，使其在这种情况下能优雅地退出循环，这意味着当 `ThreadPool` 的 `drop` 实现对线程调用 `join` 方法时，他们都将结束运行。


<a name="listing_21-24"></a>
文件名：`projects/hello/src/lib.rs`

```rust
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv();

                match message {
                    Ok(job) => {
                        println!("Worker {id} 得到作业；执行中。");

                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} 已断开；关闭中。");
                        break;
                    }
                }
            }
        });

        Worker { id, thread }
    }
}
```

**清单 21-24**：当 `recv` 返回错误时显式地退出循环

为了查看这段代码的实际效果，我们来修改 `main` 函数为仅接受两个请求，然后便优雅地关闭服务器，如下清单 21-25 中所示。

<a name="listing_21-25"></a>
文件名：`projects/hello/src/main.rs`

```rust
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println! ("关闭。");
}
```

**清单 21-25**：通过退出循环在处理完两个请求后关闭服务器

咱们不会希望真实世界的 web 服务器在仅仅处理两个请求后就关闭。这段代码只是为了演示优雅关闭和清理机制是否正常工作。

`take` 方法定义在 `Iterator` 特质中，限制迭代为最多前两个项目。`ThreadPool` 将在 `main` 函数结束时超出作用域，此时 `drop` 实现将运行。

通过 `cargo run` 启动服务器并发出三个请求。第三个请求应报错，并且在咱们的终端中，咱们应看到类似以下的输出：


```console
$ cargo run
   Compiling hello v0.1.0 (/home/hector/rust-lang-zh_CN/projects/hello)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.28s
     Running `target/debug/hello`
Worker 0 得到作业；执行中。
关闭。
关闭 worker 0
Worker 1 得到作业；执行中。
Worker 2 已断开；关闭中。
Worker 3 已断开；关闭中。
Worker 0 已断开；关闭中。
Worker 1 已断开；关闭中。
关闭 worker 1
关闭 worker 2
关闭 worker 3
```

咱们可能会看到打印 `Worker` ID 和消息的顺序不同。我们可以从消息中看出这段代码的工作原理：`Worker` 实例 `0` 和 `1` 接收了前两个请求。服务器在第二个连接后停止接受连接，而对 `ThreadPool` 的 `Drop` 实现甚至在 `Worker 3` 开始其作业之前就已开始执行。弃用 `sender` 会断开所有 `Worker` 实例的连接并告知他们关闭。每个 `Worker` 实例在断开连接时都会打印一条消息，然后线程池会调用 `join` 方法等待每个 `Worker` 线程完成。

请注意这次特定执行中的一个有趣的方面：`ThreadPool` 弃用 `drop` 后，在任何 `Worker` 收到错误之前，我们尝试聚拢 `Worker 0`。由于 `Worker 0` 尚未从 `recv` 操作中收到错误，因此主线程被阻塞，等待 `Worker 0` 完成。与此同时，`Worker 1` 接收到了作业，然后所有线程都收到了错误。当 `Worker 0` 完成时，主线程等待其余 `Worker` 实例完成。此时，他们都已退出各自的循环并停止了。

恭喜！我们现在已经完成了我们的项目；我们有了一个基本的 web 服务器，他使用线程池来异步地响应。我们能够对服务器执行优雅关闭，从而清理线程池中的所有线程。

以下是完整的代码供参考：

文件名：`projects/hello/src/main.rs`

```rust
{{#include ../../projects/hello/src/main.rs}}
```

文件名：`projects/hello/src/lib.rs`

```rust
{{#include ../../projects/hello/src/lib.rs}}
```

我们可以在这里做得更多！若咱们想要继续增强这个项目，下面是一些建议：

- 为 `ThreadPool` 及其公开方法添加更多文档；
- 添加库功能的测试；
- 修改 `unwrap` 的调用为更健壮的错误处理机制；
- 使用 `ThreadPool` 执行除处理 Web 请求之外的其他任务；
- 在 [crates.io](https://crates.io) 上寻找一个线程池代码箱，并使用该代码箱实现一个类似的 web 服务器。然后，将其 API 和健壮性与我们实现的线程池比较。


# 本章小结

干得漂亮！咱们已经读完了这本书！我们要感谢你加入我们的 Rust 之旅。现在，咱们已经准备好开始实现自己的 Rust 项目，并协助他人完成项目了。请记住，Rust 社区是个充满热情的大家庭，这里的 “Rustaceans”（Rust 社区成员）非常乐意在咱们探索 Rust 的过程中，帮助解决遇到的任何难题。
