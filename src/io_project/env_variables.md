# 使用环境变量

**Working with Environment Variables**


这里就要通过加入一项额外特性，来改进 `minigrep`：经由使用某个环境变量，用户可以开启与关闭的区分大小写的搜索选项。这里本可以将此特性，构造为一个命令行选项，并在用户打算应该该选项时，要求他们键入该命令行选项，而不是将其构造为一个环境变量，这样就允许用户只设置该环境变量一次，而在那次终端会话中的全部搜索，都是区分大小写的了。


## 编写这个区分大小写的 `search` 函数的失效测试

**Writing a Failing Test for the Case-Insensitive `search` Function**


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


## 实现 `search_case_insensitive` 函数

**Implementing the `search_case_insensitive` Function**


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
