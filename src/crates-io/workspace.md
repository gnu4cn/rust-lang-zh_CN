# Cargo 工作区

在第 12 章中，我们构建了个包含二进制代码箱和库代码箱的包。随着项目的开发，咱们会发现库代码箱不断变大，进而就会希望进一步拆分咱们的包为多个库代码箱。Cargo 提供了一项名为 *工作区* 的特性，可以帮助管理协同开发的多个相关包。

> **译注**：
>
> 总结 Rust 开发的层次结构如下：
>
> 工作区，workspace -> 包，package -> 代码箱，crate -> 模组，module -> 语句，statement


## 创建工作区

所谓 *工作区*，是一组共用同一 `Cargo.lock` 文件和输出目录的。我们来构造一个用到工作区的项目 -- 我们将使用简单代码，以便可以专注于工作区的结构。组织工作区的方式有多种，因此我们将仅展示一种常见方式。我们将有个包含二进制代码箱，和两个库代码箱的工作区。提供主要功能的二进制代码箱，将依赖于两个库代码箱。其中一个库代码箱将提供 `add_one` 函数，另一个将提供 `add_two` 函数。这三个代码箱将属于同一工作区。我们将以创建工作区的目录开始：

```console
$ mkdir add
$ cd add
```

接下来，在 `add` 目录下，我们创建 `Cargo.toml` 文件，其将配置整个工作区。这个文件将没有 `[package]` 小节。相反，他将以 `[workspace]` 小节开头，这个小节将允许我们添加成员到工作取。我们还通过设置 `resolver` 键为 `"3"`，指定使用最新、最高版本的 [Cargo 解析器](https://doc.rust-lang.org/cargo/reference/resolver.html) 算法：

文件名：`Cargo.toml`

```toml
[workspace]
resolver = "3"
```

接下来，我们将通过在 `add` 目录中运行 `cargo new` 命令，创建二进制代码箱 `adder`：

```console
$ cargo new adder
    Creating binary (application) `adder` package
      Adding `adder` as member of workspace at `/home/hector/rust-lang-zh_CN/projects/add`
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
```

在工作区内部运行 `cargo new` 命令还会自动添加新创建的包，到工作区的 `Cargo.toml` 文件中的 `[workspace]` 定义中的 `members` 键下，像下面这样：

文件名：`Cargo.toml`

```toml
[workspace]
resolver = "3"
members = ["adder"]
```

此时，我们可以通过运行 `cargo build` 构建工作区。咱们的 `add` 目录下的文件应看起来如下：

```console
.
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
├── Cargo.lock
├── Cargo.toml
└── target
```

工作区在顶层有个 `target` 目录，编译后的产物将被放置其中；`adder` 包没有自己的 `target` 目录。即使我们从 `adder` 目录内部运行 `cargo build`，编译后的产物也仍然会出现在 `add/target` 中，而不是 `add/adder/target` 目录中。Cargo 之所以如此组织 `target` 目录，是因为工作区中的代码箱是为了相互依赖。如果每个代码箱都有自己的 `target` 目录，那么为了放置编译产物于自己的 `target` 目录中，每个代码箱都将必须重新编译工作区中的其他每个代码箱。通过共用一个 `target` 目录，代码箱可以避免不必要的重新构建。


## 在工作区中创建第二个包

接下来，咱们来在工作区中创建另一个成员包，并叫他 `add_one`。生成一个名为 `add_one` 的库代码箱：

```console
$ cargo new add_one --lib
    Creating library `add_one` package
      Adding `add_one` as member of workspace at `/home/hector/rust-lang-zh_CN/projects/add`
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
```

顶层的 `Cargo.toml` 现在将包含 `add_one` 的路径于 `members` 列表中：

文件名：`Cargo.toml`

```toml
[workspace]
resolver = "3"
members = ["add_one","adder"]
```

咱们的 `add` 目录现在应该有着以下这些目录与文件：

```console
.
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
├── add_one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── Cargo.lock
├── Cargo.toml
└── target
```

在 `add_one/src/lib.rs` 文件中，我们来添加一个 `add_one` 函数：

文件名：`add_one/src/lib.rs`

```rust
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

现在我们可以让带有我们的二进制代码箱的 `adder` 包，依赖于有着我们的库代码箱的 `add_one` 包。首先，咱们将需要添加对 `add_one` 的路径依赖到 `adder/Cargo.toml`。

文件名：`adder/Cargo.toml`

```toml
[dependencies]
add_one = { path = "../add_one" }
```

Cargo 不会假设工作区中的代码箱将相互依赖，因此我们需要明确依赖关系。

接下来，我们来在 `adder` 代码箱中使用 `add_one` 函数（来自 `add_one` 代码箱）。请打开 `adder/src/main.rs` 文件，并修改 `main` 函数为调用 `add_one` 函数，如下清单 14-7 中所示。

<a name="listing_14-7"></a>
文件名：`adder/src/main.rs`

```rust
fn main() {
    let num = 10;
    println!("你好，世界！{num} 加一为 {}!", add_one::add_one(num));
}
```

**清单 14-7**：在 `adder` 代码箱中使用 `add_one` 库代码箱

我们来在顶级的 `add` 目录下运行 `cargo build` 构建工作区！

```console
$ cargo build
   Compiling add_one v0.1.0 (/home/hector/rust-lang-zh_CN/projects/add/add_one)
   Compiling adder v0.1.0 (/home/hector/rust-lang-zh_CN/projects/add/adder)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
```

要从 `add` 目录运行二进制代码箱，我们可以通过在 `cargo run` 命令下，使用 `-p` 的命令行参数和包的名字，指定我们打算运行的工作区中的包：

```console
$ cargo run -p adder
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/adder`
你好，世界！10 加一为 11!
```

这会运行 `adder/src/main.rs` 中的代码，其依赖于 `add_one` 代码箱。


## 依赖于外部代码箱

请注意，工作区只有一个位于顶层的 `Cargo.lock` 文件，而不是在每个代码箱目录下都有个 `Cargo.lock`。这确保了所有代码箱都使用所有依赖项的同一版本。当咱们添加 `rand` 包到 `adder/Cargo.toml` 与 `add_one/Cargo.toml` 两个文件时，Cargo 将解析他们俩为一个版本的 `rand`，并将其记录在一个 `Cargo.lock` 中。让工作区中所有代码箱都使用相同的依赖项，意味着这些代码箱将始终相互兼容。我们来添加 `rand` 代码箱到 `add_one/Cargo.toml` 文件中的 `[dependencies]` 小节，以便我们可以在 `add_one` 代码箱中使用 `rand` 代码箱：

文件名：`add_one/Cargo.toml`

```toml
[dependencies]
rand = "0.8.5"
```

现在，我们可以添加  `use rand;` 到 `add_one/src/lib.rs` 文件，然后通过在 `add` 目录下运行 `cargo build` 构建整个工作区，就将带入并编译 `rand` 代码箱。由于我们没有引用那个我们已带入作用域的 `rand`，因此将得到一条告警：

```console
$ cargo build
   Updating `ustc` index
remote: Enumerating objects: 21928, done.
-- 跳过输出 --
   Compiling rand v0.8.5
   Compiling add_one v0.1.0 (/home/hector/rust-lang-zh_CN/projects/add/add_one)
warning: unused import: `rand`
 --> add_one/src/lib.rs:1:5
  |
1 | use rand;
  |     ^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: `add_one` (lib) generated 1 warning (run `cargo fix --lib -p add_one` to apply 1 suggestion)
   Compiling adder v0.1.0 (/home/hector/rust-lang-zh_CN/projects/add/adder)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
```

顶层的 `Cargo.lock` 现在包含有关 `add_one` 对 `rand` 的依赖信息。然而，即使 `rand` 在工作区中的某处被用到，我们也不能在工作取中的其他代码箱中使用他，除非添加 `rand` 到其他代码箱的 `Cargo.toml` 文件。例如，当我们添加 `use rand;` 到 `adder` 包的 `adder/src/main.rs` 文件时，我们将得到一个报错：

```console
$ cargo build
-- 跳过输出 --
   Compiling adder v0.1.0 (/home/hector/rust-lang-zh_CN/projects/add/adder)
error[E0432]: unresolved import `rand`
 --> adder/src/main.rs:1:5
  |
1 | use rand;
  |     ^^^^ no external crate `rand`

For more information about this error, try `rustc --explain E0432`.
error: could not compile `adder` (bin "adder") due to 1 previous error
```

要解决这个问题，就也要编辑 `adder` 包的 `Cargo.toml` 文件并指出 `rand` 是其依赖项。构建 `adder` 包将添加 `rand` 到 `Cargo.lock` 中的 `adder` 的依赖项列表，但不会有额外的 `rand` 副本被下载。Cargo 将确保工作区中的每个包中的每个使用 `rand` 包的代码箱都使用同一版本，只要他们指定的是兼容版本的 `rand`，从而为我们节省空间并确保工作区中的代码箱都将相互兼容。

当工作区中的代码箱指定了同一依赖项的不兼容版本时，Cargo 将解析每个版本，但仍将尝试解析尽可能少的版本。


## 添加测试到工作区

针对另一项改进，我们来在 `add_one` 代码箱内，添加对 `add_one::add_one` 函数的测试：

文件名：`add_one/src/lib.rs`

```rust
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}
```

现在在顶层的 `add` 目录下运行 `cargo test`。在像这样组织的工作区中运行 `cargo test`，将运行工作区中所有代码箱的测试：

```console
$ cargo test
   Compiling add_one v0.1.0 (/home/hector/rust-lang-zh_CN/projects/add/add_one)
   Compiling adder v0.1.0 (/home/hector/rust-lang-zh_CN/projects/add/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running unittests src/lib.rs (target/debug/deps/add_one-931e477ce15f77ac)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/adder-4745fed6b42a29b6)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

输出的第一个小节显示 `add_one` 代码箱中的 `it_works` 测试已通过。下一个小节显示在 `adder` 代码箱中找到零个测试，然后最后的小节显示在 `add_one` 代码箱中找到零个文档测试。（**译注**：二进制代码箱中不会有文档测试？）

我们还可以通过使用 `-p` 命令行开关并指定我们打算测试的代码箱名字，从顶层目录针对一个特定代码箱运行测试：


```console
$ RUSTFLAGS="-A warnings" cargo test -p add_one
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running unittests src/lib.rs (target/debug/deps/add_one-931e477ce15f77ac)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

这一输出显示，`cargo test` 仅运行了 `add_one` 代码箱的测试，而未运行 `adder` 代码箱的测试。

当咱们发布工作区中的代码箱到 `crates.io` 时，工作区中的每个代码箱将需要单独发布。与 `cargo test` 一样，我们可以通过使用 `-p` 命令行开关并指定我们打算发布的代码箱名字，发布工作区中的特定代码箱。

作为额外练习，请以与 `add_one` 代码箱的类似方式，添加 `add_two` 代码箱到这个工作区！

随着项目增长，请考虑使用工作区：相比于一大堆代码，他让咱们能够在更小、更易于理解的组件下工作。此外，当代码箱会经常同时修改时，把代码箱保留在工作区中可以使代码箱之间的协作更容易。


（End）


