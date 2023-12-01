# 使用消息传递来在线程间传输数据

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


## 信道与所有权的转移

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


## 发送出多个值并观察接收器的等待

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


## 通过克隆发射器创建出多个生产者

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
