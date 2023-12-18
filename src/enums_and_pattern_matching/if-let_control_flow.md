# 使用 `if let` 的简明控制流

**Concise Control Flow with `if let`**


`if let` 这种语法，可让咱们将 `if` 和 `let` 结合起来，以一种不那么冗长的方式，处理与一种模式匹配的值，而忽略其他值。请看下面清单 6-6 中，会匹配 `config_max` 变量中的一个 `Option<u8>` 值，但只在该值为 `Some` 变体时，才打算执行代码的程序。


```rust
    let config_max = Some(3u8);

    match config_max {
        Some(max) => println! ("极大值被配置为了 {}"， max);
        _ => ();
    }
```

*清单 6-6：只关心值为 `Some` 时执行代码的 `match` 表达式*


在该值为 `Some` 时，这里就通过将那个 `Some` 变种中的值，绑定到这个模式中的变量 `max`，而打印出该值来。这里并不想要对那个 `None` 值做什么操作。为满足 `match` 表达式的要求，这里必须在处理仅仅一个变种之后，添加 `_ => ()`，这就是要添加的恼人样板代码。

相反，可使用 `if let` 语法，以较简短方式写出来。下面的代码与清单 6-6 中的 `match` 表达式表现一致：

```rust
    let config_max = Some(3u8);

    if let Some(max) = config_max {
        println! ("极大值被设置为了 {}", max);
    }
```

`if let` 语法会接收由等号分隔的一个模式与一个表达式。他与 `match` 原理相同，其中的表达式被给到 `match` 表达式，而其中的模式就是 `match` 表达式的第一支臂。在此示例中，模式即为 `Some(max)`，而这个 `max` 就绑定到了 `Some` 里面的那个值。由此，这里随后就可以与在相应的 `match` 支臂中使用 `max` 的同样方式，在后面的那个 `if let` 代码块中对 `max` 进行使用。而在该值 `config_max` 不与该模式匹配时，那个 `if let` 代码块中的代码，就不会运行。

> ***注***：`if let` 实际上是两部分，其中 `let Some(max) = config_max` 是个 scrutinee expression。

使用 `if let` 语法，就意味着较少输入、较少的缩进，以及更少的样板代码。不过会损失 `match` 表达式强制要求的穷尽检查。是根据特定情形下，手头正在做的事情，在 `match` 表达式与 `if let` 语法之间加以选择的，以及考量为收获到简洁，而是否值得损失穷尽性检查。

也就是说，可将 `if let` 语法当作，在值与某个模式匹配时运行代码，并在之后忽略所有其他值的 `match` 表达式的语法糖（in other words, you can think of `if let` as syntax sugar for a `match` that runs code when the value matches one pattern and then ignores all other values）。

这里可以在 `if let` 之下，包含一个 `else` 关键字。`else` 所带的代码块，与在和 `if let` 及 `else` 等价的 `match` 表达式中， `_` 情形所带代码块相同。回想起清单 6-4 中的那个 `Coin` 枚举定义，其中的 `Quarter` 变种还有一个 `UsState` 值。在要通告出那些 25 美分硬币的州份的同时，还要清点出找到的全部非 25 美分数目，那么就可以使用下面这样的 `match` 表达式：

```rust
let mut count = 0;

match coin {
    Coin::Quarter(state) => println! ("这是来自州份 {:?} 的 25 美分硬币！", state),
    _ => count += 1,
}
```

或者这里还可以使用一个像下面这样的 `if let` 与 `else` 的表达式：

```rust
let mut count = 0;

if let Coin::Quarter(state) = coin {
    println! ("这是来自州份 {:?} 的 25 美分硬币！", state);
} else {
    count += 1;
}
```

在遇到程序中使用 `match` 显得太过繁复的逻辑这样情形时，就要记住在 Rust 工具箱中还有 `if let`语法呢。


# 总结

本章已经讲过，怎样运用枚举，来创建可作为一套一一列出数值之一的定制类型。这里给出了标准库的 `Option<T>` 类型，是怎样在运用该类型下，防止代码错误的原理。在枚举值有着内部值时，根据所要处理的多少种情况，而可使用 `match` 表达式或 `if let` 语法，来提取并使用这些值。

现在的 Rust 程序，就可以使用结构体与枚举，对所在领域的那些概念加以表达了。通过在自己构建的 API 使用的定制类型，而确保了类型安全：Rust 编译器将令到 API 中的那些函数，只获取到这些函数所期望类型的那些值。

而为了将可直接使用上的、组织良好的 API 提供到用户，并只暴露 API 的用户所需要部分，那么就要了解一下 Rust 的模组特性了。
