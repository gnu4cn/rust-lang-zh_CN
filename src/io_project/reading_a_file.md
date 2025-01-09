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


（End）


