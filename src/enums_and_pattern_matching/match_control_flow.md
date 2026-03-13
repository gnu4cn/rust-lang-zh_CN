# `match` 控制流结构

Rust 有着一种成为 `match` 的非常强大的控制流结构，允许咱们将某个值与一系列模式比较，然后根据匹配的模式执行代码。模式可以由字面值、变量名字、通配符及许多其他内容组成；[第 19 章](../Ch18_Patterns_and_Matching.md) 会涵盖所有不同类别的模式及其作用。`match` 的威力来自模式的表达能力以及编译器确认所有可能的情况都已处理这一事实。

> **译注**：Rust 的模式匹配借鉴了 [Erlang/OTP 语言](https://erl.xfoss.com/) 的特性，该门语言以模式匹配为基础。

请将 `match` 表达式想象成硬币分拣机：硬币在有大小不一孔洞的轨道上滑下，每枚硬币都会从他遇到的第一个适合的孔掉落。以同样的方式，值会经过 `match` 中的每种模式，并在该值 “合适” 的第一个模式处，落入在执行期间要用到的关联代码块中。

说到硬币，我们就来将他们用作使用 `match` 的示例！我们可编写一个函数，该函数取一枚未知的美国硬币，并以与点数机类似的方式确定他是哪种硬币，并返回其以美分为单位价值，如下清单 6-3 中所示。

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

<a name="listing_6-3"></a>
**清单 6-3**：一个枚举和一个以该枚举的变种作为模式的 `match` 表达式

我们来分解 `value_in_cents` 函数中的 `match` 表达式。首先，我们列出 `match` 关键字，后跟一个表达式，在本例中其为值 `coin`。这看起来与 `if` 下用到的条件表达式非常相似，但有个很大的区别：在 `if` 下，条件需要评估为布尔值，但在这里，他可以是任何类型。这个示例中 `coin` 的类型是我们在第一行上定义的 `Coin` 枚举。

接下来是 `match` 的支臂。支臂有两部分：模式与一些代码。这里的第一个支臂有种为值 `Coin::Penny` 的模式，然后是分隔模式与要运行代码的 `=>` 运算符。这一情形下的代码只是值 `1`。每个支臂与下一支臂之间以逗号隔开。

当 `match` 表达式执行时，他会按顺序将结果值与每个支臂的模式比较。在某一模式匹配该值时，与该模式关联的代码就会被执行。当该模式不匹配该值时，则继续执行下一支臂，就像硬币分选机一样。我们可以根据需要设置多个支臂：在清单 6-3 中，我们的 `match` 表达式有四个支臂。

与每个支臂关联的代码属于一个表达式，匹配支臂中表达式的结果值，便是由整个 `match` 表达式所返回的值。

当匹配支臂代码很短时，我们通常不会使用花括号，如同清单 6-3 中，每个支臂都只返回一个值。而当咱们打算在匹配支臂中运行多行代码时，咱们必须使用花括号，而该支臂后面的逗号此时是可选的。例如，以下代码会在每次以 `Coin::Penny` 调用该方法时打印 `"Lucky penny!"`，但他仍会返回该代码块的最后一个值 `1`：


```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println! ("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```


## 与值绑定的模式

`match` 支臂的另一项有用特性，是他们可以绑定到匹配模式的值的各部分。这是我们可从枚举变种中提取值的方式。

举个例子，我们来将咱们的枚举变种之一，修改为于其中保存数据。从 1999 年到 2008 年，美国为 50 个州都铸造了一侧有着不同图案的 25 分硬币。其他硬币都没有州的图案，因此只有 25 美分的有这种额外的价值。通过修改 `Quarter` 变种为包含一个存储于其内部的 `UsState` 值，我们便可将这一信息添加到咱们的 `enum`，我们在清单 6-4 中完成了这点。


```rust
#[derive(Debug)]    // 这样咱们就可以很快检查州份
enum UsState {
    Alabama,
    Alaska,
    // --跳过--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
```

<a name="listing_6-4"></a>
**清单 6-4**：其中 `Quarter` 变种还保存 `UsState` 值的枚举 `Coin`

设想某位朋友正在尝试收集所有 50 个州的 25 美分硬币。当我们按硬币类型分类咱们的零钱时，我们还将报出与每个 25 美分硬币相关的州名，以便其是个我们朋友没有的时，他们可将其添加到他们的收藏中。

在下面这段代码的匹配表达式中，我们将一个名为 `state` 的变量，添加到与 `Coin::Quarter` 变种的值匹配的模式。当某个 `Coin::Quarter` 匹配时，`state` 变量将将绑定到该 25 美分硬币的州份。然后，我们就可以在该支臂的代码中使用 `state`，就像这样：


```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println! ("来自 {:?} 州的 25 美分硬币！", state);
            25
        }
    }
}
```

若我们调用 `value_in_cents(Coin::Quarter(UsState::Alaska))`，那么 `coin` 将是 `Coin::Quarter(UsState::Alaska)`。当我们将该值与每个匹配支臂比较时，直到我们到达 `Coin::Quarter(state)` 为止都没有一个匹配。而在此处，`state` 的绑定将是值 `UsState::Alaska`。然后，我们就可以在 `println!` 表达式中使用该绑定，从而从 `Quarter` 的 `Coin` 枚举变种中取出内层的状态值。


## `Option<T>` 的匹配模式

在上一小节中，我们曾打算在使用 `Option<T>` 时取出 `Some` 情形下的内层 `T` 值；我们也可以使用 `match` 处理 `Option<T>`，就像我们对 `Coin` 枚举所做的那样！我们将比较 `Option<T>` 的变种，而不是比较硬币，但 `match` 表达式的工作方式保持不变。

假设我们打算编写一个取 `Option<i32>` 的函数，当内部有个值时，加 `1` 到该值；当内部没有值时，该函数应返回 `None` 值并且不尝试执行任何操作。

归功于 `match` 表达式，这个函数非常容易编写，并将看起来像下面清单 6-5 一样。


```rust
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
```

<a name="listing_6-5"></a>
*清单 6-5：对 `Option<i32>` 使用 `match` 表达式的一个函数*


我们来更详细地检查 `plus_one` 的第一次执行。当我们调用 `plus_one(five)` 时，`plus_one` 主体中的变量 `x` 将有着值 `Some(5)`。然后我们将其与每个匹配支臂比较：


```rust
        None => None,
```


值 `Some(5)` 不匹配模式 `None`，因此我们继续下一支臂：


```rust
        Some(i) => Some(i + 1),
```


`Some(5)` 匹配 `Some(i)` 吗？他确实匹配！我们有着同样的变种。`i` 会绑定到包含在 `Some` 中的值，因此 `i` 取得值 `5`。然后该匹配支臂中的代码被执行，因此咱们加 1 加到 `i` 的值并创建一个新的 `Some` 值，内部有着咱们的和 `6`。

现在我们看看清单 6-5 中 `plus_one` 的第二次调用，其中 `x` 为 `None`。我们进入 `match` 表达式并与第一个支臂比较：


```rust
        None => None,
```


他匹配！没有要相加的值，因此程序停止并返回 `=>` 右侧的 `None`。因为第一个支臂已匹配，因此没有其他支臂会被比较。

组合 `match` 与枚举在许多情形下都很有用。咱们将在 Rust 代码中看到很多这种模式：对枚举进行 `match`，将变量与内部数据绑定，并根据其执行代码。这在一开始有点棘手，而一旦咱们习惯这种模式，咱们就将希望咱们在所有语言中都有这种模式。这种模式一直都是用户的最爱。


## 匹配属于穷举性的

我们还需讨论 `match` 表达式的另一个方面：支臂的模式必须涵盖所有的可能性。请看下面这一版的咱们 `plus_one` 函数，他有个 bug 而将不编译：


```rust
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
        }
    }
```


我们没有处理 `None` 的情形，因此这段代码将造成一个 bug。幸运的是，这是个 Rust 知道如何捕捉的 bug。当我们尝试编译这段代码时，我们将得到下面这个报错：


```console
$ cargo run
   Compiling match_expr_demo v0.1.0 (/home/hector/rust-lang-zh_CN/projects/match_expr_demo)
error[E0004]: non-exhaustive patterns: `None` not covered
  --> src/main.rs:64:15
   |
64 |         match x {
   |               ^ pattern `None` not covered
   |
note: `Option<i32>` defined here
  --> /rustc/01f6ddf7588f42ae2d7eb0a2f21d44e8e96674cf/library/core/src/option.rs:600:1
  ::: /rustc/01f6ddf7588f42ae2d7eb0a2f21d44e8e96674cf/library/core/src/option.rs:604:5
   |
   = note: not covered
   = note: the matched value is of type `Option<i32>`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
   |
65 ~             Some(i) => Some(i + 1),
66 ~             None => todo!(),
   |

For more information about this error, try `rustc --explain E0004`.
error: could not compile `match_expr_demo` (bin "match_expr_demo") due to 1 previous error
```

Rust 知道我们没有涵盖所有可能的情况，甚至知道我们忘掉了哪种模式！Rust 中的匹配属于 *穷举性的*：为了代码有效，我们必须穷尽所有最后的可能性。特别是在 `Option<T>` 这种情况下，在 Rust 防止我们忘记要显式处理 `None` 情形后，他保护了我们免受在我们可能有个空值时，却假设我们有个值的影响，从而使早先讨论的数十亿美元的错误成为不可能。


## 全包模式与 `_` 占位符

使用枚举，我们还可以对少数特定值采取特殊操作，而对所有其他值采取一种默认操作。设想我们正在实现一个游戏，其中当咱们投出骰子上的 3 点时，咱们的玩家角色不移动，而会得到一顶漂亮的新帽子。当咱们掷出 7 点时，咱们的游戏角色会失去一顶漂亮帽子。对于所有别的点数，咱们的玩家角色都会在棋盘上移动相应数量的空格。下面是个实现这一逻辑的 `match` 表达式，其中骰子的结果被硬编码了而不是随机值，所有其他逻辑都以没有主体的函数表示，因为实际实现他们超出了这个示例的范围：

```rust
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
```


对于前两个支臂，模式为字面值 `3` 与 `7`。对于覆盖所有其他可能值的最后支臂，模式为我们选择命名为 `other` 的变量。针对 `other` 支臂运行的代码，通过将其传递给 `move_player` 函数使用该变量。

即使我们尚未列出 `u8` 会有的所有可能值，这段代码也会编译，因为最后那个模式将匹配所有未特别列出的值。这种全包模式满足 `match` 表达式必须详尽的要求。请注意，我们必须把全包支臂放在最后，因为模式是按顺序执行的。若我们将全包支臂放在前面，其他支臂将永远不会运行，因此当我们在全包支臂后添加支臂时 Rust 将警告我们！

Rust 还有一种当我们想要全包但又不打算 *使用* 全包模式中的值时，可以使用的模式： `_` 属于一种特殊模式，其匹配任何值但不绑定到该值。这告诉 Rust 我们不会使用该值，因此 Rust 将不就有个未使用的变量警告我们。

我们来改变游戏规则：现在，当我们投出 3 或 7 以外的任何点数时，咱们必须再投一次。我们不再需要使用全包值，因此我们修改咱们的代码为使用` _` 而不是名为 `other` 的变量：


```rust
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {}
```

这个示例同样满足穷举性要求，因为我们在最后那个支臂中显式地忽略了所有其他值；我们没有忘掉任何情况。

最后，我们将再一次修改游戏规则，从而若咱们投出 3 或 7 以外的任何点数时，轮到咱们时不发生任何其他事情。我们可通过使用单元值（我们曾在 [“元组类型”](../programming_concepts/data_types.md#元组类型) 小节中提到过的空元组类型）作为 `_` 支臂的代码来表达这一点：


```rust
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
```


在这里，我们显式地告诉 Rust，我们不会使用未与前面支臂中模式匹配的任何其他值，并且我们不打算在这一情形下运行任何代码。

我们将在 [第 19 章](../Ch18_Patterns_and_Matching.md) 中，介绍更多关于模式与匹配的内容。现在，我们将继续讨论 `if let` 语法，其在 `match` 表达式有点冗长的情况下非常有用。



（End）


