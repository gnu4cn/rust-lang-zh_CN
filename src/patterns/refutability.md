# 可证伪性：某个模式有无可能匹配失败

**Refutability: Whether a Pattern Might Fail to Match**


模式有两种形式：可证伪与不可证伪的。将匹配所传递的任何可能值模式，即为 *不可证伪的，irrefuatable*。一个示例即为 `let x = 5;` 语句中的 `x`；由于 `x` 会匹配任何值，而因此必定不会匹配失败。那些对某些可能的值，会匹配失败的模式，便是 *可证伪的，refutable*。这样的一个示例，便是表达式 `if let Some(x) = a_value` 中的 `Some(x)`，因为若 `a_value` 中的值为 `None` 而非 `Some` 时，这个 `Some(x)` 模式就将不匹配。

函数参数、`let` 语句及 `for` 循环，就只能接受不可证伪的模式，这是由于这些情况下，当值不匹配时，程序便无法执行任何有意义的事情。`if let` 与 `while let` 表达式，接受可证伪与不可证伪的模式，但由于根据定义，他们被计划来处理可能的匹配失败：某个条件的功能，便在于其根据匹配成功或失败，而区别执行的能力，因此编译器会就不可证伪模式发出告警。

一般来说，咱们无须担心可证伪与不可证伪模式的区别；但是，咱们确实需要熟悉可证伪性的概念，这样咱们在看到报错消息时，就可以予以响应。在这些情形下，咱们将需要根据代码所预期的行为，而要么修改模式，或者修改该模式下用到的那个结构。

接下来就看看，当咱们在 Rust 要求使用不可证伪模式的地方，尝试使用某种可证伪模式，及反过来 Rust 要求使用可证伪模式，而尝试使用不可证伪模式时，会发生什么。下面清单 18-8 给出了一个 `let` 语句，不过咱们指定了一个可证伪的 `Some(x)` 模式。正如你会期待的那样，此代码将不会编译。

```rust
    let Some(x) = some_option_value;
```

*清单 18-*：在 `let` 下使用可证伪模式的尝试*


若 `some_option_value` 是个 `None` 值，他就会与模式 `Some(x)` 匹配失败，意味着该模式为可证伪的。但是，由于此处没有可处理 `None` 值的有效代码，因此该 `let` 表达式就只能接收某个不可证伪的模式。在编译时，Rust 将抱怨说咱们曾于要求不可证伪模式的某处，使用了可证伪模式：

```console
$ cargo run
   Compiling patterns v0.1.0 (file:///projects/patterns)
error[E0005]: refutable pattern in local binding: `None` not covered
   --> src/main.rs:3:9
    |
3   |     let Some(x) = some_option_value;
    |         ^^^^^^^ pattern `None` not covered
    |
    = note: `let` bindings require an "irrefutable pattern", like a `struct` or an `enum` with only one variant
    = note: for more information, visit https://doc.rust-lang.org/book/ch18-02-refutability.html
note: `Option<i32>` defined here
    = note: the matched value is of type `Option<i32>`
help: you might want to use `if let` to ignore the variant that isn't matched
    |
3   |     let x = if let Some(x) = some_option_value { x } else { todo!() };
    |     ++++++++++                                 ++++++++++++++++++++++

For more information about this error, try `rustc --explain E0005`.
error: could not compile `patterns` due to previous error
```

由于咱们不曾以 `Some(x)` 涵盖（且无法涵盖到！）所以有效值，Rust 便理所当然地产生了一个编译器报错。

而当咱们在需要不可证伪模式处，有着某个可证伪模式时，咱们可通过修改用到该模式的代码修复之：与其使用 `let`，咱们可以使用 `if let`。随后在该模式不匹配时，该代码就仅仅会跳过位于那花括号中代码，从而给了其有效继续的一种方式。下面清单 18-9 给出了修复清单 18-8 中代码的方式。


```rust
    if let Some(x) = some_option_value {
        println! ("{}", x);
    }
```

*清单 18-9：使用 `if let` 与带有可证伪模式代码块，而非 `let`*

咱们就已给了代码一条出路了！这段代码是完全有效的，尽管其意味着咱们在不接收到报错下，无法使用某个不可证伪模式。而在咱们给到 `if let` 某个将始终匹配的模式时，如下清单 18-10 中所示，编译器就会给出一条告警。

```rust
    if let x = 5 {
        println! ("{}", x);
    }
```

*清单 18-10：在 `if let` 下使用不可证伪模式的尝试*

Rust 会抱怨，以某个不可证伪模式使用 `if let` 没有意义：

```console
$ cargo run                                                                                                                lennyp@vm-manjaro
   Compiling refutable_demo v0.1.0 (/home/lennyp/rust-lang/refutable_demo)
warning: irrefutable `if let` pattern
 --> src/main.rs:2:8
  |
2 |     if let x = 5 {
  |        ^^^^^^^^^
  |
  = note: this pattern will always match, so the `if let` is useless
  = help: consider replacing the `if let` with a `let`
  = note: `#[warn(irrefutable_let_patterns)]` on by default

warning: `refutable_demo` (bin "refutable_demo") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.59s
     Running `target/debug/refutable_demo`
5
```

由于这个原因，除了应以一个不可证伪模式匹配全部剩余值的最后支臂外，其他那些匹配支臂，就必须使用可证伪模式。Rust 允许咱们在仅有一个支臂的 `match` 中，使用不可证伪模式，但这种语法不是特别有用，并可以一个更简单的 `let` 语句替换。

现在，咱们就知道了哪些地方要使用模式，以及可证伪与不可证伪模式的区别，下面就来介绍所有可用于创建模式的语法。


（End）


