# 编写猜数游戏

**Programming a Guessing Game**


咱们来一起通过一个实践项目，了解 Rust 吧！本章通过演示如何在一个实际程序中，如何运用他们，从而介绍一些常见 Rust 概念。咱们将了解 `let`、`match`、方法、关联函数、外部代码箱等！在接下来的章节中，我们将更详细地探讨这些概念。在本章中，咱们将只练习这些基本知识。

我们将实现一个经典的初学者编程问题：猜数游戏。其原理如下：程序将随机生成一个介于 1 和 100 之间的整数。然后，程序会提示玩家，输入一个猜测值。猜测值输入后，程序会显示猜测值是过低还是过高。如猜测正确，游戏将打印一条祝贺信息并退出。


## 建立一个新项目

**Setting Up a New Project**


要建立一个新项目，请进入咱们在第 1 章中，创建的 `projects` 目录，并使用 Cargo 创建一个新项目，像这样：


```console
$ cargo new guessing_game
$ cd guessing_game
```

第一条命令，`cargo new`，取项目名字（`guessing_game`）作为第一个参数。第二条命令会更改到新项目的目录。

查看生成的 `Cargo.toml` 文件：


文件名：`Cargo.toml`

```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

正如咱们在第 1 章中所看到的，`cargo new` 会给咱们生成一个 "Hello, world!" 程序。请查看 `src/main.rs` 文件：


文件名：`src/main.rs`

```rust
fn main() {
    println! ("Hello, world!");
}
```

现在我们来使用 `cargo run` 命令，在同一步骤编译并运行这个 "Hello, world!" 程序：

```console
$ cargo run
   Compiling guessing_game v0.1.0 (/home/peng/rust-lang/projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.44s
     Running `target/debug/guessing_game`
Hello, world!
```

当咱们需要在某个项目快速迭代，就像我们在这个游戏中将要做的，在进入下一迭代之前，快速测试每一次迭代时，`run` 这个命令就会派上用场。

请重新打开 `src/main.rs` 文件。咱们将在这个文件中，编写所有代码。


## 处理一个猜数

**Processing a Guess**


猜数游戏程序的第一部分，将请求用户输入，处理输入信息，并检查输入信息是否符合预期形式。首先，我们将允许玩家输入一个猜测。请在 `src/main.rs` 中，输入清单 2-1 中的代码。


文件名：`src/main.rs`

```rust
use std::io;

fn main() {
    println! ("请猜这个数！");

    println! ("请输入你的猜数。");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("读取行失败/failed to read line");

    println! ("你猜的是：{guess}");
}
```

*清单 2-1，从用户处获取一个猜数并将其打印出来的代码*


这段代码包含了大量信息，所以我们来逐行查看。要获取用户输入，然后将结果打印输出，我们就需要将 `io` 这个输入/输出库，带入作用域。`io` 库来自标准库，即 `std`：


```rust
use std::io;
```


默认情况下，Rust 在标准库中定义了一组，其会带入到每个程序作用域中的项目。这组项目被称为 *前奏，prelude*，咱们可以在 [标准库文档](https://doc.rust-lang.org/std/prelude/index.html) 中，查看他当中的全部项目。

如果咱们打算使用的某个类型不在前奏中，那么就必须用一条 `use` 语句，显式地将该类型带入作用域。使用 `std::io` 库，提供到咱们许多有用功能，包括接受用户输入的能力。

正如咱们在第 1 章所看到的，`main` 函数是该程序的入口，the entry point into the program：


```rust
fn main() {
```

`fn` 语法声明了一个新函数；括号 `()` 表明没有参数；花括号，`{`，开启了该函数的。

同样如同咱们在第 1 章中所掌握的，`println!` 是个将字符串打印到屏幕上的宏，a macro：


```console
    println! ("猜出这个数来！");

    println! ("请输入你猜的数。");
```

这段代码打印出说明游戏是什么，以及要求用户输入的提示信息。


### 使用变量存储值

**Storing Values with Variables**


接下来，我们将创建一个 *变量，variable*，来存储用户输入，就像这样：


```rust
    let mut guess = String::new();
```


现在，程序开始变得有趣起来！在这短短一行中，发生了很多事情。我们使用 `let` 语句，创建这个变量。下面是另一个例子：


```rust
let apples = 5;
```

这一行创建了个名为 `apples` 的新变量，并将其与值 5 绑定。在 Rust 中，变量默认是不可变的，immutable，这意味着一旦我们赋给变量某个值，该值就不会改变。我们将在第 3 章 [“变量和可变性”](programming_concepts/variables_and_mutability.md) 小节中，详细讨论这一概念。要使某个变量可变，我们就要在该变量的名字前，添加 `mut` 关键字：


```rust
let apples = 5; // 不可变（immutable）
let mut bananas = 5; // 可变（mutable）
```

> **注意**：其中的 `//` 语法，会开始一条持续到行尾的注释。Rust 会忽略注释中的所有内容。我们将在 [第 3 章](programming_concepts/comments.md) 详细讨论注释。


回到猜数游戏程序，咱们现在知道，`let mut guess` 将引入一个名为 `guess` 的可变变量。等号（`=`）告诉 Rust，我们现在打算给变量绑定某个东西。等号右边是 `guess` 要被绑定到的，调用 `String::new` 函数的结果，该函数会返回一个 `String` 的新实例。而 [`String`](https://doc.rust-lang.org/std/string/struct.String.html) 是标准库所提供的一种字符串类型，是可增长的、UTF-8 编码的文本。

`::new` 代码行中的 `::` 语法，表明 `new` 是 `String` 类型的一个关联函数。所谓 *关联函数，associated function*，是实现于某个类型（此示例中即 `String`）上，实现的一个函数。这个 `new` 函数，会创建一个新的空字符串。在许多类型上，咱们都会发现一个 `new` 函数，因为他是个那些构造某种新值函数的通用名称。

在那个 `::new` 代码行中的 `::` 语法，表示其中的 `new` 是 `String` 类型的一个关联函数（an associated funtion of the `String` type）。至于 *关联函数（associated function）*，指的是应用到某种类型上的函数，在此实例中，类型就是 `String` 了。这个 `new` 函数创建了一个新的、空空的字符串。由于`new` 是个构造某种新值的常见函数，因此在许多类型上，都将找到 `new` 函数。

总的来说，`let mut guess = String::new();` 这行，创建了当前绑定了一个新的、空的 `String` 实例的一个可变变量。呼！


### 接收用户输入

**Receiving User Input**


回顾一下，在程序的第一行，我们使用 `use std::io;`，包含了标准库中的输入/输出功能。现在，我们将调用 `io` 模组中，将允许咱们处理用户输入的 `stdin` 函数：


```rust
    io:stdin()
        .readline(&mut guess)
```

如果我们没有在程序开头，使用 `use std::io;` 导入 `io` 库，我们仍然可以通过将此函数调用，写成 `std::io::stdin` 来使用这个函数。`stdin` 函数会返回 [`std::io::Stdin`](https://doc.rust-lang.org/std/io/struct.Stdin.html) 的一个实例，而这是一种表示终端标准输入句柄的类型，a type that represents a handle to the standard input for your terminal。

接下来，`.read_line(&mut guess)` 这一行，调用了该标准输入句柄上的 `read_line` 方法，来获取用户输入。我们还将 `&mut guess` 作为参数，传递给 `read_line`，告诉他将用户输入的内容，存储在哪个字符串中。`read_line` 的全部工作，就是接收用户输入标准输入的内容，并将其追加到某个字符串中（不会覆盖其内容），因此我们要将该字符串，作为参数传递给他。这个字符串参数，必须是可变的，这样这个方法才能更改该字符串的内容。

其中的 `&`，表示该参数是个 *引用，reference*，其提供了一种，让咱们的代码多个部分，在无需多次将某个数据复制到内存中的情况下，即可访问该数据的方法。引用是一项复杂的特性，而 Rust 的主要优势之一，就是引用的使用，既安全又简单。对于完成现在这个程序，咱们并不需要知道很多的这些细节。现在，咱们只需知道引用与变量一样，默认情况下是不可变的。因此，咱们需要写下 `&mut guess` 而不是 `&guess`，来使其可变。(第 4 章将更详细地解释引用）。


### 使用 `Result` 处理潜在失效

**Handle Potential Failure with `Result`**


我们仍在研究这行代码。我们现在讨论的是第三行文字，但请注意，他仍然是单个逻辑行代码的一部分。下一部分，便是这个方法：


```rust
        .expect("读取输入失败");
```


我们本可以将这段代码写成：


```rust
io::stdin().read_line(&mut guess).expect("读取输入失败");
```

不过，一个长行难于阅读，所以最好将其分开。在咱们使用 `.method_name()` 语法，调用某个方法时，引入一个换行符，以及另外的空白，来帮助拆分长行，通常是明智之举。现在我们来讨论一下，这一行完成了什么。

如早先曾提到的，`read_line` 会将用户输入的任何内容，放入我们传给他的字符串中，但他还会返回一个 `Result` 值。[`Result`](https://doc.rust-lang.org/std/result/enum.Result.html) 是个 [*枚举，enumeration*](Ch06_Enums_and_Pattern_Matching.md)，通常称为 `enum`，是可处于多种可能状态之一的一种类型。我们称每种可能状态，为一个 *变种，variant*。

[第 6 章](Ch06_Enums_and_Pattern_Matching.md) 将详细介绍枚举。这些 `Result` 类型的目的，是要编码错误处理信息。

`Result` 变体，为 `Ok` 和 `Err`。`Ok` 变体表示操作成功，且 `Ok` 内是成功生成的值。`Err` 变体表示操作失败，同时 `Err` 包含了操作如何失败，或为何失败的信息。

与任何类型的值一样，`Result` 类型的值，也有定义于其上的一些方法。`Result` 的实例，有个咱们可以调用的 `expect` 方法。如果 `Result` 实例是个 `Err` 值，`expect` 就将导致程序崩溃，并显示咱们作为参数传递给 `expect` 那条信息。在 `read_line` 方法返回了一个 `Err` 时，那么很可能是底层操作系统出错所致。在这个 `Result` 实例是个 `Ok` 值时，`expect` 将取得那个 `Ok` 持有的返回值，并将该值返回给咱们，以便咱们可以使用他。在本例中，该值就是用户输入的字节数。

如果咱们不调用 `expect`，这个程序会编译，但会收到警告：


```console
$ cargo build                                                                                    ✔
   Compiling guessing_game v0.1.0 (/home/peng/rust-lang/projects/guessing_game)
warning: unused `Result` that must be used
  --> src/main.rs:10:5
   |
10 | /     io::stdin()
11 | |         .read_line(&mut guess);
   | |_______________________________^
   |
   = note: `#[warn(unused_must_use)]` on by default
   = note: this `Result` may be an `Err` variant, which should be handled

warning: `guessing_game` (bin "guessing_game") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.48s
```

Rust 警告说咱们不曾使用 `read_line` 返回的那个 `Result` 值，表明程序还没有处理可能出现的错误。

消除这条警告的正确方法，是着手编写出错误处理代码，但在我们的例子中，我们只打算在某个问题出现时，让程序崩溃，因此咱们可以使用 `expect`。咱们将在 [第 9 章](error_handling/result.md) 中，学习如何从错误中恢复。


### 使用 `println!` 占位符打印值

**Printing Values with `println!` Placeholders**

这段代码中，除了结尾的大括号，到目前为止就只有一行需要讨论了：


```rust
    println! ("你猜的数是：{guesss}");
```

这一行会打印现在包含了用户输入的那个字符串。其中的 `{}` 花括号组，是个占位符：可以把 `{}` 想象成一对用来固定某个值于某处的小蟹钳。在打印某个变量的值时，变量名可以放在这对花括号内。在打印表达式的计算结果时，就要在格式字符串中，放置空的大括号，然后在格式字符串后，添加以逗号分隔的表达式列表，并按照相同的顺序打印到各个空的大括号占位符中。在一次 `println!` 的调用中，打印一个变量和一个表达式的结果，将如下所示：


```rust
let x = 5;
let y = 10;

println! ("x = {x} 而 y + 2 = {}", y + 2);
```

此代码将打印出 `x = 5 而 y + 2 = 12`。


### 测试第一部分

**Testing the First Part**


我们来测试一下，这个猜数游戏的第一部分。请使用 `cargo run` 运行他：


```console
$ cargo run                ✔
   Compiling guessing_game v0.1.0 (/home/peng/rust-lang/projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.68s
     Running `target/debug/guessing_game`
猜出这个数来！
请输入你猜的数。
6
你猜的数为：6
```

至此，这个游戏的第一部分已经完成：我们从键盘获取输入，然后打印出来。


## 生成秘密数字

**Generating a Secret Number**


接下来，我们需要生成一个用户将尝试猜测的秘密数字。秘密数字应每次都不一样，这样游戏才会有趣，才能玩多次。我们将使用 1 到 100 之间的某个随机数，这样游戏就不会太难。Rust 尚未在其标准库中，包含随机数功能。不过，Rust 团队提供了一个包含上述功能的 [`rand` 代码箱](https://crates.io/crates/rand)。


### 使用代码箱获得更多功能

**Using a Crate to Get More Functionality**


请记住，代码箱是一些 Rust 源代码文件的集合。我们正在构建的项目，是个 *二进制代码箱，binary crate*，这是个可执行代码箱。而 `rand` 代码箱，则是个 *库代码箱，library crate*，其中包含的代码，旨在用于其他程序，而不能在其自身上执行。

Cargo 的外部板块的协调能力，正是 Cargo 的真正亮点所在。在编写用到 `rand` 的代码之前，我们需要修改那个 `Cargo.toml` 文件，将 `rand` 代码箱作为一个依赖项。现在请打开该文件，在 Cargo 为咱们创建的 `[dependencies]` 小节标题下，添加下面一行。请务必使用这个版本号，准确指定 `rand`，否则本教程中的代码示例，可能无法运行：


文件名：`Cargo.toml`

```toml
rand = "0.8.5"
```

在这个 `Cargo.toml` 文件中，某个头部之后的所有内容，都是该小节的一部分，一直持续到另一小节开始。在 `[dependencies]` 中，咱们告诉 Cargo，咱们的项目依赖于哪些外部代码箱，以及咱们需要这些代码箱的哪些版本。在本例中，我们使用语义版本说明符 `0.8.5`，指定了 `rand` 这个代码箱。Cargo 能够理解语义的版本编号，Semantic Versioning，有时也称为 *SemVer*，这是一种编写版本号的标准。`0.8.5` 实际上是 `^0.8.5` 的缩写，表示至少是 `0.8.5` 但低于 `0.9.0` 的任何版本。

Cargo 会认为，这些版本具有与 `0.8.5` 版兼容的公共 API，而这一规范，确保了咱们将得到，仍可与本章中的代码编译的最新补丁发布。任何 `0.9.0` 或更高版本，都不能保证有着与接下来的示例中，用到的相同 API。

现在，在不修改任何代码的情况下，我们来构建一下这个项目，如清单 2-2 所示。


```console
$ cargo build
    Updating crates.io index
  Downloaded ppv-lite86 v0.2.17
  Downloaded rand_chacha v0.3.1
  Downloaded cfg-if v1.0.0
  Downloaded rand_core v0.6.4
  Downloaded getrandom v0.2.11
  Downloaded rand v0.8.5
  Downloaded libc v0.2.150
  Downloaded 7 crates (910.0 KB) in 4.63s
   Compiling libc v0.2.150
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.17
   Compiling getrandom v0.2.11
   Compiling rand_core v0.6.4
   Compiling rand_chacha v0.3.1
   Compiling rand v0.8.5
   Compiling guessing_game-xfossdotcom v0.1.1 (/home/chat/rust-lang-zh_CN/projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 7.08s
```

*清单 2-2：将 rand 代码箱添加为依赖项后运行 `cargo build` 的输出**


咱们可能会看到一些不同的版本号（但他们都与代码兼容，这要归功于 SemVer！）和不同的一些行（取决于操作系统），而且这些行的顺序也可能不同。

当我们包含了某个外部依赖项时，Cargo 会从作为 [Crates.io](https://crates.io/) 上数据的一份拷贝的 *登记簿，registry* 中，获取该依赖项所需的所有内容的最新版本。Crates.io 是 Rust 生态系统中的人们，发布开源 Rust 项目供他人使用的地方。

更新登记簿后，Cargo 会检查 `[dependencies]` 小节，并下载列出的任何尚未下载的代码箱。在本例中，虽然我们只将 `rand` 列为依赖项，但 Cargo 还抓取了 `rand` 运作所依赖的其他代码箱。下载完这些代码箱后，Rust 会对他们进行编译，然后使用这些可用依赖项，编译项目。

如果咱们不做任何修改，就立即再次运行 `cargo build`，那么除了 `Finished` 那行外，咱们不会得到任何输出。Cargo 知道他已经下载并编译了依赖项，而咱们也没有在 `Cargo.toml` 文件中，对依赖项做任何修改。Cargo 也知道咱们没有修改代码，所以也不会重新编译项目。无事可做，他就直接退出了。


```console
$ cargo build                                                            ✔
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
```


如果咱们打开 `src/main.rs` 文件，进行一些简单的更改，然后保存并再次构建，咱们将只会看到两行输出：


```console
cargo build                                                            ✔
   Compiling guessing_game v0.1.0 (/home/peng/rust-lang/projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.50s
```


这几行显示，Cargo 只会根据咱们对 `src/main.rs` 文件的微小改动，来更新构建。咱们的依赖依赖并没有改变，因此 Cargo 知道，他可以重复使用已经下载并编译好的那些依赖项。


### 使用 `Cargo.lock` 文件确保可重现的构建

**Ensuring Reproducible Builds with the `Cargo.lock` File**


Cargo 有着一种可以确保咱们，或其他人每次构建代码时，都能重建出相同产物的机制： Cargo 将只使用咱们所指定的依赖项版本，除非咱们另有指示。例如，下周 `rand` 代码箱的 `0.8.6` 版本将发布，该版本包含了一个重要的错误修复，但同时也包含了一个会破坏咱们代码的回退。为了处理这个问题，Rust 会在咱们第一次运行 `cargo build` 时，创建 `Cargo.lock` 文件，所以在 `guessing_game` 目录下，我们现在会有这个文件。

当咱们首次构建某个项目时，Cargo 会计算出符合条件依赖项的全部版本，然后将其写入 `Cargo.lock` 文件。在咱们以后再构建项目时，Cargo 就会发现 `Cargo.lock` 文件的存在，并会使用其中指定的版本，而不会再重新计算版本。这样，咱们就能自动进行可重现的构建。换句话说，由于有了 `Cargo.lock` 文件，在咱们明确升级之前，咱们的项目将保持在 `0.8.5` 版本。由于 `Cargo.lock` 文件对于可重现性构建非常重要，因此他通常会与项目中的其他代码一起，进入源代码控制系统。


### 更新代码箱来获取新版本

**Updating a Crate to Get a New Version**


当咱们确实打算更新某个代码箱时，Cargo 提供了 `update` 命令，他会忽略 `Cargo.lock` 文件，并找出所有符合咱们在 `Cargo.toml` 中所要求的最新版本。然后，Cargo 会把这些版本写入 `Cargo.lock` 文件。否则，默认情况下，Cargo 只会查找大于 `0.8.5` 且小于 `0.9.0` 的版本。如果 `rand` 代码箱发布了 `0.8.6` 和 `0.9.0` 这两个新版本，那么运行 `cargo update` 时就会看到下面的内容：


```console
$ cargo update
    Updating crates.io index
    Updating rand v0.8.5 -> v0.8.6
```


Cargo 会忽略 `0.9.0` 的版本。此时，咱们还会注意到，`Cargo.lock` 文件中的一处变化，即咱们现在使用的 `rand` 代码箱，版本为 `0.8.6`。要使用 `rand` 或 `0.9.x` 系列中的任何版本，咱们必须更新 `Cargo.toml` 文件，使其看起来像这样：


```toml
[dependencies]
rand = "0.9.0"
```

在咱们下次运行 `cargo build` 时，Cargo 会更新可用代码箱的登记簿，the registry of creates available，并根据咱们所指定的新版本，重新计算咱们的 `rand` 需求。

关于 [Cargo](http://doc.crates.io/) 及 [其生态](http://doc.crates.io/crates-io.html)，还有很多内容要讲，我们将在第 14 章进行讨论，但现在，这就是咱们需要了解的全部内容。Cargo 让重用库变得非常容易，因此 Rustaceans 可以编写出，由多个包组合而成的小型项目。


### 生成随机数

**Generating a Random Number**


咱们来开始使用 `rand`，生成一个要猜的数字。下一步是要更新 `src/main.rs`，如下清单 2-3 所示。


文件名：`src/main.rs`

```rust
use std::io;
use rand::Rng;

fn main() {
    println! ("请猜数！");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println! ("秘密数字为：{secret_number}");

    println! ("请输入你的猜数。");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("读取行失败/failed to read line");

    println! ("你猜的是：{guess}");
}
```

*清单 2-3：添加代码以生成随机数*


首先，我们添加 `use rand::Rng;` 这行。`Rng` 特质，the `Rng` trait，定义了随机数生成器，所实现的那些方法，而这个特质，必须位于咱们要用到那些方法的作用域中。第 10 章将详细介绍特质。

接下来，我们在中间添加两行。在第一行中，我们调用了给到我们要用到随机数生成器的 `rand::thread_rng` 函数：一个相对于当前执行线程本地的，由操作系统提供种子的随机数发生器。然后，我们调用了这个随机数生成器上的 `gen_range` 方法。该方法由咱们已使用 `use rand::Rng;` 语句，带入到作用域的 `Rng` 特质所定义。`gen_range` 方法，取一个范围表达式作为参数，并生成该范围内的一个随机数。我们这里使用的范围表达式类别，形式为 `start...=end`，并包含下上边界，因此我们需要指定 `1...=100`，以请求一个介于 1 和 100 之间的数字。


> **注意**：咱们不会只要知道使用哪个特质、调用某个代码箱的哪些方法与函数，因此每个代码箱，都有使用说明文档。Cargo 的另一个特色便是，运行 `cargo doc --open` 命令，就会在本地构建出咱们所有依赖项提供的文档，并在浏览器中打开。例如，如果咱们对 `rand` 代码箱的其他功能感兴趣，那么请运行 `cargo doc --open`，并点击左侧边栏中的 `rand`。


第二新的行，会打印秘密数字。这在我们开发程序时很有用，可以用来测试程序，但我们会在最终版本中删除他。如果程序一开始就打印出答案，那就不算是个游戏了！

请试着运行几次程序：


```console
$ cargo run                                                           ✔  4s 
   Compiling guessing_game v0.1.0 (/home/peng/rust-lang/projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.54s
     Running `target/debug/guessing_game`
猜出这个数来！
随机生成的秘密数字为：40
请输入你猜的数。
86
你猜的数为：86

$ cargo run                                                           ✔  9s 
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/guessing_game`
猜出这个数来！
随机生成的秘密数字为：30
请输入你猜的数。
27
你猜的数为：27

```

咱们应得到不同的随机数字，且他们都应是 1 到 100 之间的数字。干得好！


## 将猜数与秘数相比较

**Comparing the Guess to the Secret Number**


现在我们有了用户输入和随机数，我们可以对他们进行比较。该步骤如下清单 2-4 所示。请注意，这段代码还不能编译，我们将对此进行说明。


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

*清单 2-4：对比较两个数字可能的返回值进行处理*


首先，我们添加了另一条 `use` 语句，从标准库中，引入名为 `std::cmp::Ordering` 的类型。`Ordering` 类型是另一个枚举，并具有 `Less`、`Greater` 和 `Equal` 三种变体。这正是在比较两个值时，可能出现的三种结果。

然后，我们在底部，添加了用到这个 `Ordering` 类型的五个新行。`cmp` 这个方法，会比较两个值，并可以在任何可被比较的项目上调用。他会取一个到咱们打算比较的任何值的引用：这里他是将 `guess` 与 `secret_number` 进行比较。然后，他会返回我们通过那条 `use` 语句，带入作用域的 `Ordering` 枚举的某个变种。我们使用了一个 `match` 表达式，根据以 `guess` 和 `secret_number` 中的值调用 `cmp` 时，所返回的何种 `Ordering` 变体，来决定下一步的操作。

`match` 表达式由数个 *支臂，arms* 组成。而一个支臂则由一个要与之匹配的 *模式，pattern*，以及在给到 `match` 的值，符合该支臂的模式时，要运行的代码组成。Rust 会取给到 `match` 的值，并依次查看每个支臂的模式。模式与这种 `match` 结构，是 Rust 的强大功能：二者可以让咱们，表达出代码可能遇到的各种情况，并确保咱们能处理全部的这些情况。第 6 章和第 18 章，将分别详细介绍这些特性。

咱们来以这里用到的这个 `match` 表达式，看一个示例。假设用户猜的是 50，而这次随机生成的秘密数字是 38。

当代码将 50 与 38 比较时，`cmp` 方法将返回 `Ordering::Greater`，因为 50 大于 38。这个 `match` 表达式就会得到 `Ordering::Greater` 这个值，并开始检查每个支臂的模式。他会查看第一个支臂的模式 `Ordering::Less`，发现值 `Ordering::Greater` 与 `Ordering::Less` 不匹配，因此他会忽略该支臂的代码，而转到下一支臂。下一支臂的模式是 `Ordering::Greater`，这 *确实* 匹配 `Ordering::Greater`！该支臂中的相关代码将执行，并打印 `太大！` 到屏幕。这个 `match` 表达式在第一次成功匹配后，就会结束，因此在这种情况下，其不再查看最后一个支臂。

然而，清单 2-4 中的代码还无法编译。咱们来尝试一下：


```console
$ cargo build                                                         ✔
   Compiling guessing_game v0.1.0 (/home/peng/rust-lang/projects/guessing_game)
error[E0308]: mismatched types
  --> src/main.rs:22:21
   |
22 |     match guess.cmp(&secret_number) {
   |                     ^^^^^^^^^^^^^^ expected struct `String`, found `i32`
   |
   = note: expected reference `&String`
              found reference `&i32`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `guessing_game` due to previous error
```


错误的核心，表明存在 *不匹配的类型，mismatched types*。Rust 有着强大的，静态类型系统。不过，他也有着类型推断，Rust has a strong, static type system. However, it also has type inference。当我们写下 `let mut guess = String::new()` 时，Rust 就能推断出，`guess` 应是个 `String`，而未曾让我们写下类型。另一方面，`secret_number` 是一个数字类型。Rust 的一些数字类型，可以有着介于 1 和 100 之间的某个：`i32`，某个 32 位的数字；`u32`，某个无符号的 32 位数字；`i64`，某个 64 位的数字；以及其他类型。除非另有说明，否则 Rust 默认会使用 `i32`，这即为 `secret_number` 的类型，除非在其他地方，添加了导致 Rust 推断出不同的数值类型的类型信息。上面这个报出的原因，是 Rust 无法比较字符串和数字类型。

最后，我们打算将程序读取的字符串输入，转换为某个真正的数字，这样咱们就可以将其与秘密数字，进行数值比较。我们要通过在那个 `main` 函数主体中，添加下面这行，完成这一点：

文件名：`src/main.rs`

```rust
    // --跳过前面的代码--

    let mut guess: String = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("读取行失败......");

    let guess: u32 = guess.trim().parse().expect("请输入一个数字！");

    println! ("你猜的数为：{}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println! ("太小了！"),
        Ordering::Greater => println! ("太大了！"),
        Ordering::Equal => println! ("你赢了！"),
    }
```

该行为：

```rust
let guess: u32 = guess.trim().parse().expect("请输入一个数字！");
```

我们创建了一个名为 `guess` 的变量。但是等等，程序中不是已经有一个名为 `guess` 的变量了吗？是有的，但好在 Rust 允许我们用一个新值，对 `guess` 的前一个值进行遮蔽处理。*遮蔽特性，shadowing* 允许咱们，重复使用这个 `guess` 变量名，而不必被迫创建出，诸如 `guess_str` 和 `guess` 这样的两个唯一变量。我们将在 [第 3 章](programming_concepts/variables_and_mutability.md#遮蔽shadowing) 中详细介绍这一功能，而现在我们要知道，当咱们打算将某个值，从一种类型转换为另一类型时，就经常会用到这一特性。

我们将这个新变量，绑定到 `guess.trim().parse()` 这个表达式。表达式中的 `guess`，指的是包含了作为字符串的输入的那个原始 `guess` 变量。某个 `String` 实例上的 `trim` 方法，将消除开头和结尾的空白，我们必须这样做才能将字符串与 `u32` 进行比较，而 `u32` 只能包含数字数据。用户必须按下回车键，来满足 `read_line` 并输入他们的猜数，这会添加一个换行符到输入字串。例如，如果用户输入 5 并按回车键，`guess` 就会看起来是这样的：`5\n`。`\n` 表示 “换行/newline”。(在 Windows 系统中，按下回车键会产生是回车和换行，即 `\r\n`）。<sup>译注 1</sup> `trim` 方法可以去掉 `\n` 或 `\r\n`，结果就只有 `5` 了。

> **译注 1**：这也是为何先前的代码：
>
```rust
    let bytes = io::stdin()
        .read_line(&mut guess)
        .expect("读取行失败/failed to read line");
```
>
> 在 Windows 的 MSYS2 上运行时，`bytes` 的输出始终会比咱们看到的字符串，要多两个字节的原因。


[字符串上的 `parse` 方法](https://doc.rust-lang.org/std/primitive.str.html#method.parse)，可将字符串转换为另一类型。在这里，我们要用他，将字符串转换为数字。我们需要使用 `let guess: u32`，告诉 Rust 我们想要的确切数字类型。`guess` 后面的冒号（`:`），告诉 Rust 我们将注解这个变量的类型。Rust 有几种内置的数字类型；这里所看到的 `u32`，是一种无符号的 32 位整数。对于小的正数来说，这是一种不错的默认选择。咱们将在 [第 3 章](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-types)，了解其他数字类型。

此外，本示例程序中的这个 `u32` 注解，及那个与 `secret_number` 的比较，意味着 Rust 将推断出 `secret_number` 也应是个 `u32`。因此，现在这个比较，将是在两个相同类型值之间的了！

`parse` 这个方法，只适用于逻辑上可以转换成数字的那些字符，因此很容易出错。例如，如果字符串包含着 `A👍%`，就无法将其转换为数字。因为其可能会失败，所以 `parse` 方法会返回一个结果类型，就像 `read_line` 方法一样（早先曾在 [“使用 `Result` 处理潜在失败”](#使用-result-处理潜在失效) 小节中讨论过）。我们将再次通过使用 `expect` 方法，以同样方式处理这个 `Result`。如果 `parse` 因无法从那个字符串，创建出一个数字而返回 `Err` 的 `Result` 变种，则 `expect` 这个调用，将导致游戏崩溃，并打印出我们给到他的信息。如果 `parse` 能成功将那个字符串转换为数字，他将返回 `Result` 的 `Ok` 变种，而 `expect` 将从这个 `Ok` 值，返回我们想要的数字。

现在咱们来运行一下这个程序：


```console
$ cargo run                                                       101 ✘  3s 
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/guessing_game`
猜出这个数来！
随机生成的秘密数字为：66
请输入你猜的数。
   76
你猜的数为：76
太大了！
```


不错！即使在猜数前添加了空格，程序仍然能判断出，用户猜测的数字是 76。请多运行几次程序，验证在不同输入情况下的不同行为：猜对数字、猜的数字太大、猜的数字太小等等。

我们现在已经让这个游戏的大部分工作了，但用户只能猜一次数。我们就来通过添加一个循环，改变这种情况！


## 通过循环实现多次猜数

**Allowing Multiple Guesses with Looping**


`loop` 关键字会创建出一个无限循环。我们将添加一个让用户有更多机会猜出数字的循环：


文件名：`src/main.rs`

```rust
    // --跳过--

    println! ("随机生成的秘密数字为：{}", secret_number);

    loop {
        println! ("请输入你猜的数。");

        // --跳过--

        match guess.cmp(&secret_number) {
            Ordering::Less => println! ("太小！"),
            Ordering::Greater => println! ("太大！"),
            Ordering::Equal => println! ("你赢了！"),
        }
    }
}
```

正如咱们所看到的，我们把从猜测输入提示开始的所有内容，都移到了一个循环中。请务必将循环内的那些行，缩进另外四个空格，然后再次运行程序。这个程序现在将一直不停要求另一个猜数，这实际上引入了一个新问题。用户似乎无法退出！

用户可以始终通过使用键盘快捷键 `ctrl-c` 来中断这个程序。但还有一种方法可以摆脱这个贪得无厌的怪物，正如 [“将猜测与秘密数字进行比较”](#将猜数与秘数相比较) 小节，`parse` 的讨论中所提到的：如果用户输入的答案不是数字，这个程序就会崩溃。我们可以利用这一点，允许用户退出，如下所示：


```console
$ cargo run
   Compiling guessing_game v0.1.0 (/home/peng/rust-lang/projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.53s
     Running `target/debug/guessing_game`

---猜出这个数来！---
请输入你猜的数。（ ‘Q/quit’ 退出游戏）
50
你猜的数为：50
太小了！
请输入你猜的数。（ ‘Q/quit’ 退出游戏）
75
你猜的数为：75
太大了！
请输入你猜的数。（ ‘Q/quit’ 退出游戏）
62
你猜的数为：62
太大了！
太小了！
请输入你猜的数。（ ‘Q/quit’ 退出游戏）
55
你猜的数为：55
你赢了！

---猜出这个数来！---
请输入你猜的数。（ ‘Q/quit’ 退出游戏）
quit
thread 'main' panicked at '请输入一个数字！: ParseIntError { kind: InvalidDigit }', src/main.rs:25:51
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

输入 `quit` 将退出这个游戏，但咱们会发现，输入任何其他非数字的输入，也会退出游戏。至少可以说，这是次优的；我们希望在猜中正确数字后，这个游戏也会停止。


### 猜对后的退出

**Quitting After a Correct Guess**


我们来通过添加一个 `break` 语句，将这个游戏编程为在用户获胜后退出：


文件名：`src/main.rs`

```rust
        // --跳过--

        match guess.cmp(&secret_number) {
            Ordering::Less => println! ("太小！"),
            Ordering::Greater => println! ("太大！"),
            Ordering::Equal => {
                println! ("你赢了！");
                break;
            },
        }
    }
}
```

在 `你赢了！` 后面添加 `break` 这行，令到程序在用户猜对秘密数字时，退出那个循环。退出那个循环，也意味着退出这个程序，因为该循环是 `main` 的最后一部分。

> **译注**：这里有个有趣的地方，`break` 后的分号可有可无，`match` 表达式最后支臂后的逗号，也是可有可无的。


### 处理无效输入

**Handling Invalid Input**


为进一步完善游戏行为，我们可以让游戏忽略非数字，这样用户就可以继续猜测，而不是在用户输入非数字时程序崩溃。通过修改 `guess` 从字符串转换为 `u32` 的行，咱们就可以做到这一点，如下清单 2-5 所示。


文件名：`src/main.rs`

```rust
        // --跳过--

        io::stdin()
            .read_line(&mut guess)
            .expect("读取行失败/failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println! ("你猜的是：{guess}");

        // --跳过--
```

*清单 2-5：忽略非数字的猜数并请求另一个猜数，而不是让程序崩溃*


我们从一个 `expect` 调用，切换到了一个 `match` 表达式，以从出错时崩溃程序，转换为处理这个出错。请记住，`parse` 回返回一个 `Result` 类型，而 `Result` 是个枚举，有 `Ok` 和 `Err` 两个变种。我们在这里使用了个 `match` 表达式，就像在在处理 `cmp` 方法的 `Ordering` 结果时一样。

如果 `parse` 成功地将那个字符串转换为数字，他将返回一个包含结果数字的 `Ok` 值。该 `Ok` 值将与第一支臂的模式匹配，而这个 `match` 表达式将只返回 `parse` 所生成并放入 `Ok` 值的那个 `num` 值。这个数字最终会出现在，我们要创建的新 `guess` 变量中。

如果 `parse` *无* 法将该字符串转化为数字，他将返回一个其中包含了更多该错误的信息的 `Err` 值。`Err` 值不会匹配到第一个 `match` 支臂中的 `Ok(num)` 模式，但会匹配到第二个支臂中的 `Err(_)` 模式。其中的下划线 `_`，是个总括值，a catchall value；在这个示例中，我们表示要匹配所有 `Err` 值，无论他们包含什么信息。因此，程序将执行第二个支臂的代码 `continue`，这告诉程序，要前往循环的下一次迭代，而请求另一个猜数。因此，实际上，程序会忽略 `parse` 可能遇到的所有错误！

现在，程序中的一切都应按预期运行。我们来试一下他：


```console
$ cargo run                                                       ✔
   Compiling guessing_game v0.1.0 (/home/peng/rust-lang/projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.57s
     Running `target/debug/guessing_game`

---猜出这个数来！---
请输入你猜的数。（ ‘Q/quit’ 退出游戏）
50
你猜的数为：50
太小了！
请输入你猜的数。（ ‘Q/quit’ 退出游戏）
75
你猜的数为：75
你赢了！
```

太棒了！最后再做一个微小的调整，我们就可以完成这个猜数游戏了。请注意，程序仍在打印出秘密数字。这对测试很有效，但却毁掉了这个游戏。咱们来删除那个输出秘密数字的 `println!`。清单 2-6 给出了最终代码。


文件名：`src/main.rs`

```rust
use rand::Rng;
use std::{cmp::Ordering, io, process};

fn main() {
    loop {
        println! ("\n---猜出这个数来！---");

        let secret_number: u32 = rand::thread_rng().gen_range(1..101);

        // println! ("随机生成的秘密数字为：{}", secret_number);

        loop {
            println! ("请输入你猜的数。（ ‘Q/quit’ 退出游戏）");

            let mut guess: String = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("读取行失败/failed to read line");

            if guess.trim().eq("Q") || guess.trim().eq("quit") { process::exit(0); }

            // let guess: u32 = guess.trim().parse().expect("请输入一个数字！");
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => { println! ("请输入一个数字！"); continue },
            };

            println! ("你猜的数为：{}", guess);

            match guess.cmp(&secret_number) {
                Ordering::Less => println! ("太小！"),
                Ordering::Greater => println! ("太大！"),
                Ordering::Equal => {
                    println! ("你赢了！");
                    break
                },
            }
        }
    }
}
```

*清单 2-6：完全的猜数游戏代码*


至此，咱们已经成功构建了这个猜数游戏。恭喜！


## 本章小结


这个项目以实践的方式，向咱们介绍了许多新的 Rust 概念：`let`、`match`、函数、外部代码箱的使用等等。在接下来的几章中，咱们将更详细地了解这些概念。第 3 章涵盖了大多数编程语言都有的概念，如变量、数据类型和函数等，并展示了如何在 Rust 中使用他们。第 4 章探讨了所有权，这是 Rust 不同于其他语言的一个特性。第 5 章会讨论结构体和方法语法，第 6 章解释了枚举的工作原理。
