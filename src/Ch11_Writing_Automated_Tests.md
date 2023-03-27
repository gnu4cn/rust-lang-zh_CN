# 编写自动化测试

在 Edsgar W. Dijkstra（迪杰斯特拉） 1972 年论文 [《谦卑的程序员（The Humble Programmer）》](https://www.cs.utexas.edu/users/EWD/transcriptions/EWD03xx/EWD340.html)中，迪杰斯特拉指出 “程序测试可以是一种揭示代码存在的非常有效方式，但对于揭示代码错误存在，程序测试又显得不那么足够（Program testing can be a very effective way to show the presence of bugs, but it is hopelessly inadequate for showing their absensce）。” 这并不意味着咱们就不要尽力进行尽可能多的测试！

所谓计算机程序正确，即为所编写代码，在多大程度上完成了想要他完成的事情。Rust 是以高度关注程序正确度而设计的，不过正确度是个复杂的问题，而不易于证明。Rust 的类型系统承担了保证正确性的很大部分，但类型系统并不能捕获到所有东西。由于这方面的原因，Rust 包括了编写自动化软件测试的支持。

这里假设说编写了一个将 `2` 加到所传入任何数字的一个函数 `add_two`。该函数的签名，会接受某个整数作为参数，并返回一个整数作为计算结果。在实现并编译那个函数时，Rust 会完成至此所掌握的全部类型检查与借用检查，来确保比如这里没有传递某个 `String` 值，或传递某个无效引用到该函数。但 Rust *无法* 就该函数将准确完成咱们所想要的操作，即返回参数加 `2`，而非参数加 `10` 或者参数减去 `50` 进行检查！这正是测试发挥作用的地方。

可编写出进行假定的一些测试来，比如，在将 `3` 传递给这个 `add_two` 函数时，返回的值就是 `5`。每当修改了代码时，就都可以运行这些测试，来确保车关系的任何既有正确行为，没有发生变化。

测试是门综合技能：尽管这里无法在一章中，涉及到怎样编写良好测试的方方面面，这里还是会对 Rust 各种测试设施的机制进行讨论。这里会讲到在编写测试时，可用的注解与宏，运行测试的默认动作与选项，以及怎样将一些测试，组织为单元测试与集成测试（unit tests and integration tests）。


## 怎样编写测试

所谓测试，是指一些验证非测试代码（the non-test code）以预期方式发挥作用的函数（tests are Rust functions that verify that the non-test code is functioning in the expected manner）。测试函数的函数体，通常执行以下三种操作：

1. 建立起全部所需的数据或状态；

2. 运行打算测试的代码；

3. 就运行结果是所期望的结果进行断言（assert the results are what you expect）。

下面就来看看，Rust 专为编写进行这些操作的测试，而提供到一些特性，包括 `test` 属性（the `test` attribute）、几个宏，以及 `should_panic` 属性（the `should_panic` attribute）。


### <a id="the-anatomy-of-a-test-function"></a>测试函数剖析

**The Anatomy of a Test Function**

Rust 最简单形态的测试，就是以 `test` 属性注解的一个函数。所谓属性，是指有关 Rust 代码片段的元数据（attributes are metadata about pieces of Rust code）；在第 5 章中，[用在结构体上的 `derive` 属性](Ch05_Using_Structs_to_Structure_Related_Data.md#adding-useful-functionality-with-derived-traits)，就是一个属性的例子。要将某个函数修改为测试函数，就要把 `#[test]` 添加在 `fn` 之前的行上。在以 `cargo test` 命令运行编写的测试时，Rust 就会构建一个运行这些注解过的函数，并就各个测试函数是否通过或失败进行汇报的测试运行器二进制文件（a test runner binary）。

每当用 Cargo 构造了一个新的库项目时，就会自动生成有着一个测试函数的测试模组。该模组给到了编写测试的模板，如此以来，就不必在每次开始新项目时，去找寻确切的测试结构及语法了。而至于要添加多少个额外测试函数与测试模组，则取决于咱们自己！

在对代码进行具体测试之前，这里将通过进行模板测试（the template test）下的试验，来探索测试工作原理的一些方面。随后就会编写一些对之前曾编写的代码进行调用，并就这些代码有着正确行为进行断言的、真实世界中的测试。

先来创建一个名为 `adder`、把两个数字相加的新库项目：

```console
$ cargo new adder --lib                                                                                 lennyp@vm-manjaro
     Created library `adder` package
$ cd adder
```


在 `adder` 库中 `src/lib.rs` 文件的内容，应看起来如清单 11-1 所示。


文件名：`src/lib.rs`

```rust
#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

}
```

*清单 11-1：由 `cargo new` 自动生成的测试模组与函数*


至于现在，就要忽略顶部的两行，并着重于那个函数。请注意那个 `#[test]` 注解：此属性表示这是个测试函数，由此测试运行器就知道将这个函数，当作一个测试对待。在那个 `tests` 模组中，可能也会有一些非测试函数，来帮助建立一些常见场景或执行一些常见操作，因此就需要表明哪些函数是测试。

这个示例函数的函数体，使用了 `assert_eq!` 宏，来对包含了 `2` 加 `2` 的结果 `result` 等于 `4` 进行断言。该断言是作为一个典型测试的格式示例，而提供的。下面就来运行他，来看到该测试会通过。

`cargo test` 命令会运行此项目中的全部测试，如下清单 11-2 所示。

```console
$ cargo test                                                                                  1m 48s lennyp@vm-manjaro
   Compiling adder v0.1.0 (/home/lennyp/rust-lang/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.58s
     Running unittests src/lib.rs (target/debug/deps/adder-3985394b39347736)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

*清单 11-2：运作这个自动生成测试的输出*


Cargo 编译并运行了这个测试。这里看到那行 `running 1 test`。接下来的行就给出了那个自动生成测试函数的名字，名为 `it_works`，以及运行那个测试的结果为 `ok`。整体结论 `test result: ok.` 就表示全部测试都通过了，而后面的 `1 passed; 0 failed` 的部分，则对通过与未通过的测试数据，做了合计。

将某个测试标记为忽略，进而其在特定实例中不运行，是可能的；在本章后面的 ["忽视某些在特别要求下才运行的测试（Ignoring Some Tests Unless Specifically Requested）"](#ignoring-some-tests-unless-specifically-requested) 小节，就会讲到这个问题。由于这里尚未完成这个问题，因此这里的测试总结，就给出了 `0 ignored`。这里还可以把一个参数，传递给这个 `cargo test` 命令，来只测试那些名字与某个字符串匹配的测试；此特性叫做 *过滤（filtering）*，在 [“通过指定测试名字运行测试子集（Running a Subset of Tests）”](#running-a-subset-of-tests) 小节，就会讲到这个问题。而这里也没有对所运行的测试加以过滤，因此在该测试小结的最后，显示了 `0 filtered out`。

其中属于基准测试的 `0 measured` 统计值，对性能进行了测量。所谓基准测试（benchmark tests），就跟其字面意思一样，只在每日构建版的 Rust 中可用。请参阅 [基准测试相关文档](https://doc.rust-lang.org/unstable-book/library-features/test.html) 了解更多信息。

测试输出接下来的部分，是以 `Doc-tests adder` 开始的，在有文档测试时，这便是文档测试的输出。虽然目前尚无文档测试，当 Rust 是可以编译在 API 文档中的全部代码示例的。此特性有助于将文档与代码保持同步！在第 14 章的 [“作为测试的文档注释（Documentation Comments as Tests）”](Ch14_More_about_Cargo_and_Crates_io.md#documentation-comments-as-tests) 小节，就会讨论怎样编写文档测试。至于现在，就会这个 `Doc-tests` 的输出加以忽略。


接下来开始将该测试，定制为咱们自己所需的样子。首先将其中的 `it_works` 函数的名字，修改到某个别的名字，比如 `exploration`，像下面这样：


文件名：`src/lib.rs`


```rust
#[cfg(test)]
mod tests {

    #[test]
    fn exploration() {
        assert_eq! (2 + 2, 4);
    }
}
```

随后再度运行 `cargo test`。其输出此时就给出了 `exploration` 而非 `it_works`：


```console
$ cargo test                                                                                                         lennyp@vm-manjaro
   Compiling adder v0.1.0 (/home/lennyp/rust-lang/adder)
    Finished test [unoptimized + debuginfo] target(s) in 1.64s
     Running unittests src/lib.rs (target/debug/deps/adder-3985394b39347736)

running 1 test
test tests::exploration ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

现在就要添加另一测试，但这次将构造一个会失败的测试！测试是在测试函数中的某个东西发生终止运行时，才失败的。每个测试都是运行在一个新线程中，并在主线程发现某个测试线程死去时，该测试就被标记为失败了。在第 9 章中，就讲到引发代码终止运行的最简单方式，即为调用 `panic!` 这个宏。请敲入一个名为 `another` 函数的新测试，那么这个 `src/lib.rs` 看起来就如同下面清单 11-3 这样。


文件名：`src/lib.rs`

```rust
#[cfg(test)]
mod tests {

    #[test]
    fn exploration() {
        assert_eq! (2 + 2, 4);
    }

    #[test]
    fn another() {
        panic! ("令该测试失败");
    }
}
```

使用 `cargo test` 再度运行这些测试。其输出看起来应如同清单 11-4 那样，显示这里的 `exploration` 测试通过而 `another` 失败了。

```console
$ cargo test                                                                                                         lennyp@vm-manjaro
   Compiling adder v0.1.0 (/home/lennyp/rust-lang/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.42s
     Running unittests src/lib.rs (target/debug/deps/adder-3985394b39347736)

running 2 tests
test tests::exploration ... ok
test tests::another ... FAILED

failures:

---- tests::another stdout ----
thread 'tests::another' panicked at '令该测试失败', src/lib.rs:15:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::another

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--lib'
```

*清单 11-4：在一项测试通过而一项测试失败时的测试输出*

这里不再是 `ok` 了，`test tests::another` 那行给出了 `FAILED`。在这单独结果与测试小结直接，出现了两个新的部分：第一部分显示各个测试失败的具体原因。在此示例中，就得到 `another` 失败详情，是由于该测试函数在 `src/lib.rs` 文件第 15 行处 `panicked at '令该测试失败'`。接下来的部分，则列出了仅所有失败测试的名字，这在有很多测试，进而有很多详细失败测试输出时，是有用的。随后就可以使用某个失败测试的名字，来只运行该项测试而更容易地对其加以调试；在 [“对测试运行方式进行控制（Controlling How Tests Are Run）”](#controlling-how-tests-are-run) 小节，将对运行测试方式，进行深入讲解。


显示在最后的测试小节行：总体上看，这个测试的结果为 `FAILED`。这里有一个测试通过，以及一个测试失败了。

既然现在已经见识了不同场景下测试结果的样子，那么就来看看在测试中，除 `panic!` 之外其他一些有用的宏。


### 以 `assert!` 宏来对测试结果进行检查

这个由标准库提供的 `assert!` 宏，在想要确保测试中某些情形求值为 `true` 时，是有用的。要给到这个 `assert!` 宏，一个求值为布尔值的参数。在求得的值为 `true` 时，就什么也不会发生，同时该测试通过。而在求得的值为 `false` 时，那么这个 `assert!` 宏就会调用 `panic!` 来造成该测试失败。使用这个 `assert!` 宏，有助于检查所编写代码，是以所计划方式运作。

在第 5 章的清单 5-15 中，用到了一个 `Rectangle` 结构体，以及一个 `can_hold` 方法，下面清单 11-5 中重复了那段代码。下面就将这段代码放在 `src/lib.rs` 文件，随后就要使用 `assert!` 宏为其编写一些测试。

文件名：`src/lib.rs`

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        (self.width > other.width && self.height > other.height) || (self.width > other.height && self.height > other.width)
    }
}
```

*清单 11-5：使用第 5 章的 `Rectangle` 结构体及其 `can_hold` 方法*

这个 `can_hold` 方法返回的是个布尔值，这就表示他是个 `assert!` 宏的绝佳用例。在下面清单 11-6 中，这里经由创建一个有着宽为 `8` 高为 `7` 的 `Rectangle` 实例，并断言其可装下另一个宽为 `5` 高为 `1` 的 `Rectangle` 实例，而编写了一个对该 `can_hold` 方法进行检查的测试。

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

*清单 11-6：`can_hold` 的一个检查较大矩形是否能够真正包含较小矩形的测试*

请注意这里在 `tests` 模组里头添加了个新行：`use super::*;`。这个 `tests` 模组是个遵循第 7 章中，[“用于指向模组树中某个项目的路径”](Ch07_Managing_Growing_Projects_with_Packages_Crates_and_Modules.md#paths-for-referring-to-an-item-in-the-module-tree)小节中曾讲到一般可见性规则的常规模组。由于这个 `tests` 模组是个内部模组，因此这里就需要将外层模组中的受测试代码，带入到这个 `tests` 内部模组的作用域。而由于这里使用了一个全局通配符（a glob, `*`），因此所有在外层模组中定义的内容，就对这个 `tests` 模组可用了。

这里已将这个测试命名为了 `larger_can_hold_smaller`，并创建除了所需的两个 `Rectanble` 实例。随后就调用了 `assert!` 宏，并将调用 `larger.can_hold(&smaller)` 的结果传递给了他。这个表达式应返回 `true`，因此这个测试将通过。那么就来试试看吧！

```console
$ cargo test                                                                                                            lennyp@vm-manjaro
   Compiling assert_demo v0.1.0 (/home/lennyp/rust-lang/assert_demo)
    Finished test [unoptimized + debuginfo] target(s) in 0.37s
     Running unittests src/lib.rs (target/debug/deps/assert_demo-504fa58455de23e3)

running 1 test
test tests::larger_can_hold_smaller ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests assert_demo

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

``` 

这个测试真的通过了！接下来添加另一个测试，这次就断言某个较小矩形，无法装下一个较大矩形：

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

由于此情形下的 `can_hold` 正确结果为 `false`，因此就需要在将该结果传递给 `assert!` 宏之前，对其取反。而作为测试结果，在 `can_hold` 返回 `false` 时，这个测试就会通过：

```console
$ cargo test                                                                                                            lennyp@vm-manjaro
   Compiling assert_demo v0.1.0 (/home/lennyp/rust-lang/assert_demo)
    Finished test [unoptimized + debuginfo] target(s) in 0.37s
     Running unittests src/lib.rs (target/debug/deps/assert_demo-504fa58455de23e3)

running 2 tests
test tests::smaller_cannot_hold_larger ... ok
test tests::larger_can_hold_smaller ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests assert_demo

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

两个测试均通过了！现在来看看在将一个代码错误（a bug）引入这里的代码时，这里的测试结果将发生什么。这里会通过在比较两个矩形宽时，将大于符号替换为小于符号，而对 `can_hold` 方法的实现加以修改：

```rust
// --跳过代码--
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        (self.width < other.width && self.height > other.height) || 
            (self.width < other.height && self.height > other.width)
    }
}
```

现在运行这些测试，就会生成下面的输出：

```console
$ cargo test                                                                           lennyp@vm-manjaro
   Compiling assert_demo v0.1.0 (/home/lennyp/rust-lang/assert_demo)
    Finished test [unoptimized + debuginfo] target(s) in 0.37s
     Running unittests src/lib.rs (target/debug/deps/assert_demo-504fa58455de23e3)

running 2 tests
test tests::larger_can_hold_smaller ... FAILED
test tests::smaller_cannot_hold_larger ... ok

failures:

---- tests::larger_can_hold_smaller stdout ----
thread 'tests::larger_can_hold_smaller' panicked at 'assertion failed: larger.can_hold(&smaller)', src/lib.rs:29:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::larger_can_hold_smaller

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--lib'
```

这些测试就捕获到了代码错误（the bug）！由于 `larger.width` 为 `8` 而 `smaller.width` 为 `5`，那么在 `can_hold` 方法中宽的比较现在就会返回 `false`: `8` 不比 `5` 小。


### 使用 `assert_eq!` 与 `assert_ne!` 两个宏测试是否相等

对功能进行验证的一种常见方式，便是对测试之前代码的输出结果，与所期望的代码返回值之间是否相等进行测试。使用 `assert!` 宏并将一个使用了 `==` 运算符的表达式传递给他，就可完成这样的测试。然而由于这是一个如此常见的测试，以致标准库提供了一对宏 -- `assert_eq!` 与 `assert_ne!` -- 来更方便地执行这样的测试。这两个宏分别比较两个参数的相等与不相等。在断言失败时，他们还会打印出那两个值，这就令到发现 *为何* 测试失败，更为容易了；与之相反，`assert!` 宏则只表明他收到了那个 `==` 表达式的 `false` 值，而没有将导致那个 `false` 值的两个值打印出来的功能。

在下面清单 11-7 中，就编写了一个名为 `add_two`、将 `2` 加到其参数的函数，随后使用 `asset_eq!` 宏对这个函数进行了测试。

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

*清单 11-7：使用 `assert_eq!` 宏对函数 `add_two` 进行测试*


下面就来看看，他通过了测试！


```console
$ cargo test                                                        lennyp@vm-manjaro
   Compiling assert_demo v0.1.0 (/home/lennyp/rust-lang/assert_demo)
    Finished test [unoptimized + debuginfo] target(s) in 0.56s
     Running unittests src/lib.rs (target/debug/deps/assert_demo-504fa58455de23e3)

running 1 test
test tests::it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests assert_demo

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

这里将 `4` 作为参数传递给了 `assert_eq!`，这与调用 `add_two(2)` 的结果相等。该测试的那一行就是 `tests::it_adds_two ... ok`，而文本 `ok` 就表明这个测试通过了！

接下来将一个 bug 引入到这里的代码，看看在 `assert_eq!` 失败时，会是什么样子。将这个 `add_two` 函数的实现修改为加 `3`：

```rust
pub fn add_two(a: i32) -> i32 {
    a + 3
}
```

在此运行这些测试（the tests）：

```console
$ cargo test                                                           lennyp@vm-manjaro
   Compiling assert_demo v0.1.0 (/home/lennyp/rust-lang/assert_demo)
    Finished test [unoptimized + debuginfo] target(s) in 0.54s
     Running unittests src/lib.rs (target/debug/deps/assert_demo-504fa58455de23e3)

running 1 test
test tests::it_adds_two ... FAILED

failures:

---- tests::it_adds_two stdout ----
thread 'tests::it_adds_two' panicked at 'assertion failed: `(left == right)`
  left: `4`,
 right: `5`', src/lib.rs:11:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::it_adds_two

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--lib'
```

这里的测试捕获到了那个 bug！其中的 `it_adds_two` 测试就失败了，同时这些消息讲到，失败的断言为 ``assert failed: `(left == right)` ``，以及其中 `left` 与 `right` 的值分别为何。该消息有助于发起调试：那个 `left` 参数为 `4`，而那个 `right` 参数，即放上 `add_two(2)` 的那个，为 `5`。那么这里就可以联想到，当有很多测试在进行时，这一点就会尤其有帮助了。

请注意在某些语言与测试框架中，相等断言函数的那两个参数，分别叫做 `expected` 与 `actual`，且指定这两个参数的顺序是至关重要的。不过在 Rust 中，他们则分别叫做 `left` 与 `right`，且在指定所期望值与代码产生值的顺序，并不重要。这里可将该断言写作 `assert_eq! (add_two(2), 4)`，这仍会导致这个显示出 ``assertion failed: `(left == right)` `` 的同样失败消息。

而 `assert_ne!` 宏则将在给到其两个不相等值时通过测试，在两个值相等时测试失败。对于在不确定某个值是什么，但却清楚该值明显不会为何时的各种情形，这个宏就是最有用的。比如，在对某个确切会以某种方式修改其输入的函数进行测试，而修改方式会根据具体每周的哪一天运行该测试发生改变时，那么加以断言的最佳事物，就会是该函数的输出，与其输入不相等。

表象之下，`assert_eq!` 与 `assert_ne!` 两个宏，分别使用了运算符 `==` 与 `!=`。在他们的断言失败时，这两个宏就会使用调试格式化（debug formatting），将他们的参数打印出来，这就意味着正被比较的两个值，必须实现了 `PartialEq` 与 `Debug` 特质。全部原生值与绝大多数的标准库类型，都实现了这两个特质。而对于咱们自己定义的结构体与枚举，就需要实现 `PartialEq` 来对这些类型的相等与否进行断言。同样还需要实现 `Debug`，来在断言失败时打印比较的两个值。由于这两个特质都正如第 5 章清单 5-12 中所提到的派生特质（derivable traits），这样就跟将 `#[derive(PartialEq, Debug)]` 注解，添加到所编写的结构体或枚举定义一样直接了。请参阅附录 C，[“可派生特质（derivable traits）”](Ch21_Appdendix.md#c-derivable-traits) 了解更多有关这两个及其他派生特质的详细信息。

### 加入定制失败消息

**Adding Custom Failure Message**

还可将与失败消息一同打印的定制消息，作为 `assert!`、`assert_eq!` 及 `assert_ne!` 宏的可选参数加入进来。在必须的两个参数之后指定的全部参数，都被传递给他们中的 `format!` 宏（第 8 章中 [“以 `+` 操作符或 `format!` 宏的字符串连接（Concatenation with the `+` Operator or the `format!` macro）”](Ch08_Common_Collections.md#concatenation-with-the-plus-operator-or-the-format-macro)） 小节曾讲到），因此就可以传递一个包含了 `{}` 占位符的格式化字符串，以及进到这些占位符的值。对于给某个断言表示什么的文档编制，这些定制消息就是有用的；在某个测试失败时，就会有着该代码下那个问题的较好理解。

比如说，这里有个按照名字来打招呼的函数，并打算就传入到该函数的名字有出现在输出中进行测试：

文件名：`src/lib.rs`

```rust
pub fn greeting(name: &str) -> String {
    format! ("你好，{}", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Lenny");
        assert! (result.contains("Lenny"));
    }
}
```

该程序的各项要求尚未达成一致，同时这里十分肯定问候开始处的 `你好` 文字将会改变。这里已经确定不打算在各项要求改变时，必定要对这个测试加以更新，因此这里将只就输出包含输出参数的文本进行断言，而非对自 `greeting` 函数返回的值，进行精确的相等检查。

下面就来通过把 `greeting` 修改未排除 `name`，而将一个 bug 引入到这段代码，来看看这个默认测试失败的样子：

```rust
pub fn greeting(name: &str) -> String {
    String::from("你好！")
}
```

运行这个测试，就会产生以下输出：

```console
$ cargo test                                                                      lennyp@vm-manjaro
   Compiling assert_demo v0.1.0 (/home/lennyp/rust-lang/assert_demo)
    Finished test [unoptimized + debuginfo] target(s) in 0.48s
     Running unittests src/lib.rs (target/debug/deps/assert_demo-504fa58455de23e3)

running 1 test
test tests::greeting_contains_name ... FAILED

failures:

---- tests::greeting_contains_name stdout ----
thread 'tests::greeting_contains_name' panicked at 'assertion failed: result.contains(\"Lenny\")', src/lib.rs:12:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::greeting_contains_name

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--lib'
```

这样的结果，正好表明了该断言失败了，以及这个失败断言所在的行。而更有用的失败消息，应会打印出那个 `greeting` 函数的值来。下面就来添加一个，由带有以获取自 `greeting` 函数的具体值所填充的占位符的格式字符串，所构成的定制失败消息：

```rust
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Lenny");
        assert! (
            result.contains("Lenny"),
            "问候语未包含名字，问候语的值为 `{}`",
            result
        );
    }
```

现在运行这个测试，就会得到内容更为的错误消息：

```console
$ cargo test                                                                      lennyp@vm-manjaro
   Compiling assert_demo v0.1.0 (/home/lennyp/rust-lang/assert_demo)
    Finished test [unoptimized + debuginfo] target(s) in 0.42s
     Running unittests src/lib.rs (target/debug/deps/assert_demo-504fa58455de23e3)

running 1 test
test tests::greeting_contains_name ... FAILED

failures:

---- tests::greeting_contains_name stdout ----
thread 'tests::greeting_contains_name' panicked at '问候语未包含名字，问候语的值为 `你好！`', src/lib.rs:12:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::greeting_contains_name

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--lib'
```

现在就可以在测试输出中，看到具体得到的值了，这将有助于对发生的事情，而非期望发生的事情进行调试，有所帮助（we can see the value we actually got in the test output, which would help us debug what happened instead of what we were expecting to happen）。


### 使用 `should_panic` 对运行中止进行检查

**Checking for Panics with `should_panic`**

除了检查返回值外，重要的是检查所编写代码有如预期那样，对各种错误情形进行处理。比如，请考虑在第 9 章清单 9-13 中所创建的那个 `Guess` 类型。使用了 `Guess` 的其他代码，就仰赖于 `Guess` 实例，将包含仅在 `1` 与 `100` 之间的值这一保证。这里就可以编写一个，确保在尝试创建带有那个范围之外值的 `Guess` 实例时，会中止运行的测试。

这里是通过将属性 `should_panic` 添加到此处的测试函数，来完成这一点的。在函数内部代码中止运行时，该测试便会通过；若函数中代码没有中止运行，那么该测试就会失败。

下面清单 11-8，就给出了一个在预期 `Guess::new` 的各种错误情形发生时，对这些错误情形进行检查的测试。

文件名：`src/lib/rs`

```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic! ("Guess 值必须在 1 与 100 之间，得到的是 {}。", value);
        }

        Guess { value }
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

*清单 11：就某个将引发 `panic!` 的情形进行测试*

这里将那个 `#[should_panic]` 属性，放在了 `#[test]` 属性之后，且在其应用到的函数之前。下面来看看在该测试通过时的样子：


```console
$ cargo test                                                                       lennyp@vm-manjaro
   Compiling assert_demo v0.1.0 (/home/lennyp/rust-lang/assert_demo)
    Finished test [unoptimized + debuginfo] target(s) in 0.64s
     Running unittests src/lib.rs (target/debug/deps/assert_demo-504fa58455de23e3)

running 1 test
test tests::greater_than_100 - should panic ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests assert_demo

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

看起来不错！现在就来通过移出当其中的值大于 `100` 时，这个 `new` 函数将中止运行的条件，而将一个 bug 引入到这里的代码：

```rust
// --跳过代码--
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic! ("Guess 值必须在 1 与 100 之间，得到的是 {}。", value);
        }

        Guess { value }
    }
}
```

此时在运行清单 11-8 中的测试，他就会失败了：

```console
$ cargo test                                                                       lennyp@vm-manjaro
   Compiling assert_demo v0.1.0 (/home/lennyp/rust-lang/assert_demo)
    Finished test [unoptimized + debuginfo] target(s) in 0.42s
     Running unittests src/lib.rs (target/debug/deps/assert_demo-504fa58455de23e3)

running 1 test
test tests::greater_than_100 - should panic ... FAILED

failures:

---- tests::greater_than_100 stdout ----
note: test did not panic as expected

failures:
    tests::greater_than_100

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--lib'

```

在这个示例中，并未获得非常有用的消息，不过在查看那个测试函数时，就会看到其被 `#[should_panic]` 给注解过。这里收到了失败，就表示在这个测试函数中的代码，并未引发运行中止。

用到 `should_panic` 的测试，可并不那么精确。即便在该测试由于某个不同于咱们所预期的原因而中止运行了，这个 `should_panic` 测试仍会通过。要令到 `should_panic` 测试更加精确，则可以将某个可选的 `expected` 参数，传递给那个 `should_panic` 属性。这种测试工具，将确保失败消息包含了所提供的文本（the test harneess will make sure that the failure message contains the provided text）。比如，请考虑下面清单 11-9 中修改过的 `Guess` 代码，其中 `new` 函数会根据该值是否过小或过大，而以不同消息中止运行。

文件名：`src/lib.rs`

```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic! (
                "Guess 值必须大于或等于 1, 得到的是 {}。", 
                value
            );
        } else if value > 100 {
            panic! (
                "Guess 值必须小于或等于 100, 得到的是 {}。", 
                value
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

*清单 11-9：对有着包含指定 _子字符串_ 的中止运行消息，的某个 `panic!` 进行测试*

由于这里放在那个 `should_panic` 属性的 `expected` 参数中的值，正是其中 `Guess::new` 函数中止运行消息的一个子字符串，因此这个测试将通过。这里本可将所预期的整个中止运行消息给指定出来，在此示例中即为 `Guess 值必须小于或等于 100，得到的是 200。` 选择指明什么，是根据中止运行消息，具有何种程度的独特性或动态变化，以及打算要整个测试具有何种级别的准确度。在此示例中，那个中止运行消息的某个子字符串，就足够用于确保该测试函数中代码，执行了 `else if value > 100` 的条件。

为看到在某个 `should_panic` 以一个 `expected` 消息失败时，会发生什么，下面就来通过调换 `if value < 1` 与 `else if value > 100` 代码块的代码体，而引入一个 bug 到这里的代码中：

```rust
        if value < 1 {
            panic! (
                "Guess 值必须小于或等于 100, 得到的是 {}。", 
                value
            );
        } else if value > 100 {
            panic! (
                "Guess 值必须大于或等于 1, 得到的是 {}。", 
                value
            );
        }
```

这次在运行这个 `should_panic` 测试时，便会失败了：

```console
$ cargo test                                                                       lennyp@vm-manjaro
   Compiling assert_demo v0.1.0 (/home/lennyp/rust-lang/assert_demo)
    Finished test [unoptimized + debuginfo] target(s) in 0.41s
     Running unittests src/lib.rs (target/debug/deps/assert_demo-504fa58455de23e3)

running 1 test
test tests::greater_than_100 - should panic ... FAILED

failures:

---- tests::greater_than_100 stdout ----
thread 'tests::greater_than_100' panicked at 'Guess 值必须大于或等于 1, 得到的是 200。', src/lib.rs:13:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: panic did not contain expected string
      panic message: `"Guess 值必须大于或等于 1, 得到的是 200。"`,
 expected substring: `"小于或等于 100"`

failures:
    tests::greater_than_100

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--lib'
```

这样的失败消息就表示，这个测试确实如预期那样中止运行了，但中止运行消息并未包含预期的字符串 `小于或等于 100`。在此示例中，真正得到中止运行消息，为 `Guess 值必须大于或等于 1, 得到的是 200。` 现在就可以开始找出，这里的 bug 在哪了！


### 在测试中使用 `Result<T, E>`

到目前为止，这里全部的测试在失败时，都会中止运行。这里通用可以编写用到 `Result<T, E>` 的测试！下面就是清单 11-1 的那个测试，只是被重写为了使用 `Result<T, E>`，并返回一个 `Err` 而非中止运行：

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

这个 `it_works` 函数现在有了 `Result<T, E>` 的返回值类型。而在该函数的函数体中，此时在那个 `if` 测试通过时，返回了 `Ok(())`，在测试失败时返回一个带有 `String` 的 `Err`，而不再调用那个 `assert_eq!` 宏了。


编写这样的返回某个 `Return<T, E>` 的测试，就令到在各个测试的函数体中，使用问号运算符（the question mark operator, `?`）可行了，而在测试函数体中使用 `?`，则可以是编写那些，在其内部返回某个 `Err` 变种时将会失败测试的便利方式。

在那些用到 `Result<T, E>` 的测试上，是不可以使用 `#[should_panic]` 注解的。而要断言某个操作返回的是一个`Result<T, E>` 枚举的 `Err` 变种，就不要在返回的 `Result<T, E>` 值上，使用问号操作符。相反，要使用 `assert!(value.is_err())` 这种方式。


既然咱们已经了解了编写测试的几种方式，那么就来看一下，在运行这些编写的测试时会发生什么，并探索一下可与 `cargo test` 一起使用的不同选项。


## 控制测试以何种方式运行

就跟 `cargo run` 会编译代码并于随后运行得出的二进制程序一样，`cargo test` 也会以测试模式编译所编写的代码，并会运行得到的测试二进制程序。而由 `cargo test` 产生出的二进制程序默认行为，即是以并行方式运行全部测试，并在测试运行期间捕获输出，阻止输出被显示出来以及令到与测试结果相关的输出，更加易于阅读（the default behavior of the binary produced by `cargo test` is to run all the tests in parallel and capture output generated during test runs, preventing the output from being displayed and making it easier to read the output related to the test results）。不过，这里是可以指定一些命令行选项，来改变这种默认行为的。

一些命令行选项是介入到 `cargo test`，而一些则是介入所得到的测试二进制程序。在介入到 `cargo test` 的命令行参数之后，跟上分隔符 `--`，随后才是那些进到测试二进制程序的参数，以这样的方式把这两种类型的命令行参数区分开。运行 `cargo test --help`，就会显示出可在 `cargo test` 下使用的选项，而运行 `cargo test -- --help` 则会显示出可在分隔符之后使用的那些选项。

### 并行还是连续地运行测试

**Running Tests in Parallel or Consecutively**

在运行多个测试时，这些测试默认使用线程以并行方式运行，意味着他们会运行得更快，而咱们也会迅速地得到反馈。由于这些测试是在同时运行的，因此就必须确保所编写的测试不会各自依赖，并依赖于任何共用的状态，包括某种共用环境，诸如当前工作目录或环境变量。

比如说，所编写的每个测试，都会运行一些在磁盘上创建名为 `test-output.txt` 的文件，并将某些数据写到那个文件的代码。随后各个测试就会读取那个文件中的数据，并就那个包含了某个特定值进行断言，这个断言的特定值在各个测试中是不同的。由于这些测试是在同一时间运行，某个测试就可能会在另一测试写入与读取这个文件期间，对该文件进行覆写。那么第二个测试随后就将并非由于代码不正确，而因为这些测试在并行运行期间，相互之间造成了影响而失败。一种解决办法，是确保各个测试写入到不同文件；另一种办法，就是以一次运行一个的方式，运行这些测试。

在不打算并行运行这些测试，或要对所用到线程数有更细粒度掌控时，就可以将 `--test-threads` 这个命令行标志，与打算使用的线程数目，发送给那个测试二进制程序。请看看下面这个示例：

```console
$ cargo test -- --test-threads=1
```

这里把测试线程数设置为了 `1`，这就告诉了该程序不要使用任何并行机制。使用一个线程运行这些测试，相比以并行方式运行他们，将耗费更长时间，但在这些测试共用了状态时，他们之间不会相互影响。


### 展示函数的输出

默认情况下，在某个测试通过时，Rust 的测试库会对任何打印到标准输出的内容加以捕获。比如，当在某个测试中调用 `println!` 且该测试通过时，就不会在终端中看到那个 `println!` 的输出；而只将看到表示该测试通过的那行。而在某个测试失败时，则会与失败消息的其余部分一起，看到任何打印到标准输出的内容，。

作为一个示例，下面清单 11-10 有着一个打印其参数值并返回 `10` 的弱智函数，以及一个会通过的测试与一个会失败的测试。

文件名：`src/lib.rs`

```rust
fn prints_and_returns_10(a: i32) -> i32 {
    println! ("我得到了一个值 {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq! (10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq! (5, value);
    }
}
```

*清单 11-10：对一个调用了 `println!` 宏的函数的两个测试*

在以 `cargo test` 运行这两个测试时，就会看到以下的输出：

```console
$ cargo test                                                                     lennyp@vm-manjaro
   Compiling assert_demo v0.1.0 (/home/lennyp/rust-lang/assert_demo)
    Finished test [unoptimized + debuginfo] target(s) in 0.38s
     Running unittests src/lib.rs (target/debug/deps/assert_demo-504fa58455de23e3)

running 2 tests
test tests::this_test_will_pass ... ok
test tests::this_test_will_fail ... FAILED

failures:

---- tests::this_test_will_fail stdout ----
我得到了一个值 8
thread 'tests::this_test_will_fail' panicked at 'assertion failed: `(left == right)`
  left: `5`,
 right: `10`', src/lib.rs:19:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::this_test_will_fail

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--lib'
```

请留意此输出中没有在哪里看到 `我得到了一个值 4`，这正是在通过的那个测试运行时所打印出的内容。那个输出就已被捕获了。而来自失败了的那个测试的输出，`我得到了一个值 8`，出现在了该测试的总结输出小节中，这个测试总结输出小节，还给出了该测试失败的原因。

在想要同样看到已通过测试的那些打印值时，就可以使用 `--show-output` 命令行开关，告诉 Rust 还要显示成功测试的输出。


```console
$ cargo test -- --show-output
```

在使用 `--show-output` 命令行开关再次运行清单 11-10 中的那些测试时，就会看到下面的输出：

```console
$ cargo test -- --show-output                                                       lennyp@vm-manjaro
   Compiling assert_demo v0.1.0 (/home/lennyp/rust-lang/assert_demo)
    Finished test [unoptimized + debuginfo] target(s) in 0.41s
     Running unittests src/lib.rs (target/debug/deps/assert_demo-504fa58455de23e3)

running 2 tests
test tests::this_test_will_fail ... FAILED
test tests::this_test_will_pass ... ok

successes:

---- tests::this_test_will_pass stdout ----
我得到了一个值 4


successes:
    tests::this_test_will_pass

failures:

---- tests::this_test_will_fail stdout ----
我得到了一个值 8
thread 'tests::this_test_will_fail' panicked at 'assertion failed: `(left == right)`
  left: `5`,
 right: `10`', src/lib.rs:19:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::this_test_will_fail

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--lib'
```

### 依据测试名称来运行测试的某个子集

**Running a Subset of Tests by Name**

在有的时候，运行一整个的测试套件可能要用很长时间。而当在某个特定方面编写代码时，就会想要只运行与正在编写代码有关的那些测试。通过将想要运行的某个或某些测试的名字，作为参数传递给 `cargo test`，就可以对想要运行哪些测试加以选择。

为了演示怎样运行测试子集，这里将首先为所编写的 `add_two` 函数，创建三个测试，如下清单 11-11 中所示，并会选择要运行哪些测试。

文件名：`src/lib.rs`

```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq! (4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq! (5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq! (102, add_two(100));
    }
}
```

*清单 11-11：有着不同名字的三个测试*

如同早先所看到的那样，在不带传递任何参数运行这些测试时，全部这些测试将以并行方式运行：

```console
$ cargo test                                                                     lennyp@vm-manjaro
   Compiling assert_demo v0.1.0 (/home/lennyp/rust-lang/assert_demo)
    Finished test [unoptimized + debuginfo] target(s) in 0.43s
     Running unittests src/lib.rs (target/debug/deps/assert_demo-504fa58455de23e3)

running 3 tests
test tests::add_three_and_two ... ok
test tests::add_two_and_two ... ok
test tests::one_hundred ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests assert_demo

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

**运行单个的测试**

可将任何测试函数的名字，传递给 `cargo test` 来只运行那个测试：


```console
$ cargo test one_hundred                                                         lennyp@vm-manjaro
   Compiling assert_demo v0.1.0 (/home/lennyp/rust-lang/assert_demo)
    Finished test [unoptimized + debuginfo] target(s) in 0.37s
     Running unittests src/lib.rs (target/debug/deps/assert_demo-504fa58455de23e3)

running 1 test
test tests::one_hundred ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

```

上面就只有那个名字为 `one_hundred` 的测试运行了；其他两个测试并不与那个指定的名字匹配。而这个测试输出，通过末尾出显示出的 `2 filtered out`，而让咱们获悉有更多测试并未运行。

以这种方式是没法指定多个测试的名字的；只有给到 `cargo test` 的第一个值，才会被用到。不过是有方法来运行多个测试的。


**使用过滤来运行多个测试**

这里可指定某个测试函数名字的一部分，那么名字与所指定值匹配的全部测试，就都会被运行。比如，由于上面的那些测试中有两个测试的名字包含了 `add`，因此这里就可以通过运行 `cargo test add`，运行这两个测试：

```console
$ cargo test add                                                                    lennyp@vm-manjaro
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/assert_demo-9c28057969510af5)

running 2 tests
test tests::add_three_and_two ... ok
test tests::add_two_and_two ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

```

此命令运行了名字中有 `add` 字样的全部测试，并将那个名为 `one_hundred` 的测试给过滤掉了。还要留意到，测试所出现在的模组，成为了该测试名字的一部分，因此就可通过以模组名字来过滤，而运行某个模组中的全部测试。


### 在未作特别要求时忽略某些测试

**Ignoring Some Tests Unless Specifically Requested**


有的时候少数几个特定测试，执行起来可能非常耗费时间，那么就会打算在绝大多数 `cargo test` 运行期间，将这些测试排除掉。与将全部想要运行的测试列为参数不同，这里是可以将那些耗费时间的测试，使用 `ignore` 属性进行注解，而将他们排除，如下所示：

文件名：`src/lib.rs`

```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn nth_fibonacci(n: u64) -> u64 {

    if n == 0 || n == 1 { 
        return n; 
    } else { 
        return nth_fibonacci(n - 1) + nth_fibonacci(n - 2); 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq! (4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq! (5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq! (102, add_two(100));
    }

    #[test]
    fn it_works() {
        assert_eq! (2 + 2, 4);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        assert_ne! (100, nth_fibonacci(50));
    }
}
```

这里在 `#[test]` 之后，把那行 `#[ignore]` 添加到了打算排除的那个测试之上。此时再运行这些测试时，原来的三个测试会运行，但 `expensive_test` 就不会运行：

```console
$ cargo test                                                                                 lennyp@vm-manjaro
   Compiling assert_demo v0.1.0 (/home/lennyp/rust-lang/assert_demo)
    Finished test [unoptimized + debuginfo] target(s) in 0.46s
     Running unittests src/lib.rs (target/debug/deps/assert_demo-9c28057969510af5)

running 5 tests
test tests::expensive_test ... ignored
test tests::add_two_and_two ... ok
test tests::one_hundred ... ok
test tests::it_works ... ok
test tests::add_three_and_two ... ok

test result: ok. 4 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests assert_demo

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

那个 `expensive_test` 函数就被列为了 `ignored`。而再打算只运行那些忽略的测试时，则可以使用 `cargo test -- --ignored`：

```console
$ cargo test -- --ignored                                                            lennyp@vm-manjaro
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/assert_demo-9c28057969510af5)

running 1 test
test tests::expensive_test has been running for over 60 seconds
test tests::expensive_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 124.65s

   Doc-tests assert_demo

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

经由控制哪些测试运行，就可以确保 `cargo test` 快速得出结果。在对那些 `ignored` 测试结果进行检查是有意义的，且有时间等待他们的结果出来时，那么就可以运行 `cargo test -- --ignored`。在打算运行全部测试，而不管他们有没有被注解为 `ignored`，那么就可以运行 `cargo test -- --include-ignored`。


```console
$ cargo test -- --include-ignored                                                    lennyp@vm-manjaro
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/assert_demo-9c28057969510af5)

running 5 tests
test tests::add_two_and_two ... ok
test tests::add_three_and_two ... ok
test tests::it_works ... ok
test tests::one_hundred ... ok
test tests::expensive_test has been running for over 60 seconds
test tests::expensive_test ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 130.68s

   Doc-tests assert_demo

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```


## 测试的组织

如同在本章开头提到的，测试是门复杂的学问，而不同人群会使用不同术语及组织方式（testing is a complex discipline, and different people use different terminology and organization）。Rust 社群认为，测试是由两个大类组成：单元测试与集成测试（unit tests and integration tests）。*单元测试* 是一些小而更为专注的测试，他们一次单独测试一个模组，并可对私有接口进行测试（*unit tests* are small and more focused, testing one module in isolation at a time, and can test private interfaces）。*集成测试* 则是完全在所编写库外部进行，并会像其他外部代码那样，对咱们的代码加以使用，因此就只会对公开接口进行使用，且潜在每个测试会检查多个模组。


这两种类型的测试编写，对于确保代码库的各个部分有在单独及共同完成所预期的事项，都是重要的。


### 单元测试

单元测试的目的，是要将各个代码单元孤立于其余代码进行测试，从而快速定位出何处代码有如预期那样工作，以及何处代码未如预期那样工作。应将单元测试，放在 `src` 目录之下，在那些有着他们要测试代码的各个文件中。约定即为要在各个文件中，创建包含那些测试函数的一个名为 `tests` 的模组，并使用 `cfg(test)` 对该模组加以注解。


**测试模组与 `#[cfg(test)]`**

在测试模组上的那个 `#[cfg(test)]` 注解，告诉 Rust 仅在运行 `cargo test`，而非运行 `cargo build`时，才编译和运行测试代码。在只打算构建该库时，这样就节省了编译时间，并由于在得到的已编译工件中不会包含测试，而在其中节省了空间。后面就会看到，由于集成测试会进到不同目录，他们就不需要这个 `#[cfg(test)]` 注解。不过由于单元测试是在与代码同样的文件中，因此就会使用 `#[cfg(test)]`，来指明他们不应被包含在编译结果中。

回顾在本章第一小节中，当那里生成那个新的 `adder` 项目时，Cargo 就为咱们生成了下面的代码：


```console
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
```

这段代码就是自动生成的测试模组。其中的属性 `cfg` 是指 *配置（configuration）*，而告诉 Rust 接下来的项目只应在给定的某个配置选项下才被包含。在此示例中，那个配置选项便是 `test`，Rust 提供的这个配置选项，用于测试的编译及运行。通过使用这个 `cfg` 属性，Cargo 就会只在咱们以 `cargo test`，明确表示要运行这些测试时，才对这里的测试代码进行编译。而这些测试代码，包含了可能位于此模组内部的全部辅助函数，以及使用 `#[test]` 注解过的那些函数。


**私有函数的测试（Testing Private Functions）**

在测试社区，有着私有函数是否应被直接测试的争论，而别的语言让对私有函数的测试，成为困难或不可行的事情。不论所才行的是何种测试理念，Rust 的私有规则，真的实现了对私有函数的测试。请考虑下面清单中，有着私有函数 `internal_adder` 的代码。

文件名：`src/lib.rs`

```rust
pub fn add_two(a: i32) -> i32 {
    internal_add(a, 2)
}

fn internal_add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq! (4, internal_add(2, 2));
    }
}
```

*清单 11-12：对私有函数进行测试*


请注意这个 `internal_adder` 函数，未被标记为 `pub`。其中的那些测试，都只是些 Rust 代码，同时那个 `tests` 模组，只是另一个模组。如同在前面的 [“用于对模组树中某个项目进行引用的路径”](Ch07_Managing_Growing_Projects_with_Packages_Crates_and_Modules.md#paths-for-referring-to-an-item-in-the-module-tree) 小节中所讨论的，子模组中的那些项目，可以使用其祖辈模组中的项目。在这个测试中，就以 `use super::*` 语句，将那个 `tests` 模组父辈的那些项目，带入到了作用域，进而该测试随后就可以调用 `internal_adder` 了。而在不认为私有函数应被测试时，那么 Rust 就没有什么可以迫使你对他们进行测试了（if you don't think private functions should be tested, there's nothing in Rust that will compel you to do so）。


### 集成测试

在 Rust 中，集成测试整个都是属于所编写库外部的。他们会以与其他代码同样方式，对咱们编写的库加以使用，这就意味着集成测试只能调用属于库公开 API 一部分的那些函数。集成测试的目的，是要就所编写库的多个部分，是否有正确地一起运作进行测试。这些各自正常工作的代码单元，在被集成在一起时，就可能有问题，因此集成后代码的测试面，也是重要的。要创建集成测试，首先就需要一个 `tests` 目录。


**`tests` 目录**

这里时在项目目录的顶层，挨着那个 `src` 目录，创建一个 `tests` 目录的。Cargo 就明白要在整个目录下，查找那些集成测试的文件。至于可以构造多少个测试文件，则是想要多少都可以，Cargo 将把这些各个文件，编译为单独的代码箱。

下面就来创建一个集成测试。使用清单 11-12 中仍在 `src/lib.rs` 文件中的代码，构造一个 `tests` 目录，并创建一个名为 `tests/integration_test.rs` 的文件。那么现在的目录结构，应像下面这样：

```console
adder
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_test.rs
```

请将下面清单 11-13 中的代码，输入到那个 `tests/integration_test.rs` 文件中：

文件名：`tests/integration_test.rs`

```rust
use adder;

#[test]
fn it_adds_two() {
    assert_eq! (4, adder::add_two(2));
}
```

*清单 11-13：一个 `adder` 代码箱中函数的集成测试*

在 `tests` 目录中的每个文件，都是个单独代码箱，因此这里就需要将所编写的库，带入到各个测试代码箱的作用域。由于这个原因，这里就要在该代码的顶部，添加 `use adder` 语句，这在之前的单元测试中就不需要。

这里不需要以 `#[cfg(test)]` 对 `tests/integration_test.rs` 中的任何代码进行注解。Cargo 会特别对待 `tests` 目录，而只在运行 `cargo test` 时，才编译此目录中的文件。现在运行 `cargo test`：

```console
$ cargo test                                                                                         lennyp@vm-manjaro
   Compiling adder v0.1.0 (/home/lennyp/rust-lang/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.52s
     Running unittests src/lib.rs (target/debug/deps/adder-7763e46d5dd299a3)

running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/integration_test.rs (target/debug/deps/integration_test-d0d0eaf0bad2a59f)

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

输出的三个部分，包含了单元测试、集成测试与文档测试。请留意在某个部分的任何测试失败时，接下来的部分就不会运行了。比如，在某个单元测试失败时，由于集成测试与文档测试只会在全部单元测试通过时才运行，因此就不再会有集成与文档测试的任何输出了。

其中单元测试的第一部分，与之前曾见到过的一样：每个单元测试一行（那行就是在清单 11-12 中所添加的名为 `internal` 的测试），并随后有个这些单元测试的小结。

集成测试部分是以那行 `Running tests/integration_test.rs` 开始的。接下来，集成测试中的每个测试函数都有一行，且在紧接着 `Doc-tests adder` 部分开始之前，就是集成测试的那些结果的一个小结。

每个集成测试都有其自己的部分，那么在把更多文件添加到那个 `tests` 目录中时，就会有更多的集成测试部分了。

通过将测试函数的名字，指明为 `cargo test` 的命令行参数，这里仍可运行某个特定集成测试函数。而要运行某个特定集成测试文件中的全部测试，则要使用跟上了该文件名字的 `cargo test` 的 `--test` 参数：

```console
$ cargo test --test integration_test                                                                 lennyp@vm-manjaro
   Compiling adder v0.1.0 (/home/lennyp/rust-lang/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.23s
     Running tests/integration_test.rs (target/debug/deps/integration_test-d0d0eaf0bad2a59f)

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

此命令只运行在 `test/integration_test.rs` 文件中的那些测试。


**集成测试中的子模组**

随着更多集成测试的添加，就会想要在那个 `tests` 目录下，构造更多文件，来帮助组织这些文件；比如就可以将那些测试函数，按照他们所测试的功能而进行分组。如同早先所提到的，在 `tests` 目录下的各个文件，都作为其自己单独的代码箱而被编译，这一点对于创建独立作用域，来对最终用户将要使用所编写代码箱的方式，进行更紧密模拟是有用的。不过，这将意味着在 `tests` 目录中的那些文件，不会如同在第 7 章中，有关 [如何将代码分离为模组与文件](Ch07_Managing_Growing_Projects_with_Packages_Crates_and_Modules.md#separating-modules-into-different-files) 部分，所掌握的 `src` 中的那些文件那样，共用同样的行为。

在有着一套在多个集成测试文件中使用的辅助函数，并尝试遵循第 7 章 [将模组分离为不同文件](Ch07_Managing_Growing_Projects_with_Packages_Crates_and_Modules.md#separating-modules-into-different-files) 中的步骤，把这些辅助函数提取到某个通用模组中时，`tests` 目录的那些文件的不同行为就最为明显了。比如说，在创建出 `tests/common.rs` 并将一个名为 `setup` 的函数放在其中时，就可以将一些要在多个测试文件的多个测试函数调用的代码，添加到 `setup`。

文件名：`tests/common.rs`

```rust
pub fn setup() {
    // 特定于库测试的一些设置代码，将放在这里
}
```

当再度运行这些测试时，即使这个 `common.rs` 文件未包含任何测试函数，也没有从任何地方调用这个 `setup` 函数，仍会在测试输出中，发现这个 `common.rs` 文件的一个新部分：

```console
$ cargo test                                                                                         lennyp@vm-manjaro
   Compiling adder v0.1.0 (/home/lennyp/rust-lang/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.47s
     Running unittests src/lib.rs (target/debug/deps/adder-7763e46d5dd299a3)

running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/common.rs (target/debug/deps/common-82aa4aac16d81562)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/integration_test.rs (target/debug/deps/integration_test-d0d0eaf0bad2a59f)

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

以显示出他的 `running 0 tests` 方式，让 `common` 出现在测试结果中，并非咱们想要的。这里只是打算在其他集成测试文字之下，共用一些代码。

要避开让 `common` 出现在测试输出中，就要创建出 `tests/common/mod.rs`，而非创建出 `tests/common.rs`。该项目目录现在看起来像下面这样：

```console
adder
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    ├── common
    │   └── mod.rs
    └── integration_test.rs
```

这是曾在第 7 章 ["替代文件路径"](Ch07_Managing_Growing_Projects_with_Packages_Crates_and_Modules.md#alternate-file-paths) 小节所提到的，Rust 同样明白的较早命名约定。以这种方式命名该文件，就告诉 Rust 不要将那个 `common` 模组，作为一个集成测试文件对待。在将这个 `setup` 函数移入到 `tests/common/mod.rs` 里头，并删除了那个 `tests/common.rs` 文件时，在测试输出中的该部分就不再出现了。`tests` 目录子目录中的那些文件，不会作为单独代码箱而被编译，也不会在测试输出中拥有自己的部分。


在创建出 `tests/common/mod.rs` 之后，就可以从任意的集成测试文件，将其作为模组而加以使用。下面就是一个从 `tests/integration_test.rs` 中的 `it_adds_two` 测试，对这个 `setup` 函数进行调用的示例：

文件名：`tests/integration_test.rs`

```rust
use adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq! (6, adder::add_two(4));
}
```

请留意其中的 `mod common;` 声明，与曾在清单 7-21 中演示过的模组声明相同。随后在那个测试函数中，这里既可以调用那个 `common::setup()` 函数了。


**二进制代码箱的集成测试**

在所编写详细是个仅包含 `src/main.rs` 文件的二进制代码箱，而没有 `src/lib.rs` 文件时，就无法在 `tests` 目录中创建集成测试，以及使用 `use` 语句，将定义在 `src/main.rs` 中的函数带入到作用域。唯有库代码箱将其他代码箱可以使用的函数，给暴露出来；二进制代码箱本来就是由他们自己来运行的（binary crates are meant to be run on their own）。

这是那些提供到二进制程序的 Rust 项目，有着一个直接了当的、对存在于 `src/lib.rs` 逻辑进行调用的 `src/main.rs` 文件的原因之一。运用那样的结构，集成测试就 *可以* 使用 `use` 对库代码箱进行测试，从而令到重要功能可用。当重要功能运作时，那么在那个 `src/main.rs` 文件中的少量代码，也将同样工作，同时那少量代码就不需要被测试了。

## 本章小结

Rust 的这些测试特性，提供到一种指明代码应如何生效，从而确保即使在进行了修改时，其仍继续如预期那样工作的方式。单元测试对库的各个部分进行单独检查，而可对一些私有实现细节进行测试。集成测试则对库的多个部分一起正确运作进行检查，同时他们会使用库的公开 API，以与外部代码使用库的同样方式，对代码进行测试。即使 Rust 的类型系统与所有权规则有助于防止某些种类的代码错误，对于消除与所编写代码预期表现方式有关的逻辑错误，测试仍是必不可少的。

下面就来将本章以及前面那些章中所掌握的知识结合起来，在一个项目上练手一下了！
