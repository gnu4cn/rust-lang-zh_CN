# `if let` 与 `let else` 下的简明控制流

`if let` 语法允许咱们将 `if` 和 `let` 结合为一种不太冗长的方式来处理匹配一种模式匹配的值，同时忽略其余值。考虑下面清单 6-6 中的程序，其匹配 `config_max` 变量中的 `Option<u8>` 值，但只打算在该值为 `Some` 变体时执行代码。


```rust
    let config_max = Some(3u8);

    match config_max {
        Some(max) => println! ("The maximum is configured to be {max}"),
        _ => ()
    }
```

<a name="listing_6-6"></a>
**清单 6-6**：只关心在值为 `Some` 时执行代码的 `match` 表达式


当值为 `Some` 时，我们通过绑定值到模式中的变量 `max` 打印出 `Some` 变种中的值。我们不打算对 `None` 值做任何事情。为了满足 `match` 表达式（穷尽要求），我们不得不在仅处理一个变种后就添加 `_ => ()`，而这属于令人讨厌的样板代码。

相反，我们可以使用 `if let` 以更简短的方式编写这段代码。以下代码与清单 6-6 中 `match` 表达式行为相同：


```rust
    let config_max = Some(3u8);

    if let Some(max) = config_max {
        println! ("The maximum is configured to be {max}");
    }
```


语法 `if let` 取以等号分隔的一个模式与一个表达式。其工作方式与 `match` 表达式相同，其中表达式被提供给 `match` 而模式为他的第一个支臂。在本例中，模式为 `Some(max)`，而 `max` 绑定到 `Some` 内的值。然后，我们可在 `if let` 代码块的主体中使用 `max`，以咱们在对应  `match` 支臂中使用 `max` 的同样方式。`if let` 代码块中的代码只会在值与模式匹配时运行。


> **译注**：
>
> - `if let` 实际上是两部分，其中 `let Some(max) = config_max` 是个检验表达式，scrutinee expression。后面会看到类似的 `while let` 语法；
>
> - 按照 Erlang/OTP 下的思路，这里的 `=` 是个模式匹配运算符，而不是赋值。


使用 `if let` 意味着更少的键入、更少的缩进及更少的样板代码。但是，咱们将失去 `match` 表达式强制执行的穷尽检查，其确保咱们没有忘记处理任何情况。在 `match` 和 `if let` 之间选择取决于咱们在特定情况下正在做什么，以及是否认为获得简洁性是值得牺牲穷尽检查的合理权衡。

换句话说，咱们可把 `if let` 视为 `match` 表达式的语法糖，当值与一种模式匹配时运行代码，然后忽略所有别的值。

我们可在 `if let` 下包含 `else`。`else` 下的代码块与等同于这个 `if let` 与 `else` 的 `match` 表达式中，`_` 情况下的代码块相同。回顾 [清单 6-4](./match_control_flow.md#listing_6-4) 中 `Coin` 枚举的定义，其中 `Quarter` 变种还保存了个 `UsState` 值。若我们打算清点我们看到的所有非 25 美分硬币，同时还要喊出 25 美分硬币的州份，我们本可以一个 `match` 表达式实现，就像下面这样：

```rust
    let mut count = 0;

    match coin {
        Coin::Quarter(state) => println! ("State quarter from {state:?}!"),
        _ => count += 1,
    }
```


或者我们可以使用 `if let` 与 `else` 表达式，就像这样：


```rust
    let mut count = 0;

    if let Coin::Quarter(state) = coin {
        println! ("State quarter from {state:?}!");
    } else {
        count += 1;
    }
```


## 在 `let...else` 下留在 “快乐小道”

**Staying on the "Happy Path" with `let...else`**


常见模式是当值存在时执行一些计算，否则返回默认值。继续我们的带有 `UsState` 值的硬币示例，若我们打算根据 25 美分硬币上的州份年代说些有趣的话，我们就可能在 `UsState` 上引入一个方法来检查州份的年代，就像这样：


```rust
impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // -- snip --
        }
    }
}
```

然后，我们可以使用 `if let` 来匹配硬币的类型，在条件的主体中引入一个 `state` 变量，如下面清单 6-7 中所示。

文件名：`src/main.rs`

```rust
fn describe_state_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
}
```

<a name="listing_6-7"></a>
**清单 6-7**：使用嵌套在 `if let` 中的条件，检查某个州在 1900 时是否存在

这样就完成了工作，但他已将工作推入了 `if let` 语句的主体中，当要完成的工作更复杂时，就可能很难准确理解顶级分支之间的关系。我们还可以利用表达式会产生值这一事实，要么从 `if let` 产生 `state` 要么提前返回，如清单 6-8 中所示。(在 `match` 下咱们也可实现类似效果。）


文件名：`src/main.rs`

```rust
fn describe_state_quarter(coin: Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}
```

<a name="listing_6-8"></a>
**清单 6-8**：使用 `if let` 产生值，或提前返回*


不过，这样做本身就有点烦人！`if let` 的一个分支会产生值，而另一分支会整个地从函数返回。

为了使这种常见模式更易于表达，Rust 提供了 `let...else`。`let...else` 语法取左侧的一个模式和右侧的一个表达式，与 `if let` 非常相似，但他没有 `if` 分支，而只有 `else` 分支。当模式匹配时，他将在外层作用域中绑定模式中的值。当模式 *不* 匹配时，程序将流入 `else` 支臂，其必须从函数返回。

在下面清单 6-9 中，咱们会看到在 `if let` 处使用 `let...else` 时清单 6-8 的样子。

文件名：`src/main.rs`

```rust
fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}
```

<a name="listing_6-9"></a>
**清单 6-9**：使用 `let...else` 来明确函数的流程

请注意，流程以这种方式保持在函数主体中的 “快乐路径” 上，而没有 `if let` 所做的那样，针对两个分支有着明显不同的控制流。

当咱们遇到咱们的程序有着过于冗长的逻辑，无法使用 `match` 表达的情况时，那么请记住 `if let` 及 `let...else` 也在咱们的 Rust 工具箱中。


# 本章小结


我们现在已经介绍了如何使用枚举来创建可以是一组枚举值之一的自定义类型。我们已经展示了标准库的 `Option<T>` 类型如何帮助咱们使用类型系统来防止错误。当枚举值内包含数据时，咱们可使用 `match` 或 `if let` 来提取并使用这些值，具体取决于咱们需要处理多少的情况。

咱们的 Rust 程序现在可以使用结构体和枚举来表达咱们领域中的概念。创建在咱们 API 中使用的自定义类型可确保类型安全：编译器将确保咱们的函数，只会得到各个函数期望类型的值。

为了向咱们的用户提供组织良好、简单易用，且仅暴露出咱们用户所需的 API，我们现在来看看 Rust 的模组。


（End）


