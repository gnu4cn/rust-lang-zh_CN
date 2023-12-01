# Cargo 工作区

**Cargo Workspaces**

在第 12 章中，咱们曾构建了个包含二进制代码箱和库代码箱的包，a package。随着咱们项目的持续开发，咱们会发现库代码箱会持续变大，而咱们就会想要把咱们的包，进一步拆分为多个库代码箱。Cargo 提供了可帮助管理多个齐头并进开发的相关包，名为 *工作区，workspace* 的特性。

> 注：总结 Rust 开发的层次结构如下：工作区，workspace -> 包，package -> 代码箱，crate -> 模组，module -> 语句，statement。


## 创建工作区


*工作区，a workspace* 是共享了同一 `Cargo.lock` 文件与输出目录的包集合。咱们来构造一个用到工作区的项目 -- 咱们将使用一些简单代码，这样咱们便可着重于工作区的结构。组织工作区有多种方式，因此咱们将只给出一种常见方式。咱们将会有包含着一个二进制代码箱，与两个库代码箱的一个工作区。其中的二进制代码箱，将提供主要功能，其将依赖于其中的两个库代码箱。而一个库代码箱将提供 `add_one` 函数，另一个则会提供 `add_two` 函数。这三个代码箱，都将是同一工作区的一部分。咱们将以创建出工作区目录开始：

```console
$ mkdir add
$ cd add
```

接下来，在 `add` 目录中，咱们就要创建出将对整个工作区加以配置的 `Cargo.toml` 文件。这个文件不会有 `[package]` 小节。相反，他会以 `[workspace]` 小节开始，其将允许咱们，通过指定出有着咱们的二进制代码箱的包路径，而把成员添加到工作区；在这个示例中，那个路径为 `adder`:

文件名：`Cargo.toml`

```toml
[workspace]
members = [
    "adder",
]
```

接着，咱们将通过在 `add` 目录里运行 `cargo new`，而创建出 `adder` 二进制代码箱：

```console
$ cargo new adder
     Created binary (application) `adder` package
```

到这里，咱们就可通过运行 `cargo build` 构建出工作区。`add` 目录下的文件，看起来应像下面这样：

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

在其顶层，工作区有个 `target` 目录，那些编译出的物件，the compiled artifacts，就会放入其中；`adder` 包没有自己的 `target` 目录。即使咱们在 `adder` 目录内运行 `cargo build`，那些编译出的物件，仍将出现在 `add/target` 中，而不是 `add/adder/target` 目录里。Cargo 之所以像这样来组织 `target` 目录，是因为工作区中的代码箱是为了依赖于彼此。若各个代码箱都有自己的 `target` 目录，那么为了把编译出的物件放在自己的 `target` 目录中，就不得不重新编译工作区中其他各个代码箱。经由共用一个 `target` 目录，代码箱就可以避免不必要的重新构建。


## 在工作区中创建第二个包

**Creating the Second Package in the Workspace**


接着，咱们来创建工作区中的另一个成员包，并将其叫做 `add_one`。请修改顶层的 `Cargo.toml`，在 `members` 清单中指明 `add_one` 的路径：

文件名：`Cargo.toml`

```toml
[workspace]

members = [
    "adder",
    "add_one",
]
```

随后生成名为 `add_one` 的新库代码箱：

```console
$ cargo new add_one --lib                                                                        lennyp@vm-manjaro
     Created library `add_one` package
```

`add` 目录现在应该有这些目录与文件：

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

在 `add_one/src/lib.rs` 文件中，咱们来添加一个 `add_one` 函数：

文件名：`add_one/src/lib.rs`

```rust
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

现在咱们就可以让有着咱们二进制代码箱的 `adder` 包，依赖于有着咱们库代码箱的 `add_one` 包了。首先，咱们将需要把有关 `add_one` 的路径依赖，a path dependency，添加到 `adder/Cargo.toml`。

文件名：`adder/Cargo.toml`

```toml
[dependencies]
add_one = { path = "../add_one" }
```

Cargo并不假设工作区中的箱子会相互依赖，所以我们需要明确说明依赖关系。

接下来，咱们就要在 `adder` 代码箱中，使用 `add_one` 函数（来自 `add_one` 代码箱）。请打开 `adder/src/main.rs` 文件，并在其顶部使用一个 `use` 行，把新的 `add_one` 库代码箱带入到作用域。随后修改 `main` 函数来调用 `add_one` 函数，如下清单 14-7 中所示。

文件名：`adder/src/main.rs`

```rust
use add_one::add_one;

fn main() {
    let num = 10;
    println!("你好，世界！{num} 加一为 {}!", add_one(num));
}
```

*清单 14-7：在 `adder` 代码箱中使用 `add_one` 库代码箱*

咱们来通过在 `add` 目录顶层运行 `cargo build`，构建工作区！

```console
$ cargo build                                                                                 lennyp@vm-manjaro
   Compiling add_one v0.1.0 (/home/lennyp/rust-lang/add/add_one)
   Compiling adder v0.1.0 (/home/lennyp/rust-lang/add/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 0.40s
```

而要在 `add` 目录运行二进制代码箱，咱们可通过使用 `-p` 命令行参数，指明咱们打算允许工作区中的哪个包，及与 `cargo run` 运行的包名字：

```console
$ cargo run -p adder                                                                          lennyp@vm-manjaro
   Compiling adder v0.1.0 (/home/lennyp/rust-lang/add/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 0.35s
     Running `target/debug/adder`
你好，世界！
        10 加 1 为 11!
```

这会运行 `adder/src/main.rs` 中的代码，其依赖于 `add_one` 代码箱。


## 于工作区中依赖外部代码箱

**Depending on an External Package in a Workspace**


请注意工作区只有一个在顶层的 `Cargo.lock` 文件，而非各个代码箱目录中都有 `Cargo.lock`。这确保了工作区的全部代码箱，都使用着同一版本的所有依赖。若咱们把 `rand` 包添加到 `adder/Cargo.toml` 及 `add_one/Cargo.toml` 两个文件，那么 Cargo 将把那两个依赖，解析为一个版本的 `rand`，并将其记录在那一个的 `Cargo.lock` 中。

让工作区中全部代码箱使用同样的依赖，意味着这些代码箱将始终相互兼容。咱们来把 `rand` 代码箱添加到 `add_one/Cargo.toml` 文件的 `[dependencies]` 小节，这样咱们便可在 `add_one` 代码箱中使用 `rand` 代码箱：

文件名：`add_one/Cargo.toml`

```toml
rand = "0.8.3"
```

现在咱们便可把 `use rand;` 添加到 `add_one/src/lib.rs` 文件了，而通过在 `add` 目录中运行 `cargo build` 构建整个工作区，就会带入并编译 `rand` 代码箱。由于咱们没有引用咱们已带入到作用域中的 `rand`，因此咱们将得到一条告警：

```console
$ cargo build                                                                                 lennyp@vm-manjaro
    Updating crates.io index
  Downloaded rand_core v0.6.4
  Downloaded ppv-lite86 v0.2.17
  Downloaded getrandom v0.2.8
  Downloaded libc v0.2.137
  Downloaded 4 crates (681.6 KB) in 1.29s
   Compiling libc v0.2.137
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.17
   Compiling getrandom v0.2.8
   Compiling rand_core v0.6.4
   Compiling rand_chacha v0.3.1
   Compiling rand v0.8.5
   Compiling add_one v0.1.0 (/home/lennyp/rust-lang/add/add_one)
warning: unused import: `rand`
 --> add_one/src/lib.rs:1:5
  |
1 | use rand;
  |     ^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: `add_one` (lib) generated 1 warning

   Compiling adder v0.1.0 (/home/lennyp/rust-lang/add/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 6.76s
```

顶层的 `Cargo.lock`，现在包含了有关 `add_one` 对 `rand` 的依赖信息。但是，即使 `rand` 在工作区中的某处被用到，除非把 `rand` 添加到其他代码箱的 `Cargo.toml` 文件，否则咱们就不能在其他代码箱中使用他。比如，若咱们把 `use rand;` 添加到 `adder` 包的 `adder/src/main.rs` 文件，咱们将得到一个报错：

```console
$ cargo build                                                                                 lennyp@vm-manjaro
   --跳过前面的告警--
   Compiling adder v0.1.0 (/home/lennyp/rust-lang/add/adder)
error[E0432]: unresolved import `rand`
 --> adder/src/main.rs:1:5
  |
1 | use rand;
  |     ^^^^ no external crate `rand`

For more information about this error, try `rustc --explain E0432`.
error: could not compile `adder` due to previous error
```

要修正这个错误，就也要编辑 `adder` 包的 `Cargo.toml` 文件，而表明 `rand` 是其依赖项。构建 `adder` 包就将把 `rand`，添加到 `Cargo.lock` 中 `adder` 的依赖项清单，但不会有额外的 `rand` 拷贝将被下载。Cargo 已确保工作区中，每个用到 `rand` 包的包中的每个代码箱，都将使用同一版本，从而给咱们节省空间，并确保工作区中的代码箱都将兼容于彼此。


## 添加测试到工作区

**Adding a Test to a Workspace**


为说明另一项改进，咱们来添加一个 `add_one` 代码箱里 `add_one::add_one` 函数的测试：

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
        let result = add_one(2);
        assert_eq!(result, 3);
    }
}
```

现在请于顶层的 `add` 目录中运行 `cargo test`。在像这样组织起来的工作区中，运行 `cargo test`，就会运行工作区中所有代码箱的测试：

```console
$ cargo test                                                                                                           lennyp@vm-manjaro
   Compiling add_one v0.1.0 (/home/lennyp/rust-lang/add/add_one)
   Compiling adder v0.1.0 (/home/lennyp/rust-lang/add/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.68s
     Running unittests src/lib.rs (target/debug/deps/add_one-837c2ad0efe6b80c)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/adder-2277ab1084738161)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

输出的首个部分，显示 `add_one` 代码箱中的 `it_works` 测试通过了。下一小节显示，在 `adder` 代码箱中找到零个测试，而随后的最后小节，显示在 `add_one` 代码箱中找到零个文档测试。（*注*：二进制代码箱中不会有文档测试？）

咱们还可通过使用 `-p` 命令行标志，并指明要测试的代码箱名字，而在顶层目录处运行工作区中特定代码箱的测试：


```console
$ cargo test -p add_one                                                                                                lennyp@vm-manjaro
    Finished test [unoptimized + debuginfo] target(s) in 0.01s
     Running unittests src/lib.rs (target/debug/deps/add_one-837c2ad0efe6b80c)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

此输出展示出，`cargo test` 只运行了 `add_one` 代码箱的测试，而未运行 `adder` 代码箱的测试。

若咱们把工作区中的代码箱发布到 `crates.io` ，工作区中的各个代码箱将需要被单独发布。与 `cargo test` 类似，咱们可通过使用 `-p` 命令行标志，并指明打算发布的代码箱名字，而发布工作区中的特定代码箱。

作为附加练习，请以与 `add_one` 代码箱类似方式，把 `add_two` 添加到这个工作区！

当咱们的项目日渐增长时，请考虑使用工作区：相比于一大块代码，要搞清楚较小的、单独的组件就更容易一些。再者，当代码箱经常同时被修改时，把这些代码箱保持在工作区中，就能令到他们之间的协作更容易。
