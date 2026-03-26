# 控制测试运行方式

正如 `cargo run` 会编译咱们的代码并随后运行产生的二进制文件一样，`cargo test` 会在测试模式下编译咱们的代码并运行产生的测试二进制文件。由 `cargo test` 生成的二进制文件的默认行为是并行运行所有测试并捕获测试运行期间生成的输出，从而防止输出显示出来并使阅读与测试结果相关的输出更容易。但是，咱们可以执行命令行选项来改变这种默认行为。

一些命令行选项作用于 `cargo test`，一些则作用于生成的测试二进制文件。为了区分这两种类型的参数，咱们要列出作用于 `cargo test` 的参数，后跟分隔符 `--`，然后列出作用于测试二进制文件的参数。运行 `cargo test --help` 显示咱们可对 `cargo test` 使用的选项，运行 `cargo test -- --help` 显示咱们可在分隔符之后使用的选项。


## 并行或顺序运行测试

当咱们运行多个测试时，默认情况下他们会使用线程并行运行，这意味着他们会更快地完成运行，咱们也会更快地获得反馈。由于测试是同时运行的，咱们必须确保测试不会相互依赖，也不依赖任何共用状态，包括共用环境，比如当前工作目录或环境变量。

例如，假设咱们的每个测试都运行了一些代码，他们都会在磁盘上创建一个名为 `test-output.txt` 的文件，并将一些数据写到该文件。然后，每个测试都读取该文件中的数据并断言该文件包含特定值，这个值在每个测试都不同。由于测试同时运行，一个测试可能会在另一测试写入和读取该文件之间的时间内覆盖该文件。第二个测试随后将失败，并非因为代码不正确，而是因为测试在并行运行时相互干扰了。一种解决方案是确保每个测试写入不同的文件；另一种解决防范是一次运行一个测试。

当咱们不希望并行运行测试，或者希望对所使用的线程数进行更细粒的控制时，咱们可以发送 `--test-threads` 命令行开关和咱们想要使用的线程数到测试二进制文件。请看以下示例：

```console
$ cargo test -- --test-threads=1
```

我们设置测试线程数为 `1`，告诉程序不要使用任何并行机制。使用一个线程运行测试将比并行运行他们需要更长的时间，但当测试共用状态时，他们不会相互影响。


## 显示函数输出

默认情况下，当测试通过时，Rust 的测试库会捕获打印到标准输出的任何内容。例如，当我们在测试中调用 `println!` 且该测试通过时，我们将不会在终端中看到 `println!` 输出；我们只会看到表明测试已通过的行。当测试失败时，我们将看到与失败消息的其余部分一起打印到标准输出的内容。

举个例子，下面清单 11-10 有个简单的函数，打印其参数的值并返回 10，以及一个会通过的测试与一个会失败的测试。

<a name="listing_11-10"></a>
文件名：`src/lib.rs`

```rust
fn prints_and_returns_10(a: i32) -> i32 {
    println! ("我得到了值 {a}");
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

**清单 11-10**：调用 `println!` 的函数的测试

当我们以 `cargo test` 运行这两个测试时，我们将看到以下输出：

```console
$ cargo test
   Compiling silly-function v0.1.0 (/home/hector/rust-lang-zh_CN/projects/silly-function)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running unittests src/lib.rs (target/debug/deps/silly_function-9b19ab024839ea05)

running 2 tests
test tests::this_test_will_fail ... FAILED
test tests::this_test_will_pass ... ok

failures:

---- tests::this_test_will_fail stdout ----
我得到了值 8

thread 'tests::this_test_will_fail' (199369) panicked at src/lib.rs:19:9:
assertion `left == right` failed
  left: 5
 right: 10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::this_test_will_fail

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`
```

请注意，在这一输出中我们没有看到 `我得到了值 4`，而这个字符串会在通过的测试运行时打印。该输出已被捕获。失败的测试的输出，`我得到了值 8` 则出现在测试摘要输出的小节中，这个小节还显示了测试失败的原因。

当我们希望也看到通过的测试的打印值时，我们可以 `--show-output` 命令行开关告诉 Rust 同时显示成功测试的输出。


```console
$ cargo test -- --show-output
```

当我们以 `--show-output` 命令行开关再次运行清单 11-10 中的测试时，我们会看到以下输出：

```console
$ cargo test -- --show-output
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/silly_function-9b19ab024839ea05)

running 2 tests
test tests::this_test_will_pass ... ok
test tests::this_test_will_fail ... FAILED

successes:

---- tests::this_test_will_pass stdout ----
我得到了值 4


successes:
    tests::this_test_will_pass

failures:

---- tests::this_test_will_fail stdout ----
我得到了值 8

thread 'tests::this_test_will_fail' (200537) panicked at src/lib.rs:19:9:
assertion `left == right` failed
  left: 5
 right: 10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::this_test_will_fail

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`
```


## 根据名字运行测试子集

运行完整的测试套件有时可能需要很长时间。当咱们正在处理某个特定方面的代码时，咱们可能只打算运行与该代码相关的测试。咱们可以通过作为参数，传递给 `cargo test` 咱们打算运行的测试的名字，选择要运行的测试。

为了演示怎样运行测试子集，我们将首先为 `add_two` 函数创建三个测试，如下清单 11-11 中所示，然后选择要运行的测试。


<a name="listing_11-11"></a>
文件名：`src/lib.rs`

```rust
pub fn add_two(a: u64) -> u64 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        let result = add_two(2);
        assert_eq! (4, result);
    }

    #[test]
    fn add_three_and_two() {
        let result = add_two(3);
        assert_eq! (5, result);
    }

    #[test]
    fn one_hundred() {
        let result = add_two(100);
        assert_eq! (102, result);
    }
}
```

**清单 11-11**：有着不同名字的三个测试

正如我们之前所见，当我们在未传递任何参数下运行测试时，所有测试都将并行运行：

```console
$ cargo test
   Compiling adder v0.1.0 (/home/hector/rust-lang-zh_CN/projects/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running unittests src/lib.rs (target/debug/deps/adder-9c63fdd4b3155cad)

running 3 tests
test tests::add_three_and_two ... ok
test tests::add_two_and_two ... ok
test tests::one_hundred ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```


### 运行单个测试

我们可以传递任何测试函数的名字给 `cargo test`，来只运行该测试：

```console
$ cargo test one_hundred
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/adder-9c63fdd4b3155cad)

running 1 test
test tests::one_hundred ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

```

只运行了名字为 `one_hundred` 的测试；其他两个测试与该名字不匹配。测试输出通过在末尾处显示 `2 filtered out`，让我们知道还有更多未运行的测试。

我们不能以这种方式指定多个测试的名字；只有给予 `cargo test` 的第一个值将被使用。但有一种运行多个测试的方式。


### 通过过滤运行多个测试

我们可以指定测试名字的一部分，而任何名字与值匹配的测试都将运行。例如，因为我们的测试中有两个都包含 `add`，所以我们可以通过运行 `cargo test add` 运行这两个测试：

```console
$ cargo test add
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/adder-9c63fdd4b3155cad)

running 2 tests
test tests::add_two_and_two ... ok
test tests::add_three_and_two ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

```

这条命令运行了所有名字中有 `add` 的测试，并过滤掉了名为 `one_hundred` 的测试。另外请注意，测试所在的模组将成为测试名字的一部分，因此我们可以通过按模组名字进行过滤，而运行某个模组中的所有测试。


## 除非特别要求，否则忽略测试

有时，少数特定测试执行起来可能非常耗时，因此咱们可能希望在大多数的 `cargo test` 运行期间排除他们。咱们可以使用 `ignore` 属性注解这些耗时的测试以排除他们，而不是作为参数列出咱们打算运行的所有测试，如下所示：

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
    fn it_works() {
        let result = add_two(2);
        assert_eq! (4, result);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        assert_ne! (100, nth_fibonacci(50));
    }
}
```

在 `#[test]` 之后，我们添加了 `#[ignore]` 行到打算排除的测试。现在，当我们运行测试时，`it_works` 会运行，而 `expensive_test` 不会：

```console
$ cargo test
  Compiling adder v0.1.0 (/home/hector/rust-lang-zh_CN/projects/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running unittests src/lib.rs (target/debug/deps/adder-9c63fdd4b3155cad)

running 2 tests
test tests::expensive_test ... ignored
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

`expensive_test` 函数被列为 `ignored`。当我们打算只运行被忽略的测试时，我们可以使用 `cargo test -- --ignored`：

```console
$ cargo test -- --ignored
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/adder-9c63fdd4b3155cad)

running 1 test
test tests::expensive_test has been running for over 60 seconds
test tests::expensive_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 76.97s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

通过控制哪些测试会运行，咱们可以确保 `cargo test` 的结果将得以快速返回。当咱们需要查看 `ignored` 测试的结果，并且咱们有时间等待结果时，咱们可以运行 `cargo test -- --ignored`。当咱们打算运行所有测试，无论他们是否被忽略时，咱们可以运行 `cargo test -- --include-ignored`。

```console
$ RUSTFLAGS="-A warnings" cargo test -- --include-ignored
   Compiling adder v0.1.0 (/home/hector/rust-lang-zh_CN/projects/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running unittests src/lib.rs (target/debug/deps/adder-9c63fdd4b3155cad)

running 2 tests
test tests::it_works ... ok
test tests::expensive_test ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 6.54s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

（End）

