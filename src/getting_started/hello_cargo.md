# 你好，Cargo！

Cargo 是 Rust 的构建系统和包管理器。由于 Cargo 处理了很多任务，诸如构建代码、下载代码所依赖的库，以及这些库的构建等等，因此绝大多数 Rust 公民都使用这个工具，来管理他们的 Rust 项目。（我们把这些库叫做代码需要依赖（we call the libraries that your code needs *dependencies*）。）

对于最简单的那些 Rust 程序，比如才写的那个，是没有任何依赖的。因此若使用 Cargo 来构建这个 `Hello, World!` 项目，那么就只会用到 Cargo 处理代码构建的部分。而随着更为复杂 Rust 程序的编写，就会添加依赖，而在开始一个用到 Cargo 的项目时，完成依赖添加就会容易得多。

由于广大 Rust 项目都用到了 Cargo，本书其余部分就假定了也使用 Cargo。若使用了在 [安装](#安装) 小节中提到的官方安装器进行的 Rust 安装，那么Cargo就已与 Rust 一起安装好了。而若是以其他方式安装的 Rust，那么就要通过在终端中敲入下面的命令，来检查 Cargo 是否已安装妥当：

```console
$ cargo --version
```

若能看到版本号，那么就有了这个工具！而若看到错误，诸如 `command not found`，就请查看你的安装方式的文档，找到怎样单独安装 Cargo 的方法。


## 使用 Cargo 创建项目

下面来使用 Cargo 创建一个新项目，并看看与原先的 “Hello, World!” 项目有何不同。现在导航至 `projects` 目录（或确定下来的保存代码的其他地方）。然后不论在那个操作系统之上，运行下面的命令：

```console
$ cargo new hello_cargo
$ cd hello_cargo
```

这第一个命令创建出了一个新的名为 `hello_cargo` 目录。这里就已将项目命名为了 `hello_cargo`，然后 Cargo 将其文件创建在了同名的目录里面。

进入到 `hello_cargo` 目录并列出那些文件。就会看到 Cargo 已经为我们生成了两个文件和一个目录：一个 `Cargo.toml`文件与一个里头有着 `main.rs` 文件的 `src` 目录。

`cargo new` 还初始化了一个新的、带有 `.gitignore` 文件的 Git 代码仓库。若是在一个既有的 Git 代码仓库运行的 `cargo new`，那么就不会生成那些 Git 文件；通过运用 `cargo new --vcs=git` 可重写此行为。

> 注意：Git 是种常用的版本控制系统。可通过上面的 `--vcs` 命令行参数，让 `cargo new` 使用其他版本控制系统或不使用版本控制系统。请运行 `cargo new --help`命令来查看所有可用选项。


文件名：`Cargo.toml`

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = '2021'

[dependencies]
```

*清单 1-2：由 `cargo new` 所生成的 `Cargo.toml` 的内容*

该文件是 [TOML](https://toml.io/) （ *Tom's Obvious, Minimal Language* ） 格式的，这是 Cargo 的配置格式。

该文件的第一行， `[package]`，是个小节标题，表示接下来的语句是在对一个包进行配置。随着往这个文件添加越来越多的信息，就会添加其他小节。

接下来的三行，对 Cargo 用于编译程序所需的信息进行了配置：项目名称、版本号及要使用的 Rust 版本。在 [附录 E](Appendix_E.md) 中会讲到这个 `edition` 关键字。

`Cargo.toml` 的最后一行，`[dependencies]`，是要列出项目全部依赖小节开始的地方。在 Rust 中，代码包被称为 *包裹（crates）*。此项目无需任何其他包裹，在第 2 章中的头一个项目，就会用到依赖包裹，因此在那时就会用到这个依赖小节。

现在打开 `src/main.rs` 然后看看：

文件名：`src/main.rs`

```rust
fn main() {
    println! ("Hello, World!");
}
```

Cargo 以及为我们生成了一个 “Hello, World!” 的程序，这个自动生成的程序就跟之前在清单 1-1 中的一样！到现在，先前的项目与这个 Cargo 生成的项目的不同之处，就是 Cargo 是将代码放在那个 `src` 目录中的，同时在顶层目录还有了一个 `Cargo.toml` 配置文件。

Cargo 希望那些源代码文件，存留在 `src` 目录里头。而顶层的项目目录，只用于 `README` 文件、许可证信息、配置文件及其他与代码无关的东西。使用 Cargo 有助于对项目的组织。一切都有了个地方，且一切都在各自的地方（there's a place for everything, and everything is in its place）。

若没有使用 Cargo 来开始项目，就如同先前在 “Hello, World!” 项目中所做那样，那么仍旧可使用 Cargo 将其转换为一个项目。将项目代码移入到 `src` 目录并创建出一个适当的 `Cargo.toml` 文件来:

```console
$ cd hello_world
$ mkdir src
$ mv main.rs src/
$ cargo init
```

## 构建和运行一个 Cargo 项目

现在来看看在使用 Cargo 来构建和运行那个 “Hello, World!” 程序有什么不同之处！在 `hello_cargo` 目录，通过敲入下面的命令，来构建该项目：

```console
$ cargo build                                                                   ✔
   Compiling hello_cargo v0.1.0 (/home/peng/rust-lang/projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.45s
```

此命令创建出在 `target/debug/hello_cargo`（或 Windows 上的`target\debug\hello_cargo.exe`）中，而非当前目录下的一个可执行文件。可使用下面这个命令运行那个可执行程序：

```console
$ ./target/debug/hello_cargo # 或者在 Windows 上的 .\target\debug\hello_cargo.exe
Hello, world!
```

若一切顺利，那么 `Hello, World!` 就会被打印到终端。首次运行 `cargo build`，还会造成 Cargo 在顶层目录创建一个新文件：`Cargo.lock`。该文件会跟踪项目中各个依赖的精确版本。由于这个项目没有依赖，因此该文件有些稀疏。绝无必要手动修改此文件；Cargo 会为我们管理他的内容。

文件名：`Cargo.lock`

```toml
# This file is automatically @generated by Cargo.
# It is not intended for manual editing.
version = 3

[[package]]
name = "hello_cargo"
version = "0.1.0"
```

*清单 1-3, `Cargo.lock`*

这里刚刚使用 `cargo build` 构建了一个项目，并用 `./target/debug/hello_cargo` 运行了这个项目，不过这里还可以将代码编译和运行编译结果，全部在一个命令，`cargo run`，中完成：

```console
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, World!
```

请注意这次并未见到表示 Cargo 曾在编译 `hello_cargo` 的输出。Cargo 发现这些文件并未发生改变，因此他就运行了那个二进制文件。若曾修改过源代码，那么 Cargo 就会在运行这个项目之前，重新构建该项目，从而会看到这样的输出：

```console
$ cargo run                                                                                        ✔
   Compiling hello_cargo v0.1.0 (/home/peng/rust-lang/projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/hello_cargo`
Hello, Cargo!
```

Cargo 还提供了一个叫做 `cargo check` 的命令。此命令会对代码进行快速检查，以确保代码可被编译，但该命令不会产生出可执行程序：

```console
$ cargo check                                                                                      ✔
   Checking hello_cargo v0.1.0 (/home/peng/rust-lang/projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.35s
```

这里为何不要一个可执行文件呢？通常，由于 `cargo check` 跳过了产生出可执行程序的步骤，因此他要比 `cargo build` 快得多。但在编写代码时，要持续检查已完成的工作时，那么 `cargo check` 的使用，就会加速工作流程！由于这个原因，许多的 Rust 公民，都会在编写他们的程序时，定期运行 `cargo check`，来确保程序通过编译。而在准备好使用可执行文件的时候，在运行 `cargo build`。

来概括一下到现在，已经掌握的有关 Cargo 的内容：

- 使用 `cargo new` 就可以创建出项目；
- 使用 `cargo build` 就可以构建出项目；
- 使用 `cargo run` 就可以一步完成项目的构建和运行；
- 使用 `cargo check`就可以在不产生出二进制程序的情况下，对项目加以构建以进行错误检查；
- Cargo 是将构建结果保存在 `target/debug` 目录，而不是保存在与源代码同样的目录。

使用 Cargo 的一个额外优势，就是不论是在何种操作系统上工作，那些命令都是同样的。基于这个原因，本书后续就不再提供针对 Linux 与 macOS，以及Windows 的特别说明了。

## 发布目的的构建

在项目最终准备好发布时，就可以使用 `cargo build --release` 来带优化地对其进行编译了。该命令将创建出一个位于 `target/release`，而非 `target/debug` 中的可执行文件。其中的那些优化，会令到项目的 Rust 代码运行得更快，不过开启这些优化，将增加程序编译的时间。这就是为什么有两种不同配置文件的原因：一个配置是为开发目的，在希望快速且频繁地对项目进行重新构建时使用的配置，而另一个，则是为构建要给到用户的、不会反复重新构建的、将尽可能快速运行的最终程序所用到的配置。在要对程序进行性能测试时，就一定要运行 `cargo build --release`，并对 `target/release` 中的可执行程序进行性能测试。

## 约定俗成的 Cargo

对于那些简单项目，相比于使用 `rustc`，Cargo 并未提供到很多价值，然而在程序变得愈加错综复杂时，他就会证明他的价值了。对于那些由多个代码箱（crates） 构成的复杂项目，让 Cargo 来对构建进行协调，就要容易得多。

即使这个`hello_cargo` 项目如此，此刻也用到了将在接下来的 Rust 编程生涯中会用到的真正工具。事实上，对于在任何既有的 Rust 项目，都应使用下面这些命令，使用 Git 来检出代码，然后前往到项目目录，进而加以构建：

```console
$ git clone example.org/someproject
$ cd someproject
$ cargo build
```

更多有关 Cargo 的信息，请查看看[Cargo 文档](https://doc.rust-lang.org/cargo/)。