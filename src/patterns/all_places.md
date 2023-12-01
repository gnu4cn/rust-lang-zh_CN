# 可使用模式的全部位置

**All the Places Patterns Can Be Used**


模式会出现在 Rust 中的数个地方，而咱们以及见到很多的使用他们而不自知！本小节会讨论模式有效的全部位置。


## `match` 的支臂

**`match` Arms**

正如第 6 章中曾讨论过的，咱们是在 `match` 表达式的那些支臂中，使用模式的。形式上看，`match` 表达式是以关键字 `match`、要匹配的某个值，以及由某个模式和在该值匹配此模式时，要运行的一个表达式组所成的一个或多个支臂，这种形式而被定义出的，就像下面这样：

```rust
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}
```

比如，下面就是清单 6-5 中，在变量 `x` 中一个 `Option<i32>` 值上匹配的那个 `match` 表达式：

```rust
match x {
    None => None,
    Some(i) => Some(i + 1),
}
```

这个 `match` 表达式中的模式，便是各个箭头左边的 `None` 与 `Some(i)`。

`match` 表达式的一项要求，便是在表达式中那个值，所必须考虑到的全部可能性方面，需要 *详尽无遗，exhaustive*。而一种确保咱们已经覆盖每种可能性的方式，便是将一个捕获全部的模式，a catchall pattern，作为最后支臂：比如，一个匹配任意值的变量名称，就绝不会失败，而因此会覆盖每种其余的情形。


特别的模式 `_`，将匹配任何东西，但他绝不会绑定到某个变量，因此他通常被用在最后的匹配支臂中。在比如咱们打算忽略任何不予指定的值时，这种 `_` 模式就会是有用的。在本章稍后的 [“忽略模式中的值”](#忽略模式中的某些值ignoring-values-in-a-pattern) 小节中，咱们将更详细地讲到这种 `_` 模式。


## 条件 `if let` 表达式

**Conditional `if let` Expressions**


在第 6 章中，咱们曾讨论过怎样将 `if let` 表达式主要用于编写，只与一种情形匹配的 `match` 表达式等价的简便方式。`if let` 可选地能有包含了在 `if let` 中模式不匹配时，要运行代码的一个相应 `else`。

下面清单 18-1 显示，混用及匹配 `if let`、`else if` 及 `else if let` 这些表达式是可行的。相比于其中咱们只能表达出，与一些模式匹配的唯一一个值的 `match` 表达式，这样做就会给到更多灵活性。并且，Rust 不要求一系列 `if let`、`else if`、`else if let` 支臂中的那些条件相互有关联。

清单 18-1 中的代码，根据数个条件的一系列检查，而判断出构造绑架的何种颜色。对于这个示例，咱们已创建了有着真实程序中，本应从用户输入接收到，但这里是一些硬编码值的数个变量。

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

*清单 18-1：混用 `if let`、`else if`、`else if let` 及 `else`*

在用户指定了喜好的颜色时，那种颜色就会被用作背景。若没有指定喜好颜色而今天是周二，背景颜色就是绿色。否则，在用户以字符串指定了他们的年龄，而咱们可以将其成功解析为数字时，根据该数字的值，颜色就会要么时紫色，抑或是橙色。而在这些条件都不适用时，背景颜色就会是蓝色。

这种条件结构，实现了对复杂要求的支持。在这里咱们所拥有的那些硬编码值下，这个示例将打印出 `使用紫色作为背景色`。

咱们可以看到，`if let` 也能以 `match` 支臂所能够的同样方式，引入一些遮蔽变量，shadowed variables：其中行 `if let Ok(age) = age` 就引入了包含着在那个 `Ok` 变种里值的一个新遮蔽 `age` 变量。这意味着咱们需要把 `if age > 30` 情形，放在该代码块里：咱们不能将这两种情形，结合进到 `if let Ok(age) = age && age > 30`。咱们打算将其与 `30` 比较的那个遮蔽 `age`，直到那个新代码块以花括号开头之前，都还是无效的。

使用 `if let` 表达式的缺点，便是编译器不会就穷尽加以检查，而在 `match` 表达式下则会。若咱们省略了其中最后的 `else` 代码块，而因此遗漏了处理某些情形，编译器也不会就可能的逻辑错误向我们发出告警。


## `while let` 条件循环


与 `if let` 的构造类似，`while let` 条件循环允许某个 `while` 循环，在某个模式持续匹配期间运行。下面清单 18-2 中，咱们编写了将一个矢量用作栈，并以该矢量中那些值被压入的相反顺序，将这些值打印处理的一个 `while let` 循环。

```rust
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println! ("{}", top);
    }

```

*清单 18-2：使用 `while let` 循环，来于 `stack.pop()` 返回 `Some` 期间打印出一些值*

此示例打印出 `3`、`2` 及随后的 `1`。`pop` 方法会取出矢量值的最后一个元素，并返回 `Some(value)`。在该矢量值为空时，`pop` 就会返回 `None`，这个循环便停止。咱们可以使用 `while let` 循环，弹出栈中的每个元素。


## `for` 循环


在 `for` 循环中，直接跟在关键字 `for` 之后的那个值，就是个模式。比如，在 `for x in y` 中，`x` 就是其中的模式。下面清单 18-3 演示了如何使用 `for` 循环中的模式，来结构，或者说拆分作为这个 `for` 循环一部分的某个元组。

```rust
fn main() {
    let v = vec! ['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println! ("{} 处于索引 {} 处", value, index);
    }
}
```

*清单 18-3：使用一个 `for` 循环中的模式，来结构某个元组*


清单 18-3 中的代码，将打印出如下内容：

```console
$ cargo run                                                                                                           lennyp@vm-manjaro
   Compiling for_demo v0.1.0 (/home/lennyp/rust-lang/for_demo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/for_demo`
a 处于索引 0 处
b 处于索引 1 处
c 处于索引 2 处
```

咱们使用 `enumerate` 方法适配了一个迭代器，如此他便会产生放入到元组中的一个值，以及那个值的索引。首个产生出的值，为元组 `(0, 'a')`。当这个值与模式 `(index, value)` 匹配时，`index` 将为 `0`，同时 `value` 将为 `'a'`，从而打印出输出的首行。


## `let` 语句

本章之前，咱们只明确讨论过 `match` 与 `if let` 下对模式的运用，但事实上，咱们也曾在别的地方用到过模式，包括在 `let` 语句中。比如，请考虑下面这个使用 `let` 的简单变量赋值：

```rust
let x = 5;
```

在咱们每次像这样使用 `let` 语句，就都用到了模式，尽管咱们可能并未意识到他！更正式地说，`let` 语句看起来是这样的：

```rust
let PATTERN = EXPRESSION;
```

在像是 `let x = 5;` 这样，于 `PATTERN` 槽处有着一个变量名的语句中，那个变量名正是模式的一种特别简单形式。Rust 会将表达式与该模式比较，而赋予其找到的任何名字，Rust compares the expression against the pattern and assigns any names it finds。因此比如在 `let x = 5;` 中，`x` 就是一个表示 “将这里所匹配的东西，绑定到变量 `x`，bind what matches here to the variable `x`。” 由于名字 `x` 为整个的模式，那么此模式便意味着 “将所有东西都绑定到变量 `x`，不管值为何，bind everything to the variable `x`, whatever the value is。”

要更清楚地看到 `let` 的模式匹配方面，请考虑下面清单 18-4，他在 `let` 下使用了一个模式，来结构某个元组。

```rust
    let (x, y, z) = (1, 2, 3);
```

*清单 18-4：使用模式来结构元组，并一次性创建处三个变量*

在这里，咱们将一个元组与某个模式匹配。Rust 会比较把值 `(1, 2, 3)`，与模式 `(x, y, z)` 相比较，并发现该值与这种模式匹配，因此 Rust 就把 `1` 绑定到 `x`，`2` 绑定到 `y`，而把 `3` 绑定到 `z`。咱们可把这种元组模式，设想为在其中嵌套了三个单独的变量。

而当模式中元素个数，不与元组中元素个数匹配时，整体的类型就不会匹配，同时咱们将得到一个编译器报错。比如，下面清单 18-5 给出了三个元素解构到两个变量的尝试，这将不会工作。

```rust
    let (x, y) = (1, 2, 3);
```

*清单 18-5：不正确的构造模式，其变量与元组中元素个数不匹配*

尝试编译此代码会导致如下这种类型报错：


```console
$ cargo run                                                                                                     lennyp@vm-manjaro
   Compiling while_let_demo v0.1.0 (/home/lennyp/rust-lang/while_let_demo)
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
error: could not compile `while_let_demo` due to previous error
```

要修复该错误，咱们可以如同即将在 [于模式中忽略一些值](#忽略模式中的某些值ignoring-values-in-a-pattern) 小节中所看到的那样，使用 `_` 或 `..`，忽略那个元组中的一个或更多的值。当问题是咱们在模式中有太多变量时，那么办法就是通过移除一些变量构造出类型，从而变量数目便等于元组中元素的数目了。


## 函数参数

**Function Parameters**


函数参数也可以是些模式。下面清单 18-6 中的代码，声明了取名为 `x` 类型 `i32` 的一个叫做 `foo` 的函数，现在看起来应不陌生。

```rust
fn foo(x: i32) {
    // 代码出现在这里
}
```

*清单 18-6：在参数中用到模式的一个函数签名*

其中那个 `x` 部分，便是个模式！与咱们曾在 `let` 下所做的那样，咱们可以在函数参数中，将某个元组和模式匹配。下面清单 18-7 就在咱们把某个元组传递给一个函数时，拆分了其中的那些值：

文件名：`src/main.rs`

```rust
其中那个 `x` 部分，便是个模式！正如咱们在 `let` 下曾做的那样，咱们可将函数参数中的某个元组，与模式匹配。下面清单 18-7，就在咱们将某个元组传递给一个函数时，拆分了其中的那些值。

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

*清单 18-7：有着一些解构某个元组参数的一个函数*

此代码会打印出 `当前坐标：(3, -5)`。值 `&(3, -5)` 匹配了模式 `&(x, y)`，因此 `x` 为值 `3` 而 `y` 就是值 `-5`。


由于如咱们在第 13 章中曾讨论过的，闭包与函数类似，咱们也可以函数参数清单中的这同样方式，在闭包参数清单中使用模式。

到这里，咱们就已经看到了运用模式的数种方式，但在咱们可使用他们的每种地方，模式并非以同样方式运作。在一些地方，模式必须是确凿的，must be irrefutable；而在别的情况下，他们则可以是可证伪的，can be refutable。接下来咱们就将讨论这两个概念。
