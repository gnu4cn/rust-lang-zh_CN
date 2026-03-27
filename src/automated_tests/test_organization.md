# 测试的组织

正如本章开头提到的，测试属于一门复杂的学科，不同的人使用不同的术语和组织方式。Rust 社群从两个主要类别考虑测试：单元测试和集成测试。所谓 *单元测试*，属于规模较小且更具针对性，一次单独测试一个模组，并且可以测试私有接口。*集成测试* 完全在咱们的库外部，而以任何其他外部代码相同的方式使用咱们的代码，仅使用公开接口，并且每个测试都可能检验多个模组。

编写这两种类型测试，对于确保库中的各个部分，分别或一起执行咱们期望的操作非常重要。


## 单元测试

单元测试的目的是，在与其他代码隔离的情况下测试每个代码单元，以快速定位代码在何处按预期工作，以及在何处未按预期工作。咱们将放置单元测试于 `src` 目录下，有着他们要测试的代码的每个文件中。约定是要在每个文件中创建名为 `tests` 的模组，包含测试函数并以 `cfg(test)` 注解该模组。


### `tests` 模组与 `#[cfg(test)]`

`tests` 模组上的 `#[cfg(test)]` 注解告诉 Rust，仅在咱们运行 `cargo test` 时编译并运行测试代码，而非运行 `cargo build` 时。在咱们只打算构建库时，这会节省编译时间，并节省生成的编译产物的空间，因为测试不会被包含。咱们将看到，由于集成测试位于不同的目录中，因此他们不需要 `#[cfg(test)]` 注解。但是，有于单元测试位于代码的同一文件中，咱们将使用 `#[cfg(test)]` 来指定他们不应被包含在编译结果中。

回顾一下，当我们在本章第一小节中生成新的 `adder` 项目时，Cargo 为咱们生成了下面这段代码：

文件：`src/lib.rs`

```console
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

在自动生成的 `tests` 模组上，属性 `cfg` 代表 *configuration*，并告诉 Rust 只应在给定特定配置选项的情况下，接下来的项目才应包含。在这一情形下，配置选项为 `test`，这是 Rust 针对编译及运行测试而提供的。通过使用 `cfg` 属性，只有当我们主动以 `cargo test` 运行测试时，Cargo 才会编译我们的测试代码。除了以 `#[test]` 注解的函数外，这还包括这一模组内可能的任何辅助函数。


### 私有函数测试

测试社区内部存在关于私有函数是否应被直接测试的争论，而其他语言使得测试私有函数变得困难或不可能。不论咱们遵循何种测试理念，Rust 的隐私规则确实允许咱们测试私有函数。请考虑下面清单 11-12 中有着私有函数 `internal_adder` 的代码。

<a name="listing_11-12"></a>
文件名：`src/lib.rs`

```rust
pub fn add_two(a: u64) -> u64 {
    internal_adder(a, 2)
}

fn internal_adder(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        let result = internal_adder(2, 2);
        assert_eq! (4, result);
    }
}
```

**清单 11-12**：测试私有函数


请注意，`internal_adder` 函数未标记为 `pub`。测试都只是些 Rust 代码，`tests` 模组也只是另一个模组。正如我们在 [引用模组树中项目的路径](../packages_crates_and_modules/paths.md) 中讨论过的，子模组中的项目可以使用其祖辈模组中的项目。在这个测试中，我们以 `use super::*` 带入所有属于 `tests` 模组父模组的项目到作用域，然后测试便可调用 `internal_adder`。当咱们不认为私有函数应被测试时，Rust 中也没有任何强制要求咱们这样做的规定。


## 集成测试

在 Rust 中，集成测试完全在咱们的库的外部。他们以与任何其他代码相同的方式使用咱们的库，这意味着他们只能调用属于咱们的库的公开 API 一部分的函数。他们的目的是测试咱们的库的许多部分一起是否正常工作。那些单独正常工作的代码单元，在集成后可能出现问题，因此集成代码的测试率也很重要。要创建集成测试，咱们首先需要一个 `tests` 目录。


### `tests` 目录

我们在项目目录的顶层、`src` 旁边创建一个 `tests` 目录。Cargo 知道要在这个目录下查找集成测试文件。然后我们可以根据需要构造任意数量的测试文件，Cargo 将把每个这些文件作为单独的代码箱编译。

我们来创建一个集成测试。以清单 11-12 中的代码仍在 `src/lib.rs` 文件中，构造一个 `tests` 目录，并创建一个名为 `tests/integration_test.rs` 的文件。咱们的目录结构应看起来如下：

```console
adder
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_test.rs
```

输入下面清单 11-13 中的代码到 `tests/integration_test.rs` 文件中：


<a name="listing_11-13"></a>
文件名：`tests/integration_test.rs`

```rust
use adder::add_two;

#[test]
fn it_adds_two() {
    let result = add_two(2);
    assert_eq!(result, 4);
}
```

**清单 11-13**：`adder` 代码箱中函数的集成测试

`tests` 目录中的每个文件都是个单独的代码箱，因此我们需要带入我们的的库到每个测试代码箱的作用域。出于这个原因，我们在代码的顶部添加 `use adder::add_two;`，在单元测试中我们不需要这点。

我们不需要以 `#[cfg(test)]` 注解 `tests/integration_test.rs` 中的任何代码。Cargo 会特别对待 `tests` 目录，仅在我们运行 `cargo test` 时才会编译这个目录中的文件。现在运行 `cargo test`：

```console
$ RUSTFLAGS="-A warnings" cargo test
   Compiling adder v0.1.0 (/home/hector/rust-lang-zh_CN/projects/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running unittests src/lib.rs (target/debug/deps/adder-9c63fdd4b3155cad)

running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/integration_test.rs (target/debug/deps/integration_test-cb65c98c270b37f9)

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

输出的三个小节包括单元测试、集成测试和文档测试。请注意，当某个小节中的任何测试失败时，后面的小节都将不会运行。例如，当某个单元测试失败时，就不会有集成与文档测试的任何输出，因为只有在所有的单元测试都通过时这些测试才会运行。

单元测试的第一个小节与我们已见到的相同：每个单元测试一行（我们在 [清单 11-12](#listing_11-12) 中添加的名为 `internal` 的行），然后是单元测试的摘要行。

集成测试小节以 `Running tests/integration_test.rs` 行开始。接下来，该集成测试中的每个测试函数都有一行，并且在 `Doc-tests adder` 小节开始之前有一个集成测试结果的摘要行。

每个集成测试都有自己的小节，因此当我们在 `tests` 目录下添加更多文件时，就将有更多的集成测试小节。

我们仍然可以通过指定测试函数的名字为 `cargo test` 的命令行参数，运行特定的集成测试函数。要运行某个特定集成测试文件中的所有测试，就要使用 `cargo test` 的 `--test` 参数，后跟该文件的名字：

```console
$ cargo test --test integration_test
   Compiling adder v0.1.0 (/home/hector/rust-lang-zh_CN/projects/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running tests/integration_test.rs (target/debug/deps/integration_test-cb65c98c270b37f9)

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

这条命令仅运行 `test/integration_test.rs` 文件中的测试。


### 集成测试中的子模组

当咱们添加更多集成测试时，咱们可能希望在 `tests` 目录下构造更多文件以帮助组织他们；例如，咱们可以根据他们测试的功能来分组测试函数。正如早先曾提到的，`tests` 目录下的每个文件都作为自己单独的代码箱编译，这对于创建单独的作用域，来更接近地模仿最终用户使用咱们的代码箱的方式非常有用。然而，这将意味着 `tests` 目录下的文件不会像 `src` 中的文件那样共用同一行为，正如咱们在第 7 章中了解的有关如何分离代码为模组与文件。

当咱们有一组要在多个集成测试文件中使用的辅助函数时，`tests` 目录文件的不同行为最为明显，咱们就要按照第 7 章的 [拆分模组为不同文件](../packages_crates_and_modules/separating_modules.md) 小节中的步骤，提取辅助函数到公共模组中。例如，当我们创建 `tests/common.rs` 并放置一个名为 `setup` 的函数在其中时，我们可以往 `setup` 添加一些我们打算从多个测试文件中的多个测试函数调用的代码：

文件名：`tests/common.rs`

```rust
pub fn setup() {
    // 特定于咱们的库的测试的设置代码，将放在这里
}
```

当我们再次运行测试时，我们将在测试输出中看到 `common.rs` 文件的一个新的小节，即使该文件未包含任何测试函数，我们也没有在任何地方调用`setup` 函数：

```console
$ cargo test
   Compiling adder v0.1.0 (/home/hector/rust-lang-zh_CN/projects/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.07s
     Running unittests src/lib.rs (target/debug/deps/adder-9c63fdd4b3155cad)

running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/common.rs (target/debug/deps/common-2b36a9e6692b2f41)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/integration_test.rs (target/debug/deps/integration_test-cb65c98c270b37f9)

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

让 `common` 出现在测试结果中并以 `running 0 tests` 对其显示并不是我们想要的。我们只希望与其他集成测试文字共用一些代码。为了避开让 `common` 出现于测试输出中，我们将创建 `tests/common/mod.rs`，而不是 `tests/common.rs`。项目目录现在看起来像下面这样：

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

这属于我们在第 7 章中 [备用文件路径](../packages_crates_and_modules/separating_modules.md#alt_file_path) 处曾提到的，Rust 也能理解的早期的命名约定。以这种方式命名文件告诉 Rust，不要将 `common` 模组视为集成测试文件。在我们迁移 `setup` 函数代码到 `tests/common/mod.rs` 中并删除 `tests/common.rs` 文件后，测试输出中的这一小节将不再出现。`tests` 目录的子目录下的文件，不会作为单独的代码箱编译，也不会有测试输出中的小节。

创建 `tests/common/mod.rs` 后，我们可以在任何集成测试文件中作为模组使用他。下面是从 `tests/integration_test.rs` 中的 `it_adds_two` 测试调用 `setup` 函数的示例：

文件名：`tests/integration_test.rs`

```rust
use adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup();

    let result = add_two(2);
    assert_eq!(result, 4);
}
```

请注意，`mod common;` 声明与我们在 [清单 7-21](../packages_crates_and_modules/separating_modules.md#listing_7-21) 中演示的模组声明相同。然后，在测试函数中，我们就可以调用 `common::setup()` 函数了。


### 二进制代码箱的集成测试

当我们的项目是个只包含 `src/main.rs` 而未包含 `src/lib.rs` 的二进制代码箱时，我们就不能在 `tests` 目录下创建集成测试，及以 `use` 语句带入定义在 `src/main.rs` 中的函数到作用域。只有库代码箱才会暴露其他代码箱可以使用的函数；二进制代码箱应该独立运行。

这是提供二进制文件的 Rust 项目，只有个简单的 `src/main.rs` 文件的原因之一，该文件会调用位于 `src/lib.rs` 中的逻辑。使用这种结构，集成测试 *可以* 测试库代码箱，通过 `use` 使重要功能可用。当重要功能可以工作时，`src/main.rs` 文件中的少量代码也将工作，进而那些少量代码就无需测试。


# 本章小结


Rust 的测试特性提供了一种指定代码应如何运作的方式，以确保即使在咱们进行修改时，代码也会继续如咱们预期那样工作。单元测试会分别验证库的不同部分，并可测试私有实现细节。集成测试检查库的许多部分是否正确地一起工作，他们会以外部代码使用库的同样方式，使用库的公开 API 测试代码。尽管 Rust 的类型系统和所有权规则有助于防止某些种类的 bug，但测试对于减少与咱们的代码的预期的行为相关的逻辑 bug 仍然很重要。

我们来结合咱们在这一章和前面那些章中学到的知识，着手完成一个项目！


（End）


