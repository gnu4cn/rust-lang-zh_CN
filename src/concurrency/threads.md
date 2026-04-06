# 使用线程同时运行代码

在大多数当前的操作系统中，受执行程序的代码都是运行于 *进程，a process* 中的，而操作系统将同时管理多个进程。在程序内部，咱们也可以有着同时运行的独立部分。运行这些独立部分的特性，称为 *线程，threads*。例如，某一 Web 服务器可以有多个线程，从而可以同时响应多个请求。

拆分程序中的运算为多个线程，以同时运行多重任务，虽然可以提升性能，但也会增加复杂度。由于线程可以同时运行，因此对于咱们代码在不同线程上的各个部分将运行的顺序，就没有固有保证。这可能会导致问题，比如：

- 竞争情形，其中线程正以不一致、易变的顺序访问数据或资源；
- 死锁问题，其中两个线程正相互等待，从而阻止两个线程都无法继续运行；
- 仅在特定情形下才发生的 bug，而难于重现并可靠修复。

Rust 试图缓解使用线程的负面影响，但在多线程情景下的编程仍要仔细斟酌，并需要不同于运行在单线程下的程序中的代码架构。

编程语言实现的线程的方式各不相同，且许多操作系统都提供 API，供编程语言调用以创建新线程。Rust 标准库使用 1:1 的线程实现模型，即程序对一个语言线程，使用一个操作系统线程。也有一些代码箱实现了其他线程模型，分别对 1:1 模型进行了不同取舍（我们将在下一章中看到的 Rust 的异步系统，也提供了另一种并发方法）。


## 以 `spawn` 创建新线程

要创建一个新线程，我们调用 `thread::spawn` 函数，并传递给他一个闭包（我们曾在 [第 13 章](../functional_features/closures.md) 中讨论过闭包），包含我们打算在新线程中运行的代码。下面清单 16-1 中的示例打印主线程中的一些文本和新线程中的其他文本。

<a name="listing_16-1"></a>
文件名：`src/main.rs`

```rust
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println! ("hi，生成的线程中的数字 {i} !");
            thread::sleep(Duration::from_millis(20));
        }
    });

    for i in 1..5 {
        println! ("hi，主线程中的数字 {i} !");
        thread::sleep(Duration::from_millis(20));
    }
}
```

**清单 16-1**：创建出一个新线程来打印一项内容，同时主线程打印另一内容

请注意，当 Rust 程序的主线程完成时，所有生成的线程都会关闭，无论他们是否已完成运行。这个程序的输出每次可能略有不同，但看起来类似于以下输出：

```console
$ cargo run
   Compiling spawn_demo v0.1.0 (/home/hector/rust-lang-zh_CN/projects/spawn_demo)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.33s
     Running `target/debug/spawn_demo`
hi，主线程中的数字 1 !
hi，生成的线程中的数字 1 !
hi，主线程中的数字 2 !
hi，生成的线程中的数字 2 !
hi，主线程中的数字 3 !
hi，生成的线程中的数字 3 !
hi，主线程中的数字 4 !
hi，生成的线程中的数字 4 !
```

对 `thread::sleep` 的调用会强制线程在短时间内停止执行，从而允许另一线程运行。线程可能会轮流运行，但这并无保证：其取决于咱们的操作系统调度线程的方式。在这次运行中，主线程就先行打印了，即使生成线程中的打印语句首先出现在代码中。而且即使我们告诉生成的线程打印直到 `i` 为 `9`，但在主线程关闭前他只到 `5`。

当咱们运行这段代码并且只看到主线程中的输出，或者没有看到任何的重叠时，请尝试增加两个范围（译注：`1..10`, `1..5`）中的数字，以便给操作系统在线程之间切换创造更多机会。


## 等待所有线程结束

清单 16-1 中的代码不仅会在大多数情况下因主线程的结束，而提前停止生成的线程，而且由于无法保证线程运行的顺序，我们也无法保证生成的线程会真正运行起来！

> **译注**：在 `thred::sleep` 为 `1ms` 时，将偶发出现下面的运行结果：
>
> ```console
> $ cargo run
>     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
>      Running `target/debug/spawn_demo`
> hi，主线程中的数字 1 !
> hi，生成的线程中的数字 1 !
> hi，主线程中的数字 2 !
> hi，生成的线程中的数字 2 !
> hi，主线程中的数字 3 !
> hi，生成的线程中的数字 3 !
> hi，主线程中的数字 4 !
> hi，生成的线程中的数字 4 !
> hi，生成的线程中的数字 %
> ```

我们可以通过保存 `thread::spawn` 的返回值于一个变量中，来解决生成的线程未运行或提前结束的问题。`thread::spawn` 的返回类型为 `JoinHandle`。 `JoinHandle` 属于自有值，当我们对其调用 `join` 方法时，他将等待其线程执行完毕。下面清单 16-2 展示了怎样使用我们在清单 16-1 中创建的线程的 `JoinHandle`，以及如何调用 `join` 以确保生成的线程会在 `main` 退出之前执行完毕。

<a name="listing_16-2"></a>
文件名：`src/main.rs`

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println! ("hi，生成的线程中的数字 {i} !");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println! ("hi，主线程中的数字 {i} !");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
```

**清单 16-2**：保存来自 `thread::spawn` 的 `JoinHandle`，以保证线程运行完成

> **译注**：结合第 9 章中 [出错时终止运行的快捷方式](../error_handling/result.md#出错时终止运行的快捷方式)，表明 `join` 返回的是个 `Result<T, E>` 类型的枚举值。

对句柄调用 `join` 会阻塞当前正在运行的线程，直到该句柄所代表的线程终止。所谓 *阻塞* 线程，意味着该线程会被阻止执行工作或被阻止退出。因为我们放置对 `join` 的调用在主线程的 `for` 循环之后，因此运行清单 16-2 应产生类似于下面这样的输出：

```console
$ cargo run
   Compiling spawn_demo v0.1.0 (/home/hector/rust-lang-zh_CN/projects/spawn_demo)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running `target/debug/spawn_demo`
hi，主线程中的数字 1 !
hi，生成的线程中的数字 1 !
hi，主线程中的数字 2 !
hi，生成的线程中的数字 2 !
hi，主线程中的数字 3 !
hi，生成的线程中的数字 3 !
hi，主线程中的数字 4 !
hi，生成的线程中的数字 4 !
hi，生成的线程中的数字 5 !
hi，生成的线程中的数字 6 !
hi，生成的线程中的数字 7 !
hi，生成的线程中的数字 8 !
hi，生成的线程中的数字 9 !
```

两个线程继续交替运行，但主线程因为这个对 `handle.join()` 的调用而会等待，并直到生成的线程完成才会结束。

但是我们来看看，当我们像下面这样，将 `handle.join()` 移至 `main` 的 `for` 之前会发生什么：

文件名：`src/main.rs`

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println! ("hi，生成的线程中的数字 {i} !");
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println! ("hi，主线程中的数字 {i} !");
        thread::sleep(Duration::from_millis(1));
    }
}
```

主线程将等待生成的线程完成，然后运行其 `for` 循环，因此输出将不再交错，如下所示：

```console
$ cargo run
   Compiling spawn_demo v0.1.0 (/home/hector/rust-lang-zh_CN/projects/spawn_demo)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
     Running `target/debug/spawn_demo`
hi，生成的线程中的数字 1 !
hi，生成的线程中的数字 2 !
hi，生成的线程中的数字 3 !
hi，生成的线程中的数字 4 !
hi，生成的线程中的数字 5 !
hi，生成的线程中的数字 6 !
hi，生成的线程中的数字 7 !
hi，生成的线程中的数字 8 !
hi，生成的线程中的数字 9 !
hi，主线程中的数字 1 !
hi，主线程中的数字 2 !
hi，主线程中的数字 3 !
hi，主线程中的数字 4 !
```

诸如 `join` 于何处被调用这样的细节，均会影响线程是否同时运行。


## 对线程使用 `move` 闭包

我们会经常对传递给 `thread::spawn` 的闭包使用 `move` 关键字，因为该闭包随后将取得他使用的环境中的值的所有权，从而将这些值的所有权从一个线程转移至另一线程。在第 13 章中的 [捕获引用抑或迁移所有权](../functional_features/closures.md#捕获引用抑或迁移所有权) 中，我们讨论过闭包语境下的 `move` 关键字。现在，我们将更多地关注 `move` 与 `thread::spawn` 之间的交互。

请注意，在清单 16-1 中，我们传递给 `thread::spawn` 的闭包没有取参数：我们没有在生成线程的代码中使用主线程中的任何数据。要在生成的线程中使用主线程中的数据，生成的线程的闭包必须捕获其所需的值。下面清单 16-3 展示了在主线程中创建一个矢量值，并在生成的线程中使用他的尝试。然而，正如咱们即将看到的那样，这尚不会工作。

<a name="listing_16-3"></a>
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

**清单 16-3**：尝试在另一线程中使用主线程创建的矢量值

闭包使用了 `v`，因此他将捕获 `v` 并使其成为闭包环境的一部分。由于 `thread::spawn` 会在新线程中运行这个闭包，因此我们应该能够在该新线程内部访问 `v`。但当我们编译这个示例时，我们得到以下报错：

```console
$ cargo run
   Compiling threads v0.1.0 (/home/hector/rust-lang-zh_CN/projects/threads)
error[E0373]: closure may outlive the current function, but it borrows `v`, which is owned by the current function
 --> src/main.rs:6:32
  |
6 |     let handle = thread::spawn(|| {
  |                                ^^ may outlive borrowed value `v`
7 |         println! ("这里有个矢量值：{v:?}");
  |                                     - `v` is borrowed here
  |
note: function requires argument type to outlive `'static`
 --> src/main.rs:6:18
  |
6 |       let handle = thread::spawn(|| {
  |  __________________^
7 | |         println! ("这里有个矢量值：{v:?}");
8 | |     });
  | |______^
help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword
  |
6 |     let handle = thread::spawn(move || {
  |                                ++++

For more information about this error, try `rustc --explain E0373`.
error: could not compile `threads` (bin "threads") due to 1 previous error
```

Rust *推断* 出如何捕获 `v`，并由于 `println!` 只需 `v` 的引用，闭包会尝试借用 `v`。然而，这里存在一个问题：Rust 无法判断生成的线程将运行多长时间，因此他不知道到 `v` 的引用是否将始终有效。

下面清单 16-4 提供了一个更有可能导致到 `v` 的引用将失效的场景。

<a name="listing_16-4"></a>
文件名：`src/main.rs`

```rust
#![allow(dead_code)]
#![allow(unused_variables)]

use std::thread;

fn main() {
    let v = vec! [1, 2, 3];

    let handle = thread::spawn(|| {
        println! ("这里有个矢量值：{v:?}");
    });

    drop(v); // 噢，不要！

    handle.join().unwrap();
}
```

**清单 16-4**：带有尝试捕获对主线程中的 `v` 的引用的闭包的线程，主线程弃用了 `v`

若 Rust 运行我们运行这段代码，那么生成的线程有可能会被立即置于后台而根本不会运行。生成的线程内部有着到 `v` 的引用，而主线程立即使用我们在第 15 章中讨论的 [`drop` 函数](../smart_pointers/drop-t.md#drop_func) 弃用了 `v`。然后，当生成的线程开始执行时，`v` 已不再有效，因此到他的引用也失效了。噢，不要！

要修复清单 16-3 中的编译器错误，我们可以使用错误消息的建议：

```console
help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword
  |
6 |     let handle = thread::spawn(move || {
  |                                ++++

```

通过在闭包前添加 `move` 关键字，我们强制闭包取得他用到的值的所有权，而不是让 Rust 推断他应该借用这些值。下面清单 16-5 所示的对清单 16-3 的修改，将如我们预期的那样编译和运行。

<a name="listing_16-5"></a>
文件名：`src/main.rs`

```rust
use std::thread;

fn main() {
    let v = vec! [1, 2, 3];

    let handle = thread::spawn(move || {
        println! ("这里有个矢量值：{v:?}");
    });

    handle.join().unwrap();
}
```

**清单 16-5**：使用 `move` 关键字强制闭包取得其使用的值的所有权

我们可能会尝试以同样的方式，通过使用 `move` 关键字来修复清单 16-4 中的代码，其中主线程调用了 `drop` 函数。然而，这种修复方法行不通，因为清单 16-4 试图执行的操作出于其他原因而被禁止。当我们添加 `move` 到闭包时，将迁移 `v` 到闭包的环境中，进而我们就无法再在主线程中对其调用 `drop` 了。我们会得到下面这个编译器报错：

```console
$ cargo run
   Compiling threads v0.1.0 (/home/hector/rust-lang-zh_CN/projects/threads)
error[E0382]: use of moved value: `v`
  --> src/main.rs:10:10
   |
 4 |     let v = vec! [1, 2, 3];
   |         - move occurs because `v` has type `Vec<i32>`, which does not implement the `Copy` trait
 5 |
 6 |     let handle = thread::spawn(move || {
   |                                ------- value moved into closure here
 7 |         println! ("这里有个矢量值：{v:?}");
   |                                     - variable moved due to use in closure
...
10 |     drop(v);
   |          ^ value used here after move
   |
help: consider cloning the value before moving it into the closure
   |
 6 ~     let value = v.clone();
 7 ~     let handle = thread::spawn(move || {
 8 ~         println! ("这里有个矢量值：{value:?}");
   |

For more information about this error, try `rustc --explain E0382`.
error: could not compile `threads` (bin "threads") due to 1 previous error
```

Rust 的所有权规则再次拯救了我们！我们之所以得到清单 16-3 中代码的报错，是因为 Rust 在那里保持保守，而仅为线程借用 `v`，这意味着主线程理论上可以使生成的线程的引用失效。通过告知 Rust 迁移 `v` 的所有权到生成的线程，我们向 Rust 保证主线程将不再使用 `v`。当我们以同样方式修改清单 16-4 时，那么当我们尝试在主线程中使用 `v` 时，我们就会所有权规则。`move` 关键字会覆盖 Rust 借用的默认保守做法；但他不会让我们破坏所有权规则。

既然我们已经介绍了什么是线程，以及线程 API 提供的方法，我们来看看我们可以使用线程的一些情形。


（End）


