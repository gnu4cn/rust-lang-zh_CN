# 运用线程来同步运行代码

**Using Threads to Run Code Simutaneously**


在绝大多数当前的操作系统中，被执行的程序代码，都是运行于 *进程，a process* 中的，而所在的操作系统，则会同时管理多个进程。在程序内部，咱们同样可以有着同步运行的一些独立部分。运行这些独立部分的特性，便被称作 *线程，threads*。比如，web 服务器就可以有多个线程，如此他就可以在同一时间，响应多于一个的请求。

将咱们程序的运算，拆分为多个线程，来在同一时间运行多个任务，可以提升性能，但这样也增加了复杂度。由于线程能够同步运行，因此在于不同线程上，将要运行代码哪个部分的顺序方面，就没有了某种固有保证，because threads can run simultaneously, there's no inherent guarantee about the order in which parts of your code on different threads will run。这就会导致一些问题，诸如：

- 竞争局面，其中线程正以不一致顺序，访问着一些数据或资源；
- 死锁问题，其中两个线程正相互等待，而阻止了他们继续运行下去；
- 只在一些确切情形下才发生，而难于重现并可靠修复的代码错误。

Rust 试图消除这些运用线程方面的负面影响，但在多线程情景下的编程，仍要深思熟虑，并要求与运行在单线程下程序，截然不同的代码架构。

诸多编程语言，都是以少数几种不同途径，实现的线程，且多数操作系统，均提供了编程语言为可以创建出线程而调用的 API。Rust 标准库使用的是线程实现的 1:1 模型，由此程序就会以一个语言线程，对应使用一个操作系统线程。也有实现了别的线程操作模型的代码箱，对这种 1:1 模型做出了取舍。


## 使用 `spawn` 函数创建出一个新的线程

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


## 使用 `join` 句柄，等待全部线程结束

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

> **注**：结合第 9 章中 [因错误而中止的快捷方式：`unwrap` 与 `expect`](Ch09_Error_Handling.md#因错误而中止的快捷方式unwrap-与-expect)，表明 `join` 返回的是个 `Result<T, E>` 类型的枚举值。

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


## 在线程上使用 `move` 闭包

**Using `move` Closures with Threads**


由于传递给 `thread::spawn` 的闭包随后将取得其用到的环境中一些值的所有权，由此就会把这些值的所有权，从一个线程转移到另一线程，因此咱们今后将经常在这些闭包上，使用 `move` 关键字。在第 13 章 [“捕获引用或迁移所有权”](Ch13_Functional_Language_Features_Iterators_and_Closures.md#捕获引用抑或迁移所有权) 小节，咱们就曾讨论过闭包语境下的 `move` 关键字。现在，咱们将更多地着重于 `move` 与 `thread::spawn` 之间的互动。

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
