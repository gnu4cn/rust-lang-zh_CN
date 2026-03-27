## 读取文件

现在，我们将添加读取于 `file_path` 参数中指定的文件的功能。首先，我们需要以一个样本文件来测试他：我们将使用一个带有多行少量文本、有一些重复单词的文件。下面清单 12-3 有一首 Emily Dickinson 的诗，非常合适！在咱们项目的根目录处创建一个名为 `poem.txt` 的文件，并输入这首诗 “I'm Nobody! Who are you?”

<a name="listing_12-3">
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

**清单 12-3**：Emily Dickinson 的一首诗是一个很好的测试用例


文本就位后，编辑 `src/main.rs` 并添加代码来读取该文件，如下清单 12-4 中所示。

<a name="listing_12-4">
文件名：`src/main.rs`

```rust
use std::{env, fs};

fn main() {
    // --跳过代码--
    println! ("在文件 {file_path} 中");

    let contents = fs::read_to_string(file_path)
        .expect("应该已经能够读取文件");

    println! ("带有文本：\n{contents}");
}
```

**清单 12-4**：读取由第二个参数指定的文件内容

首先，我们以一个 `use` 语句带入标准库的相关部分：我们需要 `std::fs` 来处理文件。

在 `main` 函数中，新的语句 `fs::read_to_string` 取 `file_path`，打开该文件，并返回一个类型 `std::io::Result<String>` 的值，包含文件的内容。

之后，我们再次添加一个临时的 `println!` 语句，在文件读取后打印 `contents` 的值，以便我们可以检查程序到目前为止是否正常运行。

我们来以任意字符串作为第一个命令行参数（因为我们还没有实现检索部分），并以 `poem.txt` 文件作为第二个参数运行这段代码：

```rust
$ cargo run -- the poem.txt
   Compiling minigrep v0.1.0 (/home/hector/rust-lang-zh_CN/projects/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/minigrep the poem.txt`

        在文件 poem.txt 中
        检索 the

带有文本：
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!

```

太棒了！这段代码读取然后打印了文件的内容。但这段代码有一些缺陷。目前，`main` 函数有着多重义务：通常，当每个函数只负责一个想法时，那么函数会更清晰，更容易维护。另一个问题是我们没有尽可能地处理错误。程序还很小，因此这些缺陷就不是什么大问题，但随着程序的增长，彻底修复他们将变得更加困难。在开发程序时尽早开始重构属于一种很好的做法，因为重构少量的代码要容易得多。接下来我们就将做这件事情。

（End）


