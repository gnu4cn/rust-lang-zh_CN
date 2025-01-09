# 接收命令行参数

现在来与往常一样，使用 `cargo new` 创建一个新的项目。这里将把这个项目，叫做 `minigrep` 来将其区别于或许已在现有系统上有的那个 `grep` 工具。

```console
$ cargo new minigrep
     Created binary (application) `minigrep` project
$ cd minigrep
```

首个任务，即要让 `minigrep` 接收他的两个命令行参数：文件路径与要检索的字符串。那就是，这里打算能够以 `cargo run`，与两个短横线（`--`）来表明接下来的参数，是这个程序的参数，这样的方式，而非 `cargo` 与一个要检索的字符串，及要在其中检索的文件路径的方式来运行这个程序，如下所示：


```console
$ cargo run -- searchstring example-filename.txt
```

而现在，由 `cargo new` 命令生成的程序，是无法处理给他的参数的。[crates.io](https://crates.io/) 上的一些既有库，可以帮助编写除接收命令行参数的程序，而由于咱们才开始了解这个概念，因此就要咱们自己来实现这项功能。

## 读取参数值

为开启 `minigrep` 对传给他的命令行参数值的读取，这里将需要在 Rust 标准库中所提供的 `std::env::args` 函数。该函数返回的是那些传递给 `minigrep` 命令行参数的一个迭代器。后面的 [第 13 章](Ch13_Functional_Language_Features_Iterators_and_Closures.md) 就会讲到迭代器。而现在，就只需要知道迭代器的两个细节：迭代器会产生出一些列值，而在某个迭代器上调用 `collect` 方法，就可以将其转换成比如矢量这这样的一个、包含着迭代器产生的全部元素的集合。

下面清单 12-1 中的代码，实现了`minigrep` 程序读取全部传递给他的命令行参数，并于随后将这些值收集到一个矢量中。

文件名：`src/main.rs`

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg! (args);
}
```

*清单 12-1：将命令行参数，收集到一个矢量中并把他们打印出来*


这里首先使用了一个 `use` 语句，将那个 `std::env` 模组带入到了作用域，如此就可以使用他的 `args` 函数了。请注意这个 `std::env::args` 函数，是嵌套在两个层级的模组中的。如同在 [第 7 章](Ch07_Managing_Growing_Projects_with_Packages_Crates_and_Modules.md#创建惯用-use-路径) 处所讨论过的，在那些所需函数是嵌套于多个模组中的情形下，那里就选择将其中的父模组带入到作用域，而非该函数本身。经由这样做，就可以轻易地使用到 `std::env` 中的其他函数了。同时相比于添加 `use std::env::args` 并在随后只使用 `args` 调用这个函数，这样做也不那么含糊其辞，这是由于 `args` 这个名字，可能稍不留意就会被误用为定义在当前模组中的某个函数。

> **`args` 函数与无效 Unicode 字符**
>
> 请注意 `std::env::args` 在由任何参数包含了无效 Unicode 字符时，将会中止运行。在程序需要接收包含了无效 Unicode 字符的参数时，就要使用 `std::env::args_os`。那个函数返回的是一个产生出 `OsString` 值，而非 `String` 值的迭代器。由于各个平台上的 `OsString` 值有所区别，且相比使用 `String` 值，`OsString` 使用起来要更为复杂，因此为简化起见，这里使用的是 `std::env::args`。

在 `main` 函数的第一行，这里调用了 `env::args`，并立即使用 `collect` 来将其所返回的那个迭代器，转换为一个包含由该迭代器所产生全部值的矢量值。由于使用这个 `collect` 函数，即可创建出许多类别的集合来，因此这里就显示地对 `args` 变量的类型进行了注解，来指明这里要的是一个字符串的矢量。尽管在 Rust 中，极少需要对类型加以注解，不过这个 `collect` 函数就是一个通常需要注解的函数，这是由于 Rust 无法推断出，代码编写者想要的集合类别来。

最后，这里使用了调试宏（`dbg!`），打印出那个矢量。下面就来尝试先不带参数运行该代码，尔后再带上两个参数：

```console
$ cargo run                                                                                 lennyp@vm-manjaro
   Compiling minigrep v0.1.0 (/home/lennyp/rust-lang/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.39s
     Running `target/debug/minigrep`
[src/main.rs:6] args = [
    "target/debug/minigrep",
]
```


```console
$ cargo run -- 检索字符串😀 demo.txt                                                        lennyp@vm-manjaro
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep '检索字符串😀' demo.txt`
[src/main.rs:6] args = [
    "target/debug/minigrep",
    "检索字符串😀",
    "demo.txt",
]
```


请注意这个矢量中的首个值，即 `"target/debug/minigrep"`，就是这里二进制程序文件的名字。这一点符合了 C 语言中参数清单的行为，让程序运用到其被触发执行的那个名字（this matches the behavior of the arguments list in C, letting programms use the name by which they were invoked in their execution）。在要于消息中打印出程序名字，或根据用于触发该程序运行的何种命令行别名，而对程序行为加以改变这样的情形下，有着对程序名字的访问，通常就比较便利。而对于本章的目的，这里就会忽略这首个参数，而只保存这里所需的那两个参数。


## 将参数值保存在变量中


这个程序此刻就可以访问到被指定为命令行参数的那些值了。现在这里需要将这两个参数的值，保存在变量中，如此就可以在这个程序的整个其余部分，使用到这些值了。在下面清单 12-2 中就要完成这个事情。

文件名：`src/main.rs`

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println! ("在文件 {} 中检索：{}", file_path, query);
}
```

*清单 12-2：创建两个变量来保存查询参数与文件路径参数*

与在打印这个矢量时所看到的一样，该程序的名字，占据了那个矢量中 `args[0]` 处的首个值，因此这里是于索引 `1` 处开始参数的。`minigrep` 取的第一个参数，即为这里正检索的字符串，因此这里把到这首个参数的索引，放在了变量 `query` 中。第二个参数将是那个文件路径，因此这里把到那第二个参数的索引，放在了变量 `file_path` 中。

这里临时性地将这两个变量的值打印出来，以证实该代码是如打算那样运行。下面就来以参数 `test` 和 `sample.txt`，再次运行这个程序：

```console
$ cargo run -- test sample.txt                                                              lennyp@vm-manjaro
   Compiling minigrep v0.1.0 (/home/lennyp/rust-lang/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.35s
     Running `target/debug/minigrep test sample.txt`
在文件 sample.txt 中检索：test
```

很好，这个程序工作了！所需参数的那些值正被保存到恰当的变量中。后面就要添加一些错误处理，来处理某些潜在的错误情形，诸如在用户未提供参数这样的情况；现在，这里将忽略那样的情况，而是会编写添加文件读取能力的代码。


（End）


