# 通过消息传递在线程间传输数据

的确保安全并发的一种日益流行的方法，便是 *消息传递*，其中线程或参与者通过相互发送包含数据的消息来进行通信。下面是 [Go 语言文档](https://golang.org/doc/effective_go.html#concurrency) 中的一句口号概括了这一理念：“不要通过共享内存来通信；相反，通过通信来共享内存。”

为了实现消息发送式的并发，Rust 标准库提供了一种信道的实现。所谓 *信道*，属于一种通用的编成概念，数据通过信道从一个线程发送到另一线程。

咱们可以把编程中的信道，想象为有方向的水渠，比如小溪或河流。当咱们把塑胶小黄鸭之类的东西投入河中时，他就会顺流而下，到达该水道的尽头。

信道有两个部分：发送端和接收端。发送端在上游位置，即咱们把小黄鸭投入河中的地方；接收端是小黄鸭顺流而下最终到达的位置。咱们代码的一部分以咱们打算发送的数据调用发送端的方法，另一部分则检查接收端是否有到达的消息。当发射端或接收端之一被弃用时，则称信道 *关闭*。

在这里，我们将逐步实现一个程序，他有一个线程生成值并发送这些值到信道，而另一线程将接收这些值并打印出他们。我们将使用信道在线程之间发送简单值，以此演示这一特性。一旦咱们熟悉了这项技巧，咱们就可以针对任何需要相互通讯的线程使用信道，比如聊天系统，或由许多线程分别执行计算的各个部分，并发送结果到一个负责汇总结果的线程的系统。

首先，在下面的清单 16-6 中，我们将创建出一个信道，但暂不对其执行任何操作。请注意，这还不会编译，因为 Rust 无法判断我们打算通过信道发送何种类型的值。

<a name="listing_16-6"></a>
文件名：`src/main.rs`

```rust
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
}
```

**清单 16-6**：创建信道，并指派两端给 `tx` 和 `rx`

我们使用 `mpsc::channel` 函数创建一个新的信道；`mpsc` 代表 *multiple producer, single consumer*。简而言之，Rust 标准库实现信道的方式，意味着信道可以有多个产生值的 *发送* 端，却只能有一个消费这些值的 *接收* 端。请设想有多条小溪汇聚成一条大河：从任何一条小溪送出的内容最终都会汇入同一条大河。我们现在将以单个生产者开始，但当我们让这个示例正常工作后，我们将添加多个生产者。

`mpsc::channel` 函数返回一个元组，其中第一个元素是发送端 -- 发送器 -- 第二个元素是接收端 -- 接收器。两个缩写 `tx` 与 `rx`，传统上在许多领域都分别用于 *transmitter* 和 *receiver*，因此我们这样命名变量来表示两端。我们以一种解构元组的模式使用了 `let` 语句；我们将在第 19 章中讨论在 `let` 语句中模式的使用和解构。目前只需知道，以这种方式使用 `let` 语句，是提取 `mpsc::channel` 返回的元组中各部分的便捷方式。

我们来迁移发送端到一个生成的线程中，并让他发送一个字符串，从而使生成的线程与主线程通信，如下清单 16-7 中所示。这就像在河的上游投入一只小黄鸭，或者像从一个线程发送聊天消息给另一线程。

<a name="listing_16-7"></a>
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

**清单 16-7**：迁移 `tx` 到一个生成线程，并发送 `"你好"`

同样，我们使用 `thread::spawn` 创建一个新线程，然后使用 `move` 把迁移 `tx` 到闭包中，以便生成的线程拥有 `tx`（译注：`rx` 为什么不会迁移到闭包中？）。生成的线程需要拥有发送器才能通过信道发送消息。

发送器有个 `send` 方法，取我们打算发送的值。`send` 方法返回一个 `Result<T, E>` 类型，因此当接收器已被弃用而没有地方发送值时，发送操作将返回错误。在这个示例中，我们调用 `unwrap` 来在发生错误的情形下终止运行。但在实际应用中，我们会妥善处理错误：请返回第 9 章，复习正确的错误处理策略。

在下面清单 16-8 中，我们将在主线程中获取接收器中的值。这就像是在河流尽头捞起水中的小黄鸭，或者接收一条聊天消息。

<a name="listing_16-8"></a>
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
    println! ("收到：{received}");
}
```

**清单 16-8**：在主线程中接收值 `"你好"` 并打印

接收器有两个有用的方法：`recv` 和 `try_recv`。我们使用了 `recv`，即 *receive* 的简写，他将阻塞主线程的执行并等待，直到有值下发到信道。一旦有值发出，`recv` 将在 `Result<T, E>` 中返回他。当发射器关闭时，`recv` 将返回一个错误，表明不再有值传入。

`try_recv` 方法不会阻塞，而是会立即返回一个 `Result<T, E>`：当有可用消息时，则返回一个包含消息的 `Ok` 值；当此时没有任何消息时，则返回一个 `Err` 值。若该线程在等待消息时还有其他工作要做，则使用 `try_recv` 非常有用：我们可以编写一个循环，每隔一段时间调用 `try_recv`，在有消息时处理消息，否则先执行其他工作一段时间，然后再次检查有无消息。

出于简化原因，我们在这个实例中使用了 `recv`；除了等待消息之外，我们在主线程中并无其他工作要做，因此阻塞主线程是合适的。

当我们运行清单 16-8 中的代码时，我们将看到该值在主线程中打印：

```console
收到：你好
```

太棒了！


## 通过信道转移所有权

所有权规则在消息发送中起着至关重要作用，因为他们帮助咱们编写安全、并发的代码。关注整个 Rust 程序的所有权的优势在于，可以防止并发编程中的错误。我们来完成一个实验，展示信道与所有权怎样一起协同工作以防止问题：我们将尝试在下发 `val` 值到信道 *之后*，再在生成的线程中使用该值。请尝试编译下面清单 16-9 中的代码，了解为何这段代码不被允许：

<a name="listing_16-9"></a>
文件名：`src/main.rs`

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("你好");
        tx.send(val).unwrap();
        println! ("val 为 {val}");
    });

    let received = rx.recv().unwrap();
    println! ("收到：{}", received);
}
```

**清单 16-9**：在我们已下发 `val` 到信道后，再尝试使用他

在这里，我们在通过 `tx.send` 下发 `val` 到信道后，尝试打印他。允许这样做将是个糟糕的主意：一旦值被发送到另一线程，该线程就可以在我们尝试再次使用该值之前，修改或弃用他。由于数据不一致或不存在，另一线程的修改可能会导致错误或未预期的结果。不过，当我们尝试编译清单 16-9 中的代码时，Rust 会给予咱们一个报错：

```console
$ cargo run
   Compiling message-passing v0.1.0 (/home/hector/rust-lang-zh_CN/projects/message-passing)
error[E0382]: borrow of moved value: `val`
  --> src/main.rs:10:27
   |
 8 |         let val = String::from("你好");
   |             --- move occurs because `val` has type `String`, which does not implement the `Copy` trait
 9 |         tx.send(val).unwrap();
   |                 --- value moved here
10 |         println! ("val 为 {val}");
   |                            ^^^ value borrowed here after move
   |
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0382`.
error: could not compile `message-passing` (bin "message-passing") due to 1 previous error
```

我们的并发错误导致了一个编译时错误。`send` 函数会取得其参数的所有权，而在值被迁移后，接收器会取得他的所有权。这可以防止我们在发送值后意外地再次使用该值；所有权系统会检查一切是否正常。


## 发送多个值

[清单 16-8](#listing_16-8) 中的代码虽然编译并运行了，但并未清晰地向我们展示两个独立线程正在通过信道相互通信。

在下面清单 16-10 中，我们进行了一些修改，以证明清单 16-8 中代码是并发运行的：生成的线程现在将发送多条消息，并在每条消息之间暂停一秒钟。

<a name="listing_16-10"></a>
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
            String::from("从"),
            String::from("这个"),
            String::from("线程"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println! ("收到：{received}");
    }
}
```

**清单 16-10**：发送多条消息并在每次发送之间暂停

这次，生成的线程有个我们打算发送给主线程的字符串矢量值。我们遍历他们、单独发送每个字符串，并以一秒的 `Duration` 值调用 `thread::sleep`，在每次发送间暂停。

在主线程中，我们不再显式调用 `recv` 函数：相反，我们视 `rx` 为一个迭代器。对于接收到的每个值，我们都打印他。当信道关闭时，迭代将结束。

运行清单 16-10 中的代码时，咱们应看到以下输出，每行之间有一秒的暂停：

```console
$ cargo run
   Compiling message-passing v0.1.0 (/home/hector/rust-lang-zh_CN/projects/message-passing)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/message-passing`
收到：你好
收到：从
收到：这个
收到：线程
```

由于我们在主线程中的 `for` 循环中没有任何暂停或延迟的代码，我们可以判断主线程正在等待接收生成的线程中的值。


## 创建多个生产者

早前咱们曾提到，`mpsc` 是 *multiple producer, single consumer* 的首字母缩写。我们来真正使用 `mpsc`，并扩展清单 16-10 中的代码为创建多个线程，都发送值到同一个接收器。我们可以通过克隆发射器实现这点，如下清单 16-11 中所示。

<a name="listing_16-11"></a>
文件名：`src/main.rs`

```rust
    // -- 跳过代码 --
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec! [
            String::from("你好"),
            String::from("从"),
            String::from("这个"),
            String::from("线程"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
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
        println! ("收到：{received}");
    }
    // -- 跳过代码 --
```

**清单 16-11**：从多个生产者发送多条消息

这次，在创建第一个生成的线程之前，我们县对发送器调用 `clone` 方法。这将给予我们一个新的发送器，我们可以传递给第一个生成的线程。我们传递原始发送器给第二个生成的线程。这给予我们两个线程，二者发送不同消息给一个接收器。

在咱们运行这段代码时，咱们的输出看起来应像下面这样：

```console
$ cargo run
   Compiling message-passing v0.1.0 (/home/hector/rust-lang-zh_CN/projects/message-passing)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/message-passing`
收到：你好
收到：给
收到：你
收到：从
收到：一些别的
收到：消息
收到：这个
收到：线程
```

根据咱们的系统的不同，咱们可能会看到别的顺序的这些值。这正是使得并发既有趣而又困难的原因所在。当咱们以 `thread::sleep` 试验时，那么在不同线程中给予他不同的值，那么每次运行将更具不确定性，并且每次都会产生不同的输出。

现在我们已经了解了信道的工作原理，我们来看看另一种不同的并发方法。


（End）


