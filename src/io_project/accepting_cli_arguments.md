# 接收命令行参数

我们来像往常一样，以 `cargo new` 创建一个新项目。我们称我们的项目为 `minigrep`，以将其与咱们系统上可能已有的 `grep` 工具区分开：

```console
$ cargo new minigrep
    Creating binary (application) `minigrep` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
$ cd minigrep
```

第一个任务是要让 `minigrep` 接收他的两个命令行参数：文件路径和要检索的字符串。也就是说，我们希望能够以 `cargo run`，及两个短横线（`--`）来表明接下来的参数是针对我们的程序的参数而不是 `cargo`，与一个要检索的字符串，以及一个要检索的文件的路径来运行我们的程序，如下所示：

```console
$ cargo run -- searchstring example-filename.txt
```

目前，由 `cargo new` 生成的程序无法处理我们给他的参数。[crates.io](https://crates.io/) 上的一些现有库可以帮助编写接受命令行参数的程序，但因为咱们恰好正在学习这个概念，所以我们来自己实现这一能力。

## 读取参数值

为了使 `minigrep` 能够读取我们传递给他的命令行参数的值，我们将需要 Rust 标准库中提供的 `std::env::args` 函数。这个函数返回传递给 `minigrep` 的命令行参数的迭代器。我们将在 [第 13 章](../Ch13_Functional_Language_Features_Iterators_and_Closures.md) 中全面介绍迭代器。现在，咱们只需要知道有关迭代器的两个细节：迭代器产生一系列值，我们可以调用迭代器的 `collect` 方法将其转换为比如矢量值的集合，其会包含迭代器产生的所有元素。

下面清单 12-1 中的代码允许咱们的 `minigrep` 程序读取传递给他的任何命令行参数，然后收集这些值到一个矢量值中。

<a name="listing_12-1"></a>
文件名：`src/main.rs`

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg! (args);
}
```

**清单 12-1**：收集命令行参数到一个矢量值中并打印他们


首先，我们以一个 `use` 语句带入 `std::env` 模组到作用域，以便我们可以使用他的 `args` 函数。请注意，`std::env::args` 函数嵌套在两层模组中。正如我们在 [第 7 章](../packages_crates_and_modules/the_use_keyword.md#创建惯用的-use-路径) 中讨论的，在所需函数嵌套于多个模组中的情形下，我们选择带入父模组而不是该函数到作用域。通过这样做，我们就可以轻易地使用 `std::env` 中的其他函数。相比于添加 `use std::env::args` 然后仅以 `args` 调用该函数，这种做法也更明确，因为 `args` 会很容易被误认为是某个定义在当前模组中的函数。

> **`args` 函数与无效的 Unicode**
>
> 请注意，当任何参数包含无效的 Unicode 时，`std::env::args` 将终止运行。当咱们的程序需要接受包含无效 Unicode 字符的参数时，请改用 `std::env::args_os`。该函数返回一个生成 `OsString` 值，而不是 `String` 值的迭代器。出于简单起见，我们在这里选择使用 `std::env::args`，因为 `OsString` 因平台而异，并且比 `String` 值使用起来更复杂。

在 `main` 函数的第一行，我们调用了 `env::args`，并立即使用 `collect` 将迭代器转换为包含由该迭代器产生的所有值的矢量值。我们可以使用 `collect` 函数创建多种类别的集合，因此我们显式地注解了 `args` 的类型，以指定我们想要一个字符串矢量值。尽管咱们很少需要在 Rust 中注解类型，但 `collect` 属于咱们经常要注解的函数之一，因为 Rust 无法推断咱们想要的集合类别。

最后，我们使用调试宏（`dbg!`）打印出该矢量值。我们来尝试先不带参数运行这段代码，然后以两个参数运行：

```console
$ cargo run
   Compiling minigrep v0.1.0 (/home/hector/rust-lang-zh_CN/projects/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/minigrep`
[src/main.rs:5:5] args = [
    "target/debug/minigrep",
]
```


```console
$ cargo run -- needle haystack
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep needle haystack`
[src/main.rs:5:5] args = [
    "target/debug/minigrep",
    "needle",
    "haystack",
]
```


请注意，矢量值中的第一个值是  `"target/debug/minigrep"`，这是我们的二进制文件的名字。这与 C 中的参数列表的行为一致，允许程序在执行时使用其被调用的名字。当咱们打算在消息中打印程序的名字，或根据用于调用程序的命令行别名改变程序的行为的情形下，能够访问程序名字通常会很方便。但就这一章的目的而言，我们将忽略他而只保存我们需要的两个参数。


## 保存参数值于变量中

这个程序目前能够访问指定为命令行参数的值。现在，我们需要保存两个参数的值于变量中，以便我们可以在程序的其余部分使用这些值。我们在下面清单 12-2 中实现了这点。

<a name="listing_12-2"></a>
文件名：`src/main.rs`

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println! ("
        在文件 {filename} 中
        检索 {query}
    ");
}
```

**清单 12-2**：创建变量来保存查询参数和文件路径参数

正如我们在打印矢量值时所看到的，程序的名字占据 `args[0]` 处的矢量值的第一个值，因此我们于索引 `1` 处开始参数。`minigrep` 取的第一个参数是我们要检索的字符串，因此我们放置到第一个参数的引用在变量 `query` 中。第二个参数将是文件路径，因此我们放置到第二个参数的引用在变量 `file_path` 中。

我们暂时打印这两个变量的值，以证明代码是按我们的预期运行。我们来以参数 `test` 和 `sample.txt` 再次运行这个程序：

```console
$ cargo run -- test sample.txt
   Compiling minigrep v0.1.0 (/home/hector/rust-lang-zh_CN/projects/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/minigrep test sample.txt`

        在文件 sample.txt 中
        检索 test

```

太好了，程序运行正常！我们需要的参数值被保存到正确的变量中。稍后我们将添加一些错误处理，来处理某些潜在的错误情形，比如当用户未提供参数时；现在，我们将忽略这种情况，转而着手添加文件读取能力。


（End）


