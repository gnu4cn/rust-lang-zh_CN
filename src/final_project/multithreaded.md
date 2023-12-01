# 将咱们的单线程服务器改写为多线程服务器

**Turning Our Single-Thread Server into a Multithreaded Server**


现在，这个服务器将依次处理每个请求，这意味着其将不会在前一个连接完成处理前，处理后一连接。若服务器收到了越来越多的请求，这种顺序执行就会越来越差。而若该服务器收到了一个要耗费较长时间处理的请求，即使后续的新请求可被快速处理，但其仍将不得不等待直到那个长时间请求完成。咱们需要修复这个问题，但首选，咱们将具体看看这个问题。


## 在当前服务器实现下模拟一个慢速请求

**Simulating a Slow Request in the Current Server Implemenation**


咱们将看看一个慢速处理的请求，能怎样影响那些到咱们当前服务器实现的其他请求。下面清单 20-10 以一个将导致服务器在响应前睡眠 5 秒的模拟慢速请求，实现了对到 `/sleep` 请求的处理。

文件名：`src/main.rs`

```rust
#![allow(warnings)]
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thred,
    time::Duration,
};
// --跳过代码--

fn handle_conn(mut stream: TcpStream) {
    // --跳过代码--

    let (status_line, filename) = match &req_line[..] {
        "GET / HTTP/1.1" => ( "HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 0K", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    // --跳过代码--
}
```

*清单 20-10：通过睡眠 5 秒模拟慢速请求*

现在咱们有了三种情况，于是就已从 `if` 切换到了 `match`。咱们需要显式地在 `req_line` 切片上，与那三个字符串字面值进行模式匹配；`match` 不会像相等比较方式所做的那样，执行自动引用与解引用。

首条支臂与清单 20-9 的 `if` 代码块是一样的。第二条支臂，是将请求与 `/sleep` 匹配。在收到那个请求时，服务器将在渲染那个成功 HTML 页面之前，睡眠 5 秒。第三支臂则与清单 20-9 的那个 `else` 代码块是一样的。

咱们可以看出，咱们的服务器有多原始：真正的库将以一种不那么冗长的方式，处理多种请求的识别！

请使用 `cargo run` 启动服务器。随后打开两个浏览器窗口：一个用于 `http://127.0.0.1/7878`，另一个用于 `http://127.0.0.1:7878/sleep`。若咱们像之前一样进入那个 `/` URI 几次，咱们将看到其响应很快。但在进入 `/sleep` 并于随后加载 `/` 时，就会看到那个 `/` 会一直等待，知道 `sleep` 已经于加载之前睡眠了 5 秒。

咱们可以用来避免慢速请求后面那些请求滞后的技巧有多种；咱们将实现的技巧，便是线程池。


## 使用线程池提升吞吐量

**Improving Throughput with a Thread Pool**


所谓 *线程池，thread pool*，是指处于等待中，并准备好处理某项任务的一组生成的线程。在程序收到一项新任务时，他便指派线程池中的一个线程给该项任务，而那个线程就会处理这个任务。池中的剩余线程，则是可以处理任何的于这首个线程进行处理时，进来的那些任务的。在这首个线程完成其任务处理时，他就会回到空闲线程的线程池，准备处理某项新任务。线程池实现了连接的并发处理，从而提升咱们服务器的吞吐能力。

咱们将把池中线程数量，先知道一个较小的数目，以保护咱们免于拒绝服务攻击，Denial of Service(DoS) attacks；若咱们让咱们的程序在每个请求进入时，创建一个新线程，那么构造出一千万个请求到咱们的服务器的某人，就能经由耗尽咱们服务器的全部资源，而使得这些请求的处理陷入停滞，而造成极大破坏。

这种技巧只是提供 web 服务器吞吐量的许多方法之一。咱们可能探讨的其他选项分别是 *分叉汇合模型，fork/join model*、*单线程异步 I/O 模型，single-threaded async I/O model*，抑或 *多线程异步 I/O 模型，multi-threaded async I/O model*。若对此问题感兴趣，那么可以阅读有关其他解决方案的资料，并尝试实现他们；对于 Rust 这种底层编程语言，所有这些选项都是可行的。


在开始实现线程池前，咱们来聊聊用到这个池子的东西会是什么样子。在咱们正要尝试设计代码时，首先编写客户端界面，可有助于引导咱们的设计。要以咱们打算调用代码 API 的方式，编写出这些有组织架构的代码 API；随后在那种组织架构下实现功能，而非先实现功能而随后设计那些公开 API。

与第 12 章中项目里用到的测试驱动方式的开发，test-driven development，类似，这里咱们将运用编译器驱动的开发，compiler-driven development。咱们将先编写出咱们打算调用那些函数的代码，而随后会看看来自编译器的那些报错，以确定出接下来咱们应修改些什么，来让代码运作起来。在咱们进行那一步之前，咱们将探讨一下咱们并不会用到的一种技巧，作为开头。


### 为每个请求生成一个线程

**Spawning a Thread for Each Request**


首先，咱们来探讨一下若咱们的代码给每隔连接创建一个新线程，他看起来会怎样。正如早先所提到的，由于潜在地生成无限数目线程的那些问题，这样做不是咱们的最终计划，但其为首先得到一个运作多线程服务器的起点。随后咱们将添加线程池作为一项改进，且将这两种方案进行对比将更容易一些。下面清单 20-11 给出了把 `main` 构造为于那个 `for` 循环里，生成一个新线程来处理每个 TCP 流的一些修改。

文件名：`src/main.rs`

```rust
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_conn(stream);
        });
    }
}
```

*清单 20-11：为每个 TCP 流生成一个新线程*

如同咱们在第 16 章中所学到的，`thread::spawn` 讲创建出一个新线程，并于随后在新线程中，运行那个闭包中的代码。当咱们运行此代码，并在浏览器中加载 `/sleep`，随后在另外两个浏览器 Tab 页中加载 `/`，咱们就会看到到 `/` 的请求就不必等待 `/sleep` 请求完毕了。不过，如同咱们曾提到过的，因为咱们正不带任何限制地构造新线程，而最终将使系统不堪重负。


### 创建有限数目的线程

**Creating a Finite Number of Threads**


咱们想要咱们的线程池，以类似的、熟悉的方式运作，而无需那些用到咱们 API 的代码有较大修改。下面清单 20-12 给出了咱们打算用到的 `ThreadPool`，而非 `thread::spawn`，的假想接口。

文件名：`src/main.rs`

```rust
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_conn(stream);
        });
    }
}
```

*清单 20-12：咱们设想的 `ThreadPool` 接口*

咱们使用了 `ThreadPool::new` 来创建出有着可配置线程数目的新线程，在此示例中为四个线程。随后，在那个 `for` 循环中，`pool.execute` 有着与 `thread::spawn` 类似的接口，其中他会取个闭包，并将其给到线程池中的某个线程运行。这段代码尚不会编译，但咱们将进行尝试，如此编译器就会引导咱们如何修复他。


### 运用编译器驱动的开发，构建出 `ThreadPool`

**Building `ThreadPool` Using Compiler Driven Development**


请完成清单 20-12 中对 `src/main.rs` 的修改，然后咱们就来运用 `cargo check` 给出的编译器报错，驱动咱们的开发。下面就是咱们所得到的第一个报错：

```console
$ cargo check
    Checking hello v0.1.0 (/home/lenny.peng/rust-lang-zh_CN/hello)
error[E0433]: failed to resolve: use of undeclared type `ThreadPool`
  --> src/main.rs:12:16
   |
12 |     let pool = ThreadPool::new(4);
   |                ^^^^^^^^^^ use of undeclared type `ThreadPool`

For more information about this error, try `rustc --explain E0433`.
error: could not compile `hello` due to previous error
```

很棒！这个错误告诉我们，咱们需要一个 `ThreadPool` 类型或模组，因此咱们现在就将构建一个出来。咱们的 `ThreadPool` 实现，将独立于咱们的 web 服务器所完成工作的类型。因此，咱们就来将这个 `hello` 代码箱，从二进制代码箱切换为一个库代码箱，来保存咱们的 `ThreadPool` 实现。在咱们改变为库代码箱后，咱们就可以在打算用到线程池的任何项目，而不只是用来服务 web 请求中，也可以使用这个独立的线程池了。

请创建一个包含了下面这个咱们目前所能有的 `ThreadPool` 结构体极简定义的 `src/lib.rs` 文件：

文件名：`src/lib.rs`

```rust
pub struct ThreadPool;
```

随后编辑 `main.rs`，来通过加入下面的代码到 `src/main.rs` 顶部，将 `ThreadPool` 从那个库代码箱，带入作用域：

文件名：`src/main.rs`

```rust
use hello::ThreadPool;
```

这段代码仍不会工作，但咱们就来再检查一边，以得到咱们需要解决的下一报错：

```console
$ cargo check
    Checking hello v0.1.0 (/home/lenny.peng/rust-lang-zh_CN/hello)
error[E0599]: no function or associated item named `new` found for struct `ThreadPool` in the current scope
  --> src/main.rs:14:28
   |
14 |     let pool = ThreadPool::new(4);
   |                            ^^^ function or associated item not found in `ThreadPool`

For more information about this error, try `rustc --explain E0599`.
error: could not compile `hello` due to previous error
```

此报错表明，接下来咱们就要给 `ThreadPool` 创建一个名为 `new` 的关联函数。咱们还知道了那个 `new` 需要有一个可将 `4` 作为实参接收的形参，并应返回一个 `ThreadPool` 的实例。下面就来实现将有着那些特性的这个极简 `new` 函数：

文件名：`src/lib.rs`

```rust
pub struct ThreadPool;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        ThreadPool
    }
}
```

由于咱们清楚一个负的线程数目不会有任何意义，因此咱们选择了 `usize` 作为那个 `size` 参数的类型。咱们还知道咱们将使用这个 `4` 作为线程集合中原始的个数，那即使这个 `usize` 类型的目的所在，正如第三章的 [整数类型](Ch03_Common_Programming_Concepts.md#整形) 小节中曾讨论过的。

下面来再次检查：

```console
$ cargo check
    Checking hello v0.1.0 (/home/lenny.peng/rust-lang-zh_CN/hello)
error[E0599]: no method named `execute` found for struct `ThreadPool` in the current scope
  --> src/main.rs:19:14
   |
19 |         pool.execute(|| {
   |              ^^^^^^^ method not found in `ThreadPool`

For more information about this error, try `rustc --explain E0599`.
error: could not compile `hello` due to previous error
```

现在的报错之所以出现，是因为在 `ThreadPool` 上咱们没有一个 `execute` 方法。回顾 ["创建有限数目的线程"](#创建有限数目的线程) 小节到，咱们已决定咱们的线程池，应有一个类似与 `thread::spawn` 的接口。此外，咱们将实现这个 `execute` 函数，如此其便会取那个给到他的闭包，并将其交给线程池中的某个空闲进程运行。

咱们将在 `ThreadPool` 上定义这个 `execute` 方法，来取一个闭包作为参数。回顾第 13 章中 [“将捕获值迁移出闭包与 `Fn` 特质”](Ch13_Functional_Language_Features_Iterators_and_Closures.md#将捕获到的值迁移出闭包与-fn-特质) 到咱们可以三种不同特质，将闭包取作参数：`Fn`、`FnMut` 与 `FnOnce`。咱们需要确定出这里要使用何种类别的闭包。咱们清楚咱们将以完成一些类似于标准库的 `thread::spawn` 实现类似的东西结束，因此咱们就可以看看 `thread::spawn` 的签名在其参数上有些什么。文档给出咱们下面的东西：

```rust
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
```

其中的 `F` 类型参数，就是咱们在这里所关心的那个；那个 `T` 类型参数于返回值相关，而咱们并不关心那个。咱们可以看出，`spawn` 使用 `FnOnce` 作为 `F` 上的特质边界。由于咱们将最终将把咱们在 `execute` 中获得的实参，传递给 `spawn`，因此这或许也正是咱们想要的。由于为运行某个请求的线程，将只执行那个请求的闭包一次，而这是与 `FnOnce` 中的 `Once` 是相匹配的，故咱们可以进一步确信，`FnOnce` 便是咱们要用到的特质。

其中的 `F` 类型参数，还有着特质边界 `Send` 与生命周期边界 `'static`，在咱们这种情况下他们是有用的：咱们需要 `Send` 来将闭包，从一个线程转移到另一线程，并由于咱们不知道那个线程将耗时多久来执行，因此而需要 `'static`。下面咱们就来在 `ThreadPool` 上，创建出将取到有着这些边界的，类型 `F` 的泛型参数的 `execute` 方法：

文件名：`src/lib.rs`

```rust
#![allow(warnings)]
pub struct ThreadPool;

impl ThreadPool {
    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
    {
    }
}
```

由于这个 `FnOnce` 表示一个不会取参数，且返回的是单元类型 `()` 的闭包，因此咱们仍旧使用了 `FnOnce` 后的 `()`。就跟函数定义一样，返回值类型可以在签名中省略，但即使咱们没有参数，咱们仍需这对括号。

又一次，这仍是那个 `execute` 方法的极简实现：他什么也没做，但咱们只是在试着让咱们的代码编译。咱们再来对其加以检查：

```console
$ cargo check
    Checking hello v0.1.0 (/home/lenny.peng/rust-lang-zh_CN/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 0.36s
```

他编译了！不过请注意，当咱们尝试 `cargo run` 并在浏览器中构造一次请求时，咱们将看到浏览器中，一些咱们在本章开头曾看到过的报错。咱们的库尚未真正调用传递给 `execute` 的那个闭包！

> 注意：咱们或许听说过与有着严格编译器语言，比如 Haskell 与 Rust，有关的一种说法，即 “若代码编译了，他就会工作。” 然而这种说法并非一概而论。咱们的项目编译了，但他绝对什么也没干！若咱们是在构建一个真实、完整的项目，那么此时就将是开始编写检查代码编译与否，*以及* 是否具有咱们想要的行为的单元测试的好时机。


### 在 `new` 中验证线程数目

**Validating the Number of Threads in `new`**


咱们没有对 `new` 与 `execute` 的参数做任何事情。下面就来以咱们打算的行为，实现这两个函数的函数体。咱们来构思一下 `new`，作为开始。早先由于负的线程数目没有意义，因此咱们给那个 `size` 参数，选择了一个无符号整数类型。不过尽管零也是相当有效的 `usize`，但零个线程的线程池，同样是无意义的。咱们将在返回一个 `ThreadPool` 实例前，添加检查 `size` 大于零的代码，并在程序收到一个零时，通过使用 `assert!` 宏，让程序终止运行，如下面清单 20-13 中所示。

文件名：`src/lib.rs`

```rust
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

        ThreadPool
    }

    // --跳过代码--
}
```

*清单 20-13：将 `ThreadPool` 实现为在 `size` 为零时终止运行*

咱们还以一些文档注释，doc comments，给咱们的 `ThreadPool` 结构体添加了一些文档。请注意咱们通过添加如同第 14 章中曾讨论过的，一个会呼出咱们的函数可能终止运行时的那些情形的小节，而遵循了良好的文档实践。请尝试运行 `cargo doc --open` 并点击那个 `ThreadPool` 结构体，来看到为 `new` 生成的文档看起来是怎样的！

与其如咱们在这里所做的添加这个 `assert!` 宏，咱们则可把 `new` 改为 `build`，并像咱们曾在清单 12-9 中那个 I/O 项目里的 `Config::build` 下所做的那样，返回一个 `Result`。但咱们已经决定，在此示例中是在尝试创建一个，其中全部线程都不应是不可恢复错误的线程池。若你觉得信心满满，那就请编写一个名为 `build`，有着下面签名的函数，来与这个 `new` 函数相比较：

```rust
pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
```


### 创建空间来存储这些线程

**Creating Space to Store the Threads**


既然咱们有了获悉咱们有着要在线程池中存储线程有效数目的一种办法了，咱们便可以创建出这些线程，并在返回这个 `ThreadPool` 结构体前，将他们存储在该结构体中。但是咱们要怎么 “存储” 一个线程呢？下面又来看看那个 `thread::spawn` 签名：

```rust
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
```

`spawn` 函数返回了一个 `JoinHandle<T>`，其中的 `T` 为闭包所返回的类型。咱们也来尝试使用 `JoinHandle`，并观察会发生什么。在咱们的用例中，咱们传递给线程池的闭包，将处理 TCP 连接，而不会返回任何东西，因此其中的 `T` 将是单元类型 `()`。

下面清单 20-14 中的代码将会编译，但尚不会创建任何线程。咱们已将 `ThreadPool` 的定义，修改为保存了一个 `thread::JoinHandle<()>` 实例的矢量值，以 `size` 大小初始化了这个矢量值，还建立了一个将运行某些代码来创建出那些线程的 `for` 循环，并返回了一个包含着这些线程的 `ThreadPool` 实例。

文件名：`src/lib.rs`

```rust
use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    // --跳过代码--
    pub fn new(size: usize) -> ThreadPool {
        assert! (size > 0);

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            // 创建出一些线程并将他们存储在那个矢量中
        }

        ThreadPool { threads }
    }
    // --跳过代码--
}
```

*清单 20-14：为保存那些线程而给 `ThreadPool` 创建一个矢量值*

由于咱们正使用 `thread::JoinHandle` 作为 `ThreadPool` 中那个矢量值条目的类型，因此咱们已将 `std::thread` 带入到这个库代码箱的作用域。

一旦收到有效的大小，咱们的 `ThreadPool` 就会创建出可保存 `size` 个条目的一个新矢量值。那个 `with_capacity` 函数，执行的是与 `Vec::new` 同意的任务，但有个重要的不同之处：他会预先分配那个矢量值中的空间。由于咱们清楚咱们需要在那个矢量值中存储 `size` 个元素，那么预先完成这种分配，相比使用在有元素插入时调整自身的 `Vec::new`，就会稍微更具效率。

在再度运行 `cargo check` 时，其应成功。


### 负责将代码从 `ThreadPool` 发送给某个线程的 `Worker` 结构体

**A `Worker` Struct Responsible for Sending Code from the `ThreadPoll` to a Thread**


在清单 20-14 中的那个 `for` 循环里，咱们留了一个有关线程创建过程的注释。这里，咱们将看看咱们具体要怎么创建出那些线程来。标准库提供了 `thread::spawn` 作为创建线程的一种方式，而 `thread::spawn` 则期望得到一些线程在其一创建出来，就应立即运行的代码。然而，在咱们的示例中，咱们打算创建出这些线程，并让他们 *等待，wait* 咱们稍后将要发送的那些代码。标准库的线程实现，没有包含任何实现那样做法的方式；咱们必须亲自实现他。

咱们将通过引入介于 `ThreadPool` 与那些线程之间，将对这种新行为加以管理的一种新数据结构，来实现这样的行为。咱们将把这种数据结构称作 `Worker`，在线程池实现中，这是个常见的术语。`Worker` 会拾取需要运行的代码，并在该 `Worker` 的线程中运行那些代码。设想某家饭馆中工作的人们：工人们会一直等待，直到有顾客点的菜单进来，而随后他们就负责接下这些菜单，并让顾客们满意。

在线程池中存储的，不再是 `JoinHandle<()>` 实例的矢量值，咱们将存储这个 `Worker` 结构体的实例。每个 `Worker` 都将存储一个单独的 `JoinHandler<()>` 实例。随后咱们将在 `Worker` 上实现一个，将取得要运行代码的闭包，并将其发送到已经运行着的线程去执行的方法。咱们还将给到每个 `Worker` 一个 `id`，如此咱们就可以在日志记录或调试时，区分出线程池中那些不同的 `Worker`。


以下便是在咱们创建一个 `ThreadPool` 时，将发生的一个新过程。咱们将在以此方式建立起 `Worker` 结构体后，再实现把闭包发送给线程的那些代码：

1. 定义出一个保存了一个 `id` 与一个 `JoinHandler<()>` 的 `Worker` 结构体；
2. 把 `ThreadPool` 修改为保存一个 `Worker` 实例构成的矢量值；
3. 定义出会取一个 `id` 数字，并返回保存着这个 `id`，以及带有所生成的有着一个空闭包的线程的一个 `Worker` 实例，这样一个 `Worker::new` 函数；
4. 在 `Thread::new` 中，会使用那个 `for` 循环的计数器，来生成一个 `id`、用那个 `id` 创建出一个新的 `Worker`，并将该 `Worker` 存储在那个矢量值中。


若咱们准备挑战一下，那么请尝试在查看清单 20-15 中代码之前，自己实现这些修改。

准备好了吗？下面就是有着一种做出前面那些修改的一种方式的清单 20-15。

文件名：`src/lib.rs`

```rust
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    // --跳过代码--
    pub fn new(size: usize) -> ThreadPool {
        assert! (size > 0);

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            workers.push(Worker::new(id));
        }

        ThreadPool { workers }
    }
    // --跳过代码--
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});

        Worker { id, thread }
    }
}
```

*清单 20-15：将 `ThreadPool` 修改为保存 `Worker` 实例而非直接保存线程*

由于 `ThreadPool` 现在保存的是一些 `Worker` 实例而非 `JoinHandle<()>` 实例，因此咱们已将其上那个字段的名字，从 `threads` 修改为了 `workers`。咱们将那个 `for` 循环中的计数器，用作给 `Worker::new` 的参数，同时咱们将每个新的 `Worker`，存储在那个名为 `workers` 的矢量值中。

外部代码（就像 `src/main.rs` 中咱们的服务器），无需知悉 `ThreadPool` 里某个 `Worker` 结构体使用方面的实现细节，因此咱们是将这个 `Worker` 结构体及其 `new` 函数，构造为了私有。`Worker::new` 函数使用了咱们给他的那个 `id`，并将经由使用空闭包而生成一个新线程，而创建出的一个 `JoinHandle<()>` 实例存储起来。


> 注意：若操作系统因没有足够系统资源而无法创建出一个线程，那么 `thread::spawn` 就将终止运行。那样的话，即使一些线程创建可能成功，也将导致咱们整个服务器终止运行。为简化起见，这种实现做法是无可厚非的，但在生产的线程池实现中，咱们就大概打算使用 [`std::thread::Builder`](https://doc.rust-lang.org/std/thread/struct.Builder.html) 与他的返回 `Result` 的 [`spawn`](https://doc.rust-lang.org/std/thread/struct.Builder.html#method.spawn) 方法了。

这段代码将编译，并将咱们指定给 `ThreadPool::new` 数目的 `Worker` 实例存储起来。但咱们 *仍* 未处理咱们在 `execute` 中得到的闭包。接下来就要看看怎样完成那一步。


### 经由通道把请求发送给线程

**Sending Requests to Threads via Channels**


接下来咱们将要解决的，便是所给到 `thread::spawn` 的那些闭包什么也没做的问题。当前，咱们在那个 `execute` 方法中，获取到了咱们打算执行的那个闭包。但咱们需要于那个 `ThreadPool` 创建期间，在咱们创建出各个 `Worker` 时，给到 `thread::spawn` 一个闭包。

咱们想要咱们刚创建出的那些 `Worker` 结构体，从一个保存在 `ThreadPool` 中的队列中获取要运行的代码，并把那些代码发送他的线程运行。

第 16 章中咱们学过的通道 -- 两个线程间通信的一种简单方式 -- 对于这个用例将是最佳的。咱们将使用一个函数的通道，作为作业队列，the queue of jobs，而 `execute` 将把来自 `ThreadPool` 的某项作业，发送到那些 `Worker` 实例，其将把该项作业，发送给他的线程。下面便是这个方案：

1. `ThreadPool` 将创建出一个通道，并保存于 `sender` 上；
2. 每个 `Worker` 实例，将保存于 `receiver` 上；
3. 咱们将创建出将保存那些咱们打算下发到通道上闭包的一个新 `Job` 结构体；
4. `execute` 方法将经由那个 `sender`，发送其打算执行的作业；
5. 在 `Worker` 实例的线程中，其将遍历其 `receiver` 并执行其所接收到的任何作业的闭包。

咱们来通过在 `ThreadPool::new` 中创建一个通道，并在 `ThreadPool` 实例中保存 `send` 开始，如下清单 20-16 中所示。其中的 `Job` 结构体现在没有保存任何东西，但将保存咱们下发到通道项目类型。

文件名：`src/lib.rs`

```rust
use std::{sync::mpsc, thread};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Job;

impl ThreadPool {
    // --跳过代码--
    pub fn new(size: usize) -> ThreadPool {
        assert! (size > 0);

        let (sender, receiver) = mpsc::channel();

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id));
        }

        ThreadPool { workers, sender }
    }

    // --跳过代码--
}
```

*清单 20-16：将 `ThreadPool` 修改为存储传递 `Job` 实例通道的 `sender`*

在 `ThreadPool::new` 中，咱们创建出来咱们的新通道，并让线程池保存了该通道的 `sender`。这段代码将成功编译。

下面就来尝试在这个线程池创建出该通道时，把其 `receiver` 传入各个 `worker`。咱们清楚咱们是要在那些 `workers` 生成的线程中使用这个 `receiver`，因此咱们将在那个闭包中，引用这个 `receiver` 参数。下面清单 20-17 中的代码尚不会很好地编译。

文件名：`src/lib.rs`

```rust
impl ThreadPool {
    // --跳过代码--
    pub fn new(size: usize) -> ThreadPool {
        assert! (size > 0);

        let (sender, receiver) = mpsc::channel();

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, receiver));
        }

        ThreadPool { workers, sender }
    }

    // --跳过代码--
}

// --跳过代码--
impl Worker {
    fn new(id: usize, receiver: mpsc::Receiver<Job>) -> Worker {
        let thread = thread::spawn(|| {
            receiver;
        });

        Worker { id, thread }
    }
}
```

*清单 20-17：将 `receiver` 传递给 `workers`*

咱们作出了一些小而简单直接的修改：咱们把那个 `receiver` 传入到 `Worker::new`，并随后在那个闭包里使用了他。

当咱们尝试检查这段代码时，就会得到如下报错：

```console
$ cargo check
    Checking hello v0.1.0 (/home/peng/rust-lang-zh_CN/hello)
error[E0382]: use of moved value: `receiver`
  --> src/lib.rs:27:42
   |
22 |         let (sender, receiver) = mpsc::channel();
   |                      -------- move occurs because `receiver` has type `std::sync::mpsc::Receiver<Job>`, which does not implement the `Copy` trait
...
27 |             workers.push(Worker::new(id, receiver));
   |                                          ^^^^^^^^ value moved here, in previous iteration of loop

For more information about this error, try `rustc --explain E0382`.
error: could not compile `hello` due to previous error
```

这段代码试图将 `receiver` 传递给多个 `Worker` 实例。正如回顾到第 16 章，这样做不会工作：Rust 所提供的通道实现，属于多 `producer`、单 `consumer` 的。这意味着咱们不能只克隆通道的消费端来修复这段代码。咱们也不打算将一条消息，多次发送给多个消费者；咱们是要一个带有多个 `worker` 的消息列表，如此每条消息，都将被一次性处理。

此外，从通道队列里取出一项作业，还涉及到令 `receiver` 可变，因此这些县城就需要一种共用与修改 `receiver` 的安全方式；否则，咱们就会面临竞争情形（如同第 16 章中所讲到的）。

回顾第 16 章中曾讨论过的线程安全灵巧指针：为在多个线程间共用所有权，以及实现这些线程修改值，咱们需要用到 `Arc<Mutex<T>>`。`Arc` 类型将实现多个 `worker` 都拥有那个 `receiver`，而 `Mutex` 将确保某个时刻只有一个 `worker` 从 `receiver` 获取一项作业。下面清单 20-18 给出了咱们需要作出的修改。

文件名：`src/lib.rs`

```rust
use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};
// --跳过代码--

impl ThreadPool {
    // --跳过代码--
    pub fn new(size: usize) -> ThreadPool {
        assert! (size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    // --跳过代码--
}

// --跳过代码--

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // --跳过代码--
    }
}
```

*清单 20-18：运用 `Arc` 与 `Mutex` 在那些 `worker` 间共用 `receiver`*

在 `ThreadPool::new` 中，咱们把 `receiver` 放入到一个 `Arc` 与一个 `Mutex` 中。对于各个新 `worker`，咱们克隆了那个 `Arc`，从而增加了引用计数，这样那些 `worker` 就可以共用 `receiver` 的所有权。

有了这些修改，代码就会编译了！咱们就要达到目的了！


### 实现 `execute` 方法

**Implementing the `execute` method**


咱们来最终实现那个 `ThreadPool` 上的 `execute` 方法。咱们还将把 `Job` 从结构体，修改为保存着 `execute` 接收到闭包类型的特质对象的类型别名。正如第 19 章 [“使用类型别名创建类型义词”](Ch19_Advanced_Features.md#使用类型别名创建类型同义词) 小节中曾讨论过的，类型别名实现了为易于使用而将长类型构造缩短。请看看下面清单 20-19.

文件名：`src/lib.rs`

```rust
// --跳过代码--

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    // --跳过代码--

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}
// --跳过代码--
```

*清单 20-19：为保存着各个闭包的 `Box` 创建出 `Job` 类型别名，并于随后把作业下发到通道*

使用咱们在 `execute` 中得到的闭包创建出一个新的 `Job` 实例后，咱们便把那项作业下发到通道的发送端。对于发送失败的情形，咱们调用了 `send` 上的 `unwrap` 方法。在比如咱们停止全部线程执行，即表示接收端已停止接收新消息时，发送失败就可能发生。在那个时刻，咱们是无法停止咱们的线程执行的：只要这个线程池存在，咱们的线程就会继续执行。咱们使用 `unwrap` 的原因，就是咱们清楚这样的失败情况不会发生，但编译器是不了解这点的。

但咱们还没有大功告成！在 `worker` 里，传递给 `thread::spawn` 的闭包，仍然只 *引用* 了通道的接收端。相反，咱们需要闭包一直循环，向通道接收端请求一项作业，并在其获取到一项作业时运行该项作业。下面咱们就来完成下面清单 20-20 中所给出的对 `Worker::new` 的修改。

文件名：`src/lib.rs`

```rust
// --跳过代码--

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println! ("Worker {id} 获取到一项作业；执行中。");

            job();
        });

        Worker { id, thread }
    }
}
```

*清单 20-20：在 `worker` 的线程中接收并执行作业*

这里，咱们首选调用了 `receiver` 上的 `lock` 来请求那个互斥量，mutex，并于随后调用 `unwrap` 来在出现任何错误时终止运行。在互斥量处于 *中毒，poisoned* 状态时，请求锁就会失败，在有别的某个线程终止运行的同时，持有着而没有释放该锁时，这种情况便会发生。在这种情况下，调用 `unrap` 来让这个线程终止运行，便是要采取的正确措施。请放心地把这个 `unwrap`，修改为一个带有对咱们有意义报错信息的 `expect`。

当咱们获得了那个互斥量上的锁时，咱们就会调用 `recv` 来从通道接收一个 `Job`。最后的 `unwrap` 也会带过这里的任何错误，在持有 `sender` 的线程已关闭时就会发生这些错误，就跟 `receiver` 关闭时那个 `send` 方法会返回 `Err` 类似。

到 `recv` 的调用会阻塞，因此在尚无作业时，当前线程将等待，直到有某项作业可用。`Mutex<T>` 确保了一次只有一个 `Worker` 线程是在尝试请求作业。

咱们的线程池现在就处于工作状态了！给他一次 `cargo run` 并构造一些请求：


```console
$ cargo run
   Compiling hello v0.1.0 (/home/lenny.peng/rust-lang-zh_CN/hello)
warning: field `workers` is never read
 --> src/lib.rs:7:5
  |
6 | pub struct ThreadPool {
  |            ---------- field in this struct
7 |     workers: Vec<Worker>,
  |     ^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: fields `id` and `thread` are never read
  --> src/lib.rs:48:5
   |
47 | struct Worker {
   |        ------ fields in this struct
48 |     id: usize,
   |     ^^
49 |     thread: thread::JoinHandle<()>,
   |     ^^^^^^

warning: `hello` (lib) generated 2 warnings
    Finished dev [unoptimized + debuginfo] target(s) in 0.60s
     Running `target/debug/hello`
Worker 1 获取到一项作业；执行中。
Worker 0 获取到一项作业；执行中。
Worker 2 获取到一项作业；执行中。
Worker 3 获取到一项作业；执行中。
Worker 1 获取到一项作业；执行中。
Worker 0 获取到一项作业；执行中。
```

成功了！咱们现在有了一个会异步执行 TCP 连接的线程池。绝不会有超过四个线程被创建出来，因此在服务器收到很多请求时，咱们的系统将不会过载。在咱们构造了一个到 `/sleep` 的请求时，服务器通过让另一线程运行别的一些请求，而将能服务这些请求。

> 注意：若咱们在多个窗口同时打开 `/sleep`，他们可能会在设置的时间间隔每次加载一个。有些 web 浏览器会出于缓存原因，而顺序执行同一请求的多个实例。这样的局限并不是由咱们的服务器导致的。

在了解了第 18 章中的 `while let` 循环后，咱们可能想知道，为何咱们没有如下清单 20-21 中所示的那样，编写 `worker` 线程的代码。

文件名：`src/lib.rs`

```rust
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            while let Ok(job) = receiver.lock().unwrap().recv() {
                println! ("Worker {id} 获取到一项作业；执行中。");

                job();
            }
        });

        Worker { id, thread }
    }
}
```

*清单 20-21：使用 `while let` 的一种 `Worker::new` 替代实现*

这段代码将会编译及运行，但不会产生所需的线程行为：慢速请求仍将导致别的请求等待被处理。至于原因则有点微妙：由于锁的所有权是基于 `lock` 方法返回的 `LockResult<MutexGuard<T>>` 中，`MutexGuard<T>` 的生命周期，因此这个 `Mutex` 结构体没有公开的 `unlock` 方法。在编译时，借用检查器可于随后，就除非咱们拿着 `Mutex` 所守卫的某项资源的锁，否则无法访问该项资源这一规则强制加以检查。但是，若咱们没有注意到 `MutexGuard<T>` 的生命周期，那么这样的实现同样能导致锁相较预期被占用更长时间。

由于在 `let` 之下，等号右侧的表达式中用到的任何临时值，都会在 `let` 语句结束时被立即丢弃，因此使用了 `let job = receiver.lock().unwrap().recv().unwrap();` 的清单 20-20 中代码是工作的。但是，`while let`（以及 `if let` 与 `match`） 则是在相关代码块结束前，不会丢弃那些临时值。在清单 20-21 中，锁会在到 `job()` 的调用其将保持被持有，这意味着别的 `worker` 就没法收到作业。
