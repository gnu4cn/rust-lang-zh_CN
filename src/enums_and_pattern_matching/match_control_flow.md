# `match` 控制流结构

**The `match` Control Flow Construct**


Rust 有一种名为 `match` 的非常强大的控制流结构，他允许咱们，将某个值与一系列模式比较，然后根据匹配的模式执行代码。模式可以由字面值、变量名、通配符及许多其他内容组成；[第 18 章](../Ch18_Patterns_and_Matching.md) 将介绍所有不同种类的模式及其作用。`match` 的威力，来自于模式的表现力，以及编译器确认所有可能情况都已得到处理，这一事实。

请把 `match` 表达式，设想成一台硬币分拣机：硬币沿着带有大小不一孔的轨道滑下，每枚硬币都会从他遇到的第一个适合的孔中落下。同样，值会遍历 `match` 表达式中的每个模式，在值 “适合” 的第一个模式处，值会落入关联的代码块，以便在执行过程中使用。


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

*清单 6-3：一个枚举和一个以该枚举的变种为模式的 `match` 表达式*


我们来分析一下 `value_in_cents` 函数中的那个 `match` 表达式。首先，我们列出了后跟一个表达式（本例中即为 `coin` 这个值）的 `match` 关键字。这似乎与 `if` 中使用的条件表达式非常相似，但有个很大的区别：在 `if` 中，条件需要求值为某个布尔值，但在这里，则可以是任何类型。本例中的 `coin` 类型，就是我们在第一行定义的那个 `Coin` 枚举。

接下来是那些 `match` 的支臂。支臂由两部分组成：模式与一些代码。这里的第一个支臂有着值 `Coin::Penny` 的模式，然后是分隔模式和要运行代码的 `=>` 操作符。此情形下的代码只是值 `1`。每个支臂之间，用逗号隔开。

当 `match` 表达式执行时，他会将 `match` 关键字后表达式的结果值，按顺序地与每个支臂的模式进行比较。如果某个模式与该值匹配，则执行与该模式相关的代码。如果该模式与该值不匹配，则继续执行下一支臂，就像硬币分拣机一样。我们可以根据需要，有着任意多个支臂：在清单 6-3 中，我们的 `match` 表达式，就有四个支臂。

与每个支臂相关的代码，是个表达式，匹配的支臂中表达式的结果值，就是整个 `match` 表达式返回的值。

如果匹配支臂代码很短，就像清单 6-3 中，每个匹配臂只返回一个值那样，我们通常不会使用花括号。如果要在某个匹配支臂中运行多行代码，则必须使用花括号，且匹配支臂后面的那个逗号，此时便成为可选的了。例如，下面的代码会在每次以 `Coin::Penny` 调用该函数时，打印出 "Lucky penny!"，但仍会返回那个代码块的最后值 `1`：


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


`match` 支臂的另一有用特性是，他们可以绑定到匹配该模式的值的部分。这就是我们从枚举变种中，提取值的方法。

举个例子，我们来改变 `Coin` 这个枚举中的某个枚举变种，在其内部保存数据。从 1999 年到 2008 年，美国为 50 个州铸造了其中一面图案各不相同的 25 美分硬币。其他硬币都没有州的图案，因此只有 25 美分硬币，有着这种额外价值。我们可以通过更改 `Quarter` 变量，包含一个存储在其中的 `UsState` 值，具体做法见下面清单 6-4。


```rust
#[derive(Debug)]    // 这样就可以很快检查州份
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

*清单 6-4：其中 `Quarter` 变量还包含一个 `UsState` 值的 `Coin` 枚举*


设想某位朋友正在努力收集 50 个州的 25 美分硬币。当我们按照硬币种类，对零钱进行分类时，我们还会喊出与每个 25 美分硬币相关的州名，这样，如果我们的朋友没有这个硬币，他们就可以将其添加到自己的收藏中。

在这段代码的 `match` 表达式中，我们就要在匹配 `Coin::Quarter` 变种值的模式中，添加一个名为 `state` 的变量。当某个 `Coin::Quarter` 匹配时，这个 `state` 变量，就会绑定到那个 25 美分银币的州份。然后，我们就可以在该支臂的代码中，使用 `state` 了，就像下面这样：


```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state: UsState) => {
            println! ("来自 {:?} 州份的 25 美分硬币！", state);
            25
        }
    }
}
```


如果我们调用 `value_in_cents(Coin::Quarter(UsState::Alaska))`，`coin` 就会是 `Coin::Quarter(UsState::Alaska)`。当我们将该值，与每个匹配臂进行比较时，在我们到达 `Coin::Quarter(state)` 前，没有一个匹配支臂会匹配 。而在 `Coin::Quarter(state)` 处，`state` 的绑定值，将是 `UsState::Alaska`。然后，我们就可以在 `println!` 表达式中使用该绑定，从而从这个 `Coin` 枚举变种中，得到 `Quarter` 的内部州份值。


## 匹配 `Option<T>`

**Matching with `Option<T>`**


在上一节中，我们希望在使用 `Option<T>` 时，从 `Some` 情形中获取到内部的 `T` 值；我们也可以使用 `match` 来处理 Option<T>，就像我们在处理 `Coin` 枚举时所做的那样！我们将比较 `Option<T>` 的两个变种，而不是比较那些硬币，但 `match` 表达式的工作方式会保持不变。

假设我们要编写一个，会取某个 `Option<i32>` 值的函数，并在其中有值时，将 1 与该值相加。如果里面没有值，该函数应返回 `None` 值，且不会尝试执行任何运算。

由于有了 `match` 表达式，这个函数非常容易编写，看起来就像下面清单 6-5 一样。


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

*清单 6-5：使用了某个 `Option<i32>` 上的 `match` 表达式的函数*



我们来详细看看，`plus_one` 的第一次执行。当我们调用 `plus_one(five)` 时，`plus_one` 主体中的变量 `x` 的值为 `Some(5)`。然后，我们将其与每个匹支臂进行比较：


```rust
        None => None,
```


`Some(5)` 这个值与模式 `None` 不匹配，因此我们继续到下一支臂：


```rust
        Some(i) => Some(i + 1),
```


`Some(5)` 是否匹配 `Some(i)` 呢？匹配！我们有着同样的变种。`i` 会与包含在 `Some` 中的值绑定，因此 `i` 会取得值 `5`。然后该匹配支臂中的代码会被执行，因此咱们会将 1 与 `i` 的值相加，并创建处一个新的，其中有着咱们的和 `6` 的 `Some` 值。

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
