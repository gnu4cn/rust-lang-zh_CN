# 无惧并发

**Fearless Concurrency**

安全并高效地处理并发编程，是 Rust 的另一主要目标。所谓 *并发编程，concurrent programming*，是指其中程序的各部分独立地执行着，而 *并行编程，parallel programming*，则是指程序的不同部分于同一时间执行，随着越来越多的计算机利用了多处理器的优势，这两种编程范式变得日益重要起来。历史上，这两种情景下的编程，曾是有难度且容易出错的：Rust 就有望来改变这种局面。

早期阶段，Rust 团队曾认为确保内存安全与防止并发问题，属于要以不同方法来解决的两个单独挑战。随着时间的推移，团队发现所有权与类型系统，正是有助于管理内存安全，*及* 并发问题的一套强有力的工具！经由利用所有权与类型检查，许多的并发错误，就成了 Rust 中的编译时错误，而非运行时错误。因此，就不再是要咱们，在出现运行时并发错误时，花费大量时间尽力重现那些确切情形，而是那些不正确代码，将拒绝编译，并给出代码问题的错误提示。由此，咱们就可以在编写出错误代码时，而非潜在地于交付代码到生产之后，修复好这些代码。这里将 Rust 此方面的特性，亲切地取名为 *无惧并发，fearless concurrency*。无惧并发实现了编写出不带难以察觉错误的代码，且易于在不引入新代码错误之下，对代码加以重构。

> **注意**：为简化起见，这里将把许多的这些问题，指为 *并发，concurrency*，而非称作更准确的 *并发及/或并行，concurrency and/or parallel*。若本书是有关并发及/或并行编程的书，那么咱们就会更为具体。对于本章，请在任何提及 *并发* 之处，在内心里将其以 *并发及/或并行* 代换。

许多语言在他们所提供的，用于解决并发问题的方案上，都是机械教条主义的。比如，Erlang 有着消息传递方面并发的优雅功能，但在共用线程间状态方面，却只有一些晦涩难懂的的途径，for example, Erlang has elegant functionality for message-passing concurrency, but has only obscure ways to share state between threads。对于这类高级语言来讲，仅支持可行方案的子集，是说得通的一种策略，这是由于高级语言以放弃部分的掌控，而换取到抽象方面的收益。然而，那些底层语言，则被期望在各种情形下，都要提供最具性能的方案，进而在硬件上有着较少抽象。因此，Rust 便提供了用以适合于咱们自己不同情形与需求的各种方式，对问题加以建模的各种工具，therefore, Rust offers a variety of tools for modeling problems in whatever way is appropriate for your situtation and requirements。

以下即为本章咱们将涵盖的几个话题：

- 怎样创建出线程，来在同一时间运行代码的不同片段，how to create threads to run multiple pieces of code at the same time；
- *消息传递，message-passing* 方面的并发，其中有着于线程间发送消息的一些通道；
- *状态共用，shared-state* 方面的并发，其中多个线程均对某个数据加以访问；
- `Sync` 与 `Send` 特质，他们俩把 Rust 并发方面的保证，扩展到 Rust 使用者所定义的类型，以及由标准库所提供的那些类型。


## 运用线程来同步运行代码

**Using Threads to Run Code Simutaneously**

在绝大多数当前的操作系统中，被执行的程序代码，都是运行于 *进程，a process* 中的，而所在的操作系统，则会同时管理多个进程。在程序内部，咱们同样可以有着同步运行的一些独立部分。运行这些独立部分的特性，便被称作 *线程，threads*。比如，web 服务器就可以有多个线程，如此他就可以在同一时间，响应多于一个的请求。

将咱们程序的运算，拆分为多个线程，来在同一时间运行多个任务，可以提升性能，但这样也增加了复杂度。由于线程能够同步运行，因此在于不同线程上，将要运行代码哪个部分的顺序方面，就没有了某种固有保证，because threads can run simultaneously, there's no inherent guarantee about the order in which parts of your code on different threads will run。这就会导致一些问题，诸如：

- 竞争局面，其中线程正以不一致顺序，访问着一些数据或资源；
- 死锁问题，其中两个线程正相互等待，而阻止了他们继续运行下去；
- 只在一些确切情形下才发生，而难于重现并可靠修复的代码错误。

Rust 试图消除这些运用线程方面的负面影响，但在多线程情景下的编程，仍要深思熟虑，并要求与运行在单线程下程序，截然不同的代码架构。

诸多编程语言，都是以少数几种不同途径，实现的线程，且多数操作系统，均提供了编程语言为可以创建出线程而调用的 API。Rust 标准库使用的是线程实现的 1:1 模型，由此程序就会以一个语言线程，对应使用一个操作系统线程。也有实现了别的线程操作模型的代码箱，对这种 1:1 模型做出了取舍。


### 使用 `spawn` 函数创建出一个新的线程

要创建出一个新的线程，咱们就要调用 `thread::spawn` 函数，并传递给他一个包含了打算在这个新线程中运行代码的闭包（在第 13 章中曾谈到过闭包）。下面清单 16-1 中的示例，会打印出来自主线程的一些文本，以及来自新线程的一些文本：

文件名：`src/main.rs`

```rust
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println! ("\t- 你好，这是来自生成线程的数字 {} !", i);
            thread::sleep(Duration::from_millis(20));
        }
    });

    for i in 1..5 {
        println! ("- 你好，这是来自主线程的数字 {} !", i);
        thread::sleep(Duration::from_millis(20));
    }
}
```

*清单 16-1：创建出一个新线程来打印某物件，与此同时主线程也在打印着其他东西*

请注意在 Rust 程序主线程完毕时，全部生成的线程就被关闭了，而不论他们是否已结束运行。该程序的输出每次都会有些许不同，但其看起来将如下所示：

```console
- 你好，这是来自主线程的数字 1 !
        - 你好，这是来自生成线程的数字 1 !
- 你好，这是来自主线程的数字 2 !
        - 你好，这是来自生成线程的数字 2 !
- 你好，这是来自主线程的数字 3 !
        - 你好，这是来自生成线程的数字 3 !
- 你好，这是来自主线程的数字 4 !
        - 你好，这是来自生成线程的数字 4 !
        - 你好，这是来自生成线程的数字 5 !
```

到 `thread::sleep` 的调用，强制线程停止其执行短暂的时间，而允许别的线程运行。这些线程可能会轮流运行，但那并无保证：这取决于咱们的操作系统调度线程的方式。在此运行中，主线程就先行打印了，即便生成的线程中的打印语句，首先出现在代码中。而即便这里告诉了生成的线程，打印直到 `i` 为 `9` 的时候，但 `i` 在主线程关闭之前，仍只到了 `5`。

若在运行此代码时，只看到主线程的输出，或未看到任何重叠部分，那么就要尝试增加其中那个范围（`1..10`, `1..5`）的数字，来给操作系统创造出，更多的与线程之间切换的机会。


### 使用 `join` 把手，等待全部线程结束

**Waiting for All Threads to Finish Using `join` Handles**

清单 16-1 中的代码，不仅会由于主线程的结束而提前停止生成线程，并因为在线程运行的顺序上没有保证，咱们还根本无法确保其中的生成线程将得到完整运行！

> **注**：在 `thred::sleep` 为 `1ms` 时，将偶发出现下面的运行结果：

```console
- 你好，这是来自主线程的数字 1 !
        - 你好，这是来自生成线程的数字 1 !
- 你好，这是来自主线程的数字 2 !
        - 你好，这是来自生成线程的数字 2 !
        - 你好，这是来自生成线程的数字 3 !
- 你好，这是来自主线程的数字 3 !
- 你好，这是来自主线程的数字 4 !
        - 你好，这是来自生成线程的数字 4 !
        - 你好，这是来自生成线程的数字 %
```

咱们可以通过将 `thread::spawn` 的返回值，保存在一个变量中，来修复该生成线程不运行或提前结束的问题。`thread::spawn` 的返回值类型为 `JoinHandle`。而 `JoinHandle` 值则是一个自有值，在咱们于其上调用 `join` 方法时，他将等待其线程执行完毕。下面清单 16-2 就给出了怎样使用清单 16-1 中所创建出的那个 `JoinHandle`，来确保该生成线程在 `main` 退出之前执行完毕：

文件名：`src/main.rs`

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println! ("\t- 你好，这是来自生成线程的数字 {} !", i);
            thread::sleep(Duration::from_millis(20));
        }
    });

    for i in 1..5 {
        println! ("- 你好，这是来自主线程的数字 {} !", i);
        thread::sleep(Duration::from_millis(20));
    }

    handle.join().unwrap();
}
```

*清单 16-2：保存一个来自 `thread::spawn` 的 `JoinHandle` 来确保该线程运行完毕*

> **注**：结合第 9 章中 [因错误而中止的快捷方式：`unwrap` 与 `expect`](Ch09_Error_Handling.md#shortcuts-for-panic-on-error-unwrap-and-expect)，表明 `join` 返回的是个 `Result<T, E>` 类型的枚举值。

在这个把手上调用 `join`，就会阻塞那个当前运行的线程，直到由该把手所表示的该线程终止。所谓 *阻塞，blocking* 某个线程，是指那个线程被阻止执行工作或退出，*blocking* a thread means that thread is prevented from performing work or exiting。由于咱们已将到 `join` 的调用，放在了那个主线程的 `for` 循环之后，因此运行清单 16-2 中的代码，应产生出如下类似的输出（注：但每次运行的输出仍然不同）：

```console
- 你好，这是来自主线程的数字 1 !
        - 你好，这是来自生成线程的数字 1 !
- 你好，这是来自主线程的数字 2 !
        - 你好，这是来自生成线程的数字 2 !
- 你好，这是来自主线程的数字 3 !
        - 你好，这是来自生成线程的数字 3 !
- 你好，这是来自主线程的数字 4 !
        - 你好，这是来自生成线程的数字 4 !
        - 你好，这是来自生成线程的数字 5 !
        - 你好，这是来自生成线程的数字 6 !
        - 你好，这是来自生成线程的数字 7 !
        - 你好，这是来自生成线程的数字 8 !
        - 你好，这是来自生成线程的数字 9 !
```

两个线程依旧交替运行，但因为这个到 `handle.join()` 的调用，主线程就会等待，而在生成线程完毕之前不会结束。

不过来看看像下面这样，当咱们把 `handle.join()` 移至 `main` 中那个 `for` 循环前面时，会发生什么：

文件名：`src/main.rs`

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println! ("\t- 你好，这是来自生成线程的数字 {} !", i);
            thread::sleep(Duration::from_millis(20));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println! ("- 你好，这是来自主线程的数字 {} !", i);
        thread::sleep(Duration::from_millis(20));
    }
}
```

主线程将等待生成线程运行完毕，并于随后运行他的 `for` 循环，因此输出将不再交错，如下所示：

```console
        - 你好，这是来自生成线程的数字 1 !
        - 你好，这是来自生成线程的数字 2 !
        - 你好，这是来自生成线程的数字 3 !
        - 你好，这是来自生成线程的数字 4 !
        - 你好，这是来自生成线程的数字 5 !
        - 你好，这是来自生成线程的数字 6 !
        - 你好，这是来自生成线程的数字 7 !
        - 你好，这是来自生成线程的数字 8 !
        - 你好，这是来自生成线程的数字 9 !
- 你好，这是来自主线程的数字 1 !
- 你好，这是来自主线程的数字 2 !
- 你好，这是来自主线程的数字 3 !
- 你好，这是来自主线程的数字 4 !
```

诸如 `join` 于何处被调用这样的细节，均会影响到咱们的线程，是否在同一时间运行。


### 在线程上使用 `move` 闭包

**Using `move` Closures with Threads**

由于传递给 `thread::spawn` 的闭包随后将取得其用到的环境中一些值的所有权，由此就会把这些值的所有权，从一个线程转移到另一线程，因此咱们今后将经常在这些闭包上，使用 `move` 关键字。在第 13 章 [“捕获引用或迁移所有权”](Ch13_Functional_Language_Features_Iterators_and_Closures.md#capturing-reference-or-moving-ownership) 小节，咱们就曾讨论过闭包语境下的 `move` 关键字。现在，咱们将更多地着重于 `move` 与 `thread::spawn` 之间的互动。

请注意在清单 16-1 中，传递给 `thread::spawn` 的那个闭包没有取任何参数：咱们没有在生成线程中，使用主线程中的任何数据。为在生成线程中使用主线程中的数据，那么生成线程的闭包就必须捕获其所需的值。下面清单 16-3 给出了在主线程中创建出一个矢量值，并在生成线程中用到这个矢量值的一种尝试。然而，正如即将看到的那样，这将尚不会运作。

文件名：`src/main.rs`

```rust
use std::thread;

fn main() {
    let v = vec! [1, 2, 3];

    let handle = thread::spawn(|| {
        println! ("这里有个矢量值：{:?}", v);
    });

    handle.join().unwrap();
}
```

*清单 16-3：尝试在另一线程中，使用由主线程创建出的一个矢量值*

这个闭包用到了 `v`，因此他将捕获 `v` 并将其构造为该闭包环境的一部分。由于 `thread::spawn` 是在一个新线程中运行此闭包，因此咱们应能够在那个新线程内部访问 `v`。然而在编译这个示例时，咱们会得到如下报错：

```console
cargo run                                                                                                                        lennyp@vm-manjaro
   Compiling concur_demo v0.1.0 (/home/lennyp/rust-lang/concur_demo)
error[E0373]: closure may outlive the current function, but it borrows `v`, which is owned by the current function
  --> src/main.rs:9:32
   |
9  |     let handle = thread::spawn(|| {
   |                                ^^ may outlive borrowed value `v`
10 |         println! ("这里有个矢量值：{:?}", v);
   |                                           - `v` is borrowed here
   |
note: function requires argument type to outlive `'static`
  --> src/main.rs:9:18
   |
9  |       let handle = thread::spawn(|| {
   |  __________________^
10 | |         println! ("这里有个矢量值：{:?}", v);
11 | |     });
   | |______^
help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword
   |
9  |     let handle = thread::spawn(move || {
   |                                ++++

For more information about this error, try `rustc --explain E0373`.
error: could not compile `concur_demo` due to previous error
```

Rust *推断出了，infers* 怎样去捕获 `v`，并由于 `println!` 值需要到 `v` 的一个引用，因此该闭包就尝试借用 `v`。然而，这里有个问题：Rust 无法识别出这个生成线程将运行多久，因此他就不清楚到 `v` 的引用是否将始终有效。

下面清单 16-4 提供了更倾向于有着到 `v` 的不将有效引用的一种场景：

文件名：`src/main.rs`

```rust
#![allow(dead_code)]
#![allow(unused_variables)]

use std::thread;

fn main() {
    let v = vec! [1, 2, 3];

    let handle = thread::spawn(|| {
        println! ("这里有个矢量值：{:?}", v);
    });

    drop(v); // 噢，不要啊！

    handle.join().unwrap();
}
```

*清单 16-4：有着尝试从弃用了 `v` 的主线程捕获到 `v` 引用的闭包的一个线程*

若 Rust 运行咱们运行此代码，那么就有可能在一点也没有运行那个生成线程下，其就会被立即置于后台中，if Rust allowed us to run this code, there's a possibility the spawned thread would be immediately put in the background without running at all。那个生成线程内部有着一个到 `v` 的引用，而主线程则使用第 15 章中曾讨论过的 `drop` 函数，立即弃用了 `v`。随后，在生成线程开始执行时，`v` 就不再有效了，一次到他的引用也失效了。噢，不要！

要修复清单 16-3 中的编译器错误，咱们可以使用错误消息中的建议：

```console
help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword
   |
9  |     let handle = thread::spawn(move || {
   |                                ++++

```

经由在那个闭包前添加 `move` 关键字，咱们就强制该闭包取得其用到值的所有权，而非让 Rust 来推断出他应借用该值。下面清单 16-5 给出的对清单 16-3 的修改，将如咱们设想的那样编译和运行：

文件名：`src/main.rs`

```rust
use std::thread;

fn main() {
    let v = vec! [1, 2, 3];

    let handle = thread::spawn(move || {
        println! ("这里有个矢量值：{:?}", v);
    });

    handle.join().unwrap();
}
```

*清单 16-5：使用 `move` 关键字来强制闭包取得他所用到值的所有权*

或许也会尝试以同样做法，通过使用 `move` 关键字，去修复清单 16-4 中，主线程调用了 `drop` 的代码。然而，由于清单 16-4 尝试完成的事情，因为一种不同原因而不被允许，那么这样的修复就不会凑效。在咱们把 `move` 添加到闭包时，咱们就会把 `v` 迁移到该闭包的环境中，进而咱们就无法再在主线程中，于其上调用 `drop` 了。这是会得到如下的编译器错误：

```console
$ cargo run                                                                                                                        lennyp@vm-manjaro
   Compiling concur_demo v0.1.0 (/home/lennyp/rust-lang/concur_demo)
error[E0382]: use of moved value: `v`
  --> src/main.rs:13:10
   |
7  |     let v = vec! [1, 2, 3];
   |         - move occurs because `v` has type `Vec<i32>`, which does not implement the `Copy` trait
8  |
9  |     let handle = thread::spawn(move || {
   |                                ------- value moved into closure here
10 |         println! ("这里有个矢量值：{:?}", &v);
   |                                            - variable moved due to use in closure
...
13 |     drop(v);
   |          ^ value used here after move

For more information about this error, try `rustc --explain E0382`.
error: could not compile `concur_demo` due to previous error
```

Rust 的所有权规则，再次挽救了咱们！由于 Rust 一直以来的保守，以及只为那个线程借用了 `v`，就意味着主线程理论上可以令到生成线程的引用失效，而得到了清单 16-3 中代码的报错。通过告知 Rust 将 `v` 的所有权迁移到生成线程，咱们就向 Rust 保证了主线程不会再使用 `v`。而若咱们以同样方式修改清单 16-4，那么随后在咱们于主线程中尝试使用 `v` 时，就破坏了那些所有权规则。这个 `move` 关键字，覆盖了 Rust 借用方面的保守做法；但他并无让咱们破坏所有权规则。

有了线程及线程 API 方面的基本认识，接下来就有看看用线程可以 *做，do* 些什么。


## 使用消息传递来在线程间传输数据

**Using Message Passing to Transfer Data Between Threads**

一种日渐流行的确保并发安全的方法，便是 *消息传递，message passing*，其中线程或参与者，通过相互发送包含数据的消息进行通信。下面就是摘自 [Go 语言文档](https://golang.org/doc/effective_go.html#concurrency) 的一句口号：“勿要通过共用内存进行通信；而要经由通信来共用内存。”

为达成消息发送式的并发，Rust 标准库提供了 *信道，channels* 的一种实现。所谓信道，即数据被从一个线程，发送到另一线程，这种形式的一个通用编程概念。

咱们可以把编程中的信道，设想为带流向的水渠，如同一条小溪或一条河。在咱们把像是一只塑胶小黄鸭投入到一条小河中时，他就会顺流而下到达该水路的尽头。

信道有着两端：一个发送者和一个接收者。发送端即在将小黄鸭投入到河流中的上游位置，而接收端即为小黄鸭抵达的下游了。咱们代码中一个部分以打算发送的数据，调用发送者上的方法，而另一个部分则会查看接收端的抵达消息。在发射者或接收者端之一被弃用时，就算是信道被 *关闭，closed* 了。

下面，咱们将完成有着一个线程生成一些值并将这些值发送到信道，同时有另一线程将接收这些值并将其打印出来的这么一个程序。咱们将在线程间使用信道发送一些简单值，来演示这项特性。一旦咱们熟悉了这项技巧，那么就可以对任何需要相互通讯的线程，比如聊天系统，或其中有许多线程执行着某项计算的各个部分，并把这些部分发送到结果汇总线程的系统等中使用信道。

首先，在下面的清单 16-6 中，咱们将创建出一个信道而不使用他来完成任何事情。请注意由于 Rust 无法分辨出咱们打算通过该信道，发送何种类型的值，该代码尚不会编译。

文件名：`src/main.rs`

```rust
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
}
```

*清单 16-6：创建出一个信道，并将两端赋值给 `tx` 与 `rx`*

咱们使用 `mpsc::channel` 函数，创建了一个新的信道；`mpsc` 表示的是 *多生产者，单一消费者，multiple producer, single consumer*。简而言之，Rust 标准库实现信道的方式，表明信道可以有多个生成值的 *发送，sending* 端，但只有一个消费这些值的 *接收，receiving* 端。请设想有多条小溪，汇流到一条大河：那么从任何这些小溪，送下来的东西，都将在最后抵达一条河中。现在咱们将以单个的生产者开始，在令到这个示例工作起来时，就将添加多个生产者。

这个 `mpsc::channel` 函数返回的是一个元组，元组的首个元素为发送端 -- 发送器，the transmitter -- 而第二个元素就是接收端 -- 接收器，the receiver。两个缩写 `tx` 与 `rx`，传统上在许多领域，都相应地被用于表示 *transmitter* 与 *receiver*，因此咱们就把这两个变量，如此命名来表示两端。咱们使用了带有模式，a pattern （`(tx, rx)`）的一个 `let` 语句，对这个元组加以解构；在第 18 章中，咱们将讨论这种 `let` 遇见中模式的运用及解构问题。至于现在，请明白以这种方式使用 `let` 语句，是提取由 `mpsc::channel` 返回元组中那些部分的便捷方式。

下面就把其中的发射端，移入到一个生成线程，并让其发送一个字符串，从而生成线程就在与主线程通信了，如下清单 16-7 中所示。这就像是在河流上游投入一只小黄鸭，或是在一个线程发送了一条聊天消息给另一线程。

文件名：`src/main.rs`

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("你好");
        tx.send(val).unwrap();
    });
}
```

*清单 16-7：把 `tx` 迁移到一个生成线程并发送 "你好"*

又一次，咱们使用了 `thread::spawn` 创建出一个新线程，并使用 `move` 把 `tx` 迁移进到那个闭包，于是这个生成的线程，便拥有了 `tx`。该生成线程需要拥有发送器，才能够经由信道发送消息。发送器有着取咱们打算发送值的一个 `send` 方法。而这个 `send` 方法返回的是个 `Result<T, E>` 类型值，那么在接收器已被弃用，而无处发送值时，那么发送操作就将返回一个错误。在此示例中，咱们调用了 `unwrap` 来在出现错误时终止运行，panic in case of an error。而在真实应用中，咱们应予以恰当处理：请回到第 9 章，回顾那些那些适当的错误处理策略。

在下面的清单 16-8 中，咱们将自主线程中的接收器，获取到那个值。这就像是在河流尽头接收到小黄鸭，或是接收到一条聊天消息。

文件名：`src/main.rs`

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("你好");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println! ("收到：{}", received);
}
```

*清单 16-8：在主进程中接收值 “你好” 并将其打印出来*

接收器有着两个有用方法：`recv` 与 `try_recv`。这里使用的是 `recv`，是 *receive* 的简写，该方法将阻塞主线程执行，而等待直到某个值被下发到信道。一旦某个值被发出，那么 `recv` 就会在一个 `Result<T, E>` 中将其返回。在发射器关闭时，`recv` 就会返回一个错误，表明不会再有值到来。

`try_recv` 方法则不会阻塞，而相反会立即返回一个 `Result<T, E>`：在消息可用时的一个保存着消息的 `Ok` 值，同时此刻没有任何消息时的一个 `Err` 值。在该线程在等待消息时，有其他工作要完成的情况下，使用 `try_recv` 便是有用的：咱们可以编写出每隔一段时间就调用 `try_recv` 的循环，在有消息时处理消息，再次检查是否收到消息之前的空隙，完成一些其他工作。

这里使用 `recv` 是为了简化；在主线程中，除了等待消息之外并无其他工作要做，因此阻塞主线程是恰当的。

当咱们运行清单 16-8 中的代码时，就会看到该值在主线程中被打印出来：

```console
收到：你好
```

好极了！


### 信道与所有权的转移

**Channels and Ownship Transference**

由于所有权规则帮助咱们编写出安全、并行的代码，因此其在消息发送中起着至关重要作用。在并发式编程中，于咱们 Rust 程序通篇考虑所有权的好处，就在于这样可以防止错误。接下来就要完成一项实验，来展示信道与所有权，是怎样一起运作以阻止问题发生的：咱们将在生成线程中，把一个 `val` 值送到信道 *之后，after*，再尝试使用这个值。尝试编译清单 16-9 中的代码，来观察为何该代码是不被允许的：

文件名：`src/main.rs`

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("你好");
        tx.send(val).unwrap();
        println! ("val 为 {}", val);
    });

    let received = rx.recv().unwrap();
    println! ("收到：{}", received);
}
```

*清单 16-9：在咱们已将 `val` 发送到信道之后在尝试使用他*

在这里，咱们在经由 `tx.send` 已把 `val` 发出到信道之后，尝试打印出他。允许这样做将是个糟糕的主意：一旦该值已被发送到另一线程，那么在咱们尝试再度使用该值之前，发往的那个线程就可能修改或是弃用掉该值。而潜在地，另一线程的这些改动，就会由于不一致或不存在的数据，而造成错误或未预期结果。不过，在咱们尝试编译清单 16-9 中的代码时，Rust 会给到咱们一个报错：

```console
$ cargo run                                                                                ✔  
   Compiling mp_demo v0.1.0 (/home/peng/rust-lang/mp_demo)
error[E0382]: borrow of moved value: `val`
  --> src/main.rs:13:31
   |
11 |         let val = String::from("你好");
   |             --- move occurs because `val` has type `String`, which does not implement the `Copy` trait
12 |         tx.send(val).unwrap();
   |                 --- value moved here
13 |         println! ("val 为 {}", val);
   |                                ^^^ value borrowed here after move
   |
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0382`.
error: could not compile `mp_demo` due to previous error
```

咱们犯下的并发错误就造成一个编译时报错。其中的 `send` 函数取得了其参数的所有权，进而在那个值被迁移时，接收器便取得了他的所有权。这就阻拦了咱们在发送了该值后，无意中地再度使用该值；所有权系统会检查各方面都妥当无虞。

### 发送出多个值并观察接收器的等待

**Sending Multiple Values and Seeing the Receiver Waiting**

清单 16-8 中的代码编译并运行了，不过其并未清楚地给出，两个单独线程是怎样通过信道相互交流的。在下面清单 16-10 中，咱们已做出将证实清单 16-8 中代码有在并发运行的一些修订：生成线程现在将发出多条消息，并在每条消息之间暂停一秒钟。

文件名：`src/main.rs`

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec! [
            String::from("你好"),
            String::from("自"),
            String::from("此"),
            String::from("线程"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    for received in rx {
        println! ("收到：{}", received);
    }
}
```

*清单 16-10：发送多条消息，并在每次发送之间进行暂停*

这次，其中的生成线程，有着咱们打算发送到主线程的一个字符串矢量值。咱们对其迭代，而分别发送每条消息，同时通过使用 `500` 毫秒的一个 `Duration` 值，调用 `thread::sleep` 在每次消息发送间暂停。

在主线程中，咱们未再显式调用 `recv` 函数：相反，咱们将 `rx` 当作了迭代器。对于所接收到的每个值，咱们就将其打印出来。在信道被关闭时，迭代就结束了。

在运行清单 16-10 中的代码时，咱们就会看到下面每行之间有着 500ms 暂停的输出：

```console
收到：你好
收到：自
收到：此
收到：线程
```

由于咱们在主线程中的那个 `for` 循环中，并无任何暂停或延迟的代码，因此咱们就可以说，主线程是在等待接收来自生成线程的那些值。

### 通过克隆发射器创建出多个生产者

**Creating Multiple Producers by Cloning the Transmitter**

早前咱们曾提到，`mpsc` 是 *multiple producer, single consumer* 的首字母缩写。接下来就要就要运用上 `mpsc`，并将清单 16-10 中的代码，扩充为创建出均将一些值发送到同一接收器的多个线程。通过克隆发射器，咱们就可以这样做，如下清单 16-11 中所示：

文件名：`src/main.rs`

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec! [
            String::from("你好"),
            String::from("自"),
            String::from("此"),
            String::from("线程"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    thread::spawn(move || {
        let vals = vec! [
            String::from("给"),
            String::from("你"),
            String::from("一些别的"),
            String::from("消息"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    for received in rx {
        println! ("收到：{}", received);
    }
}
```

*清单 16-11：从多个生产者发出多条消息*

这次在创建出首个生成线程之前，咱们调用了发射器上的 `clone` 方法。这样做将给到咱们可传递给那首个生成线程的一个新发射器。咱们把原先的发射器，传递给了第二个生成线程。这样就给到了咱们两个线程，二者都把不同消息，发送到那一个的接收器。

在运行此代码时，咱们的输出看起来应像下面这样：

```console
收到：你好
收到：给
收到：自
收到：你
收到：此
收到：一些别的
收到：线程
收到：消息
```

根据咱们所在系统的不同，也可能会看到另外顺序的这些值。这种消息每次出现顺序的不一致，正是令到并发有趣而又有难度的地方。而若带上 `thread::sleep` 加以实验，即在两个不同线程中给到不同睡眠值，这时的每次运行，将更具不确定性，而每次运行都造成不同输出。

既然咱们已经看到了信道的工作原理，那么接下来就要看看一种方式迥异的并发了。


## 状态共用的并发

**Shared-State Concurrency**

消息传递是处理并发的一种很好方式，但其并非唯一的一种。另一种方式将是，多个线程访问同一共用数据。请重新考虑一下摘自 Go 语言文档的那句口号的这个部分：“勿要经由共用内存通信。do not communicate by sharing memory.”

那么经由共用内存的通信，又会是怎样的呢？另外，为何消息传递方式拥趸们，会警告不要使用内存共用方式呢？

在某种程度上，任何编程语言中的信道，均类似于单一所有权，因为一旦咱们把值传递到信道，那么就不应再使用那个值了。内存共用的并发，则像是多重所有权：多个线程均可在同一时间，访问同一内存位置。正如咱们在第 15 章中曾见到过的，那里的灵巧之中令到多重所有权可行，多重所有权会因为这些不同所有者需要管理，而增加复杂度。Rust 的类型系统与所有权规则，极大地助力了实现这样的管理正确无误。作为一个示例，接下来咱们就要看看作为共用内存的一种更常见并发原语，即所谓的互斥量，for an example, let's look at mutexes, one of the more common concurrency primitives for shared memory。


### 运用互斥量实现一个时间仅允许一个线程访问数据

**Using Mutexes to Allow Access to Data from One Thread at a Time**

*互斥，mutex* 是 *相互排斥，mutual exclusion* 的缩写，正如互斥量在任何给定时间，都只允许一个线程访问某个数据。要访问互斥量中的数据，线程就必须首先通过询问来获取到该互斥量的 *锁，lock*，表明其打算访问。所谓锁，则是保持着当前是谁（哪个线程）有着对该数据排他性访问的追踪，作为该互斥量一部分的一种数据结构，the lock is a data structure that is part of the mutex that keeps track of who currently has exclusive access to the data。因此，所谓互斥量，就被描述为经由这种加锁系统，而 *守护着，guarding* 其所保存着的数据。

由于咱们务必要记住以下两条规则，互斥量便有了难以运用的名声：

- 在使用数据之前，咱们必须尝试获取到锁；
- 在完成互斥量所保护数据的操作时，咱们必须解开该数据，以便其他线程能够获取到锁。

至于互斥量的真实世界比喻，请设想在仅有一只麦克风的会议上的一个小组讨论。那么在小组成员能发言之前，他们就不得不请求或表明，他们打算使用麦克风。在他们得到麦克风时，他们便可以想要讲多长时间便讲多长时间，并在随后吧麦克风，递给下一位要求发言的小组成员。在某名小组成员于用完麦克风，却忘记交出麦克风时，就没有人能发言了。在这个共用麦克风的管理出错时，这个小组就将不会如计划那样运作了！

互斥量的管理非常棘手，难以做到正确无误，这正是许多人热衷于信道的原因。但是，归功于 Rust 的类型系统与所有权规则，咱们就无法在互斥量的加锁与解锁上出错了。


### `Mutex<T>` 的 API

下面是如何使用互斥量的一个示例，接下来咱们就要如下面清单 16-12 中所给出的那样，通过在单一线程情形下使用互斥量开始：

文件名：`src/main.rs`

```rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println! ("m = {:?}", m);
}
```

*清单 16-12：为简化目的在单个线程情形下探讨 `Mutex<T>` 的 API*

与许多类型一样，咱们使用关联函数 `new` 创建出了一个 `Mutex<T>`。而为了访问这个互斥量内部的数据，咱们使用了 `lock` 方法来获取到锁。此调用将阻塞当前线程，从而在轮到咱们拥有锁之前，当前线程就无法完成任何工作。

若有另一持有着锁的线程已终止运行，那么到 `lock` 的调用就会失败。在那种情况下，就没人能获得锁了，因此咱们就选择了 `unwrap`，而在咱们陷入到那样的情形时，让这个线程终止运行。

在获取到锁后，咱们就可以对此示例中名为 `num` 的返回值，作为到互斥量内部数据的可变引用，而加以处理了。类型系统会确保咱们在使用 `m` 里的值前，获取到锁。`m` 的类型为 `Mutex<i32>`，而非 `i32`，因此咱们为了使用那个 `i32` 值， 就 *必须* 调用 `lock`。这是不能忘掉的；否则类型系统就不会让咱们访问那个内层的 `i32`。

正如咱们可能怀疑的那样，`Mutex<T>` 是个灵巧指针。更准确地讲，到 `lock` 的调用，*返回的是* 封装在咱们曾以到 `unwrap` 调用处理的 `LockResult` 中，一个叫做 `MutexGuard` 的灵巧指针。`MutexGuard` 灵巧之中实现了 `Deref`，来指向咱们的内层数据；这个灵巧指针还有着在 `MutexGuard` 超出作用域，即清单 16-12 的示例内存作用域结束处所发生时，自动释放锁的一个 `Drop` 实现。而其结果就是，由于锁的释放是自动发生的，因此咱们就不会面临，忘记释放锁而阻塞该互斥量为其他线程使用的风险。

在弃用了该所之后，咱们就可以打印出该互斥量的值，并看到咱们是能够把那个内层的 `i32`，修改为 `6` 的。


### <a id="sharing-a-mutex-t-between-multiple-threads"></a>在多个线程间共用 `Mutex<T>`

现在，咱们就来尝试使用 `Mutex<T>`，在多个线程见共用值。咱们将启动 10 个线程，并让他们分别都把一个计数器增加 `1`，因此那个计数器就会从 `0` 到达 `10`。接下来清单 16-13 中的示例，将有着一个编译器报错，同时咱们将使用那个报错，来掌握更多有关使用 `Mutex<T>`，以及 Rust 如何帮助咱们正确运用他的知识。

文件名：`src/main.rs`

```rust
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Mutex::new(0);
    let mut handles = vec! [];

    for _ in 0..10 {
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println! ("结果为：{}", *counter.lock().unwrap());
}
```

*清单 16-13：各自分别对由 `Mutex<T>` 所守护计数器递增的十个线程*

与在清单 16-12 中一样，咱们创建出了一个在 `Mutex<T>` 内，保存着一个 `i32` 的 `counter` 变量。接下来，咱们通过对数字范围的迭代，创建出了 10 个线程。咱们使用了 `thread::spawn`，并给到全部线程同样闭包：把那个计数器迁移进到线程，通过调用 `lock` 方法取得那个 `Mutex<T>` 上的锁，并于随后加 `1` 到该互斥量的值的这样一个闭包。在线程完成运行其闭包时，`num` 就会超出作用域而释放那把锁，从而另一线程便可以取得该锁。

在主线程中，咱们收集起了所有连接把手，collect all the join handles。随后，如同在清单 16-2 中所做的那样，咱们在各个把手上调用了 `join`，来确保所有现场运行完毕。在那个点位处，主线程将取得那把锁，并打印出该程序的结果。

咱们曾暗示过此示例不会编译。现在就来找出原因为何！

```console
$ cargo run                                                              ✔  
   Compiling mutex_demo v0.1.0 (/home/peng/rust-lang/mutex_demo)
error[E0382]: use of moved value: `counter`
  --> src/main.rs:12:36
   |
8  |     let counter = Mutex::new(0);
   |         ------- move occurs because `counter` has type `Mutex<i32>`, which does not implement the `Copy` trait
...
12 |         let handle = thread::spawn(move || {
   |                                    ^^^^^^^ value moved into closure here, in previous iteration of loop
13 |             let mut num = counter.lock().unwrap();
   |                           ------- use occurs due to use in closure

For more information about this error, try `rustc --explain E0382`.
error: could not compile `mutex_demo` due to previous error
```

这个报错消息指出，其中的 `counter` 值在循环的上一次迭代中已被迁移。Rust 正告诉咱们，不能将锁 `counter` 的所有权，迁移进到多个线程中。下面就来使用第 15 张中曾讨论过的多重所有权方式，修正这个编译器报错。

### 多线程下的多重所有权

**Multiple Ownership with Multiple Threads**

在第 15 章中，咱们曾通过使用灵巧指针 `Rc<T>`，来创建出一个引用计数的值，而将一个值赋予到多个所有者。下面就来完成那同样的操作，并看到会发生什么。咱们将在清单 16-14 中，把那个 `Mutex<T>` 封装在 `Rc<T>` 中，并在把所有权迁移到线程之前，克隆这个 `Rc<T>`。

文件名：`src/main.rs`

```rust
use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec! [];

    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println! ("结果为：{}", *counter.lock().unwrap());
}
```

*清单 16-14：尝试使用 `Rc<T>` 来实现多个线程拥有那个 `Mutex<T>`*

又一次，咱们编译并得到......一些不同的报错！编译器给了咱们很多指教。

```console
$ cargo run                                                                           ✔  
   Compiling mutex_demo v0.1.0 (/home/peng/rust-lang/mutex_demo)
error[E0277]: `Rc<Mutex<i32>>` cannot be sent between threads safely
  --> src/main.rs:14:36
   |
14 |           let handle = thread::spawn(move || {
   |                        ------------- ^------
   |                        |             |
   |  ______________________|_____________within this `[closure@src/main.rs:14:36: 14:43]`
   | |                      |
   | |                      required by a bound introduced by this call
15 | |             let mut num = counter.lock().unwrap();
16 | |
17 | |             *num += 1;
18 | |         });
   | |_________^ `Rc<Mutex<i32>>` cannot be sent between threads safely
   |
   = help: within `[closure@src/main.rs:14:36: 14:43]`, the trait `Send` is not implemented for `Rc<Mutex<i32>>`
note: required because it's used within this closure
  --> src/main.rs:14:36
   |
14 |         let handle = thread::spawn(move || {
   |                                    ^^^^^^^
note: required by a bound in `spawn`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `mutex_demo` due to previous error
```

喔，那报错消息真的非常罗嗦！而这些才是要关注的重要部分：`` `Rc<Mutex<i32>>` cannot be sent between threads safely ``。编译器还告诉咱们了其原因：`` the trait `Send` is not implemented for `Rc<Mutex<i32>>` ``。下一小节咱们就要讲到 `Send` 特质：他是确保咱们用到类型，是意图用于并发情形的特质之一。

不幸的是，`Rc<T>` 于跨线程的共用上是不安全的。在 `Rc<T>` 管理着引用计数时，他会增加每次到 `clone` 调用的计数，并在每个克隆被弃用时减去计数。但其并未使用任何并发原语，any concurrency primitives，来确保那些对该计数的改变，不被另一线程中断。这就会导致错误的计数 -- 进而会导致内存泄漏，或在咱们未完成值处理之前，该值就已被启用这样的一些微妙代码缺陷。咱们所需要的，是像极了 `Rc<T>`，但会令到引用计数以线程安全方式得以改变的一种类型。

> **注**：简单地说，与各种编程语言中的那些原生数据类型，primitive data types 一样，所谓并发原语，concurrency primitives，指的就是用于并发编程的一些基本设施，the basic facilities for concurrent programming，这样的说法，某种程度上是跨越某个语言家族（比如 C 语言家族）。
>
> 参考：[What-are-concurrency-primitives-"K Symbol"](https://qr.ae/prtpz6)

### `Arc<T>` 下的原子引用计数

**Atomic Reference Counting with `Arc<T>`**

幸运的是，`Arc<T>` *正是* 安全用于并发情形下的一个像是 `Rc<T>` 的类型。其中的 `a` 代表着 `原子，atomic`，表示其是一种 *原子的引用计数，atomically reference counted* 类型。原子类型是咱们不会在此详细讨论的一类额外并发原生类型：请参阅 [`std::sync::atomic` 的标准库文档](https://doc.rust-lang.org/std/sync/atomic/index.html)，了解更多细节。此刻，咱们只需要知道这些原子类型会像那些原生类型一样运作，只不过他们对于跨线程的共用是安全的。

到这里咱们可能想知道，为何全部原生类型不是原子的，以及为何标准库的那些类型，没有默认使用 `Arc<T>` 实现。原因就是线程安全自带了性能损失，而只有在咱们真的需要线程安全时，才会打算付出。在咱们只是在单线程里于一些值上执行操作时，若咱们的代码不必强制实现原子类型所提供的那些保证，那么这些代码就可以运行得快得多。

接下来回到那个示例：`Arc<T>` 与 `Rc<T>` 有着同样的 API，因此通过修改其中的 `use` 语句行、到 `new` 的调用，以及到 `clone` 的调用，咱们就可以修复那个程序。清单 16-15 中的代码最终将会编译及运行：

文件名：`src/main.rs`

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec! [];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println! ("结果为：{}", *counter.lock().unwrap());
}
```

*清单 16-15：为能够跨越多线程地共用所有权，而使用 `Arc<T>` 来封装那个 `Mutex<T>`*

此代码将打印出下面的内容：

```console
结果为：10
```

咱们就做到了！咱们从 `0` 计数到了 `10`，这或许看起来不是非常印象深刻，但他真的教会了咱们很多有关 `Mutex<T>` 与线程安全的东西。咱们也可以运用这个程序的架构，完成相比于增加计数器，一些更为复杂的操作。运用这种策略，咱们可把某项计算，划分为一些独立部分，将这些部分拆解为多个线程，并于随后使用 `Mutex<T>` 来让各个各个线程，使用其自己部分对最终结果加以更新。

请注意若咱们是在完成一些简单的数字运算，你们就有由 [标准库的 `std::sync::atomic` 模组](https://doc.rust-lang.org/std/sync/atomic/index.html) 所提供的，相较于 `Mutex<T>` 更简单的一些类型。这些类型提供到原生类型安全、并发、原子的访问。咱们为这个示例而选择带有原生类型的 `Mutex<T>`，目的是可以着重于 `Mutex<T>` 的工作原理。


### `RefCell<T>`/`Rc<T>` 与 `Mutex<T>`/`Arc<T>` 二者之间的相似点

**Similarities Between `RefCell<T>`/`Rc<T>` and `Mutex<T>`/`Arc<T>`**

咱们或许已经留意到，其中那个 `counter` 是不可变的，但咱们却能获取到其内部值的可变引用；这意味着与 `Cell` 家族，the `Cell` family 所做的一样， `Mutex<T>` 提供了内部可变性。与咱们在第 15 章中曾使用 `RefCell<T>` 来实现修改 `Rc<T>` 内部内容同样的方式，咱们使用了 `Mutex<T>` 来修改 `Arc<T>` 内部内容。

另一个需要注意的细节，便是在咱们使用 `Mutex<T>` 时，Rust 无法保护咱们免于全部类别的逻辑错误。回顾在第 15 章中，`Rc<T>` 运用就伴随着创建出循环引用风险，其中两个 `Rc<T>` 值相互指向，导致内存泄漏。与此类似，`Mutex<T>` 则附带着创建出 *死锁，deadlocks* 的风险。在某个操作需要锁住两项资源，同时两个线程分别均已请求获取两把锁中的一把时，就造成他们一直等待着对方释放各自所需的锁。若对死锁方面感兴趣，那么请尝试创建出有着死锁的一个 Rust 程序；随后就要研究任何一门语言中，互斥量的死锁消除策略，并试试在 Rust 中实现这些策略。`Mutex<T>` 和 `MutexGuard` 的标准库 API 文档，就提供了一些有用信息。

咱们将通过讲解 `Send` 与 `Sync` 两个特质，以及怎样与一些定制类型来运用他们来完结本章。


## <a id="extensible-concurrency-with-the-sync-and-send-trait"></a>`Sync` 与 `Send` 两个特质下的可扩展并发

**Extensible Concurrency with the `Sync` and `Send` Traits**


有趣的是，Rust 语言并发方面的特性 *非常* 少。本章中到目前为止咱们讲到过的每种并发特性，都已是标准库而非语言本身的一部分。用于处理并发问题的选项，并不局限于这门语言或标准库；咱们可以编写自己的并发特性，或可以使用由其他人编写的并发特性。

不过，在这门语言中，是嵌入了两个并发概念的：即 `std::marker` 特质 `Sync` 与 `Send`。


### 使用 `Send` 特质实现线程间所有权转移

**Allowing Transference of Ownership Between Threads with `Send`**


这个 `Send` 标识符特质，表示实现 `Send` 类型值的所有权，可以在线程间转移。几乎全部 Rust 类型都是 `Send` 类型，但有一些例外，包括 `Rc<T>`：由于在咱们克隆了某个 `Rc<T>` 并尝试将这份克隆的所有权，转移到另一线程时，两个现场可能在同一时间更新引用计数，因此 `Rc<T>` 就不能是 `Send` 类型。由于这个原因，`Rc<T>` 正是为其间咱们不打算付出线程安全方面性能开销的那些单线程情形，而实现的。

由此，Rust 的类型系统与特质边界，type system and trait bounds，就确保了咱们绝不会意外地将某个 `Rc<T>`，不安全地跨越线程发送。当咱们在清单 16-14 中尝试这样做时，咱们就曾得到编译器报错 `` the trait `Send` is not implemented for `Rc<Mutex<i32>>` ``。而在咱们切换到 `Arc<T>` 这种 `Send` 类型时，那段代码就编译了。

由全部 `Send` 类型所组成的类型，也会被自动标记为 `Send` 类型。除开那些原始指针，raw pointers 外，那么可以说几乎全部原生类型都是 `Send` 的，咱们将在第 19 章中，讲到那些原始指针。


### 使用 `Sync` 实现来自多个线程的访问

**Allowing Access from Multiple Threads with `Sync`**

`Sync` 标识符表示实现 `Sync` 特质的类型，其被从多个线程引用是安全的。换句话说，任何类型 `T` 在 `&T` （即到 `T` 的不可变引用） 为 `Send` 的时，那么其即为 `Sync` 的，表示该引用可以安全地发送到另一线程。与 `Send` 类似，原生类型均为 `Sync` 的，且由全部都是 `Sync` 的类型所组成的类型，也都是 `Sync` 的。

灵巧指针 `Rc<T>` 因为其不是 `Send` 的同样原因，其也不是 `Sync` 的。`RefCell<T>` 类型（咱们曾在第 15 章讲过）以及相关的 `Cell<T>` 类型家族，都不是 `Sync` 的。`RefCell<T>` 在运行时所完成的借用检查实现，不是线程安全的。灵巧指针 `Mutex<T>` 是 `Sync` 的，并正如咱们在 [于多个线程间共用 `Mutex<T>`](#sharing-a-mutex-t-between-multiple-threads) 小节中看到的，其可被用于多个线程下共用访问。


### 手动实现 `Send` 与 `Sync` 是不安全的

**Implementing `Send` and `Sync` Manually Is Unsafe**


由于 `Send` 与 `Sync` 特质构成的类型自动也是 `Send` 与 `Sync` 的，因此咱们大可不必手动实现这两个特质。而作为标记性特质，二者甚至都没有任何要实现的方法。他们只是在执行与并发性有关的不变性方面很有用。

手动实现这两个特质，涉及到实现一些不安全 Rust 代码，unsafe Rust code。在第 19 章咱们将讲到运用不安全 Rust 代码；至于现在，要点在于构造不是由一些 `Send` 与 `Sync` 部分组成的新并发类型，需要深思熟虑来维持那些安全保证。[The Rustonomicon](https://doc.rust-lang.org/nomicon/index.html) 有着这些保证的更多信息，以及维持这些保证的方式。


## 本章小节

这不会是你在本书中将见到并发的最后一章：第 20 张中的那个项目，就将在相比于这里所讨论过较小示例，而更具现实意义的情形下用到本章中的那些概念。

正如早先所提到的，由于只有极少量的 Rust 处理并发方式，属于这门语言的一部分，因此许多并发解决方案，都是作为代码箱实现的。这些方案相比标准库进化更为迅速，那么就要确保在线搜寻当前的、最前沿代码箱，来用于多线程情形中。

Rust 标准库提供了用于消息传递的信道，以及诸如 `Mutex<T>` 与 `Arc<T>` 等安全用于并发情景中的一些灵巧指针类型。类型系统与借用检查器，会确保应用了这些方案的代码，不会以数据竞争或无效引用结束。一旦让代码编译了，咱们就可以放下心来，代码将愉快地运行于多线程之上，而不会有在其他语言中常见的那些难于追踪的问题。并发编程自此不再是令人害怕的概念：去吧，让你的程序并发起来，无所畏惧！

接下来，咱们将讲到，随着咱们的 Rust 程序变得大了起来，建模问题与架构出方案的一些管用做法。此外，咱们将讨论 Rust 的一些习惯说法，这些说法可能与面向对象编程中所熟悉的有关。
