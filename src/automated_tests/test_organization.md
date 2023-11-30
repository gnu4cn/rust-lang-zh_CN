# 测试的组织

如同在本章开头提到的，测试是门复杂的学问，而不同人群会使用不同术语及组织方式（testing is a complex discipline, and different people use different terminology and organization）。Rust 社群认为，测试是由两个大类组成：单元测试与集成测试（unit tests and integration tests）。*单元测试* 是一些小而更为专注的测试，他们一次单独测试一个模组，并可对私有接口进行测试（*unit tests* are small and more focused, testing one module in isolation at a time, and can test private interfaces）。*集成测试* 则是完全在所编写库外部进行，并会像其他外部代码那样，对咱们的代码加以使用，因此就只会对公开接口进行使用，且潜在每个测试会检查多个模组。


这两种类型的测试编写，对于确保代码库的各个部分有在单独及共同完成所预期的事项，都是重要的。


## 单元测试

单元测试的目的，是要将各个代码单元孤立于其余代码进行测试，从而快速定位出何处代码有如预期那样工作，以及何处代码未如预期那样工作。应将单元测试，放在 `src` 目录之下，在那些有着他们要测试代码的各个文件中。约定即为要在各个文件中，创建包含那些测试函数的一个名为 `tests` 的模组，并使用 `cfg(test)` 对该模组加以注解。


### 测试模组与 `#[cfg(test)]`


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


### 测试私有函数

**Testing Private Functions**


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


请注意这个 `internal_adder` 函数，未被标记为 `pub`。其中的那些测试，都只是些 Rust 代码，同时那个 `tests` 模组，只是另一个模组。如同在前面的 [“用于对模组树中某个项目进行引用的路径”](Ch07_Managing_Growing_Projects_with_Packages_Crates_and_Modules.md#用于引用目录树中项目的路径) 小节中所讨论的，子模组中的那些项目，可以使用其祖辈模组中的项目。在这个测试中，就以 `use super::*` 语句，将那个 `tests` 模组父辈的那些项目，带入到了作用域，进而该测试随后就可以调用 `internal_adder` 了。而在不认为私有函数应被测试时，那么 Rust 就没有什么可以迫使你对他们进行测试了（if you don't think private functions should be tested, there's nothing in Rust that will compel you to do so）。


## 集成测试

**Integration Tests**


在 Rust 中，集成测试整个都是属于所编写库外部的。他们会以与其他代码同样方式，对咱们编写的库加以使用，这就意味着集成测试只能调用属于库公开 API 一部分的那些函数。集成测试的目的，是要就所编写库的多个部分，是否有正确地一起运作进行测试。这些各自正常工作的代码单元，在被集成在一起时，就可能有问题，因此集成后代码的测试面，也是重要的。要创建集成测试，首先就需要一个 `tests` 目录。


### `tests` 目录

**The `tests` Directory**


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
