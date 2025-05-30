# 使用 `use` 关键字将路径纳入作用域

**Bringing Paths into Scope with the `use` Keyword**

必须写出调用函数的路径，可能会让人感到不便和重复。在 [清单 7-7](paths.md#list_7-7) 中，无论我们选择 `add_too_waitlist` 函数的绝对路径还是相对路径，每次要调用 `add_too_waitlist` 时，都必须指定 `front_of_house` 和 `hosting`。幸运的是，有种方法可以简化这一过程：我们可以使用 `use` 关键字，为路径创建一个快捷方式，然后在作用域的其他地方，使用这个较短的名字。

在清单 7-11 中，我们将 `crate::front_of_house::hosting` 模组，引入了 `eat_at_restaurant` 函数的作用域，因此只需指明 `hosting::add_too_waitlist`，就能调用 `eat_at_restaurant` 中的 `add_too_waitlist` 函数。

<a name="list_7-11"></a>
文件名：`src/lib.rs`

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

*清单 7-11：使用 `use` 关键字，将模组纳入作用域*

在某个作用域中加入 `use` 关键字及某个路径，类似于在文件系统中创建一个符号链接。而通过在代码箱根中添加 `use crate::front_of_house::hosting`，`hosting` 现在便是该作用域中的一个有效名字，就像在代码箱根中已经定义了 `hosting` 模组一样。使用 `use` 关键字纳入作用域的路径，也会像其他路径一样检查隐私。

请注意，`use` 关键字只会为该 `use` 语句所在的特定作用域，创建快捷方式。清单 7-12 将 `eat_at_restaurant` 函数，移到了一个名为 `customer` 的新子模组中，该子模组的作用域与 `use` 语句的作用域不同，因此这个函数体就无法编译。


文件名：`src/lib.rs`

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}
```

*清单 7-12：`use` 语句只适用于他所在的作用域*

编译器错误显示，快捷方式不再适用于这个 `customer` 模组：

```console
$ cargo build
   Compiling restaurant v0.1.0 (/home/peng/rust-lang/restaurant)
error[E0433]: failed to resolve: use of undeclared crate or module `hosting`
  --> src/lib.rs:33:9
   |
33 |         hosting::add_to_waitlist();
   |         ^^^^^^^ use of undeclared crate or module `hosting`

warning: unused import: `crate::front_of_house::hosting`
  --> src/lib.rs:28:5
   |
28 | use crate::front_of_house::hosting;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

For more information about this error, try `rustc --explain E0433`.
warning: `restaurant` (lib) generated 1 warning
error: could not compile `restaurant` due to previous error; 1 warning emitted
```

请注意这里还有个警告，即在其作用域中那个 `use` 不再被用到！要解决这个问题，同时要把这个 `use` 语句，移到 `customer` 模组中，或者在 `customer` 子模组中，使用 `super::hosting` 引用父模组中的那个快捷方式。


## 创建惯用 `use` 路径

**Creating Idiomatic `use` Paths**

在 [清单 7-11](#list_7-11) 中，咱们可能想知道，为什么我们指定了使用 `crate::front_of_house::hosting`，然后在 `eat_at_restaurant` 中调用 `hosting::add_to_waitlist`，而不是像清单 7-13 中那样，为了达到同样的效果，指定出直达 `add_too_waitlist` 函数的 `use` 路径。

文件名：`src/lib.rs`

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
}
```

*清单 7-13：使用 `use` 将 `add_to_waitlist` 纳入作用域，这种非惯常用法*

尽管清单 7-11 和清单 7-13 都完成了同样任务，但清单 7-11 是将函数纳入作用域的惯用方法。使用 `use` 将函数的父模组纳入作用域，意味着我们必须在调用该函数时，指明其父模组。在调用函数时指定父模组，可以清楚地表明该函数不是本地定义的，同时仍然最大限度减少了完整路径的重复。清单 7-13 中的代码并不明晰，因为`add_too_waitlist` 的定义位置不明晰。

而另一方面，在以 `use` 关键字引入结构体、枚举及其他一些项目时，指定完整路径就是种习惯做法。清单 7-14 展示了将标准库的 `HashMap` 结构体，纳入某个二进制代码箱作用域的惯用方法。

文件名：`src/main.rs`

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```

*清单 7-14：以惯用方式将 `HashMap` 纳入作用域*

这个习惯用法背后，并没有很强的理由：他只是种约定俗成的习惯，人们已经习惯了用这种方式，来阅读和编写 Rust 代码。

如果我们要使用 `use` 语句，将两个同名项目纳入作用域，那么这个习惯用法就是个例外，因为 Rust 不允许这样做。清单 7-15 展示了如何将两个名字相同但父模组不同的 `Result` 类型纳入作用域，以及如何引用他们。

文件名：`src/lib.rs`

```rust
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --跳过--
}

fn function2() -> io::Result {
    // --跳过--
}
```

*清单 7-15：将两个同名类型纳入同一作用域，就需要用到他们的父模组*

正如咱们所见到的，使用父模组就可以区分这两种 `Result` 类型。而如果我们指定了 `use std::fmt::Result` 和 `use std::io::Result`，我们就会在同一作用域中，有了两个 `Result` 类型，但 Rust 不会知道，我们使用 `Result` 时指的是哪个。


## 使用 `as` 关键字提供新名字

**Providing New Names with the `as` Keyword**

还有另一种方法，可以解决将两个同名类型纳入同一作用域的问题：在路径之后，我们可以指定 `as` 和一个新的本地名字，或 *别名，alias*。清单 7-16 展示了另一种编写清单 7-15 代码的方法，即使用 `as` 重命名两个 `Result` 类型中的一个。

文件名：`src/lib.rs`

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --跳过--
}

fn function2() -> IoResult {
    // --跳过--
}
```

*清单 7-16：将某个类型纳入作用域时，使用 `as` 关键字对其重命名*

在第二个 `use` 语句中，我们为 `std::io::Result` 类型，选择了新名字 `IoResult`，这不会与我们同时引入该作用域的 `std::fmt` 中的 `Result` 冲突。清单 7-15 和清单 7-16 都被认为是惯用的，因此选择由咱们自己决定！


## 使用 `pub use` 重新导出名字

**Re-exporting Names with `pub use`**

当我们使用 `use` 关键字将名字纳入作用域时，新作用域中可用的这个名字是私有的。为了使调用咱们代码的代码，就好像这个名字是在咱们代码作用域中定义的那样，引用这个名字，我们可以将 `pub` 和 `use` 结合使用。这种技术被称为 *再导出，re-exporting*，因为我们在将某个项目纳入作用域的同时，还让其他人可以将该项目纳入他们的作用域了。

清单 7-17 展示清单 7-11 中的代码，其中根模组中的 `use` 已更改为 `pub use`。

文件名：`src/lib.rs`

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

<a name="listing-7-17"></a> *清单 7-17：使用 `pub use` 让任何代码都可以从新作用域中使用某个名字*

在这一修改之前，外部代码必须使用 `restaurant::front_of_house::hosting::add_too_waitlist()` 路径，来调用 `add_to_waitlist` 函数，这还需要 `front_of_house` 模组被标记为 `pub`。现在，这个 `pub use` 已经从该根模组中，重新导出了 `hosting` 模组，外部代码就可以使用 `restaurant::hosting::add_to_waitlist()` 路径了。

当咱们代码的内部结构，与调用咱们代码的程序员对有关领域的理解有差别时，重新导出就非常有用。例如，在这个餐馆的比方中，经营餐馆的人考虑的是“前厅”和“后厨”。但光顾餐厅的顾客，可能不会从这些角度来考虑餐厅的各个部分。通过 `pub use`，我们可以一种结构编写代码，而暴露出不同的结构。这样做可以使我们的程序库井井有条，便于编写这个库的程序员，也便于调用这个库的程序员。我们将在第 14 章的 [“使用 `pub use` 导出好用的公开 API”](../crates-io/publishing.md##使用-pub-use-导出便利的公开-api) 小节，介绍 `pub use` 的另一个示例，以及他对代码箱文档的影响。


## 使用外部包

**Using External Packages**

在第 2 章中，我们编写了个竞猜游戏项目，该项目使用了名为 `rand` 的外部包，来获取随机数。为了在项目中使用 `rand`，我们在 `Cargo.toml` 中添加了这一行：

文件名：`Cargo.toml`

```toml
rand = `0.8.5`
```

在 `Cargo.toml` 中将 `rand` 添加为依赖项后，Cargo 就会从 [crates.io](https://crates.io/) 下载 `rand` 包和任何依赖项，并将 `rand` 提供到咱们的项目。

然后，为了将 `rand` 定义带入咱们包的作用域，我们添加了以该代码箱名字 `rand` 开头的 `use` 行，并列出了咱们打算带入作用域的那些项目。清回想一下，在第 2 章 [“生成随机数”](../Ch02_Programming_a_Guessing_Game.md#生成随机数) 小节中，我们将 `Rng` 特质引入了作用域，并调用了 `rand::thread_rng` 函数：

```rust
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_rang(1..=100);
}
```

Rust 社区成员在 [crates.io](https://crates.io/) 上，已经提供了许多包，将其中任何包拉入咱们的包，都需要这些同样步骤：在咱们包的 `Cargo.toml` 文件中列出他们，并使用 `use` 将他们代码箱中的项目，引入到作用域。

请注意，标准库 `std` 同样属于相对于咱们包外部的一个代码箱。因为标准库是随 Rust 语言一起提供的，所以我们不需要修改 `Cargo.toml` 来包含 `std`。但我们确实需要用 `use` 来引用他，以便将其中的项目引入咱们包的作用域。例如，对于 `HashMap` 我们可以使用这行：

```rust
use std::collections::HashMap;
```

这是个以 `std`（标准库代码箱名字）开头的绝对路径。


## 使用嵌套路径清理大型 `use` 清单

**Using Nested Paths to Clean Up Large `use` Lists**


如果我们使用的是定义在同一代码箱，或同一模组中的多个项目，而将每个项目单独列一行，就会占用文件中的大量垂直空间。例如，我们在 [清单 2-4](Ch02_Programming_a_Guessing_Game.md#list_2-4) 的猜谜游戏中，使用的这两条 `use` 语句，将 `std` 中的项目引入了作用域：

文件名：`src/main.rs`

```rust
// --跳过--
use std::cmp::Ordering;
use std::io;
// --跳过--
```

实际上，我们可以使用嵌套路径，在一行中将同样这些项目纳入作用域。具体做法是指定出路径的共同部分，后面加两个冒号，然后在这些项目的路径不同部分列表周围，加上花括号，如清单 7-18 所示。

文件名：`src/main.rs`

```rust
// --跳过--
use std::{cmp::Ordering, io};
// --跳过--
```

*清单 7-18：指定嵌套路径，将具有同样前缀的多个项目纳入作用域*

在大型程序中，使用嵌套路径将同一代码箱或同一模组中的许多项目引入作用域，可以大大减少所需的单独 `use` 语句的数量！

在路径的任何层级，我们都可以使用嵌套路径，在对共用子路径的两个 `use` 语句加以组合时，这尤为有用。例如，清单 7-19 给出了两个 `use` 语句：一个将 `std::io` 引入作用域，另一个将 `std::io::Write` 引入作用域。


文件名：`src/lib.rs`

```rust
use std::io;
use std::io::Write;
```

*清单 7-19：两个 `use` 语句，其中一个是另一个的子路径*

这两个路径的共同部分是 `std::io`，正是完整的第一个路径。要将这两个路径合并为一条 `use` 语句，我们可以在嵌套路径中使用 `self`，如清单 7-20 所示。


文件名：`src/main.rs`

```rust
use std::io::{self, Write};
```

*清单 7-20：将清单 7-19 中的路径合并为一个 `use` 语句*

这行代码会将 `std::io` 和 `std::io::Write` 纳入作用域。


## 全局操作符，`*`

**The Glob Operator**

如果我们想将某个路径中定义的 *所有* 公开项目都纳入作用域，可以在指定路径后加上 `*` 这个全局操作符：


```rust
use std::collections::*;
```

这条 `use` 语句会将 `std::collections` 中定义的所有公开项目引入当前作用域。使用全局操作符时要当心！全局性会使咱们更难分辨，哪些名称是在作用域中，以及程序中使用的名字是在何处定义。

全局操作符通常在测试时使用，以便将所有被测试内容引入 `tests` 模块；我们将在第 11 章 [“如何编写测试”](../automated_tests/howto.md#怎样编写测试) 小节中讨论这个问题。全局操作符有时也作为前奏模式，the prelude pattern，的一部分使用：有关该模式的更多信息，请参阅 [标准库文档](https://doc.rust-lang.org/std/prelude/index.html#other-preludes)。


（End）


