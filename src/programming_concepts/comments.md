# 注释

所有程序员都会致力于让他们的代码易于理解，而有时是需要额外解释的。在这种情况下，程序员们就会在他们的源代码中，留下会被编译器忽略的 *注释（comments）*，而那些阅读到源代码的人会发现有用。

下面就是个简单的注释：

```rust
// hello, world
```
在 Rust 中，惯用的注释风格，是以双斜杠来开始一条注释，同时该注释会持续到那行的结束。对于那些超过一行的注释，在每行注释就都要包含 `//`，就像这样：

```rust
// 那么这里要编写一些复杂的注释，这注释长到要用多个行
// 才能写完！噢！还好，这条注释会解释接下来要做些什么。
```

注释也可以放在那些包含代码行的末尾：

文件名：`src/main.rs`

```rust
fn main() {
    let lucky_number = 7; // 今天我感受到了好运
}
```

不过更常见的则是以下面这种形式运用的注释，其中注释位处单独的、在其要注解代码之上的行：

文件名：`src/main.rs`

```rust
fn main() {
    // 今日感到幸运
    let lucky_number = 7;
}
```

Rust 还有另外一种注释，叫做文档注释，在第 14 章的 [将代码箱发布到 Crates.io](Ch14_More_about_Cargo_and_Crates.io.md#将代码箱发布到-cratesio) 中会对文档注释进行讨论。