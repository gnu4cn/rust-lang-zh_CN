# 代码包与代码箱

**Packages and Crates**


这里将讲到的 Rust 模组系统的头几个部分，即为代码包与代码箱。


*代码箱（a crate）* 是 Rust 编译器一次识别到的最低数量的代码（a *crate* is the smallest amount of code that the Rust compiler considers as a time）。即使运行 `rustc` 而非 `cargo`，并传递单个源码文件（就如同在第 1 章 [“编写并运行一个 Rust 程序”](Ch01_Getting_Started.md#hello-world) 小节中曾干过的），编译器也会将那个文件，视为一个代码箱。代码箱可以包含一些模组，而这些模组则会被定义在其他的、与该代码箱一同被编译的一些文件中，就如同在接下来的小节中将看到的那样。

代码箱有着两种形式：二进制代码箱（a binary crate），或库代码箱(a library crate)。*二进制代码箱（binary crates）* 是一些可编译为能够运行的可执行程序的一些程序，譬如命令行程序或服务器。二进制代码箱必须有着一个叫做 `main` 的、定义了在可执行文件运行时所发生事情的函数。到目前为止本书中创建的全部代码箱，都是二进制代码箱。

*库代码箱* 是没有 `main` 函数的，且他们不会编译到可执行文件。相反，他们定义的是计划在多个项目下共用的功能。比如在 [第二章](Ch02_Programming_a_Guessing_Game.md#生成随机数) 中用到的 `rand` 代码箱，就提供了生成随机数的功能。在多数时候当 Rust 公民提到 “代码箱（crate）” 时，他们指的就是库代码箱，并且他们将 “代码箱（crate）” 与一般编程概念中的 “库（library）” 互换使用。

*代码箱根（crate root）* 是个 Rust 编译器开始之处的源文件，并构成了代码箱的根模组（the *crate root* is a source file that the Rust compiler starts from and makes up the root module of your crate. 后面在 [定义控制作用域和私有化的模组](#定义控制作用域和隐私的模组) 小节，将深入探讨到模组概念）。

*包（a package）* 即为提供了一套功能的一个或多个代码箱的捆绑包（a *package* is a bundle of one or more crates that provides a set of functionality）。包，包含了描述如何构建那些代码箱的一个 `Cargo.toml` 文件。Cargo 本身实际上就是，包含了前面曾用于构建代码的命令行工具二进制代码箱的包。Cargo 包还包含了一个该二进制代码箱所依赖的库代码箱。别的项目便可依靠这个 Cargo 库代码箱，来运用与 Cargo 命令行工具，所用到的同样逻辑。

代码包能包含些什么，是由数条规则所确定的。一个代码包，可包含尽可能多的二进制代码箱，但却只能包含至多一个的库代码箱。一个代码包必须包含至少一个代码箱，不管是库或二进制代码箱。

下面就来看看在创建代码包时，会发生些什么。首先，这里要敲入命令 `cargo new`:

```console
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project                                                                            ✔
Cargo.toml  src
$ ls my-project/src                                                                        ✔
main.rs
```

在运行了 `cargo new` 之后，这里便使用 `ls` 来查看 Cargo 创建了些什么。在该项目目录下，有着一个 `Cargo.toml` 文件，这就给到咱们一个代码包。其中还有一个包含了 `main.rs` 的 `src` 目录。在文本编辑器中打开 `Cargo.toml` 文件，就会注意到其中并未提及 `src/main.rs`。Cargo 遵循了 `src/main.rs` 即为与该代码包同名二进制代码箱箱根，这样一条约定。与此类似，Cargo 还知道，在代码包目录包含了 `src/lib.rs` 时，那么这个代码包就包含了与该包同名的一个库代码箱，而那个 `src/lib.rs` 就是该库代码箱的箱根。Cargo 会将代码箱根文件，传递给 `rustc`，来构建出相应的库或二进制程序。

这里有一个只包含了 `src/main.rs` 的代码包，意味着他只包含了名为 `my-project` 的一个二进制代码箱。而在代码包同时包含了 `src/main.rs` 与 `src/lib.rs` 时，他就会有两个代码箱：一个二进制和一个库代码箱，二者都有着与该代码包同样的名字。通过将一些文件放入到 `src/bin` 目录，Rust 包就可以有多个二进制代码箱：其中的每个文件，都将是单独的二进制代码箱。



