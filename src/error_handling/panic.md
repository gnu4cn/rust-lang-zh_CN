# 使用 `panic!` 的不可恢复错误

**Unrecoverable Errors with `panic!`**

某些时候，在代码中不好的事情发生了，而对其无计可施。在这些情形下，Rust 有着 `panic!` 宏。在 `panic!` 宏执行时，程序就会打印一条失败消息，释放（unwind）并清理掉栈，并在随后退出。在侦测到某种类别的代码错误，且在编写程序时刻，尚不清楚怎样处理这个故障时，就会触发一个程序中止（invoke a panic）。

> **对程序终止进行响应的栈解除或栈终止（Unwinding the Stack or Aborting in Response to a Panic）**
>
> 默认情况下，在程序终止发生时，程序就开始 *解除栈（unwinding）*，这是指 Rust 对栈进行回退，并清理他所遇到的各个函数的数据。然而，这样的回退与清理，是很多的工作量。那么因此 Rust 就允许编程者选择立即 *终止（aborting）* 的替代方案，该替代方案会不加清理的结束程序。程序曾用过的内存，这时就需要由操作系统来清理。如过在项目中，需要将生成的二进制执行文件构造得尽可能小，你们就可以通过把 `panic= 'abort'`，添加到 `Cargo.toml` 文件中恰当的 `[profile]` 小节，而从程序中止的栈解除切换为立即终止。比如，若想要在发布模式中，于程序中止时立即终止，那么就要添加这个：

```toml
[profile.release]
panic = 'abort'
```

下面就来在一个简单程序中，尝试调用 `panic!` 宏：

文件名：`src/main.rs`

```rust
fn main() {
    panic! ("崩溃并燃烧");
}
```

在运行该程序时，就会看到下面这样的东西：


```console
$ cargo run
   Compiling error_handling_demo v0.1.0 (/home/lenny/rust-lang/error_handling_demo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.40s
     Running `target/debug/error_handling_demo`
thread 'main' panicked at '崩溃并燃烧', src/main.rs:2:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

这个到 `panic!` 的调用，引起了包含在最后两行中的错误消息。第一行给出了程序中止消息及源码里程序中止发生的位置：`src/main.rs:2:5` 表示是在这里的 `src/main.rs` 文件的第二行、第五个字符处。

在此情况下，所指出的那行，就是这里代码的一部分，而在前往到那行时，就会看到那个 `panic!` 宏调用。在别的情况下，`panic!` 调用可能会在所编写代码调用的代码中，那么由该错误消息报告出的文件名与行号，就会是 `panic!` 被调用所在之处的其他人的代码，而不会是最终引起那个 `panic!` 调用的自己编写的代码行。这里可以使用该 `panic!` 调用来自那些函数的回溯，来弄清楚此处代码的哪个部分导致了该问题。接下来就要详细讨论这种回溯。

## 运用 `panic!` 回溯

现在来看一下另一个示例，看看由于代码中的编码错误，而非由于在代码中直接调用 `panic!` 宏时，来自库的 `panic!` 调用到底会是什么样子。下面清单 9-1 有一些尝试访问某个矢量中超出了有效索引范围索引的代码。

文件名：`src/main.rs`

```rust
fn main() {
    let v = vec! [1, 2, 3];

    v[99];
}
```

*清单 9-1：尝试访问某个超出了矢量末端的元素，这会导致一个到 `panic!` 的调用*

这里正尝试访问这里矢量的第 100 个元素（由于索引开始于零处，故那是在索引 99 处），但这个矢量只有 3 个元素。在此情况下，Rust 就会中止。使用 `[]` 被认为是要返回一个元素的，但在传递某个无效索引时，这里就没有 Rust 可返回的正确元素。

在 C 语言中，尝试读取超出某种数据结构，属于未定义的行为。那么就可以会得到与该数据结构中元素对应的、内存中那个位置处的任何东西，即便该内存不属于那个数据结构。这就叫做 *缓冲区重读取（a buffer overread）*，并能在攻击者可以这样的方式操作索引，来读取存储在该数据结构之后的、本不应允许他们读取的数据时，导致安全漏洞。

Rust 为保护程序免受这类漏洞的危害，就会在尝试位于某个不存在索引处的元素时，停止程序的执行而拒绝继续下去。来尝试运行一下上面的代码看看：

```console
$ cargo run                                                                                                            lenny@vm-manjaro
   Compiling error_handling_demo v0.1.0 (/home/lenny/rust-lang/error_handling_demo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.47s
     Running `target/debug/error_handling_demo`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

此错误指出了，在这里 `main.rs` 的第 4 行，其中尝试访问索引 `99` 处。接下来的注解行，讲到这里可将 `RUST_BACKTRACE` 环境变量，设置为获取究竟发生什么，才导致了这个错误。所谓 *回溯（a backtrace）*，即为到此已调用的全部函数的清单。Rust 中的回溯，与其他语言中的回溯完成的事情一样：阅读回溯的冠军，就是要从顶部开始，一直要读到自己编写的文件为止。那便是该问题缘起之处。在那个点位之上的那些行，就是所编写代码曾调用过的代码；而所编写代码之下的那些行，则是调用所编写代码的代码。这些前前后后的行，就可能包含核心 Rust 代码、 标准库代码，或者正使用着的代码箱。下面就来通过将 `RUST_BACKTRACE` 环境变量，设置为除 `0` 之外的任何值，尝试获取到回溯。下面清单 9-2 展示了与将会看到的类似输出。

```console
$ RUST_BACKTRACE=1 cargo run                                                                                           lenny@vm-manjaro
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/error_handling_demo`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5
stack backtrace:
   0: rust_begin_unwind
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/core/src/panicking.rs:142:14
   2: core::panicking::panic_bounds_check
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/core/src/panicking.rs:84:5
   3: <usize as core::slice::index::SliceIndex<[T]>>::index
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/core/src/slice/index.rs:242:10
   4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/core/src/slice/index.rs:18:9
   5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/alloc/src/vec/mod.rs:2591:9
   6: error_handling_demo::main
             at ./src/main.rs:4:5
   7: core::ops::function::FnOnce::call_once
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/core/src/ops/function.rs:248:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

*清单 9-2：在设置了 `RUST_BACKTRACE` 环境变量时，由到 `panic!` 调用生成的回溯被显示了出来*

那可是很多的输出了！具体看到的输出，可能根据操作系统与 Rust 版本而有所不同。为从此信息中获得回溯，就要开启那些调试符号。在不带 `--release` 标志使用 `cargo build` 或 `cargo run` 时，如同这里这样，这些调试符号默认就是开启的。

在上面清单 9-2 里的输出中，回溯所指向到这里项目中行的第 6 行，就是导致问题的行：即 `src/main.rs` 的第 4 行。在不想要这个程序中止时，就应在首个提到了自己所编写文件的行，所指向的那个位置，开始排查。在之前的清单 9-1 中，那里有意编写了会中止的代码，而修正程序中止的办法，就是不要请求某个超出那个矢量索引范围的元素。而在今后代码中止时，就需要搞清楚代码是在对什么值进行何种操作，而导致了中止，以及代码应该怎么做。

在本章的 [要 `panic!` 或不要 `panic!`](#要-panic-还是不要-panic) 小节，将回到 `panic!` 这个话题，并讨论在何时要用 `panic!`，何时不应使用 `panic!` 来处理不同错误情形。接下来，就会看看怎样使用 `Result`，从错误中恢复过来。


（End）


