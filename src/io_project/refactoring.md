# 重构以改进模组化与错误处理

为了改进我们的程序，我们将解决四个与程序结构及其潜在错误处理方式相关的问题。首先，我们的 `main` 函数现在执行两项任务：解析命令行参数和读取文件。随着我们程序的增长，`main` 函数处理的单独任务的数目将会增加。随着函数承担的责任越来越多，他会变得更加难于推理、更难于测试，并且在不破坏其某个部分的情况下修改也愈发困难。最好要分离功能，以便每个函数负责一项任务。

这个问题还与第二个问题有关：尽管 `query` 和 `file_path` 属于我们程序的配置变量，但像 `contents` 这样的变量则被用于执行程序的逻辑。`main` 函数变得越长，我们将需要带入作用域的变量就越多；我们在作用域中的变量越多，跟踪每个变量的目的就越困难。因此最好分组配置变量为一种数据结构，使其目的明确。

第三个问题则是，我们使用了 `expect` 在读取文件失败时打印一条错误消息，但错误消息只是打印 `应该已经能够读取文件`。读取文件可能会以多种方式失败：比如，文件可能丢失，或我们可能没有打开他的权限等等。目前，无论何种情形，我们都将针对所有原因打印同一条错误消息，这不会给予用户任何信息！

第四，我们使用 `expect` 来处理错误，当用户在未指定足够参数的情况下运行我们的程序时，他们将得到 Rust 的一个 `index out of bounds` 报错，而这个报错并不能清楚地解释问题。若所有错误处理代码都在一处就最好，以便今后的维护人员在错误处理逻辑需要修改时，就只需在一个地方查阅代码。将所有的错误处理代码放在一处，还将确保我们打印的是对最终用户有意义的消息。

我们来通过重构我们的项目解决这四个问题。


## 二进制项目中的关注点分离

指派多个任务的责任给 `main` 函数的组织问题，常见于许多二进制项目。因此，许多 Rust 程序员发现，当 `main` 函数开始变大时，拆分二进制程序的各个关注点很有用。这一过程有着以下步骤：

- 拆分程序为 `main.rs` 文件和与 `lib.rs` 文件，并迁移咱们程序的逻辑到 `lib.rs`；
- 只要咱们的命令行解析逻辑很小，他就可以保留在 `main` 函数中；
- 当命令行解析逻辑开始变得复杂时，就要从 `main` 函数提取到其他函数或类型中。

在经历这一过程后，保留在 `main` 函数中的职责，应仅限于以下这些：

- 以参数值调用命令行解析逻辑；
- 建立任何其他配置；
- 调用 `lib.rs` 中的 `run` 函数；
- 当 `run` 返回错误时处理错误。

这种模式是为实现关注点分离：`main.rs` 负责程序运行，而 `lib.rs` 处理当前任务的所有逻辑。因为咱们无法直接测试 `main` 函数，这种结构通过把程序的所有逻辑迁出 `main` 函数，让咱们可以测试程序的所有逻辑。保留在 `main` 函数中的代码将足够小，以至通过阅读他即可验证其正确性。我们来按照这一过程重写我们的程序。


### 提取参数解析器

我们将提取解析参数的功能到一个 `main` 会调用的函数中。下面清单 12-5 展示了 `main` 函数的新开头，其调用了一个新函数 `parse_config` ，我们将在 `src/main.rs` 中定义他。

<a name="listing_12-5"></a>
文件名：`src/main.rs`

```rust
use std::{env, fs};

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

**清单 12-5**：自 `main` 函数中提取 `parse_config` 函数

我们仍收集命令行参数到一个矢量中，但不再指派索引 1 处的参数值给变量 `query`，及索引 2 处的参数值给变量 `file_path`，我们传递整个矢量值给 `parse_config` 函数。然后 `parse_config` 函数保有确定哪个放入哪个变量的逻辑，并传回值给 `main` 函数。我们仍在 `main` 中创建 `query` 和 `file_path` 变量，但 `main` 不再负责确定命令行参数与变量如何对应。

对于我们这个小程序来说，这一重写似乎有些小题大作，但我们正在以小步渐进的方式进行重构。在进行这一改动后，再次运行程序以验证参数解析是否仍能正常工作。经常检查咱们的进展是好的做法，有帮助于在问题发生时确定问题的原因。


### 分组配置值

我们可以再采取一小步，来进一步改进 `parse_config` 函数。目前，我们正返回一个元组，但随后我们立即又拆分该元组为单独部分。这是个我们或许还没有正确的抽象的迹象。

另一个表明仍有改进空间的指标是 `parse_config` 的 `config` 部分，他暗示我们返回的两个值是相关的，并且都是一个配置值的一部分。除了分组这两个值编为元组外，我们目前并未以数据结构传达这种含义；我们将改为放置这两个值于一个结构体中，并为结构体的每个字段取个意义的名字。这样做将使这段代码的未来维护者更容易理解不同值如何相互关联以及他们的目的。

下面清单 12-6 展示了对 `parse_config` 函数的改进。

<a name="listing_12-6"></a>
文件名：`src/main.rs`

```rust
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println! ("
        在文件 {} 中
        检索 {}", config.file_path, config.query);

    let contents = fs::read_to_string(config.file_path)
        .expect("应该已经能够读取文件");

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

**清单 12-6**：重构 `parse_config` 以返回 `Config` 结构体的实例

我们添加了个名为 `Config` 的结构体，被定义为有着名为 `query` 与 `file_path` 两个字段。现在 `parse_config` 的签名表明他返回一个 `Config` 值。在 `parse_config` 的函数体中，我们曾返回引用 `args` 中 `String` 值的字符串切片，现在则定义 `Config` 为包含自有的 `String` 值。`main` 中的 `args` 变量是参数值的所有者，并且只允许 `parse_config` 函数借用他们，这意味着当 `Config` 尝试取得 `args` 中值的所有权时，我们将违反 Rust 的借用规则。

我们可以通过数种方式管理 `String` 数据；最简单但效率有些低的方法是调用值上的 `clone` 方法。这将为 `Config` 实例持有而构造数据的完整拷贝，相比存储到字符串数据的引用，这会消耗更多时间与内存。然而，克隆数据也使我们的代码变得非常简单，因为我们不必管理引用的生命周期；在这一情形下，放弃一点性能以获得简单性是值得的权衡。

> **使用 `clone` 的权衡**
>
> 由于 `clone` 的运行时开销，许多 Rustaceans 倾向于避免使用 `clone` 来解决所有权问题。在 [第 13 章](./Ch13_Functional_Language_Features_Iterators_and_Closures) 中，咱们将学习如何在这种情形下使用更高效的方法。但目前，复制几个字符串以继续取得进展是可以的，因为咱们只会构造一次这些拷贝，并且咱们的文件路径和查询字符串都非常小。相比于一开始就尝试过渡优化代码，hyperoptimize code，有个不那么高效的正常运行的程序会更好。随着咱们对 Rust 日益熟练，从最高效的解决方案入手就会变得更容易，但目前，调用 `clone` 完全是可接受的。

我们已更新 `main` 以便其放置由 `parse_config` 返回的 `Config` 实例到名为 `config` 的变量中，并且更新了之前使用单独的 `query` 和 `file_path` 的代码，以便其现在改用 `Config` 结构体上的字段。

现在我们的代码更清楚地传达了 `query` 和 `file_path` 是相关的，并且他们的目的是配置程序的工作方式。任何使用这两个值的代码就都知道，要在 `config` 实例中，以其目的命名的字段中找到他们。


### 创建 `Config` 的构造器

到目前为止，我们已从 `main` 中提取了负责解析命令行参数的逻辑，并将其放置于 `parse_config` 函数中。这样做帮助我们看出 `query` 与 `file_path` 两个值是相关的，并且这种关系应在我们的代码中得以传达。然后，我们添加了个 `Config` 的结构体来命名 `query` 与 `file_path` 的这种关联目的，并能够从 `parse_config` 函数作为字段的名字返回值的名字。

因此，现在 `parse_config` 函数的目的是创建一个 `Config` 实例，我们可以将 `parse_config` 从普通函数修改为与 `Config` 结构体关联的，名为 `new` 的函数。进行这一修改将使代码更符合惯例。我们可以创建标准库的类型的实例，比如通过调用 `String::new` 创建 `String` 值。类似地，通过修改 `parse_config` 为与 `Config` 关联的 `new` 函数，我们将能够通过调用 `Config::new` 创建 `Config` 的实例。下面清单 12-7 显式了我们需要做出的修改。

<a name="listing_12-7"></a>
文件名：`src/main.rs`

```rust
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

**清单 12-7**：修改 `parse_config` 为 `Config::new`*


我们已更新 `main`，其中原先我们调用 `parse_config` 的地方改为调用 `Config::new`。我们已修改 `parse_config` 的名字为 `new`，并将其迁移到 `impl` 代码块中，这个代码块将 `new` 函数与 `Config` 关联起来。请尝试再次编译这段代码以确保其正常运行。


## 修复错误处理

现在我们将着手修复错误处理。回想以下，当矢量值 `args` 包含少于三个项目时，尝试访问其中索引 1 或索引 1 处的值将导致程序终止运行。尝试不带任何参数的情况下运行这个程序；他将看起来像下面这样：

```console
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep`

thread 'main' (302106) panicked at src/main.rs:25:21:
index out of bounds: the len is 1 but the index is 1
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

其中的行 `index out of bounds: the len is 1 but the index is 1` 是留给程序员的错误消息。他不会帮到我们的最终用户了解他们应该做什么。我们来解决这个问题。


### 改进错误消息

在下面清单 12-8 中，我们在 `new` 函数中添加了一项检查，将在访问索引 1 和索引 2 之前验证切片是否足够长。当切片不够长时，程序就会终止运行并显示一条更好的错误消息。


<a name="listing_12-8"></a>
文件名：`src/main.rs`

```rust
    // --跳过代码--
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic! ("参数不足");
        }
        // --跳过代码--
```

**清单 12-8**：添加对参数数目的检查

这段代码类似于 [我们在清单 9-13 中编写的 `Guess::new` 函数](../error_handling/panic_or_not.md#listing_9-13) ，其中我们在 `value` 参数超出有效值范围时调用了 `panic!`。这里我们不再检查值的范围，而是检查 `args` 的长度是否至少为 `3`，进而函数的其余部分可以在此条件满足的假设下运行。当 `args` 的项目少于三个时，这一条件将为 `true`，进而我们调用 `panic!` 宏来立即结束程序。

在`new` 中的额外这几行代码下，我们来再次不带任何参数运行该程序，看看报错现在如何：

```console
$ cargo run
   Compiling minigrep v0.1.0 (/home/hector/rust-lang-zh_CN/projects/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/minigrep`

thread 'main' (305263) panicked at src/main.rs:26:13:
参数不足
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

这个输出更好：我们现在有条合理的错误消息。然而，我们有了一些不打算提供给用户的无关信息。也许我们在清单 9-13 中使用的技巧并不适合这里：正如 [第 9 章中讨论的](../error_handling/panic_or_not.md#错误处理指南准则)，调用 `panic!` 更适合编程问题，而非使用问题。相反，我们将使用咱们在第 9 章中学到的另一技巧 -- [返回 `Result`](../error_handling/result.md)，表示成功或出错。


### 返回 `Result` 而不是调用 `panic!`

我们可以改为返回一个 `Result` 值，将在成功的情形下包含一个 `Config` 实例，在错误情形下则将描述问题。我们还将把函数名字从 `new` 修改为 `build`，因为许多程序员都会期望 `new` 函数永远不会失败。当 `Config::build` 与 `main` 通信时，我们可以使用 `Result` 类型来发出存在问题的信号。然后，我们可以修改 `main` 为将 `Err` 变种转换为对用户更实用的错误消息，而不会有调用 `panic!` 宏导致的前后的有关 `thread 'main'` 及 `RUST_BACKTRACE` 等字眼。

下面清单 12-9 展示了我们需要对现在我们称为 `Config::Build` 的函数的返回值作出的修改，以及该函数为返回 `Result` 值所需的函数体。请注意，除非我们也更新 `main`，否则这段代码不会编译，我们将在下一清单中进行这一更新。

<a name="listing_12-9"></a>
文件名：`src/main.rs`

```rust
impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("参数不足");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
```

**清单 12-9**：从 `Config::build` 返回一个 `Result`

我们的 `build` 函数在成功情形下返回带有 `Config` 实例的 `Result` 值，在错误情况下返回有着一个字符串字面值的 `Result` 值。我们的错误值将始终是有着 `'static` 生命周期的字符串字面值。

我们在该函数的函数体中进行了两处修改：当用户未传递足够参数时我们不再调用 `panic!`，现在返回一个 `Err` 值，并且我们将 `Config` 返回值包装在 `Ok` 中。这些修改使该函数符合其新的类型签名。

从 `Config::build` 返回 `Err` 值，使 `main` 函数能够处理 `build` 函数返回的 `Result` 值，进而在错误情形下能够更干净地退出进程。


### 调用 `Config::build` 并处理错误

为了处理错误情形并打印用户友好的消息，我们需要更新 `main` 以处理 `Config::build` 返回的 `Result` 值，如下清单 12-10 中所示。我们还将负责在不使用 `panic!` 宏后，手动实现以非零错误代码退出这个命令行工具。非零退出状态属于一种约定，用来像调用我们程序的进程发出信号，表明程序以错误状态退出了。

<a name="listing_12-10"></a>
文件名：`src/main.rs`

```rust
use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println! ("解析参数时遇到问题：{err}");
        process::exit(1);
    });

    // --跳过代码--
```

**清单 12-10**：构建 `Config` 失败时以错误代码退出

在这个代码清单中，我们使用了个尚未详细介绍的方法：`unwrap_or_else`，他是由标准库定义在 `Result<T, E>` 上。使用 `unwrap_or_else` 允许我们定义一些定制的、非 `panic!` 的错误处理。当 `Result` 是 `Ok` 值时，这个方法的行为与 `unwrap` 类似：他返回 `Ok` 封装的内部值。但是，当值为 `Err` 时，这个方法会调用 *闭包，closure* 中的代码，所谓闭包，是我们定义的一个匿名函数，并作为参数传递给 `unwrap_or_else`。我们将在 [第 13 章](../Ch13_Functional_Language_Features_Iterators_and_Closures.md) 中更详细地介绍闭包。目前，咱们只需要知道 `unwrap_or_else` 将通过出现于垂直管道（译注：两条竖线）之间的参数 `err`，传递 `Err` 的内层值给闭包，`Err` 的内层值在这一情形下，即为我们在清单 12-9 中添加的静态字符串 `参数数量不足`。然后，闭包中的代码可以在其运行时使用 `err` 值。

我们添加了个新的 `use` 行，带入标准库中的 `process` 到作用域中。将在错误情形下运行的闭包中的代码只有两行：我们打印 `err` 值，然后调用 `process::exit`。`process::exit` 函数将立即停止程序，并返回作为退出状态代码传递的数字。这类似于我们在清单 12-8 中使用的基于 `panic!` 的处理，但我们不再得到所有额外的输出。我们来尝试一下：

```console
$ cargo run
   Compiling minigrep v0.1.0 (/home/hector/rust-lang-zh_CN/projects/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/minigrep`
解析参数时遇到问题：参数不足
```

棒极了！这一输出对我们的用户来说更加友好。


## 提取 `main` 中的逻辑

现在我们已经完成了配置解析的重构，我们来看看程序的逻辑。正如我们在 [二进制项目中的关注点分离](#二进制项目中的关注点分离) 小节中指出的，我们将提取出一个名为 `run` 的函数，将容纳 `main` 函数中当前不涉及建立配置或错误处理的所有逻辑。当我们完成后，`main` 将变得简洁并易于通过目测验证，并且我们将能够针对所有其他逻辑编写测试。

下面清单 12-11 展示了提取  `run` 函数这一微小、渐进的改进

<a name="listing_12-11"></a>
文件名：`src/main.rs`

```rust
fn main() {
    // --跳过代码--
    println! ("
        在文件 {} 中
        检索 {}", config.file_path, config.query);

    run(config);
}

fn run(config: Config) {
    let contents = fs::read_to_string(config.file_path)
        .expect("应该已经能够读取文件");

    println! ("带有文本：\n{contents}");
}

// --跳过代码--
```

**清单 12-11**：提取包含其余程序逻辑的 `run` 函数

`run` 函数现在包含 `main` 中从读取文件开始的所有剩余逻辑。`run` 函数取 `Config` 实例作为参数。


### 返回 `run` 中的错误

随着剩余的程序逻辑被分离到 `run` 函数中，我们可以改进错误处理，就像在 [清单 12-9](#listing_12-9) 中对 `Config::build` 所做的那样。当出现错误时，`run` 函数将返回 `Result<T, E>`，而不是通过调用 `expect` 让程序终止运行。这将使我们能够以用户友好的方式，进一步把错误处理相关的逻辑整合到 `main` 中。下面清单 12-12 展示了我们需要对 `run` 的签名及函数体进行的修改。

<a name="listing_12-12"></a>
文件名：`src/main.rs`

```rust
// --跳过代码--
use std::{env, fs, process, error:Error};

// --跳过代码--

fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;

    println! ("有着文本：\n{contents}");

    Ok(())
}
```

**清单 12-12**：修改 `run` 函数为返回 `Result`

我们在这里做出了三处显著修改。首先，我们修改 `run` 函数的返回类型为 `Result<(), Box<dyn Error>>`。这个函数之前返回单元值类型，`()`，我们保留其为 `Ok` 情形下返回的值。

对于错误类型，我们使用了特质对象 `Box<dyn Error>` （并且我们以顶部的 `use` 语句带入了 `std::error::Error` 到作用域）。我们将在 [第 18 章](../Ch17_Object_Oriented_Programming_Features_of_Rust.md) 中介绍特质对象。目前，只需知道 `Box<(), Error>` 表示该函数将返回一个实现 `Error` 特质的类型，但我们不必指定返回值将是何特定类型。这赋予了我们在不同的错误情形下，返回可能为不同类型的错误值的灵活性。`dyn` 关键字是 *dynamic* 的缩写。

其次，正如我们在 [第 9 章](../error_handling/result.md#-操作符快捷方式) 中讨论的那样，我们移除了对 `expect` 的调用，转而使用 `?` 运算符。与错误上的 `panic!` 不同，`?` 将返回当前函数中的错误值供调用者处理。

第三，`run` 函数现在会在成功情形下返回 `Ok` 值。我们在签名中已声明 `run` 函数的成功类型为 `()`，这意味着我们需要将单元值封装在 `Ok` 值中。这种 `Ok(())` 语法乍一看可能有点奇怪。但像这样使用 `()` 属于惯用的方式，表明我们只是为了他的副作用而调用 `run`； 他不会返回我们需要的值。

当咱们运行这段代码时，他将编译但会显示一条告警：

```console
$ cargo run the poem.txt
warning: unused `Result` that must be used
  --> src/main.rs:15:5
   |
15 |     run(config);
   |     ^^^^^^^^^^^
   |
   = note: this `Result` may be an `Err` variant, which should be handled
   = note: `#[warn(unused_must_use)]` (part of `#[warn(unused)]`) on by default
help: use `let _ = ...` to ignore the resulting value
   |
15 |     let _ = run(config);
   |     +++++++

warning: `minigrep` (bin "minigrep") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep the poem.txt`

        在文件 poem.txt 中
        检索 the
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

Rust 告诉我们，我们的代码忽略了 `Result` 值，而 `Result` 值可能表明发生了错误。但我们并未检查是否存在错误，编译器提醒我们，我们或许是要在这里放一些错误处理代码！我们来纠正这个问题。


### 在 `main` 中处理自 `run` 返回的错误

我们将检查错误，并使用我们在 [清单 12-10](#listing_12-10) 曾对 `Config::build` 使用的类似技巧来处理他们，但略有不同：


文件名：`src/main.rs`

```rust
fn main() {
    // --跳过代码--

    println! ("
        在文件 {} 中
        检索 {}", config.file_path, config.query);

    if let Err(e) = run(config) {
        println! ("应用程序错误：{e}");
        process::exit(1);
    }
}
```

我们使用 `if let` 而非 `unwrap_or_else` 检查 `run` 是否返回 `Err` 值，并返回 `Err` 值时调用 `process::exit(1)`。`run` 函数没有以与`Config::build` 返回 `Config` 实例的同样方式，返回一个我们打算要 `unwrap` 的值。因为 `run` 在成功情形下返回 `()`，而我们只关心探测错误，因此我们无需 `unwrap_or_else` 来返回解封装值，该值只会是 `()`。

在两种情形下，`if let` 与 `unwrap_or_else` 两个函数的函数体是相同的：我们都会打印错误并退出。


## 拆分代码到库代码箱

到目前为止，我们的 `minigrep` 项目看起来不错！现在我们将拆分 `src/main.rs` 文件并放置一些代码到 `src/lib.rs` 文件中。这样一来，我们就可以测试代码，并有一个职责更少的 `src/main.rs`。

我们来在 `src/lib.rs` 而不是 `src/main.rs` 中定义负责检索文本的代码，这将让我们（或任何使用我们的 `minigrep` 库的人）可以从相比我们的 `minigrep` 二进制项目的更多上下文中调用检索功能。

首先，我们来在 `src/lib.rs` 中定义 `search` 函数签名，如下清单 12-13 中所示，以一个调用 `unimplemented!` 宏的函数体。我们将在填入实现时更详细地解释这个签名。

<a name="listing_12-13"></a>
文件名：`src/lib.rs`

```rust
pub fn search<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    unimplemented! ();
}
```

**清单 12-13**：在 `src/lib.rs` 中定义 `search` 函数

我们在函数定义上使用了 `pub` 关键字，指定 `search` 为我们库代码箱公开 API 的一部分。我们现在有了个即可以在二进制代码箱中使用，我们又可测试的库代码箱！

现在我们需要带入定义在 `src/lib.rs` 中的代码到 `src/main.rs` 中的二进制代码箱作用域并调用他，如下清单 12-14 中所示。


<a name="listing_12-14"></a>
文件名：`src/main.rs`

```rust
// -- 跳过代码 --
use minigrep::search;

fn main() {
// -- 跳过代码 --
}

// -- 跳过代码 --

fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println! ("{line}");
    }

    Ok(())
}
```

**清单 12-14**：在 `src/main.rs` 中使用 `minigrep` 库代码箱的 `search` 函数

我们添加了一个 `use minigrep::search;` 行，带入库代码箱中的 `search` 函数到二进制代码箱的作用域。然后，在 `run` 函数中，我们不再打印出文件的内容，而是调用 `search` 函数并作为参数传递 `config.query` 和 `contents`。然后，`run` 将使用 `for` 循环打印从 `search` 返回的与查询匹配的每个行。这也是移除 `main` 中显示查询字符串和文件路径的 `println!` 调用的好时机，以便我们的程序仅打印检索结果（当无错误发生时）。

请注意，检索函数将在任何打印发生之前，收集所有结果到一个其返回的矢量值中。这种实现在检索大型文件时可能会显示结果较慢，因为结果不会在找到时打印；我们将在第 13 章中讨论一种使用迭代器解决这个问题的可能方式。

呼！虽然工作量不小，但我们已经为今后的成功打下了坚实的基础。现在处理错误变得更加容易，而且我们使代码也更加模组化。从现在开始，我们几乎所有的工作都将在 `src/lib.rs` 中完成。

让我们利用这种新获得的模组化特性，做一些在旧代码下很难、但在新代码中却很简单的事情：我们将编写一些测试！
