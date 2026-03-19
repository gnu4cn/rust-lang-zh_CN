# `Result` 下的可恢复错误

大多数错误都不会严重到需要程序完全停止。有时，当某个函数失败时，是出于某种咱们容易解释并做出响应的原因。例如，当咱们试图打开某个文件，而该操作因该文件不存在而失败时，咱们可能想要创建这个文件而不是终止进程。

回顾第二章中的 [以 `Result` 处理潜在失效](../Ch02_Programming_a_Guessing_Game.md#以-result-处理潜在失效) 小节，`Result` 枚举被定义为有两个变种，`Ok` 与 `Err`，如下所示：

```rust
enum Result<T, E> {
    Ok<T>,
    Err<E>,
}
```

其中 `T` 和 `E` 均为泛型参数，generic type parameter：我们将在第 10 章中更详细地讨论泛型。咱们现在需要知道的是，`T` 表示将在成功情况下于 `Ok` 变种内返回的值类型，`E` 表示将在失败情况下于 `Err` 变种中返回的错误类型。由于 `Result` 有这两个泛型参数，因此我们可在许多不同情形下使用 `Result` 类型及定义在其上的函数，其中我们希望返回的成功值和错误值可能不同。

我们来调用一个返回 `Result` 值的函数，因为该函数可能会失败。在下面清单 9-3 中，我们尝试打开某个文件。

<a name="listing_9-3"></a>
文件名：`src/main.rs`

```rust
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");
}
```

**清单 9-3**：打开文件

`File::open` 的返回类型是个 `Result<T，E>`。其中泛型参数 `T` 已由 `File::open` 的实现以成功值的类型 `std::fs::File` 填入，其为文件句柄。用于错误值中的类型 `E` 为 `std::io::Error`。这一返回类型意味着对 `File::open` 的调用可能成功并返回一个我们可以读取或写入的文件句柄。这个函数调用也可能失败：例如，文件可能不存在，或者我们可能没有访问文件的权限。`File::open` 函数需要有一种方式来告诉我们他是成功还是失败，并同时给到我们文件句柄或错误信息。这些信息正是 `Result` 枚举传达的内容。

在 `File::open` 成功的情况下，变量 `greeting_file_result` 中的值将是个包含着文件句柄的 `Ok` 的实例。而在其失败的情况下，`greeting_file_result` 变量中的值将是个 `Err` 的实例，包含有关发生的错误的类别的更多信息。

我们需要添加到清单 9-3 中的代码，以根据 `File::open` 返回的值采取不同的操作。下面清单 9-4 展示了使用一项基本工具来处理 `Result`的一种方式，即我们在第 6 章中讨论的 [`match` 表达式](../enums_and_pattern_matching/match_control_flow.md)。

<a name="listing_9-4"></a>
文件名：`src/main.rs`

```rust
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic! ("打开文件出现问题：{error:?}"),
    };
}
```

**清单 9-4**：使用 `match` 表达式处理可能返回的 `Result` 变种

请注意，与 `Option` 枚举一样，`Result` 枚举及其变种均已由前奏，the prelude，带入到作用域中，因此我们无需在 `match` 支臂中的 `Ok` 和 `Err` 变种前指定 `Result::`。

当结果为 `OK` 时，该代码将从 `OK` 变种返回内层的 `file` 值，然后我们指派该文件句柄值给变量 `greeting_file`。在 `match` 后，我们便可使用该文件句柄进行读取或写入。

`match` 表达式的另一支臂处理咱们从 `File::open` 得到 `Err` 值的情况。在这个示例中，我们选择了调用 `panic!` 宏。若我们的当前目录下没有名为 `hello.txt` 的文件并且我们运行这段代码时，我们将看到来自 `panic!` 宏的以下输出：


```console
$ cargo run
   Compiling result_demo v0.1.0 (/home/hector/rust-lang-zh_CN/projects/result_demo)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/result_demo`

thread 'main' (523922) panicked at src/main.rs:8:23:
打开文件出现问题：Os { code: 2, kind: NotFound, message: "No such file or directory" }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

像往常一样，这一输出告诉我们究竟出了什么问题。


## 匹配不同的错误

无论 `File::open` 因何种原因失败，清单 9-4 中的代码都将 `panic!`。然而，我们希望针对不同的失败原因采取不同的操作。当 `File::open` 因文件不存在失败时，我们打算创建处该文件并返回到新文件的句柄。当 `File::open` 因任何其他原因失败 -- 比如，因为我们没有打开该文件的权限时 -- 我们仍希望代码 `panic!`，以其在清单 9-4 所做的同一方式。为此，我们添加一个内层的 `match` 表达式，如下清单 9-5 中所示。


<a name="listing_9-5"></a>
文件名：`src/main.rs`

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic! ("创建该文件时出现问题：{e:?}"),
            },
            _ => {
                panic! ("打开文件出现问题：{error:?}");
            }
        },
    };
}
```

**清单 9-5**：以不同方式处理不同类别的错误


`File::open` 在 `Err` 变种内返回的值类型为 `io::Error`，这是由标准库提供的一个结构体（译注：类型为 `Os`）。这个结构体有个方法 `kind`，我们可调用他来得到一个 `io::ErrorKind` 值。枚举 `io::ErrorKind` 由标准库提供，并有着表示一次 `io` 操作可能得到的不同错误类别的变种。我们打算使用的变种是 `ErrorKind::NotFound`，表示我们尝试打开的文件还不存在。因此，我们对 `greeting_file_result` 进行匹配，但我们还有个对 `error.kind()` 的内层匹配。

我们在内层匹配中希望检查的条件，是由 `error.kind()` 返回的值是否是 `ErrorKind` 枚举的 `NotFound` 变种。当是时，我们尝试以 `File::create` 创建该文件。不过，由于 `File::create` 也会失败，因此我们需要一个内层 `match` 表达式中的第二支臂。当文件无法创建时，一条不同的错误消息得以打印。外层 `match` 表达式的第二支臂保持不变，因此该程序会因除文件找不到的错误外的任何错误而终止运行。


> **对 `Result<T, E>` 使用 `match` 的替代方案**
>
> 这可真是不少的 `match`！`match` 表达式非常有用但也非常原始。在第 13 章中，咱们将学习闭包，其会与定义在 `Result<T, E>` 上的许多方法一起使用。在处理咱们代码中的 `Result<T, E>` 值时，这些方法比使用 `match` 更为简洁。
>
> 例如，下面是编写与清单 9-5 中所示同一逻辑的另一种方式，这次使用闭包及 `unwrap_or_else` 方法：
>
> ```rust
> use std::fs::File;
> use std::io::ErrorKind;
>
> fn main() {
>     let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
>         if e.kind() == ErrorKind::NotFound {
>             File::create("hello.txt").unwrap_or_else(|error| {
>                 panic! ("创建文件时发生问题：{error:?}");
>             })
>         } else {
>             panic! ("打开文件时出现问题：{error:?}");
>         }
>     });
> }
> ```
>
虽然这段代码有着与清单 9-5 相同的行为，但他未包含任何 `match` 表达式，进而读起来更清晰。请在咱们读完第 13 章后回到这个示例，并在标准库文档中查找 `unwrap_or_else` 方法。在咱们处理错误时，还有更多的这些方法可以清理庞大、嵌套的 `match` 表达式。


### 出错时终止运行的快捷方式

使用 `match` 效果很好，但他可能可能有点冗长，并且并不总是很好地传达意图。`Result<T, E>` 类型有许多定义在其上的辅助方法，以执行各种更具体的任务。`unwrap` 方法是个快捷方法，被实现为就像我们在清单 9-4 中编写的 `match` 表达式。当 `Result` 值为 `Ok` 变种时，`unwrap` 将返回 `Ok` 内的值。当 `Result` 为 `Err` 变种时，`unwrap` 将为我们调用 `panic!` 宏。下面是个 `unwrap` 的实际示例：


文件名：`src/main.rs`

```rust
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap();
}
```

若我们在没有 `hello.txt` 文件下运行这段代码，我们将看到一条来自 `unwrap` 方法所发起的 `panic!` 调用的错误消息：


```console
thread 'main' (536746) panicked at src/main.rs:4:49:
called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

同样，`expect` 方法还允许我们选择 `panic!` 错误消息。使用 `expect` 而不是 `unwrap` 并提供良好的错误消息，可以传达我们的意图并使追溯终止运行根源变得更为容易。`expect` 的语法如下：


文件名：`src/main.rs`

```rust
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt 应包含在此项目中");
}
```

我们以与 `unwrap` 相同方式使用 `expect`：返回文件句柄或调用 `panic!` 宏。`expect` 在对 `panic!` 的调用中使用的错误消息，将是我们传递给 `expect` 的参数，而不是 `unwrap` 使用的默认 `panic!` 消息。其看起来像下面这样：


```console
thread 'main' (539093) panicked at src/main.rs:5:10:
hello.txt 应包含在此项目中: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

在生产质量代码中，大多数 Rustaceans 都会选择 `expect` 而不是 `unwrap`，并会提供更多有关为何操作被认为总是会成功的背景信息。这样，当咱们的假设即使被证明是错的时，咱们也会有更多在调试过程中使用的信息。


## 传播错误

在函数的实现调用了可能失败的某些代码时，与其在该函数本身内处理错误，咱们可返回错误给调用代码，从而其可以决定要做些什么。这称为 *传播，propagating* 错误并赋予更多控制权给调用代码，相比咱们在咱们的代码上下文中有的可用信息或逻辑，调用代码中可能有更多决定错误应如何处理的信息或逻辑。

例如，下面清单 9-6 显示了一个读取文件中用户名的函数。当文件不存在或无法读取时，该函数将返回这些错误给调用该函数的代码。


<a name="listing_9-6"></a>
```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
```

**清单 9-6**：使用 `match` 表达式返回错误给调用代码的函数

这个函数可以更简短的方式编写，但为了探讨错误处理，我们将以手动方式完成其绝大部分开始；最后，我们将展示更简短的方式。我们首先来看看函数的返回类型：`Result<String, io::Error>`。这意味着该函数正返回一个 `Result<T, E>` 类型的值，其中泛型参数 `T` 已以具体类型 `String` 填入，且泛型 `E` 已以具体类型 `io::Error` 填入。

当这个函数在没有任何问题下成功时，调用该函数的代码将收到一个 `Ok` 值，其中保存着一个 `String` -- 该函数从文件中读取到的 `username`。当该函数遇到任何问题时，调用代码将收到一个 `Err` 值，该值保存着 `io::Error` 的一个实例，其包含有关问题为何的更多信息。我们选择 `io::Error` 作为这个函数的返回类型，因为他恰好同时是我们在这个函数的主体中，调用的两个可能失败的操作：`File::open` 函数与 `read_to_string` 方法返回的错误值类型。

该函数的主体以调用 `File::open` 函数开始。然后，我们以一个类似于清单 9-4 中的 `match` 处理 `Result` 值。当 `File::open` 成功时，模式变量 `file` 中的文件句柄成为可变变量 `username_file` 中的值而该函数会继续。在 `Err` 情形下，我们没有调用 `panic!`，而是使用 `return` 关键字完全从该函数提前返回，并传递 `File::open` 中的错误值，其现在位于模式变量 `e` 中，回给调用代码。

因此，当我们在 `username_file` 中有文件句柄时，该函数随后会在变量 `username` 中创建一个新的 `String`，并调用 `username_file` 中的文件句柄上的 `read_to_string` 方法，读取该文件的内容到 `username` 中。`read_to_string` 方法也会返回 `Result`，因为即使 `File::open` 成功了，他也可能失败。因此，我们需要另一个 `match` 表达式来处理这个 `Result`：当 `read_to_string` 成功时，那么我们的函数就成功了，进而我们返回文件中的用户名，其现在位于封装在 `Ok` 中的 `username` 中。当 `read_to_string` 失败时，我们以返回处理 `File::open` 返回值的 `match` 表达式中的错误值的同一方式返回这个错误值。不过，我们不需要显式指明 `return`，因为这是函数中的最后一个表达式。

调用这段代码的代码随后将处理获取到要么包含用户名的 `Ok` 值，要么包含 `io::Error` 的 `Err` 值（译注：两种情况）。对这两种值要做些什么由调用代码自行决定。当调用代码得到 `Err` 值时，他可以调用 `panic!` 并崩溃程序、使用默认用户名，或从该文件以外的其他地方查找用户名。由于我们没有调用代码到底要做什么的足够信息，因此我们向上传播所有成功或错误的信息，以供其恰当处理。

这种传播错误的模式在 Rust 中是如此的常见，以至 Rust 提供了问号操作符 `?`，the question mark operator，使其变得更容易。


### `?` 操作符快捷方式

下面清单 9-7 显示了 `read_username_from_file` 的一种实现，其有着与清单 9-6 中同样的功能，但这一实现使用 `?` 操作符。

<a name="listing_9-7"></a>
文件名：`src/main.rs`

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
```

**清单 9-7**：使用 `?` 操作符返回错误给调用代码的函数

放在 `Result` 值后面的 `?`，被定义为以几乎与 [清单 9-6](#listing_9-6) 中我们定义的，处理 `Result` 值的 `match` 表达式的同一方式工作。当 `Result` 的值为 `Ok` 时，`Ok` 中的值将从这一表达式返回，并且程序将继续。当 `Result` 的值为 `Err` 时，`Err` 将从整个函数中返回，就像我们使用了 `return` 关键字一样，以便错误值得以传播到调用代码。

清单 9-6 中的 `match` 表达式执行的操作与 `?` 操作符之间有个差异：于其上调用 `?` 操作符的错误值，会经历在标准库中的 `From` 特质中定义的 `from` 函数，该函数被用于转换一种类型的值为另一类型。当 `?` 操作符调用 `from` 函数时，接收到的错误类型会被转换为定义在当前函数返回类型中的错误类型。在即使各个部分可能因许多不同原因而失败，函数仍返回表示函数可能失败的所有方式的一种类型时，这一点非常有用。

例如，我们可以修改清单 9-7 中的 `read_username_from_file` 函数，为返回我们定义的一个名为 `OurError` 的自定义错误类型。当我们也为 `OurError` 定义了 `impl From<io::Error>`，以从一个 `io::Error` 构造一个 `OurError` 实例时，那么 `read_username_from_file` 主体中的 `?` 操作符调用将调用 `from`，并会在无需添加任何代码到该函数下转换错误类型。

在清单 9-7 的上下文中，`File::open` 调用末尾处的 `?` 操作符将返回 `Ok` 内的值给变量 `username_file`。当错误发生时，`?` 操作符将提前从整个函数返回并给予任何 `Err` 值给调用代码。同样的事情也适用于 `read_to_string` 调用末尾的 `?` 操作符。

`?` 操作符消除了大量模板代码，而使这个函数的实现更简单。我们甚至可以通过在 `?` 后立即链接方法调用，进一步缩短这段代码，如下清单 9-8 中所示。


<a name="listing_9-8"></a>
文件名：`src/main.rs`

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}
```

**清单 9-8**：在 `?` 操作符后链接方法调用


我们已移动 `username` 中新的 `String` 的创建至函数的开头；这部分未曾改变。与其创建变量 `username_file`，我们已直接链接到 `read_to_string` 的调用到 `File::open("hello.txt")?` 的结果上。在 `read_to_string` 调用末尾我们仍然有个 `?`，而在 `File::open` 与 `read_to_string` 两个调用都成功后，我们仍返回一个包含 `username` 的 `Ok` 值，而不是返回错误。功能再度与清单 9-6 和清单 9-7 中的相同；这只是编写他的一种不同的、更符合人体工程学的方式。

下面清单 9-9 展示了一种使用 `fs::read_to_string` 使其更简短的方式。

<a name="listing_9-9"></a>
文件名：`src/main.rs`

```rust
use std::{fs, io};

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```

**清单 9-9**：使用 `fs::read_to_string` 而不是打开然后读取文件

读取文件到字符串中属于一个相当常见的操作，因此标准库提供了便捷的 `fs::read_to_string` 函数，其会打开文件、创建一个新的 `String`、读取文件内容、放入该内容到 `String` 中，并将其返回。当然，使用 `fs::read_too_string` 函数不会给予我们解释所有错误处理的机会，所以我们先以更长的方式实现了他。


### 哪些地方要使用 `?` 操作符？

`?` 操作符只能用在那些返回类型与 `?` 所用在值兼容的那些函数中。这是因为 `?` 操作符被定义为以与我们在清单 9-6 中定义的 `match` 表达式相同方式，执行一次从函数提前返回某个值。在清单 9-6 中，`match` 表达式使用了个 `Result` 值，并提前返回那个返回了个 `Err(e)` 值的支臂。该函数的返回类型，必须是个 `Result`，这样才能与这个 `return` 语句兼容。

在下面的清单 9-10 中，我们来看看若咱们在某个 `main` 函数中，与某个跟咱们在其上使用 `?` 的值不兼容返回类型，一起使用 `?` 操作符时，会得到一个什么样的报错。

文件名：`src/mian.rs`

```rust
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt");
}
```

*清单 9-10：尝试在返回 `()` 的 `main` 函数中使用 `?` 将不编译*


这段代码打开某个文件，但这可能会失败。操作符 `?` 跟在由 `File::open` 返回的一个 `Result` 值后，但这个 `main` 函数有着返回类型 `()`，而非 `Result`。当我们编译这段代码时，我们会得到以下错误消息：

```console
$ cargo run
  Compiling result_demo v0.1.0 (/home/hector/rust-lang-zh_CN/projects/result_demo)
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
 --> src/main.rs:4:48
  |
3 | fn main () {
  | ---------- this function should return `Result` or `Option` to accept `?`
4 |     let greeting_file = File::open("hello.txt")?;
  |                                                ^ cannot use the `?` operator in a function that returns `()`
  |
help: consider adding return type
  |
3 ~ fn main () -> Result<(), Box<dyn std::error::Error>> {
4 |     let greeting_file = File::open("hello.txt")?;
5 +     Ok(())
  |

For more information about this error, try `rustc --explain E0277`.
error: could not compile `result_demo` (bin "result_demo") due to 1 previous error
```

这个错误指出，我们只被允许在返回 `Result`、`Option` 或其他实现了 `FromResidual` 的类型函数中，使用 `?` 操作符。

要修复这个错误，咱们有两个选择：

- 一种选择是将咱们函数的返回类型，修改为与咱们在其上使用 `?` 操作符的值兼容，只要咱们没有阻止这样做的限制；
- 另一选择是使用 `match` 表达式，或 `Result<T, E>` 的方法之一，以任何适当的方式处理这个 `Result<T,E>`。


报错消息还提到，`?` 也可与 `Option<T>` 的值一起使用。与对 `Result` 使用 `?` 操作符一样，咱们只能在返回 `Option` 的函数中，于 `Option` 上使用 `?` 操作符。在某个 `Option<T>` 上调用 `?` 操作符的行为，与在某个 `Result<T, E>` 上调用 `?` 操作符的行为类似：在值为 `None` 时，这个 `None` 就将在此时从该函数提前返回。在值为 `Some`，`Some` 中的值就是该表达式的结果值，同时该函数会继续执行。下面清单 9-11 有着一个查找给定文本中第一行最后一个字符的函数示例。


```rust
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
```

*清单 9-11：在某个 `Option<T>` 值上使用 `?` 操作符*


这个函数会返回 `Option<char>`，因为该处可能存在某个字符，但也可能不存在。这段代码取 `text` 这个字符串片段参数，并在其上调用返回该字符串中各行迭代器的 `lines`。由于这个函数要检查第一行，因此他会调用迭代器上的 `next`，获取迭代器中的首个值。在 `text` 是空字符串时，则这个到 `next` 的调用将返回 `None`，在这种情况下，我们使用 `?` 停止执行，并从 `last_char_of_first_line` 返回 `None`。在 `text` 不是空字符串时，`next` 将返回一个包含着 `text` 中第一行字符串切片的 `Some` 值。

`?` 会提取字符串切片，而我们在该字符串切片上调用 `chars`，获取其字符的一个迭代器。我们感兴趣的是第一行中的最后一个字符，因此我们调用了 `last`，返回该迭代器中的最后一项。这是个 `Option`，因为第一行可能是个空字符串；比如，在文本以空行开始，但其他行有字符，如 `"\nhi"` 时。但在第一行有个最后字符时，他将在 `Some` 变种中返回。中间的操作符 `?` 给了我们表达这一逻辑的简洁方式，允许我们以一行代码实现这个函数。若我们不能在 `Option` 上使用 `?` 操作符，我们就必须使用更多方法调用，或一个 `match` 表达式实现这一逻辑。

请注意，咱们可在返回 `Result` 的函数中对 `Result` 使用 `?` 操作符，也可在返回 `Option` 的函数中对 `Option` 使用 `?` 操作符，但咱们不能混用及匹配。`?` 操作符不会自动将一个 `Result` 转换为一个 `Option`，反之亦然；在这种情况下，咱们可使用 `Result` 的 `ok` 方法，或 `Option` 的 `ok_or` 方法，显式地执行该转换。

到目前为止，我们曾用到的所有 `main` 函数，都会返回 `()`。`main` 函数比较特殊，因为他 **是可执行程序的入口点及出口点**，为了使程序的运行符合预期，其返回类型是有限制的。

幸运的是，`main` 也可以返回 `Result<(), E>`。下面清单 9-12 有着清单 9-10 中的代码，但我们已将 `main` 的返回类型，改为了 `Result<(), Box<dyn Error>>`，并将一个返回值 `Ok(())` 添加到末尾。这段代码现在将会编译。


```rust
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
```

*清单 9-12：将 `main` 修改为返回 `Result<(), E>`，就允许在 `Result` 值上 `?` 操作符的使用*


`Box<dyn Error>` 类型是个 *特质对象，trait object*，我们将在第 18 章 [“使用允许不同类型值的特质对象”](../oop/trait_objects.md) 中讨论他。现在，咱们可将 `Box<dyn Error>` 理解为 “任何类型的错误”。在有着错误类型 `Box<dyn Error>` 的某个 `main` 函数中，对某个 `Result` 值使用 `?` 是允许的，因为他任何 `Err` 值都得以提前返回。尽管这个 `main` 函数的主体，将都只返回 `std::io::Error` 类型的错误，但通过指定 `Box<dyn Error>`，即使往 `main` 的主体中添加了返回其他错误的更多代码，这个函数签名仍将保持正确。

在某个 `main` 函数返回一个 `Result<(), E>` 时，那么若 `main` 返回了 `Ok(())`，那么该可执行文件将以值 `0` 退出，同时在 `main` 返回一个 `Err` 值时，那么该可执行文件将以非 0 值退出。以 C 编写的可执行文件，在他们退出时会返回整数：成功退出的程序会返回整数 `0`，出错的程序会返回 `0` 以外的某个整数。Rust 同样会从可执行文件返回整数，以符合这一约定。


`main` 函数可以返回任何实现了 [`std::process::Termination` 特质](https://doc.rust-lang.org/std/process/trait.Termination.html) 的类型，该特质包含返回 `ExitCode` 的函数 `report`。有关为咱们自己类型实现 `Termination` 特质的更多信息，请查阅标准库文档。

既然我们已经讨论了调用 `panic!` 或是返回 `Result` 的细节，那么我们再来讨论，如何决定在哪些情况下，使用哪一个合适。

（End）


