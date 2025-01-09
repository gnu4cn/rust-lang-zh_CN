# 怎样编写测试

所谓测试，是指一些验证非测试代码（the non-test code）以预期方式发挥作用的函数（tests are Rust functions that verify that the non-test code is functioning in the expected manner）。测试函数的函数体，通常执行以下三种操作：

1. 建立起全部所需的数据或状态；

2. 运行打算测试的代码；

3. 就运行结果是所期望的结果进行断言（assert the results are what you expect）。

下面就来看看，Rust 专为编写进行这些操作的测试，而提供到一些特性，包括 `test` 属性（the `test` attribute）、几个宏，以及 `should_panic` 属性（the `should_panic` attribute）。


## 测试函数剖析

**The Anatomy of a Test Function**

Rust 最简单形态的测试，就是以 `test` 属性注解的一个函数。所谓属性，是指有关 Rust 代码片段的元数据（attributes are metadata about pieces of Rust code）；在第 5 章中，[用在结构体上的 `derive` 属性](Ch05_Using_Structs_to_Structure_Related_Data.md#使用派生特质加入有用功能)，就是一个属性的例子。要将某个函数修改为测试函数，就要把 `#[test]` 添加在 `fn` 之前的行上。在以 `cargo test` 命令运行编写的测试时，Rust 就会构建一个运行这些注解过的函数，并就各个测试函数是否通过或失败进行汇报的测试运行器二进制文件（a test runner binary）。

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

将某个测试标记为忽略，进而其在特定实例中不运行，是可能的；在本章后面的 ["忽视某些在特别要求下才运行的测试（Ignoring Some Tests Unless Specifically Requested）"](#在未作特别要求时忽略某些测试) 小节，就会讲到这个问题。由于这里尚未完成这个问题，因此这里的测试总结，就给出了 `0 ignored`。这里还可以把一个参数，传递给这个 `cargo test` 命令，来只测试那些名字与某个字符串匹配的测试；此特性叫做 *过滤（filtering）*，在 [“通过指定测试名字运行测试子集（Running a Subset of Tests）”](#依据测试名称来运行测试的某个子集) 小节，就会讲到这个问题。而这里也没有对所运行的测试加以过滤，因此在该测试小结的最后，显示了 `0 filtered out`。

其中属于基准测试的 `0 measured` 统计值，对性能进行了测量。所谓基准测试（benchmark tests），就跟其字面意思一样，只在每日构建版的 Rust 中可用。请参阅 [基准测试相关文档](https://doc.rust-lang.org/unstable-book/library-features/test.html) 了解更多信息。

测试输出接下来的部分，是以 `Doc-tests adder` 开始的，在有文档测试时，这便是文档测试的输出。虽然目前尚无文档测试，当 Rust 是可以编译在 API 文档中的全部代码示例的。此特性有助于将文档与代码保持同步！在第 14 章的 [“作为测试的文档注释（Documentation Comments as Tests）”](Ch14_More_about_Cargo_and_Crates_io.md#作为测试的文档注释) 小节，就会讨论怎样编写文档测试。至于现在，就会这个 `Doc-tests` 的输出加以忽略。


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

这里不再是 `ok` 了，`test tests::another` 那行给出了 `FAILED`。在这单独结果与测试小结直接，出现了两个新的部分：第一部分显示各个测试失败的具体原因。在此示例中，就得到 `another` 失败详情，是由于该测试函数在 `src/lib.rs` 文件第 15 行处 `panicked at '令该测试失败'`。接下来的部分，则列出了仅所有失败测试的名字，这在有很多测试，进而有很多详细失败测试输出时，是有用的。随后就可以使用某个失败测试的名字，来只运行该项测试而更容易地对其加以调试；在 [“对测试运行方式进行控制（Controlling How Tests Are Run）”](#控制测试以何种方式运行) 小节，将对运行测试方式，进行深入讲解。


显示在最后的测试小节行：总体上看，这个测试的结果为 `FAILED`。这里有一个测试通过，以及一个测试失败了。

既然现在已经见识了不同场景下测试结果的样子，那么就来看看在测试中，除 `panic!` 之外其他一些有用的宏。


## 使用 `assert!` 宏，检查测试结果

**Checing Results with the `assert!` Macro**


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

请注意这里在 `tests` 模组里头添加了个新行：`use super::*;`。这个 `tests` 模组是个遵循第 7 章中，[“用于指向模组树中某个项目的路径”](Ch07_Managing_Growing_Projects_with_Packages_Crates_and_Modules.md#用于引用目录树中项目的路径)小节中曾讲到一般可见性规则的常规模组。由于这个 `tests` 模组是个内部模组，因此这里就需要将外层模组中的受测试代码，带入到这个 `tests` 内部模组的作用域。而由于这里使用了一个全局通配符（a glob, `*`），因此所有在外层模组中定义的内容，就对这个 `tests` 模组可用了。

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


## 使用 `assert_eq!` 与 `assert_ne!` 两个宏，测试相等性

**Testing Equality with the `assert_eq!` and `assert_ne!` Macros**


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

表象之下，`assert_eq!` 与 `assert_ne!` 两个宏，分别使用了运算符 `==` 与 `!=`。在他们的断言失败时，这两个宏就会使用调试格式化（debug formatting），将他们的参数打印出来，这就意味着正被比较的两个值，必须实现了 `PartialEq` 与 `Debug` 特质。全部原生值与绝大多数的标准库类型，都实现了这两个特质。而对于咱们自己定义的结构体与枚举，就需要实现 `PartialEq` 来对这些类型的相等与否进行断言。同样还需要实现 `Debug`，来在断言失败时打印比较的两个值。由于这两个特质都正如第 5 章清单 5-12 中所提到的派生特质（derivable traits），这样就跟将 `#[derive(PartialEq, Debug)]` 注解，添加到所编写的结构体或枚举定义一样直接了。请参阅附录 C，[“可派生特质（derivable traits）”](Ch21_Appdendix.md#附录-c派生特质) 了解更多有关这两个及其他派生特质的详细信息。


## 添加定制的失败消息

**Adding Custom Failure Message**


还可将与失败消息一同打印的定制消息，作为 `assert!`、`assert_eq!` 及 `assert_ne!` 宏的可选参数加入进来。在必须的两个参数之后指定的全部参数，都被传递给他们中的 `format!` 宏（第 8 章中 [“以 `+` 操作符或 `format!` 宏的字符串连接（Concatenation with the `+` Operator or the `format!` macro）”](Ch08_Common_Collections.md#使用--运算符或-format-宏的字符串连接)） 小节曾讲到），因此就可以传递一个包含了 `{}` 占位符的格式化字符串，以及进到这些占位符的值。对于给某个断言表示什么的文档编制，这些定制消息就是有用的；在某个测试失败时，就会有着该代码下那个问题的较好理解。

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


## 使用 `should_panic`，检查中止运行

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


## 在测试中使用 `Result<T, E>`

**Using `Result<T, E>` in Tests**


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


（End）


