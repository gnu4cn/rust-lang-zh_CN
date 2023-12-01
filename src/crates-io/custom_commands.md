# 使用定制命令扩展 Cargo

**Extending Cargo with Custom Commands**

Cargo 被设计为在无需修改 Cargo 下，咱们就可以使用新的子命令，对其加以扩展。若咱们的 `$PATH` 中有名为 `cargo-something` 的二进制程序，咱们便可通过运行 `cargo something`，将其作为 Cargo 的子命令运行。像这样的定制命令，还会在咱们运行 `cargo --list` 时给列出来。使用 `cargo install` 安装扩展，并随后跟运行内建的 Cargo 工具一样运行他们的这种能力，正是 Cargo 设计的一项超级便利的好处！


# 本章小结

运用 Cargo 与 [crates.io](https://crates.io) 分享代码，是令到 Rust 生态对于许多不同任务都有用的一个方面。Rust 的标准库是小型且稳定的，但在不同于语言本身的时间线上，代码箱则易于共享、运用以及改进。请不要羞于在 [crates.io](https://crates.io) 上分享对自己有用的代码；那些代码或许对其他人也同样有用！
