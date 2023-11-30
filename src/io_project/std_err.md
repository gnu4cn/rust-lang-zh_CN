# 把错误消息写到标准错误而非标准输出

**Writing Error Messages to Standard Error Instead of Standard Output**


- 标准错误输出：standard error

- 标准输出：standard output

到目前为止，咱们都是在把全部输出，使用那个 `println!` 宏输出到终端。而在绝大多数终端里，都有着两种类型的终端：用于通用信息的 *标准输出* （ *standard output*，`stdout`），及用于错误消息的 *标准错误* （ *standard error*，`stderr`）。这种区别，就可以让用户选择把程序的成功输出，导向某个文件，而仍把错误消息，打印到屏幕上。

那个 `println!` 宏，只能打印到标准输出，因此这里就不得不使用其他物件，来打印到标准错误了。


## 检视报错被写到何处

**Checking Where Errors Are Written**


首先，我们来观察一下 `minigrep` 打印的内容，目前是如何写入标准输出的，包括我们想转而写入标准错误的任何错误信息。为此，我们将在有意引入一处错误的同时，把标准输出流重定向到某个文件。我们不会重定向标准错误流，因此发送到标准错误的任何内容，仍都将继续显示在屏幕上。

命令行程序预期会将错误信息，发送到标准错误流，因此即使我们将标准输出流，重定向到某个文件，也能在屏幕上看到错误信息。我们的程序目前表现不佳：我们将看到，他会将错误信息输出保存到某个文件中！

为演示这种行为，我们将以 `>` 和我们打算将标准输出流重定向到的文件 `output.txt` 路径，运行这个程序。我们将不传递任何参数，因为这会导致一个错误：

```console
$ cargo run > output.txt
```

其中的 `>` 语法，告诉 shell 将标准输出的内容，写入 `output.txt` 而非屏幕。我们没有看到，咱们所预期的错误信息，被打印在屏幕上，这就意味着他必定已经进入了那个文件。下面就是 `output.txt` 文件所包含的内容：


```txt
解析参数时遇到问题：未曾获取到查询字串
```

没错，我们的错误信息，被打印到了标准输出。将这样的错误信息，打印到标准错误中，会更有用，这样只有成功运行的数据，才会出现在文件中。我们将改变这一点。


## 将错误信息打印到标准错误

**Printing Errors to Standard Error**

我们将使用清单 12-24 中的代码，来更改错误信息的打印方式。由于本章前面的重构，所有打印错误信息的代码，都在 `main` 一个函数中。标准库提供了 `eprintln!` 这个，可以打印到标准错误流中的宏，因此咱们来将调用 `println!` 打印错误信息的两个地方，改为使用 `eprintln!`。

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

*清单 12-24：使用 `eprintln!` 将错误消息写到标准错误而非标准输出*

咱们现在来以不带任何参数，并使用 `>` 重定向标准输出的同样方式，再度运行这个程序：

```console
$ cargo run > output.txt                                                ✔
   Compiling minigrep v0.1.0 (/home/peng/rust-lang/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.49s
     Running `target/debug/minigrep`
解析参数时遇到问题：参数数量不足
```

现在，我们在屏幕上看到了错误，而 `output.txt` 不包含任何内容，这正是我们所期望的命令行程序的行为。

咱们来以一些不会导致错误，但仍会将标准输出重定向到文件的参数，再度运行这个程序，就像这样：

```console
$ cargo run -- to poem.txt > output.txt
```

我们不会看到，任何到终端的输出，而 `output.txt` 将包含我们的结果：

文件名：`output.txt`

```plaintext
在文件 poem.txt 中检索：to
Are you nobody, too?
How dreary to be somebody!
```

这表明我们现在，恰如其分的将标准输出，用于了成功的输出，而将标准错误，用于了错误输出。


# 本章小节

本章回顾了到目前为止曾学过的一些主要概念，并涵盖了在 Rust 中怎样完成常见 I/O 操作。经由使用命令行参数、文件、环境变量，以及那个用于打印错误的 `eprintln!` 宏，现在就已准备好编写命令行应用程序了。结合先前那些章中的概念，咱们所编写的代码将是良好组织、以恰当数据结构有效地存储着数据、对错误加以优美地处理，并被妥善地测试过。

接下来，这里将探讨受函数式编程所影响的一些 Rust 特性：闭包与迭代器（closures and iterators）。
