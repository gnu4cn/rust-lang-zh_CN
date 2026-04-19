# 可以使用模式的所有地方

模式出现在 Rust 中的很多地方，咱们可能在不知不觉中已经频繁使用了他们！这一小节讨论模式有效的所有地方。

## `match` 支臂

正如第 6 章中所讨论的，我们在 `match` 表达式的支臂中使用模式。从形式上讲，`match` 表达式被定义为关键字 `match`、要匹配的值，以及一个或多个匹配支臂，每个支臂由一个模式和一个在值与该支臂模式匹配时要运行的表达式构成，如下所示：

```rust
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}
```

例如，下面是 [清单 6-5](../enums_and_pattern_matching/match_control_flow.md#listing_6-5) 中的 `match` 表达式，匹配变量 `x` 中的 `Option<i32>` 值：

```rust
match x {
    None => None,
    Some(i) => Some(i + 1),
}
```

这个 `match` 表达式中的模式是每个箭头左侧的 `None` 和 `Some(i)`。

`match` 表达式的一项要求是，他们必须穷尽所有情况，即必须考虑 `match` 表达式中值的所有可能性。确保咱们覆盖所有可能性的一种方式是，对最后一个支臂使用全包模式，a catchall pattern：例如，匹配任何值的变量名字就永远不会失败，而因此涵盖了所有剩余的情形。

特定模式 `_` 将匹配任何内容，但他永远不会绑定到变量，因此常被用于最后一个支臂。例如，当咱们打算忽略任何未指定的值时，`_` 模式就非常有用。我们在本章后面的 [忽略模式中的值](./syntax.md#忽略模式中的值) 中更详细地介绍 `_` 模式。

## `let` 语句

在本章之前，我们只明确讨论过通过 `match` 和 `if let` 使用模式，但事实上，我们也在其他地方使用了模式，包括在 `let` 语句中。例如，请考虑下面这个通过 `let` 的简单变量赋值：

```rust
let x = 5;
```

每当咱们使用这样的 `let` 语句时，咱们都在使用模式，尽管可能并没有意识到这一点！更正式地说，`let` 语句如下所示：

```rust
let PATTERN = EXPRESSION;
```

在像 `let x = 5;` 这样的语句中，当变量名处于 `PATTERN` 槽中时，变量名只是模式的一种特别简单的形式。Rust 会将表达式与该模式比较，并指派他找到的任何名字。 因此，在 `let x = 5;` 这个示例中，`x` 是个模式，意思是 “绑定此处匹配的内容到变量 `x`”。由于名字 `x` 就是整个模式，这个模式实际上意味着 “绑定所有内容到变量 `x`，无论值是什么”。

为了更清楚地了解 `let` 的模式匹配方面，请考虑下面清单 19-1，其中通过 `let` 使用模式来解构元组。

<a name="listing_19-1"></a>
```rust
    let (x, y, z) = (1, 2, 3);
```

**清单 19-1**：使用模式解构元组，并一次创建三个变量

在这里，我们将元组与模式匹配。Rust 会将值 `(1, 2, 3)` 与模式 `(x, y, z)` 比较，并发现该值与模式匹配 —— 即二者中的元素的数量相同 —— 因此 Rust 会绑定 `1` 到 `x`，`2` 到 `y`，`3` 到 `z`。咱们可以将这个元组模式，视为内部嵌套了三个单独变量的模式。

当模式中的元素数量与元组中的元素数量不匹配时，整体类型将不匹配，我们将得到编译器报错。例如，下清单 19-2 展示了一次将有着三个元素的元组解构为两个变量的尝试，但这是行不通的。

<a name="listing_19-2"></a>
```rust
    let (x, y) = (1, 2, 3);
```

**清单 19-2**：错误地构造了一个模式，其变量数量与元组的元素数量不匹配

尝试编译这段代码会导致以下类型报错：

```console
$ cargo run
   Compiling patterns v0.1.0 (/home/hector/rust-lang-zh_CN/projects/patterns)
error[E0308]: mismatched types
 --> src/main.rs:2:9
  |
2 |     let (x, y) = (1, 2, 3);
  |         ^^^^^^   --------- this expression has type `({integer}, {integer}, {integer})`
  |         |
  |         expected a tuple with 3 elements, found one with 2 elements
  |
  = note: expected tuple `({integer}, {integer}, {integer})`
             found tuple `(_, _)`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `patterns` (bin "patterns") due to 1 previous error
```

要解决这一错误，我们可以使用 `_` 或 `..` 忽略元组中的一个或多个值，正如咱们将在 [忽略模式中的值](./syntax.md#忽略模式中的值) 小节中看到的那样。当问题在于模式中的变量过多时，解决方法是通过移除变量来使类型匹配，以便变量的数量等于元组中元素的数量。


## 条件 `if let` 表达式

在 [第 6 章](../enums_and_pattern_matching/if-let_control_flow.md) 中，我们讨论了怎样使用 `if let` 表达式，主要作为编写仅匹配一种情况的 `match` 表达式的等价的简短方式。此外，`if let` 还可以有一个相应的 `else`，在 `if let` 中的模式不匹配时包含代码。

下面清单 19-3 展示了还可以混合使用 `if let`、`else if` 和 `else if let` 表达式。这样做给予我们相比 `match` 表达式更大的灵活性，在 `match` 表达式中我们只能表达一个值与模式比较。此外，Rust 不要求一系列 `if let`、`else if`、`else if let` 支臂中的条件相互关联。

清单 19-3 中的代码会根据多个条件的一系列检查，类确定构成咱们背景的颜色。对于这个示例，我们以硬编码的值创建了变量，这些硬编码值在真实程序中可能来自用户输入。

<a name="listing_19-3"></a>
文件名：`src/main.rs`

```rust
fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println! ("使用你喜欢的颜色，{color}，作为背景");
    } else if is_tuesday {
        println! ("周二是绿色的一天！");
    } else if let Ok(age) = age {
        if age > 30 {
            println! ("使用紫色作为背景色");
        } else {
            println! ("使用橙色作为背景色");
        }
    } else {
        println! ("使用蓝色作为背景色");
    }
}
```

**清单 19-3**：混合使用 `if let`、`else if`、`else if let` 及 `else`

当用户指定了喜好的颜色时，该颜色会被用作背景。当没有指定喜好颜色并且今天是周二时，背景颜色为绿色。否则，当用户以字符串形式指定了他们的年龄，并且我们可以成功将其解析为数字时，则根据该数字的值，颜色为紫色或橙色。当这些条件都不适用时，背景颜色为蓝色。

这种条件结构使我们可以支持复杂的需求。在这里的硬编码值下，这个示例将打印 `使用紫色作为背景色`。

咱们可以看到，`if let` 还可以与 `match` 支臂的同样方式，引入遮蔽现有变量的新变量：其中 `if let Ok(age) = age` 行引入了一个新的 `age` 变量，包含 `Ok` 变种内的值。这意味着我们需要放置 `if age > 30` 的条件于该代码块中：我们无法合并这两个条件为 `if let Ok(age) = age && age > 30`。在新的作用域以花括号开始前，我们打算与 `30` 比较的新 `age` 是无效的。

使用 `if let` 表达式的缺点是，编译器不会检查是否详尽，而对 `match` 表达式他会检查。当我们省略了最后的 `else` 代码块，而因此遗漏了处理某些情况时，编译器将不会提醒我们可能的逻辑 bug。


## `while let` 条件循环

与 `if let` 的结构类似，`while let` 条件循环允许 `while` 循环在模式持续匹配的情况下一直运行。在下面清单 19-4 中，我们展示了一个 `while let` 循环，等待线程之间发送的消息，但在这一情形下检查 `Result` 而非 `Option`。

<a name="listing_19-4"></a>
```rust
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        for val in [1, 2, 3] {
            tx.send(val).unwrap();
        }
    });

    while let Ok(value) = rx.recv() {
        println!("{value}");
    }
```

**清单 19-4**：使用 `while let` 循环，来在 `rx.recv()` 返回 `Ok` 期间打印值

这个示例会打印 `1`、`2`，然后是 `3`。`recv` 方法从信道的接收端取出第一条消息，并返回一个 `Ok(value)`。当我们在第 16 章中第一次看到 `recv` 时，我们曾直接解包错误，或通过使用 `for` 循环将其作为迭代器与其交互。但如清单 19-4 所示，我们也可以使用 `while let`，因为只要发送方存在，`recv` 方法就会在每次消息到达时都返回 `Ok`，而一旦发送方断开连接就会返回 `Err`。


## `for` 循环

在 `for` 循环中，紧跟关键字 `for` 之后的值即为模式。例如，在 `for x in y` 中，`x` 就是模式。下面清单 19-5 演示了如何在 `for` 循环使用模式解构，或者拆解元组，作为 `for` 循环的一部分。

<a name="listing_19-5"></a>
```rust
fn main() {
    let v = vec! ['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println! ("{} 处于索引 {} 处", value, index);
    }
}
```

**清单 19-5**：在 `for` 循环使用模式解构元组


清单 19-5 中的代码将打印出以下内容：

```console
$ cargo run
   Compiling patterns v0.1.0 (/home/hector/rust-lang-zh_CN/projects/patterns)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/patterns`
a 处于索引 0 处
b 处于索引 1 处
c 处于索引 2 处
```

我们使用 `enumerate` 方法适配迭代器，以便他生成值及该值的索引，便放入元组中。生成的第一个值是元组 `(0, 'a')`。当这个值与模式 `(index, value)` 匹配时，`index` 将为 `0`，`value` 将为 `'a'`，从而打印输出的第一行。


## 函数参数

函数参数也可以是模式。下面清单 19-6 中的代码，声明了一个名为 `foo` 的函数，取一个名为 `x` 类型 `i32` 的参数，现在看起来应该很很熟悉。

<a name="listing_19-6"></a>
```rust
fn foo(x: i32) {
    // 代码放在这里
}
```

**清单 19-6**：在参数中使用模式的函数签名

`x` 部分便是个模式！就像我们对 `let` 所做的那样，我们可以将函数参数中的元组与模式匹配。下面清单 19-7 在我们传递元组给函数时，拆分了其中的值。

<a name="listing_19-7"></a>
文件名：`src/main.rs`

```rust
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("当前坐标：({}, {})", x, y);
}

fn main() {
    let point = (3, -5);
    print_coordinates(&point);
}
```

**清单 19-7**：带有解构元组的参数的函数

这段代码会打印 `当前坐标：(3, -5)`。值 `&(3, -5)` 匹配模式 `&(x, y)`，因此 `x` 为值 `3`，`y` 为值 `-5`。

由于如在第 13 章中所讨论的那样，闭包与函数类似，因此我们也可以在闭包的参数列表中，以与函数的参数列表相同的方式使用模式。

至此，咱们就已经了解了数种使用模式的方式，但模式在我们可以使用他们的每个地方，不会都以相同的方式工作。在一些地方，模式必须是不可证伪的；而在其他情况下，他们可以是可证伪的。接下来我们将讨论这两个概念。


（End）


