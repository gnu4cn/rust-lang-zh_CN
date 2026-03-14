# 代码包与代码箱

我们将介绍的模组系统的前两个部分，分别是代码包与代码箱。

所谓 *代码箱，crate*，是 Rust 编译器一次考虑的最小代码量。即使咱们运行 `rustc` 而不是 `cargo`，并传递单个源代码文件（正如我们在第 1 章中 [Rust 程序基础](../getting_started/hello_world.md#rust-程序基础) 小节中所做的那样），编译器也会将该文件视为一个代码箱。代码箱可以包含模组，而模组可以定义在与该代码箱一起编译的其他文件中，正如我们将在接下来的小节中看到的那样。

代码箱可以有两种形式：二进制代码箱或库代码箱。所谓 *二进制代码箱，binary crates*，属于可以咱们编译为咱们可运行的可执行文件的程序，例如命令行程序或服务器。每个二进制代码箱都必须有个名为 `main` 的函数，定义可执行文件运行时会发生什么。到目前为止我们创建的所有代码箱都属于二进制代码箱。

所谓 *库代码箱，library crates*，没有 `main` 函数，并且他们不会编译为可执行文件。相反，他们定义了意图在多个项目中共享的功能。例如，我们在 [第 2 章](../Ch02_Programming_a_Guessing_Game.md#生成随机数) 中用到 `rand` 代码箱就提供了生成随机数的功能。大多数时候，Rustaceans 说 “代码箱，crate” 时，他们指的是库代码箱，并且他们会将 “代码箱” 与一般编程概念的 “库，library” 互换使用。

所谓 *代码箱根，crate root*，是 Rust 编译器开始之处的源代码文件，并构成了咱们代码箱的 *根模组，root module*（我们将在 [在模组下控制作用域及隐私](/packages_crates_and_modules/defining_modules.md) 处深入解释模组）。

所谓 *包，package*，是提供一组功能的一或多个代码箱的捆绑。包会包含一个 `Cargo.toml` 文件，描述如何构建这些代码箱。Cargo 实际上是个包，包含着咱们用来构建代码的命令行工具的二进制包。Cargo 包还包含了二进制代码箱所依赖的库代码箱。其他项目可依赖 Cargo 库代码箱，使用 Cargo 命令行工具使用的同样逻辑。

一个包可以包含任意数量的二进制代码箱，但最多只有一个库代码箱。一个包必须至少包含一个代码箱，无论是库还是二进制代码箱。

我们来看看当我们创建包时会发生什么。首先，我们输入命令 `cargo new my-project`：


```console
$ cargo new my-project
    Creating binary (application) `my-project` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
$ ls my-project
Cargo.toml  src
$ ls my-project/src
main.rs
```


在我们运行 `cargo new` 后，我们使用 `ls` 来查看 Cargo 创建了什么。在 `my-project` 目录下，有个 `Cargo.toml` 文件，给到我们一个包。还有个包含 `main.rs` 的 `src` 目录。请以咱们的文本编辑器打开 `Cargo.toml`，注意其中没有提到 `src/main.rs`。Cargo 遵循一项约定，即 `src/main.rs` 是与包同名的二进制代码箱的代码箱根。同样，Cargo 知道当包目录包含 `src/lib.rs` 时，那么该包就包含一个与包同名的库代码箱，并且 `src/lib.rs` 即为其代码箱根。Cargo 会传递代码箱根文件给 `rustc` 以构建库或二进制程序。

在这里，我们有个仅包含 `src/main.rs` 的包，这意味着他仅包含一个名为 `my-project` 的二进制代码箱。当某个包包含 `src/main.rs` 及 `src/lib.rs` 时，那么他就有两个代码箱：一个二进制代码箱和一个库代码箱，二者均有着与包相同的名字。通过放置文件与 `src/bin` 目录下，一个包可以有着多个二进制代码箱：每个文件都将是个单独的二进制代码箱。


（End）


