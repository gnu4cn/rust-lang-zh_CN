# 编写猜数游戏

**Programming a Guessing Game**

让我们通过一起完成一个实践项目来学习 Rust 吧! 本章介绍了一些常见的 Rust 概念，告诉咱们如何在一个真正的程序中使用他们。咱们将学习到 `let`、`match`、方法、关联函数、外部代码箱等！在接下来的章节中，我们将更详细地探讨这些概念。在这一章中，咱们将只是练习基础知识。

我们将实现一个经典的初级编程问题：一个猜数游戏。他是这样工作的：程序将生成一个 `1` 到 `100` 之间的随机整数。然后他将提示玩家输入一个猜测。在输入猜测后，程序将显示猜测是否过低或过高。如果猜测正确，游戏将打印一条祝贺信息并退出。

## 建立一个新项目

要建立一个新的项目，请进入咱们在第一章中创建的 `projects` 目录，并使用 Cargo 构造一个新项目，像下面这样：


```console
$ cargo new guessing_game
$ cd guessing_game
```

第一条命令，`cargo new`，把项目名称（`guessing_game`）作为第一个参数。而第二条命令则是前往到这个新项目的目录。

看一下生成的 `Cargo.toml` 文件：

文件名：`Cargo.toml`

```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

正如咱们在第 1 章中所看到的，`cargo new` 为咱们生成了一个 "Hello, world!" 程序。请看 `src/main.rs` 文件：

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

当咱们需要快速迭代项目时，`run` 命令就会派上用场，就像我们在这个游戏中所做的那样，在继续下一个迭代之前快速测试每个迭代。

重新打开 `src/main.rs` 文件。咱们将在这个文件中编写所有的代码。

## 处理一个猜数

**Processing a Guess**

这个猜数游戏的第一部分，将请求用户的输入、处理那个输入，进而检查该输入是否有着正确格式。这里将实现玩家输入一个猜数开始。请敲入清单 2-1 中的代码到 `src/main.rs` 里去。

文件名：`src/main.rs`

```rust
use std::io;

fn main() {
    println! ("猜出这个数来！");

    println! ("请输入你猜的数。");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("读取行失败");

    println! ("你猜的数为：{}", guess);
}
```

*清单 2-1，从用户获取到一个猜数并将其打印出来的代码*

此代码包含了很多信息，那么这里就来一行一行的走一遍。要获取到用户输入并将结果打印出来，就需要将 `io` 输入/输出库带入到作用域中。而 `io` 库则是来自名为 `std` 的标准库：

```rust
use std::io;
```

默认情况下，Rust 只有少数几个定义在标准库中、由标准库带入到每个程序的项目（by default, Rust has a few items defined in the standard library that it brings into the scope of every program）。这个集合被称为 Rust 序曲（`prelude`），在 [标准库文档](https://doc.rust-lang.org/std/prelude/index.html) 中可找到全部的标准库 `prelude` 项目。

在要使用的类型，不在 Rust 序曲集合中时，就必须将那个类型，显式地通过 `use` 语句带入到作用域中。`std::io` 库的使用，提供了数个有用特性，包括接收用户输入的能力。

就跟在第 1 章所见到的那样，`main` 函数即是这个程序的进入点：

```rust
fn main() {
```

`fn` 语法声明了一个函数，而这个圆括号，`()`，表示这里没有参数，同时那个花括号，`{`，是该函数的函数体的开始。

同样与在第 1 章中所了解的那样，`println!` 是个将字符串打印到屏幕的宏（macro）：

```console
    println! ("猜出这个数来！");

    println! ("请输入你猜的数。");
```

这段代码在打印提示消息，表明该游戏是什么及正在请求用户输入。

## 使用变量保存那些值

接下来，就要创建一个 *变量（variable）* 来存储用户输入，像下面这样：

```rust
    let mut guess = String::new();
```

现在这个程序就变得有趣起来了！这小小一行，可是有很多东西。这里使用了 `let` 语句来创建这个变量。下面是另一个示例：

```rust
let apples = 5;
```

这行代码创建了一个新的名为 `apples` 的变量，并将其绑定到了值 `5`。在 Rust 中，默认变量是不可变的（immutable）。在后续第 3 章的 [变量及可变性](Ch03_Common_Programming_Concepts.md#变量及可变性) 小节，将对此概念加以讨论。而要让变量可变，就要将变量名字前加上 `mut` 关键字：

```rust
let apples = 5; // 不可变（immutable）
let mut bananas = 5; // 可变（mutable）
```

> 注意：这里的 `//` 语法，开始了一条持续到那个行结束的代码注释。Rust 会忽略注释中的全部内容。在 [第 3 章](Ch03_Common_Programming_Concepts.md#注释) 将更加详细地讨论代码注释。

回到这个猜数游戏程序，那么此刻就明白了那个 `let mut guess` 将引入一个名为 `guess` 的可变变量。而那个等号（`=`），则是告诉 Rust，现在要将某个东西绑定到该变量了。等号右边就是要绑定到 `guess` 的那个值，而这个值则是调用 `String::new` 的结果，这个 `String::new`，则又是一个返回一个 `String` 实例的函数。`String` 是由标准库提供的一个字符串类型，为一个可增大的、经 UTF-8 位编码的文本（a growable, UTF-8 encoded bit of text）。

在那个 `::new` 代码行中的 `::` 语法，表示其中的 `new` 是 `String` 类型的一个关联函数（an associated funtion of the `String` type）。至于 *关联函数（associated function）*，指的是应用到某种类型上的函数，在此实例中，类型就是 `String` 了。这个 `new` 函数创建了一个新的、空空的字符串。由于`new` 是个构造某种新值的常见函数，因此在许多类型上，都将找到 `new` 函数。

整体上看，这个 `let mut guess = String::new();` 语句，完成了一个当前绑定到新的、`String` 类型空实例的可变变量的创建。总算讲清楚了！

## 接收用户输入

回顾程序第一行上，以 `use std::io;` 从标准库所包含进来的输入/输出功能。那么现在就要调用那个 `io` 模组中的 `stdin` 函数，该函数将实现对用户输入的处理：

```rust
    io:stdin()
        .readline(&mut guess)
```

若在程序的开头不曾以 `std::io` 方式，将 `io` 库导入，那么仍然可以将该函数写作 `std::io::stdin` 形式，而对其进行使用。`stdin` 函数返回的是 `std::io::Stdin` 的实例， 而 `std::io::Stdin` 则表示终端标准输入句柄的类型（the `stdin` function returns an instance of `std::io::Stdin`, which is a type that represents a handle to the standard input for your terminal）。

接下来的代码行 `.readling(&mut guess)` 调用了标准输入句柄类型实例上的 `read_line` 方法，用于获取用户输入。这里还将 `&mut guess` 作为 `read_line` 的参数进行了传递，以告诉 `read_line` 函数，将用户输入存入到哪个字符串中。`read_line` 的整个职能，就要将用户敲入到标准输入的东西，追加到某个字符串（在不覆盖掉这个字符串内容的情况下），因此这里是将那个字符串作为参数传递的。为了这个 `read_line` 方法可以修改其内容，这里的字符串就要是可变的。

其中的 `&` 表明该参数是个 *引用（reference）*，而引用则是一种无需将数据多次拷贝到内存中的情况下，就可以实现代码多个部分对该数据进行读写的特性（注：在 C 家族语言中，`&`表示内存地址，因此 Rust 中的引用，与指针有类似之处）。引用是一项复杂特性，同时 Rust 的主要优点之一，就是安全而便利地运用引用的方式。对于完成这个猜数游戏，是不必对这些细节有过多了解的。现在要明白的是，与变量类似，引用默认也是不可变的。因此，这里就要写上 `&mut guess` 而不是 `&guess`，来令到这个到 `guess` 的引用为可变的。（第 4 章将更详细地对引用进行解释。）

## 处理潜在的带有 `Result` 的程序失效

**Handle Potential Failure with the `Result` Type**

这里还在解析代码行。尽管这里讨论的是代码文本的第三行，但他仍是单个逻辑代码行的一部分。接下来的部分是这个方法：

```rust
        .expect("读取输入失败");
```

这代码本可以写成下面这样：

```rust
io::stdin().read_line(&mut guess).expect("读取输入失败");
```

不过这样的一个长代码行，难于阅读，因此最好将其分开为多个断行。在以 `.method_name()` 语法调用方法时，通过引入另起一行及缩进，来将长的代码行拆分为短代码行，通常是明智的。下面就来说说这一行完成了什么。

前面讲过，`read_line`方法将用户敲入的东西，放入到传递给他的那个字符串中，然而 `read_line` 还会返回一个值 -- 在此实例中，返回的就是一个 `io::Result` 类型值。Rust 在他的标准库中，有着数个名为 `Result` 的类型：这是一个泛型的 `Result`，对于那些子模组都有着特定版本，比如这里的 `io::Result`。`Result` 的那些类型都属于 [枚举，enumerations](Ch06_Enums_and_Pattern_Matching.md#定义一个枚举)，枚举常被写为 `enums`，枚举有着一套被称作 *变种（variants）* 的可能值。枚举常常是和 `match` 关键字一起使用的，而 `match` 则是一种条件判断，在符合某个条件时，就可以很方便地根据枚举中的哪个变种，来执行不同代码。

第 6 章将深入涵盖到枚举数据结构。而这些 `Result` 类型的目的，则是对错误处理信息进行编码。

这个 `Result` 的变种，就是 `Ok` 与 `Err`。`Ok` 变种表示该操作是成功的，而在 `Ok` 内部，就是成功生成的值。相反 `Err` 变种，则意味着操作失败了，同时 `Err` 包含了关于操作失败的方式与原因。

`Result` 类型的那些值，跟其他任何类型都差不多，在这些值上都定义了一些方法。`io::Result` 实例，就有一个可供调用的 [`expect` 方法](https://doc.rust-lang.org/std/result/enum.Result.html#method.expect)。在这个 `io::Result` 实例是个 `Err` 变种时，那么`expect` 方法就会导致程序崩溃，并将传递给 `expect` 方法的参数显示出来。若 `read_line` 方法返回了一个 `Err`，那很可能是来自所采用操作系统错误的结果（if the `read_line` method returns an `Err`, it would likely be the result of an error coming from the underlying operating system）。而若该 `io::Result` 实例是个 `Ok` 值，那么 `expect` 方法就会取得那个 `Ok` 所保存的返回值，并只将该值返回，从而就可以使用到这个返回值。在此实例中，那个值，就是用户输入中的字节数目。

若这里没有对 `expect` 方法进行调用，那么该程序会编译，不过会收到一条告警信息：

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

Rust 警告说不曾对返回自 `read_line` 的 `Result` 值进行使用，表示程序没有对可能的错误加以处理。

消除该警告信息的正确方式，就是要老老实实地编写错误处理代码，而在这个实例中，则只要在问题发生时，崩溃掉这个程序即可，因此这里就可以使用 `expect`。在第 9 章的 [带有 Result 的可恢复错误](Ch09_Error_Handling.md#带有-result-的可恢复错误) 小节，会掌握到如何从错误中恢复过来。

## 使用 `println!` 的占位符将值打印出来

**Printing Values with `println!` Placeholders**

紧接着那个结束花括号前面，就只有剩下的一行代码要讨论了：

```rust
    println! ("你猜的数是：{}", guess);
```

这行代码是将此刻包含了用户输入的那个字符串打印出来。其中的那套花括号 `{}` ，就是一个占位符（placeholder）：请将`{}`当作是些在那个地方留有一个值的小螃蟹。使用一些这样的花括号，就可以打印出多个值来：第一套花括号保留着在格式化字符串之后列出的第一个值，第二套保留着第二个值，如此等等。一个 `println!` 调用中多个值的打印，看起来会是下面这样：

```rust
let x = 5;
let y = 10;

println! ("x = {} 同时 y = {}", x, y);
```

此代码将打印出 `x = 5 同时 y = 10`。

## 对第一部分的测试

下面就来测试一下这猜数游戏的第一部分。用 `cargo run` 运行他：

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

此刻，这游戏的第一部分就算完成了：这里正从键盘获取到输入，并随后将输入打印出来。


## 生成秘密数字

接下来，就需要生成一个用户将要试着去猜的秘密数字了。生成的秘密数字应每次都不相同，这样这游戏在多次玩的时候才有趣。为了不让这个游戏太难，这里要用一个 `1` 到 `100` 之间的随机数。Rust 在其标准库中尚未包含随机数功能。不过 Rust 团队还真的提供了一个 [`rand` 代码箱](https://crates.io/crates/rand)，这里就姑且把这样的代码箱，称之为功能吧。

### 运用代码箱（a Crate） 获取到更多功能

请记住，所谓代码箱，即为一些 Rust 源代码文件的集合。之前曾构建好的项目，则是一个 *二进制的代码箱（binary crate）*，那是个可执行程序。而 `rand` 代码箱，则是个 *库代码箱（library crate）*，这样的库代码箱，包含了预期将在其他程序中会用到的代码，同时库代码箱自身并不能执行（the `rand` crate is a *library crate*, which contains code intended to be used in other programs, and can't be executed on its own）。

Cargo 对外部代码箱的协调能力，正是 Cargo 真正闪耀之处。在能够编写出用到 `rand` 库代码箱的代码之前，先要将 `Cargo.toml` 加以修改，将 `rand` 代码箱作为依赖包含进来。打开那个文件并将下面的行，添加到底部、那个 Cargo 创建出的`[dependencies]` 小节标题之下。要确保像这里一样，带着版本号地精确指明 `rand` 代码箱，否则此教程中的代码示例就不会工作。

文件名：`Cargo.toml`

```toml
rand = "0.8.3"
```

在这 `Cargo.toml` 文件中，凡在某个标题之后的东西，都是那个小节的一部分，直到另一小节开始为止。在 `[dependencies]` 小节，告诉 Cargo 的是项目依赖了哪些外部代码箱（external crates），以及所需的这些代码箱版本。在此实例中，就指明了有着语义版本指示符（the semantic version specifier） `0.8.3` 的 `rand` 库代码箱。Cargo 能明白 [语义化版本控制（Sementic Versioning）](http://semver.org/)（有时也叫做 *`SemVer`*），这是编制版本号的标准。数字 `0.8.3` 实际上是 `^0.8.3` 的缩写，表示高于 `0.8.3` 却低于 `0.9.0` 的任何版本。Cargo 认为这些版本有着与 `0.8.3` 兼容的公共 APIs，同时这样的规定，确保了将获取到在本章中代码仍可编译的情况下，最新的补丁发布。那些 `0.9.0` 及更高的版本，无法保证接下来示例用到同样的 API。

现在，在不修改任何代码的情况下，来构建一下这个项目，如清单 2-2 所示：

```console
$ cargo build
    Updating crates.io index
  Downloaded rand v0.8.3
  Downloaded libc v0.2.86
  Downloaded getrandom v0.2.2
  Downloaded cfg-if v1.0.0
  Downloaded ppv-lite86 v0.2.10
  Downloaded rand_chacha v0.3.0
  Downloaded rand_core v0.6.2
   Compiling rand_core v0.6.2
   Compiling libc v0.2.86
   Compiling getrandom v0.2.2
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.10
   Compiling rand_chacha v0.3.0
   Compiling rand v0.8.3
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53s
```

*清单 2-2-1：在添加了作为依赖的 `rand` 代码箱后运行 `cargo build` 的输出（书上的输出）*

```console
$ cargo build                                                      ✔ 
    Updating crates.io index
  Downloaded cfg-if v1.0.0
  Downloaded rand_chacha v0.3.1
  Downloaded rand_core v0.6.3
  Downloaded getrandom v0.2.7
  Downloaded ppv-lite86 v0.2.16
  Downloaded rand v0.8.5
  Downloaded libc v0.2.126
  Downloaded 7 crates (773.8 KB) in 3.41s
   Compiling libc v0.2.126
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.16
   Compiling getrandom v0.2.7
   Compiling rand_core v0.6.3
   Compiling rand_chacha v0.3.1
   Compiling rand v0.8.5
   Compiling guessing_game v0.1.0 (/home/peng/rust-lang/projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 56.66s
```

*清单 2-2-2：在添加了作为依赖的 `rand` 代码箱后运行 `cargo build` 的输出（实际输出）*

这里可能会看到不同的一些版本号（归功于 `SemVer`，这些不同版本号将与示例代码全都兼容！）、不同的输出行（取决于所在的操作系统），以及这些行可能以不同顺序出现。

在包含外部依赖时，Cargo 会从 *登记处（registry）* 拉取到那个依赖所需的全部最新版本的代码箱，而所谓登记处，则是 [Crates.io](https://crates.io/) 数据的一份拷贝。Crates.io 是 Rust 生态中的人们，发布给其他人使用的开放源代码项目的地方。

在更新了登记处索引之后，Cargo 就对 `[denpendencies]` 小节进行查看，并下载所列代码箱中尚未下载的那些。在此实例中，尽管只列出了依赖 `rand`，Cargo 还抓取了其他 `rand` 赖以运作的一些代码箱。在下载了这些代码箱之后，Rust 会对他们进行了编译，并随后以这些可用的依赖，对这项目进行了编译。

若不做任何修改，就立即再次运行 `cargo build`，那么除了那行 `Finished` 输出之外，就再也没有别的输出了。Cargo 明白他以及下载并编译好了那些依赖，还明白尚未对 `Cargo.toml` 文件做任何修改。Cargo 还知道，这里并未对项目代码做任何修改，因此他也没有对项目代码重新编译。既然无事可做，那么他就直接退出了。

```console
$ cargo build                                                            ✔ 
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
```

若此时打开 `src/main.rs` 文件，做个细微修改，然后保存并再次构建，那么就只会看到下面这两行输出:

```console
cargo build                                                            ✔ 
   Compiling guessing_game v0.1.0 (/home/peng/rust-lang/projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.50s
```

这些行显示 Cargo 只更新了对 `src/main.rs` 文件细微修改的构建。由于依赖不曾改变，因此 Cargo 清除他可以重用那些已经下载和编译好的依赖。

### 使用 `Cargo.lock` 文件确保可重现的构建

**Ensuring Reproducible Builds with the `Cargo.lock` File**

Cargo 具备一种不论是自己还是其他要构建代码的人来说，确保每次都可以构建出同样程序组件（the same artifact）的机制：除非另有指定，Cargo 都将只使用在 `[denpendencies]` 小节中所指定的依赖版本。比如说下周 `0.8.4` 版本的 `rand` 就要释出，且那个版本包含了一个重要的错误修复，但也包含了一个会破坏咱们代码的特性撤回。为了应对这样的情况，Rust 在首次运行 `cargo build`时，就创建了 `Cargo.lock` 文件，也就是现在在 `guessing_game` 目录下就有这么个文件。

在首次构建项目时，Cargo 会找出那些依赖满足条件的所有版本，并将其写入到这 `Cargo.lock` 文件。在今后对项目进行构建时，Cargo 就会查看是否存在那个 `Cargo.lock` 文件，并使用其中所指定的那些版本，而不会再次完成找出那些版本的工作了。这样就自动实现了可重现的构建。也就是说，得益于这个 `Cargo.lock` 文件，除非显式地升级了 `rand` 的版本号，项目将保持其版本为 `0.8.3`。

### 更新代码箱来获取新版本

**Updating a Crate to Get a New Version**

在确实要更新某个代码箱时，Cargo 提供了 `update` 命令，该命令会忽略 `Cargo.lock` 文件，并找出与`Cargo.toml`中的那些规格相适合的全部最新版本。Cargo 随后将把这些版本写入到 `Cargo.lock` 文件。否则的话，默认 Cargo 就会只查找那些高于 `0.8.3` 且低于 `0.9.0` 的版本。在 `rand` 库代码箱已发布了两个新的 `0.8.4` 和 `0.9.0` 版本时，此时若运行 `cargo update`，就会看到下面的输出：

```console
$ cargo update
    Updating crates.io index
    Updating rand v0.8.3 -> v0.8.4
```

Cargo 忽略了那个 `0.9.0` 的发布。此刻还会注意到在 `Cargo.lock` 文件中，一处标记现在所用 `rand` 代码箱版本为 `0.8.4` 的改变。要使用版本 `0.9.0` 或任何 `0.9.x` 系列中某个版本的 `rand`，就必须将 `Cargo.toml` 更新为下面这样：

```toml
[dependencies]
rand = "0.9.0"
```

在下次运行 `cargo build` 时，Cargo 就会更新可用代码箱的登记处，并根据所指定的新版本，重新对 `rand` 需求加以评估。

关于 [Cargo](http://doc.crates.io/) 及 [Cargo 生态](http://doc.crates.io/crates-io.html)，有很多要讲的东西，这些在第 14 章会讨论到，而此时，了解上面这些就够了。Cargo 实现了非常便利的库重用，因此 Rust 公民们就能够编写出，从数个软件包组合而来的那些体量较小的项目。

### 生成随机数

现在就来开始使用 `rand` 库代码箱，生成用于猜测的数字。接下来的步骤就是更新 `src/main.rs`，如下清单 2-3 所示：

文件名：`src/main.rs`

```rust
use std::io;
use rand::Rng;

fn main() {
    println! ("猜出这个数来！");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println! ("秘密数字为：{}", secret_number);

    println! ("请输入你猜的数。");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("读取行失败......");

    println! ("你猜的数为：{}", guess);
}
```

*清单 2-3：添加生成随机数的代码*

首先，这里添加了那行 `use rand::Rng`。这 `Rng` 特质（the `Rng` trait）定义了一些随机数生成器实现的方法，而为了使用这些方法，此特质就必须要在作用域中。第 10 章将详细涵盖到特质（traits）。

接下来在中间部分，添加了两行新代码。在第一行代码中，调用了 `rand::thread_rng` 函数，该函数给到了这里即将用到的特定随机数生成器：一个相对于当前执行线程，属于本地的随机数生成器，其用到的种子由操作系统提供。随后在这个随机数生成器实例上的 `gen_range` 方法。该方法是由前面 `use rand::Rng` 语句带入到作用域的 `Rng` 特质定义。这 `gen_range` 方法取的是一个范围表达式，这里用到的范围表达式，所采取的是 `start..end` 形式，该范围表达式包含了左边界，但排除了右边界，因此就要指定 `1..101` 来求得一个 `1` 到 `100` 之间的数字。或者也可以传递范围 `1..=100`，这是等价的。

> 注意：对于不知道到底该使用那个 Rust 特质，以及要调用代码箱的那些方法和函数的情况，那么每个代码箱都有着如何使用他的说明文档。Cargo 的另一灵巧特性，便是通过运行 `cargo doc --open` 命令，就会构建出由全部本地依赖提供的文档来，并在浏览器中打开这些文档。比如说若对 `rand` 这个代码箱的其他功能感兴趣，那么运行 `cargo doc --open` 命令然后点击左侧边栏中的 `rand` 即可进一步了解。

那第二个新行，则是打印出那个秘密数字。在开发这个程序期间，这是有用的，这样能够对程序进行测试，不过在最终版本那里就会删除这行代码。若程序在一开始就打印出谜底，显然这就算不上是个游戏了。

尝试运行几次这个程序：

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

就会得到不同的随机数字，并且他们都应是 `1` 到 `100` 之间的数字。非常棒！

## 将猜数与秘数相比较

既然有了用户输入和随机数，就可以加以比较了。比较的步骤在下面的清单 2-4 中给出了。请注意这个代码还不会编译，原因后面会解释。

文件名：`src/main.rs`

```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // --跳过前面的代码--

    println! ("你猜的数为：{}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println! ("太小了！"),
        Ordering::Greater => println! ("太大了！"),
        Ordering::Equal => println! ("你赢了！"),
    }
}
```

*清单 2-4：对比较两个数可能的返回值进行处理*

首先这里添加了另一个 `use` 语句，将标准库的一个名为 `std::cmp::Ordering` 的类型，带入到作用域。这 `Ordering` 了新是另一个枚举，且其有着 `Less`、`Greater` 和 `Equal` 共计三个变种。这些就是在对两个值进行比较时，三个可能的输出了。

随后在该程序底部，添加了用到这 `Ordering` 类型的五行新代码。其中的 `cmp` 方法是对两个值进行比较，并可在任何被可比较物上进行调用。`cmp` 方法会取一个要与之相比的引用（a reference）：这里他是在将 `guess` 与 `secret_number` 相比。随后他就返回了前面用 `use` 语句带入到作用域的 `Ordering` 枚举的一个变种。这里用一个 `match` 表达式，根据以 `guess` 和 `secret_number` 中的值，对 `cmp` 调用所返回具体 `Odering` 变种，而确定出下一步要做什么。

`match` 表达式由数个 *支臂（arms）* 构成。每个支臂是由要与之匹配的 *模式（pattern）* ，及在给到 `match` 的值与该支臂的模式符合时，应运行的代码所组成。Rust 取给到 `match` 的值，并以此检视各个支臂的模式。模式及 `match` 结构，是强大的 Rust 特性，实现对代码可能遇到的各种情况的表达，并确保对全部的这些情况进行处理。在第 6 章和第 18 章，相应地将详细涵盖到这些特性。

下面就来对这里使用的 `match` 表达式的一个示例走一遍。假设说用户猜的数是 `50`，同时随机生成的秘密数这次是 `38`。在代码将 `50` 与 `38` 作比较时，由于 `50` 比 `38` 大，因此那个 `cmp` 方法就会返回 `Odering::Greater`。于是 `match` 表达式就获取到值 `Odering::Greater` 并开始对各个支臂的模式进行检查。他看了第一个支臂的模式，是 `Ordering::Less`，并发现值 `Ordering::Greater` 与 `Odering::Less` 不匹配，那么他就会忽略第一个支臂中的代码而移步到下一支臂。下一支臂的模式为 `Ordering::Greater`，这正好与 `Odering::Greater` 相匹配！那个支臂中的相关代码就会执行，进而将 `太大了！`打印到屏幕。在此场景中，由于`match` 表达式无需检视那最后的支臂，因此他就结束了。

然而清单 2-4 中的代码并不会编译。这里试着编译一下：

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

这些错误状态的核心，指向的是存在 *不匹配的类型（mismatched types）*。Rust 有着强静态类型系统（Rust has a strong, static type system）。不过他也有着类型推导（type inference）。在写下 `let mut guess = String::new()` 时，Rust 当时就能推导出 `guess` 应是个 `String`，而没有要求一定要写出该类型i`String`。但对于 `secret_number` 来说，则是一个数字类型。有几种 Rust 数字类型都可以保有一个 `1` 到 `100` 之间的值：`i32`，32 位整数；`u32`，32 位无符号整数；`i64`，64 位整数；还有一些其他的。除非有特别指明，Rust 默认都是个 `i32` 整数，除非在某处给 `secret_number` 添加了引起 Rust 推断出不同数字类型的类型信息，那么 `secret_number` 的类型就会是 `i32`。上面错误的原因，就是 Rust 无法将字符串与数字类型相比较。

最后，这里就要将程序以输入形式读取到的 `String`，转换成具体数字类型，如此就可以将其与`secret_number`进行数学上的比较。这里通过将下面这行添加到 `main` 函数体完成的：

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

添加的那行就是：

```rust
let guess: u32 = guess.trim().parse().expect("请输入一个数字！");
```

这里创建了一个名为 `guess` 的变量。不过稍等一下，这个程序不是已经有了一个名为 `guess` 的变量了吗？他确实已经有了个名为 `guess` 的变量，然而好在 Rust 是允许以一个新的 `guess` 变量，对其先前的值进行 *遮蔽（shadow）* 操作的。这样的遮蔽特性，实现了对`guess` 这个变量名的重用，而非强制创建两个诸如 `guess_str` 和 `guess` 这样的独特变量。在第 3 章将对此进行更详细的讲解，此时只要明白，此特性通常用在要将某个值从一种类型转换到另一类型的时候。

这里将这个新变量，绑定到了表达式 `guess.trim().parse()`。该表达式中的 `guess` 援引的是原来那个包含着字符串形式输入的 `guess`。而作用在 `String` 实例上的 `trim` 方法，将消除开头和结尾的全部空白，必须要进行这个操作，才能将字符串转换到 `u32` 类型，`u32`只能包含数字数据。为了满足到 `read_line` 并输入他们的猜数，用户必须要按下回车键，这样就会将一个换行字符添加到那个字符串。比如在用户敲入了 `5` 然后按下回车键时，`guess`看起来就会是这样：`5\n`。其中的 `\n` 表示 “换行（newline）”。（在 Windows 上，按下回车键会导致一个回车字符和一个换行字符，即 `\r\n`）。这 `trim` 会将 `\n` 或 `\r\n` 消除，而结果就只是 `5` 了。

[字符串上的 `parse` 方法](https://doc.rust-lang.org/std/primitive.str.html#method.parse) 将只会在那些逻辑上可被转换成数字的字符上运作，而因此就很可能引起错误。比如说在字符串包含了 `A👍%` 时，就没有办法将其转换成一个数字。由于 `parse` 方法会失败，因此他返回的是个 `Result` 类型，这与 `read_line` 方法所做的一样（在早先的 [用 `Result` 类型处理潜在失败](#处理潜在的带有-result-的程序失效) 中讨论过）。这里再次使用 `expect` 方法对这个`Result` 进行了同样的处理。在因为 `parse` 无法从字符串创建出一个数字，而返回了一个 `Err` 的 `Result` 变种时，这个 `expect` 就会令到游戏崩溃，并将给他的那条消息打印出来。而在 `parse` 可成功将那个字符串，转换成数字时，`expect` 就会返回 `Result` 的 `Ok` 变种，同时 `expect` 会返回这里想要的、`Ok` 值中的数字。

现在来运行一下这个程序！

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

很棒！尽管在猜数前加了一些空格，程序仍然算出了用户猜的是 `76`。多运行几次这个程序，来验证在各种输入时其不同的表现：猜对一个数、猜个太大的数，以及猜个过小的数。

现在这个游戏大致在工作了，然而用户只能猜一次。下面就来通过添加循环对其进行修改！

## 用循环来实现多次猜数

**Allowing Multiple Guesses with Looping**

关键字 `loop` 创建出无限循环。这里就要添加一个循环，来让用户有更多机会去猜数：

文件名：`src/main.rs`

```rust
    // --跳过--

    println! ("随机生成的秘密数字为：{}", secret_number);

    loop {
        println! ("请输入你猜的数。");

        // --跳过--

        match guess.cmp(&secret_number) {
            Ordering::Less => println! ("太小了！"),
            Ordering::Greater => println! ("太大了！"),
            Ordering::Equal => { println! ("你赢了！"); break },
        }
    }
}
```

可以看到，这里已将自猜数输入提示开始的全部代码，移入到循环中了。请确保循环中的那些代码行，都另外缩进四个空格，然后再次运行这个程序。现在程序将会一直要求另一猜数，这实际上引入了新的问题。好像是用户无法退出了！

用户可一直通过键盘快捷键 `Ctrl-C`，来中断这个程序。不过还是有别的方法，来退出这头贪厌的怪兽，就像在 [将猜数与秘密数字比较](#将猜数与秘数相比较)中对 `parse` 方法讨论中提到的那样：在用户输入了非数字的答案时，程序就会崩溃。这里就利用了那个，来实现用户退出，如下所示：

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

敲入 `quit` 就会退出这游戏，不过正如所注意到的，这样做将就要敲入别的非数字输入。至少可以是这种做法是次优的；这里想要在猜到了正确数字时，游戏也要停止。

## 猜对后的退出

下面就来通过添加一条 `break` 语句，将游戏编程为在用户赢了时退出：

文件名：`src/main.rs`

```rust
        // --跳过--

        match guess.cmp(&secret_number) {
            Ordering::Less => println! ("太小了！"),
            Ordering::Greater => println! ("太大了！"),
            Ordering::Equal => {
                println! ("你赢了！"); 
                break
            },
        }
    }
}
```

在 `你赢了！` 之后添加上 `break` 代码行，就令到游戏在用户猜中了秘密数字时，退出那个循环。由于该循环是 `main` 函数体的最后部分，因此退出循环也意味着退出这个程序。


## 无效输入的处理

为了进一步改进游戏表现，而不要在用户输入了非数字时将程序崩溃掉，那么接下来就要使得游戏忽略非数字，从而用户可以继续猜数。通过把`guess`从 `String` 转换为 `u32` 的那行加以修改，来完成这个目的，如下面的清单 2-5 所示：

文件名：`src/main.rs`

```rust
        // --跳过--

        io::stdin()
            .read_line(&mut guess)
            .expect("读取行失败......");

        if guess.trim().eq("Q") || guess.trim().eq("quit") { process::exit(0); }

        // let guess: u32 = guess.trim().parse().expect("请输入一个数字！");
        let guess: u32 = match guess.trim().parse() {
           Ok(num) => num,
           Err(_) => { println! ("请输入一个数字！"); continue },
        };

        println! ("你猜的数为：{}", guess);

        // --跳过--
```

*清单 2-5：忽略非数字的猜解进而询问另一猜数，而不再是崩溃掉程序*

这里将原来的 `expect` 调用，转换到了一个 `match` 表达式，而实现了一错误就程序崩溃，到对错误进行处理的转变。请记住 `parse` 返回的是个 `Result` 类型，而 `Result` 则是个枚举，有着变种 `Ok` 和 `Err`。与先前对 `cmp` 方法返回结果 `Ordering` 的处理一样，这里运用了一个 `match` 表达式。

在 `parse` 能够成功将那个字符串，转换为数字时，他就会返回一个包含了所得结果数的 `Ok` 值。那 `Ok` 值就会匹配上第一个支臂的模式，而这个 `match` 表达式将值返回 `parse` 产生的、放在`Ok` 值里头的那个 `num` 值。那个数字就会刚好放在这里想要他呆的地方，即这里正在创建的那个新 `guess` 变量了。

在 `parse` 无法将那个字符串转换成数字时，他就会返回一个包含了有关该错误详细信息的 `Err` 值。该 `Err` 值不与第一个 `match` 支臂中的 `Ok(num)` 模式匹配，不过却正好匹配第二个支臂中的 `Err(_)` 模式。其中的下划线，`_`，是个收集错误信息的值（a catch-all value）；在此示例中，就是要匹配所有 `Err` 值，而不管这些 `Err` 值中包含了什么信息。那么程序就会执行第二支臂的代码，即 `continue`，这是告诉程序前往到那个 `loop` 循环的下一次迭代，进而询问另一个猜数。就这样，有效地方让程序忽略了全部 `parse` 可能会发生的错误了！

现在程序各方面就应如预期那样工作了。就来试试：

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

非常棒！只需最后一个小的优化，就将完成这个猜数游戏了。没忘记这个程序仍是把秘密数字打印出来的吧。那样做对测试来说没有问题，但却毁掉了这个游戏。这里就来将输出了秘密数字的那个 `prinln!` 给删掉。下面的清单 2-6 给出了最终代码。

文件名：`src/main.rs`

```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::process;

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
                .expect("读取行失败......");

            if guess.trim().eq("Q") || guess.trim().eq("quit") { process::exit(0); }

            // let guess: u32 = guess.trim().parse().expect("请输入一个数字！");
            let guess: u32 = match guess.trim().parse() {
               Ok(num) => num,
               Err(_) => { println! ("请输入一个数字！"); continue },
            };

            println! ("你猜的数为：{}", guess);

            match guess.cmp(&secret_number) {
                Ordering::Less => println! ("太小了！"),
                Ordering::Greater => println! ("太大了！"),
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

## 小结

到了这里，就成功构建了这个猜数游戏。恭喜！

该项目以动手的方式，教了许多新的 Rust 概念：`let`，`match` 等关键字，函数、运用外部代码箱及更多。在接下来的几章中，会更深入地掌握这些概念。第 3 章涵盖了大多数编程语言都有的一些概念，诸如变量、数据类型及函数，并展示了如何在 Rust 中使用他们。第 4 章对 Rust 中的所有权（ownership）进行了探索，所有权是一项令到 Rust 不同于其他语言的特性。第 5 章对结构体（structs）和方法语法（method syntax）进行了讨论，而第 6 章解释了枚举的原理。
