# 重定向错误到标准错误

目前，我们正使用 `println!` 宏将所有输出写到终端。在大多数终端中，都有两种类别的输出：

- 用于一般信息的 *标准输出，standard output*，`stdout`；
- 和用于错误消息的 *标准错误，standard error*，`stderr`。

这种区分使用户可以选择把程序的成功输出定向到文件，而仍打印错误消息到屏幕。

`println!` 宏只能打印到标准输出，因此我们必须使用其他东西来打印到标准错误。


## 检查错误被写到何处

首先，我们来观察 `minigrep` 打印的内容目前是如何写到标准输出的，包括我们打算写到标准错误的任何错误信息。我们将通过重定向标准输出流到文件，同时故意引发一个报错来实现这点。我们不会重定向标准错误流，因此发送到标准错误的内容，都将继续在屏幕上显示。

> **译注**：
>
> - the standard output stream，标准输出流
> - the standard error stream，标准错误流

命令行程序应发送错误信息到标准错误流，这样即使我们重定向了标准输出流到文件，我们仍然可以在屏幕上看到错误信息。我们的程序目前表现不佳：我们将看到他反而会保存错误消息输出到文件中！

为了演示这一行为，我们将以 `>`，和我们打算重定向标准输出流到其中的文件路径 `output.txt` 运行程序。我们将不传递任何参数，这应引发错误：

```console
$ cargo run > output.txt
```

`>` 语法告诉 shell 将标准输出的内容写到 `output.txt` 而不是屏幕。我们没有看到我们期望打印到屏幕的错误信息，所以这意味着其必定已在文件中。下面是 `output.txt` 包含的内容：


```txt
解析参数时遇到问题：参数不足
```

没错，我们的错误信息正被打印到标准输出。这样的错误信息打印到标准错误会更有用，这样只有成功运行的数据才会出现在文件中。我们将改变这一点。


## 打印错误到标准错误

我们将使用下面清单 12-24 中的代码，来修改错误消息的打印方式。由于我们在这一章前面所做的重构，所有打印错误消息的代码都在 `main` 一个函数中。标准库提供了 `eprintln!` 宏，可以打印到标准错误流，因此咱们来修改我们之前调用 `println!` 打印错误的两处为使用 `eprintln!`。

<a name="listing_12-24"></a>
文件名：`src/main.rs`

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln! ("解析参数时遇到问题：{err}");
        process::exit(1);
    });

    println! ("在文件 {} 中检索：{}", config.file_path, config.query);

    if let Err(e) = minigrep::run(config) {
        eprintln! ("应用程序错误：{e}");
        process::exit(1);
    }
}
```

**清单 12-24**：使用 `eprintln!` 将错误消息写到标准错误而不是标准输出

现在咱们来以同一方式再次运行程序，不带任何参数并以 `>` 重定向标准输出：

```console
$ cargo run > output.txt
   Compiling minigrep v0.1.0 (/home/hector/rust-lang-zh_CN/projects/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/minigrep`
解析参数时遇到问题：参数不足
```

现在我们在屏幕上看到错误，并且 `output.txt` 不包含任何内容，这正是我们对命令行程序所期望的行为。

我们来以不会导致错误的参数运行程序，但仍会重定向标准输出到文件，就像这样：

```console
$ cargo run -- to poem.txt > output.txt
```

我们不会看到任何到终端的输出，而 `output.txt` 将包含我们的结果：

文件名：`output.txt`

```plaintext

        在文件 poem.txt 中
        检索 to
Are you nobody, too?
How dreary to be somebody!
```

这表明我们现在正恰如其分地对成功输出使用标准输出，对错误输出使用标准错误。


# 本章小节

这一章回顾了咱们迄今为止学过的一些主要概念，并介绍了在 Rust 中怎样执行常见的 I/O 操作。通过使用命令行参数、文件、环境变量以及用于打印错误的 `eprintln!` 宏，咱们现在已经准备好编写命令行应用了。结合前几章中的概念，咱们的代码将组织良好、能有效地存储数据于适当的数据结构中、很好地处理错误，并得以良好测试。

接下来，我们将探讨一些受函数式编程影响的 Rust 特性：闭包与迭代器。


（End）


