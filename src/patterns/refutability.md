# 可证伪性：模式是否会匹配失败

模式有两种形式：可证伪与不可证伪的。对于任何可能传递的值都匹配的模式，属于 *不可证伪的，irrefuatable*。例如，语句 `let x = 5;` 中的 `x`，因为 `x` 会匹配任何值，因此不可能匹配失败。对于某些可能的值可能匹配失败的模式，属于 *可证伪的，refutable*。例如，表达式 `if let Some(x) = a_value` 中的 `Some(x)`，因为当 `a_value` 中的值为 `None` 而非 `Some` 时，`Some(x)` 模式将不匹配。

函数参数、`let` 语句及 `for` 循环只能接受不可证伪的模式，因为当值不匹配时程序无法执行任何有意义的操作。`if let` 与 `while let` 表达式，以及 `let...else` 语句，接受可证伪和不可证伪的模式，但编译器会对不可证伪模式发出告警，因为根据定义，他们意图处理可能的失败：条件的功能在于其能够根据成功或失败，而以不同方式执行的能力。

一般来说，咱们不必担心可证伪与不可证伪模式的区别；但是，咱们确实需要熟悉可证伪性的概念，以便在在报错消息中看到时可以予以响应。在这种情形下，咱们需要根据代码的预期行为，修改模式或者与模式一起使用的结构。

我们来通过一个实例看看，当我们尝试在 Rust 要求使用不可证伪模式的地方使用可证伪模式，或者反之，会发生什么。下面清单 19-8 展示了一个 `let` 语句，但我们对模式指定了个可证伪的模式 `Some(x)`。正如咱们所料，这段代码将不编译。

<a name="listing_19-8"></a>
```rust
    let Some(x) = some_option_value;
```

**清单 19-8**：尝试对 `let` 使用可证伪模式

当 `some_option_value` 是个 `None` 值时，他将与模式 `Some(x)` 匹配失败，这意味着该模式是可证伪的。但是，`let` 语句只能接受不可证伪模式，因为没有可处理 `None` 值的有效代码。在编译时，Rust 将抱怨我们试图在要求不可证伪模式的地方使用可证伪模式：

```console
$ cargo run
   Compiling patterns v0.1.0 (/home/hector/rust-lang-zh_CN/projects/patterns)
error[E0005]: refutable pattern in local binding
 --> src/main.rs:3:9
  |
3 |     let Some(x) = some_option_value;
  |         ^^^^^^^ pattern `None` not covered
  |
  = note: `let` bindings require an "irrefutable pattern", like a `struct` or an `enum` with only one variant
  = note: for more information, visit https://doc.rust-lang.org/book/ch19-02-refutability.html
  = note: the matched value is of type `Option<i32>`
help: you might want to use `let...else` to handle the variant that isn't matched
  |
3 |     let Some(x) = some_option_value else { todo!() };
  |                                     ++++++++++++++++

For more information about this error, try `rustc --explain E0005`.
error: could not compile `patterns` (bin "patterns") due to 1 previous error
```

由于我们没有以模式 `Some(x)` 覆盖（也无法涵盖！）所有有效值，Rust 理所当然地会产生编译器报错。

当我们在需要不可证伪模式的地方使用了可证伪模式时，可以通过修改使用该模式的代码来修复：我们可以使用 `let...else`，而不是使用 `let`。然后，当模式不匹配时，花括号中的代码就会处理该值。下面清单 19-9 展示了如何修复清单 19-8 中的代码。


```rust
    let Some(x) = some_option_value else {
        return;
    };
```

**清单 19-9**：对可证伪模式使用 `let...else` 和一个代码块，而非 `let`

我们给予了代码一种退出条件！这段代码完全有效，尽管这意味着我们无法在没有收到告警的情况下，使用不可证伪模式。当我们给予 `let...else` 某个始终匹配的模式时，如下清单 19-10 中所示，编译器将给出告警。

```rust
    let x = 5 else {
        return;
    };
```

**清单 19-10**：尝试对 `let...else` 使用不可证伪模式

Rust 会抱怨，对 `let...else` 使用不可证伪模式没有意义：

```console
$ cargo run
   Compiling patterns v0.1.0 (/home/hector/rust-lang-zh_CN/projects/patterns)
warning: irrefutable `let...else` pattern
 --> src/main.rs:2:5
  |
2 |     let x = 5 else {
  |     ^^^^^^^^^
  |
  = note: this pattern will always match, so the `else` clause is useless
  = help: consider removing the `else` clause
  = note: `#[warn(irrefutable_let_patterns)]` on by default

warning: `patterns` (bin "patterns") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running `target/debug/patterns`
```

因此，匹配支臂必须使用可证伪模式，除了最后支臂外，他应以不可证伪模式匹配任何剩余的值。Rust 允许我们在仅有一个支臂的 `match` 中使用不可证伪模式，但这种语法并不是特别有用，可以更简单的 `let` 语句替换。

现在咱们知道了哪里使用模式，以及可证伪与不可证伪模式的区别，我们来介绍可用于创建模式的所有语法。


（End）


