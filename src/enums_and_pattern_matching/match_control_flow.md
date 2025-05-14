# `match` 控制流结构

**The `match` Control Flow Construct**


Rust 有种非常强大，允许咱们将某个值与一系列模式进行比较，然后根据匹配的模式执行代码，名为 `match` 的控制流结构。模式可由字面值、变量名、通配符及许多其他内容组成；第 19 章会涵盖到所有不同类型的模式以及他们的作用。`match` 的威力来自于模式的表现力，以及编译器会确认到所有可能的情况，都已得到处理这一事实。

请将某个 `match` 表达式，想象成一台硬币分拣机：硬币在一个有大小不一孔洞的轨道上滚下，每枚硬币都会从他遇到的第一个适合的孔出掉落。同样，值会穿过某个 `match` 表达式中的每种模式，并在该值 “合适” 的第一个模式处，落入关联代码块种，在执行期间被用到。

说到硬币，我们就来将他们用作一个用到 `match` 的示例！我们可以编写一个接收一枚未知美制硬币，并以类似于点钞机的方式，确定出他是哪一枚硬币，然后返回其价值（以美分为单位）的程序，如清单 6-3 所示。

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

*清单 6-3：一个枚举与一个以该枚举的变种作为模式的 `match` 表达式*

我们来分析一下 `value_in_cents` 这个函数中的 `match` 表达式。首先，我们列出了后跟一个表达式的 `match` 关键字，在本例中后跟的表达式为 `coin` 的值。这似乎跟与 `if` 一起使用的条件表达式非常相似，但有个很大的区别：在 `if` 下，条件需要评估为一个布尔值，但在这里，他可以是任何类型。本例中 `coin` 的类型，就是我们在第一行上定义的那个 `Coin` 枚举。

接下来是 `match` 的那些支臂。某个支臂有两部分：模式与一些代码。这里的第一个支臂，有个值 `Coin::Penny` 的模式，接着是分隔模式与要运行代码的 `=>` 运算符。该情形下代码只是值 `1`。每个支臂与下一支臂之间用逗号隔开。

当 `match` 表达式执行时，他会按顺序将结果值，与每个支臂的模式进行比较。在某个模式与该值匹配时，与该模式关联的代码就会被执行。在该模式与值不匹配时，则继续执行下一支臂，这与硬币分选机一样。我们可以按咱们所需，设置多个支臂：在清单 6-3 中，我们 `match` 表达式有四个支臂。

与每个支臂关联的代码，是个表达式，匹配支臂中表达式的结果值，就是整个 `match` 表达式所返回的值。


在匹配支臂代码很短时，我们通常不会使用花括号，就像清单 6-3 中，每个支臂都只返回一个值那样。而若咱们打算在某个匹配支臂中运行多行代码，咱们就必须使用花括号，同时匹配支臂后面的逗号此时是可选的。例如，下面的代码会在每次以 `Coin::Penny` 调用该方法时，都打印出 `"Lucky penny!"`，但仍会返回其中代码块的最后一个值 `1`：


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


## 绑定到值的模式

**Patterns that Bind to Values**


`match` 支臂的另一有用特性，是他们可以绑定到与该模式匹配值的部分。这就是我们从枚举变种中，提取值的方法。


举个例子，我们来将咱们的其中一个枚举变量，让他在内部保存数据。从 1999 年到 2008 年，美国为50 个州分别铸造了一面图案各不相同的 25 分硬币。其他硬币都没有州的图案，因此只有 25 美分硬币有这种额外的价值。通过修改 `Quarter` 变种为在其内部存储一个 `UsState` 值，我们就可以将这一信息添加到咱们的 `enum` 中，在清单 6-4 中我们已完成这一点。


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

*清单 6-4：其中 `Quarter` 变种还包含了个 `UsState` 值的 `Coin` 枚举*

设想某个朋友正努力收集所有 50 个州的硬币。在我们按照硬币种类，对咱们的零钱进行分类时，我们还将叫出与每个 25 美分硬币相关的州名，这样，在该硬币是我们的朋友没有的是，他们就可以将其添加到自己的收藏中。

在这段代码的匹配表达式中，我们会将一个名为 `state`，与变种 `Coin::Quarter` 的值匹配的变量，添加到该模式。当某个 `Coin::Quarter` 匹配时，`state` 这个变量将绑定到该 25 美分硬币的 `state` 值。然后，我们就可以在该支臂的代码中使用 `state` 了，就像这样：


```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println! ("来自 {:?} 州份的 25 美分硬币！", state);
            25
        }
    }
}
```

如果我们调用 `value_in_cents(Coin::Quarter(UsState::Alaska))`，那么 `coin` 将是 `Coin::Quarter(UsState::Alaska)`。当我们将该值与每个匹配支臂比较时，直到我们到达 `Coin::Quarter(state)` 没有一个支臂匹配 。此时，`state` 的绑定将是 `UsState::Alaska`。然后，我们就可以在 `println!` 表达式中使用该绑定，从而从 `Quarter` 这个 `Coin` 枚举变量中，获取到那个内部状态值。


## 匹配 `Option<T>`

**Matching with `Option<T>`**


在上一小节中，我们曾打算在使用 `Option<T>` 时，从 `Some` 情形中获取到内部 `T` 值；就像我们在处理 `Coin` 枚举时所做的那样，我们也可以使用 `match` 处理 `Option<T>`！我们将比较 `Option<T>` 的变种，而不是比较硬币，但 `match` 表达式的工作方式保持不变。


假设我们打算编写一个取个 `Option<i32>`，在其中有个值时，在该值上加 `1` 的函数。在里面没有值时，该函数应返回 `None` 值，并且不尝试执行任何操作。

得益于 `match` 表达式，这个函数非常容易编写，且看起来就像清单 6-5 一样。


```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
```

*清单 6-5：对某个 `Option<i32>` 运用 `match` 表达式的一个函数*


我们来仔细看看 `plus_one` 的第一次执行。当我们调用 `plus_one(five)` 时，`plus_one` 主体中的变量 `x` 的值为 `Some(5)`。然后，我们将其与每个支臂比较：


```rust
        None => None,
```


值 `Some(5)` 与模式 `None` 不匹配，因此我们继续下一支臂：


```rust
        Some(i) => Some(i + 1),
```


`Some(5)` 会匹配 `Some(i)` 吗？确实匹配！我们有着同样的变种。其中的 `i` 会绑定到包含于 `Some` 中的值，因此 `i` 会取得值 `5`。然后该匹配支臂中的代码会被执行，因此咱们会把 1 加到 `i` 的值，并创建出一个新的，内部有着咱们的和 `6` 的 `Some` 值。

现在我们来看看清单 6-5 中，`plus_one` 的第二次调用，其中 `x` 为 `None`。我们进入那个 `match` 表达式，并与第一支臂进行比较：


```rust
        None => None,
```


他匹配了！没有相加的值，因此程序停止，并返回 `=>` 右侧的 `None`。因为第一个支臂已经匹配，就不会比较其他支臂了。

在很多情况下，将 `match` 和枚举结合，都是很有用的。咱们会经常在 Rust 代码中，看到这种模式：对枚举 `match`，将某个变量与枚举中的数据绑定，然后基于他执行代码。这在一开始有点棘手，而一旦咱们习惯了，咱们就会希望在所有语言中，都能使用他。这一直是用户的最爱。


## 匹配要彻底

**Matches Are Exhaustive**


我们还需要讨论 `match` 表达式的另一方面：支臂的模式，必须涵盖所有可能性。请看下面这个版本的 `plus_one` 函数，其有着一个错误，而因此不会编译：


```rust
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
        }
    }
```


我们没有处理 `None` 的情况，因此这段代码会引起错误。幸运的是，这是个 Rust 知道如何捕捉的错误。如果我们尝试编译这段代码，我们将得到下面这个报错：


```console
$ cargo run
   Compiling match_demo v0.1.0 (C:\tools\msys64\home\Lenny.Peng\rust-lang-zh_CN\projects\match_demo)
error[E0004]: non-exhaustive patterns: `None` not covered
  --> src\main.rs:21:11
   |
21 |     match x {
   |           ^ pattern `None` not covered
   |
note: `Option<i32>` defined here
  --> /rustc/79e9716c980570bfd1f666e3b16ac583f0168962\library\core\src\option.rs:563:1
  ::: /rustc/79e9716c980570bfd1f666e3b16ac583f0168962\library\core\src\option.rs:567:5
   |
   = note: not covered
   = note: the matched value is of type `Option<i32>`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
   |
22 ~         Some(i) => Some(i + 1),
23 ~         None => todo!(),
   |

For more information about this error, try `rustc --explain E0004`.
error: could not compile `match_demo` (bin "match_demo") due to previous error
```


Rust 知道，我们没有涵盖所有可能的情况，甚至知道我们忘记了哪个模式！Rust 中的匹配是 *穷举性的，exhaustive*：我们必须穷举每一种可能，代码才能有效。特别是在 `Option<T>` 的情况下，在 Rust 防止我们忘记显式处理 `None` 情况时，他保护我们在我们可能有着空值时，不会错误假定我们有个值，从而使前面讨论的十亿美元错误，不可能发生。


## 全包模式与 `_` 占位符

**Catch-all Patterns and the `_` Placeholder**


使用枚举，我们还可以对少数特定值，采取特殊操作，而对所有其他值，采取默认操作。请试想一下，我们在实现某个游戏时，如果掷骰子掷出 3 点，玩家就不会移动，但会得到一顶新的花式帽子。如果掷出的骰子是 7 点，玩家就会失去一顶花式帽子。对于所有其他数值，咱们的玩家都会在游戏棋盘上，移动相应数量的空间。下面是个实现了上述逻辑的 `match` 表达式，其中掷骰子的结果，是硬编码而不是随机值，所有其他逻辑都用没有主体的函数表示，因为具体实现这些函数，不在本例的范围之内：


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


对于前两个臂，模式均为字面值 `3` 和 `7`。对于涵盖了所有其他可能值的最后支臂，其中的模式为我们选择命名为 `other` 的变量。为这个 `other` 支臂运行的代码，会通过将该变量传递给 `move_player` 函数，而使用该变量。

即使我们没有列出 `u8` 的所有可能值，这段代码也会编译，因为最后那个模式，将匹配所有未特别列出的值。这种全包模式，满足了 `match` 表达式必须穷举的要求。请注意，我们必须把这个全包支臂放在最后，因为模式是按顺序求值的。如果我们把这个全部支臂放在了前面，其他支臂就不会运行，所以如果我们在全包支臂后，添加了支臂，Rust 就会发出警告！

Rust 还提供了一种，在我们需要一个全包，但又不想 *使用* 全包模式中的值时，可以使用的模式：`_` 是个特殊的模式，他可以匹配任何值，但不会与该值绑定。这会告诉 Rust，我们不打算使用该值，因此 Rust 不会警告我们，有个未使用的变量。

我们来改变一下游戏规则：现在，如果掷出的不是 3 或 7 点，就必须再掷一次。我们不再需要使用那个全包值，因此我们把咱们的代码修改为使用 `_`，代替名为 `other` 的变量：


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


这个例子也符合穷举性要求，因为我们显式地忽略了最后一个支臂中，所有其他值；我们没有忘记任何可能性。

最后，我们再修改一次游戏规则，如果掷出的不是 3 或 7 点，则在咱们的回合中，不会发生任何其他事情。我们可以使用单元值（在 [元组类型](../programming_concepts/data_types.md#元组类型) 小节中曾提到过的空元组类型），作为 `_` 支臂的代码，来表达这一点：


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


在这里，我们显式地告诉 Rust，我们不会使用，任何与先前支臂中模式不匹配的其他值，且在这种情况下，我们不打算运行任何代码。

我们将在 [第 18 章](../Ch18_Patterns_and_Matching.md) 中，结束更多有关模式与匹配的内容。现在，我们将继续讨论 `if let` 这种语法，其在 `match` 表达式显得有些冗长的情况下，非常有用。


（End）


