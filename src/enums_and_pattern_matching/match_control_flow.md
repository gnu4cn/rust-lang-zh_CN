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


`match` 支臂的另一有用特性，便是这些支臂可绑定到值与模式进行匹配值的多个部分（another useful feature of `match` arms is that they can bind to the parts of the values that match the pattern）。这就是从枚举变种提取出值的原理。

作为一个示例，下面就来将这里的枚举变种之一，修改为其内部保存数据。自 1999 年到 2008 年，美国在 25 美分硬币的一面，铸造上 50 个州不同的设计。别的硬币则没有这样的州份设计，因此只有这些 25 美分硬币才有这额外价值。那么就可以通过修改这个 `Quarter` 变种为内部包含一个 `UsState` 值，来将此信息添加到这里的 `enum` 类型，就如同下面清单 6-4 中所做的。

```rust
#[derive(Debug)]    // 这样就可以很快对州份进行检查
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

来设想一下有个朋友是在尝试收集全部 50 个州份的 25 美分硬币。在按照硬币类型对零钱进行分类的同时，还将叫出与每个 25 美分硬币关联的州份名字，如此就可以在发现那个 25 美分硬币，是那位朋友还没有的时候，就可以把那个硬币添加到收藏。

而在这个代码的 `match` 表达式中，就要添加一个名为 `state` 的变量到匹配变种 `Coin::Quarter` 的那些值。在有 `Coin::Quarter` 匹配时，这个 `state` 变量就会绑定到那个 25 美分硬币的状态值。随后就可以在那个支臂的代码里，使用 `state` 变量了，如同下面这样：

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

这时在调用了 `value_in_cents(Coin::Quarter(UsState::Alaska))` 后，`coin` 就将是 `Coin::Quarter(UsState::Alaska)`。在将该值与各支臂进行比较时，在到达 `Coin::Quarter(state: UsState)` 支臂之前，是不会有任何支臂于其匹配的。而在 `Coin::Quarter(state: UsState)` 支臂处，`state` 所绑定的，将是值 `UsState::Alaska`。这时就可以使用那个 `println!` 表达式中的绑定，进而就从 `Quarter` 的 `Coin` 枚举变种，获取到那个内部 `state` 值了。


## 使用 `Option<T>` 的模式匹配

**Matching with `Option<T>`**


在前一小节，那里是想要在运用 `Option<T>` 时，从 `Some` 情形中获取到那个内部的 `T` 值；像前面对 `Coin` 枚举所做的那样，也可以这样来对 `Option<T>` 加以处理！比较的不再是那些硬币，而是将比较 `Option<T>` 的两个变种，不过那个 `match` 表达式的原理还是一样的。

下面就假设说要编写一个取 `Option<i32>` 类型值的函数，同时当 `Option<i32>` 里面有个值时，就将 `1` 加到那个值上。在 `Option<i32>` 里没有值时，该函数则会返回 `None` 值，并不会尝试执行任何运算。

归功于 `match` 表达式，这个函数写起来很容易，他将看起来像下面清单 6-5 这样。

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(n) => Some(n + 1),
    }
}

fn main() {
    let five = Some(5);
    let none = None;
    println! ("{:?}, {:?}", plus_one(five), plus_one(none));
}
```

*清单 6-5：在 `Option<i32>` 类型上运用了 `match` 表达式的一个函数*

下面就来详细检视一下 `plus_one` 函数的首次执行。在调用 `plus_one(five)` 时，`plus_one` 函数体中的变量 `x` 将有着值 `Some(5)`。之后就会与 `match` 表达式的各个支臂进行比较。

```rust
        None => None,
```

该 `Some(5)` 值不与模式 `None` 匹配，因此就会继续到下一支臂。

```rust
        Some(n) => Some(n + 1),
```

`Some(5)` 与 `Some(n)` 匹配吗？当然是匹配的！这里有着同样的变种。这个 `n` 绑定的是包含在 `Some` 中的那个值，因此 `n` 就会取到值 `5`。随后该 `match` 支臂中的代码就会被执行，从而就会将 `1` 加到 `n` 的值，并创建出一个新的、内部有着这里的和 `6` 的 `Some` 值来。

现在来看看清单 6-5 中第二个 `plus_one` 的调用，其中 `x` 则是 `None` 了。这里进入到那个 `match` 表达式，并与第一个支臂进行比较。

```rust
        None => None,
```

他是匹配的！就没有要加的值了，因此程序就停下来并返回 `=>` 右侧上的那个 `None` 值。由于第一个支臂已经匹配，因此就不会再比较其他支臂了。

在许多场合，将 `match` 表达式与枚举结合都是有用的。在 Rust 代码中将会看到很多这样的模式：对某个枚举的 `match` 操作，将某个变量绑定到内部数据，并随后据此执行代码（`match` against an enum, bind a variable to the data inside, and then execute code based on it）。在刚开始的时候这显得有些难以琢磨，而一旦熟悉了这种模式，就会希望在全部语言中都有这样的模式。这样的模式一直是编程者的最爱。


## 匹配要彻底

**Matches Are Exhaustive**


这里有个需要讨论到的 `match` 表达式的另一方面。想想这个有着代码错误而不会编译的 `plus_one` 版本：

```rust
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(n) => Some(n + 1),
        }
    }
```

这里没有对 `None` 情形加以处理，因此该代码就会引起错误。幸运的是，那是个 Rust 知道怎样取捕获的代码错误。在尝试编译此代码时，就会得到这样的错误：

```console
$ cargo run
   Compiling enum_demo v0.1.0 (/home/peng/rust-lang/projects/enum_demo)
error[E0004]: non-exhaustive patterns: `None` not covered
   --> src/main.rs:2:11
    |
2   |     match x {
    |           ^ pattern `None` not covered
    |
note: `Option<i32>` defined here
    = note: the matched value is of type `Option<i32>`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
    |
3   ~         Some(n) => Some(n + 1),
4   ~         None => todo!(),
    |

For more information about this error, try `rustc --explain E0004`.
error: could not compile `enum_demo` due to previous error
```

Rust 是知道这里未曾覆盖到每种可能情形，并甚至清楚这里忘记了那个模式！ Rust 中的 `match` 表达式要是 *彻底的（exhaustive）*：为了让代码有效，就必须穷尽所有的可能性。尤其是在 `Option<T>` 这个示例中，在 Rust 阻止这里忘记显式地处理 `None` 这个情形时，在这里可能会有个 `null` 值时，他就保护了避免有个值的错误假设，进而让那个先前讨论到的十亿美金错误成为不可能了。


## 捕获所有模式与 `_` 占位符

**Catch-all Patterns and the `_` Placeholder**


运用枚举，还可以对少数特定值采取特别动作，而对所有其他值采取一种默认动作。设想正在实现某个游戏，其中在投中了骰子上的 3 点时，游戏角色就不会移动，而是会收到一顶新的帽子道具。而在投中 7 点时，游戏角色会失去一定道具帽子。对于其他所有点数值，游戏角色都会在游戏板上移动相应数目的格子。下面就是个实现了该逻辑的 `match` 表达式，其中的骰子点数结果，是硬编码而非随机值，至于其他由不带函数体的函数所表示的逻辑，则是由于实现这些函数超出了本示例的范围：

```rust
let dice_roll = 9;

match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player() {}
```

对于前两个支臂，模式为字面值 `3` 和 `7`。而那最后的最比，则涵盖了所有其他可能的值，该模式为这里以选为命名为 `other` 的那个变量。为该 `other` 支臂所运行的代码，通过将这个 `other` 变量传递给 `move_player` 函数，而用到了这个变量。

由于那最后的模式将匹配到未特别列出的全部值，因此尽管这里并未列出 `u8` 类型变量有的全部可能值，这段代码仍会编译。这种捕获全部的模式，满足了 `match` 表达式务必彻底的要求。请注意由于这些模式是求值的，因此这里必须将那个捕获全部支臂放在最后。若在捕获全部之后，添加了其他支臂，那么 Rust 就会告警，这是由于这些在捕获全部之后的支臂根本不会匹配到！

Rust 还有一种在不愿使用捕获全部模式中的值时，可使用的一种模式：`_`，这是一种特别的、未与该值绑定的其他所有值。这种模式告诉 Rust 这里将不会使用该值，因此 Rust 就不会发出有关某个未用到变量的告警了（Rust also has a pattern we can use when we don't want to use the value in the catch-all pattern: `_`, which is a special pattern that matches any value and doen't not bind to that value. This tells Rust we aren't going to use the value, so Rust won't warn us about an unused varialbe）。

下面就来将那个游戏的规则修改为，在投中骰子的三点和七点之外别的点数时，就必须再投一次骰子。那么这里就不需要用到那个点数值了，因此就可以将这里的代码修改为使用 `_` 而不是那个名为 `other` 的变量：

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

由于这里在最后的支臂中，显式地忽略了全部其他值，因此该示例也是满足 `match` 表达式的穷尽要求的；这里并未忘记掉任何东西。

若再一次修改此游戏的规则，修改为在抛出即非三点也非七点的其他点数时，什么也不会发生，那么就可以通过使用单元值（即在 [元组类型](Ch03_Common_Programming_Concepts.md#元组类型) 小节中讲到的那个空元组类型）作为该 `_` 支臂后的代码，来表达这样的游戏规则：

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

这里就显式地告诉 Rust，这里将不使用那些不与先前支臂匹配的全部其他值，且在此情形下这里不要运行任何代码。

在 [第 18 章](Ch18_Patterns_and_Matching.md) 将涉及到更多有关模式与匹配的内容。而现在就要移步到 `if let` 语法，在那些使用 `match` 表达式显得多余的情形下，`if let` 语法就会有用。
