# 怎样编写测试

所谓 *测试*，属于一些 Rust 函数，验证非测试代码是否以预期方式运行。测试函数的主体通常执行以下三种操作：

- 任何需要的数据或状态；
- 运行咱们打算测试的代码；
- 断言结果是咱们所期望的。

我们来看看，Rust 专门为编写执行这些操作的测试而提供的特性，包括 `test` 属性、数个宏以及 `should_panic` 属性。


## 组织测试函数

最简单来说，Rust 中的测试，就是一个以 `test` 属性注解的函数。所谓属性，是指有关 Rust 代码片段的元数据；一个示例是我们在第 5 章中 [与结构体一起使用的 `derive` 属性](../structs/example_program.md#以派生特质添加功能)。要改变函数为测试函数，就要在 `fn` 之前的行上添加 `#[test]`。当咱们以 `cargo test` 命令运行测试时，Rust 会构建一个测试运行程序的二进制文件，运行这些注解过的函数并报告每个测试函数通过或失败。

每当我们以 Cargo 构造一个新的库项目时，就会自动为我们生成一个包含测试函数的测试模组。这个模组给予我们一个用于编写测试的模板，这样咱们就不必在每次开始新项目时都去查找确切的测试结构和语法。咱们可以根据需要添加任意数量的额外测试函数与测试模组！

在实际测试任何代码之前，我们将通过试验模板测试来探讨测试工作原理的一些方面。然后，我们将编写一些真实世界的测试，调用我们已编写的一些代码并断言其行为是正确的。

我们来创建一个名为 `adder` 的新库项目，将把两个数字相加：

```console
$ cargo new adder --lib
    Creating library `adder` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
$ cd adder
```


咱们 `adder` 库中 `src/lib.rs` 文件内容，应看起来如下清单 11-1 那样。


<a name="listing_11-1"></a>
文件名：`src/lib.rs`

```rust
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

**清单 11-1**：由 `cargo new` 自动生成的代码

该文件以示例的 `add` 函数开头，以便我们有要测试的内容。

现在，我们仅关注 `it_works` 函数。请注意 `#[test]` 注解：这个属性表明这是个测试函数，因此测试运行程序知道，要将这个函数视为测试。我们也可能在 `tests` 模组中有非测试函数，来帮助建立常见场景或执行常见操作，因此我们始终需要表明哪些函数属于测试。

示例函数体使用 `assert_eq!` 宏来断言 `result` 等于 4，其包含以 2 和 2 调用 `add` 的结果。这个断言充当典型测试的格式示例。我们来运行他，看看这个测试是否通过。

`cargo test` 命令会运行项目中的所有测试，如下清单 11-2 中所示。

<a name="listing_11-2"></a>
```console
$ RUSTFLAGS="-A warnings" cargo test
   Compiling adder v0.1.0 (/home/hector/rust-lang-zh_CN/projects/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.37s
     Running unittests src/lib.rs (target/debug/deps/adder-54be618fc528890c)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

> **译注**：这里以 `RUSTFLAGS="-A warnings" cargo test` 命令，在设置环境变量 `RUSTFLAGS` 后，可以抑制编译器的告警消息。

**清单 11-2**：运行自动生成的测试的输出

Cargo 编译并运行了该测试。我们看到行 `running 1 test`。下一行显示生成的测试函数的名字，称为 `it_works`，并且运行该测试的结果为 `ok`。总体总结 `test result: ok.` 表示所有测试都通过了，而读 `1 passed; 0 failed` 的部分则统计了通过与失败的测试数据。

可以标记测试为忽略，这样他就不会在特定实例中运行；我们将在本章后面的 [除非特别要求否则忽略测试](./how_tests_are_run.md#除非特别要求否则忽略一些测试) 小节中介绍这点。因为我们在这里尚未这样做，所以总结显示 `0 ignored`。我们还可以传递参数给 `cargo test` 命令，以仅运行名字与某个字符串匹配的测试；这称为 *过滤，filtering*，我们将在 [根据名字运行测试子集](./how_tests_are_run.md#根据名字运行测试子集) 小节中介绍他。在这里，我们未曾过滤运行的测试，因此总结的末尾显示 `0 filtered out`。

`0 measured` 的统计数据，针对测量性能的基准测试。截至本文撰写时，基准测试仅在每日构建版的 Rust 中可用。请参阅 [关于基准测试的文档](https://doc.rust-lang.org/unstable-book/library-features/test.html) 了解更多信息。

从 `Doc-tests adder` 处开始的测试输出的下一部分，属于文档测试的结果。我们还没有任何文档测试，但 Rust 可以编译出现于 API 文档中的任何代码示例。这一特性有助于保持咱们的文档与代码同步！我们将在第 14 章的 [作为测试的文档注释](../crates-io/publishing.md#作为测试的文档注释) 小节中讨论怎样编写文档测试。现在，我们将忽略 `Doc-tests` 的输出。

我们来开始根据需求定制测试。首先，修改 `it_works` 函数的名字为别的名字，比如 `exploration`，像下面这样：


文件名：`src/lib.rs`


```rust
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

然后，再次运行 `cargo test`。输出现在显示 `exploration`，而不是 `it_works`：


```console
$ RUSTFLAGS="-A warnings" cargo test
   Compiling adder v0.1.0 (/home/hector/rust-lang-zh_CN/projects/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.07s
     Running unittests src/lib.rs (target/debug/deps/adder-54be618fc528890c)

running 1 test
test tests::exploration ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

现在我们将添加另一个测试，但这次我们将构造一个会失败的测试！当测试函数中某处代码终止运行时，测试就会失败。每个测试都在一个新的线程中运行，当主线程发现某个测试线程已死亡时，该测试会被标记为失败。在第 9 章中，我们讨论过终止运行的最简单方式是调用 `panic!` 宏。请将新的测试作为名为 `another` 的函数输入，这样咱们的 `src/lib.rs` 看起来就像下面的清单 11-3 那样。


<a name="listing_11-3"></a>
文件名：`src/lib.rs`

```rust
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        panic! ("使这个测试失败");
    }
}

```

**清单 11-3**：添加第二个测试，由于我们调用了 `panic!` 宏，该测试将失败

使用 `cargo test` 再次运行测试。输出应如同清单 11-4 那样，表明我们 `exploration` 测试通过了，而 `another` 失败了。

<a name="listing_11-4"></a>
```console
$ RUSTFLAGS="-A warnings" cargo test
   Compiling adder v0.1.0 (/home/hector/rust-lang-zh_CN/projects/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.07s
     Running unittests src/lib.rs (target/debug/deps/adder-54be618fc528890c)

running 2 tests
test tests::exploration ... ok
test tests::another ... FAILED

failures:

---- tests::another stdout ----

thread 'tests::another' (116643) panicked at src/lib.rs:17:9:
使这个测试失败
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::another

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`
```

**清单 11-4**：一项测试通过与一项测试失败时的测试结果

`test tests::another` 行显示 `FAILED`，而不是 `ok`。在单个结果与摘要之间出现两个新的小节：第一个小节显示每个测试失败的详细原因。在这一情形下，我们得到 `tests::another` 失败的详细信息，因为他在 `src/lib.rs` 文件中的第 17 行，以 `使该测试失败` 消息终止了运行。下一个小节仅列出所有失败测试的名字，这在存在大量测试和大量详细失败测试输出时非常有用。我们可以使用失败测试的名字来运行该项测试，以便更容易地调试他；我们将在 [控制测试运行方式](./how_tests_are_run.md) 小节进一步讨论运行测试的方式。

摘要行显示在最后：总体上看，我们的测试结果为 `FAILED`。我们有一项测试通过，一项测试失败。

现在我们已经了解了不同场景下的测试结果，我们来看看除了 `panic!` 外，还有哪些在测试中很有用的宏。


## 以 `assert!` 检查结果

在我们想要确保测试中某一条件求值为 `true` 时，标准库提供的 `assert!` 宏非常有用。我们给予 `assert!` 宏一个求值为布尔值的参数。当值为 `true` 时，就什么也不会发生，测试通过。当值为 `false` 时，则 `assert!` 宏调用 `panic!` 导致测试失败。使用 `assert!` 宏会帮助我们检查代码是否以我们预期的方式运作。

在第 5 章 [清单 5-15](../structs/method_syntax.md#listing_5-15) 中，我们使用了 `Rectangle` 结构体和 `can_hold` 方法，其在下面清单 11-5 中得以复现。我们来放置这段代码于 `src/lib.rs` 文件中，然后使用 `assert!` 宏为其编写一些测试。

<a name="listing_11-5"></a>
文件名：`src/lib.rs`

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        (self.width > other.width && self.height > other.height)
        || (self.width > other.height && self.height > other.width)
    }
}
```

**清单 11-5**：第 5 章中的 `Rectangle` 结构体及其 `can_hold` 方法

`can_hold` 方法返回一个布尔值，这意味着他是 `assert!` 宏的绝佳用例。在下面清单 11-6 中，我们编写了个测试，通过创建一个宽度为 8 ，高度为 7 的 `Rectangle` 实例，并断言他可以容纳另一个宽度为 5，高度为 1 的 `Rectangle` 实例来验证 `can_hold` 方法。


<a name="listing_11-6"></a>
文件名：`src/lib.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert! (larger.can_hold(&smaller));
    }
}
```

**清单 11-6**：对 `can_hold` 的测试，检查较大的矩形是否确实可以容纳较小的矩形

请注意 `tests` 模组内部的 `use super::*;` 这一行。`tests` 模组是个常规模组，遵循我们在第 7 章中 [引用模组树中项目的路径](../packages_crates_and_modules/paths.md) 小节中介绍的常见可见性规则。因为 `tests` 模组是个内层模组，所以我们需要带入外层模组中的受测试代码到这个内层模组的作用域。我们在这里使用了通配符，a glob, `*`，因此我们在外层模组中定义的任何内容都对这个 `tests` 模组可用。

我们已命名测试为 `larger_can_hold_smaller`，并创建了我们需要的两个 `Rectanble` 实例。然后，我们调用了 `assert!` 宏并传递了 `larger.can_hold(&smaller)` 的结果给他。这个表达式应返回 `true`，因此我们的测试应该通过。我们来看看吧！


```console
$ RUSTFLAGS="-A warnings" cargo test
   Compiling rectangle v0.1.0 (/home/hector/rust-lang-zh_CN/projects/rectangle)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running unittests src/lib.rs (target/debug/deps/rectangle-a5e83cc30155b35c)

running 1 test
test tests::larger_can_hold_smaller ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests rectangle

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

确实通过了！我们来添加另一个测试，这次断言较小的矩形无法容纳较大的矩形：

文件名：`src/lib.rs`


```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        // --跳过代码--
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 4,
            height: 9,
        };
        let smaller = Rectangle {
            width: 8,
            height: 3,
        };

        assert! (!smaller.can_hold(&larger));
    }
}
```

由于在此情形下 `can_hold` 函数的正确结果为 `false`，因此我们需要在传递结果给 `assert!` 宏之前对该结果取反。因此，当`can_hold` 返回 `false` 时，我们的测试将通过：

```console
$ RUSTFLAGS="-A warnings" cargo test
   Compiling rectangle v0.1.0 (/home/hector/rust-lang-zh_CN/projects/rectangle)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.07s
     Running unittests src/lib.rs (target/debug/deps/rectangle-a5e83cc30155b35c)

running 2 tests
test tests::larger_can_hold_smaller ... ok
test tests::smaller_cannot_hold_larger ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests rectangle

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

两个测试都通过了！现在我们来看看，当我们在代码中引入一个 bug 后，测试结果会发生什么。我们将修改 `can_hold` 方法的实现，替换其比较宽度时的大于号为小于号：

```rust
// --跳过代码--
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        (self.width < other.width && self.height > other.height)
            || (self.width < other.height && self.height > other.width)
    }
}
```

现在运行测试会产生以下输出：

```console
$ RUSTFLAGS="-A warnings" cargo test
   Compiling rectangle v0.1.0 (/home/hector/rust-lang-zh_CN/projects/rectangle)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.07s
     Running unittests src/lib.rs (target/debug/deps/rectangle-a5e83cc30155b35c)

running 2 tests
test tests::smaller_cannot_hold_larger ... ok
test tests::larger_can_hold_smaller ... FAILED

failures:

---- tests::larger_can_hold_smaller stdout ----

thread 'tests::larger_can_hold_smaller' (141951) panicked at src/lib.rs:29:9:
assertion failed: larger.can_hold(&smaller)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::larger_can_hold_smaller

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`
```

我们的测试就捕获到了这个 bug！由于 `larger.width` 为 `8` 而 `smaller.width` 为 `5`，因此 `can_hold` 中的宽度比较现在返回 `false`: `8` 不小于 `5`。


## 以 `assert_eq!` 与 `assert_ne!` 测试相等性

验证功能的一种常见方式是，测试被测代码的结果与咱们期望代码返回的值之间是否相等。咱们可以使用 `assert!`， 并传递一个使用 `==` 运算符的表达式给他来实现这点。然而，由于这种测试如此常见，标准库提供了一对宏 -- `assert_eq!` 与 `assert_ne!` -- 以便更方便地进行此类测试。这两个宏分别比较两个参数的相等或不相等。当断言失败时，他们还会打印两个值，这让发现测试 *为何* 失败更容易；反之，`assert!` 宏则只会表明他得到了 `==` 表达式的 `false` 值，而不打印导致 `false` 值的值。

在下面清单 11-7 中，我们编写了个名为 `add_two` 的函数，加 2 到其参数，然后我们使用 `assert_eq!` 宏测试这个函数。

<a name="listing_11-7"></a>
文件名：`src/lib.rs`

```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq! (4, add_two(2));
    }
}
```

**清单 11-7**：使用 `assert_eq!` 宏测试函数 `add_two`


我们来检查一下他是否通过！


```console
$ RUSTFLAGS="-A warnings" cargo test
   Compiling adder v0.1.0 (/home/hector/rust-lang-zh_CN/projects/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running unittests src/lib.rs (target/debug/deps/adder-9c63fdd4b3155cad)

running 1 test
test tests::it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

我们创建了个名为 `result` 的变量，保存调用 `add_two(2)` 的结果。然后，我们将 `result` 和 `4` 作为参数传递给 `assert_eq!` 宏。这个测试的输出行是 `tests::it_adds_two ... ok`，文本 `ok` 表明我们的测试通过了！

我们来引入一个 bug 到我们的代码，看看 `assert_eq!` 在失败时会是什么样子。修改 `add_two` 函数的实现为加 `3`：

```rust
pub fn add_two(a: i32) -> i32 {
    a + 3
}
```

再次运行测试：

```console
$ RUSTFLAGS="-A warnings" cargo test
   Compiling adder v0.1.0 (/home/hector/rust-lang-zh_CN/projects/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running unittests src/lib.rs (target/debug/deps/adder-9c63fdd4b3155cad)

running 1 test
test tests::it_adds_two ... FAILED

failures:

---- tests::it_adds_two stdout ----

thread 'tests::it_adds_two' (148898) panicked at src/lib.rs:12:9:
assertion `left == right` failed
  left: 5
 right: 4
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::it_adds_two

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`
```

我们的测试捕获到了这个 bug！`test::it_adds_two` 这个测试失败，消息告诉我们失败的断言是 `left == right` 以及 `left` 与 `right` 值是什么。这条消息帮助我们开始调试：其中我们保存调用 `add_two(2)` 的 `left` 参数为 `5`，而 `right` 参数为 `5`。咱们可以想象，当我们正在进行大量测试时，这将特别有用。

请注意，在某些语言和测试框架中，相等性断言函数的参数称为 `expected` 与 `actual`，并且我们指定参数的顺序很重要。但在 Rust 中，他们称为 `left` 和 `right`，我们指定期望值与代码生成值的顺序并不重要。我们可以将这个测试中的断言写作 `assert_eq! (4, result)`，这将导致显示 ``assertion failed: `(left == right)` `` 的相同失败消息。

当我们给他的两个值不相等时，`assert_ne!` 宏将通过，并在二者相等时失败。在我们不确定某个值 *将* 为何，但我们知道该值绝对 *不应* 为何的情形下，这个宏最有用。例如，当我们正在测试一个肯定会以某种方式改变其输入的函数，而输入修改的方式取决于我们在周几运行测试时，那么要断言的最佳内容可能就是函数的输出不等于与输入。

表象之下，`assert_eq!` 与 `assert_ne!` 两个宏分别使用运算符 `==` 与 `!=`。在断言失败时，这两个宏使用调试格式打印其参数，这意味着被比较的值必须实现 `PartialEq` 与 `Debug` 特质。所有原始类型和大多数标准库类型都实现了这两个特质。对于咱们自己定义的结构体与枚举，咱们将需要实现 `PartialEq` 来断言这些类型的像等性。咱们还需要实现 `Debug` 以在断言失败时打印值。因为这两个特质都是可派生特质，正如第 5 章中 [清单 5-12](../structs/example_program.md#listing_5-12) 中提到的，所以这通常就跟添加 `#[derive(PartialEq, Debug)]` 注解到咱们的结构体或枚举定义一样简单。有关这两个可派生特质及其他可派生特质的更多详细信息，请参阅 [附录 C，派生特质](../appendix/derivable_traits.md)。


## 添加定制失败消息

咱们还可以添加与失败消息一起打印的定制消息，作为 `assert!`、`assert_eq!` 及 `assert_ne!` 宏的可选参数。在必需参数之后指定的所有参数，都会传递到 `format!` 宏（在第 8 章中的 [以 `+` 或 `format!` 连接字符串](../common_collections/strings.md#以--或-format-连接字符串)  中讨论过），因此咱们可以传递一个包含 `{}` 占位符的格式化字符串及要放入这些占位符的值的格式字符串。定制消息对于记录断言的含义很有用；当测试失败时，咱们将更好地了解代码的问题所在。

例如，假设我们有个根据名字向人们打招呼的函数，我们打算测试传入该函数的名字是否出现在输出中：

文件名：`src/lib.rs`

```rust
pub fn greeting(name: &str) -> String {
    format! ("你好，{}！", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Hector");
        assert! (result.contains("Hector"));
    }
}
```

这个程序的需求尚未达成一致，我们相当确定问候语开头的 `你好` 文本将变化。我们决定，不希望在需求改变时必须更新测试，因此我们将仅断言输出包含输入参数的文本，而不是检查与 `greeting` 函数返回的值是否完全相等。

现在，我们来通过修改 `greeting` 为排除 `name` 来引入一个 bug 到这段代码，看看默认的测试失败会是什么样子：

```rust
pub fn greeting(name: &str) -> String {
    String::from("你好！")
}
```

运行这个测试会产生以下输出：

```console
$ RUSTFLAGS="-A warnings" cargo test
   Compiling greeter v0.1.0 (/home/hector/rust-lang-zh_CN/projects/greeter)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.15s
     Running unittests src/lib.rs (target/debug/deps/greeter-a93a22c38250d6d8)

running 1 test
test tests::greeting_contains_name ... FAILED

failures:

---- tests::greeting_contains_name stdout ----

thread 'tests::greeting_contains_name' (164061) panicked at src/lib.rs:12:9:
assertion failed: result.contains("Hector")
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::greeting_contains_name

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`
```

这一结果仅表明断言失败以及断言所在的行。更有用的失败消息将打印来自 `greeting` 函数的值。我们来添加一个由格式字符串组成的定制失败消息，其中占位符以我们从 `greeting` 函数获得的实际值填充：

```rust
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Hector");
        assert! (
            result.contains("Hector"),
            "问候语未包含名字，值为 `{result}`",
        );
    }
```

现在，当我们运行测试时，我们将得到一条信息更丰富的错误消息：

```console
$ RUSTFLAGS="-A warnings" cargo test
   Compiling greeter v0.1.0 (/home/hector/rust-lang-zh_CN/projects/greeter)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running unittests src/lib.rs (target/debug/deps/greeter-a93a22c38250d6d8)

running 1 test
test tests::greeting_contains_name ... FAILED

failures:

---- tests::greeting_contains_name stdout ----

thread 'tests::greeting_contains_name' (166138) panicked at src/lib.rs:12:9:
问候语未包含名字，值为 `你好！`
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::greeting_contains_name

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`
```

我们可以在测试输出中看到实际得到的值，这将帮助我们调试发生的情况，而不是我们原本期望发生的情况。


## 以 `should_panic` 检查终止运行

除了检查返回值外，检查我们的代码是否如我们预期那样处理错误情形也很重要。例如，请考虑在我们在第 9 章 [清单 9-13](../error_handling/panic_or_not.md#listing_9-13) 中创建的 `Guess` 类型。使用 `Guess` 的其他代码仰赖于 `Guess` 实例将仅包含 1 和 100 之间的值的保证。我们可以编写一个测试，确保尝试创建有着该范围之外的值的 `Guess` 实例会终止运行。

我们通过添加属性 `should_panic` 到我们的测试函数做到这点。当函数内的代码终止运行时测试通过；当函数内的代码没有终止运行时，则测试失败。

下面清单 11-8 展示了个测试，检查 `Guess::new` 的错误条件是否会在我们期望的时间发生。


<a name="listing_11-8"></a>
文件名：`src/guessing_game.rs`

```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("猜数值必须在 1 和 100 之间，得到了 {value}。");
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

**清单 11-8**：测试某一情形是否将导致 `panic!`

我们放置 `#[should_panic]` 属性于 `#[test]` 属性之后、在其应用到的函数之前。我们来看看这个测试通过时的结果：


```console
$ RUSTFLAGS="-A warnings" cargo test
   Compiling guessing_game v0.1.0 (/home/hector/rust-lang-zh_CN/projects/guessing_game)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.86s
     Running unittests src/lib.rs (target/debug/deps/guessing_game-05c9c0f5018ae320)

running 1 test
test guessing_game::tests::greater_than_100 - should panic ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/guessing_game-32b26094974d6f68)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests guessing_game

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

看起来不错！现在，我们来通过移出当值大于 100 时 `new` 函数将终止运行的条件，在代码中引入一个 bug：

```rust
// --跳过代码--
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("猜数值必须在 1 和 100 之间，得到了 {value}。");
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
```

当我们运行清单 11-8 中的测试时，他将失败：

```console
$ RUSTFLAGS="-A warnings" cargo test
   Compiling guessing_game v0.1.0 (/home/hector/rust-lang-zh_CN/projects/guessing_game)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running unittests src/lib.rs (target/debug/deps/guessing_game-05c9c0f5018ae320)

running 1 test
test guessing_game::tests::greater_than_100 - should panic ... FAILED

failures:

---- guessing_game::tests::greater_than_100 stdout ----
note: test did not panic as expected at src/guessing_game.rs:25:8

failures:
    guessing_game::tests::greater_than_100

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`
```

在这种情形中，我们没有得到非常有用的消息，但在查看测试函数时，我们看到他被标注了 `#[should_panic]`。我们得到的失败意味着这个测试函数中的代码未引发终止运行。

使用 `should_panic` 的测试可能不够精确。即使测试出于某种不同于我们所预期的原因终止运行，`should_panic` 测试仍将通过。为了使 `should_panic` 测试更加精确，我们可以添加一个可选的 `expected` 参数到 `should_panic` 属性。测试工具将确保失败消息包含了所提供的文本。例如，请考虑下面清单 11-9 中 `Guess` 修改后的代码，其中 `new` 函数会根据该值是太小还是太大，而以不同消息终止运行。

<a name="listing_11-9"></a>
文件名：`src/lib.rs`

```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic! (
                "Guess 值必须大于或等于 1, 得到 {value}。"
            );
        } else if value > 100 {
            panic! (
                "Guess 值必须小于或等于 100, 得到 {value}。"
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "小于或等于 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

**清单 11-9**：以包含指定子字符串的终止运行消息，测试 `panic!`

这个测试将通过，因为我们放在 `should_panic` 属性中的 `expected` 参数中的值，是 `Guess::new` 函数以其终止运行的消息的子字符串。我们本可以指定预期的整个终止消息，在这一情形下将是 `Guess 值必须小于或等于 100，得到 200。`。咱们选择指定的内容取决于终止运行消息中有多少是唯一的或动态的，以及咱们希望测试要有多精确。在这种情况下，终止运行消息的子字符串足以确保测试函数中的代码执行了 `else if value > 100` 的情形。

为了看看当带有 `expected` 消息的 `should_panic` 失败时会发生什么，我们来通过交换 `if value < 1` 与 `else if value > 100` 代码块的主体，引入一个 bug 到我们的代码：

```rust
        if value < 1 {
            panic! (
                "Guess 值必须小于或等于 100, 得到 {value}。"
            );
        } else if value > 100 {
            panic! (
                "Guess 值必须大于或等于 1, 得到 {value}。"
            );
        }
```

这次当我们运行 `should_panic` 测试时，他将失败：

```console
$ RUSTFLAGS="-A warnings" cargo test
   Compiling guessing_game v0.1.0 (/home/hector/rust-lang-zh_CN/projects/guessing_game)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running unittests src/lib.rs (target/debug/deps/guessing_game-05c9c0f5018ae320)

running 1 test
test guessing_game::tests::greater_than_100 - should panic ... FAILED

failures:

---- guessing_game::tests::greater_than_100 stdout ----

thread 'guessing_game::tests::greater_than_100' (183031) panicked at src/guessing_game.rs:12:13:
Guess 值必须大于或等于 1, 得到 200。
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: panic did not contain expected string
      panic message: "Guess 值必须大于或等于 1, 得到 200。"
 expected substring: "小于或等于 100"

failures:
    guessing_game::tests::greater_than_100

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`
```

失败消息表明，这个测试确实如我们预期的那样终止运行了，但终止运行消息并未包含预期的字符串 `小于或等于 100`。在这种情况下，我们真正得到终止运行消息是 `Guess 值必须大于或等于 1, 得到 200。` 。现在我们可以开始找出我们的 bug 在哪里了！


## 在测试中使用 `Result<T, E>`

到目前为止，我们所有的测试都会在失败时终止运行。我们也可以编写使用 `Result<T, E>` 的测试！下面是 [清单 11-1](#listing_11-1) 中的测试，已重写为使用 `Result<T, E>` 并返回 `Err` 而不是终止运行：

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("二加二不等于四"))
        }
    }
}
```

`it_works` 函数现在有了 `Result<T, E>` 的返回类型。在函数体中，我们不再调用 `assert_eq!` 宏，而是在测试通过返回 `Ok(())`，在测试失败时返回内部有着一个 `String` 的 `Err`。

编写返回 `Return<T, E>` 的测试，使咱们可以在测试的主体中使用问号运算符，这会是一种编写测试的便捷方式，当测试中的任何操作返回 `Err` 变种，这些测试都会失败。

咱们不能在哪些使用 `Result<T, E>` 的测试上使用 `#[should_panic]` 注解。为了断言某个操作会返回 `Err` 变种，*请勿* 在 `Result<T, E>` 值上使用问号操作符。相反，要使用 `assert!(value.is_err())`。

现在咱们已经了解编写测试的几种方式，我们来看一下运行测试时会发生什么，并探讨可与 `cargo test` 一起使用的不同选项。


（End）


