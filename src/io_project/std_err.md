# 把错误消息写到标准错误而非标准输出

**Writing Error Messages to Standard Error Instead of Standard Output**


- 标准错误输出：standard error

- 标准输出：standard output

到目前为止，咱们都是在把全部输出，使用那个 `println!` 宏输出到终端。而在绝大多数终端里，都有着两种类型的终端：用于通用信息的 *标准输出* （ *standard output*，`stdout`），及用于错误消息的 *标准错误* （ *standard error*，`stderr`）。这种区别，就可以让用户选择把程序的成功输出，导向某个文件，而仍把错误消息，打印到屏幕上。

那个 `println!` 宏，只能打印到标准输出，因此这里就不得不使用其他物件，来打印到标准错误了。


## 对错误被写到何处进行检视

这里将使用清单 12-24 中的代码，来修改错误消息被打印出的方式。由于本章中早前完成的重构，现在打印错误消息的全部代码，就在一个函数，即 `main` 中了。Rust 标准库提供了打印到标准错误流（the standard error stream）的 `eprintln!` 宏，那么这里就来修改之前曾调用了 `println!` 的两个地方，以使用 `eprintln!` 来打印错误消息。

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

现在来一同样方式再度运行这个程序，不带任何参数并使用 `>` 对标准输出进行重定向（redirecting standard output with `>`）：

```console
$ cargo run > output.txt                                                ✔
   Compiling minigrep v0.1.0 (/home/peng/rust-lang/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.49s
     Running `target/debug/minigrep`
解析参数时遇到问题：参数数量不足
```

现在就看到了屏幕上的错误消息，同时发现 *output.txt* 中什么也没有，而这正是这所期望的命令行程序的行为了。

下面来以一些不会引起错误的参数，再度运行这个程序，不过仍要把标准输出重定向到某个文件，像下面这样：

```console
$ cargo run -- to poem.txt > output.txt
```

这里将不会看见到终端的任何输出，而 *output.txt* 则会包含这里的结果：

文件名：`output.txt`

```plaintext
在文件 poem.txt 中检索：to
Are you nobody, too?
How dreary to be somebody!
```

这就证明现在正分别对成功输出使用着标准输出，而对错误输出使用着标准错误。


## 本章小节

本章回顾了到目前为止曾学过的一些主要概念，并涵盖了在 Rust 中怎样完成常见 I/O 操作。经由使用命令行参数、文件、环境变量，以及那个用于打印错误的 `eprintln!` 宏，现在就已准备好编写命令行应用程序了。结合先前那些章中的概念，咱们所编写的代码将是良好组织、以恰当数据结构有效地存储着数据、对错误加以优美地处理，并被妥善地测试过。

接下来，这里将探讨受函数式编程所影响的一些 Rust 特性：闭包与迭代器（closures and iterators）。
