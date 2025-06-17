# 使用 `panic!` 的不可恢复错误

**Unrecoverable Errors with `panic!`**

有时，咱们的代码中会发生一些糟糕的事情，而咱们却无能为力。在这种情况下，Rust 提供了 `panic!` 宏。在实践中，有两种方法会造成程序死机：

- 采取导致代码死机的一些操作（例如访问超出数组末端）；
- 或显式调用 `panic!`。

在这两种情况下，我们都会在程序中引起死机。默认情况下，这些死机会打印一条失败消息、栈解除，unwind、清理栈并退出。通过一个环境变量，咱们还可以让 Rust 在发生死机时显示出调用栈，以便更容易追踪死机的源头。


> **对程序死机的栈解除或栈终止**
>
> **Unwinding the Stack or Aborting in Response to a Panic**
>
> 默认情况下，当发生死机时，程序会开始 *栈解除，unwinding*，这意味着 Rust 会对栈进行回退，并清理他遇到的每个函数的数据。然而，回退并清理数据是项繁重的工作。因此，Rust 允许咱们选择立即 *终止，aborting*，即在不进行清理的情况下结束程序。
>
> 此时程序使用的内存将需要由操作系统清理。当咱们需要在项目中，使生成的二进制文件尽可能小时，咱们可通过在 `Cargo.toml` 文件中相应的 `[profile]` 小节，添加 `panic = 'abort'`，在出现死机时将栈解除切换为终止。例如，如果咱们打算在发布模式下，于出现死机时终止，就要添加以下内容：

```toml
[profile.release]
panic = 'abort'
```

> 参考：
>
> - [Stack unwinding (C++ only)](https://www.ibm.com/docs/en/xl-c-and-cpp-aix/16.1.0?topic=only-stack-unwinding-c)
>
> - [Stack Unwinding in C++](https://www.geeksforgeeks.org/cpp/stack-unwinding-in-c/)

咱们来在一个简单程序中，尝试调用 `panic!` 这个宏：

文件名：`src/main.rs`

```rust
fn main() {
    panic! ("崩溃并燃烧吧");
}
```

当咱们运行这个程序时，咱们将看到类似下面的东西：

```console
$ cargo run
   Compiling panic_demo v0.1.0 (/home/hector/rust-lang-zh_CN/projects/panic_demo)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.28s
     Running `target/debug/panic_demo`

thread 'main' panicked at src/main.rs:2:5:
崩溃并燃烧吧
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

其中对 `panic!` 的调用，引发了最后两行中的错误消息。第一行显示了我们的死机消息与咱们源代码中发送死机的位置：`src/main.rs:2:5` 表示其为咱们 `src/main.rs` 文件的第二行第五个字符处。

在这种情况下，所指明的行是我们代码的一部分，而在我们前往到该行时，就会看到那个 `panic!` 宏的调用。在其他情况下，`panic!` 的调用可能出现在我们代码所调用的代码中，而错误消息报告的文件名与行号，将是调用 `panic!` 宏的其他代码，而不是我们代码中最终导致 `panic!` 调用的那个行。

我们可使用 `panic!` 调用所处在的函数的回溯，找出咱们代码中造成问题的部分。为理解如何使用 `panic!` 回溯，我们来看另一示例，看看在由于我们代码中的错误，而不是由于直接调用这个宏，导致的某个库中 `panic!` 出现时，会发生什么情况。下面清单 9-1 有着一些试图访问某个矢量中，超出有效索引范围的索引。


文件名：`src/main.rs`

```rust
fn main() {
    let v = vec! [1, 2, 3];

    v[99];
}
```

*清单 9-1：试图访问超出某个矢量值末端的元素，将导致 `panic!` 的调用**

在这里，我们正试图访问咱们矢量值的第 100 个元素（因为索引从 0 开始，所以他位于索引 99 处），但这个矢量只有三个元素。在这种情况下，Rust 会死机。使用 `[]` 本应返回一个元素，但如果咱们传递了个无效索引，Rust 在这里就无法返回正确的元素了。

在 C 中，试图读取超过某种数据结构末尾的内容，是种未定义的行为。咱们可能会获取到内存中与该数据结构中这个元素相对应位置上的任何内容，即使该内存并不属于这个结构。这就是所谓的 *缓冲区超量读取，buffer overread*，并在攻击者能够以此类方式操作索引，读取数据结构之后存储的不被允许读取的数据时，造成安全漏洞。

为保护咱们的程序免于此类漏洞攻击，若咱们尝试读取某个不存在索引中的元素，Rust 将停止执行并拒绝继续。我们来试试看：

```console
$ cargo run
  Compiling panic_demo v0.1.0 (/home/hector/rust-lang-zh_CN/projects/panic_demo)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/panic_demo`

thread 'main' panicked at src/main.rs:5:6:
index out of bounds: the len is 3 but the index is 99
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

该错误发生在咱们的 `main.rs` 中，我们试图访问 `v` 中矢量索引 `99` 的第 5 行处。


其中 `note:` 行告诉我们，可以设置 `RUST_BACKTRACE` 这个环境变量，获取导致该错误发生具体原因的回溯信息。所谓 *回溯，backtrace*，是到此为止所调用的所有函数的列表。Rust 中的回溯与其他语言中的回溯一样：读取回溯的关键，是从顶部开始读取，直到咱们看到咱们自己编写的那些文件。这就是问题的源头。该点上方的行，是咱们代码调用的代码；下方的行，是调用咱们代码的代码。这些前后行可能包括 Rust 核心代码、标准库代码或咱们正使用的一些代码箱。我们来尝试通过将 `RUST_BACKTRACE` 这个环境变量，设置为除 `0` 以外的任意值，获取到回溯。下面清单 9-2 显示了与咱们将看到的类似输出。


```console
$ RUST_BACKTRACE=1 cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/panic_demo`

thread 'main' panicked at src/main.rs:5:6:
index out of bounds: the len is 3 but the index is 99
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/17067e9ac6d7ecb70e50f92c1944e545188d2359/library/std/src/panicking.rs:697:5
   1: core::panicking::panic_fmt
             at /rustc/17067e9ac6d7ecb70e50f92c1944e545188d2359/library/core/src/panicking.rs:75:14
   2: core::panicking::panic_bounds_check
             at /rustc/17067e9ac6d7ecb70e50f92c1944e545188d2359/library/core/src/panicking.rs:273:5
   3: <usize as core::slice::index::SliceIndex<[T]>>::index
             at /rustc/17067e9ac6d7ecb70e50f92c1944e545188d2359/library/core/src/slice/index.rs:274:10
   4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
             at /rustc/17067e9ac6d7ecb70e50f92c1944e545188d2359/library/core/src/slice/index.rs:16:9
   5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
             at /rustc/17067e9ac6d7ecb70e50f92c1944e545188d2359/library/alloc/src/vec/mod.rs:3372:9
   6: panic_demo::main
             at ./src/main.rs:5:6
   7: core::ops::function::FnOnce::call_once
             at /rustc/17067e9ac6d7ecb70e50f92c1944e545188d2359/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

*清单 9-2：环境变量 `RUST_BACKTRACE` 设置后，`panic!` 调用生成的回溯信息就显示了出来**


这有很多输出！根据咱们操作系统与 Rust 版本的不同，咱们看到的具体输出可能会有所不同。要获得包含这些信息的回溯，必须启用调试符号。默认情况下，在不带 `--release` 命令行开关下使用 `cargo build` 或 `cargo run` 时，调试符号是启用的，就像我们这里这样。


在清单 9-2 的输出中，回溯信息中第 6 行指向了咱们项目中造成问题的那个行，即 `src/main.rs` 的第 5 行。若我们不打算咱们的程序死机，我们应该从提到我们编写文件的第一行，所指向的位置开始排查。在清单 9-1 中，我们故意编写了会引起程序死机的代码，解决死机方法是不要请求某个超出矢量值索引范围的元素。当咱们的代码在将来出现死机时，咱们需要弄清楚代码对哪些值，采取了哪些操作导致了死机，以及代码应该做些什么。



在本章的 [要 `panic!` 或不要 `panic!`](#要-panic-还是不要-panic) 小节，我们将回到 `panic!`，以及何时要、何时不应使用 `panic!`，处理错误情形。接下来，我们将看看怎样使用 `Result`，从错误中恢复。


（End）


