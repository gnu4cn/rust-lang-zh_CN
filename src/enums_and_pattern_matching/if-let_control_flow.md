# 使用 `if let` 的简明控制流

**Concise Control Flow with `if let`**


`if let` 这种语法，可让咱们将 `if` 和 `let` 结合起来，以一种不那么冗长的方式，处理与一种模式匹配的值，而忽略其他值。请看下面清单 6-6 中，会匹配 `config_max` 变量中的一个 `Option<u8>` 值，但只在该值为 `Some` 变体时，才打算执行代码的程序。


```rust
    let config_max = Some(3u8);

    match config_max {
        Some(max) => println! ("The maximum is configured to be {}", max),
        _ => ()
    }
```

*清单 6-6：只关心值为 `Some` 时执行代码的 `match` 表达式*


如果值为 `Some`，我们会将该值绑定到模式中的变量 `max`，打印出 `Some` 变种中的值。我们不打算对 `None` 值，做任何处理。为了满足 `match` 表达式的要求，我们必须在仅处理一个变种后，加上 `_ => ()`，而这是很烦人的样板代码。

相反，我们可以使用 `if let`，以更简短的方式编写这段代码。以下代码的行为，与清单 6-6 中的 `match` 表达式相同：


```rust
    let config_max = Some(3u8);

    if let Some(max) = config_max {
        println! ("The maximum is configured to be {}", max);
    }
```


语法 `if let` 取以等号分隔的一个模式和表达式。其工作原理与 `match` 表达式相同，`if let` 中的表达式，会被给到 `match`，而其中的模式，便是 `match` 的首个支臂。在本例中，模式为 `Some(max)`，`max` 会绑定大 `Some` 中的值。我们随后便可以在 `if let` 代码块的正文中，以咱们曾在对应的  `match` 支臂中使用 `max` 的同样方式，使用 `max` 了。如果值与模式不匹配，这个 `if let` 代码块中的代码就不会运行。


> **译注**：`if let` 实际上是两部分，其中 `let Some(max) = config_max` 是个检验表达式 scrutinee expression。后面会看到类似的 `while let` 语法。


使用 `if let`，意味着更少输入、更少缩进和更少模板代码。但是，咱们会失去 `match` 表达式所带来的穷尽检查。在 `match` 和 `if let` 之间做出选择，取决于咱们在咱们的特定情况下，正在做什么；以及在失去穷尽检查的同时，是否能获得了简洁性。

换句话说，咱们可以把 `if let`，视为在值与一种模式匹配时执行代码，并在随后忽略所有其他值的 `match` 表达式的语法糖。

我们可以在 `if let` 中加入一个 `else`。与 `else` 搭配的代码块，与等同于这个 `if let` 与 `else` 的 `match` 表达式中，`_` 情况下的代码块相同。回想清单 6-4 中的 `Coin` 枚举定义，其中 `Quarter` 变种还包含了一个 `UsState` 值。如果我们想清点我们见到的所有非 25 美分硬币，同时还要公布 25 美分硬币的州份，我们本可以使用一个 `match` 表达式来完成这点，就像下面这样：


```rust
let mut count = 0;

match coin {
    Coin::Quarter(state) => println! ("State quarter from {:?}!", state),
    _ => count += 1,
}
```


或者，我们可以使用 `if let` 和 `else` 表达式，就像这样：


```rust
let mut count = 0;

if let Coin::Quarter(state) = coin {
    println! ("State quarter from {:?}!", state);
} else {
    count += 1;
}
```


如果咱们遇到咱们的程序中，有着使用 `match` 表示表达显得过于冗长的逻辑，那么请记住，`if let` 也在 Rust 工具箱中。


# 本章小结


现在我们已经介绍了如何使用枚举来创建出，可以是一组枚举值中的一个的自定义类型。我们已经展示了标准库的 `Option<T>` 类型，如何帮助咱们使用类型系统，来防止错误。当枚举值中包含数据时，根据咱们需要处理多少种情况，咱们可以使用 `match` 或 `if let`，来提取和使用这些值。

咱们的 Rust 程序现在可以使用结构体和枚举，来表达咱们领域中的概念了。在咱们的 API 中创建一些自定义类型，确保了类型安全：编译器将确保咱们的函数，只获取到每个他们所期望类型的值。

为了向咱们的用户，提供组织良好、简单易用的 API，并且只暴露出咱们用户所需的内容，我们现在来看看 Rust 的模组。


（End）


