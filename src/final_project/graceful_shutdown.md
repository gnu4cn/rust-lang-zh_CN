## 优雅关机与内存清理

**Graceful Shutdown and Cleanup**


清单 20-20 中的代码，经由线程池而如咱们所设想的那样，异步响应请求。咱们会收到有关 `workers`、`id` 及 `thread` 这三个，咱们未以直接方式用到字段的一些告警，这些告警就提醒了咱们，咱们没有清理任何东西。当咱们使用不那么优雅的 `Ctrl + c` 方式，来挂起主线程时，全部其他线程也会被立即停止，即使他们处于服务某个请求中。

接下来，咱们随后将实现 `Drop` 特质，以在线程池中各个线程上调用 `join`，如此这些线程便可以在关闭前，完成他们正工作于其上的请求。随后咱们将实现一种告知线程他们应停止接受新请求并关闭的方法。为观察这方面代码的运作，咱们将把咱们的服务器，修改为在有序关闭其线程池之前，只接受两个请求。


## 实现 `ThreadPool` 上的 `Drop` 特质

**Implementing the `Drop` Trait on `ThreadPool`**


咱们来以在咱们的线程池上实现 `Drop` 开始。当线程池被丢弃时，咱们的那些线程就都应归拢，join 一下，以确保他们完成他们的工作。下面清单 20-22 给出了 `Drop` 实现的首次尝试；此代码尚不会很好地编译。

文件名：`src/lib.rs`

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

*清单 20-22：在线程池超出作用域时归拢各个线程*

> 注：关于线程的 `join` 方法，请参考 [Java Thread.join详解](https://zhuanlan.zhihu.com/p/57927767)，[Joining Threads in Java](https://www.geeksforgeeks.org/joining-threads-in-java/)。

首选，咱们遍历了线程池中 `workers` 的各个线程。由于 `self` 是个可变引用，且咱们还需要能修改 `worker`，因此咱们为这个遍历使用了 `&mut`。对于各个 `worker`，咱们打印了讲到这个特定 `worker` 正要关闭的一条消息，并在随后在那个 `worker` 的线程上调用了 `join`。当到 `join` 的这个调用失败时，咱们便使用 `unwrap` 来令到 Rust 终止运行，并进入到非优雅有序关闭。

下面时在咱们编译这代码时，得到的报错信息：

```console
$ cargo check
    Checking hello v0.1.0 (/home/lenny.peng/rust-lang-zh_CN/hello)
error[E0507]: cannot move out of `worker.thread` which is behind a mutable reference
  --> src/lib.rs:71:13
   |
71 |             worker.thread.join().unwrap();
   |             ^^^^^^^^^^^^^ ------ `worker.thread` moved due to this method call
   |             |
   |             move occurs because `worker.thread` has type `JoinHandle<()>`, which does not implement the `Copy` trait
   |
note: `JoinHandle::<T>::join` takes ownership of the receiver `self`, which moves `worker.thread`
  --> /rustc/2c8cc343237b8f7d5a3c3703e3a87f2eb2c54a74/library/std/src/thread/mod.rs:1589:17

For more information about this error, try `rustc --explain E0507`.
error: could not compile `hello` due to previous error
```

这个报错告诉我们，由于咱们只有各个 `worker` 的可变借用，而 `join` 会取得其参数的所有权，因此咱们无法调用 `join`。为解决这个额外难题，咱们就需要将线程从拥有 `thread` 的 `Worker` 实例迁出，如此 `join` 就可以消费那个线程了。咱们曾在清单 17-15 中这样做过：若 `Worker` 保存的是一个 `Option<thread::JoinHandle<()>>`，那么咱们就可以在 `Option` 上调用 `take` 方法，来将 `Some` 变种中的那个值迁出，并在其位置处留下一个 `None`。也就是说，正运行的一个 `Worker`，将有着 `thread` 中的一个 `Some` 变种，而当咱们打算清理某个 `Worker` 时，咱们就将以 `None` 来替换 `Some`，如此那个 `Worker` 就没有了要运行的线程了。

因此咱们就明白了咱们是要如下更新 `Worker` 的定义：

文件名：`src/lib.rs`

```rust
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
```

现在咱们来依靠编译器，找出其他需要修改的地方。对此代码进行检查，咱们会得到两个报错：

```console
$ cargo check
    Checking hello v0.1.0 (/home/lenny.peng/rust-lang-zh_CN/hello)
error[E0308]: mismatched types
  --> src/lib.rs:62:22
   |
62 |         Worker { id, thread }
   |                      ^^^^^^ expected enum `Option`, found struct `JoinHandle`
   |
   = note: expected enum `Option<JoinHandle<()>>`
            found struct `JoinHandle<_>`
help: try wrapping the expression in `Some`
   |
62 |         Worker { id, thread: Some(thread) }
   |                      +++++++++++++      +

error[E0599]: no method named `join` found for enum `Option` in the current scope
  --> src/lib.rs:71:27
   |
71 |             worker.thread.join().unwrap();
   |                           ^^^^ method not found in `Option<JoinHandle<()>>`
   |
note: the method `join` exists on the type `JoinHandle<()>`
  --> /rustc/2c8cc343237b8f7d5a3c3703e3a87f2eb2c54a74/library/std/src/thread/mod.rs:1589:5
help: consider using `Option::expect` to unwrap the `JoinHandle<()>` value, panicking if the value is an `Option::None`
   |
71 |             worker.thread.expect("REASON").join().unwrap();
   |                          +++++++++++++++++

Some errors have detailed explanations: E0308, E0599.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `hello` due to 2 previous errors
```

咱们来解决那第二个错误，其指向了 `Worker::new` 末尾的代码；在创建新 `Worker` 时，咱们需要把那个 `thread` 值封装在 `Some` 中。请做出如下修改来修复这个错误：

文件名：`src/lib.rs`

```rust
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // --跳过代码--

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
```

第一个错误是在咱们的 `Drop` 实现中，早先咱们曾提到，咱们原本打算调用这个 `Option` 值上的 `take`，来将 `thread` 从 `worker` 中迁出。下面的修改就将这样做：

文件名：`src/lib.rs`

```rust
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println! ("关闭 worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
```

正如曾在第 17 章中讨论过的那样，`Option` 上的 `take` 方法，会将那个 `Some` 变种取出，并在其位置处留下 `None`。咱们使用了 `if let` 来解构那个 `Some` 而得到了那个线程；随后咱们在线程上调用了 `join`。若某个 `worker` 的线程已经是 `None`，那么咱们就知道那个 `worker` 已经让他的线程清理掉了，因此在那种情况下什么也不会发生。


## 通知线程停止收听作业

**Signaling to the Threads to Stop Listening for Jobs**


在咱们已做出的全部修改下，咱们的代码会不带任何错误的编译了。但是，坏消息是这些代码尚不会按照咱们想要的方式运作。问题关键在于，由 `Worker` 实例的线程运行的闭包中的逻辑：此刻，咱们调用了 `join`，但由于线程是在一直 `loop` 查找作业，所以那样做将不会关闭线程。若咱们以咱们当前的 `drop` 实现丢弃咱们的 `ThreadPool`，那么主线程将一直阻塞于等待第一个线程结束。

为修复这个问题，咱们将需要 `ThreadPool` 的 `drop` 实现中的一个修改，以及其后的 `Worker` 循环中的一个修改。

首选，咱们将把 `ThreadPool` 的 `drop` 实现，修改为在等待线程结束前显式地丢弃 `sender`。下面清单 20-23 给出了对 `ThreadPool` 显示丢弃 `sender` 的修改。为能将 `send` 从 `ThreadPool` 迁出，咱们使用了与咱们曾对线程做过的同样 `Option` 于 `take` 技巧：

文件名：`src/lib.rs`

```rust
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}
// --跳过代码--
impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        // --跳过代码--

        ThreadPool {
            workers,
            sender: Some<sender>,
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

        for worker in &mut self.workers {
            println! ("关闭 worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
```

*清单 20-23：在归拢那些 `worker` 线程前显式丢弃 `sender`*

丢弃 `sender` 就会关闭通道，这表明将不会有更多消息发出。当那发生时，在无限循环中那些 `worker` 所做的到 `recv` 的全部全部调用，就会返回错误。在下面清单 20-24 中，咱们修改了 `Worker` 的循环，来在那种情况下优雅有序地退出循环，这就意味着在 `ThreadPool` 的 `drop` 实现在那些线程上调用 `join` 时，他们将结束。


文件名：`src/lib.rs`

```rust
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println! ("Worker {id} 获取到一项作业；执行中。");

                    job();
                }
                Err(_) => {
                    println! ("Worker {id} 已断开链接；关闭中。");
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
```

*清单 20-24：在 `recv` 返回错误时显式跳出循环*

要看到运作中的代码，咱们就来把 `main` 修改为在有序关闭服务器钱，只接收两个请求，如下清单 20-25 中所示。

文件名：`src/main.rs`

```rust
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_conn(stream);
        });
    }

    println! ("关闭中。");
}
```

*清单 20-25： 在服务两个请求后通过退出循环关闭服务器*

咱们是不会想要真实世界的 web 服务器在仅服务两个请求后就关闭的。这段代码只演示了这种有序关闭与清理是在正常工作。

其中的 `take` 方法，是定义在 `Iterator` 特质中的，且将迭代限制到最多头两个项目。在 `main` 的结束处，`ThreadPool` 将超出作用域，而 `drop` 实现将运行。

请以 `cargo run` 启动服务器，并构造三个请求。第三个请求应会出错，而在终端里咱们应看到类似于下面这样的输出：

```console
$ cargo run                                                                   16s
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/hello`
Worker 0 获取到一项作业；执行中。
关闭中。
关闭 worker 0
Worker 1 获取到一项作业；执行中。
Worker 3 已断开链接；关闭中。
Worker 2 已断开链接；关闭中。
Worker 1 已断开链接；关闭中。
Worker 0 已断开链接；关闭中。
关闭 worker 1
关闭 worker 2
关闭 worker 3
```

咱们可能会看到不同顺序的 `worker` 与消息打印出来。咱们能从这些消息，看出代码是如何工作的：`worker` `1` 与 `2` 获取到了头两个请求。服务器在第二个 TCP 连接之后，便停止了接收连接，而 `ThreadPool` 上的 `Drop` 实现，在 `worker` `2` 还没开始其作业前，便开始了执行。丢弃 `sender` 会断开全部 `worker` 并告诉他们要关闭。那些 `worker` 在他们断开连接时，都各自打印了一条消息，而随后线程池便调用了 `join` 来等待各个 `worker` 线程结束。

请注意这次特定执行的一个有趣方面：`ThreadPool` 弃用了 `sender`，而在有任何 `worker` 接收到错误前，咱们就尝试归拢了 `worker` `0`。`worker` `0`  还不曾从 `recv` 获取到一个错误，因此主线程就阻塞于等待 `worker` `0` 结束。与此同时，`worker` `1` 收到了一项作业，而随后全部线程都收到了错误。在 `worker` `0` 结束时，主线程就等待其余 `worker` 结束。而在那个时候，他们都已退出了他们的循环并停止了。

恭喜！咱们先进已经完成了咱们的项目；咱们有了一个运用线程池来异步响应的基本 web 服务器。咱们能够完成服务器有序关闭，这会清理掉线程池中的全部线程。

以下是用于参考的全部代码：

文件名：`src/main.rs`

```rust
use hello::ThreadPool;

use std::{
    fs,
    thread,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    time::Duration,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_conn(stream);
        });
    }

    println! ("关闭中。");
}

fn handle_conn(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let req_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &req_line[..] {
        "GET / HTTP/1.1" => ( "HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(10));
            ("HTTP/1.1 200 0K", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let resp =
        format! ("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(resp.as_bytes()).unwrap();
}
```


文件名：`src/lib.rs`

```rust
use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// 创建出一个新的 ThreadPool。
    ///
    /// 其中的 size 为线程池中线程的数目。
    ///
    /// # 终止运行
    ///
    /// 这个 `new` 函数将在 size 为零时终止运行。
    pub fn new(size: usize) -> ThreadPool {
        assert! (size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

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

        for worker in &mut self.workers {
            println! ("关闭 worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println! ("Worker {id} 获取到一项作业；执行中。");

                    job();
                }
                Err(_) => {
                    println! ("Worker {id} 已断开链接；关闭中。");
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
```

这里咱们可以做更多事情！若咱们打算继续加强这个项目，下面是一些想法：

- 给 `ThreadPool` 及其公开方法添加更多文档;

- 给这个库的功能添加测试;

- 把一些调用修改为 `unwrap`，以获得更多的错误处理鲁棒性;

- 运用 `ThreadPool` 来完成除服务 web 请求外的一些别的任务；

- 在 [crates.io](https://crates.io/) 上找到某个线程池代码箱，并用该代码箱实现一个类似的 web 服务器。随后将其 API 及鲁棒性，与咱们实现的线程池相比较。


# 本章小结

干得好！咱们已经读完了这整本书！要感谢咱们加入到这次 Rust 之旅中来。咱们现在已经准备好实现咱们自己的 Rust 项目，以及帮助其他人的项目了。请记住有那么一个由热衷于就咱们在 Rust 道路上，所遇到的任何挑战，而帮助咱们的其他 Rust 公民的热情社区。
