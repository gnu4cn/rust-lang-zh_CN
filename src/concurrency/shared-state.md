# 共用状态的并发

消息传递是处理并发的一种不错的方式，但并非唯一的方式。另一种方法是让多个线程访问同一共用的数据。请再考虑一下 Go 语言文档中口号的这一部分：“不要通过共用内存通信。”

通过共用内存的通信会是什么样子？此外，为何消息传递的拥护者会告诫不要使用共用内存？

在某种程度上，任何编程语言中的信道都类似于单一所有权，因为一旦咱们传输值到信道中，咱们就不应再使用该值。共用内存的并发就像多重所有权：多个线程可以同时访问同一内存位置。正如咱们在第 15 章中看到的，虽然灵巧指针使多重所有权可行，但多重所有权会增加复杂性，因为这些不同的所有者需要管理。Rust 的类型系统和所有权规则，极大地助力了实现这种管理正确无虞。举个例子，我们来看看互斥量，这是共用内存的较为常见的并发原语之一。


## 以互斥量控制访问

所谓 *互斥量，mutex*，是 *mutual exclusion，相互排斥* 的缩写，因为在互斥量下，在任何给定时间都只允许一个线程访问某一数据。要访问互斥量中的数据，线程就必须首先通过请求获取互斥量的锁来表明其访问意图。所谓 *锁*，属于一种数据结构，是互斥量的一部分，跟踪当前谁有着对数据的独占访问权。因此，互斥量就被描述为通过锁定机制 *保护* 其包含的数据。

互斥量素来以难以使用著称，因为咱们必须记住以下两条规则：

1. 在使用数据之前，咱们必须尝试获取锁；
2. 在咱们使用完互斥量保护的数据后，必须解锁数据，以便其他线程可以获到锁。

对于互斥量的现实世界比喻，请设想在仅有一只麦克风的会议上的小组讨论。在小组成员发言之前，他们必须请求或示意希望使用麦克风。再他们拿到麦克风后，他们可以随意发言，然后把麦克风交给下一位要求发言的小组成员。当某名小组成员用完麦克风后忘记交出麦克风时，其他人就不能发言。当这只共用麦克风的管理出现问题时，小组讨论将无法按计划进行！

正确管理互斥量可能非常棘手，这也正是如何这么多人对信道如此推崇的原因。然而，得益于 Rust 的类型系统与所有权规则，咱们不会让加锁和解锁出错。


### `Mutex<T>` 的 API


作为使用互斥量的一个示例，我们来以在单线程的上下文中使用互斥量开始，如下清单 16-12 中所示。

<a name="listing_16-12"></a>
文件名：`src/main.rs`

```rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println! ("m = {m:?}");
}
```

**清单 16-12**：出于简化目，在单线程上下文中探讨 `Mutex<T>` 的 API

与许多类型一样，我们使用关联函数 `new` 创建 `Mutex<T>` 值。要访问互斥量内的数据，我们使用 `lock` 方法获取锁。这一调用将阻塞当前线程，使其在轮到我们获得锁之前无法执行任何操作。

当持有锁的另一线程终止运行时，则对 `lock` 的调用将会失败。在这种情况下，没人能够获得锁，因此我们选择在遇到这种情况时 `unwrap`，让这个线程终止运行。

获取锁后，我们可以视返回值为到内部数据的可变引用，在这一情形下名为 `num`。类型系统确保我们在使用 `m` 中的值前先获取到锁。`m` 的类型为 `Mutex<i32>`，而不是 `i32`，因此我们 *必须* 调用 `lock` 才能使用 `i32` 值。我们不能忘掉这点；否则类型系统不会让我们访问内层的 `i32` 值。

到 `lock` 的调用返回一个名为 `MutexGuard` 的类型，封装在我们通过调用  `unwrap` 处理的 `LockResult` 中。`MutexGuard` 实现了 `Deref` 特质，以指向内层数据；这一类型还有着 `Drop` 的实现，当 `MutexGuard` 超出作用域时，即发生于内层作用域结束处，该实现会自动释放锁。因此，我们不会面临忘记释放锁而阻塞互斥量被其他线程使用的风险，因为锁释放会自动发生。

在弃用锁后，我们可以打印互斥量值，并看到我们能够修改内层的 `i32` 为 `6`。


### 共用对 `Mutex<T>` 的访问

现在，我们来尝试使用 `Mutex<T>` 在多个线程间共用值。我们将启动 10 个线程，并让他们每个都递增计数器值加 1，因此计数器会从 0 增加到 10。下面清单 16-13 中的示例将有个编译器报错，而我们将使用该报错进一步了解有关使用 `Mutex<T>` 的更多信息，以及 Rust 如何帮助我们正确使用他。

<a name="listing_16-13"></a>
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

    println! ("结果：{}", *counter.lock().unwrap());
}
```

**清单 16-13**：十个线程，每个线程都递增一个由 `Mutex<T>` 保护的计数器

我们像在清单 16-12 中所做的那样，创建 `counter` 变量来保存 `Mutex<T>` 内的 `i32` 值。接下来，我们通过遍历一个数字范围来创建 10 个线程。我们使用 `thread::spawn`，并给予全部线程同样的闭包：迁移计数器到线程中，通过调用 `lock` 方法取得该 `Mutex<T>` 上的锁，然后加 `1` 到互斥量中的值。在某一线程完成运行其闭包后，`num` 将超出作用域而释放锁，以便另一线程可以获取该锁。

在主线程中，我们收集所有连接句柄。然后，如同我们在清单 16-2 中所做的那样，我们对每个句柄调用 `join`，以确保所有线程都完成。此时，主线程获取锁并打印这个程序的结果。

我们曾暗示这个示例不会编译。现在我们来找出原因！

```console
$ cargo run
   Compiling shared-state v0.1.0 (/home/hector/rust-lang-zh_CN/projects/shared-state)
error[E0382]: borrow of moved value: `counter`
  --> src/main.rs:21:25
   |
 5 |     let counter = Mutex::new(0);
   |         ------- move occurs because `counter` has type `std::sync::Mutex<i32>`, which does not implement the `Copy` trait
...
 8 |     for _ in 0..10 {
   |     -------------- inside of this loop
 9 |         let handle = thread::spawn(move || {
   |                                    ------- value moved into closure here, in previous iteration of loop
...
21 |     println! ("结果：{}", *counter.lock().unwrap());
   |                            ^^^^^^^ value borrowed here after move
   |
help: consider moving the expression out of the loop so it is only moved once
   |
 8 ~     let mut value = counter.lock();
 9 ~     for _ in 0..10 {
10 |         let handle = thread::spawn(move || {
11 ~             let mut num = value.unwrap();
   |

For more information about this error, try `rustc --explain E0382`.
error: could not compile `shared-state` (bin "shared-state") due to 1 previous error
```

报错消息指出，`counter` 这个值在循环的上一次迭代中已被迁移。Rust 是在告诉我们，不能迁移 `counter` 的锁的所有权到多个线程中。我们来通过我们在第 15 章中讨论的多重所有权方法来修复这个编译器报错。


### 多线程下的多重所有权

在第 15 章中，我们通过使用灵巧指针 `Rc<T>` 创建引用计数的值，而将值赋予多个所有者。我们来在这里执行同样的操作，看看会发生什么。我们将在下面清单 16-14 中，将 `Mutex<T>` 封装在 `Rc<T>` 中，并在迁移所有权到线程之前克隆该 `Rc<T>`。

<a name="listing_16-14"></a>
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

**清单 16-14**：尝试使用 `Rc<T>` 来允许多个线程拥有 `Mutex<T>`

再次，我们得到并得到......不同的报错！编译器教会了我们很多东西：

```console
$ cargo run
   Compiling shared-state v0.1.0 (/home/hector/rust-lang-zh_CN/projects/shared-state)
error[E0277]: `Rc<std::sync::Mutex<i32>>` cannot be sent between threads safely
  --> src/main.rs:11:36
   |
11 |           let handle = thread::spawn(move || {
   |                        ------------- ^------
   |                        |             |
   |  ______________________|_____________within this `{closure@src/main.rs:11:36: 11:43}`
   | |                      |
   | |                      required by a bound introduced by this call
12 | |             let mut num = counter.lock().unwrap();
13 | |
14 | |             *num += 1;
15 | |         });
   | |_________^ `Rc<std::sync::Mutex<i32>>` cannot be sent between threads safely
   |
   = help: within `{closure@src/main.rs:11:36: 11:43}`, the trait `Send` is not implemented for `Rc<std::sync::Mutex<i32>>`
note: required because it's used within this closure
  --> src/main.rs:11:36
   |
11 |         let handle = thread::spawn(move || {
   |                                    ^^^^^^^
note: required by a bound in `spawn`
  --> /rustc/01f6ddf7588f42ae2d7eb0a2f21d44e8e96674cf/library/std/src/thread/functions.rs:125:1

For more information about this error, try `rustc --explain E0277`.
error: could not compile `shared-state` (bin "shared-state") due to 1 previous error
```

哇，这条错误消息太啰嗦了！这才是要关注的重要部分：`` `Rc<Mutex<i32>>` cannot be sent between threads safely ``。编译器还告诉咱们原因：`` the trait `Send` is not implemented for `Rc<Mutex<i32>>` ``。我们将在下一小节讨论 `Send` 特质：他是确保我们在线程下使用的类型，适合在并发情形下使用的特质之一。

遗憾的是，`Rc<T>` 无法安全地跨线程共用。在 `Rc<T>` 管理引用计数时，他会增加每次 `clone` 调用的计数，并在每个克隆被弃用时减少计数。但他并未使用任何并发原语来确保对计数的更会不会被其他线程中断。这会导致错误的计数 -- 这些隐蔽的 bug 进而可能导致内存泄漏，或导致某个值在我们未使用完毕前被弃用。我们需要的是一种与 `Rc<T>` 完全相同，但能以线程安全的方式修改引用计数的类型。

> **译注**：简单地说，与各种编程语言中的那些原生数据类型，primitive data types 一样，所谓并发原语，concurrency primitives，指的就是用于并发编程的一些基本设施，the basic facilities for concurrent programming，这样的说法，某种程度上是跨越某个语言家族（比如 C 语言家族）的。
>
> 参考：[What-are-concurrency-primitives-"K Symbol"](https://qr.ae/prtpz6)


### `Arc<T>` 下的原子引用计数

幸运的是，`Arc<T>` *正是* 一种类似于 `Rc<T>` 的类型，可以在并发情形下安全使用。其中 *a* 代表 *atomic*，表示他是一种 *原子引用计数* 类型。所谓原子，是另一种并发原语，我们不会在这里详细探讨：请参阅 [`std::sync::atomic` 的标准库文档](https://doc.rust-lang.org/std/sync/atomic/index.html) 了解更多细节。目前，咱们只需要知道原子类型的工作原理与基元类型相似，但可以安全地跨线程共用。

然后咱们可能想知道，为什么所有基元类型都不是原子的，以及为什么标准库默认实现为使用 `Arc<T>`。原因是线程安全会带来性能损失，而只有在咱们真的需要时才愿意付出。当咱们只是在单线程内对值执行操作时，若代码不必强制执行原子类型提供的保证，代码就会运行得更快。

我们来回到我们的示例：`Arc<T>` 与 `Rc<T>` 有着同样的 API，因此我们通过修改 `use` 行、对 `new` 的调用，以及对 `clone` 的调用来修复程序。下面清单 16-15 中的代码最终将编译并运行。

<a name="listing_16-15"></a>
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

    println! ("结果：{}", *counter.lock().unwrap());
}
```

**清单 16-15**：使用 `Arc<T>` 封装 `Mutex<T>`，以便能够跨越多个线程共用所有权

这段代码将打印以下内容：

```console
结果：10
```

我们做到了！我们从 0 计数到了 10，这看起来可能并不太令人印象深刻，但他确实教会了我们很多有关 `Mutex<T>` 和线程安全的知识。咱们也可以使用这个程序的架构，完成比单纯递增计数器更复杂的操作。运用这一策略，咱们可以分解计算为独立的部分，跨线程拆分这些部分，然后使用 `Mutex<T>` 让每个线程以其结果更新最终结果。

请注意，当咱们正在进行简单的数字运算时，有一些 [标准库的 `std::sync::atomic` 模组](https://doc.rust-lang.org/std/sync/atomic/index.html) 提供的比 `Mutex<T>` 更简单的类型。这些类型提供对原始类型的安全、并发、原子的访问。对于这个示例我们选择与原生类型一起使用 `Mutex<T>`，以便专注于 `Mutex<T>` 的工作原理。


## 比较 `RefCell<T>`/`Rc<T>` 与 `Mutex<T>`/`Arc<T>`

咱们可能已经注意到，`counter` 是不可变的，但我们可以获得到其内部值的可变引用；这意味着 `Mutex<T>` 提供了内部可变性，就像 `Cell` 家族一样。就像我们在第 15 章使用 `RefCell<T>` 来允许我们修改 `Rc<T>` 内的内容一样，我们使用 `Mutex<T>` 来修改 `Arc<T>` 内的内容。

另一个要注意的细节是，当咱们使用 `Mutex<T>` 时，Rust 无法保护咱们免受所有类别的逻辑错误的影响。回顾在第 15 章中，使用 `Rc<T>` 会带来创建引用环的风险，其中两个 `Rc<T>` 值相互相引用，从而导致内存泄漏。同样，`Mutex<T>` 会带来创建 *死锁* 的风险。当某一操作需要锁住两个资源，且两个线程分别获取了其中一个锁，导致他们一直互相等待时，就会发生这种情况。若咱们对死锁感兴趣，请尝试创建一个有着死锁的 Rust 程序；然后，研究任意一门语言中针对互斥量的死锁缓解策略，并尝试在 Rust 中实现他们。`Mutex<T>` 和 `MutexGuard` 的标准库 API 文档提供了有用的信息。

我们将通过讨论 `Send` 与 `Sync` 两个特质，以及怎样与定制类型一起使用他们来结束本章。


（End）


