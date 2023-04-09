# 模式与匹配

**Patterns and Matching**

*模式，patterns* 属于 Rust 中，用于与不论简单，还是复杂的类型结构体，做匹配的一种特别语法。与 `match` 表达式及其他一些构件，other constructs，结合使用模式，就会给到咱们对程序控制流程更多的掌控。模式有以下元素的一些组合构成：

- 字面值

- 解构过后的数组、枚举、结构体或元组等，destructured arrays, enums, structs, or tuples

- 变量

- 通配符，wildcards

- 占位符，placeholders


一些示例模式，包括 `x`、`(a, 3)` 及 `Some(Color::Red)` 等。在模式为有效的语境中，这些组件描述了数据的形状，the shape of data。咱们的程序随后就会将一些值，与这些模式做匹配，来判断其是否有着数据的正确形状，而继续运行代码的某个特定片段。


要运用某个模式，咱们就要将其与某个值比较。在该模式与那个值匹配时，咱们在咱们的代码中，使用这个值的那些部分。回顾第 6 章中用到模式的那些 `match` 表达式，比如那个硬币分类机器示例。在值满足模式形状时，咱们就可以使用那些命名的代码片段。而在不满足时，与该模式关系的代码就不会运行。

本章时与模式相关全部内容的一个参考。咱们将涵盖运用模式的那些有效位置、可证伪与不可证伪模式的区别，the difference between refutable and irrefutable patterns，以及可能见到的那些不同类别的模式语法。在本章最后，咱们将获悉，如何运用模式来清晰地表达许多概念。


## 可使用模式的全部位置

**All the Places Patterns Can Be Used**


模式会出现在 Rust 中的数个地方，而咱们以及见到很多的使用他们而不自知！本小节会讨论模式有效的全部位置。


### `match` 的那些支臂

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


### 条件 `if let` 表达式

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


### `while let` 条件循环

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


### `for` 循环

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


### `let` 语句

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


### 函数参数

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


## 可证伪性：某个模式有无可能匹配失败

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


## 模式语法

**Pattern Syntax**

在这个小节中，咱们会聚齐模式方面的全部有效语法，并讨论因何及何时会打算使用这每种的语法。


### 匹配字面值

**Matching Literals**

正如咱们在第 6 章中曾看到的那样，咱们可以直接将模式与字面值匹配。下面的代码给到了一些示例：


```rust
    let x = 1;

    match x {
        1 => println! ("壹"),
        2 => println! ("贰"),
        3 => println! ("叁"),
        _ => println! ("万物"),
    }
```

由于 `x` 里的值为 `1`，此代码会打印出 `壹`。当咱们打算代码，在其获取到某个特定具体值而采取某种动作时，这种语法就是有用的。


### 匹配命名变量

**Matching Named Variables**

命名变量属于匹配任意值的不可证伪模式，同时咱们已在本书中，用到他们许多次了。不过，当咱们在 `match` 表达式中使用命名变量时，便有着一种复杂性。由于 `match` 关键字开启了一个新的作用域，作用模式部分，而该 `match` 表达式内部声明出的那些变量，将遮蔽该 `match` 结构，the `match` construct 外部那些有着同意名字的变量，这与所有变量下的情况一样。在下面清单 18-11 中，咱们以值 `Some(5)` 声明了名为 `x` 的一个变量，及有着值 `10` 的一个变量 `y`。随后咱们在值 `x` 上创建了一个 `match` 表达式。请注意那些匹配支臂中的模式与末尾处的 `println!`，并在运行此代码或阅读接下来的内容前，尝试得出该代码将打印出什么。

文件名：`src/main.rs`

```rust
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println! ("得到了 50"),
        Some(y) => println! ("已匹配，y = {y}"),
        _ => println! ("默认情况，x = {:?}", x),
    }

    println! ("最后：x = {:?}, y = {y}", x);
```

*清单 18-11：有着引入了遮蔽变量 `y` 的一条支臂的 `match` 表达式*

下面就来走一遍，在这个 `match` 表达式运行时会发生些什么。首个匹配支臂中的模式不会匹配 `x` 所定义的值，因此代码会继续。

第二个匹配支臂中，引入了名为 `y` 新变量的那个模式，将匹配某个 `Some` 值内部的任意值。由于咱们是在这个 `match` 表达式内部的新作用域中，因此这就是个新的 `y` 变量，而不再是开头的以值 `10` 定义的 `y` 了。这个新的 `y` 绑定，将匹配某个 `Some` 内不的任意值，那便是咱们在 `x` 中所拥有的那个值了。因此，这个新 `y` 就绑定到了 `x` 中那个 `Some` 的内层值。那个值为 `5`，因此那个支臂的表达式就会执行，并打印出 `已匹配，y = 5`。

而若 `x` 曾为 `None` 值而非 `Some(5)`，那么头两个支臂中的模式，就都不会匹配，而该值将与其中的下划线 `_` 匹配。咱们并未以那个下划线模式，引入这个 `x` 变量，因此该表达式中的 `x` 仍为未被遮蔽的外层 `x`。而在这个假定情况中，该 `match` 将打印出 `默认情况，x = None`。

在这个 `match` 表达式完成是，他的作用域就结束了，而内层作用域的 `y` 也结束了。最后的 `println!` 会产生出 `最后：x = Some(5), y = 10`。

为创建出比较外层作用域中 `x` 与 `y` 值的一个 `match` 表达式，而非引入一个遮蔽变量，咱们将需要使用某种匹配卫兵条件，a match guard conditional。稍后咱们将在 [“带有匹配保护的额外条件”](#使用匹配卫兵的额外条件extra-conditionals-with-match-guards) 小节，讨论到匹配保护问题。



### 多个模式

**Multiple Patterns**

在 `match` 表达式中，咱们可以使用 `|` 语法，即模式 *或，or* 运算符，匹配多个模式。比如，在下面的代码中，咱们把 `x` 的值与那些匹配支臂匹配，其中头一个支臂就有一个 *或，or* 选项，表示在 `x` 的值与那条支臂中两个值之一匹配时，那条支臂的代码都将运行：


```rust
    let x = 1;

    match x {
        1 | 2 => println! ("一或二"),
        3 => println! ("三"),
        _ => println! ("万物"),
    }
```

此代码会打印出 `一或二`。


### 使用 `..=` 匹配值范围

**Matching Ranges of Values with `..=`**


这种 `..=` 语法，允许咱们与某个包容性值范围匹配，match to an inclusive range of values。下面的代码中，当某个模式匹配给定范围中任何值时，那条支臂便会执行：

```rust
    let x = 5;

    match x {
        1..=5 => println! ("一到五"),
        _ => println! ("万物"),
    }
```

在 `x` 为 `1, 2, 3, 4` 或 `5` 时，头一条支臂将匹配。相比于使用 `|` 运算符，对于多个匹配值，这种语法更便于表达同样的概念；若咱们使用的是 `|`，那么将不得不指明 `1 | 2 | 3 | 4 | 5`。而指明一个范围就简短多了，尤其是在打算匹配比如任何 `1` 到 `1000` 之间的数字时！

编译器会在编译时检查范围不为空，而由于 Rust 可识别出某个范围为空或不为空的类型，就只有 `char` 与数字值，因此就只运行数字或 `char` 值两种范围。


下面是使用 `char` 值范围的一个示例：

```rust
    let x = 'c';

    match x {
        'a'..='j' => println! ("靠前 ASCII 字母"),
        'k'..='z' => println! ("靠后 ASCII 字母"),
        _ => println! ("其他东西"),
    }
```

Rust 能分辨出 `c` 是在头一个模式的范围内，并打印出 `靠前 ASCII 字母`。


### 将值拆散的解构

**Destructuring to Break Apart Values**


咱们还可以运用模式，来解构结构体、枚举及元组，从而用到这些值的不同部分。下面就来贯穿这各个的值。


**解构结构体，Destructuring Stucts**


下面清单 18-12 给出了咱们可使用带有一个 `let` 语句的模式，而予以拆散的、有着两个字段，`x` 与 `y` 的一个 `Point` 结构体。


文件名：`src/main.rs`

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: -7 };

    let Point { x: a, y: b } = p;

    assert_eq! (0, a);
    assert_eq! (-7, b);
}
```

*清单 18-12：将结构体的那些字段解构为一些单独变量*

这段代码创建出匹配结构体 `p` 的 `x` 与 `y` 字段的变量 `a` 与 `b`。此示例展示了模式中变量的名字，不必匹配结构体的字段名字。但是，将变量名字与字段相匹配，以令到更易与记住哪些变量来自那个字段，则是通常做法。由于这种普遍用法，同时由于写下 `let Point { x: x, y: y } = p;`，包含了很多重复，Rust 便有了匹配结构体字段模式的一种简写：咱们只需列出结构体字段的名字，那么自该模式创建出的那些变量，就将有着这些同样名字。下面清单会与清单 18-12 中的代码，以同样方式行事，不过在那个 `let` 模式中创建出的变量，为 `x` 与 `x`，而不再是 `a` 与 `b` 了。


文件名：`src/main.rs`

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: -7 };

    let Point { x, y } = p;

    assert_eq! (0, x);
    assert_eq! (-7, y);
}
```

*清单 18-12：运用结构体字段简写解构结构体字段*

此代码创建了与变量 `p` 的 `x` 与 `y` 字段相匹配的变量 `x` 与 `y`。结果便是变量 `x` 与 `y` 包含着来自结构体 `p` 的那些值。

咱们也能以一些字面值，作为结构体模式的部分，而非创建出所有字段的变量，而加以解构。这样做允许咱们在创建出一些变量来解构其他字段的同时，而测试一些字段。


在下面清单 18-14 中，咱们有着一个将 `Point` 值分离到三种情形的一个 `match` 表达式：直接位于 `x` 轴上的那些点（在 `y = 0` 时此模式为真）、在 `y` 轴上的那些点，或既不在 `x` 也不在 `y` 轴上的那些点。

文件名：`src/main.rs`

```rust
    let p = Point { x: 0, y: -7 };

    match p {
        Point { x, y: 0 } => println! ("在 x 轴的 {x} 处"),
        Point { x: 0, y } => println! ("在 y 轴的 {y} 处"),
        Point { x, y } => {
            println! ("不在两个轴上：({x}, {y})");
        }
    }
```

*清单 18-14：同时在一个模式中的解构与字面值匹配*

首个支臂通过指明 `y` 字段在其值与字面值 `0` 匹配时匹配，而将匹配位于 `x` 轴上任意点。该模式仍创建了咱们可在此支臂代码中用到的变量 `x`。

类似地，第二条支臂通过指明 `x` 字段在其值为 `0` 时匹配，而会匹配位于 `y` 轴上的任意点，同时创建处 `y` 字段值的一个变量 `y`。第三条支臂没有指定任何字面值，因此其会匹配全部其他 `Point`，并创建出 `x` 与 `y` 字段的两个变量。

在此示例中，值 `p` 会由于 `x` 包含着一个 `0`，而匹配第二条支臂，从而此代码将打印出 `在 y 轴的 -7 处`.

请记住 `match` 表达式一旦找到第一个匹配的模式，就会停止检查支臂了，因此尽管 `Point { x: 0, y: 0 }` 是在 `x` 轴与 `y` 轴上，此代码将只打印出 `在 x 轴的 0 处`。


**解构枚举，Destructuring Enums**

本书中咱们已经解构过枚举（比如，第 6 章中的清单 6-5），但尚未明确讨论过以与存储在枚举内部数据所定义方式的相对应方式，来解构某个枚举的模式。作为一个示例，在下面清单 18-15 中，咱们使用清单 6-2 中的那个 `Message` 枚举，并编写了带有将解构各个内部值的一个 `match` 表达式。

文件名：`src/main.rs`

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println! ("Quit 变种没有要解构的数据。");
        }
        Message::Move { x, y } => {
            println! {"在 x 方向移动 {x}，在 y 方向移动 {y}"};
        }
        Message::Write(text) => {
            println! ("文本消息：{text}");
        }
        Message::ChangeColor(r, g, b) => {
            println! ("把颜色改为 红 {r}，绿 {g}，和蓝 {b}");
        }
    }
}
```

*清单 18-15：解构保存着不同类别值的枚举变种*

此代码将打印出 `把颜色改为 红 0，绿 160，和蓝 255`。请尝试修改 `msg` 的值，来看到该代码自其他支臂运行。

对应不带任何数据的那些枚举变种，像是 `Message::Quit`，咱们就无法进一步解构值。咱们只能匹配字面的 `Message::Quit` 值，且在那个模式中没有变量。

对于类似结构体的枚举变量，好比 `Message::Move`，咱们可以使用类似于指明用于匹配结构体的那种模式。在变种名字之后，咱们放置了一对花括号，并在随后列出有着变量的那些字段，从而咱们就拆散了要在此支臂代码中用到的各个部分。这里咱们运用了曾在清单 18-13 中曾用过的简写形式。

而对于类似元组的那些枚举变种，好比保存着有一个元素元组 `Message::Write` 与保存着有三个元素元组的 `Message::ChangeColor`，其模式便于指定用于匹配元组的模式类似。模式中的变量个数，务必要与咱们所匹配的变种中元素个数相匹配。


**嵌套结构体与枚举的解构，Destructuring Nested Structs and Enums**

到目前为止，咱们的这些示例都匹配的是一层深的结构体与枚举，而匹配也是能够在嵌套项目上工作的！比如，咱们可将清单 18-15 中的代码，重构为在 `ChangeColor` 消息中，支持 RGB 与 HSV 两种颜色，如下清单 18-16 中所示：

文件名：`src/main.rs`

```rust
enum Color {
    Rgb(u32, u32, u32),
    Hsv(u32, u32, u32),
}

emum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println! ("将颜色改为红 {r}、绿 {g} 及蓝 {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println! ("将颜色改为色调 {h}、饱和度 {s} 及颜色值 {v}");
        }
        _ => (),
    }
}
```

*清单 18-16：嵌套枚举上的匹配*


该 `match` 表达式中首个支臂的模式，匹配包含着 `Color::Rgb` 变种的 `Message::ChangeColor` 枚举变种；随后该模式绑定到那三个内部的 `i32` 值。第二支臂的模式，同样匹配的是 `Message::ChangeColor` 枚举变种，只不过那内部枚举匹配的是 `Color::Hsv` 了。咱们可在一个 `match` 表达式中，指定这些复杂条件，即使涉及到两个枚举。


**解构结构体与元组，Destructing Structs and Tuples**


咱们甚至可以更复杂的方式，对解构模式进行混用、匹配及嵌套。下面的示例，给出了一种复杂的解构，其中在一个元组中，嵌套了结构体与元组，并讲全部原生值解构了出来：


```rust
    let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10});
```

此代码实现将复杂类型，拆分为其各个组件部分，从而咱们可单独使用咱们所感兴趣的那些值。

以模式来解构数据，是各自独立使用诸如结构体中各个字段值，此类各部分值的一种便利方式。


### 忽略模式中的某些值，Ignoring Values in a Pattern


咱们已经看到，某些时候忽略模式中的一些值是有用的，比如在为获取到不具体完成任何事情，而确实处理全部剩余可能值的捕获全部的 `match` 最后支臂中。有少数几种忽略模式中全部或部分值的方式：使用 `_` 模式（the `_` pattern, 咱们已经见到过）、在另一模式中使用 `_` 模式、使用以下划线开头的名字，或使用 `..` 来忽略某个值的其余部分。下面就来探讨，怎样及为何要使用各个的这些模式。


**以 `_` 忽略整个值，Ignoring an Entire Value with `_`**

咱们已把这个下划线，作为将匹配任意值，却不绑定到该值的通配符模式进行了使用。这作为 `match` 表达式中的最后支臂尤其有用，但咱们也可在任意模式中用他，包括一些函数参数中，如下清单 18-17 中所示。

文件名：`src/main.rs`

```rust
fn foo(_: i32, y: i32) {
    println! ("此代码仅使用那个参数 y：{}", y);
}

fn main() {
    foo(3, 4);
}
```

*清单 18-17：在函数签名中使用 `_`*

此代码将完全忽略作为第一个参数传递的值 `3`，并将打印 `此代码仅使用那个参数 y：4`。

在当不再需要某个特定函数参数的大多数情况下，咱们就会修改函数签名，从而其不会包含未用到的参数。而在比如正实现某个特质时，需要某种确切类型签名，而咱们的实现中函数体不需要某个的这些参数，这样的情形中，忽略某个函数参数就会特别有用。随后咱们便避免了收到关于未使用的函数参数的编译器告警，这样的告警在使用某个参数名字时就会收到。


**使用嵌套的 `_` 忽略某个值的部分，Ignoring Parts of a Value with a Nested `_`**


在另一模式内部，咱们也可以使用 `_` 来仅忽略某个值的部分，比如当咱们打算仅测试某个值的部分，而在打算运行的相应代码中用不到其他部分时。下面清单 18-18 给出了负责管理某个设置值的代码。业务方面的要求为不应允许用户覆写某项设置的某个既有定制设置，但可以取消该项设置并在其当前未设置时给予其某个值。


```rust
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println! ("无法覆写既有定制值");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println! ("设置项值为 {:?}", setting_value);
```

*清单 18-18：当咱们不需要用到 `Some` 中的值时，在匹配一些 `Some` 变种的模式里使用下划线*


此代码将打印 `无法覆写既有定制值`，及随后的 `设置项值为 Some(5)`。在首个匹配支臂中，咱们无需匹配或是使用两个 `Some` 变种里的那些值，但确实需要就 `setting_value` 于 `new_setting_value` 为 `Some` 变种时的情形，加以测试。在那样的情形下，咱们会打印出不修改 `setting_value`，以及其不会被修改的理由。


在由第二支臂中 `_` 模式所表示的全部其他情形下（即 `setting_value` 或 `new_setting_value` 为 `None` 时），咱们就打算允许 `new_setting_value` 成为 `setting_value`。

咱们还可以在一个模式里的多处，使用下划线来忽略一些特定值。下面清单 18-19 给出了忽略某五个项目元组中，第二与第四个值的示例。


```rust
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println! ("一些数字为： {first}, {third}, {fifth}");
        }
    }
```

*清单 18-19：忽略元组的多个部分*


此代码将打印 `一些数字为： 2, 8, 32`，而值 `4` 与 `16` 将被忽略。


**通过以 `_` 开头的名字，忽略某个未用到的变量，Ignoring an Unused Variable by Starting Its Name with `_`**

在咱们创建了某个变量，但未在任何地方用到他时，由于未使用变量可能是代码问题，因此 Rust 通常将发出一条告警。然而，有的时候创建出尚未用到的某个变量则是有用的，比如在咱们正构造程序原型，或刚开始某个项目时。在这种情况下，咱们就可以通过以一个下划线，开启该变量的名字，而告诉 Rust 不要就这个未用到变量发出告警。下面清单 18-20 中，咱们创建了两个未使用变量，但在编译此代码时，咱们应只收到他们中一个的告警。

```rust
fn main() {
    let _x = 5;
    let y = 10;
}
```

*清单 18-20：以一个下划线开始变量名，来避免收到未使用变量的告警*

这里咱们会得到有关未用到变量 `y` 的告警，但不会收到未使用的 `_x` 的告警。

请注意在仅使用 `_` 与使用以下划线开头的名字之间，有着细微差别。`_x` 的语法仍将该值绑定到变量，而 `_` 则完全没有绑定。为给出其中这种区别重要性的情形，下面清单 18-21 将给到咱们一个报错。


```rust
    let s = Some(String::from("你好！"));

    if let Some(_s) = s {
        println! ("找到一个字符串");
    }

    println! ("{:?}", s);
```

*清单 18-21：以下划线开头的未使用变量，仍会绑定值，这就会取得该值的所有权*

由于这个 `s` 值将仍被迁移到 `_s` 中，而这会阻止咱们再度使用 `s`，因此咱们将收到一个报错。然而，使用下划线本身，就绝不会绑定到值。由于下面清单 18-22 中的 `s` 不会被迁移到 `_` 中，因此该代码将不带任何错误的编译。


```rust
    let s = Some(String::from("你好！"));

    if let Some(_) = s {
        println! ("找到一个字符串");
    }

    println! ("{:?}", s);
```

*清单 18-22：使用下划线不会绑定值*

由于咱们绝不会把 `s` 绑定到任何变量，他就没有被迁移，进而此代码工作良好。


**使用 `..` 忽略值的剩余部分，Ignoring Remaining Parts of a Value with `..`**


对于有着许多部分的值，咱们可以使用 `..` 语法来使用其特定部分而忽略剩下部分，避免列出各个忽略值那些下划线这样的需求。这种 `..` 模式，会忽略咱们在模式其余部分中，未曾显示匹配的任何部分。在下面清单 18-23 中，有着一个保存了三维空间中坐标的 `Point` 结构体。在那个 `match` 表达式中，咱们打算只在 `x` 坐标上运算，而忽略 `y` 与 `z` 两个字段中的值。


```rust
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println! ("x 为 {}", x),
    }
```

*清单 18-23：通过使用 `..` 忽略 `Point` 中除 `x` 外的全部字段*

咱们列出了值 `x` 并在随后只包含了模式 `..`。这要比列出 `y: _` 与 `z: _` 要快一些，尤其是当咱们在处理那些有着很多字段，而其中只有一两个字段是攸关的情形下。

`..` 语法将扩展到其所需的那么多个值。下面清单 18-24 给出了怎样在元组下使用 `..`。


文件名：`src/main.rs`

```rust
    let numbers = (2, 4, 6, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println! ("一些数字为： {first}, {last}");
        }
    }
```

*清单 18-24：匹配元组中首个与最后值，而忽略全部其他值*

在此代码中，首个与最后值，是以 `first` 及 `last` 匹配到的。其中的 `..` 将匹配并忽略中间的全部值。

不过，使用 `..` 必须必须要是明确的。在不明白哪些值是要匹配的，哪些值应被忽略时，Rust 就将给到我们一个报错。下面清单 18-25 给出了含混不清地使用 `..` 的一个示例，因此其不会编译。


文件名：`src/main.rs`


```rust
fn main() {
    let numbers = (2, 4, 6, 8, 16, 32);

    match numbers {
        (.., second, ..) => {
            println! ("一些数字为： {}", second);
        }
    }
}
```

*清单 18-25：尝试以模棱两可方式使用 `..`*

当咱们编译此示例时，就可到下面这个报错：

```console
$ cargo run
   Compiling pattern_syntax_demo v0.1.0 (/home/lenny.peng/rust-lang/pattern_syntax_demo)
error: `..` can only be used once per tuple pattern
 --> src/main.rs:5:22
  |
5 |         (.., second, ..) => {
  |          --          ^^ can only be used once per tuple pattern
  |          |
  |          previously used here

error: could not compile `pattern_syntax_demo` due to previous error
```

Rust 不可能确定出在以 `second` 匹配某个值之前，元组中有多少个值要忽略，并随后在那之后又有多少个值要忽略。此代码可能是指咱们打算忽略 `2`，将 `second` 绑定到 `4`，并随后忽略 `8`、`16` 及 `32`；或是指咱们打算忽略 `2` 与 `4`，将 `second` 绑定到 `8`，并随后忽略 `16` 与 `32`；如此等等。名为 `second` 的变量，对于 Rust 并不表示任何特殊的东西，从而由于在两处使用 `..` 属于模棱两可的，因此咱们就收到一个编译报错。


### 使用匹配卫兵的额外条件，Extra Conditionals with Match Guards


所谓 *匹配卫兵，match guard*，是于 `match` 支臂之后被指定出来，对于这条支臂要被选中，而也必须匹配的一个额外 `if` 条件。对于表达相对于所允许的单独模式，更为复杂的一些概念，这样的匹配卫兵就是有用的。

该条件可使用模式中创建出的那些变量。下面清单 18-26 给出了其中首条支臂有着模式 `Some(x)`，并同时有着 `if x % 2 == 0` 的匹配卫兵（在该数为偶数时将为 `true` ）的一个 `match`。


```rust
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println! ("数字 {} 为偶数", x),
        Some(x)  => println! ("数字 {} 为奇数", x),
        None => (),
    }
```

*清单 18-26：添加匹配卫兵到模式*

此示例将打印 `数字 4 为偶数`。在 `num` 与首个支臂中的模式相比时，由于 `Some(4)` 匹配了 `Some(x)`，因此他就匹配了。随后那个匹配卫兵就会检查 `x` 除以 `2` 的余数是否等于 `0`，而由于 `4` 除以 `2` 确实等于零，所以首个支臂便被选中了。

若 `num` 改作 `Some(5)`，那么由于 `5` 除以 `2` 的余数为 `1`，而不等于 `0`，那么首个支臂中的匹配卫兵将为 `false`。Rust 随后就会移步到第二支臂，由于第二支臂没有匹配卫兵，而因此会匹配任意 `Some` 变种，那么这第二支臂就会匹配到。

某个支臂里没有表达 `if x % 2 == 0` 的方式，因此这种匹配卫兵特性，便给到我们表达这种逻辑能力。这种额外表达力的缺点，便是在涉及到匹配卫兵时，编译器不会尝试检查完备性。

清单 18-11 中，咱们曾提到咱们可以使用匹配卫兵，来解决咱们的模式遮蔽问题，pattern-shadowing problem。回顾到咱们曾在那个 `match` 表达式中的支臂里，创建了一个新变量，而不是使用 `match` 外的那个变量。那个新变量就意味着咱们无法将其与其中的外层变量进行比对测试了。下面清单 18-27 给出了咱们怎样能使用匹配卫兵，修复这个问题。


文件名：`src/main.rs`

```rust
fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println! ("得到了 50"),
        Some(n) if n == y => println! ("匹配了，n = {n}"),
        _ => println! ("默认情况，x = {:?}", x),
    }

    println! ("最后：x = {:?}, y = {y}", x);
}
```

*清单 18-27：使用匹配卫兵测试与外层变量是否相等*

此代码现在将打印 `默认情况，x = Some(5)`。第二匹配支臂中的模式，没有引入将遮蔽外层 `y` 的新变量 `y`，意味着咱们可以在其中的匹配卫兵中使用那个外层的 `y`。与其将模式指明为将遮蔽外层 `y` 的 `Some(y)`，咱们指明的是 `Some(n)`。由于在这个 `match` 外没有变量 `n`，因此这创建了一个不会遮蔽任何东西的变量 `n`。

其中的匹配卫兵 `if n == y` 不是个模式，而因此不会引入新的变量。这个 `y` *便是* 外层的 `y`，而非一个新遮蔽的 `y`，进而咱们可以通过将 `n` 与 `y` 比较，查找与这个外层的 `y` 有着同样值的一个值。

咱们还可在匹配卫兵中，使用 *或，or* 运算符 `|`，来指定多个模式；匹配卫兵条件将应用到全部这些模式。下面清单 18-28 展示了将使用了 `|` 的模式，与匹配卫兵结合时的优先级。这个示例的重要之处是，其中的 `if y` 匹配卫兵，会应用到 `4`、`5` *及* `6`，即使看起来 `if y` 只应用到 `6`。


```rust
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println! ("是"),
        _ => println! ("否"),
    }
```

*清单 18-28：将多个模式与匹配卫兵相结合*


其中的匹配条件指出，该支臂仅在 `x` 的值等于 `4`、`5` 或 `6`， *且* 在 `y` 为 `true` 时匹配。在此代码运行时，由于 `x` 为 `4`，因此首条支臂的模式会匹配，但匹配卫兵 `if y` 是 `false`，从而首条支臂未被选中。代码就移步到第二支臂，其就匹配了，而此程序就打印出 `否`。原因就是，其中的 `if` 条件会应用到整个模式 `4 | 5 | 6`，而不仅是应用到最后的值 `6`。也就是说，匹配守卫相对于模式的优先级表现如下：

```rust
(4 | 5 | 6) if y => ...
```

而非这样：

```rust
4 | 5 | (6 if y) => ...
```

在运行此代码后，这种优先级行为便是显而易见的了：若那个匹配卫兵，只被应用到使用 `|` 运算符所指定的值清单中的最后那个值，那么该支臂将匹配，而这个程序就会打印出 `是`。


### `@` 绑定，`@` Bindings

*地址，at* 运算符 `@` 实现了在咱们将某个值与模式匹配测试的同时，创建出保存该值的一个变量来。在下面清单 18-29 中，咱们打算测试某个 `Message::Hello` 的 `id` 是否在范围 `3..=7` 中。咱们还要将该值绑定到变量 `id_variable`，从而咱们可以在与该支臂相关的代码中使用他。咱们可将这个变量命名为 `id`，与那个字段相同，而对于这个示例，咱们将使用不同的名字。


```rust
fn main() {
    enum Message {
        Hello { id: u32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println! ("找到位于范围内的一个 id: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println! ("找到位于另一范围的一个 id");
        },
        Message::Hello { id } => println! ("找到别的一个 id: {}", id),
    }
}
```

*清单 18-29：于模式中在测试某个值的同时，使用 `@` 将其加以绑定*

这个示例将打印 `找到位于范围内的一个 id: 5`。通过在范围 `3..=7` 前指明 `id_variable @`，咱们在测试该值与这个范围匹配的同时，捕获了与该范围匹配的任何值。

在第二支臂中，那里咱们只在模式中指定了一个范围，与该支臂相关的代码，就不会有包含了这个 `id` 字段具体值的一个变量。这个 `id` 字段的值，可能是 `10`、`11` 或 `12`，但那个支臂下的代码却不清楚其为何。由于咱们不曾将那个 `id` 值保存在某个变量中，模式代码便无法使用 `id` 字段的值。

在最后支臂中，那里咱们指定了一个不带范围的变量，咱们确实令到了这个值，在该支臂代码中一个名为 `id` 的变量里可供使用。原因在于咱们使用了结构体字段速记语法，the struct field shorthand syntax。不过咱们不曾在这个支臂中，应用任何测试到这个 `id` 字段中的值，正如咱们对前两个支臂所做的那样：那么所有值都将匹配这个支臂。

运用 `@` 实现了在一个模式里，对某个值的测试，并将其保存在某个变量中。


## 本章小结

Rust 的模式，在区分不同类别数据方面非常有用。当在 `match` 表达式中用到模式时，Rust 就会确保咱们的那些模式，涵盖每个可能的值，否则咱们的程序便不会编译。`let` 语句与函数参数中的模式，会令到这两种结构更为有用，在实现值解构为一些更小的部分的同时，赋值给一些变量。咱们可以创建出简单抑或复杂的模式，来适合咱们的需求。

接下来，作为本书倒数第二章，咱们将数种 Rust 特性中，一些高级的方面。
