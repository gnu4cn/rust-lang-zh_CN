# 以测试驱动开发添加功能

现在，我们让 `src/lib.rs` 中的检索逻辑从 `src/main.rs` 中分离出来，这样编写代码的核心功能的测试就容易多了。我们可以不同参数直接调用函数并检查返回值，而无需从命令行调用二进制文件。

在这一小节中，我们将使用测试驱动的开发，the test-driven development, TDD，流程，按照以下步骤添加检索逻辑到 `minigrep` 程序：

1. 编写一个会失败的测试，并运行他以确保他因咱们预期的原因失败；
2. 编写或修改刚好让新测试通过的代码；
3. 重构咱们刚添加或修改的代码，并确保测试继续通过；
4. 从步骤 1 开始重复！


尽管这只是编写软件的许多方法之一，但测试驱动开发可以帮助推动代码设计。在咱们编写使测试通过的代码之前编写测试，有助于在整个过程中保持较高的测试覆盖率。

我们将测试驱动这一功能的实现，其将具体完成在文件内容中检索查询字符串，并生成与查询字符串匹配的行的列表。我们将在名为 `search` 的函数中添加这一功能。


## 编写一个失败的测试

在 `src/lib.rs` 中，我们将添加一个带有测试函数的 `tests` 模组，就像我们在 [第 11 章](../automated_tests/howto.md#组织测试函数) 中所做的那样。这个测试函数指定我们希望 `search` 要有的行为：他将取一个查询字符串和要检索的文本，并将仅返回文本中包含该查询字符串的行。下面清单 12-15 展示了这个测试。

<a name="listing_12-15"></a>
文件名：`src/lib.rs`

```rust
// -- 跳过代码 --
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

**清单 12-15**：为我们希望有的功能创建 `search` 函数的一个失败测试

这个测试会检索字符串 `"duct"`。我们检索的文本有三行，其中只有一行包含 `"duct"`（请注意，开头的双引号后的反斜杠告诉 Rust 不要在字符串字面值内容的开头添加换行符）。我们断言 `search` 函数返回的值只包含我们期望的行。

当我们运行这个测试时，他目前将失败，因为 `unimplemented!` 宏会以消息 `"not implemented"` 终止运行。根据 TDD 原则，我们将采取微小步骤，通过定义 `search` 函数始终返回空矢量值，仅添加让测试不致终止运行的足够代码，如下清单 12-16 中所示。然后，这个测试应编译并失败，因为空矢量值不匹配包含 `"safe, fast, productive"` 行的矢量值。


<a name="listing_12-16"></a>
文件名：`src/lib.rs`

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    vec! []
}
```

**清单 12-16**：仅定义 `search` 函数的必要部分，以便调用他不会终止运行

现在我们来讨论一下为什么我们需要在 `search` 的签名中，定义显式的生命周期 `'a` 并对 `contents` 参数和返回值使用该生命周期。回顾 [第 10 章](../generic_types_traits_and_lifetimes/lifetimes.md)，生命周期参数规定哪个参数的生命周期与返回值的生命周期相关联。在这一情形下，我们表明返回的矢量值应包含引用参数 `contents`（而非参数 `query`） 中的切片的字符串切片。

换句话说，我们告诉 Rust，`search` 函数返回的数据将与在 `contents` 参数中传递给 `search` 函数的数据具有相同的生命周期。这一点非常重要！切片引用的数据必须有效，引用才有效；当编译器假设我们正在构造 `query` 的字符串切片，而不是 `contents` 的字符串切片时，他将错误地执行安全检查。

当我们忘记生命周期注解并尝试编译这个函数时，我们将得到下面这个报错：


```console
$ cargo build
   Compiling minigrep v0.1.0 (/home/hector/rust-lang-zh_CN/projects/minigrep)
error[E0106]: missing lifetime specifier
 --> src/lib.rs:1:52
  |
1 | pub fn search (query: &str, contents: &str) -> Vec<&str> {
  |                       ----            ----         ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `query` or `contents`
help: consider introducing a named lifetime parameter
  |
1 | pub fn search<'a> (query: &'a str, contents: &'a str) -> Vec<&'a str> {
  |              ++++          ++                 ++              ++

For more information about this error, try `rustc --explain E0106`.
error: could not compile `minigrep` (lib) due to 1 previous error
```

Rust 无法知道针对输出我们需要两个参数中的哪一个，因此我们需要显式地告诉他。请注意，帮助文本建议对所有参数及输出类型，指定同一个声明周期参数，这是不正确的！因为 `contents` 是包含我们所有文本的参数，而我们打算该文本中匹配的部分，所以我们知道 `contents` 是应使用生命周期语法连接到返回值的唯一参数。

其他编程语言不要求咱们在签名中连接参数到返回值，但随着时间推移，这种做法将变得更加容易。咱们可能希望将这个示例，与第 10 章中 [以生命周期验证引用](../generic_types_traits_and_lifetimes/lifetimes.md) 小节中的示例比较。


## 编写代码通过测试

目前，我们的测试是失败的，因为我们总是返回一个空矢量值。为了解决这个问题并实现 `search`，我们的程序需要遵循以下步骤：

1. 遍历内容中的每一行；
2. 检查该行是否包含我们的查询字符串;
3. 当包含时，添加他到我们返回的值列表中；
4. 当不包含时，什么也不做；
5. 返回匹配的结果列表。

我们来从迭代行开始，完成每个步骤。


### 以 `lines` 方法遍历行

Rust 有个处理字符串的逐行遍历的方便方法，方便地命名为 `lines`，其工作原理如下清单 12-17 中所示。请注意，这还不会编译。

<a name="listing_12-17"></a>
文件名：`src/lib.rs`

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    for line in contents.lines() {
        // 对行执行一些操作
    }
}
```

**清单 12-17**：遍历 `contents` 中的每一行

`lines` 方法返回一个迭代器。我们将在 [第 13 章](../functional_features/iterators.md) 中详细讨论迭代器。但请回想一下，咱们在 [清单 3-5](../programming_concepts/control_flow.md#listing_3-5)  中见过这种使用迭代器的方式，其中我们对迭代器使用 `for` 循环，对集合中的每个元素运行一些代码。


### 针对查询字符串检索每行

接下来，我们将检查当前行是否包含我们的查询字符串。幸运的是，字符串有个名为 `contains` 的有用方法，为我们完成这点！在 `search` 函数中添加到 `contains` 方法的调用，如下清单 12-18 中所示。请注意，这仍不会编译。

<a name="listing_12-18"></a>
文件名：`src/lib.rs`

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    for line in contents.lines() {
        if line.contains(query) {
            // 对行执行某些操作
        }
    }
}
```

**清单 12-18**：添加功能以查看该行是否包含 `query` 中的字符串

目前，我们正在构建功能。为了使代码编译，我们需要自函数体中返回一个值，就像我们在函数签名中指出的那样。


### 存储匹配行

为了完成这一功能，我们需要一种存储我们打算返回的匹配行的方式。为此，我们可在 `for` 循环前创建一个可变矢量值，并调用 `push` 方法存储 `line` 于该矢量中。在 `for` 循环之后，我们返回这个矢量值，如下清单 12-19 中所示。


<a name="listing_12-19"></a>
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

**清单 12-19**：存储匹配的行以便我们可以返回他们

现在 `search` 函数应返回包含 `query` 的行，并且我们的测试应该通过。我们来运行测试：

```console
$ cargo test
   Compiling minigrep v0.1.0 (/home/hector/rust-lang-zh_CN/projects/minigrep)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.17s
     Running unittests src/lib.rs (target/debug/deps/minigrep-cd30d5119c388f2d)

running 1 test
test tests::one_result ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/minigrep-4ae7116f75385004)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests minigrep

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

我们的测试通过了，所以我们知道他正常工作！

此时，我们可以在保持测试通过以维持相同的功能的同时，考虑重构检索函数的实现的机会。检索函数中的代码并不算太差，但他并未利用迭代器的一些有用特性。我们将在 [第 13 章](../functional_features/iterators.md) 回到这个示例，届时我们将详细探讨迭代器，并研究如何改进他。

现在整个程序应该可以运行了！我们来试一试，首先以一个应该恰好返回 Emily Dickinsion 诗中一行的单词：*frog*。

```console
$ cargo run -- frog poem.txt
   Compiling minigrep v0.1.0 (/home/hector/rust-lang-zh_CN/projects/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/minigrep frog poem.txt`

        在文件 poem.txt 中
        检索 frog
How public, like a frog
```

很好！现在我们来尝试一个将匹配多行的单词，比如 *body*：

```console
$ cargo run -- body poem.txt
   Compiling minigrep v0.1.0 (/home/hector/rust-lang-zh_CN/projects/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/minigrep body poem.txt`

        在文件 poem.txt 中
        检索 body
I'm nobody! Who are you?
Are you nobody, too?
How dreary to be somebody!
```

最后，我们来确保当我们检索诗中任何地方都没有的单词时，我们不会得到任何行，比如 *monomorphization*：

```console
$ cargo run -- monomorphization poem.txt
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep monomorphization poem.txt`

        在文件 poem.txt 中
        检索 monomorphization
```

相当出色！我们已构建了一个经典工具的自己的迷你版本，并学习了很多有关如何架构应用的知识。我们还学习了一些有关文件输入与输出、声明周期、测试与命令行解析的知识。

为了完善这个项目，我们将简要演示如何使用环境变量，以及如何打印到标准错误，这两个方面在咱们编写命令行程序时都很有用。


