# 一个文件系统 I/O 项目：构建一个命令行程序

这一章是对到目前为止所学到技能的一个回顾，又是对少数几个另外标准库特性的一个探索。这里将构建一个与文件及命令行输入输出进行交互的命令行工具，来练习咱们现在掌握到的一些 Rust 概念。

Rust 的速度、安全性、单一二进制可执行程序输出，还有跨平台支持，令其成为创建命令行工具的理想编程语言，那么对于这个项目，这里将构造自己版本的那个经典的命令行搜索工具 `grep` （**g**lobally search a **r**egular **e**xpression and **p**rint，正则表达式全局搜索及打印程序）。在最简单用例中，`grep` 会对某个指定文件，就某个指定字符串而加以搜索。为完成这个目的，`grep` 就会取一个文件路径与一个字符串作为其命令行参数。随后他会读取那个文件，找到那个文件中包含有该字符串参数的那些行，并打印出这些行。

在构造这个命令行程序的道路上，这里将展示如何让这个命令行工具，使用到其他命令行工具都会用到的一些终端特性（the terminal features）。这里将读取某个环境变量的值，来允许使用者对这个工具默认行为进行配置。这里还会将错误消息打印到标准错误控制台的流（the standard error console stream, `stderr`），而非打印到标准输出（`stdout`），如此一来，用户就可以将成功的输出重定向到某个文件，而仍能从屏幕上看到错误消息，并有着其他一些好处。

名为 Andrew Gallant 的一位 Rust 社区成员，就已经创建了一个特性完整、非常快版本的 `grep`，名叫 [`ripgrep`](https://github.com/BurntSushi/ripgrep)。相比之下，这个版本将相当简单，不过这一章将给到一些掌握诸如 `ripgrep` 这样的真实项目，所需的背景知识。

这个 `grep` 项目，将结合至今所掌握的下面几个到目前为止已掌握的概念：

- 对代码进行组织（使用 [第 7 章](Ch07_Managing_Growing_Projects_with_Packages_Crates_and_Modules.md) 中所掌握的有关模组的知识）
- 对矢量值与字符串的使用（集合，[第 8 章](Ch08_Common_Collections.md)）
- 对错误的处理（[第 9 章](Ch09_Error_Handling.md)）
- 在恰当之处使用特质与生命周期（[第 10 章](Ch10_Generic_Types_Traits_and_Lifetimes.md)）
- 编写测试（[第 11 章](Ch11_Writing_Automated_Tests.md)）

这里还会简要对闭包、迭代器及特质对象等，进行简要介绍，后面的 [第 13 章](Ch13_Functional_Languages_Features_Iterator_and_Closures.md) 与 [第 17 章](Object_Oriented_Programming_Features_of_Rust.md) 等章节，将详细讲解到这些特性。


## 接收命令行参数

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

### 读取参数值

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


这里首先使用了一个 `use` 语句，将那个 `std::env` 模组带入到了作用域，如此就可以使用他的 `args` 函数了。请注意这个 `std::env::args` 函数，是嵌套在两个层级的模组中的。如同在 [第 7 章](Ch07_Managing_Growing_Projects_with_Packages_Crates_and_Modules.md#creating-idiomatic-use-path) 处所讨论过的，在那些所需函数是嵌套于多个模组中的情形下，那里就选择将其中的父模组带入到作用域，而非该函数本身。经由这样做，就可以轻易地使用到 `std::env` 中的其他函数了。同时相比于添加 `use std::env::args` 并在随后只使用 `args` 调用这个函数，这样做也不那么含糊其辞，这是由于 `args` 这个名字，可能稍不留意就会被误用为定义在当前模组中的某个函数。

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


### 将参数值保存在变量中


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

## 读取文件

现在就要添加读取那个在 `file_path` 参数中所指定文件的功能了。首先，这里需要一个样本文件来对其进行测试：这里将使用一个有着少量文字、其中多个行均有一些重复文字的文件。下面清单 12-3 这首 Emily Dickinson 的诗歌用起来就会不错！在项目的根目录处创建一个叫做 `poem.txt` 的文件，并敲入这首 “I'm Nobody! Who are you?” 的诗歌。

文件名：`poem.txt`

```txt
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
```

*清单 12-3：一首 Emily Dickinson 的诗歌成就了一个良好的测试用例*


有了这个文本后，就要编辑 `src/main.rs` 并添加读取该文件的代码了，如下清单 12-4 中所示。

文件名：`src/main.rs`

```rust
use std::env;
use std::fs;

fn main() {
    // --跳过代码--
    println! ("在文件 {} 中检索：{}", file_path, query);

    let contents = fs::read_to_string(file_path)
        .expect("应能读取这个这个文件");

    println! ("有着文本：\n{}", contents);
}
```

*清单 12-4：对由第二个参数所指定的文件内容进行读取*

首先，这里使用了一个 `use` 语句，将标准库的一个相对部分（a relevant part）带入进来：这里需要 `std::fs` 来对文件进行处理。

在 `main` 函数中，那个新的 `fs::read_to_string` 取了其中的 `file_path` 做参数，打开那个文件，并返回一个该文件内容的 `std::io::Result<String>` 类型值。

在那之后，这里再次添加了一个临时的、于该文件被读取之后打印 `contents` 值的 `println!` 语句，因此这里就该程序到此在运行而进行检查了。

下面就来以任意字符串作为第一个参数（由于这里尚未实现检索的部分），并以那个 `poem.txt` 文件作为第二个参数，运行这段代码：

```rust
$ cargo run -- the poem.txt                                                                    lennyp@vm-manjaro
   Compiling minigrep v0.1.0 (/home/lennyp/rust-lang/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.36s
     Running `target/debug/minigrep the poem.txt`
在文件 poem.txt 中检索：the
有着文本：
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!

```

很好！这代码就读取并于随后打印出了那个文件的内容。但这代码有着少数几个缺陷。此时的这个 `main` 函数，有着多重义务：一般来讲，在每个函数只负责一件事情时，那么他们就是些更为清晰明了，并更易于维护的函数了。另一问题则是这里没有尽可能地对错误进行处理。这个程序还很小，因此这些缺陷就不是什么大问题，不过随着程序变大，就会变得更加难于彻底修复这些缺陷。在开发某个程序时，由于重构数量较少的代码要容易得多，因此尽早开始重构，是一种良好实践。接下来就会干这件事 -- 重构。


## 对代码进行重构来改进模组性与错误处理

为改进这个程序，这里就要修复与该程序结构及其处理潜在错误方式有关的四个问题。首先，这里的 `main` 函数现在执行了两个任务：他对参数进行解析并读取文件。随着程序的增长，这个 `main` 函数所处理的独立任务数目将不断增加。而随着函数不断获得其任务，就会变得更加难于推理，更难于对其进行测试，以及更难于在不破坏其各个部分的情况下对其进行修改。那么最后就要将功能拆分，从而每个函数负责一项任务。

这个问题同样联系着第二个问题：尽管这里的 `query` 与 `file_path` 属于这个程序的配置性变量，而像 `contents` 这样的变量则被用于执行该程序的逻辑处理。这个 `main` 函数变得越长，那么这里就会将更多的变量引入到作用域；在作用域中的变量越多，那么就会越难对各个变量的目的保持追踪。因此就最好将这些配置变量，分组到某个结构体中，而令到他们的目的明确。

第三个问题则是，在读取那个文件失败时，这里使用了 `expect` 将一条错误消息打印处理，而该错误消息只会打印 “应能读取这个这个文件。” 文件读取以多种方式失败：比如那个文件可能没有，或可能没有打开他的权限。此时，无论何种情形，这里都将打印同样的错误消息，这样并不会给到用户任何信息！

第四，这里重复地使用了 `expect` 来处理不同重复，而在用户未指定足够参数时，他们就会得到一个并不会清楚解释问题原因、 Rust 的 `index out of bounds` 错误。若全部错误处理代码都在一个地方，那么就最好了，这样在错误处理代码需要修改时，那么以后的维护者就只有一个地方来查阅代码。将全部错误处理代码放在一处，还将确保这里打印的消息，是会对终端用户有意义的那些消息。

下面就来通过对这里的项目进行重构，来解决这四个问题。


### <a id="separation-of-concerns-for-binary-projects"></a>二进制程序项目的关注点分离

**Separation of Concerns for Binary Projects**

将多重任务分配给那个 `main` 函数方面的组织性问题，常见于许多二进制项目。由此 Rust 社区业已开发了在 `main` 开始变得大型起来时，将二进制程序单个关注点进行剥离的守则。这个剥离单独关注点的过程，有着以下几个步骤：

- 将程序剥离为一个 `main.rs` 与一个 `lib.rs`，并将程序逻辑迁移到 `lib.rs`；
- 由于命令行解析逻辑不大，因此他仍然留在 `main.rs` 中；
- 而在命令行解析逻辑开始变得复杂的时候，就要将其从 `main.rs` 提取出来，并将其迁移到 `lib.rs`。


那么在经历了剥离单独关注点这个过程后，留在这个 `main` 函数中的任务就应局限于下面这些了：

- 以那些参数值，对命令行解析逻辑进行调用；
- 建立起全部其他配置；
- 调用 `lib.rs` 中的某个 `run` 函数；
- 在 `run` 返回了某个错误时，对该错误加以处理。


这种模式，是有关关注点分离的：`main.rs` 对运行程序加以处理，而 `lib.rs` 处理的则是手头任务的全部逻辑。由于无法对 `main` 函数直接进行测试，因此这种结构通过将全部程序逻辑移入到 `lib.rs` 种的函数，而允许对他们进行测试了。保留在 `main.rs` 种的代码，将足够小到通过过目一下，就可以验证其正确性。下面就来依照这些步骤，重制这个程序。

### 提取参数解析器

这里将把解析参数的功能，提取到一个 `main` 会调用到的函数种，从而把命令行解析逻辑（the command line parsing logic），迁移到 `src/lib.rs`。下面清单 12-5 就给出了调用了一个新函数 `parse_config` 的 `main` 新开头，此刻这里将把这个新函数 `parse_config` 定义在 `src/main.rs` 中。

文件名：`src/main.rs`

```rust
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, file_path) = parse_config(&args);
    // --跳过代码--
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}
```

*清单 12-5：自 `main` 中提取一个 `parse_config` 函数*

这里仍是将那些命令行参数，收集到一个矢量中，而与在 `main` 函数中，将索引 `1` 处的参数值指派给变量 `query`，及将索引 `2` 处的参数值指派给变量 `file_path` 不同，这里将那整个矢量，传递给了 `parse_config` 函数。这个 `parse_config` 函数随后就持有了确定哪个参数进到哪个变量，及将这些值传回到 `main` 的逻辑。在 `main` 中，这里仍创建了 `query` 与 `file_path` 两个变量，但 `main` 不再具有确定命令行参数与变量如何对应起来的义务了。

对于这里的小型程序，这项重制可能看起来矫枉过正了，但这里是正在以小的、渐进式的步骤进行重构。在做出这项修改后，就要再次运行这个程序来验证参数解析仍会运作。频繁检查所取得的进展，从而在有问题发生时，帮助识别出问题的原因，总是不错的做法。

### 对配置值进行分组

**Grouping Configuration Values**

这里可以进一步对那个 `parse_config` 函数加以改进。此刻，这里返回的是个元组，然后随后又立即将那个元素，再次拆分为了单独的一些部分。这便是个或许这里尚未有着恰当抽象的表征。

有着改进空间的另一指标，便是 `parse_config` 的 `config` 部分，这暗示了这里返回的两个值是有关联的，且他俩都是某个配置值的组成部分。由于这里是将这两个值编组为了元组，而并未以数据结构（the structure of the data）方式分组，因此当前并未揭示出这层意义来；那么这里就要将这两个值，放入到某种结构体中，并分别给到该结构体的两个字段有意义的名字。这样做将让此代码的未来维护者更加容易理解，不同值直接是怎样相互联系起来的，以及他们各自的目的为何。

下面清单 12-6 就给出了对这个 `parse_config` 函数的改进。

文件名：`src/main.rs`

```rust
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println! ("在文件 {} 中检索：{}", config.file_path, config.query);

    let contents = fs::read_to_string(config.file_path)
        .expect("应能读取这个这个文件。");

    // --跳过代码--
}

struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config { query, file_path }
}
```

*清单 12-6：将 `parse_config` 重构为返回 `Config` 结构体的实例*

这里就已添加了一个名为 `Config`、定义为有着名为 `query` 与 `file_path` 字段的一个结构体。现在 `parse_config` 的签名，就表示其返回了一个 `Config` 值。而在那个 `parse_config` 的函数体中，之前曾于其中返回引用了 `args` 中那些 `String` 值的字符串切片，现在则定义了 `Config` 来包含持有所有权的一些 `String` 值。`main` 中的那个 `args` 变量，即为那些参数值的所有者，且仅允许那个 `parse_config` 函数借用那些参数值，这就意味着在 `Config` 尝试取得 `args` 中那些值的所有权时，就会破坏 Rust 的借用规则。

对于 `String` 数据的管理，是可以采用多种方式的；而其中最容易的途径，当然虽不那么高效，便是在这些值上调用 `clone` 方法。这将构造出为那个 `Config` 实例所持有的该数据的完整拷贝，相比于存储到那个字符串数据的一个引用，这样做会消耗更多时间与内存。但对数据进行克隆，由于就不必对引用的生命周期加以管理，而也会令到这里的代码相当直接；在这样的情形下，为获得简单性而舍弃一点小小的性能，即是有价值的一种取舍。

> **使用 `clone` 上的权衡**
>
> 在相当多 Rust 公民中间，有着由于 `clone` 的运行时开销，而避免使用其来修复所有权问题的这种倾向。在接下来的 [第 13 章](Ch13_Functional_Language_Features_Iterators_and_Closures.md) 中，就会掌握到在这类情形下，怎样使用别的一些高效的方法。而现在，则由于仅会构造这些拷贝一次，且文件路径与查询字串都相当小，那么对少量字符串加以拷贝，以继续进行关注点分离过程，是可以的。相比于在起步阶段就尝试对代码进行超优化（hyperoptimize），更好的选择当然是有一个不那么高效的运行的程序了。而随着对 Rust 日益熟练，就会更容易以最为高效的解决办法开始，而此刻，调用 `clone` 是相当可接受的做法。

这里已对 `main` 进行了更新，如此其就把由 `parse_config` 所返回的那个 `Config` 实例，置于一个名为 `config` 的变量中，同时这里更新了之前使用了 `query` 与 `file_path` 两个单独变量的代码，如此该代码现在使用的就是那个 `Config` 结构体上的字段了。

现在这里的代码，就更清楚地反应了 `query` 与 `file_path` 二者是相关的，以及他们的目的是要配置该程序将如何运作。任何用到这两个值的代码，就都知道了要在那个 `config` 实例中，在以其目的而取名的字段中找到他们。


### 给 `Config` 创建一个构造器

到这里，就已把负责解析命令行参数的逻辑，从 `main` 中提取了出来，而将其放在了那个 `parse_config` 函数中。这样做有助于看出其中 `query` 与 `file_path` 两个值是相关的，而那层联系应在这里的代码中体现出来。随后这里添加了一个 `Config` 的结构体，来命名 `query` 与 `file_path` 这种关联目的，并能够将这些值的名字作为结构体字段，自这个 `parse_config` 函数而加以返回。

那么既然这个 `parse_config` 函数的目的是要创建一个 `Config` 的实例，那么就可以将 `parse_config` 从一个普通函数，修改为一个命名为 `new` 的、与 `Config` 结构体关联起来的函数。进行这一修改，将令到代码更加符合 Rust 语言习惯。对于标准库中的那些类型，譬如 `String`， 就可以通过调用 `String::new` 创建出他们的实例来。与此类似，通过将 `parse_config` 修改为与 `Config` 关联起来的 `new` 函数，就可以通过调用 `Config::new` 而创建出 `Config` 的实例来。下面清单 12-7 给出了这里需要做出的修改。

文件名：`src/main.rs`

```rust
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);
    // --跳过代码--
}

// --跳过代码--

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}
```

*清单 12-7：把 `parse_config` 修改为 `Config::new`*


这里已将其中曾对 `parse_config` 进行调用的 `main`，更新为了调用 `Config::new`。已将 `parse_config` 这个名字，修改为了 `new`，并将其移入到了一个 `impl` 代码块里头，而正是这个 `impl` 代码块，把这个 `new` 函数，与 `Config` 关联了起来。请尝试再次编译此代码，来确保其的运作。

### 对错误处理进行修复

现在就要开始修复这里的错误处理了。回顾到之前在尝试访问 `args` 矢量中索引 `1` 或索引 `2` 处的值，若该矢量包含了少于三个条目，那么就会导致该程序终止运行。请以不带任何参数运行这个程序；他就会看起来像下面这样：

```console
cargo run                                                                               lennyp@vm-manjaro
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep`
thread 'main' panicked at 'index out of bounds: the len is 1 but the index is 1', src/main.rs:24:21
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

其中的行 `index out of bounds: the len is 1 but the index is 1` 是一条留给代码编写者的错误消息。该消息不会帮到终端用户，搞明白又该怎样做。现在就要来修复这个问题。


**改进错误消息**

下面的清单 12-8 中，这里于那个 `new` 函数中，在访问索引 `1` 与 `2` 之前，添加一个验证那个切片是否足够长的检查。若该切片没有足够长，那么这个程序就会终止运行，并显示出一个更好的错误消息。


文件名：`src/main.rs`

```rust
    // --跳过代码--
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic! ("参数数量不足");
        }
        // --跳过代码--
```

*清单 12-8：添加一个参数个数的检查*


此代码与 [清单 9-13 中曾编写过的 `Guess::new` 函数](Ch09_Error_Handling.md#creating-custom-types-for-validation) 类似，其中在那个 `value` 参数超出有效值边界时，就调用了 `panic!` 宏。这里没有检查值的边界，而是就 `args` 的长度至少为 `3` 进行了检查，进而该函数的其余部分，就可以在此条件已满足的假定下运作了。在 `args` 所拥有的条目少于三个时，此条件便为真，进而这里就会条约那个 `panic!` 宏，来立即结束这个程序。

有了`new` 中的这些额外少数几行，下面就不带任何参数地再度运行这个程序，来看看现在错误看起来如何：

```console
$ cargo run                                                                            lennyp@vm-manjaro
   Compiling minigrep v0.1.0 (/home/lennyp/rust-lang/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.57s
     Running `target/debug/minigrep`
thread 'main' panicked at '参数数量不足', src/main.rs:25:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

此输出好了一些：现在这里就有了一个合理的错误消息了。不过，这里还有一些不希望给到用户的无关信息。或许运用曾在清单 9-13 中用到的那种技巧，并非这里要用到的最佳技巧：到 `panic!` 的调用，相比于用法方面的问题，是更适合于编程方面的问题的，如同 [第 9 章中所讨论的那样](Ch09_Error_Handling.md#guidelines-for-error-handling)。相反，这里将使用之前在第 9 章中曾学到的另一项技能 -- [返回一个 `Result`](Ch09_Error_Handling.md#recoverable-errors-with-result)，以表示成功执行成功或是出错。


**返回一个 `Result` 值，而非调用 `panic!` 宏**

与上面调用 `panic!` 相比，这里可返回将包含成功情形下的 `Config` 实例，及在错误情形下对问题进行描述的 `Result` 值。由于许多编程者都期望 `new` 函数绝不失败，因此这里还将把该函数的名字，从 `new` 修改为 `build`。在 `Config::build` 与 `main` 通信时，这里就可以使用这个 `Result` 类型，来发出存在某个问题的信号了。接下来就可以将 `main` 修改为将 `Err` 变种，转换为一个对程序使用者来说更实际的错误消息，而不再带有那些因调用 `panic!` 宏，而引发的前后有关 `thread 'main'` 及 `RUST_BACKTRACE` 的字眼。

下面清单 12-9 给出了对现在调用的 `Config::Build` 函数返回值，以及该函数体需要一个返回 `Result` 值，而需要做出的修改。请注意在下一代码清单中，同时更新 `main` 之前，此代码是不会编译的。

文件名：`src/main.rs`

```rust
impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("参数数量不足");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
```

*清单 12-9：自 `Config::build` 返回一个 `Result`*

这里的 `build` 函数，返回的是一个在成功情形下带有 `Config` 实例，在错误情况下有着一个 `&'static str` 的 `Result` 值。这里的错误值将始终是有种 `'static` 生命周期的字符串字面值。

在该函数的函数体中，这里完成了两处改变：与在使用者未传递足够参数时调用 `panic!` 宏不同，现在这里返回的是一个 `Err` 值，同时这里已将那个 `Config` 的返回值，封装在了一个 `Ok` 中。这些修改就令到该函数与其新的类型签名相符了。

从 `Config::build` 返回一个 `Err` 的值，就允许 `main` 函数对自那个 `build` 函数返回的 `Result` 值加以处理，进而在错误情形下，更明确的退出该程序进程。


**对 `Config::build` 进行调用并对错误进行处理**

为对错误情形加以处理，并打印出用户友好的消息，这里就需要更新 `main`，以处理由 `Config::build` 所返回的那个 `Result` 值，如下清单 12-10 中所示。这里还将承担在不使用 `panic!` 宏后，以一个非零错误代码退出这个命令行工具的任务，并要亲自实现这个任务。非零的退出状态，是一条用于向调用咱们编写的程序的进程，发出程序以错误状态退出信号的约定。

文件名：`src/main.rs`

```rust
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println! ("解析参数时遇到问题：{err}");
        process::exit(1);
    });

    // --跳过代码--
```

*清单 12-10：在构建一个 `Config` 失败时以一个错误代码退出*

在此代码清单中，业已使用一个尚未详细讲过的方法：`unwrap_or_else`，这是个由标准库定义在 `Result<T, E>` 上的方法。使用 `unwrap_or_else`，就允许定义出一些定制的、非 `panic!` 的错误处理。在由 `Config::build` 返回的那个 `Result` 为一个 `Ok` 的值时，该方法的行为就跟 `unwrap` 类似：其返回 `Ok` 所封装的那个内部值。不过在返回的 `Result` 是个 `Err` 时，该方法就会调用那个 *闭包（closure）* 中的代码，而该闭包代码，则是这里所定义、并将其作为一个参数，而传递给 `unwrap_or_else` 的一个匿名函数（an anonymous function）。在 [第 13 章](Ch13_Functional_Language_Features_Iterators_and_Closures.md)，会更深入地讲到闭包特性。而此刻，仅需要明白 `unwrap_or_else` 将把那个 `Err` 的内部值，即此示例中的那个此前于清单 12-9 中所添加的静态字符串 `参数数量不足`，传递到这里的闭包中，那个位处于两个竖直管线之间的参数里。那么闭包中的代码，随后就可以在其运行的时候，使用这个 `err` 值了。

这里已添加了一个新的、将标准库的 `process` 带入到作用域中的 `use` 代码行。而将在错误情形下运行的那闭包中的代码，则只有两行：这里打印了那个 `err` 值，并于随后对 `process::exit` 进行了调用。这个 `process::exit` 函数，将立即停止该程序，并返回作为推出状态代码传递的那个数字。这与清单 12-8 中曾使用过的基于 `panic!` 的处理类似，只不过这里不在会受到先前全部的那些额外输出了。现在来尝试运行一下：

```console
$ cargo run                          ✔ 
   Compiling minigrep v0.1.0 (/home/peng/rust-lang/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.66s
     Running `target/debug/minigrep`
解析参数时遇到问题：参数数量不足
```

棒极了！这样的输出对于程序使用者来说，就友好多了。

### 从 `main` 中提取出逻辑

**Extract Logic from `main`**

既然这里已经完成了对配置解析的重构，那么就来转向该程序的逻辑部分。如同在 [“二进制项目的关注点分离”](#separation-of-concerns-for-binary-projects) 小节中所指出的，这里将提取出一个保有当前在这个 `main` 函数中，不涉及到建立配置与错误处理的全部逻辑的 `run` 函数。在完成此过程时，`main` 就变得简洁而易于经由目测得以验证，并能编写出全部其他逻辑的测试。

下面清单 12-11 给出了那个被提取出的 `run` 函数。此时，这里只进行小的、渐进式的提出该函数的改进。此时仍将该函数定义在 `src/main.rs` 中。

文件名：`src/main.rs`

```rust
fn main() {
    // --跳过代码--
    println! ("在文件 {} 中检索：{}", config.file_path, config.query);

    run(config);
}

fn run(config: Config) {
    let contents = fs::read_to_string(config.file_path)
        .expect("应能读取这个这个文件。");

    println! ("有着文本：\n{}", contents);
}

// --跳过代码--
```

*清单 12-11：提取出一个包含了程序逻辑其余部分的 `run` 函数*
 
这个 `run` 函数现在就包含了 `main` 中自读取文件开始的全部剩余逻辑。该 `run` 函数取了那个 `Config` 实例，作为一个参数。


### 从那个 `run` 函数返回错误

在其余程序逻辑分离到这个 `run` 函数之下，就可以改进错误处理了，就跟在清单 12-9 中对 `Config::build` 所做的那样。与其经由调用 `expect` 而允许该程序终止允许，这个 `run` 函数将在发生某种错误时，返回一个 `Result<T, E>` 类型的值。这样做就允许咱们进一步把有关错误处理的逻辑，以用户友好的方式整合到 `main` 中。下面清单 12-12 给出了这里需要对 `run` 的签名及函数体做出的修改。

文件名：`src/main.rs`

```rust
// --跳过代码--
use std::error::Error;

// --跳过代码--

fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;

    println! ("有着文本：\n{}", contents);

    Ok(())
}
```

**清单 12-12：将 `run` 函数修改为返回 `Result`**

这里做出了三处显著修改。首先，这里把这个 `run` 函数的返回值类型，修改为了 `Result<(), Box<dyn Error>>`。此函数先前返回的是单元类型（the unit type），`()`，而这里则将其保留作了 `Ok` 情形中返回的值。

而对于错误类型，这里使用了那个特质对象（the `trait object`） `Box<dyn Error>` （且这里已在代码顶部，使用一条 `use` 语句，而已将 `std::error::Error` 带入到了作用域）。这里将在 [第 17 章](Ch17_Object_Oriented_Programming_Features_of_Rust.md) 讲到特质对象。至于现在，则只要了解那个 `Box<(), Error>` 表示该函数将返回一个实现了 `Error` 特质的类型，而这里不必指明该返回值将是何种特定类型。这就给到了在不同错误情形下，返回值可能为不同类型的灵活性。这个 `dyn` 关键字，是 “动态（dynamic）” 的缩写。

其次，这里通过使用那个 `?` 运算符，而已将到 `expect` 的调用移除，正如在 [第 9 章](Ch09_Error_Handling.md#a-shortcut-for-propagating-errors-the-question-mark-operator) 中曾讲到过的那样。与在某个错误上 `panic!` 不同，`?` 将返回把当前函数中的错误值，返回给调用者来加以处理。

第三，这个 `run` 函数现在会在成功情形下返回一个 `Ok` 值。在函数签名中，这里已将该 `run` 函数的成功类型定义为 `()`，这就意味着需要将那个单元值，封装在 `Ok` 值中。乍一看这个 `Ok(())` 语法或许有点陌生，不过像这样使用 `()`，则正是一种表明这里调用 `run` 只是为了其副作用的方式；他不会返回一个这里所需要的值。

在运行此代码时，此代码将编译，不过将显示出一条告警：

```console
$ cargo run the poem.txt                                                                                                                                                  lennyp@vm-manjaro
   Compiling minigrep v0.1.0 (/home/lennyp/rust-lang/minigrep)
warning: unused `Result` that must be used
  --> src/main.rs:16:5
   |
16 |     run(config);
   |     ^^^^^^^^^^^^
   |
   = note: `#[warn(unused_must_use)]` on by default
   = note: this `Result` may be an `Err` variant, which should be handled

warning: `minigrep` (bin "minigrep") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 1.94s
     Running `target/debug/minigrep the poem.txt`
在文件 poem.txt 中检索：the
有着文本：
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!

```

Rust 告诉咱们，这里的代码忽略了那个 `Result` 值，而该 `Result` 值可能表示发生了某个错误。而这里没有对到底有无错误进行检查，同时编译器提醒了，这里或许是要有一些错误处理代码！下面就来纠正这个问题。

### 在 `main` 中对返回自 `run` 的错误进行处理

这类就要对错误加以检查，并是要与代码清单 12-10 中曾用到的类似技巧，不过要以些许不同的方式，对这些错误加以处理：


文件名：`src/main.rs`

```rust
fn main() {
    // --跳过代码--

    println! ("在文件 {} 中检索：{}", config.file_path, config.query);

    if let Err(e) = run(config) {
        println! ("应用程序错误：{e}");
        process::exit(1);
    }
}
```

这里是要了 `if let` 而非 `unwrap_or_else`，来对 `run` 是否返回一个 `Err` 值加以检查，并在 `run` 确实返回了一个 `Err` 值时，调用 `process::exit(1)`。这个 `run` 函数并未返回一个，这里所要以与`Config::build` 返回的那个 `Config` 实例同样方式，而去 `unwrap` 的值，这里只关心的是探测到某个错误，因此这里就不需要 `unwrap_or_else` 来返回那个解封装值，亦即这里的 `()`。

其中的 `if let` 与 `unwrap_or_else` 两个函数的函数体，在成功及失败两种情形下是同样的：这里都打印出错误并退出程序。


### 将代码分离到库代码箱

到现在这个 `minigrep` 项目看起来就不错了！现在就要拆分这个 `src/main.rs` 文件，并将一些代码放入到 `src/lib.rs` 文件。那样就可以对代码加以测试，并有了一个有着更少职责的 `src/main.rs` 文件。

接下来就要将那些非 `main` 函数的代码，从 `src/main.rs` 迁移到 `src/lib.rs`：

- 那个 `run` 函数的定义；
- 相关的 `use` 语句；
- `Config` 结构体的定义；
- 其中 `Config::build` 函数的定义。


那么 `src/lib.rs` 的内容，就应包含下面清单 12-13 中所显示的那些签名（这里出于简洁考虑，已省略这些函数的函数体）。请注意在清单 12-14 中修改 `src/main.rs` 之前，这还不会编译。

文件名：`src/lib.rs`

```rust
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // --跳过代码--
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    // --跳过代码--
}
```

*清单 12-13：将 `Config` 与 `run` 迁移到 `src/lib.rs` 中*

这里业已大量使用到那个 `pub` 关键字：在 `Config` 上，在其字段与其 `build` 方法上，以及在那个 `run` 函数上。现在这里就有了一个带有可测试公共 API 的库代码箱了！

现在就需要把那些已迁移到 `src/lib.rs` 的代码，带入到 `src/main.rs` 中二进制代码箱的作用域中了，如下清单 12-14 中所示。

文件名：`src/main.rs`

```rust
use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // --跳过代码--
    if let Err(e) = minigrep::run(config) {
        // --跳过代码--
    }
}
```

*清单 12-14：在 `src/main.rs` 中使用 `minigrep` 库代码箱*

这里添加了一个 `use minigrep::Config` 的语句行，来将这个 `Config` 类型，从那个库代码箱，带入到这个二进制代码箱的作用域中，同时把这里的代码箱名字，作为了那个 `run` 函数的前缀。那么现在这全部功能，就应联系起来并生效了。使用 `cargo run` 运行这个程序，并确保所有东西都正确运作。

咦！这可是干了很多活了，还好现在已经为将来的成功做好了准备。现在处理错误就容易多了，同时令到代码更具模块性。从现在开始，几乎咱们的全部工作，就将在 `src/lib.rs` 完成了。

下面就来通过运用现在这种新发现的模组性优势，完成一些对于早先不具模组性代码较难实现，而对这新代码却易于实现的事情。


## 在测试驱动开发方法论下，开发出库的功能

既然已将业务逻辑提取到了 `src/lib.rs` 中，而将参数收集与错误处理留在 `src/main.rs` 中，那么编写这里代码核心功能的测试，就容易多了。这里可直接以不同参数来调用那些函数，并在不必从命令行调用这里二进制程序之下，对他们的返回值加以检查。

在本小节中，这里将按照以下步骤，运用测试驱动开发流程（the test-driven development(TDD) process），把搜索逻辑添加到这个 `minigrep` 程序：

1. 编写一个会失败的测试并加以运行，从而确保其会以所设想的原因失败；
2. 编写或修改仅足够的代码，来令到新的测试通过；
3. 对刚添加或修改过的代码加以重构，并确保那些测试继续通过；
4. 重复步骤 `1` 开始的上述步骤。

尽管这只是众多编写软件方式之一，TDD 是可以推动代码设计的。在编写令到测试通过的代码之前就编写测试，有助于维持贯穿整个开发过程中，较高程度的测试覆盖面。

这里将以测试驱动具体完成搜索出文件内容中查询字符串，以及产生出与该查询匹配的行清单两个功能的实现。这里将把此功能，添加在一个叫做 `search` 的函数里。


### 编写一个失败测试

由于不再需要 `src/lib.rs` 与 `src/main.rs` 中的那些，曾用于对该程序行为加以检查的 `println!` 语句，因此这里就要将其移出掉。随后，就要在 `src/lib.rs` 中，添加带有一个测试函数的 `tests` 模组，就跟曾在 [第 11 章](Ch11_Writing_Automated_Tests.md#the-anatomy-of-a-test-function) 曾做过的那样。该测试函数指明了这里所打算的这个 `search` 函数要有的行为：他将取得一个查询字串，与要搜索的文本，同时他将只返回搜索文本中，包含了查询字串的那些行。下面清单 12-15 给出了这个测试，该清单尚不会编译。

文件名：`src/lib.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";

        assert_eq! (vec! ["safe, fast, productive."], search(query, contents));
    }
}
```

*清单 12-15：创建出一个这里所期望有的那个 `search` 函数的失败测试*

这个测试搜索的是字符串 `"duct"`。而这里正搜索的文本是三个行，三个行中只有一行包含了 `"duct"`（请注意那第一个双引号之后的反斜杠`\`，是告诉 Rust 不要把另起一行字符，放在这个字符串字面值内容的开头）。这里就那个 `search` 函数的返回值，包含了这里所预计的那唯一行进行了断言。

由于这个测试现在甚至不会编译，因此这里尚不能运行这个测试而看到其失败：那个 `search` 函数还不存在！按照 TDD 的各项原则，这里将通过只添加这个 `search` 函数的始终返回某个空矢量值定义，而足够令到这个测试编译并运行的一些代码，如下清单 12-16 中所示。随后该测试将编译，并由于空矢量值不与包含了行 `"safe, fast, productive."` 的矢量匹配而失败。

文件名：`src/lib.rs`

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    vec! []
}
```

*清单 12-16：定义出一个刚好让这里测试编译的那个 `search` 函数来*

请注意这里需要在 `search` 的函数签名中，定义一个显式的生命周期 `'a`，并在 `contents` 参数与返回值上，使用那个生命周期。回顾 [第 10 章](Ch10_Generic_Types_Traits_and_Lifetimes.md#validating-references-with-lifetimes) 中讲到，这些生命周期参数指明了哪个参数生命周期，是与返回值生命周期联系起来的。在这个示例中，这就表示那个返回的矢量，应包含引用了参数 `contents` （而非参数 `query`）的一些切片的字符串切片。

也就是说，这里告诉 Rust，由 `search` 函数返回的数据，将存活到与传递给那个 `search` 函数的、在 `contents` 参数中数据同样长时间。这是相当重要的！*为* 某个切片所引用的数据，需要在该引用有效期间保持有效；若编译器假定这里是在构造 `query` 而非 `contents` 的字符串切片，那么他就会执行错误地安全性检查。

而在忘掉了这些生命周期注解并尝试编译该函数时，就会得到下面这个错误：


```console
$ cargo build                                                                                                  lennyp@vm-manjaro
   Compiling minigrep v0.1.0 (/home/lennyp/rust-lang/minigrep)
error[E0106]: missing lifetime specifier
  --> src/lib.rs:35:51
   |
35 | pub fn search(query: &str, contents: &str) -> Vec<&str>{
   |                      ----            ----         ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `query` or `contents`
help: consider introducing a named lifetime parameter
   |
35 | pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str>{
   |              ++++         ++                 ++              ++

For more information about this error, try `rustc --explain E0106`.
error: could not compile `minigrep` due to previous error
```

Rust 是不可能明白，这里需要的到底是两个参数中哪一个的，因此这里就需要显式地告诉 Rust。而由于 `contents` 正是那个包含了这里全部文本的参数，而这里打算返回的，就是与那个文本匹配的部分，因此这里清楚 `contents` 就应是要运用生命周期语法，将其与返回值联系起来的那个参数。

别的编程语言并不会要求在函数签名中，将参数与返回值联系起来，但随着时间的推移，这样的实践将变得容易起来。或许你会将这个示例，与第 10 章中的 [“使用生命周期对引用进行验证” 小节](Ch10_Generic_Types_Traits_and_Lifetimes.md#validating-references-with-lifetimes) 加以比较。

现在来运行测试：

```console
$ cargo test                                                                                            12m 0s lennyp@vm-manjaro
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/minigrep-7d3f5b041202a66e)

running 1 test
test tests::one_result ... FAILED

failures:

---- tests::one_result stdout ----
thread 'tests::one_result' panicked at 'assertion failed: `(left == right)`
  left: `["safe, fast, productive."]`,
 right: `[]`', src/lib.rs:51:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::one_result

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--lib'
```

相当棒，这个测试失败了，就如这里的预期一样。接下来就要让这个测试通过！


### 编写让测试通过的代码

此刻，由于这里始终返回一个空的矢量值，导致这里的测试失败。要修复这个测试失败并实现 `search`，这里的程序就需要遵循下面这些步骤：

- 对那个内容的各个行加以迭代；
- 检查该行是否包含这里的查询字串；
- 在包含查询字串时，将该行添加到这里正要返回的值清单；
- 在不包含查询字串时，就什么也不做；
- 返回匹配结果的清单。

下面就来逐一完成各个步骤，从那些文本行的迭代开始。


**使用 `lines` 方法对文本行进行遍历**

Rust 有着一个用于处理字符串一行行迭代的有用方法，其被方便地命名为了 `lines`，如下清单 12-17 中所示的那样运作。请注意下面的代码尚不会编译。

文件名：`src/lib.rs`

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    for line in contents.lines() {
        // 对单个文本行进行一些操作
    }
}
```

*清单 12-17：遍历 `contents` 中的各行*

这个 `lines` 方法，返回的是个迭代器（an iterator）。在 [第 13 章](Ch13_Functional_Language_Features_Iterators_and_Closures.md#processing-a-series-of-items-with-iterators) 中，就会讲到迭代器，不过回顾一下 [清单 3-5](Ch03_Common_Programming_Concepts.md#looping-through-a-collection-with-for) 中，就曾见过这种用到迭代器的方式，那里曾用到一个 `for` 循环, 就带有一个用于在集合中各个元素上，运行某些代码的迭代器。


**在各行中搜索那个查询字串**

接下来，这里就要检查当前行是否包含着这里的查询字串。幸运的是，字符串有着一个为咱们完成这件事的名为 `contains` 的有用方法！在 `search` 函数中添加一个到这个 `contains` 方法的调用，如下清单 12-18 中所示。请注意这仍上不会编译。

文件名：`src/lib.rs`

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    for line in contents.lines() {
        if line.contains(query) {
            // 对文本行执行某些操作
        }
    }
}
```

*清单 12-18：加入检视该行是否包含 `query` 中字符串的功能*

此刻，这里正在构建起功能来。而要让代码编译，就需要从其中的函数体，返回一个在该函数签名中，曾表明的应返回的某个值。


**对匹配的那些行进行存储**

要完成这个函数，就需要某种对这里打算返回的那些匹配行，加以存储的方法。为那个目的，这里可以在其中的 `for` 循环之前，构造出一个可变矢量（a mutable vector），并调用 `push` 方法，来把某个 `line` 存储在该矢量中。在那个 `for` 循环之后，这里就返回那个矢量，如下清单 12-19 中所示。

文件名：`src/lib.rs`

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
```

*清单 12-19：对匹配的那些行进行存储，如此就可以返回他们了*

现在这个 `search` 函数就应只返回那些包含了 `query` 的行了，同时这里的测试应通过。下面就来运行该测试：

```console
$ cargo test                                                                                                   lennyp@vm-manjaro
   Compiling minigrep v0.1.0 (/home/lennyp/rust-lang/minigrep)
    Finished test [unoptimized + debuginfo] target(s) in 0.49s
     Running unittests src/lib.rs (target/debug/deps/minigrep-7d3f5b041202a66e)

running 1 test
test tests::one_result ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/minigrep-38ae0a181a4574d5)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests minigrep

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

这里的测试通过了，进而咱们就明白 `search` 函数是工作的了！

到这里，咱们就会在保持这些测试通过，以维持这同样功能的同时，考虑对这个 `search` 函数的实现，进行重构的一些机会。这个 `search` 函数中的代码虽不怎么差劲，但他并没有利用上迭代器的一些有用特性。在 [第 13 章](Ch13_Functional_Language_Features_Iterators_and_Closures.md#processing-a-series-of-items-with-iterators) 中将回到这个示例，那里就会详细探讨到迭代器，进而会看看怎样来改进这个 `search` 函数。


**在 `run` 函数中使用这个 `search` 函数**

既然 `search` 函数运作起来并被测试过，那么这里就需要在这里的 `run` 函数中，调用 `search` 了。这里需要将那个 `config.query` 值与 `run` 从文件中读取到的 `contents`，传递给这个 `search` 函数。随后 `run` 将打印出从 `search` 返回的各行：

文件名：`src/lib.rs`

```rust
pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println! ("{line}");
    }

    Ok(())
}
```

这里仍使用一个 `for` 循环，来返回来自 `search` 的各行并将其打印出来。

现在这整个程序就应工作了！接下来就要试一下他了，首先以一个应确切地从这首 Emily Dickinson 的诗返回一行的词，“frog”：

```console
$ cargo run -- frog poem.txt                                                                                   lennyp@vm-manjaro
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep frog poem.txt`
在文件 poem.txt 中检索：frog
How public, like a frog
```

酷！现在来试一个将匹配多行的词，比如 “body”：

```console
$ cargo run -- body poem.txt                                                                                   lennyp@vm-manjaro
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep body poem.txt`
在文件 poem.txt 中检索：body
I'm nobody! Who are you?
Are you nobody, too?
How dreary to be somebody!
```

相当棒！这里已经构建了一个经典工具自己的小型版本，并掌握了很多有关如何建构应用程序的知识。这里还了解到有关文件输入输出、生命周期、测试及命令行参数解析等方面的点滴内容。

而为了完善这个项目，接下来就主要会演示怎样使用环境变量，以及怎样打印到标准错误输出（print to standard error），在编写命令行程序时，这两方面的知识都是有用的。

## 运用环境变量

**Working with Environment Variables**

这里就要通过加入一项额外特性，来改进 `minigrep`：经由使用某个环境变量，用户可以开启与关闭的区分大小写的搜索选项。这里本可以将此特性，构造为一个命令行选项，并在用户打算应该该选项时，要求他们键入该命令行选项，而不是将其构造为一个环境变量，这样就允许用户只设置该环境变量一次，而在那次终端会话中的全部搜索，都是区分大小写的了。


### 为这个区分大小写的 `search` 函数编写出一个失败测试

首先这里要加入一个新的、在该环境变量有着某个值时会调用到的  `search_case_insensitive` 函数。这里将继续遵循 TDD 流程，因此第一步就是要再度编写一个失败测试（a failing test）。这里将给这个新的 `search_case_insensitive` 函数，添加一个新的测试，并将其中原来的测试，从 `one_result` 改名为 `case_sensitive`，以区分这里两个测试的不同之处，如下清单 12-20 中所示。


文件名：`src/lib.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq! (vec! ["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq! (
            vec! ["Rust:", "Trust me."], 
            search_insensitive(query, contents)
        );
    }
}
```

*清单 12-20：给那个即将添加的不区分大小写函数添加一个新的失败测试*

请注意这里也已编辑过原先测试的 `contents` 了。这里添加了一个有着文本 `"Duct tape."` 的新行，其中用到一个在以区分大小写方式进行搜索时，不应与查询字串 `"duct"` 匹配的大写字母 D。以这种方式修改原来的那个测试，有助于确保这里不会意外破坏这里已经实现了的区分大小写检索功能。这个区分大小写的测试，现在应会通过，并应在实现不区分大小写检索过程中继续通过测试。

那个新的不区分大小写检索的测试，使用了 `"rUsT"` 作为其查询字串。在那个这里即将添加的 `search_case_insensitive` 函数中，该查询字串 `"rUsT"` 应匹配到包含有着大写字母 R 的 `"Rust:"` 行，并匹配到行 `"Trust me."`，尽管这两行都有着与该查询字串不同的大小写。这就是这里的失败测试，而由于这里尚未定义出那个 `search_case_insensitive` 函数，因此该测试将会失败。请随意添加一个始终返回空矢量值的骨架实现，就跟在清单 12-16 中对 `search` 函数所做的类似，来对测试编译与失败加以检视。

### 实现这个 `search_case_insensitive` 函数

在下面清单 12-21 中所给出的这个 `search_case_insensitive` 函数，将与那个 `search` 函数几乎完全一样。唯一区别就是，这里将把其中的 `query` 与各个 `line` 做小写的处理，这样一来不论输入的参数是大写还是小写，在就该行是否包含查询字串时，他们都将是同样的拼写。

文件名：`src/lib.rs`

```rust
pub fn search_insensitive<'a>(
    query: &str, 
    contents: &'a str
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
```

*清单 12-21：定义出一个在对查询字串与文本行进行比较前，先对他们进行小写处理的 `search_case_insensitive` 函数*

首先，这里把那个 `query` 字符串进行小写处理，并将其存储在一个有着同样名字的遮蔽变量中（in a shadowed variable with the same name）。在查询字串上调用 `to_lowercase` 是必要的，如此不用户的查询为 `"rust"`、`"RUST"`、`"Rust"` 还是 `rUsT`，这里都将把查询字串，当作其为 `rust` 处理，而变得不区分大小写。尽管 `to_lowercase` 会处理基本 Unicode 字符，但他并不会 100% 精确。因此在编写真正应用时，这里就要完成些许更多的工作，但由于本小节是有关环境变量，而非 Unicode，因此这里就点到为止了。

请注意由于调用 `to_lowercase` 会创建出一个新数据，而非引用既有收据，因此现在的 `query` 就是一个新的 `String` 了。比如说查询字串为 `rUsT`：那个字符串切片并不包含这里要用到的小写字母 `u` 或 `t`，因此这里就不得不分配一个新的包含着 `rust` 的 `String` 变量。现在将 `query` 作为参数，传递给 `contains` 是，由于 `contains` 的函数签名被定义为取一个字符串切片，因此这里就需要添加一个地址符号 `&`。

接下来，这里在各个 `line` 上添加了到 `to_lowercase` 的调用，来对全部字符小写处理。现在就已将 `line` 及 `query` 转换成了小写，这里就将找出与查询字串大小写无关的那些匹配了。

现在来看看这种实现是否通过那些测试：

```console
$ cargo test                                                            ✔ 
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/minigrep-7d3f5b041202a66e)

running 2 tests
test tests::case_insensitive ... ok
test tests::case_sensitive ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/minigrep-38ae0a181a4574d5)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests minigrep

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

很棒！他们都通过了。现在，就要在那个 `run` 函数中，调用这个新的 `search_case_insensitive` 函数了。首先，这里将把一个配置项，添加到其中的 `Config` 结构体，来在区分大小写与不区分大小写检索之间加以切换。由于尚未在任何地方对这个字段进行初始化，因此这个字段的添加，将导致一些编译器错误：

文件名：`src/lib.rs`

```rust
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}
```

这里添加了那个保存了一个布尔值（a Boolean） 的 `ignore_case` 字段。接下来，这里就需要那个 `run` 函数来检查这个 `ignore_case` 字段值，并使用该值来确定是要调用 `search` 函数还是 `search_case_insensitive` 函数，如下清单 12-22 中所示。这代码仍将不会编译。

文件名：`src/lib.rs`


```rust
pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;

    let results: Vec<&str> = if config.ignore_case {
        search_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    }

    for line in results {
        println! ("{line}");
    }

    Ok(())
}
```

*清单 12-22：依据 `config.ignore_case` 中的值，调用 `search` 还是 `search_case_insensitive`*

最后，这里需要就环境变量加以检查了。用于处理环境变量的那些函数，位于便准库的 `env` 模组中，因此这里就要在 `src/lib.rs` 的顶部，把那个模组带入到作用域中来。随后这里就会使用 `env` 模组中的 `var` 函数，来检视是否已有给名为 `IGNORE_CASE` 设置某个值，如下清单 12-23 中所示。

文件名：`src/lib.rs`

```rust
use std::env;
// --跳过代码--

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("参数数量不足");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { 
            query, 
            file_path,
            ignore_case,
        })
    }
}
```

*清单 12-23：就一个名为 `IGNORE_CASE` 的环境变量中的值进行检查*

这里创建了一个新的变量 `ignore_case`。而为设置他的值，这里调用了 `env::var` 函数，并传递给了其那个 `IGNORE_CASE` 环境变量的名字。这个 `env::var` 函数返回的是一个 `Result` 值。在该环境变量有曾被设置为某个值时，其就会返回一个包含了该环境变量值的成功 `Ok` 变种。而在该环境变量未曾被设置时，该函数将返回 `Err` 变种。

这里在那个返回的 `Result` 上使用了 `is_ok` 方法，来检查该环境变量是否有被设置，这就意味着该程序应完成一次不区分大小写的检索。在这个 `IGNORE_CASE` 环境变量未被设置为某个值时，那么 `is_ok` 就会返回 `false`，而这个程序就会执行一次区分大小写的检索。这里并不关系那个环境变量的 *值*，而只关心他是否被设置或未设置，因此这里使用的就是 `is_ok`，而非使用 `unwrap`、`expect` 或其他任何已见到过的 `Result` 上的那些方法。

这里把在 `ignore_case` 变量中的值，传递给了那个 `Config` 实例，这样一来 `run` 函数就可以读取到那个值，并判定是要调用 `search_case_insensitive` 还是 `search`，就如同在清单 12-22 中所实现的那样。

现在就来试着运行一下！首先，这里将在未设置那个环境变量及查询字串为 `to` 之下，运行这个程序，这样应匹配到包含了全部小写单词 “to” 的那些行：

```rust
$ cargo run -- to poem.txt                                              ✔ 
   Compiling minigrep v0.1.0 (/home/peng/rust-lang/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.55s
     Running `target/debug/minigrep to poem.txt`
在文件 poem.txt 中检索：to
Are you nobody, too?
How dreary to be somebody!
```

看起来那代码仍会工作！现在，就在 `IGNORE_CASE` 被设置为 `1`，而查询字串同样为 `to` 之下，运行这个程序。


```console
$ IGNORE_CASE=1 cargo run -- to poem.txt                                ✔ 
```

在使用的是 PowerShell 时，就需要用单独的命令，来设置该环境变量与运行这个程序：

```PowerShell
PS> $Env:IGNORE_CASE=1; cargo run -- to poem.txt
```

这样就会令到 `IGNORE_CASE` 持续到本次 shell 会话终止为止。使用 `Remove-Item` cmdlet 其就可以被清除设置。

```PowerShell
PS> Remove-Item Env:IGNORE_CASE
```

这里应得到包含了可能有着大写字母 "to" 的那些行：

```console
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```

非常好，这里还得到了包含着 “To” 的那些行了！这里的 `minigrep` 现在可以完成，由一个环境变量控制的不区分大小写检索了。现在就清楚了怎样运用命令行参数，或是环境变量，来管理程序选项集了。

有的程序，同时实现同一配置的命令行参数 *与* 环境变量。在这样的情形下，这些程序就会确定下其中之一有着较高优先级。好比你自己的另一代码练习中，就会尝试经由命令行参数，或同时经由环境变量，对是否区分大小写进行控制。就会在程序在一种设置下区分大小写，而另一种设置下不区分大小写时，对到底命令行参数优先，还是环境变量优先，加以确定。

这个 `std::env` 模组，包含了许多用于处理环境变量的其他有用特性：请查看其文档来看看有哪些可用特性。


## 把错误消息写到标准错误而非标准输出

**Writing Error Messages to Standard Error Instead of Standard Output**

- 标准错误输出：standard error 
- 标准输出：standard output

到目前为止，咱们都是在把全部输出，使用那个 `println!` 宏输出到终端。而在绝大多数终端里，都有着两种类型的终端：用于通用信息的 *标准输出* （ *standard output*，`stdout`），及用于错误消息的 *标准错误* （ *standard error*，`stderr`）。这种区别，就可以让用户选择把程序的成功输出，导向某个文件，而仍把错误消息，打印到屏幕上。

那个 `println!` 宏，只能打印到标准输出，因此这里就不得不使用其他物件，来打印到标准错误了。

### 对错误被写到何处进行检视

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
