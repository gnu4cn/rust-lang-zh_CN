# 以测试驱动的方法，开发这个库的功能

**Developing the Library's Functionality with Test Driven Development**


既然已将业务逻辑提取到了 `src/lib.rs` 中，而将参数收集与错误处理留在 `src/main.rs` 中，那么编写这里代码核心功能的测试，就容易多了。这里可直接以不同参数来调用那些函数，并在不必从命令行调用这里二进制程序之下，对他们的返回值加以检查。

在本小节中，这里将按照以下步骤，运用测试驱动开发流程（the test-driven development(TDD) process），把搜索逻辑添加到这个 `minigrep` 程序：

1. 编写一个会失败的测试并加以运行，从而确保其会以所设想的原因失败；
2. 编写或修改仅足够的代码，来令到新的测试通过；
3. 对刚添加或修改过的代码加以重构，并确保那些测试继续通过；
4. 重复步骤 `1` 开始的上述步骤。

尽管这只是众多编写软件方式之一，TDD 是可以推动代码设计的。在编写令到测试通过的代码之前就编写测试，有助于维持贯穿整个开发过程中，较高程度的测试覆盖面。

这里将以测试驱动具体完成搜索出文件内容中查询字符串，以及产生出与该查询匹配的行清单两个功能的实现。这里将把此功能，添加在一个叫做 `search` 的函数里。


## 编写一个失效测试

**Writing a Failing Test**


由于不再需要 `src/lib.rs` 与 `src/main.rs` 中的那些，曾用于对该程序行为加以检查的 `println!` 语句，因此这里就要将其移出掉。随后，就要在 `src/lib.rs` 中，添加带有一个测试函数的 `tests` 模组，就跟曾在 [第 11 章](Ch11_Writing_Automated_Tests.md#测试函数剖析) 曾做过的那样。该测试函数指明了这里所打算的这个 `search` 函数要有的行为：他将取得一个查询字串，与要搜索的文本，同时他将只返回搜索文本中，包含了查询字串的那些行。下面清单 12-15 给出了这个测试，该清单尚不会编译。

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

请注意这里需要在 `search` 的函数签名中，定义一个显式的生命周期 `'a`，并在 `contents` 参数与返回值上，使用那个生命周期。回顾 [第 10 章](Ch10_Generic_Types_Traits_and_Lifetimes.md#使用生命周期对引用加以验证) 中讲到，这些生命周期参数指明了哪个参数生命周期，是与返回值生命周期联系起来的。在这个示例中，这就表示那个返回的矢量，应包含引用了参数 `contents` （而非参数 `query`）的一些切片的字符串切片。

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

别的编程语言并不会要求在函数签名中，将参数与返回值联系起来，但随着时间的推移，这样的实践将变得容易起来。或许你会将这个示例，与第 10 章中的 [“使用生命周期对引用进行验证” 小节](Ch10_Generic_Types_Traits_and_Lifetimes.md#使用生命周期对引用加以验证) 加以比较。

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


## 编写让测试通过的代码

**Writing Code to Pass the Test**


此刻，由于这里始终返回一个空的矢量值，导致这里的测试失败。要修复这个测试失败并实现 `search`，这里的程序就需要遵循下面这些步骤：

- 对那个内容的各个行加以迭代；
- 检查该行是否包含这里的查询字串；
- 在包含查询字串时，将该行添加到这里正要返回的值清单；
- 在不包含查询字串时，就什么也不做；
- 返回匹配结果的清单。

下面就来逐一完成各个步骤，从那些文本行的迭代开始。


### 使用 `lines` 方法对文本行进行遍历

**Iterating Through Lines with the `lines` Method**


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

这个 `lines` 方法，返回的是个迭代器（an iterator）。在 [第 13 章](Ch13_Functional_Language_Features_Iterators_and_Closures.md#使用迭代器对条目系列进行处理) 中，就会讲到迭代器，不过回顾一下 [清单 3-5](Ch03_Common_Programming_Concepts.md#使用-for-对集合进行遍历) 中，就曾见过这种用到迭代器的方式，那里曾用到一个 `for` 循环, 就带有一个用于在集合中各个元素上，运行某些代码的迭代器。


### 在各行中搜索查询字串

**Searching Each Line for the Query**


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


### 存储匹配的那些行

**Storing Matching Lines**


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

到这里，咱们就会在保持这些测试通过，以维持这同样功能的同时，考虑对这个 `search` 函数的实现，进行重构的一些机会。这个 `search` 函数中的代码虽不怎么差劲，但他并没有利用上迭代器的一些有用特性。在 [第 13 章](Ch13_Functional_Language_Features_Iterators_and_Closures.md#使用迭代器对条目系列进行处理) 中将回到这个示例，那里就会详细探讨到迭代器，进而会看看怎样来改进这个 `search` 函数。


### 在函数 `run` 中使用 `search` 函数

**Using the `search` Function in the `run` Function**


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


（End）


