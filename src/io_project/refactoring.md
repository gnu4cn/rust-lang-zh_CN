# 重构以改进模块化与错误处理

**Refactoring to Improve Modularity and Error Handling**


为改进这个程序，这里就要修复与该程序结构及其处理潜在错误方式有关的四个问题。首先，这里的 `main` 函数现在执行了两个任务：他对参数进行解析并读取文件。随着程序的增长，这个 `main` 函数所处理的独立任务数目将不断增加。而随着函数不断获得其任务，就会变得更加难于推理，更难于对其进行测试，以及更难于在不破坏其各个部分的情况下对其进行修改。那么最后就要将功能拆分，从而每个函数负责一项任务。

这个问题同样联系着第二个问题：尽管这里的 `query` 与 `file_path` 属于这个程序的配置性变量，而像 `contents` 这样的变量则被用于执行该程序的逻辑处理。这个 `main` 函数变得越长，那么这里就会将更多的变量引入到作用域；在作用域中的变量越多，那么就会越难对各个变量的目的保持追踪。因此就最好将这些配置变量，分组到某个结构体中，而令到他们的目的明确。

第三个问题则是，在读取那个文件失败时，这里使用了 `expect` 将一条错误消息打印处理，而该错误消息只会打印 “应能读取这个这个文件。” 文件读取以多种方式失败：比如那个文件可能没有，或可能没有打开他的权限。此时，无论何种情形，这里都将打印同样的错误消息，这样并不会给到用户任何信息！

第四，这里重复地使用了 `expect` 来处理不同重复，而在用户未指定足够参数时，他们就会得到一个并不会清楚解释问题原因、 Rust 的 `index out of bounds` 错误。若全部错误处理代码都在一个地方，那么就最好了，这样在错误处理代码需要修改时，那么以后的维护者就只有一个地方来查阅代码。将全部错误处理代码放在一处，还将确保这里打印的消息，是会对终端用户有意义的那些消息。

下面就来通过对这里的项目进行重构，来解决这四个问题。


## 二进制程序项目的关注点分离

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


### 提取命令行参数解析器

**Extracting the Argument Parser**


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

**Creating a Constructor for `Config`**


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


## 修复错误处理

**Fixing the Error Handling**


现在就要开始修复这里的错误处理了。回顾到之前在尝试访问 `args` 矢量中索引 `1` 或索引 `2` 处的值，若该矢量包含了少于三个条目，那么就会导致该程序终止运行。请以不带任何参数运行这个程序；他就会看起来像下面这样：

```console
cargo run                                                                               lennyp@vm-manjaro
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep`
thread 'main' panicked at 'index out of bounds: the len is 1 but the index is 1', src/main.rs:24:21
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

其中的行 `index out of bounds: the len is 1 but the index is 1` 是一条留给代码编写者的错误消息。该消息不会帮到终端用户，搞明白又该怎样做。现在就要来修复这个问题。


### 改进错误消息

**Improving the Error Message**


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


此代码与 [清单 9-13 中曾编写过的 `Guess::new` 函数](Ch09_Error_Handling.md#创建用于验证的定制类型) 类似，其中在那个 `value` 参数超出有效值边界时，就调用了 `panic!` 宏。这里没有检查值的边界，而是就 `args` 的长度至少为 `3` 进行了检查，进而该函数的其余部分，就可以在此条件已满足的假定下运作了。在 `args` 所拥有的条目少于三个时，此条件便为真，进而这里就会条约那个 `panic!` 宏，来立即结束这个程序。

有了`new` 中的这些额外少数几行，下面就不带任何参数地再度运行这个程序，来看看现在错误看起来如何：

```console
$ cargo run                                                                            lennyp@vm-manjaro
   Compiling minigrep v0.1.0 (/home/lennyp/rust-lang/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.57s
     Running `target/debug/minigrep`
thread 'main' panicked at '参数数量不足', src/main.rs:25:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

此输出好了一些：现在这里就有了一个合理的错误消息了。不过，这里还有一些不希望给到用户的无关信息。或许运用曾在清单 9-13 中用到的那种技巧，并非这里要用到的最佳技巧：到 `panic!` 的调用，相比于用法方面的问题，是更适合于编程方面的问题的，如同 [第 9 章中所讨论的那样](Ch09_Error_Handling.md#错误处理守则)。相反，这里将使用之前在第 9 章中曾学到的另一项技能 -- [返回一个 `Result`](Ch09_Error_Handling.md#带有-result-的可恢复错误)，以表示成功执行成功或是出错。


### 返回 `Result` 值，而非调用 `panic!` 宏

**Returning a `Result` Instead of Calling `panic!`**


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


### 调用 `Config::build` 并处理错误

**Calling `Config::build` and Handling Errors**


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


## 提取 `main` 中得逻辑

**Extract Logic from `main`**


既然这里已经完成了对配置解析的重构，那么就来转向该程序的逻辑部分。如同在 [“二进制项目的关注点分离”](#二进制程序项目的关注点分离) 小节中所指出的，这里将提取出一个保有当前在这个 `main` 函数中，不涉及到建立配置与错误处理的全部逻辑的 `run` 函数。在完成此过程时，`main` 就变得简洁而易于经由目测得以验证，并能编写出全部其他逻辑的测试。

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


### 返回 `run` 函数中的错误

**Returning Errors from the `run` Function**


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

其次，这里通过使用那个 `?` 运算符，而已将到 `expect` 的调用移除，正如在 [第 9 章](Ch09_Error_Handling.md#传递错误的快捷方式-操作符) 中曾讲到过的那样。与在某个错误上 `panic!` 不同，`?` 将返回把当前函数中的错误值，返回给调用者来加以处理。

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


### 处理自 `main` 中的 `run` 所返回的错误

**Handling Errors Returned from `run` in `main`**


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


## 拆分代码到库代码箱

**Splitting Code into a Library Crate**


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
