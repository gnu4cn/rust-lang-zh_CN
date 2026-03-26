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
    internal_add(a, 2)
}

fn internal_add(left: u64, right: u64) -> u64 {
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


### 集成测试中的子模组

**Submodules in Integration Tests**


随着更多集成测试的添加，就会想要在那个 `tests` 目录下，构造更多文件，来帮助组织这些文件；比如就可以将那些测试函数，按照他们所测试的功能而进行分组。如同早先所提到的，在 `tests` 目录下的各个文件，都作为其自己单独的代码箱而被编译，这一点对于创建独立作用域，来对最终用户将要使用所编写代码箱的方式，进行更紧密模拟是有用的。不过，这将意味着在 `tests` 目录中的那些文件，不会如同在第 7 章中，有关 [如何将代码分离为模组与文件](Ch07_Managing_Growing_Projects_with_Packages_Crates_and_Modules.md#将模组拆分为不同文件) 部分，所掌握的 `src` 中的那些文件那样，共用同样的行为。

在有着一套在多个集成测试文件中使用的辅助函数，并尝试遵循第 7 章 [将模组分离为不同文件](Ch07_Managing_Growing_Projects_with_Packages_Crates_and_Modules.md#将模组拆分为不同文件) 中的步骤，把这些辅助函数提取到某个通用模组中时，`tests` 目录的那些文件的不同行为就最为明显了。比如说，在创建出 `tests/common.rs` 并将一个名为 `setup` 的函数放在其中时，就可以将一些要在多个测试文件的多个测试函数调用的代码，添加到 `setup`。

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

这是曾在第 7 章 ["替代文件路径"](Ch07_Managing_Growing_Projects_with_Packages_Crates_and_Modules.md#备用文件路径) 小节所提到的，Rust 同样明白的较早命名约定。以这种方式命名该文件，就告诉 Rust 不要将那个 `common` 模组，作为一个集成测试文件对待。在将这个 `setup` 函数移入到 `tests/common/mod.rs` 里头，并删除了那个 `tests/common.rs` 文件时，在测试输出中的该部分就不再出现了。`tests` 目录子目录中的那些文件，不会作为单独代码箱而被编译，也不会在测试输出中拥有自己的部分。


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


### 二进制代码箱的集成测试

**Integration Tests for Binary Crates**


在所编写详细是个仅包含 `src/main.rs` 文件的二进制代码箱，而没有 `src/lib.rs` 文件时，就无法在 `tests` 目录中创建集成测试，以及使用 `use` 语句，将定义在 `src/main.rs` 中的函数带入到作用域。唯有库代码箱将其他代码箱可以使用的函数，给暴露出来；二进制代码箱本来就是由他们自己来运行的（binary crates are meant to be run on their own）。

这是那些提供到二进制程序的 Rust 项目，有着一个直接了当的、对存在于 `src/lib.rs` 逻辑进行调用的 `src/main.rs` 文件的原因之一。运用那样的结构，集成测试就 *可以* 使用 `use` 对库代码箱进行测试，从而令到重要功能可用。当重要功能运作时，那么在那个 `src/main.rs` 文件中的少量代码，也将同样工作，同时那少量代码就不需要被测试了。


# 本章小结


Rust 的这些测试特性，提供到一种指明代码应如何生效，从而确保即使在进行了修改时，其仍继续如预期那样工作的方式。单元测试对库的各个部分进行单独检查，而可对一些私有实现细节进行测试。集成测试则对库的多个部分一起正确运作进行检查，同时他们会使用库的公开 API，以与外部代码使用库的同样方式，对代码进行测试。即使 Rust 的类型系统与所有权规则有助于防止某些种类的代码错误，对于消除与所编写代码预期表现方式有关的逻辑错误，测试仍是必不可少的。

下面就来将本章以及前面那些章中所掌握的知识结合起来，在一个项目上练手一下了！


（End）


