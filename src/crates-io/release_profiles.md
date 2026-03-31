# 在发布配置文件下定制构建

在 Rust 中，所谓 *发布配置文件，release profiles*，是一些预定义的、定制的配置文件，带有允许程序员对编译代码的各种有着更多控制的不同选项。每个配置文件都是相对于其他配置文件独立配置的。

Cargo 有两个主要配置文件：

- 当咱们运行 `cargo build` 时 Cargo 使用的 `dev` 配置文件、
- 以及当咱们运行 `cargo build --release` 时 Cargo 用使用的 `release` 配置文件。

`dev` 配置文件定义了用于开发的一些良好默认配置，`release` 配置文件则有着一些用于发布构建的良好默认配置。

这两个配置文件名字，可能在咱们构建的输出中很熟悉：

```console
$ cargo build
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
$ cargo build --release
    Finished `release` profile [optimized] target(s) in 0.32s
```

其中的 `dev` 及 `release` 便是编译器用到的两种不同配置文件。

Cargo 对每种配置文件都有一些默认配置，应用于咱们未曾在项目的 `Cargo.toml` 文件中显式地添加任何 `[profile.*]` 小节时。通过针对咱们打算定制的配置文件的 `[profile.*]` 小节，咱们可以覆盖默认设置中的任何子集。例如，下面是 `dev` 与 `release` 配置文件的 `opt-level` 设置的默认值：

文件名：`Cargo.toml`

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

`opt-level` 设置控制 Rust 将应用到咱们代码的优化数量，范围为 0 到 3。应用更多优化会延长编译时间，因此当咱们处于开发阶段而频繁编译代码时，那么即使生成的代码运行较慢，咱们也会希望更少的优化来编译更快。因此默认的 `opt-level` 是 `0`。当咱们准备好发布代码时，那么最好花更多时间编译。咱们将只以发布模式编译一次，但会运行编译好的程序许多次，因此发布模式以更长的编译时间换取运行更快的代码。这就是为什么 `release` 配置文件的 `opt-level` 默认为 `3`。

咱们可以通过在 `Cargo.toml` 中为默认设置添加不同值来覆盖默认设置。例如，当我们打算在开发配置文件中使用优化级别 1 时，我们可以添加下面这两行到项目的 `Cargo.toml` 文件：

文件名：`Cargo.toml`

```toml
[profile.dev]
opt-level = 1
```

这段代码会覆盖默认设置 `0`。现在，当我们运行 `cargo build` 时，Cargo 将使用 `dev` 配置文件的默认设置以及我们对 `opt-level` 的定制设置。由于我们设置 `opt-level` 为 `1`，Cargo 将应用相比默认设置更多的优化，但不如发布构建下的那么多。

针对每种配置文件的完整配置选项和默认设置清单，请参阅 [Cargo 文档](https://doc.rust-lang.org/cargo/reference/profiles.html)。


（End）


