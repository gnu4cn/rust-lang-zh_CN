# 控制测试以何种方式运行

就跟 `cargo run` 会编译代码并于随后运行得出的二进制程序一样，`cargo test` 也会以测试模式编译所编写的代码，并会运行得到的测试二进制程序。而由 `cargo test` 产生出的二进制程序默认行为，即是以并行方式运行全部测试，并在测试运行期间捕获输出，阻止输出被显示出来以及令到与测试结果相关的输出，更加易于阅读（the default behavior of the binary produced by `cargo test` is to run all the tests in parallel and capture output generated during test runs, preventing the output from being displayed and making it easier to read the output related to the test results）。不过，这里是可以指定一些命令行选项，来改变这种默认行为的。

一些命令行选项是介入到 `cargo test`，而一些则是介入所得到的测试二进制程序。在介入到 `cargo test` 的命令行参数之后，跟上分隔符 `--`，随后才是那些进到测试二进制程序的参数，以这样的方式把这两种类型的命令行参数区分开。运行 `cargo test --help`，就会显示出可在 `cargo test` 下使用的选项，而运行 `cargo test -- --help` 则会显示出可在分隔符之后使用的那些选项。


## 并行抑或连续地运行测试

**Running Tests in Parallel or Consecutively**


在运行多个测试时，这些测试默认使用线程以并行方式运行，意味着他们会运行得更快，而咱们也会迅速地得到反馈。由于这些测试是在同时运行的，因此就必须确保所编写的测试不会各自依赖，并依赖于任何共用的状态，包括某种共用环境，诸如当前工作目录或环境变量。

比如说，所编写的每个测试，都会运行一些在磁盘上创建名为 `test-output.txt` 的文件，并将某些数据写到那个文件的代码。随后各个测试就会读取那个文件中的数据，并就那个包含了某个特定值进行断言，这个断言的特定值在各个测试中是不同的。由于这些测试是在同一时间运行，某个测试就可能会在另一测试写入与读取这个文件期间，对该文件进行覆写。那么第二个测试随后就将并非由于代码不正确，而因为这些测试在并行运行期间，相互之间造成了影响而失败。一种解决办法，是确保各个测试写入到不同文件；另一种办法，就是以一次运行一个的方式，运行这些测试。

在不打算并行运行这些测试，或要对所用到线程数有更细粒度掌控时，就可以将 `--test-threads` 这个命令行标志，与打算使用的线程数目，发送给那个测试二进制程序。请看看下面这个示例：

```console
$ cargo test -- --test-threads=1
```

这里把测试线程数设置为了 `1`，这就告诉了该程序不要使用任何并行机制。使用一个线程运行这些测试，相比以并行方式运行他们，将耗费更长时间，但在这些测试共用了状态时，他们之间不会相互影响。


## 展示函数的输出

**Showing Function Output**


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


## 依据名字运行测试的某个子集

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


### 运行单个测试

**Running Single Tests**


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


### 使用过滤来运行多个测试

**Filtering to Run Multiple Tests**


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


## 除非有特别要求，否则忽略某些测试

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


（End）


