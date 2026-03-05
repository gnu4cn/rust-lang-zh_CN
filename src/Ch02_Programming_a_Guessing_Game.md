# 编写猜数游戏

**Programming a Guessing Game**


咱们来一起完成一个实践项目，开始学习 Rust！这一章通过向咱们展示如何在实际程序中如何运用他们，向咱们介绍一些常见 Rust 概念。咱们将了解

- `let`
- `match`
- 方法
- 关联函数
- 外部代码箱

等等！在接下来的章节中，我们将更详细地探讨这些概念。在本章中，咱们将只练习这些基本知识。

我们将实现一个经典的初学者编程问题：猜数游戏。其原理如下：

- 程序将随机生成一个介于 1 和 100 之间的整数；
- 然后，程序会提示玩家，输入一个猜测值；
- 猜测值输入后，程序会显示猜测值是过低还是过高；
- 如猜测正确，游戏将打印一条祝贺信息并退出。


## 建立一个新项目

要建立一个新项目，请进入咱们在第 1 章中创建的 `projects` 目录，并使用 Cargo 构造一个新项目，像这样：


```console
$ cargo new guessing_game
$ cd guessing_game
```

第一条命令，`cargo new`，取项目名字（`guessing_game`）作为第一个参数。第二条命令切换到这个新项目的目录。

查看生成的 `Cargo.toml` 文件：


文件名：`Cargo.toml`

```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2024"

[dependencies]
```

正如咱们在第 1 章中所看到的，`cargo new` 会为咱们生成一个 "Hello, world!" 程序。请查看 `src/main.rs` 这个文件：


文件名：`src/main.rs`

```rust
fn main() {
    println! ("Hello, world!");
}
```

现在我们来使用 `cargo run` 命令，在同一步骤编译并运行这个 "Hello, world!" 程序：

```console
$ cd ~/rust-lang-zh_CN/projects/guessing_game
$ cargo run
   Compiling guessing_game v0.1.0 (/home/hector/rust-lang-zh_CN/projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/guessing_game`
Hello, world!
```

当咱们需要对某个项目快速迭代，而在继续下一迭代前快速测试每次迭代，就像我们在这个游戏中将要做的时，`run` 这个命令就会派上用场。

请重新打开 `src/main.rs` 文件。咱们将在这个文件中，编写所有代码。


## 处理猜数


这个猜数游戏程序的第一部分，将请求用户输入，处理该输入并检查输入是否为预期形式。首先，我们将允许玩家输入一个猜测。请将清单 2-1 中的代码，输入到 `src/main.rs` 中。


文件名：`src/main.rs`

```rust
use std::io;

fn main() {
    println!("猜猜这个数!");

    println!("请输入你的猜数。");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("读取行失败");

    println!("你猜的是: {guess}");
}
```

*<a name="list-2-1">清单 2-1</a>，从用户处获取一个猜数并将其打印出来的代码*

这段代码包含了大量信息，所以我们来逐行查看。要获取用户输入，然后将结果作为输出打印，我们就需要将 `io` 这个输入/输出库，带入作用域。`io` 库来自标准库，称为 `std`：

```rust
use std::io;
```

默认情况下，Rust 在标准库中定义了一个项目集合，其会带入到每个程序作用域中。这个集合被称为 *前奏，prelude*，在 [标准库文档](https://doc.rust-lang.org/std/prelude/index.html) 中，咱们就能看到这个集合中的全部项目。

当咱们打算使用的某个类型不在前奏中时，咱们就必须以一条 `use` 语句，显式地将该类型带入作用域。使用 `std::io` 这个库，提供到咱们数种有用功能，包括接受用户输入的能力。

正如咱们在第 1 章所看到的，`main` 函数是程序的入口点，the entry point into the program：


```rust
fn main() {
```

`fn` 这一语法，会声明一个新的函数；括号 `()` 表明没有参数；而花括号 `{` 开启了该函数的函数体。

如同咱们还在第 1 章中所了解的，`println!` 是个打印字符串到屏幕的宏，a macro：


```console
    println! ("猜出这个数来！");
    println! ("请输入你猜的数。");
```

这段代码正打印提示信息，说明游戏为何并请求用户输入。


### 使用变量存储值


接下来，我们将创建一个 *变量，variable* 存储用户输入，就像这样：

```rust
    let mut guess = String::new();
```

现在，程序开始变得有趣起来！在这短短一行中，发生了很多事情。我们使用 `let` 语句创建变量。下面是另一个例子：


```rust
let apples = 5;
```

这一行创建了个名为 `apples` 的新变量，并将其与值 `5` 绑定。在 Rust 中，变量默认是不可变的，immutable，这意味着一旦我们赋予变量某个值，那么该值就不会改变。我们将在第 3 章 [“变量和可变性”](programming_concepts/variables_and_mutability.md) 小节中，详细讨论这一概念。要使某个变量可变，我们就要在该变量的名字前添加 `mut` 关键字：


```rust
let apples = 5; // 不可变（immutable）
let mut bananas = 5; // 可变（mutable）
```

> **注意**：其中的 `//` 语法会开始一条注释，一直持续到行尾。Rust 会忽略注释中的所有内容。我们将在 [第 3 章](programming_concepts/comments.md) 详细讨论注释。


回到猜数游戏程序，咱们现在知道，`let mut guess` 将引入一个名为 `guess` 的可变变量。等号（`=`）告诉 Rust，现在我们打算绑定某个东西到该变量。等号右边是 `guess` 要被绑定到的值，其为调用 `String::new` 的结果，这是个返回一个 `String` 新实例的函数。所谓 [`String`](https://doc.rust-lang.org/std/string/struct.String.html) ，是由标准库提供的一种字符串类型，是可增长的、UTF-8 编码的文本。

`::new` 代码行中的 `::` 语法，表明 `new` 是 `String` 类型的一个关联函数。所谓 *关联函数，associated function*，是在某个类型上实现的函数，这一情形下类型即 `String`。这个 `new` 函数会创建一个新的空字符串。在许多类型上，咱们都将发现一个 `new` 函数，因为他是构造某种新值函数的通用名字。

完整来说，`let mut guess = String::new();` 这行，已创建了个可变变量，该变量当前绑定到一个新的、`String` 的空实例。呼！


### 接收用户输入

回顾一下，在程序的第一行，我们使用 `use std::io;` 包含了标准库中的输入/输出功能。现在，我们将调用 `io` 模组中的 `stdin` 函数，其将允许咱们处理用户输入：


```rust
    io:stdin()
        .readline(&mut guess)
```

若我们没有在程序开头，以 `use std::io;` 导入 `io` 库，我们仍就可以通过将这一函数调用，写作 `std::io::stdin` 使用这个函数。`stdin` 函数会返回 [`std::io::Stdin`](https://doc.rust-lang.org/std/io/struct.Stdin.html) 的一个实例，而这是表示终端标准输入句柄的一种类型，a type that represents a handle to the standard input for your terminal。

接下来，`.read_line(&mut guess)` 这一行，调用了该标准输入句柄上的 `read_line` 方法，获取用户输入。我们还将 `&mut guess` 作为参数，传递给 `read_line`，告诉他将用户输入的内容，存储在哪个字符串中。`read_line` 的全部工作，就是取得用户输入到标准输入中的内容，并将其追加到某个字符串中（不会覆盖其内容），因此我们要将该字符串作为参数传递。这个字符串参数需要是可变的，从而这个方法才能更改该字符串的内容。

其中的 `&`，表示这个参数是个 *引用，reference*，其提供了一种在无需多次将某个数据复制到内存中的情况下，咱们代码的多个部分即可访问该数据的方法。引用属于一项复杂特性，而 Rust 的主要优点之一，便是使用引用的安全与便利。要完成这个程序，咱们无需知道很多这些细节。现在，咱们只需知道引用与变量一样，默认情况下是不可变的。因此，咱们需要写下 `&mut guess`，而不是 `&guess` 使其可变。([第 4 章](./ownership/references_and_borrowing.md) 将更详细地解释引用）。


### 以 `Result` 处理潜在失效


我们仍在研究这行代码。我们现在讨论的是第三行文字，但要注意，他仍是单个逻辑代码行的一部分。下一部分是这个方法：


```rust
        .expect("读取输入失败");
```


我们本可以将这段代码写成：


```rust
io::stdin().read_line(&mut guess).expect("读取输入失败");
```

不过，一个长行难于阅读，所以最好将其分开。在咱们使用 `.method_name()` 语法，调用某个方法时，引入一个换行符，以及另外的空白，来帮助拆分长行，通常是明智之举。现在我们来讨论一下这行的作用。

如早先曾提到的，`read_line` 会将用户输入的任何内容，放入我们传给他的字符串中，但他还是会返回个 `Result` 值。所谓 [`Result`](https://doc.rust-lang.org/std/result/enum.Result.html)，是个 [*枚举值，enumeration*](Ch06_Enums_and_Pattern_Matching.md)，通常称为 `enum`，是一种可处于多种可能状态之一的类型。我们称每种可能状态，为一个 *变种，variant*。

[第 6 章](Ch06_Enums_and_Pattern_Matching.md) 将详细介绍枚举。这些 `Result` 类型的目的，是要编码错误处理信息。

`Result` 的变种，分别为 `Ok` 与 `Err`。`Ok` 变种表示操作成功，且其包含了成功产生的值。`Err` 变种表示操作失败，且其包含了操作怎样及为何失败的信息。

与任何类型的值一样，`Result` 类型的值，也有着定义于其上的一些方法。`Result` 的实例有着一个咱们可调用的 [`expect` 方法](https://doc.rust-lang.org/std/result/enum.Result.html#method.expect)。当 `Result` 实例是个 `Err` 值时，`expect` 就将导致程序崩溃，并显示咱们作为参数传递给 `expect` 那条信息。当 `read_line` 方法返回了个 `Err` 时，那么很可能是来自底层操作系统的某种出错的结果。当这个 `Result` 实例是个 `Ok` 值时，`expect` 将取得那个 `Ok` 持有的返回值，并将该值返回给咱们，以便咱们可以使用他。在本例中，该值就是用户输入的字节数。

若咱们不调用 `expect`，这个程序会编译，但咱们将收到一条告警：


```console
$ cargo build
   Compiling guessing_game v0.1.0 (/home/hector/rust-lang-zh_CN/projects/guessing_game)
warning: unused `Result` that must be used
  --> src/main.rs:10:5
   |
10 | /     io::stdin()
11 | |         .read_line(&mut guess);
   | |______________________________^
   |
   = note: this `Result` may be an `Err` variant, which should be handled
   = note: `#[warn(unused_must_use)]` (part of `#[warn(unused)]`) on by default
help: use `let _ = ...` to ignore the resulting value
   |
10 |     let _ = io::stdin()
   |     +++++++

warning: `guessing_game` (bin "guessing_game") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
```

Rust 警告咱们未曾使用自 `read_line` 返回的 `Result` 值，表明程序尚未处理一个可能的错误。

消除这一告警的正确方法，是着手编写出错误处理代码，但在我们的情况下，我们只打算在问题出现时崩溃这个程序，因此咱们可以使用 `expect`。咱们将在 [第 9 章](error_handling/result.md) 中，学习如何从错误中恢复。


### 以 `println!` 的占位符打印值

这段代码中，除了结尾的大括号，到目前为止就只有一行需要讨论了：


```rust
    println! ("你猜的数是：{guesss}");
```

这行会打印现在包含着用户输入的那个字符串。其中的 `{}` 花括号组是个占位符：把 `{}` 想象成固定某个值位置的两个小螃蟹钳子。在打印某个变量的值时，变量名可以放在这对花括号内。在打印对表达式求值的结果时，就要在格式字符串中放置空的大括号，然后在格式字符串后，跟上以逗号分隔的表达式列表，以相同顺序在各个空的大括号占位符中打印。在对 `println!` 的一次调用中，打印某个变量及某个表达式的结果，将如下所示：


```rust
let x = 5;
let y = 10;

println! ("x = {x} 而 y + 2 = {}", y + 2);
```

此代码将打印出 `x = 5 而 y + 2 = 12`。


### 测试第一部分

**Testing the First Part**


我们来测试一下这个猜数游戏的第一部分。请使用 `cargo run` 运行他：


```console
$ cargo run
   Compiling guessing_game v0.1.0 (/home/hector/rust-lang-zh_CN/projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/guessing_game`
猜猜这个数!
请输入你的猜数。
6
你猜的是: 6

```

至此，这个游戏的第一部分就已完成：我们从键盘获取输入，并随后将其打印。


## 生成秘密数字

接下来，我们需要生成一个用户将尝试猜测的秘密数字。秘密数字应每次都不同，这样游戏玩多次才有趣。我们将使用 1 到 100 之间的某个随机数，这样游戏就不会太难。Rust 尚未在其标准库中包含随机数功能。不过，Rust 团队确实提供了个带有上述功能的 [`rand` 代码箱](https://crates.io/crates/rand)。


### 以代码箱增加功能


请记住，代码箱属于 Rust 源代码文件的集合。我们一直在构建的这个项目，是个 *二进制代码箱，binary crate*，属于一个可执行代码箱。而 `rand` 代码箱，则是个 *库代码箱，library crate*，包含着旨在用于其他程序，而不能在其本身上执行的代码。

Cargo 对外部代码箱的协调能力，正是 Cargo 的真正亮点所在。在我们可以编写那些用到 `rand` 的代码前，我们需要修改 `Cargo.toml` 这个文件，作为一个依赖项包含 `rand` 这个代码箱。现在请打开该文件，在 Cargo 为咱们创建的 `[dependencies]` 小节标题下，添加下面一行到底部。要务必以这个版本号，准确指定 `rand`，否则这一教程中的代码示例，可能无法工作：


文件名：`Cargo.toml`

```toml
[dependencies]
rand = "0.8.5"
```

在 `Cargo.toml` 这个文件中，某头部之后的所有内容，都属于该小节的部分，一直持续到另一小节开始。在 `[dependencies]` 下，咱们告诉 Cargo，咱们的项目依赖于哪些外部代码箱，以及咱们需要这些代码箱的哪个版本。在本例中，我们以语义版本说明符 `0.8.5`，指定了 `rand` 这个代码箱。Cargo 理解 [语义版本编号，Semantic Versioning](http://semver.org/)，有时也称为 *SemVer*，这是一项编写版本号的标准。`0.8.5` 实际上是 `^0.8.5` 的缩写，表示至少是 `0.8.5` 但低于 `0.9.0` 的任何版本。

Cargo 认为这些版本有着与 `0.8.5` 版本兼容的公共 API，而这一规范确保咱们将得到，仍可与本章中代码一起编译的最新补丁发布。任何 `0.9.0` 或更高版本，都不保证有着与接下来示例用到的相同 API。

现在，在不修改任何代码下，我们来构建这个项目，如清单 2-2 中所示。


```console
$ cargo build
    Updating `ustc` index
remote: Enumerating objects: 1043606, done.
remote: Counting objects: 100% (935070/935070), done.
remote: Compressing objects: 100% (288426/288426), done.
remote: Total 1043606 (delta 693593), reused 848030 (delta 620187), pack-reused 108536 (from 1)
接收对象中: 100% (1043606/1043606), 632.81 MiB | 1.89 MiB/s, 完成.
处理 delta 中: 100% (724019/724019), 完成.
来自 https://mirrors.ustc.edu.cn/crates.io-index
 * [新引用]                HEAD       -> origin/HEAD
     Locking 14 packages to latest Rust 1.93.1 compatible versions
      Adding cfg-if v1.0.4
      Adding getrandom v0.2.17
      Adding libc v0.2.182
      Adding ppv-lite86 v0.2.21
      Adding proc-macro2 v1.0.106
      Adding quote v1.0.45
      Adding rand v0.8.5 (available: v0.10.0)
      Adding rand_chacha v0.3.1
      Adding rand_core v0.6.4
      Adding syn v2.0.117
      Adding unicode-ident v1.0.24
      Adding wasi v0.11.1+wasi-snapshot-preview1
      Adding zerocopy v0.8.40
      Adding zerocopy-derive v0.8.40
  Downloaded cfg-if v1.0.4 (registry `ustc`)
  Downloaded getrandom v0.2.17 (registry `ustc`)
  Downloaded ppv-lite86 v0.2.21 (registry `ustc`)
  Downloaded rand_chacha v0.3.1 (registry `ustc`)
  Downloaded rand_core v0.6.4 (registry `ustc`)
  Downloaded rand v0.8.5 (registry `ustc`)
  Downloaded zerocopy v0.8.40 (registry `ustc`)
  Downloaded libc v0.2.182 (registry `ustc`)
  Downloaded 8 crates (1.2MiB) in 6.49s
   Compiling libc v0.2.182
   Compiling zerocopy v0.8.40
   Compiling cfg-if v1.0.4
   Compiling getrandom v0.2.17
   Compiling rand_core v0.6.4
   Compiling ppv-lite86 v0.2.21
   Compiling rand_chacha v0.3.1
   Compiling rand v0.8.5
   Compiling guessing_game v0.1.0 (/home/hector/rust-lang-zh_CN/projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 9m 51s
```

*<a name="list-2-2">清单 2-2</a>：在添加 `rand` 代码箱作为依赖项添加后，运行 `cargo build` 的输出**

> **译注**：这里译者使用了国内的 `ustc` Cargo 登记簿镜像。


咱们可能会看到一些不同版本号（但归功于 SemVer，他们都将与这段代码兼容！），以及一些不同行（取决于操作系统），同时这些行可能顺序不同。

在我们包含了某个外部依赖项后，Cargo 会从 *登记簿，registry* 获取该依赖项所需的所有内容最新版本，所谓登记簿，是 [crates.io](https://crates.io/) 上数据的拷贝。Crates.io 是 Rust 生态系统中的人们，发布他们的开源 Rust 项目供他人使用的地方。

更新登记簿后，Cargo 会检查 `[dependencies]` 小节，下载列出的任何尚未下载的代码箱。在本例中，虽然我们只将 `rand` 列为依赖项，但 Cargo 还抓取了 `rand` 运作所依赖的其他代码箱。下载这些代码箱后，Rust 会编译他们，然后在这些可用依赖项下编译这个项目。

当咱们在不做任何修改下立即再次运行 `cargo build`，那么除了 `Finished` 行外咱们不会得到任何输出。Cargo 知道他已下载并编译了依赖项，而咱们也并未在 `Cargo.toml` 文件中对他们做任何修改。Cargo 还知道咱们并未对咱们的代码修改任何东西，所以他也未重新编译项目。由于无事可做，他就直接退出了。


```console
$ cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
```


若咱们打开 `src/main.rs` 文件，进行一些简单修改，然后保存并再次构建，那么咱们将只会看到两行输出：


```console
$ cargo build
   Compiling guessing_game v0.1.0 (/home/hector/rust-lang-zh_CN/projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
```

这些行显示，Cargo 只会在咱们对 `src/main.rs` 文件的微小改动下更新构建。咱们的依赖依赖并未改变，因此 Cargo 明白，他可针对这些依赖项，重用他已下载并编译好的那些。


### 以 `Cargo.lock` 文件确保可重现的构建

Cargo 具有一种确保咱们或其他人每次构建咱们代码时，都能重新构建出同一产物的机制： Cargo 将只使用咱们所指定的依赖项版本，除非咱们另有指示。例如，`rand` 代码箱下周的 `0.8.6` 版本将发布，而该版本包含着一个重要 bug 修复，但同时也包含了一个将破坏咱们代码的回退。为处理这一问题，Rust 会在咱们首次运行 `cargo build` 时创建 `Cargo.lock` 文件，因此在 `guessing_game` 目录下，我们现在就会有这一文件。

当咱们首次构建某个项目时，Cargo 会计算出符合条件的所有依赖项版本，然后将其写入 `Cargo.lock` 文件。在咱们今后构建咱们的项目时，Cargo 就会发现 `Cargo.lock` 文件存在，并将使用指定于该处的版本，而再行完成再次计算版本的全部工作。这样做让咱们能自动获得可重现的构建。换句话说，归功于 `Cargo.lock` 这个文件，在咱们显式升级 `rand` 依赖项前，咱们的项目将保持在 `0.8.5` 版本。由于 `Cargo.lock` 文件对可重现的构建非常重要，因此他通常会与项目中的其他代码一起，签入到源代码控制系统中。


### 更新代码箱获取新版本

当咱们 *确实* 打算更新某个代码箱时，Cargo 提供了 `update` 命令，他会忽略 `Cargo.lock` 文件，并计算出所有符合咱们在 `Cargo.toml` 中规范的全部最新版本。Cargo 随后将把这些版本写入 `Cargo.lock` 文件。否则，默认情况下，Cargo 将只查找大于 `0.8.5` 且小于 `0.9.0` 的版本。当 `rand` 代码箱已发布了两个新版本 `0.8.6` 和 `0.999.0` 时，那么当咱们运行 `cargo update` 时，就会看到下面的内容：


```console
$ cargo update
    Updating crates.io index
    Updating rand v0.8.5 -> v0.8.6
```


Cargo 会忽略 `0.999.0` 这个版本。此时，咱们还会注意到咱们 `Cargo.lock` 文件中的一处变化，显示咱们现在使用的 `rand` 代码箱版本为 `0.8.6`。要使用 `rand` 版本 `0.999.0` 或 `0.999.x` 系列中的任何版本，咱们就必须更新 `Cargo.toml` 文件为看起来像这样（请勿实际执行这一修改，因接下来的示例假设咱们使用的是 `rand` 0.8）：


```toml
[dependencies]
rand = "0.999.0"
```

下次咱们运行 `cargo build` 时，Cargo 将更新可用的代码箱登记簿，the registry of crates available，并根据咱们已指定的新版本，重新计算咱们的 `rand` 需求。

关于 [Cargo](http://doc.crates.io/) 及 [其生态](http://doc.crates.io/crates-io.html)，还有很多内容要讲，我们将在 [第 14 章](./Ch14_More_about_Cargo_and_Crates-io.md) 讨论，但现在，这就是咱们需要了解的全部内容。Cargo 让重用库变得非常容易，因此 Rustaceans 就能够编写出由数个包组装而成的一些小型项目。


### 生成随机数

咱们来开始使用 `rand` 生成一个要猜的数字。下一步是要更新 `src/main.rs`，如下清单 2-3 中所示。


文件名：`src/main.rs`

```rust
use std::io;
use rand::Rng;

fn main() {
    println! ("猜猜这个数!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println! ("秘密数字是：{secret_number}");

    println! ("请输入你的猜数。");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("读取行失败！");

    println! ("你猜的是: {guess}");
}
```

*<a name="list-2-3">清单 2-3</a>：添加生成随机数的代码*


首先，我们要添加行 `use rand::Rng;`。`Rng` 这个特质定义了随机数生成器所实现的一些方法，而这个特质必须要在咱们要用到那些方法的作用域中。[第 10 章](./generic_types_traits_and_lifetimes/traits.md) 将详细介绍特质。

接下来，我们在中间添加了两行。在第一行中，我们调用了 `rand:thread_rng` 这个函数，他给到我们即将用到的特定随机数生成器，the particular random number generator：一个属于当前执行线程本地的，由操作系统提供种子的随机数生成器。然后，我们调用了这个随机数生成器上的 `gen_range` 方法。这个方法是由我们以 `use rand::Rng;` 语句，带入到作用域的 `Rng` 特质定义的。`gen_range` 方法会取一个范围表达式作为参数，生成该范围中的一个随机数。我们这里使用的范围表达式类别，采用了 `start..=end` 形式，并包含下上边界，因此我们需要指定 `1...=100`，请求一个 1 和 100 之间的数字。


> **注意**：咱们不会凭空知道要使用哪个特质、调用某个代码箱中的哪些方法与函数，因此每个代码箱都有使用说明文档。Cargo 的另一巧妙特性便是，运行 `cargo doc --open` 命令就将在本地构建出有所有咱们依赖项所提供的文档，并在咱们浏览器中打开。例如，当咱们对 `rand` 代码箱中别的功能感兴趣时，那么就运行 `cargo doc --open`，并点击左侧边栏中的 `rand`。


第二个新行会打印秘密数字。这在我们开发程序时很有用，能够测试程序，但我们将在最终版本中删除他。如果这个程序一开始就打印出答案，那就不算是个游戏了！

尝试运行这个程序几次：


```console
$ cargo run
   Compiling guessing_game v0.1.0 (/home/hector/rust-lang-zh_CN/projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/guessing_game`
猜猜这个数!
秘密数字是：14
请输入你的猜数。
80
你猜的是: 80

$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/guessing_game`
猜猜这个数!
秘密数字是：49
请输入你的猜数。
5
你猜的是: 5

```

咱们应得到不同的随机数字，且他们都应是 1 到 100 之间的数字。干得好！


## 比较猜数与秘密数


现在我们有了用户输入和随机数，我们可以比较他们。该步骤于清单 2-4 中所示。请注意，这段代码还不会编译，我们将进行解释。

<a name="list_2-4"></a>
文件名：`src/main.rs`

```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // --跳过前面的代码--

    println! ("你猜的数为：{}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println! ("太小！"),
        Ordering::Greater => println! ("太大！"),
        Ordering::Equal => println! ("你赢了！"),
    }
}
```

*<a name="list-2-4">清单 2-4</a>：处理比较两个数字可能的返回值*


首先，我们添加另一条 `use` 语句，将一个名为 `std::cmp::Ordering` 的类型，从标准库中带入作用域。`Ordering` 类型属于另一个枚举，有着 `Less`、`Greater` 及 `Equal` 三种变体。这正是咱们在比较两个值时，可能的三种结果。

然后，我们在底部添加用到这个 `Ordering` 类型的五个新行。`cmp` 这个方法会比较两个值，并可在任何可比较项目上调用。他会取一个到咱们打算比较的任何值的引用：这里他正将 `guess` 与 `secret_number` 比较。然后，他会返回我们以 `use` 语句带入作用域的 `Ordering` 枚举的某个变种。我们使用一个 `match` 表达式，根据 `guess` 和 `secret_number` 中值下，对 `cmp` 的调用所返回的何种 `Ordering` 变种，决定下一步要做什么。

`match` 表达式由一些 *支臂，arms* 组成。而支臂由要匹配的 *模式，pattern*，及在给到 `match` 的值符合该支臂的模式时，要运行的代码组成。Rust 会取得给到 `match` 的值，并依次查看各个支臂的模式。模式与 `match` 结构，属于 Rust 的强大功能：二者让咱们可以表达出咱们代码可能遇到的各种情况，他们确保咱们能处理全部的这些情况。这两种特性，将在 [第 6 章](./enums_and_pattern_matching/match_control_flow.md) 和 [第 19 章](./Ch18_Patterns_and_Matching.md) 分别详细介绍。

咱们来以这里用到的 `match` 表达式，看一个示例。假设用户猜的是 50，而这次随机生成的秘密数字是 38。

当代码比较 50 与 38 时，`cmp` 方法将返回 `Ordering::Greater`，因为 50 大于 38。`match` 表达式会得到 `Ordering::Greater` 这个值，并开始检查各个支臂的模式。他看着第一个支臂的模式，`Ordering::Less`，发现值 `Ordering::Greater` 未匹配 `Ordering::Less`，因此他会忽略该支臂的代码并移步到下一支臂。下一支臂的模式是 `Ordering::Greater`，这 *确实* 匹配 `Ordering::Greater`！该支臂中的相关代码将执行，并打印 `太大！` 到屏幕。`match` 表达式会在第一次成功匹配后结束，因此在这种情况下他将不查看最后支臂。

然而，清单 2-4 中的代码还无法编译。咱们来尝试一下：


```console
$ cargo build
   Compiling guessing_game v0.1.0 (/home/hector/rust-lang-zh_CN/projects/guessing_game)
error[E0308]: mismatched types
  --> src/main.rs:22:21
   |
22 |     match guess.cmp(&secret_number) {
   |                 --- ^^^^^^^^^^^^^^ expected `&String`, found `&{integer}`
   |                 |
   |                 arguments to this method are incorrect
   |
   = note: expected reference `&String`
              found reference `&{integer}`
note: method defined here
  --> /rustc/01f6ddf7588f42ae2d7eb0a2f21d44e8e96674cf/library/core/src/cmp.rs:987:8

For more information about this error, try `rustc --explain E0308`.
error: could not compile `guessing_game` (bin "guessing_game") due to 1 previous error
```


该报错的核心，指出了有着 *不匹配类型，mismatched types*。Rust 有着强大的、静态的类型系统。不过，他也有类型推断，Rust has a strong, static type system. However, it also has type inference。在我们写下 `let mut guess = String::new()` 后，Rust 就能够推断出 `guess` 应是个 `String`，而未曾让我们写下类型。另一方面，`secret_number` 是个数字类型。Rust 的一些数字类型，可以有 1 和 100 之间的值：

- `i32`，32 位数字；
- `u32`，无符号的 32 位数字；
- `i64`，64 位数字；
- 以及其他类型。


除非另有说明，否则 Rust 默认为 `i32`，这便是 `secret_number` 的类型，除非咱们别处添加了将导致 Rust 推断出不同数值类型的类型信息。这一报错的原因是 Rust 无法比较字符串和数字类型。

最终，我们会打算将程序作为输入读取的 `String` 转换为某种数字类型，以便我们可将其与秘密数字进行数值比较。我们要通过添加下面这行到 `main` 函数体完成这点：

文件名：`src/main.rs`

```rust
    // --跳过前面的代码--
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("读取行失败！");

    let guess: u32 = guess.trim().parse().expect("请输入一个数字！");

    println! ("你猜的是: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println! ("太小！"),
        Ordering::Greater => println! ("太大！"),
        Ordering::Equal => println! ("你赢了！"),
    }
```

该行为：

```rust
let guess: u32 = guess.trim().parse().expect("请输入一个数字！");
```

我们创建了个名为 `guess` 的变量。但等等，这个程序不是已经有了个名为 `guess` 的变量了吗？确实如此，但好在 Rust 允许我们以一个新值，遮蔽 `guess` 的先前值。所谓 *遮蔽，shadowing*，允许咱们重用 `guess` 这个变量名，而不是强迫我们创建两个唯一变量，比如 `guess_str` 和 `guess`。我们将在 [第 3 章](programming_concepts/variables_and_mutability.md#遮蔽) 中更详细地介绍这一特性，而现在我们要知道，当咱们打算将某个值从一种类型转换为另一类型时，会经常用到这一特性。

我们将这个新变量绑定到 `guess.trim().parse()` 这个表达式。表达式中的 `guess`，指的是包含了作为字符串的输入的原先 `guess` 变量。`String` 实例上的 `trim` 方法，将消除开头和结尾的任何空白，在将字符串转换为 `u32` 前我们必须执行此操作，`u32` 只能包含数字数据。用户必须按下回车键 `enter`，满足 `read_line` 并输入他们的猜数，这会添加一个换行符到字符串。例如，当用户输入 `5` 并按回车键时，`guess` 看起来是这样的：`5\n`。其中 `\n` 表示 “换行/newline”。(在 Windows 系统上，按下回车键会产生回车和换行，即 `\r\n` <sup>1</sup>）。`trim` 方法会消除 `\n` 或 `\r\n`，得到仅 `5`。

> **译注 1**：这也是为何先前的代码：
>
> ```rust
>     let bytes = io::stdin()
>         .read_line(&mut guess)
>         .expect("读取行失败/failed to read line");
> ```
>
> 在 Windows 的 MSYS2 上运行时，`bytes` 的输出始终会比咱们看到的字符串，要多两个字节的原因。


[字符串上的 `parse` 方法](https://doc.rust-lang.org/std/primitive.str.html#method.parse) 会将字符串转换为另一类型。在这里，我们用他将字符串转换为数字。我们需要通过使用 `let guess: u32`，告诉 Rust 我们想要的确切数字类型。`guess` 后的冒号（`:`）告诉 Rust，我们将注解这个变量的类型。Rust 有几种内置的数字类型；这里看到的 `u32`，属于一种无符号的 32 位整数。对于小的正数来说，这是种不错的默认选项。咱们将在 [第 3 章](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-types) 中了解其他数字类型。

此外，这个示例程序中的 `u32` 注解及与 `secret_number` 的比较，意味着 Rust 将推断出 `secret_number` 也应是个 `u32`。因此，现在这个比较将是两个相同类型值间的了！

`parse` 方法将只工作于逻辑上可转换为数字的一些字符，因此容易引发错误。例如，当字符串包含了 `A👍%` 时，就无法将其转换为数字。因为 `parse` 方法可能失败，`parse` 方法就会返回一个 `Result` 类型，就像 `read_line` 方法一样（早先在 [“以 `Result` 处理潜在失败”](#以-result-处理潜在失效) 中曾讨论过）。我们将再次通过使用 `expect` 方法，以同样方式处理这个 `Result`。当 `parse` 因无法从字符串创建出一个数字而返回 `Err` 的 `Result` 变种时，那么这个 `expect` 调用将崩溃游戏并打印我们给他的信息。当 `parse` 能成功转换字符串为数字时，他将返回 `Result` 的 `Ok` 变种，而 `expect` 就将从这个 `Ok` 值返回我们想要的数字。

现在咱们来运行这个程序：


```console
$ cargo run
   Compiling guessing_game v0.1.0 (/home/hector/rust-lang-zh_CN/projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/guessing_game`
猜猜这个数!
秘密数字是：57
请输入你的猜数。
  80
你猜的是: 80
太大！
```

不错！即使有空格添加在猜数前，程序仍然识别出用户猜测的是 76。请多运行几次程序，验证在不同输入类别下的不同行为：猜对数字、猜的数字太大、猜的数字太小等等。

现在我们让这个游戏的大部分都工作了，但用户只能猜一次数。我们来通过添加一个循环，改变这种情况！


## 以循环实现多次猜数


`loop` 关键字会创建出一个无限循环。我们将添加一个循环，让用户有更多机会猜出数字：


文件名：`src/main.rs`

```rust
    // --跳过--
    println! ("秘密数字是：{secret_number}");

    loop {
        println! ("请输入你的猜数。");

        // --跳过--

        match guess.cmp(&secret_number) {
            Ordering::Less => println! ("太小！"),
            Ordering::Greater => println! ("太大！"),
            Ordering::Equal => println! ("你赢了！"),
        }
    }
}
```

正如咱们所看到的，我们已将猜数输入提示之后的所有内容，都移入到一个循环中。要确保将这个循环内部的每行代码，都再缩进四个空格，并再次运行这个程序。程序现在一直不停地请求另一个猜数，这实际上引入了一个新问题。用户似乎无法退出了！

用户始终可通过键盘快捷键 `ctrl-C` 中断程序。但正如 [比较猜数与秘密数](#比较猜数与秘密数) 中 `parse` 的讨论中所提到的，还有另一种逃离这个贪婪怪物的办法：当用户输入某个非数字答案时，这个程序就将崩溃。我们可以利用这点允许用户退出，如下所示：


```console
$ cargo run
   Compiling guessing_game v0.1.0 (/home/hector/rust-lang-zh_CN/projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/guessing_game`
猜猜这个数!
秘密数字是：2
请输入你的猜数。
45
你猜的是: 45
太大！
请输入你的猜数。
1
你猜的是: 1
太小！
请输入你的猜数。
30
你猜的是: 30
太大！
请输入你的猜数。
2
你猜的是: 2
你赢了！
请输入你的猜数。
quit

thread 'main' (406104) panicked at src/main.rs:21:47:
请输入一个数字！: ParseIntError { kind: InvalidDigit }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

输入 `quit` 将退出游戏，但正如咱们将发现的，输入任何其他非数字输入也会这样。至少可以说，这是次优的；我们希望正确数字猜到后游戏也会停止。


### 猜对后的退出

我们来通过添加一个 `break` 语句，将游戏编程为在用户获胜后退出：


文件名：`src/main.rs`

```rust
        // --跳过--

        match guess.cmp(&secret_number) {
            Ordering::Less => println! ("太小！"),
            Ordering::Greater => println! ("太大！"),
            Ordering::Equal => {
                println! ("你赢了！");
                break
            }
        }
    }
}
```

在 `println! ("你赢了！");` 后添加 `break` 行，令到程序在用户正确猜出秘密数字时退出循环。退出这个循环也意味着退出程序，因为该循环是 `main` 的最后部分。

> **译注**：这里有个有趣的地方，`break` 后的分号可有可无，`match` 表达式最后支臂后的逗号，也是可有可无的。


### 处理无效输入

为进一步完善游戏的行为，我们来将游戏构造为忽略非数字，以便用户可以继续猜数，而不是在用户输入非数字时崩溃程序。通过修改其中 `guess` 从字符串转换为 `u32` 的行，咱们便可做到这点，如下清单 2-5 中所示。


文件名：`src/main.rs`

```rust
        // --跳过--

        io::stdin()
            .read_line(&mut guess)
            .expect("读取行失败！");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println! ("你猜的是: {guess}");

        // --跳过--
```

*<a name="list-2-5">清单 2-5</a>：忽略非数字的猜数并请求另一猜数，而不是让崩溃程序*


我们从 `expect` 调用切换到 `match` 表达式，以从出错时崩溃程序转而处理错误。请记住，`parse` 会返回一个 `Result` 类型，而 `Result` 是个枚举，有着 `Ok` 和 `Err` 两个变种。我们在这里使用了个 `match` 表达式，就像我对 `cmp` 方法的 `Ordering` 结果所做的一样。

当 `parse` 能够成功将字符串转换为数字时，他将返回包含着结果数字的一个 `Ok` 值。该 `Ok` 值将匹配第一支臂的模式，而 `match` 表达式将只返回 `parse` 生成并放入 `Ok` 值的 `num` 值。这个数字最终将位于我们所期望的我们正创建的新 `guess` 变量中。

当 `parse` *不* 能该字符串转换为数字时，他将返回一个包含着更多有关该错误信息的 `Err` 值。`Err` 值不会匹配第一个 `match` 支臂中的 `Ok(num)` 模式，但他确实会匹配到第二个支臂中的 `Err(_)` 模式。其中的下划线 `_`，是个总括值，a catchall value；在这个示例中，我们表明我们打算匹配所有 `Err` 值，无论他们内部有着何种信息。因此，程序将执行第二个支臂的代码，`continue`，这告诉程序，要前往 `loop` 的下一次迭代，而请求另一个猜数。因此，程序会有效地忽略 `parse` 可能遇到的所有错误！


现在，这个程序中的一切都应按预期运行。我们来试一下：


```console
$ cargo run
   Compiling guessing_game v0.1.0 (/home/hector/rust-lang-zh_CN/projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/guessing_game`
猜猜这个数!
秘密数字是：88
请输入你的猜数。
adb
请输入你的猜数。
88
你猜的是: 88
你赢了！
```

太棒了！只需最后一个微小调整，我们即将完成这个猜数游戏了。回想一下这个程序仍在打印秘密数字。这对测试来说效果很好，但却毁掉了这个游戏。咱们来删除那个输出秘密数字的 `println!`。清单 2-6 给出了最终代码。


文件名：`src/main.rs`

```rust
use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println! ("猜猜这个数!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println! ("请输入你的猜数。");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("读取行失败！");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println! ("你猜的是: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println! ("太小！"),
            Ordering::Greater => println! ("太大！"),
            Ordering::Equal => {
                println! ("你赢了！");
                break
            }
        }
    }
}
```

*<a name="list-2-6">清单 2-6</a>：完整的猜数游戏代码*


至此，咱们已成功构建了这个猜数游戏。恭喜！


## 本章小结


这个项目以实践的方式，向咱们介绍了许多新的 Rust 概念：

- `let`
- `match`
- 函数
- 外部代码箱的使用等等。


在接下来的几章中，咱们将更详细地了解这些概念。

- 第 3 章涵盖了大多数编程语言都有的一些概念，如变量、数据类型和函数等，并展示了如何在 Rust 中使用他们；
- 第 4 章探讨了所有权，这是 Rust 不同于其他语言的一个特性；
- 第 5 章会讨论结构体及方法语法；
- 第 6 章解释了枚举的工作原理。


（End）

