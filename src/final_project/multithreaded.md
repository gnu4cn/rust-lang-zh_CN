# 从单线程服务器到多线程服务器

目前，服务器将依次处理每个请求，这意味着在第一个连接处理完毕之前，他不会处理第二个连接。当服务器收到越来越多的请求时，这种串行执行将越来越不理想。当服务器收到一个需要很长时间处理的请求时，后续请求就必须等该该请求处理完毕，即使这些新请求可被快速处理。我们需要解决这个问题，但首选我们将看看实际操作中的问题。


## 模拟慢速请求

我们将探讨处理慢速请求会怎样影响向当前服务器发出的其他请求。下面清单 21-10 实现了通过模拟慢速响应来处理对 `/sleep` 的请求，这将导致服务器在响应之前休眠 5 秒钟。

<a name="listing_21-10"></a>
文件名：`projects/hello/src/main.rs`

```rust
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
// --跳过代码--

fn handle_connection(mut stream: TcpStream) {
    // --跳过代码--

    let (status_line, filename) = match &request_line[..] {
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

**清单 21-10**：通过休眠 5 秒来模拟慢速请求

现在我们有三种情况，于是已从 `if` 切换为 `match`。我们需要显式地对 `request_line` 的一个切片匹配，以与字符串字面值模式匹配；`match` 不会像相等比较方式那样，执行自动引用和解引用。

第一个支臂与 [清单 21-9](./single-threaded.md#listing_21-9) 中的 `if` 代码块相同。第二个支臂匹配到 `/sleep` 的请求。收到该请求后，服务器将在渲染成功 HTML 页面之前休眠 5 秒。第三个支臂与清单 21-9 中的 `else` 代码块相同。

咱们可以看到我们的服务器是多么的原始：真正的库将以更简洁的方式处理多个请求的识别！

请使用 `cargo run` 启动服务器。然后，打开两个浏览器窗口：一个用于 `http://127.0.0.1/7878`，另一个用于 `http://127.0.0.1:7878/sleep`。当咱们像之前那样多次输入 `/` 的 URI，咱们将发现他响应很快。但当咱们先输入 `/sleep`，然后加载 `/` 时，就会发现 `/` 会等待 `sleep` 休眠 5 秒后才加载。

我们可使用多种技术，来避免请求在慢速请求后面积压，包括像在第 17 中那样使用异步；我们将实现的是线程池。


## 通过线程池提升吞吐量

所谓 *线程池，thread pool*，是一组已创建的线程，他们出于就绪状态并等待处理任务。当程序接收到新任务时，他会将池中的线程之一分配给该任务，进而该线程将处理该任务。在第一个线程处理期间，池中剩余的线程可用于处理任何新到任务。当第一个线程处理完其任务后，他会被返回到空闲线程池，准备处理新任务。线程池允许咱们同时处理连接，从而提高服务器的吞吐量。

我们将把线程池中的线程数量限制为少量，以保护我们免受拒绝服务，DoS，攻击；若我们让程序为每个传入的请求都创建一个新线程，那么当某人向我们的服务器发出 1000 万次请求时，就会耗尽服务器的所有资源，导致请求处理彻底瘫痪，从而造成严重破坏。

因此，与其生成无限数量的线程，我们不如让固定数量的线程在池中等待。传入的请求将发送到池中进行处理。线程池将维护一个传入请求的队列。池中的每个线程都将弹出池中的一个请求，处理该请求，然后向队列请求另一个请求。在这种设计下，我们最多可以同时处理 `N` 个请求，其中 `N` 是线程数。当每个线程都在处理耗时较长的请求时，后续请求仍然会在队列中积压，但我们增加了在到达该临界点之前，可以处理的耗时请求的数量。

这种技术只是提高 web 服务器吞吐量的众多方法之一。咱们可能探索的其他选项，比如

- [分叉汇合模型，fork/join model](https://en.wikipedia.org/wiki/Fork%E2%80%93join_model)、
- [单线程异步 I/O 模型，single-threaded async I/O model](https://medium.com/@sairaju.atukuri123/how-does-async-handle-api-requests-in-a-single-thread-1eeff8480dab)，
- 以及 [多线程异步 I/O 模型，multi-threaded async I/O model](https://en.wikipedia.org/wiki/Asynchronous_I/O) 等等。

若咱们对这一主题感兴趣，可以进一步了解其他解决方案并尝试实现他们；对于 Rust 这样的底层编程语言，所有这些选项都是可行的。

在开始实现线程池之前，我们来先讨论以下使用线程池子应呈现何种形态。当咱们尝试设计代码时，首先编写客户端接口有助于引导咱们的设计思路。应按照咱们希望调用代码的组织方式编写代码的 API；然后，在这种组织方式下实现功能，而不是先实现功能再设计公开 API。

与我们在第 12 章中的项目中使用的测试驱动开发的方式类似，我们在这里将使用编译器驱动开发，compiler-driven development。我们将编写所需函数的代码，然后我们将查看编译器中的报错，以确定下一步应如何修改代码使其正常运行。但在开始之前，我们将先探讨一种我们不会使用的技术作为起点。


### 为每个请求生成一个线程

首先，我们来探讨一下，当为每个连接都创建一个新线程时，我们的代码会是什么样子。正如早先提到的，由于可能生成无限数量的线程，这并非我们的最终方案，但他是构建一个可运行的多线程服务器的起点。然后，我们将添加线程池作为改进，从而对比这两种方案会更容易。

下面清单 21-11 展示了对 `main` 构造的更改，以便在 `for` 循环内为处理每个流而生成新线程。

<a name="listing_21-11"></a>
文件名：`projects/hello/src/main.rs`

```rust
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}
```

**清单 21-11**：为每个流都生成一个新线程

正如咱们在第 16 章中所学到的，`thread::spawn` 将创建一个新线程，然后在新线程中运行闭包中的代码。当咱们运行这段代码，并在浏览器中加载 `/sleep`，然后在另外两个浏览器 Tab 页中加载 `/`，咱们确实会发现到 `/` 的请求不必等待 `/sleep` 完成。然而，正如我们提到的，这最终将使系统不堪重负，因为咱们会无限制地创建新线程。

咱们可能还记得第 17 章中的内容，这正是异步和等待真正大显身手的情形！在我们构建线程池时请记住这一点，并思考在异步下会有何不同或相同点。


### 创建有限数量的线程

我们希望线程池以类似、熟悉的方式工作，这样在使用我们 API 的代码中，从单线程切换到线程池是，就无需进行大量更改。下面清单 21-12 展示了我们打算用来替换 `thread::spawn` 的 `ThreadPool` 结构体的假设接口。

<a name="listing_21-12"></a>
文件名：`src/main.rs`

```rust
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}
```

**清单 21-12**：我们的理想 `ThreadPool` 接口

我们使用 `ThreadPool::new` 创建一个线程池，带有可配置的线程数量，在这一情形下为四个。然后，在 `for` 循环中，`pool.execute` 有着与 `thread::spawn` 类似的接口，即他取一个闭包，线程池应针对每个流运行该闭包。我们需要实现 `pool.execute` 方法，使其取闭包并将该闭包交由线程池中的某个线程执行。这段代码还不会编译，但我们将尝试编译，以便编译器可以指导我们如何修复他。


### 使用编译器驱动开发构建 `ThreadPool`

请对 `src/main.rs` 进行清单 21-12 中的修改，然后我们来运用 `cargo check` 中的编译器报错驱动我们的开发。下面是我们得到的第一个报错：

```console
$ cargo check
    Checking hello v0.1.0 (/home/hector/rust-lang-zh_CN/projects/hello)
error[E0433]: failed to resolve: use of undeclared type `ThreadPool`
  --> src/main.rs:11:16
   |
11 |     let pool = ThreadPool::new(4);
   |                ^^^^^^^^^^ use of undeclared type `ThreadPool`

For more information about this error, try `rustc --explain E0433`.
error: could not compile `hello` (bin "hello") due to 1 previous error
```

太好了！这个错误告诉我们，我们需要一个 `ThreadPool` 类型或模组，所以我们现在就构建一个。我们的 `ThreadPool` 实现将独立于我们的 web 服务器正在执行的工作类别。因此，我们来将 `hello` 代码箱，从二进制代码箱切换为库代码箱，来保存我们的 `ThreadPool` 实现。更改为库代码箱后，我们还可以针对我们打算使用线程池来完成的任何工作，都使用这个独立的线程池，而不仅仅用于服务 web 请求。

请创建一个 `src/lib.rs` 文件，包含以下代码，这是目前我们可以实现的 `ThreadPool` 结构体的最简单定义：

文件名：`projects/hello/src/lib.rs`

```rust
pub struct ThreadPool;
```

然后，编辑 `main.rs`，通过添加以下代码到 `src/main.rs` 的顶部，从库代码箱带入 `ThreadPool` 作用域：

文件名：`projects/hello/src/main.rs`

```rust
use hello::ThreadPool;
```

这段代码仍然无法运行，但我们来再检查一遍，以得到下一个我们需要解决的报错：

```console
$ cargo check
    Checking hello v0.1.0 (/home/hector/rust-lang-zh_CN/projects/hello)
error[E0599]: no function or associated item named `new` found for struct `ThreadPool` in the current scope
  --> src/main.rs:13:28
   |
13 |     let pool = ThreadPool::new(4);
   |                            ^^^ function or associated item not found in `ThreadPool`

For more information about this error, try `rustc --explain E0599`.
error: could not compile `hello` (bin "hello") due to 1 previous error
```

这个报错表明，接下来我们需要为 `ThreadPool` 创建一个名为 `new` 的关联函数。我们还知道，`new` 需要有个形参，可以接受 `4` 作为实参，并且应该返回一个 `ThreadPool` 实例。我们来实现一个有着这些特征的最简单的 `new` 函数：

文件名：`projects/hello/src/lib.rs`

```rust
pub struct ThreadPool;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        ThreadPool
    }
}
```

我们选择 `usize` 作为 `size` 参数的类型，因为我们知道负数的线程数毫无意义。我们还知道，我们将使用 `4` 作为线程集合中元素的个数，这就是 `usize` 类型的用途，正如第 3 章中 [整型](../programming_concepts/data_types.md#整型) 小节中讨论的那样。

我们来再次检查代码：

```console
$ cargo check
    Checking hello v0.1.0 (/home/hector/rust-lang-zh_CN/projects/hello)
error[E0599]: no method named `execute` found for struct `ThreadPool` in the current scope
  --> src/main.rs:18:14
   |
18 |         pool.execute(|| {
   |         -----^^^^^^^ method not found in `ThreadPool`

For more information about this error, try `rustc --explain E0599`.
error: could not compile `hello` (bin "hello") due to 1 previous error
```

现在报错出现，是因为我们在 `ThreadPool` 上没有 `execute` 方法。回顾 [创建有限数量的线程](#创建有限数量的线程) 小节，我们决定线程池应该有个类似于 `thread::spawn` 的接口。此外，我们将实现 `execute` 函数，使其取得给予他的闭包，并将该闭包交给线程池中的空闲线程运行。

我们将定义 `ThreadPool` 上的 `execute` 方法为取一个闭包作为参数。回顾第 13 章中的 [从闭包中迁出捕获值](../functional_features/closures.md#从闭包中迁出捕获值) 小节，我们可以通过三种不同特质取闭包作为参数：`Fn`、`FnMut` 与 `FnOnce`。我们需要决定在这里使用哪种闭包类别。我们知道我们最终将执行一些类似于标准库的 `thread::spawn` 实现的操作，因此我们可以查看 `thread::spawn` 的签名，在参数上有哪些边界。文档向我们展示了以下内容：

```rust
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
```

其中 `F` 类型参数是我们在这里关心的；`T` 类型参数与返回值有关，而我们并不关心这点。我们可以看到，`spawn` 使用 `FnOnce` 作为 `F` 的特质边界。这很可能也是我们想要的，因为我们最终将传递 `execute` 中获得的实参给 `spawn`。由于运行请求的线程将只执行该请求的闭包一次，这与 `FnOnce` 中的 `Once` 一致，因此我们可以进一步确信 `FnOnce` 就是我们要使用的特质。

`F` 类型参数还有着特质边界 `Send` 和生命周期边界 `'static`，这在我们的情形下非常有用：我们需要 `Send` 来从一个线程转移闭包到另一线程，需要 `'static` 是由于我们不知道线程执行需要多长时间。我们来对 `ThreadPool` 创建一个 `execute` 方法，将取有着以下边界的类型 `F` 的泛型参数：

文件名：`src/lib.rs`

```rust
impl ThreadPool {
    // -- 跳过代码 --
    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
    {
    }
}
```

我们在 `FnOnce` 之后仍然使用 `()`，因为这个 `FnOnce` 表示一个不取参数且返回单元值类型 `()` 的闭包。就像函数定义一样，返回值类型可在签名中省略，但即使我们没有参数，我们仍然需要这对括号。

同样，这是 `execute` 方法的最简单实现：他什么也不做，但我们只是试图让代码编译。我们来再次检查一下：

```console
$ cargo check
    Checking hello v0.1.0 (/home/hector/rust-lang-zh_CN/projects/hello)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s
```

他编译了！但请注意，当咱们尝试运行 `cargo run` 并在浏览器中发出请求时，咱们将看到浏览器中看到在本章开头曾看到的那些报错。我们的库实际上还没有调用传递给 `execute` 的闭包！

> **注意**：关于像 Haskell 和 Rust 这样有着严格编译器的语言，咱们或许听说过一种说法，即 “当代码编译时，他就会运行。” 但这种说法并非放之四海而皆准。我们的项目编译了，但他什么也没做！若我们正在构建一个真实且完整的项目，那么现在正是开始编写单元测试的好时机，以验证代码不仅会编译，*还* 有着我们想要的行为。

请思考：若我们即将执行一个未来值而不是闭包，这里会有什么不同？


### 验证 `new` 中的线程数量

我们没有对 `new` 和 `execute` 的参数执行任何操作。我们来以我们希望的行为实现这两个函数的主体。首先，我们来思考一下 `new`。之前我们为 `size` 参数选择了无符号类型，因为线程数为负的线程池没有意义。然而，有着零个线程的线程池也没有意义，但零是完全有效的 `usize`。在返回 `ThreadPool` 实例之前，我们将添加检查 `size` 是否大于零的代码，在返回一个 `ThreadPool` 实例前，添加检查 `size` 大于零的代码，并当程序通过使用 `assert!` 宏收到零时让程序终止运行，如下清单 21-13 中所示。

<a name="listing_21-13"></a>
文件名：`projects/hello/src/lib.rs`

```rust
impl ThreadPool {
    /// 创建一个新的 ThreadPool。
    ///
    /// 其中 size 为线程池中线程的数量。
    ///
    /// # Panics
    ///
    /// `new` 函数将在 size 为零时终止运行。
    pub fn new(size: usize) -> ThreadPool {
        assert! (size > 0);

        ThreadPool
    }

    // --跳过代码--
}
```

**清单 21-13**：实现 `ThreadPool` 为当 `size` 为零时终止运行

我们还通过 [文档注释](../crates-io/publishing.md#制作有用的文档注释) 为我们的 `ThreadPool` 添加了一些文档。请注意，我们遵循了良好的文档实践，添加了一个小节，之处我们的函数可能会终止运行的情况，正如第 14 章中所讨论的那样。请尝试运行 `cargo doc --open` 并点击 `ThreadPool` 结构体，看看为 `new` 生成的文档是什么样的！

> **译注**：此时 `new` 的文档如下。
>
> ![](../images/21-01.doc_for_new.png)

与其像这里这样添加 `assert!` 宏，我们也可以改 `new` 为 `build`，并像在 [清单 12-9](../io_project/refactoring.md#listing_12-9) 中 I/O 项目中的 `Config::build` 那样返回一个 `Result`。但我们已经决定在这种情况下，尝试创建一个没有任何线程的线程池应该是不可恢复的错误。若咱们感兴趣，那就编写一个有着以下签名的名为 `build` 的函数，与 `new` 函数比较：

```rust
pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
```


### 创建空间来存储线程

现在我们有一种方法知道，我们有要存储在池中的有效线程数量，我们可以创建这些线程并在 `ThreadPool` 结构体中存储他们。但我们要怎样 “存储” 线程呢？我们再看看 `thread::spawn` 的签名：

```rust
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
```

`spawn` 函数返回一个 `JoinHandle<T>`，其中 `T` 是闭包返回的类型。我们也来尝试使用 `JoinHandle`，看看会发生什么。在我们的情形下，传递给线程池的闭包将处理连接，并且不返回任何值，因此 `T` 将是单元值类型 `()`。

下面清单 21-14 中的代码虽然将编译，当目前尚未创建任何线程。我们已修改 `ThreadPool` 的定义为包含一个 `thread::JoinHandle<()>` 实例的矢量值，初始化该矢量值的容量为 `size`，设置了一个 `for` 循环，将运行一些代码来创建线程，并返回一个包含这些线程的 `ThreadPool` 实例。

<a name="listing_21-14"></a>
文件名：`projects/hello/src/lib.rs`

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
            // 创建出一些线程并将其存储在矢量中
        }

        ThreadPool { threads }
    }
    // --跳过代码--
}
```

**清单 21-14**：为 `ThreadPool` 创建一个用于保存线程的矢量值

我们在库代码箱中带入了 `std::thread` 到作用域，因为我们使用 `thread::JoinHandle` 作为 `ThreadPool` 中矢量值项目的类型。

一旦收到有效的大小，我们的 `ThreadPool` 就会创建一个可包含 `size` 个条目的新矢量值。`with_capacity` 函数执行与 `Vec::new` 相同的任务，但有个重要的区别：他会预先分配矢量值中的空间。由于我们知道需要在矢量值中存储 `size` 个元素，因此预先完成这种分配比使用 `Vec::new` 稍微高效一些，后者会在元素插入时调整自身大小。

在咱们再次运行 `cargo check` 时，他应该会成功。


### 发送 `ThreadPool` 中的代码到线程

我们在清单 21-14 中的 `for` 循环中，留下了关于线程创建的注释。在这里，我们将探讨如何实际创建线程。标准库提供了 `thread::spawn` 作为创建线程的方式，而 `thread::spawn` 期望在线程创建后，立即获得该线程应该运行的一些代码。然而，在我们的情形下，我们打算先创建线程，然后让他们 *等待* 我们稍后将发送的代码。标准库的线程实现并未包含这样做的任何方式；我们必须手动实现他。

我们将通过在 `ThreadPool` 与线程之间引入一种新的数据结构来实现这种行为，该数据结构将管理这种新行为。我们称这一数据结构为 *Worker*，这是线程池实现中常用的术语。`Worker` 会选取需要运行的代码，并他的线程中运行这些代码。

设想餐厅厨房里的工作人员：工作人员会等待顾客下但，然后负责接收菜单并完成烹饪。

我们不再在线程池中存储 `JoinHandle<()>` 实例的矢量值，而是存储 `Worker` 结构体的实例。每个 `Worker` 都将存储单个 `JoinHandler<()>` 实例。然后，我们将对 `Worker` 实现一个方法，该方法将取一个要运行的代码闭包，并将闭包发送到已经运行的线程来执行。我们还将给予每个 `Worker` 一个 `id`，以便在日志记录或调试时，能够区分线程池中不同的 `Worker` 实例。

以下是在创建 `ThreadPool` 时将发生的新过程。在以这种方式设置好 `Worker` 后，我们将实现发送闭包到线程的代码：

1. 定义一个 `Worker` 结构体，包含一个 `id` 和一个 `JoinHandler<()>`；
2. 修改 `ThreadPool` 为包含一个 `Worker` 实例的矢量值；
3. 定义一个 `Worker::new` 函数，取一个 `id` 编号并一个 `Worker` 实例，包含该 `id` 和一个以空闭包生成的线程；
4. 在 `ThreadPool::new` 中，使用 `for` 循环计数器生成一个 `id`，使用该 `id` 创建一个新的 `Worker`，并存储该 `Worker` 于矢量值中。


若咱们愿意接受挑战，不妨在查看清单 21-15 中的代码之前，先尝试自己实现这些修改。

准备好了吗？下面是清单 21-15，有着一种进行上述修改的方式。

<a name="listing_21-15"></a>
文件名：`projects/hello/src/lib.rs`

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

**清单 21-15**：修改 `ThreadPool` 为包含 `Worker` 实例，而非直接包含线程

我们将 `ThreadPool` 中的字段名字从 `threads` 修改为了 `workers`，因为他现在包含的是 `Worker` 实例，而非 `JoinHandle<()>` 实例。我们使用 `for` 循环中的计数器作为 `Worker::new` 的参数，并存储每个新的 `Worker` 在名为 `workers` 的矢量值中。

外部代码（如 `projects/hello/src/main.rs` 中的服务器）不需要知道有关 `ThreadPool` 中使用 `Worker` 结构体的实现细节，因此我们构造 `Worker` 结构体及其 `new` 函数为私有。`Worker::new` 函数使用我们给予他的 `id`，并存储一个由使用空闭包生成新线程创建的 `JoinHandler<()>` 实例。


> **注意**：当操作系统因没有足够系统资源，而无法创建线程时，`thread::spawn` 将终止运行。这会导致整个服务器终止运行，即使部分线程的创建可能成功。为了简化起见，这种实现行为是可以接受的，但在生产环境的线程池实现中，咱们可能更希望使用 [`std::thread::Builder`](https://doc.rust-lang.org/std/thread/struct.Builder.html)，及其返回 `Result` 的 [`spawn`](https://doc.rust-lang.org/std/thread/struct.Builder.html#method.spawn) 方法。

这段代码将编译，并将存储我们作为参数指定给 `ThreadPool::new` 数量的 `Worker` 实例。但我们 *仍然* 没有处理我们在 `execute` 中得到的闭包。接下来，我们来看看怎样做到这点。


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


（End）


