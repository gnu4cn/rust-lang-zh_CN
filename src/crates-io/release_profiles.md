# 使用不同发布配置文件，对构建进行定制

**Customizing Builds with Release Profiles**


在 Rust 中，所谓 *发布配置文件，release profiles*，是带有实现程序员对编译代码有着更多掌控的，一些预定义及可定制的配置文件。相对其他配置文件，每个配置文件都是被独立配置的。

Cargo 有两个主要发布配置文件：运行 `cargo build` 时 Cargo 用到的 `dev` 配置文件，与运行 `cargo build --release` 时 Cargo 用到的 `release` 配置文件。`dev` 配置文件被定义为有着用于开发的一些良好默认配置，而 `release` 配置文件有着用于发布构建的良好默认配置。

从咱们构建的输出中，这些配置文件名字或许不陌生：

```console
$ cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
$ cargo build --release
    Finished release [optimized] target(s) in 0.0s
```

其中 `dev` 及 `release`，即由编译器用到的不同配置文件。

Cargo 有着在咱们在项目的 `Cargo.toml` 文件中，未曾显式添加任何 `[profile.*]` 小节时，所适用的各个配置文件的默认设置。通过添加咱们打算定制的任何配置文件的 `[profile.*]` 小节，咱们就会覆盖掉默认设置的任何子集。比如，下面是 `dev` 与 `release` 配置文件中 `opt-level` 设置的默认值：

文件名：`Cargo.toml`

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

这个 `opt-level` 设置项，控制了 Rust 将应用到咱们代码的优化数目，有着范围 `0` 到 `3` 的取值范围。应用更多优化会延长编译时间，因此若咱们是在开发过程中而频繁编译代码，那么即使产生出的代码运行较慢，咱们也会想要更少的优化来更快地编译。因此默认的 `opt-level` 就是 `0`。而在咱们已准备好发布咱们的代码时，那么就最好用更多时间来编译。咱们将只以发布模式编译一次，但会运行编译好的程序许多次，因此发布模式就以较长的编译时间，换取到运行较快的代码。那就是 `release` 配置文件的 `opt-level` 默认为 `3` 的原因。

通过在 `Cargo.toml` 中，给某个默认值添加不同的值，就可以覆盖掉这个默认值。比如，在打算于开发配置文件中使用优化级别 `1` 时，就可以把下面这两行，添加到项目的 `Cargo.toml`：

文件名：`Cargo.toml`

```toml
[profile.dev]
opt-level = 1
```

此代码会覆盖默认设置 `0`。现在当咱们运行 `cargo build` 时，Cargo 将使用 `dev` 配置文件的默认设置，加上咱们对 `opt-level` 的定制。由于咱们把 `opt-level` 设置为了 `1`，Cargo 将应用相比于默认设置更多，但不如发布构建那样多的优化。

对于各个配置文件的完整配置项清单与默认设置，请参阅 [Cargo 文档](https://doc.rust-lang.org/cargo/reference/profiles.html)。


（End）


