# 以定制命令扩展 Cargo

Cargo 被设计为咱们可以新的子命令，在不必修改他的情形下就可以扩展他。当咱们的 `$PATH` 中的某个二进制程序名为 `cargo-something` 时，咱们可以通过运行 `cargo something` 来运行他，就像他是 Cargo 的子命令一样。像这样的定制命令，在咱们运行 `cargo --list` 时也会被列出。能够使用 `cargo install` 安装扩展，然后像内置的 Cargo 工具一样运行他们，是 Cargo 设计的一个超级便利的好处！


# 本章小结

通过 Cargo 和 [crates.io](https://crates.io) 分享代码，正是让 Rust 生态对于许多不同任务有用的因素之一。Rust 的标准库小而稳定，但代码箱在不同于语言的时间线上，却易于共享、使用和改进。请不要羞于在 [crates.io](https://crates.io) 上分享对咱们有用的代码；这些代码很可能对其他人也有用！


（End）


