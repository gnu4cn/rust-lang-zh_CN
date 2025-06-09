# 以测试驱动的开发方式，开发库的功能

**Developing the Library's Functionality with Test Driven Development**


现在，我们已将业务逻辑提取到了 `src/lib.rs` 中，并将参数收集和错误处理，留在了 `src/main.rs` 中，这样就更容易为咱们代码的核心功能编写测试。无需在命令行调用我们的二进制文件下，我们即可以不同参数直接调用函数，并检查返回值。


在本节中，我们将使用测试驱动的开发，the test-driven development, TDD，流程，将检索逻辑添加到这个 `minigrep` 程序，步骤如下：

1. 编写一个会失败的测试，并运行他以确保他会以咱们预期的原因失败；
2. 编写或修改足够的代码，使新测试通过；
3. 重构咱们刚刚添加或修改的代码，并确保测试继续通过；
4. 重复步骤 `1` 开始的上述步骤。


尽管测试驱动的开发只是编写软件的众多方法之一，但他有助于推动代码设计。在编写令到测试通过的代码之前先编写测试，有助于在整个过程中，保持较高的测试覆盖率。

我们将以测试驱动，将具体完成对文件内容中查询字符串进行检索，并生成与查询匹配行清单的功能实现。我们将把此功能添加到一个名为 `search` 的函数中。


## 编写一个失败测试

**Writing a Failing Test**


因为我们不再需要 `src/lib.rs` 及 `src/main.rs` 中，那些咱们曾用于检查程序行为的 `println!` 语句，咱们来移除掉他们。然后，在 `src/lib.rs` 中，我们将添加带有一个测试函数的 `tests` 模组，就像我们在第 11 章中所做的那样。该测试函数指定了我们希望这个 `search` 函数所具备的行为：他将取一个查询及要检索的文本，并仅返回文本中包含该查询的行。下面清单 12-15 展示了这个测试，目前他还不会编译。


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

*清单 12-15：为我们希望有的 `search` 函数创建一个失败测试*


这个测试会检索字符串 `"duct"`。我们要检索的文本共有三行，其中仅有一行包含 `"duct"`（请注意，第一个双引号后的反斜杠，告诉 Rust 不要在这个字符串字面值内容的开头添加换行符）。我们断言了这个 `search` 函数所返回的值，仅包含我们预期的那个行。

我们目前还无法运行这个测试并观察其失败，因为该测试甚至不会编译：`search` 函数还不存在！遵循 TDD 原则，通过添加始终返回一个空矢量值的 `search` 函数定义，我们将只添加足够使该测试编译并运行的代码，如下清单 12-16 所示。然后这个测试应编译并失败，因为空的矢量值与包含 `"safe, fast, productive"` 这个行的矢量不匹配。


文件名：`src/lib.rs`

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    vec! []
}
```

*清单 12-16：仅定义 `search` 函数的必要部分，以便我们的测试将会编译*

请注意，我们需要在 `search` 的签名中，定义一个显式的生命周期 `'a`，并对 `contents` 参数及返回值，使用该生命周期。回顾 [第 10 章](..generic_types_traits_and_lifetimes/lifetimes.md)，生命周期参数会指定出哪个参数的生命周期，与返回值的生命周期有联系。在这个示例中，我们表明了返回的矢量值，应包含引用了 `contents` 参数切片（而非 `query` 参数的切片）的字符串切片。

换句话说，我们告诉 Rust，由 `search` 函数返回的数据，将与 `contents` 参数中传递给 `search` 函数的数据，具有相同生命周期。这一点非常重要！某个切片所引用的数据必须有效，这个引用才会有效；若编译器假定我们构造的是 `query` 而非 `contents` 的字符串切片，那么他将不正确地执行安全检查。

如果我们忘记了这些生命周期注解并尝试编译这个函数，我们将得到以下报错：


```console
$ cargo build
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

Rust 不可能明白我们需要两个参数中的哪个，因此我们需要显式地告诉他。由于 `contents` 是包含所有文本的那个参数，而我们打算返回与该文本匹配的部分，因此我们知道，`contents` 应是要使用生命周期语法，与返回值关联的那个参数。

其他编程语言不会要求咱们在函数签名中，将参数与返回值联系起来，但这种做法将随时间推移而变得愈加容易。咱们可将这个示例，与第10章 [“使用生命周期验证引用”](../generic_types_traits_and_lifetimes/lifetimes.md) 小节中的示例进行比较。


现在咱们来运行这个测试：

```console
$ cargo test
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

太好了，这个测试失败了，正如我们预期。我们来让这个测试通过吧！


## 编写代码让测试通过

**Writing Code to Pass the Test**

目前，我们测试失败是因为我们总是返回一个空的矢量值。为修复这个问题并实现 `search`，我们的程序需要完成以下步骤：

1. 遍历内容的每一行；
2. 检查该行是否包含我们的查询字符串;
3. 在包含查询字符串时，将其添加到我们返回的值列表中；
4. 在不包含时，什么也不做；
5. 返回匹配结果清单。

下面就来逐一完成各个步骤，从那些文本行的迭代开始。


### 以 `lines` 方法遍历文本行

**Iterating Through Lines with the `lines` Method**


Rust 有个处理字符串逐行迭代的方便方法，该方法名为 `lines`，其工作方式如下清单 12-17 所示。请注意，这段代码还不会编译。

文件名：`src/lib.rs`

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    for line in contents.lines() {
        // 对单个文本行进行一些操作
    }
}
```

*清单 12-17：遍历 `contents` 中的各行*

`lines` 方法会返回一个迭代器。我们将在 [第 13 章](../functional_features/iterators.md) 中，详细讨论迭代器，但请回想一下，咱们在 [清单 3-5](../programming_concepts/control_flow.md#listing-3-5)  中就已见过这种使用迭代器的方法，当时我们与迭代器一起使用了 `for` 循环，对集合中的各个元素执行一些代码。


### 在各行中检索查询字符串

**Searching Each Line for the Query**


接下来，我们将检查当前行是否包含我们的查询字符串。幸运的是，字符串提供了一个名为 `contains`，帮助我们完成这项任务的有用方法！在 `search` 函数中，添加一个到 `contains` 方法的调用，如下清单 12-18 所示。请注意，此时代码仍不会编译。



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

*清单 12-18：添加检查行中是否包含 `query` 中字符串的功能*

目前，我们正构建起功能来。为使代码编译，我们需要自函数体返回一个值，正如我们在函数签名中所指明的那样。


### 存储匹配的行

**Storing Matching Lines**

要完成这个函数，我们需要一种存储我们打算返回匹配行的方法。为此，我们可在 `for` 循环前，创建一个可变的矢量值，并调用 `push` 方法将某个 `line` 存储在该矢量中。在 `for` 循环后，我们就要返回该矢量值，如下清单 12-19 所示。


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

*清单 12-19：存储匹配行，从而咱们可以返回他们*


现在这个 `search` 函数应只返回那些包含 `query` 的行，同时我们的测试应该通过。我们来运行这个测试：


```console
$ cargo test
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

我们的测试通过了，所以我们知道 `search` 函数可以工作了！


此时，我们就可以在确保测试通过以保持相同功能的同时，考虑对 `search` 函数的实现进行重构。这个检索函数中的代码并不算太差，但他并未充分利用迭代器的一些有用特性。我们将在 [第 13 章](../functional_features/iterators.md) 中回到这个示例，那里我们将详细探讨迭代器，并研究如何对其进行改进。


### 在函数 `run` 中使用 `search` 函数

**Using the `search` Function in the `run` Function**


现在这个 `search` 函数已经工作并经过测试，我们需要在咱们的 `run` 函数中调用 `sarch` 了。我们需要将 `config.query` 值，及 `run` 从文件中读取的 `contents`，传递给 `search` 函数。然后 `run` 将打印从 `search` 函数返回的各行：


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

我们仍使用了个 `for` 循环，返回 `search` 结果中的各行并打印出来。

现在整个程序就应该可以工作了！我们来试一试，首先以一个应该能从艾米莉·狄金森的诗中，准确返回一行内容的单词：*frog*。

```console
$ cargo run -- frog poem.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep frog poem.txt`
在文件 poem.txt 中检索：frog
How public, like a frog
```

太棒了！现在我们来尝试一个可以匹配多行内容的词，比如 *body*：

```console
$ cargo run -- body poem.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep body poem.txt`
在文件 poem.txt 中检索：body
I'm nobody! Who are you?
Are you nobody, too?
How dreary to be somebody!
```


最后，我们来确保在咱们检索诗中不存在的单词时，不会出现任何行，比如 *monomorphization*：


```console
$ cargo run -- monomorphization poem.txt
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/minigrep monomorphization poem.txt`
```


太棒了！我们已经构建出了一个经典工具的咱们自己的迷你版本，并掌握了如何设计架构应用的大量知识。此外，我们还掌握了文件输入输出、生命周期、测试以及命令行解析等方面的知识。

为完善这个项目，我们将简要演示：

- 如何使用环境变量；
- 以及如何将输出打印到标准错误。

这两者在咱们编写命令行的程序时，都非常有用。



（End）


