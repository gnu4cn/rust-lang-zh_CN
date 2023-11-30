# 使用 `Result` 的可恢复错误

多数错误都没有严重到要求程序整个地停止运行。某些时候，在某个函数失败时，必定是由于某种可易于解释进而加以响应的原因。比如在尝试打开某个文件，而因为要打开的文件不存在，那个操作失败了时，那么可能希望创建该文件，而不是中止这个进程。

回顾第二章中的 [处理潜在带有 `Result` 类型的程序失败](Ch02_Programming_a_Guessing_Game.md#处理潜在的带有-result-的程序失效) 小节，其中的 `Result` 枚举被定义为有两个变种，`Ok` 与 `Err`，如下所示：

```rust
enum Result<T, E> {
    Ok<T>,
    Err<E>,
}
```

这里的 `T` 与 `E`，都属于泛型参数（generic type parameters）：在第 10 章就会更深入讨论泛型。此刻需要明白的是，这里的 `T` 表示在操作成功情形下，那个 `Ok` 变种里返回值的类型，而这里的 `E`，则表示在失效情形下，将返回的在 `Err` 变种里错误的类型。由于 `Result` 有着这些泛型参数，因此就可以在打算返回成功值与错误值有所区别的许多不同情形下，使用到这个 `Result` 及定义在其上的函数。

下面就来调用一个由于其会失败，而返回 `Result` 值的函数。在下面清单 9-3 中，是尝试打开一个文件。

文件名：`src/main.rs`

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
}
```

*清单 9-3：打开某个文件*


怎样知道 `File::open` 会返回一个 `Result` 呢？这里就可以看看 [标准库 API 文档](https://doc.rust-lang.org/std/fs/struct.File.html#method.open)，或者可以询问一下编译器！在赋予 `f` 一个明知 *不是* 该函数返回值类型的类型注解，并随后尝试编译该代码时，编译器就会告知，这两个类型不匹配。给出的错误消息，就会告诉 `f` 的类型是什么。来试试吧！这里已知 `File::open` 的返回类型不是 `u32`，因此就把那个 `let f` 语句修改为下面这样：

```rust
let f: u32 = File::open("hello.txt");
```

现在尝试编译，就会给到接下来的输出：

```console
$ cargo run                                                                                       lennyp@vm-manjaro
   Compiling error_handling_demo v0.1.0 (/home/lennyp/rust-lang/error_handling_demo)
error[E0308]: mismatched types
 --> src/main.rs:4:18
  |
4 |     let f: u32 = File::open("hello.txt");
  |            ---   ^^^^^^^^^^^^^^^^^^^^^^^ expected `u32`, found enum `Result`
  |            |
  |            expected due to this
  |
  = note: expected type `u32`
             found enum `Result<File, std::io::Error>`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `error_handling_demo` due to previous error
```

这就是说，`File::open` 函数的返回类型，是个 `Result<T, E>`。泛型参数 `T`，在这里已被使用成功值的类型，`std::fs::File`，即一个文件句柄（a file handle）填充。而用于错误值的类型 `E`，则为 `std::io::Error`。

这样的返回值类型，表示到 `File::open` 的调用，可能会成功而返回一个能够自该处读取，或写入到该处的文件句柄。该函数调用同样可能失败：比如该文件可能不存在，或可能没有访问该文件的权限。那么这个 `File::open` 函数，就需要具备已知告知其是否成功或失败的方式，与此同时给到一个文件句柄，或者错误信息。这样的信息，正是这个 `Result` 枚举所要表达的。

此示例中，在 `File::open` 成功处，变量 `f` 中的值就会是包含了一个文件句柄的 一个 `Ok` 实例。而在其失败的情况下，`f` 中的那个值，就会是包含了有关所发生错误类别的更多信息的一个 `Err` 实例。

这里就需要对清单 9-3 中代码进行添加，从而根据 `File::open` 所返回值，而采取不同措施。下面清单 9-4 就给出了一种使用基本工具，即在第 6 章中曾讨论过的 `match` 表达式，对那个 `Result` 进行处理的方法。

文件名：`src/main.rs`

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(e) => panic! ("打开文件出现问题：{:?}", e),
    };
}
```

*清单 9-4：运用 `match` 表达式来处理可能返回的各个 `Result` 变种*

请注意，与 `Option` 枚举类似，这个 `Result` 枚举及其变种，是已由 Rust 前奏（the prelude）带入到作用域中了的，因此这里无需在那两个 `match` 支臂中的 `Ok` 与 `Err` 变种之前，指明 `Result::`。

在返回结果为 `Ok` 时，此代码就会返回从 `Ok` 变种抽出的那个内部的 `file` 值，且这里随后就把那个文件句柄值，指派给那个变量 `f`。在这个 `match` 之后，就可以将这个文件句柄，用于读取或写入了。

而那个 `match` 的另一支臂，则处理了从 `File::open` 得到一个 `Err` 值的情形。在此示例中，选择了调用 `panic!` 宏。在当前目录中没有名为 `hello.txt` 的文件，并运行此代码时，就会看到来自那个 `panic!` 宏的如下输出：

```console
$ cargo run                                                                                      lennyp@vm-manjaro
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/error_handling_demo`
thread 'main' panicked at '打开文件出现问题：Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:8:19
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

与往常一样，此输出告知了到底什么出错了。


## 匹配不同的错误

**Matching on Different Errors**


上面清单 9-4 中的代码，不论 `File::open` 因何而失败，都会 `panic!`。然而，这里是要因应不同失败原因，而采取不同措施：在 `File::open` 因为那个文件不存在而失败时，就要创建该文件并返回到那个新建文件的句柄。在那个 `File::open` 因别的其他原因失败 -- 比如没有打开该文件的权限时，这里仍要该代码以清单 9-4 中所做的同样方式，`panic!` 掉。为此，这里就要添加一个内部的 `match` 表达式，如下清单 9-5 中所示。

文件名：`src/main.rs`

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
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

*清单 9-5：以不同方式处理不同类别的错误*

`File::open` 所返回的位于 `Err` 变种内部的值的类型为 `io::Error`，他是一个由标准库提供的结构体。该结构体有个可供调用以获取到 `io::ErrorKind` 值的方法 `kind`。而枚举 `io::ErrorKind` 亦是由标准库提供，并有着表示那些可能自某个 `io` 操作而引起的，不同类别错误的一些变种。这里打算使用的变种为 `ErrorKind::NotFound`，表示了正尝试打开的文件尚不存在。因此这里既对 `f` 进行了匹配，而同时还有了在 `e.kind()` 上的一个内层匹配。

这里打算检查的那个内层匹配中的条件，则是由 `e.king()` 所返回的那个值，是否为 `ErrorKind` 枚举的 `NotFound` 变种。在 `e.kind()` 返回的值为 `ErrorKind` 的 `NotFound` 变种时，这里就尝试以 `File::create` 来创建该文件。然而由于 `Fiel::create` 仍会失败，因此这里就需要在那个内层 `match` 表达式中的第二个支臂。在该文件无法被创建出来时，就会打印出一条不同的错误消息。外层那个 `match` 表达式的第二支臂保持原样，因此该程序会在除了文件未找到错误之外的其他任何错误时，都会中止运行。

> **这种结合`Result<T, E>` 运用 `match` 表达式的替代方案**
>
> 那可是有好多的 `match` ！`match` 表达式是很有用，但同样也是很原始的。在第 13 章，就会了解到闭包（closures），这种与定义在 `Result<T, E>` 上的众多方法一起使用的特性。在对代码中的 `Result<T, E>` 值进行处理时，比起使用 `match` 表达式，这样的闭包方式可以简练得多。
> 比如，下面就是编写与清单 9-5 中同样逻辑，不过却使用了闭包特性与 `unwrap_or_else` 方法的另一种方式。

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt").unwrap_or_else(|e| {
        if e.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic! ("创建文件时发生问题：{:?}", error);
            })
        } else {
            panic! ("打开文件时出现问题：{:?}", e);
        }
    });

    println! ("{:?}", f);
}
```

> 尽管此代码与清单 9-5 有着同样行为，但他并未包含任何的 `match` 表达式，且读起来更清楚。请在读完了第 13 章，并看看标准库文档中的这个 `unwrap_or_else` 方法后，再回到这个示例。在对错误进行处理时，许多别的这些方法，都可以清理掉大量嵌套的 `match` 表达式。


## 出错时而中止的快捷方式：`unwrap` 与 `expect`

**Shortcuts for Panic on Error: `unwrap` and `expect`**


运用 `match` 运作足够良好，不过那样可能有点冗长，且不总是良好地传达了意图。这个 `Result<T, E>` 类型，其上本来就定义了许多用于完成各种各样的、更为具体任务的辅助方法。其中的 `unwrap` 方法，就是一个实现了刚好与前面清单 9-4 中所编写的 `match` 表达式类似的快捷方法。在 `Result` 的值为 `Ok` 变种时，`unwrap` 就会返回那个 `Ok` 内部的值。而在该 `Result` 为 `Err` 变种时，`unwrap` 则会代为调用 `panic!` 宏。下面就是运作中的一个 `unwrap` 示例：

文件名：`src/main.rs`

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
}
```

在没有 `hello.txt` 文件下运行此程序时，就会看到一条来自由这个 `unwrap` 方法做出的 `panic!` 宏调用的错误消息：

```console
$ cargo run                                                         lennyp@vm-manjaro
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/error_handling_demo`
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:4:37
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

同样，`Result<T, E>` 上的 `expect` 方法，则可实现对这条 `panic!` 错误消息的选取。使用 `expect` 而非 `unwrap` 并提供良好的错误消息，就能够传达到自己的意图，进而令到追踪程序中止缘由更为容易。`expect` 方法的语法如下所示：

文件名：`src/main.rs`

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("打开 hello.txt 失败");
}
```

这里以与 `unwrap` 同样方式，使用了 `expect`：用于返回文件句柄，或者对 `panic!` 宏进行调用。而在 `expect` 调用 `panic!` 时用到的错误消息，就将是这里传递给 `expect` 的那个参数，而不再是 `unwrap` 所用到的那个默认 `panic!` 消息了。下面就是该错误消息看起来的样子：

```console
$ cargo run                                                         lennyp@vm-manjaro
   Compiling error_handling_demo v0.1.0 (/home/lennyp/rust-lang/error_handling_demo)
    Finished dev [unoptimized + debuginfo] target(s) in 1.23s
     Running `target/debug/error_handling_demo`
thread 'main' panicked at '打开 hello.txt 失败: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:4:37
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

由于此错误消息是以这里所指定的，`打开 hello.txt 失败` 开始，因此就会更易于搞清楚，此错误消息来自代码中的何处。而若在多处使用 `unwrap`，那么在要精准找出到底是那个 `unwrap` 导致了程序中止时，就会因为所有这些调用了 `panic!` 的 `unwrap`，都打印出同样消息，而要耗费更多时间。


## 传播错误

**Propagating Errors**


在某函数实现调用了可能失败的某些东西时，与其在该函数自身里头对错误进行处理，还可以将该错误返回给调用该函数的代码，这样调用该函数的代码就可以自己决定要做些什么。这就叫做 *传递（propagating）* 错误，而将更多的控制，给到调用该函数的代码，相比于当前实现的函数代码，调用代码中可能会有更多决定该错误应如何被处理的信息或逻辑。

比如，下面清单 9-6 就给出了一个从某个文件读取用户名的函数。在那个文件不存在或无法读取时，这个函数就会将这些错误返回给调用该函数的代码。

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

*清单 9-6：使用 `match` 将错误返回给调用代码的一个函数*

虽然可以简单得多的方式，来重写该函数，不过为了对错误处理进行探索，因此这里就要通过亲自动手完成其大部分代码开头；在结束时，就会给出那更简短的方式。首先来看看该函数的返回值类型：`Result<String, io::Error>`。这表示该函数要返回一个类型 `Result<T, E>` 的值，其中的泛型参数 `T` 已被具体类型 `String` 填充，而那个泛型 `E` 则已被具体类型 `io::Error` 填充。若此函数不带任何问题的成功运行，那么调用该函数的代码，就会收到一个保存着一个 `String` 的 `Ok` 值 -- 即该函数从那个文件中读取到的用户名。而在该函数出现任何问题时，那么调用代码就会收到一个，保存着包含了有关所出现问题更多信息的 `io::Error` 示例的 `Err` 值。这里之所以选择 `io::Error` 作为此函数的返回值，是因为在该函数的函数体中所调用的两个都可能失败的操作：`File::open` 与 `read_to_string`，他们所返回的错误值都是这个 `io::Error` 类型。

该函数的函数体，是以调用 `File::open` 函数开始的。随后这里就以与清单 9-4 中类似方式，使用了一个 `match` 处理 `File::open` 返回的 `Result`。在 `File::open` 成功时，那么在模式变量 `file` 中的文件句柄，就成为那个可变变量 `f` 中的值，且函数会继续执行。而在 `Err` 情形下，这里使用了 `return` 关键字，早早地就从这个函数 `return` 了出去，同时将来自 `File::open` 的那个错位值，此时是在模式变量 `e` 中，作为该函数的错误值，传回给调用该函数的代码。

因此在 `username_file` 有着一个文件句柄时，该函数随后就会创建一个在变量 `username` 中的新 `String`，并调用 `username_file` 中文件句柄上的 `read_to_string` 方法，来将该文件中的内容，读取到 `username` 中。因为即使 `File::open` 运行成功，这个 `read_to_string` 仍可能失败，因此他同样会返回一个 `Result`。那么这里就需要另一个 `match`，来处理这个 `Result`：在 `read_to_string` 成功时，那么接下来这个函数就成功执行了，进而就从这个文件，返回到此时位于封装在一个 `Ok` 中的 `username` 中的用户名来。而在 `read_to_string` 失败时，这里就会以与之前在那个处理 `File::open` 返回值的 `match` 中返回错误值的同样方式，返回现在这个 `read_to_string` 的错误值。不过，由于这是该函数中的最后一个表达式，因此这里无需显示地写下 `return`。

调用此代码的代码，随后就会对收到的包含了用户名 `Ok` 值，或者包含了一个 `io::Error` 类型的 `Err` 值进行处理。至于要对这些值做何处理，则取决于调用代码了。在调用代码收到 `Err` 值时，他就可以采取好比调用 `panic!` 并崩溃掉该程序，可以使用某个默认用户名，或者从相比该文件的其他地方，查找该用户名等操作。这里没有关于那个调用代码确切地尝试要做什么的足够信息，因此这里就把全部的成功或错误信息，向上传递给调用代码，让调用代码进行适当处理。

由于在 Rust 中这样的传递错误模式是如此普遍，以致于 Rust 提供了问好操作符（the question mark operator, `?`），来令到错误传递更加容易。


### 传播错误的快捷方式：`?` 操作符

**A Shortcut for Propagating Errors: the `?` Operator**


下面清单 9-7 给出了与清单 9-6 有着同样功能的一个 `read_username_from_file` 实现，只是此实现使用了 `?` 操作符。

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

*清单 9-7：一个使用 `?` 操作符将错误返回给调用代码的函数*

那个放在某个 `Result` 值后面的 `?`，被定义为几乎与之前所定义的那些，用于处理清单 9-5 中那些 `Result` 值的 `match` 表达式，以同样方式运作。在 `Result` 的值为 `Ok` 时，那么那个 `Ok` 内部的值，就会从该表达式得以返回，且程序将继续运行。而在该 `Result` 值为一个 `Err` 时，则会如同之前曾用到的 `return` 关键字一样，将自这整个函数，返回这个 `Err` 值，进而这个错误值，就被传递给了调用代码。

清单 9-6 中的 `match` 表达式完成的事情，与这个 `?` 操作符完成的事情有个不同点：调用了这个 `?` 操作符的错误值，会经过定义在标准库中 `From` 特质（the `From` trait in the standard library）中定义的 `from` 函数，而该函数被用于将一种类型的值，转换到另一种类型中。当 `?` 操作符调用 `from` 函数时，被接收到的错误类型，就被转换为了定义在当前函数返回值类型中的类型了（即 `Result<String, io::Error>`）。在某个函数可能失败，即便该函数的一些部分而不是整个函数，由于许多不同原因而失败，而返回一种表示这些全部失败方式的一种错误类型时，这个不同之处就会有用。

比如，这里本可将清单 9-7 中的 `read_username_from_file` 函数，修改为返回一个自己定义的名为 `OurError` 的定制错误类型。而在同时给 `OurError` 定义了 `impl From<io::Error>`，以从 `io::Error` 构造出一个 `OurError` 的实例时，那么随后无需添加任何代码到这个函数，`read_username_from_file` 函数中的这些 `?` 操作符，就会调用 `from` 并对那些错误类型进行转换。

在清单 9-7 的语境下，位于 `File::open` 调用末尾的那个 `?`，将返回一个 `Ok` 内部的值给变量 `username_file`。而在有错误发生时，这个 `?` 操作符，就会早早地从整个函数退出，并把任何的 `Err` 值给到调用代码。对于那个 `read_to_string` 调用末尾处的 `?`，适用这同样的情况。

`?` 操作符消除了很多样板代码（a lot of boilerplate），并令到此函数的实现更为简单。通过将这些方法调用在整个 `?` 即刻链接起来，甚至可以进一步缩短此代码，如下清单 9-8 中所示。

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

*清单 9-8：在 `?` 操作符后将方法调用链接起来*

这里已将那个 `username` 中的新 `String` 的创建，挪到了该函数的开头；整个函数就整个部分未作改动。这里没有了变量 `username_file` 的创建，而是已将到 `read_to_string` 的函数调用，直接链接到了 `File::open("hello.txt")?` 的结果上。在 `read_to_string` 调用的末尾仍有一个 `?`，同时在这两个 `File::open` 与 `read_to_string` 调用都成功，而不返回错误时，这里就会返回一个包含了 `username` 的 `Ok` 值。功能仍旧与清单 9-6 和清单 9-7 中是一样的；这只是一种不同的、更为符合人体工程学的编写方式。

下面清单 9-9 给出了使用 `fs::read_to_string` 的一种甚至更加简短的方式。

文件名：`src/main.rs`

```rust
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```

*清单 9-9：使用 `fs::read_to_string` 而非打开在读取那个文件*

将某个文件读取到字符串中，是个相当常见的操作，因此标准库提供了便捷的打开文件、创建一个新 `String`、读取文件内容、将内容放入到那个 `String`，并将其返回的 `fs::read_to_string` 函数。当然，`fs::read_to_string` 的使用，并不能赋予到这里对全部错误处理加以解释的机会，因此这里才要走过前面那些常常的过程。


### 哪些地方可以使用 `?` 操作符

**Where The `?` Operator Can Be Used**


`?` 操作符仅可用于那些返回值类型，与这个 `?` 被用于的那个值类型兼容的函数中。这是由于 `?` 操作符被定义为与在清单 9-6 中，所定义的那个 `match` 表达式类似方式，执行一个该函数早期阶段的退出。在清单 9-6 中，那个 `match` 使用的是一个 `Result` 值，同时那个先期返回支臂返回的是一个 `Err(e)` 值。那么那个函数的返回值类型，就必须是个 `Result`，这样才与这个 `return` 兼容。

在下面清单 9-10 中，就要看看一个有着与其上使用了 `?` 的类型值不兼容返回值的 `main` 函数中，使用 `?` 操作符会收到的错误：

文件名：`src/mian.rs`

```rust
use std::fs::File;

fn main() {
    let greating_file = File::open("hello.txt");
}
```

*清单 9-10：尝试在返回 `()` 的 `main` 函数中使用 `?` 就不会编译*


此代码是要打开一个文件，这就可能失败。那个 `?` 操作符接续了由 `File::open` 所返回的 `Return` 值，然而这个 `main` 函数的返回值类型为 `()`，而非 `Result`。那么在编译此代码时，就会得到以下的错误消息：

```console
$ cargo run                                                                              lennyp@vm-manjaro
   Compiling error_handling_demo v0.1.0 (/home/lennyp/rust-lang/error_handling_demo)
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
 --> src/main.rs:4:48
  |
3 | / fn main() {
4 | |     let greating_file = File::open("hello.txt")?;
  | |                                                ^ cannot use the `?` operator in a function that returns `()`
5 | | }
  | |_- this function should return `Result` or `Option` to accept `?`
  |
  = help: the trait `FromResidual<Result<Infallible, std::io::Error>>` is not implemented for `()`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `error_handling_demo` due to previous error
```

此错误指出了只允许在返回 `Result`、`Option` 或别的实现了 `FromResidual` 的类型的函数中，使用 `?` 操作符。

而要修正这个错误，则有两个选择。一个选择是在没有修改函数返回值类型的限制时，那么就将其修改为与在其上使用 `?` 操作符的值类型兼容。另一技巧，则是使用一个 `match` 表达式，或某个 `Result<T, E>` 的那些方法，来以某种恰当方式对这个 `Result<T, E>` 进行处理了。

这个错误消息还提到，`?` 还可与 `Option<T>` 类型的值一同使用。与在 `Result` 上使用 `?` 一样，可在返回一个 `Option` 的函数中的 `Option` 上使用 `?`。在某个 `Option<T>` 上调用 `?` 操作符的行为，与在 `Result<T, E>` 上其被调用时的行为类似：在该值为 `None` 时，`None` 就会在那个地方及早地从该函数被返回。而在该值为 `Some` 时，那么这个 `Some` 内部的值，就是该表达式的结果值，同时函数会继续执行。下面清单 9-11 有着一个在给定文本中找到第一行最后一个字符的函数示例：

```rust
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
```

*清单 9-11：在某个 `Option<T>` 的值上使用 `?` 操作符*

由于可能那里有个字符，不过同样坑能那里没有字符，因此此函数返回的是 `Option<char>`。这个代码取那个 `text` 字符串切片参数，并在其上调用了 `lines` 方法，该方法返回的是对该字符串中那些文本行的一个迭代器。由于此函数是要对首个文本行进行检查，因此他调用了那个迭代器上的 `next`，来从迭代器上获取头一个值。在 `text` 为空字符串时，那么这个到 `next` 的调用，就会返回 `None`，这也就是这里使用 `?` 来停止这个 `last_char_of_first_line` 函数，并自其返回 `None` 的情形。而在 `text` 不为空字符串时，`next` 就会返回一个包含了在 `text` 中第一行文本的字符串切片的 `Some` 值。

此时 `?` 操作符会提取这个字符串切片，进而就可以在那个字符串切片上调用 `chars`，来获取到他那些字符的一个迭代器。这里关心的是第一行文本中的最后一个字符，因此就要调用 `last` 来返回迭代器中的最后一个条目。因为首个文本行为空字符串是可能的，比如在 `text` 以空行开头却在其他行上有一些字符，如同在 `"\nhi"` 中一样，因此 `last` 得到一个就是个 `Option` 值。不过在首行上有最后一个字符时，这个字符就会在 `Some` 变种里被返回。中间的 `?` 操作符，给到了一种表达此逻辑的简洁方式，运行在一个行里来实现该函数。若无法在 `Option` 上运用这个 `?` 操作符，那么就必须使用更多方法调用，或 `match` 表达式来实现此逻辑。

注意在返回 `Result` 函数中的 `Result` 上，可以使用 `?` 操作符，而在返回 `Option` 函数中的 `Option` 上，可使用 `?` 操作符，但不能混用及进行匹配。`?` 操作符不会自动将 `Result` 转换为 `Option`，或反过来将 `Option` 转换为 `Result`；在这些情况下，是可以在 `Result` 上使用诸如 `ok`，或在 `Option` 上使用 `ok_or` 这样的方法，来显示地完成转换。

到目前为止，这里使用过的所有 `main` 函数，返回的都是 `()`。由于 `main` 函数是可执行程序的进入与退出点，因此他是特殊的，而关于其返回值类型可以是什么，为了程序如预期那样执行，是有一些限制的。

幸运的是，`main` 函数同样可以返回 `Result<(), E>`。下面清单 9-12 有着来自 9-10 的代码，不过这里将 `main` 函数的返回值类型，改成了 `Result<(), Box<dyn Error>>`，并在最后添加了一个返回值 `Ok(())`。现在该代码就会编译了：


```rust
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
```

*清单 9-12：将 `main` 修改为返回 `Result<(), E>`，就实现了在 `Result` 值上 `?` 操作符的使用*

这里的 `Box<dyn Error>` 类型，是个 *特质对象（trait object）*，在第 17 章中的 [“使用允许不同类型值的特质对象”](Ch17_Object_Oriented_Programming_Features_of_Rust.md#使用允许不同类型值的特质对象) 小节，就会讲到这个特性。而现在，可将 `Box<dyn Error>` 理解为表示 “任何类别的错误”。由于 `?` 操作符允许将任何 `Err` 值及早返回，因此将 `?` 用在有着错误类型 `Box<dyn Error>` 的 `main` 函数中， 某个 `Result` 值上是允许的。即使这个 `main` 函数的函数体，将只会返回类型 `std::io::Error` 的那些错误，而经由指定 `Box<dyn Error>`，即使将返回其他错误的代码添加到 `main` 的函数体，该函数签名 `fn main() -> Result<(), Box<dyn Error>>` 仍将无误。

在 `main` 函数返回了一个 `Result<(), E>` 时，那么若 `main` 返回的是 `Ok(())`，则该可执行程序就会以值 `0` 退出，并在 `main` 返回 `Err` 值时，以非零值退出。C 语言编写的可执行程序，在退出时返回的是些整数：成功退出的程序返回整数 `0`，而出错的程序返回某些非 `0` 的整数。Rust 从可执行程序返回的也是整数，从而与此约定兼容。

`main` 函数可能返回任何实现了 [`std::process::Termination` 特质（the `std::process::Termination`）](https://doc.rust-lang.org/std/process/trait.Termination.html) 的任何类型，该特质包含了返回某个 `ExitCode` 的 `report` 函数。请参考标准库文档，了解更多有关实现自己类型 `Termination` 的信息。

现在既然已经讨论了调用 `panic!` 或返回 `Result` 的细节，那么就要回到怎样判断，在何种情形下，使用哪种方式属于恰当的话题了。



