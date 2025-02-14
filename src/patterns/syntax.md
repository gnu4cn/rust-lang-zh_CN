# 模式语法

**Pattern Syntax**


在这个小节中，咱们会聚齐模式方面的全部有效语法，并讨论因何及何时会打算使用这每种的语法。


## 匹配字面值

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


## 匹配命名变量

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



## 多个模式

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


## 使用 `..=` 匹配值范围

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


## 将值拆散的解构

**Destructuring to Break Apart Values**


咱们还可以运用模式，来解构结构体、枚举及元组，从而用到这些值的不同部分。下面就来贯穿这各个的值。


### 解构结构体

**Destructuring Stucts**


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


### 解构枚举

**Destructuring Enums**


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


### 嵌套结构体与枚举的解构

**Destructuring Nested Structs and Enums**


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


### 解构结构体与元组

**Destructing Structs and Tuples**


咱们甚至可以更复杂的方式，对解构模式进行混用、匹配及嵌套。下面的示例，给出了一种复杂的解构，其中在一个元组中，嵌套了结构体与元组，并讲全部原生值解构了出来：


```rust
    let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10});
```

此代码实现将复杂类型，拆分为其各个组件部分，从而咱们可单独使用咱们所感兴趣的那些值。

以模式来解构数据，是各自独立使用诸如结构体中各个字段值，此类各部分值的一种便利方式。


## 忽略模式中的某些值

**Ignoring Values in a Pattern**


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


### 使用嵌套的 `_` 忽略某个值的部分

**Ignoring Parts of a Value with a Nested `_`**


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


### 通过以 `_` 开头的名字，忽略某个未用到的变量

**Ignoring an Unused Variable by Starting Its Name with `_`**


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


### 使用 `..` 忽略值的剩余部分

**Ignoring Remaining Parts of a Value with `..`**


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


## 使用匹配卫兵的额外条件

**Extra Conditionals with Match Guards**


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


## `@` 绑定

**`@` Bindings**


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


# 本章小结

Rust 的模式，在区分不同类别数据方面非常有用。当在 `match` 表达式中用到模式时，Rust 就会确保咱们的那些模式，涵盖每个可能的值，否则咱们的程序便不会编译。`let` 语句与函数参数中的模式，会令到这两种结构更为有用，在实现值解构为一些更小的部分的同时，赋值给一些变量。咱们可以创建出简单抑或复杂的模式，来适合咱们的需求。

接下来，作为本书倒数第二章，咱们将数种 Rust 特性中，一些高级的方面。


（End）


