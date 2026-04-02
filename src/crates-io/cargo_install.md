# 以 `cargo install` 安装二进制代码箱

`cargo install` 命令允许咱们在本地安装和使用二进制代码箱。这并不是为了取代系统包；他的目的是为 Rust 开发者提供一种便捷的方式，来安装其他人在 [crates.io](https://crates.io) 上共享的工具。请注意，我们只能安装有着二进制目标的包。所谓 *二进制目标*，是在代码箱有着 `src/main.rs` 文件或指定为二进制代码箱的另一文件时创建的可运行程序，与本身不可运行而适合于包含在其他程序中的库目标相反。通常，代码箱在 `README` 文件中有着关于其是否是库，还是有着二进制目标，或者二者皆具方面的信息。

以 `cargo install` 安装的所有二进制代码箱，都存储在安装根目录下的 `bin` 文件夹中。当咱们是使用 `rustup.rs` 安装的 Rust，且没有任何定制配置时，那么这个目录将是 `$HOME/.cargo/bin`。为了能够运行咱们使用 `cargo install` 安装的程序，请确保这个目录在咱们的 `$PATH` 中。

例如，在第 12 章中我们曾提到，有个名为 `ripgrep` 的 `grep` 工具的 Rust 实现 `ripgrep`，用于检索文件。要安装 `ripgrep`，我们可以运行以下命令：

```console
$ cargo install ripgrep
    Updating crates.io index
  Installing ripgrep v13.0.0
  -- 跳过输出 --
   Compiling grep v0.2.10
   Compiling num_cpus v1.14.0
    Finished release [optimized + debuginfo] target(s) in 1m 09s
  Installing /home/lennyp/.cargo/bin/rg
   Installed package `ripgrep v13.0.0` (executable `rg`)
```

输出的倒数第二行显示已安装的二进制程序的位置与名字，在 `ripgrep` 的情形下为 `rg`。正如前面提到的，只要安装目录在咱们的 `$PATH` 中，随后咱们就可以运行 `rg --help`，并开始使用这个更快、更具 Rust 风格的工具来检索文件！

> **译注**：可在任意位置运行 `cargo install` 命令，安装 Crates.io 上的 Rust 二进制程序，这些程序都将被安装在 `$HOME/.cargo/bin` 下。若已安装了某个 Rust 程序后再安装他，那么就会有如下输出：
>
> ```console
> $ cargo install ripgrep
>    Updating `ustc` index
>      Ignored package `ripgrep v15.1.0` is already installed, use --force to override
> ```
>
> 或更新已安装的二进制代码箱。
>
> ```console
> $ cargo install ripgrep
>     Updating `ustc` index
> remote: Enumerating objects: 2145, done.
> remote: Counting objects: 100% (2145/2145), done.
> remote: Compressing objects: 100% (773/773), done.
> remote: Total 2145 (delta 1397), reused 2097 (delta 1349), pack-reused 0 (from 0)
> 接收对象中: 100% (2145/2145), 1.41 MiB | 1.71 MiB/s, 完成.
> 处理 delta 中: 100% (1397/1397), 完成 66 个本地对象.
> 来自 https://mirrors.ustc.edu.cn/crates.io-index
>  + aeafb51a4b2...dd78db21681 HEAD       -> origin/HEAD  (强制更新)
>   Downloaded ripgrep v15.1.0 (registry `ustc`)
>   Downloaded 1 crate (217.1KiB) in 2.66s
>   Installing ripgrep v15.1.0
>      Locking 46 packages to latest compatible versions
>   Downloaded anyhow v1.0.102 (registry `ustc`)
>   -- 跳过输出 --
>    Compiling grep v0.4.1
>     Finished `release` profile [optimized + debuginfo] target(s) in 25.11s
>   Installing /home/hector/.cargo/bin/rg
>    Installed package `ripgrep v15.1.0` (executable `rg`)
> ```


（End）
