# 状态共用的并发

**Shared-State Concurrency**


消息传递是处理并发的一种很好方式，但其并非唯一的一种。另一种方式将是，多个线程访问同一共用数据。请重新考虑一下摘自 Go 语言文档的那句口号的这个部分：“勿要经由共用内存通信。do not communicate by sharing memory.”

那么经由共用内存的通信，又会是怎样的呢？另外，为何消息传递方式拥趸们，会警告不要使用内存共用方式呢？

在某种程度上，任何编程语言中的信道，均类似于单一所有权，因为一旦咱们把值传递到信道，那么就不应再使用那个值了。内存共用的并发，则像是多重所有权：多个线程均可在同一时间，访问同一内存位置。正如咱们在第 15 章中曾见到过的，那里的灵巧之中令到多重所有权可行，多重所有权会因为这些不同所有者需要管理，而增加复杂度。Rust 的类型系统与所有权规则，极大地助力了实现这样的管理正确无误。作为一个示例，接下来咱们就要看看作为共用内存的一种更常见并发原语，即所谓的互斥量，for an example, let's look at mutexes, one of the more common concurrency primitives for shared memory。


## 运用互斥量实现一个时间仅允许一个线程访问数据

**Using Mutexes to Allow Access to Data from One Thread at a Time**


*互斥，mutex* 是 *相互排斥，mutual exclusion* 的缩写，正如互斥量在任何给定时间，都只允许一个线程访问某个数据。要访问互斥量中的数据，线程就必须首先通过询问来获取到该互斥量的 *锁，lock*，表明其打算访问。所谓锁，则是保持着当前是谁（哪个线程）有着对该数据排他性访问的追踪，作为该互斥量一部分的一种数据结构，the lock is a data structure that is part of the mutex that keeps track of who currently has exclusive access to the data。因此，所谓互斥量，就被描述为经由这种加锁系统，而 *守护着，guarding* 其所保存着的数据。

由于咱们务必要记住以下两条规则，互斥量便有了难以运用的名声：

- 在使用数据之前，咱们必须尝试获取到锁；
- 在完成互斥量所保护数据的操作时，咱们必须解开该数据，以便其他线程能够获取到锁。

至于互斥量的真实世界比喻，请设想在仅有一只麦克风的会议上的一个小组讨论。那么在小组成员能发言之前，他们就不得不请求或表明，他们打算使用麦克风。在他们得到麦克风时，他们便可以想要讲多长时间便讲多长时间，并在随后吧麦克风，递给下一位要求发言的小组成员。在某名小组成员于用完麦克风，却忘记交出麦克风时，就没有人能发言了。在这个共用麦克风的管理出错时，这个小组就将不会如计划那样运作了！

互斥量的管理非常棘手，难以做到正确无误，这正是许多人热衷于信道的原因。但是，归功于 Rust 的类型系统与所有权规则，咱们就无法在互斥量的加锁与解锁上出错了。


## `Mutex<T>` 的 API


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


## 在多个线程间共用 `Mutex<T>`


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


## 多线程下的多重所有权

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


## `Arc<T>` 下的原子引用计数

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
