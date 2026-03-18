# `panic!` 下的不可恢复错误

有时一些糟糕的事情会发生于咱们的代码中，而咱们对此无计可施。在这些情况下，Rust 提供了 `panic!` 宏。在实践中，造成程序停止执行的方式有两种：

- 采取导致咱们代码停止执行的一些操作，比如于超出末端处访问数组；
- 或显式调用 `panic!`。

在这两种情况下，我们都会引起程序中的停止执行。默认情况下，这些停止执行将打印失败消息、展开栈帧、清理栈并退出。通过一个环境变量，咱们还可以让 Rust 在终止执行发生时显示出调用栈，以使追踪终止执行的源头更容易。


> **响应终止执行的展开栈帧或终止**
>
> **Unwinding the Stack or Aborting in Response to a Panic**
>
> 默认情况下，当终止运行发生时，程序会开始 *展开栈帧，unwinding*，这意味着 Rust 会沿栈向上遍历，并清理他遇到的每个函数的数据。然而，回退并清理工作量很大。因此，Rust 允许咱们选择立即 *退出，aborting* 的替代方案，这会在不清理的情况下结束程序。
>
> 此时，程序使用的内存将需要由操作系统清理。当在项目中，咱们需要使最终二进制文件尽可能小时，咱们可通过添加 `panic = 'abort'` 到咱们 `Cargo.toml` 文件中的相应 `[profile]` 小节，在终止运行时从展开栈帧切换为退出。例如，若咱们希望在发布模式下当终止运行时退出，就要添加以下内容：
>
> ```toml
> [profile.release]
> panic = 'abort'
> ```
>
> 参考：
>
> - [Stack unwinding (C++ only)](https://www.ibm.com/docs/en/xl-c-and-cpp-aix/16.1.0?topic=only-stack-unwinding-c)
>
> - [Stack Unwinding in C++](https://www.geeksforgeeks.org/cpp/stack-unwinding-in-c/)

咱们来在一个简单程序中尝试调用 `panic!`：

文件名：`src/main.rs`

```rust
fn main() {
    panic! ("crash and burn");
}
```

当咱们运行程序时，咱们将看到类似下面的东西：

```console
$ cargo run
   Compiling panic_demo v0.1.0 (/home/hector/rust-lang-zh_CN/projects/panic_demo)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/panic_demo`

thread 'main' (471961) panicked at src/main.rs:2:5:
crash and burn
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

对 `panic!` 的调用会引发包含在最后两行中的错误消息。第一行显示了我们的终止运行消息，以及与咱们源代码中终止运行发生处的位置：`src/main.rs:2:5` 表示这是咱们的 `src/main.rs` 文件的第二行第五个字符。

在这一情形下，所指的行是我们代码的一部分，当我们前往该行，就会看到 `panic!` 宏调用。在别的情形下，`panic!` 调用可能位于我们的代码调用的代码中，而由错误消息报告的文件名与行号，将是 `panic!` 宏被调用处的其他人的代码，而不是最终导致 `panic!` 调用的咱们的代码的行。

我们可使用对 `panic!` 调用所在函数的回溯，找出咱们代码中造成问题的部分。为理解怎样使用 `panic!` 回溯，我们来看另一示例，看看在由于我们代码中的 bug，而不是在我们的代码中直接调用该宏，来自某个库中的 `panic!` 时会是什么样的。下面清单 9-1 有着一些代码，试图访问某个矢量中超出有效索引范围的索引。


<a name="listing_9-1"></a>
文件名：`src/main.rs`

```rust
fn main() {
    let v = vec! [1, 2, 3];

    v[99];
}
```

**清单 9-1**：尝试访问超出矢量末尾的元素，这将导致对 `panic!` 的调用

在这里，我们尝试访问矢量值的第 100 个元素（其位于索引 99 处，因为索引从 0 开始），但这个矢量只有三个元素。在这种情况下，Rust 将终止运行。使用 `[]` 本应返回一个元素，但当咱们传递无效索引时，Rust 便无法在这里返回正确的元素。

在 C 中，尝试读取超出数据结构末尾的内容属于未定义的行为。咱们可能得到内存中将对应于数据结构中该元素的位置处的任何内容，即使该内存不属于这一结构。这被称为 *缓冲区超量读取，buffer overread* <sup>1</sup>，并会在攻击者能够以这种方式操作索引，而读取到存储在数据结构之后、他们不被允许的数据时造成安全漏洞。

> **译注**：
>
> <sup>1</sup>：又称为堆缓冲区超量读取、越界读取，heap buffer overread, out-of-bounds read。
>
> 参考：
>
> - [Buffer over-read](https://en.wikipedia.org/wiki/Buffer_over-read)
>
> - [缓冲区过读](https://zh.wikipedia.org/wiki/%E7%BC%93%E5%86%B2%E5%8C%BA%E8%BF%87%E8%AF%BB)

为了保护咱们的程序免于此类漏洞的影响，当咱们尝试读取某个不存在的索引处的元素时，Rust 将停止执行并拒绝继续。我们来试试看：

```console
$ cargo run
  Compiling panic_demo v0.1.0 (/home/hector/rust-lang-zh_CN/projects/panic_demo)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/panic_demo`

thread 'main' (476738) panicked at src/main.rs:4:6:
index out of bounds: the len is 3 but the index is 99
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

这一报错指向咱们的 `main.rs` 的第 4 行，其中我们尝试访问变量 `v` 中的矢量的索引 `99`。

`note:` 行告诉我们，我们可以设置 `RUST_BACKTRACE` 环境变量来获取回溯，关于究竟发生了什么导致错误。所谓 *回溯，backtrace*，是到此为止调用的所有函数的列表。Rust 中的回溯与其他语言中的回溯工作方式一样：阅读回溯的关键是从顶部开始，读到看到咱们自己编写的文件为止。那便是问题的根源之处。该点上方的行是咱们的代码调用的代码；下方的行是调用咱们代码的代码。这些之前与之后的行可能包含核心 Rust 代码、标准库代码，或咱们正在使用的代码箱。我们来尝试通过设置 `RUST_BACKTRACE` 环境变量为除 `0` 以外的任何值来获取一次回溯。下面清单 9-2 显示了类似咱们将看到的输出。


<a name="listing_9-1"></a>
```console
$ RUST_BACKTRACE=1 cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/panic_demo`

thread 'main' (477857) panicked at src/main.rs:4:6:
index out of bounds: the len is 3 but the index is 99
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/01f6ddf7588f42ae2d7eb0a2f21d44e8e96674cf/library/std/src/panicking.rs:689:5
   1: core::panicking::panic_fmt
             at /rustc/01f6ddf7588f42ae2d7eb0a2f21d44e8e96674cf/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_bounds_check
             at /rustc/01f6ddf7588f42ae2d7eb0a2f21d44e8e96674cf/library/core/src/panicking.rs:271:5
   3: <usize as core::slice::index::SliceIndex<[T]>>::index
             at /rustc/01f6ddf7588f42ae2d7eb0a2f21d44e8e96674cf/library/core/src/slice/index.rs:272:10
   4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
             at /rustc/01f6ddf7588f42ae2d7eb0a2f21d44e8e96674cf/library/core/src/slice/index.rs:19:15
   5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
             at /rustc/01f6ddf7588f42ae2d7eb0a2f21d44e8e96674cf/library/alloc/src/vec/mod.rs:3663:9
   6: panic_demo::main
             at ./src/main.rs:4:6
   7: core::ops::function::FnOnce::call_once
             at /rustc/01f6ddf7588f42ae2d7eb0a2f21d44e8e96674cf/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

*清单 9-2：环境变量 `RUST_BACKTRACE` 设置后，由 `panic!` 调用生成的回溯信息得以显示**

这有很多输出！根据咱们操作系统与 Rust 版本的不同，咱们看到的具体输出可能会有所不同。要获得包含这些信息的回溯，必须启用调试符号。默认情况下，在不带 `--release` 命令行开关下使用 `cargo build` 或 `cargo run` 时，调试符号是启用的，就像我们这里这样。


在清单 9-2 的输出中，回溯信息中第 6 行指向了咱们项目中造成问题的那个行，即 `src/main.rs` 的第 5 行。若我们不打算咱们的程序死机，我们应该从提到我们编写文件的第一行，所指向的位置开始排查。在清单 9-1 中，我们故意编写了会引起程序死机的代码，解决死机方法是不要请求某个超出矢量值索引范围的元素。当咱们的代码在将来出现死机时，咱们需要弄清楚代码对哪些值，采取了哪些操作导致了死机，以及代码应该做些什么。



在本章的 [要 `panic!` 或不要 `panic!`](#要-panic-还是不要-panic) 小节，我们将回到 `panic!`，以及何时要、何时不应使用 `panic!`，处理错误情形。接下来，我们将看看怎样使用 `Result`，从错误中恢复。


（End）


