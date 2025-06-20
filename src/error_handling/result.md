# `Result` 下的可恢复错误

**Recoverable Errors with `Result`**


大多数错误都不会严重到要求程序完全停止。有时，当某个函数失败时，其会是咱们容易解释并做出响应的某个原因。例如，当咱们试图打开某个文件，但因为该文件不存在而操作失败，咱们可能就会想要创建这个文件，而不是终止进程。


回顾第二章中 [使用 `Result` 处理潜在失效](../Ch02_Programming_a_Guessing_Game.md#使用-result-处理潜在失效) 小节，其中的 `Result` 枚举被定义为有两个变种，`Ok` 与 `Err`，如下所示：

```rust
enum Result<T, E> {
    Ok<T>,
    Err<E>,
}
```

其中的 `T` 和 `E` 均为泛型参数，generic type parameters：我们将在第 10 章详细讨论泛型。咱们现在需要知道的是，`T` 表示 `Ok` 这个变种中，成功情况下将返回的值类型，`E` 表示 `Err` 变种中，失败情况下将返回的错误类型。由于 `Result` 有着这两个泛型参数，我们就可以在我们打算返回的成功值和错误值可能会有所不同的许多不同情况下，使用这个 `Result` 类型及其上定义的函数。

我们来调用一个会返回 `Result` 值的函数，因为该函数可能会失败。在下面清单 9-3 中，我们尝试打开某个文件。

文件名：`src/main.rs`

```rust
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");
}
```

*清单 9-3：打开某个文件*

`File::open` 的返回类型为 `Result<T，E>`。其中泛型参数 `T` 已由 `File::open` 的实现，以文件句柄的成功值类型 `std::fs::File` 填入。错误值中用到的类型 `E` 为 `std::io::Error`。这种返回类型意味着，到 `File::open` 的调用可能会成功，而返回一个我们可以读取或写入的文件句柄。该函数调用也可能失败：例如，该文件可能不存在，或者我们可能没有访问文件的权限。`File::open` 函数需要有一种告诉我们他成功了或失败了的方式，同时给到我们文件句柄或错误信息。这些信息正是 `Result` 这个枚举所要传达的。


在 `File::open` 成功的情况下，变量 `greeting_file_result` 中的值将是个包含着文件句柄的 `Ok` 实例。而在其失败的情况下，`greeting_file_result` 变量中的值将是个其中包含了所发生错误类型更多信息的 `Err` 实例。


我们需要在清单 9-3 中添加代码，根据 `File::open` 返回的值采取不同的操作。下面清单 9-4 展示了使用一项基本工具，即我们在第 6 章中讨论过的 `match` 表达式的一种方法。

文件名：`src/main.rs`

```rust
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(e) => panic! ("打开文件出现问题：{:?}", e),
    };
}
```

*清单 9-4：使用 `match` 表达式处理可能返回的 `Result` 变种*


请注意，与 `Option` 枚举一样，`Result` 枚举及其变种，也已被 Rust 的前奏，the prelude，纳入到了作用域，因此我们无需在 `match` 支臂中的 `Ok` 和 `Err` 变种前指明 `Result::`。

在返回结果为 `OK` 时，该代码将返回 `OK` 变种内部的 `file` 值，然后我们将该文件句柄值，赋值给变量 `greeting_file`。在这个 `match` 表达式后，我们就可以将该文件句柄用于读写操作。

`match` 表达式的另一支臂，处理了咱们从 `File::open` 得到一个 `Err` 值的情况。在本例中，我们选择调用 `panic!` 宏。若当前目录下没有名为 `hello.txt` 的文件，我们就会运行这段代码，我们将看到 `panic!` 宏的以下输出：


```console
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/result_demo`

thread 'main' panicked at src/main.rs:8:19:
打开文件出现问题：Os { code: 2, kind: NotFound, message: "No such file or directory" }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

像往常一样，此输出会准确地告诉我们出了什么问题。


## 不同错误的匹配

**Matching on Different Errors**


无论 `File::open` 为何失败，上面清单 9-4 中的代码都会死机。但是，我们希望因应不同失败原因，采取不同操作。在 `File::open` 因文件不存在而失败时，我们打算创建处该文件，并返回这个新文件的句柄。在 `File::open` 因其他原因失败 -- 比如，因为我们没有打开文件的权限时 -- 我们仍想要代码以与清单 9-4 相同的方式 `panic!`。为此，我们添加了个内层的 `match` 表达式，如下清单 9-5 所示。


文件名：`src/main.rs`

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(error) => panic! ("创建该文件时出现问题：{:?}", error),
            },
            other_error => panic! ("打开文件出现问题：{:?}", other_error),
        },
    };
}
```

*清单 9-5：以不同方式处理不同类别错误*


`File::open` 在 `Err` 变种中返回的值类型为 `io::Error`，这是由标准库提供的一个结构体。这个结构体有个方法 `kind`，我们可以调用他获取到一个 `io::ErrorKind` 值。枚举 `io::ErrorKind` 由标准库提供，其变种表示了 `io` 操作可能得到的不同类别错误。我们打算使用的变种是 `ErrorKind::NotFound`，表示我们试图打开的文件还不存在。因此，我们对 `greeting_file_result` 进行匹配，但同时也对 `error.kind()` 有个内层的匹配。

我们在内层的匹配中打算检查的条件，是由 `error.kind()` 返回的值，是否为 `ErrorKind` 枚举的 `NotFound` 变种。在是时，我们就尝试以 `File::create` 创建出该文件。不过，由于 `File::create` 也可能失败，因此我们需要在这个内层的 `match` 表达式中，添加第二支臂。当该文件无法创建时，就会打印不同错误消息。外层的 `match` 表达式的第二支臂保持不变，因此除了文件找不到错误外，这个程序会在出现任何错误时都会死机。


> **`Result<T, E>` 下使用 `match` 表达式的替代方案**
>
> 这里的 `match` 表达式太多了！`match` 表达式非常有用，但也非常原始。在第 13 章中，咱们将了解与许多定义在 `Result<T, E>` 上方法一起使用的闭包。在与咱们的代码中处理 `Result<T, E>` 值时，这些方法比使用 `match` 更为简洁。
>
> 例如，下面是编写与清单 9-5 中所示同一逻辑的另一种方法，这次使用了闭包及 `unwrap_or_else` 这个方法：

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|e| {
        if e.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic! ("创建文件时发生问题：{:?}", error);
            })
        } else {
            panic! ("打开文件时出现问题：{:?}", e);
        }
    });

    println! ("{:?}", greeting_file);
}
```

> 虽然这段代码的行为与清单 9-5 相同，但他未包含任何的 `match` 表达式，而读起来更简洁。在咱们读完第 13 章后，请再来看这个示例，并在标准库文档中找到 `unwrap_or_else` 这个方法。在咱们处理错误时，还有更多的这些方法，可以清理庞大的 `match` 匹配表达式。


### 出错时死机的快捷方式：`unwrap` 与 `expect`

**Shortcuts for Panic on Error: `unwrap` and `expect`**


使用 `match` 可以很好地工作，但这样做可能有点啰嗦，而且并不总是很好地传达意图。`Result<T, E>` 这个类型，定义了许多用于完成各种更具体任务的辅助方法。`unwrap` 方法是个实现了与我们在清单 9-4 中，所编写的 `match` 表达式一样的快捷方法。在结果值为 `Ok` 变种时，`unwrap` 将返回 `Ok` 内的值。在结果值为 `Err` 变种时，`unwrap` 将为我们调用 `panic!` 宏。下面是个 `unwrap` 的示例：


文件名：`src/main.rs`

```rust
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap();
}
```

若我们在没有 `hello.txt` 文件的情况下运行这段代码，我们将看到 `unwrap` 方法调用 `panic!` 的错误消息：


```console
thread 'main' panicked at src/main.rs:5:49:
called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

类似地，`expect` 方法还允许我们选择 `panic!` 的错误消息。使用 `expect` 而不是 `unwrap`，并提供良好的错误消息，可以传达咱们的意图，并使追踪死机的源头变得更为容易。`expect` 的语法如下：


文件名：`src/main.rs`

```rust
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt 应包含在此项目中");
}
```

我们以与 `unwrap` 相同方式使用 `expect`：返回该文件的句柄或调用 `panic!` 宏。`expect` 在其对 `panic!` 的调用中，所使用的错误消息将是我们传递给 `expect` 的参数，而不是 `unwrap` 使用的默认 `panic!` 消息。其看起来像下面这样：


```console
thread 'main' panicked at src/main.rs:5:10:
hello.txt 应包含在此项目中: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

在生产质量的代码中，大多数 Rust 公民都会选择 `expect` 而不是 `unwrap`，并提供更多有关为什么该操作会总是成功的上下文消息。这样，在咱们的假设即使被证明是错误时，咱们也有更多用于调试的信息。



## 传播错误

**Propagating Errors**


在某个函数的实现，调用了可能失败的某些代码时，与其在该函数本身中处理错误，咱们可将错误返回给调用代码，让他决定要做些什么。这就是所谓的 *传播，propagating* 错误，而将更多控制权交给了调用代码，相比咱们代码的上下文，调用代码中可能有更多决定如何处理错误的信息或逻辑。

例如，下面清单 9-6 显示了一个从文件中读取用户名的函数。在该文件不存在或无法读取时，这个函数将把这些错误，返回给调用该函数的代码。


```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut  username_file = match username_file_result {
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

*清单 9-6：使用 `match` 表达式将错误返回给调用代码的一个函数*


这个函数可以更简短方式编写，但为探索错误处理，我们将从手动方式完成他开始；在最后，我们将展示那种更简短的方式。我们先看看这个函数的返回类型：`Result<String, io::Error>`。这意味着该函数返回的是个 `Result<T, E>` 类型的值，其中泛型参数 `T` 已填充为具体类型 `String`，且泛型 `E` 已填充为具体类型 `io::Error`。

在这个函数在没有任何问题下成功执行时，调用该函数的代码将收到保存着一个字符串的 `Ok` 值 -- 及该函数从文件中读取到的 `username`。在该函数遇到任何问题时，调用代码将收到保存着一个 `io::Error` 实例的 `Err` 值，该实例包含了问题为何的更多信息。我们选择 `io::Error` 作为此函数的返回类型，是因为在此函数的主体中，我们调用的两个操作：`File::open` 函数与 `read_too_string` 方法，都可能失败，而这两个操作返回的错误值类型，恰好都是 `io::Error`。


该函数的主体以调用 `File::open` 函数开始。然后，我们以一个与清单 9-4 中类似的 `match` 处理 `Result` 值。在 `File::open` 成功时，模式变量 `file` 中的文件句柄，就成为可变变量 `username_file` 中的值，同时函数继续执行。在 `Err` 情形下，我们没有调用 `panic!`，而是使用 `return` 关键字提前从该函数整个返回，并将 `File::open` 中的错误值，即现在模式变量 `e` 中的值，作为该函数的错误值传回调用代码。


因此，若我们在 `username_file` 中有了个文件句柄，该函数就会在变量 `username` 中创建出一个新的 `String`，然后调用 `username_file` 中文件句柄上的 `read_to_string` 方法，将该文件的内容读入 `username`。`read_to_string` 方法也会返回一个 `Result`，因为即使 `File::open` 成功了，他也可能失败。因此，我们需要另一个 `match` 表达式处理这个 `Result`：在 `read_to_string` 成功时，那么我们的函数就成功了，同时我们会返回该文件中的用户名，现在是封装在一个 `Ok` 里的 `username` 中。在 `read_to_string` 失败时，我们会以我们处理 `File::open` 返回值的 `match` 表达式中同一方式，返回这个错误值。不过，我们无需说明 `return`，因为这是该函数中的最后一个表达式。


调用这段代码的代码，将处理获取到包含用户名的一个 `Ok` 值，或处理包含 `io::Error` 的一个 `Err` 值。调用代码将自行决定对这些值执行什么操作。在调用代码得到一个 `Err` 值时，他可以调用 `panic!` 使程序崩溃、使用一个默认用户名，或者从文件以外的其他地方查找用户名。我们没有有关调用代码到底要做什么的足够，因此我们会将所有的成功或错误信息向上传播，以便其进行适当处理。

这种传播错误的模式在 Rust 中非常常见，因此 Rust 提供了问号操作符 `?`，the question mark operator，使其更容易。


### 传播错误的捷径：`?` 操作符

**A Shortcut for Propagating Errors: the `?` Operator**

下面清单 9-7 显示了 `read_username_from_file` 的一种实现，其功能与清单 9-6 中的相同，但这一实现使用了 `?` 操作符。

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

*清单 9-7：使用 `?` 操作符将错误返回给调用代码的一个函数*

放在某个 `Result` 值后面的 `?`，被定义与清单 9-6 中，我们为处理 `Result` 值而定义的 `match` 表达式工作方式几乎相同。在 `Result` 的值是个 `Ok`，则这个 `Ok` 中的值将从该表达式返回，同时程序将继续运行。在 `Result` 的值是个 `Err` 时，那么这个 `Err` 将从整个函数中返回，就像我们曾使用的那个 `return` 关键字一样，如此错误值就得以传播到调用代码。


清单 9-6 中 `match` 表达式的作用，与 `?` 运算符的作用有所不同：调用 `?` 运算符的错误值，会经过标准库中 `From` 特质中定义的 `from` 函数，该函数被用来将值从一种类型，转换为另一类型。当 `?` 运算符调用 `from` 函数时，收到的错误类型会被转换为当前函数返回类型中，所定义的错误类型。在某个函数返回表示函数可能失败所有方式的一种类型，即使各个部分可能因多种不同原因失败时，这种方法非常有用。

例如，我们可以将清单 9-7 中的 `read_username_from_file` 函数，修改为返回我们定义的名为 `OurError` 的自定义错误类型。在我们也为 `OurError` 定义了 `impl From<io::Error>`，以从 `io::Error` 构造出一个 `OurError` 实例时，那么 `read_username_from_file` 主体中的 `?` 操作符调用，就将调用 `from`，并无需在函数中添加更多代码下，转换这些错误类型。


在清单 9-7 的上下文中，`File::open` 调用结尾处的 `?` 操作符，将把一个 `Ok` 内的值，返回给变量 `username_file`。在发生错误时，`?` 操作符将从整个函数提前返回，并将 `Err` 值返回给调用代码。同样的情况也适用于 `read_to_string` 调用结束处的那个 `?` 操作符。

`?` 操作符消除了大量模板代码，而使这个函数的实现更简单。如下清单 9-8 所示，通过在 `?` 操作符后立即链接这些方法调用，我们甚至能进一步缩短这段代码。


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

*清单 9-8：在 `?` 操作符后链接方法调用*


我们已将 `username` 中那个新 `String` 的创建，移至该函数的开头；这部分未曾改变。与创建变量 `username_file` 相反，我们已将到 `read_to_string` 的调用，直接链接到 `File::open("hello.txt")?` 的结果上。在 `read_to_string` 调用末尾，我们仍然有个 `?`，而在 `File::open` 及 `read_to_string` 两个调用都成功时，我们仍会返回一个包含着 `username` 的 `Ok` 值，而不是返回错误。清单 9-6 和清单 9-7 中的功能又一次相同；这只是编写他的一种不同的、更符合人体工程学的方式。

下面清单 9-9 展示了一种使用 `fs::read_to_string`，使其更简短的方法。

文件名：`src/main.rs`

```rust
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```

*清单 9-9：使用 `fs::read_to_string` 而非打开然后读取文件*


将某个文件读入字符串，是种相当常见的操作，因此标准库提供了可以打开该文件、创建出一个新 `String`、读取该文件内容、将内容放入这个 `String`，并将其返回的便利的 `fs::read_too_string` 函数。当然，使用 `fs::read_too_string` 函数并并未给到我们解释所有这些错误处理的机会，所以我们先以更长的方式实现。



### `?` 操作符可用于哪些地方

**Where The `?` Operator Can Be Used**


`?` 操作符只能用在那些返回类型与 `?` 所用在值兼容的那些函数中。这是因为 `?` 操作符被定义为以与我们在清单 9-6 中定义的 `match` 表达式相同方式，执行一次从函数提前返回某个值。在清单 9-6 中，`match` 表达式使用了个 `Result` 值，并提前返回那个返回了个 `Err(e)` 值的支臂。该函数的返回类型，必须是个 `Result`，这样才能与这个 `return` 语句兼容。

在下面的清单 9-10 中，我们来看看若咱们在某个 `main` 函数中，与某个跟咱们在其上使用 `?` 的值不兼容返回类型，一起使用 `?` 运算符时，会得到一个什么样的报错。

文件名：`src/mian.rs`

```rust
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt");
}
```

*清单 9-10：尝试在返回 `()` 的 `main` 函数中使用 `?` 将不编译*


这段代码打开某个文件，但这可能会失败。运算符 `?` 跟在由 `File::open` 返回的一个 `Result` 值后，但这个 `main` 函数有着返回类型 `()`，而非 `Result`。当我们编译这段代码时，我们会得到以下错误消息：

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

- 一种选择是将咱们函数的返回类型，修改为与咱们在其上使用 `?` 运算符的值兼容，只要咱们没有阻止这样做的限制；
- 另一选择是使用 `match` 表达式，或 `Result<T, E>` 的方法之一，以任何适当的方式处理这个 `Result<T,E>`。


报错消息还提到，`?` 也可与 `Option<T>` 的值一起使用。与对 `Result` 使用 `?` 运算符一样，咱们只能在返回 `Option` 的函数中，于 `Option` 上使用 `?` 运算符。在某个 `Option<T>` 上调用 `?` 操作符的行为，与在某个 `Result<T, E>` 上调用 `?` 操作符的行为类似：在值为 `None` 时，这个 `None` 就将在此时从该函数提前返回。在值为 `Some`，`Some` 中的值就是该表达式的结果值，同时该函数会继续执行。下面清单 9-11 有着一个查找给定文本中第一行最后一个字符的函数示例。


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


