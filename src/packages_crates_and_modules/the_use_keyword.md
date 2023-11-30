# 使用 `use` 关键字将路径带入作用域

**Bringing Paths into Scope with the `use` Keyword**


为调用一些函数，而不得不写出他们的路径，就会感到不便与重复。比如在清单 7-7 中，对于到 `add_to_waitlist` 函数，无论是选择绝对路径还是相对路径，在每次在打算调用 `add_to_waitlist` 时，都必须还要指明 `front_of_house` 与 `hosting`。幸运的是，有简化此过程的办法：这里可以使用 `use` 关键字，一次性创建出到某个路径的快捷方式，尔后就可以在该作用域中所有地方，使用这个较短名字了。

在下面清单 7-11 中，就将 `crate::front_of_house::hosting` 模组，带入到了 `eat_at_restaurant` 函数的作用域，由此就只须指明 `hosting::add_to_wait`，而在 `eat_at_restaurant` 中调用这个 `add_to_waitlist` 函数了。

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

*清单 7-11：使用 `use` 关键字，将模组带入到作用域*

在作用域中添加 `use` 及某个路径，与在文件系统中创建一个符号链接类似。通过在该代码箱根处，添加 `use crate::front_of_house::hosting`，那么 `hosting` 现在就是一个有效的名字，就如同这个 `hosting` 模组，已在该代码箱根中被定义过一样。使用 `use` 关键字带入到作用域中的那些路径，与任何其他路径一样，同样会检查隐私性。

请注意 `use` 关键字只会针对在该 `use` 出现的特定作用域，创建快捷方式。下面清单 7-12 将 `eat_at_restaurant` 移入到了新的名为 `customer` 的子模组中，这个模组就与那个 `use` 语句属于不同作用域了，因此那个函数体就不会编译：

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

*清单 7-12：`use` 语句只适用于其所在的作用域*

编译器错误指出，在 `customer` 模组里头，那个快捷方式不再适用：

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

请注意这里还有那个 `use` 在其作用域中已不再被使用的一个告警！为修复此问题，就同时要将那个 `use` 语句，移入到那个 `customer` 模组内部，或者在那个子 `customer` 模组内部，以 `super::hosting` 来引用父模组中的那个快捷方式。


## 创建惯用 `use` 路径

**Creating Idiomatic `use` Paths**


在上面的清单 7-11 中，你或许会想，为什么那里指定了 `use crate::front_of_house::hosting`，并随后在 `eat_at_restaurant` 函数中调用了 `hosting::add_to_waitlist`，而不是将那个 `use` 路径，指定为一直到那个 `add_to_waitlist` 函数，以达到同样目的，即如下清单 7-13 中那样。

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

*清单 7-13：使用 `use` 将 `add_to_waitlist` 带入到作用域，此为非惯用做法*

尽管清单 7-11 与 7-13 都完成了同样任务，但清单 7-11 则是以 `use` 关键字将函数带入到作用域的惯用方式。以 `use` 关键字将函数的父模组带入到作用域中，就意味着在调用该函数时，必须指明父模组。而在调用函数时指明父模组，就令到该函数是非本地函数，这一事实变得明了，同时仍旧减少了完整路径的重复。而清单 7-13 中的代码，对于 `add_to_waitlist` 在何处创建，则并不清楚。

另一方面，在使用 `use` 关键字，将结构体、枚举及其他程序项目带入时，惯用的就是指明完整路径了。下面清单 7-14 给出了将标准库的 `HashMap`，带入到某个二进制代码箱的惯用方式。

文件名：`src/lib.rs`

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```

*清单 7-14：以惯用方式将 `HashMap` 带入到作用域*

这种惯用语法背后并没有什么有力理由：他不过是业已形成的约定，且人们已经习惯了以这样的方式，阅读和编写 Rust 代码。

由于 Rust 不允许使用 `use` ，将两个有着同样名字的程序项目带入到作用域，那么这就正是此惯用语法的例外了。下面清单 7-15 给出了，怎样将两个有着同样名字，但父模组不同的 `Result` 类型带入作用域，及怎样去引用他们。

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

*清单 7-15：将有着同样名字的两种类型带入到同一作用域，就要求使用他们的父模组*

可以看到，这里使用父模组，就将两个 `Result` 类型区分开了。相反如果指明的是 `use std::fmt::Result;` 与 `use std::io::Result;`，就会得到同一作用域中的两个 `Result` 类型，而 Rust 就不明白在使用 `Result` 时，到底是要哪个了。


## 使用 `as` 关键字提供新名字

**Providing New Names with the `as` Keyword**


解决以 `use` 关键字将有着同样名字的两个类型，带入到同一作用域的问题，还有另一方法：在路径后面，可指定 `as`，与该类型的一个新本地名字，或者说 *别名（alias）*。下面清单 7-16 给出了通过将那两个 `Result` 类型中的一个，使用 `as` 关键字进行重命名，而编写清单 7-15 中代码的另一种方式。

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

*清单 7-16：在将某个类型带入作用域时，使用 `as` 关键字对其进行重命名*


在第二个 `use` 语句中，选择了 `IoResult` 作为 `std::io::Result` 类型的新名字，这就不会与同时带入到作用域的、来自 `std::fmt` 的 `Result` 冲突了。清单 7-15 与清单 7-16 都被当作惯用方式，因此选择哪个就随你所愿了！


## 使用 `pub use` 将名字重新导出

**Re-exporting Names with `pub use`**


在使用 `use` 关键字将某个名字带入到作用域中时，这个在新作用域中可用的名字即为私有的。为了那些会调用到引入作用域代码的其他代码，能够像这个名字是被定义在引入到作用域的代码的作用域中一样，对这个名字进行引用，这时就可以结合上 `pub` 与 `use` 关键字。由于这里是将某个程序项目带入到作用域，而又同时将那个程序项目构造为可被其他代码将其带入他们的作用域，因此该技巧被称为 *重导出（re-exporting）*。

下面清单 7-17 给出了将根模组中的 `use` 修改为 `pub use` 后，清单 7-11 中的代码。

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

*清单 7-17：使用 `pub use`，于一个新作用域处将某个名字构造为对任意代码可用*

在此项修改之前，外部代码必须通过使用路径 `restaurant::front_of_house::hosting::add_to_waitlist()`，来调用其中的 `add_to_waitlist` 函数。现在既然这个 `pub use` 已将该 `hosting` 模组，自根模组中重新导出，那么外部代码现在就可以使用 `restaurant::hosting::add_to_waitlist()` 路径了。

在所编写代码的内部结构，与调用代码的程序员们对该领域有着不同设想时，重导出就是有用的。比如，在这个饭馆的比喻中，运营该饭馆的人设想的是“前厅”与“后厨”。但造访饭馆的食客，或许不会用这样的词汇，来认识饭馆的这些部位。有了 `pub use`，就可以一种结构来编写代码，而以另一种结构将代码暴露出来。这样做就让这个库，对于在该库上编写代码的程序员，与调用这个库的程序员，均具备良好的组织。在第 14 章的 [“运用 `pub use` 导出便利的公共 API”](Ch14_More_about_Cargo_and_Crates_io.md#使用-pub-use-导出好用的公开-api) 小节，将会看到另一个 `pub use` 的示例，并了解他是怎样影响到代码箱的文档。


## 使用外部 Rust 包

**Using External Packages**


在第 2 章中，那里曾编写了用到名为 `rand` 外部包来获取一个随机数的猜数游戏项目。为了在项目中使用 `rand`，那里曾添加下面这行到 `Cargo.toml` 文件：

文件名：`Cargo.toml`

```toml
rand = `0.8.3`
```

将 `rand` 作为依赖项添加到 `Cargo.toml`，就告诉 Cargo，去 [crates.io](https://crates.io/) 下载那个 `rand` 包和任何的依赖项，而令到 `rand` 对此项目可用。

随后为了将 `rand` 的一些定义，带入到所编写的包中，这里添加了以代码箱名字，`rand`，开头，并列出了打算要带入到作用域中的那些条目的一个 `use` 行。回顾第 2 章中的 [“生成一个随机数”](Ch02_Programming_a_Guessing_Game.md#生成随机数) 小节，那里就将那个 `Rng` 特质，带入到了作用域，并调用了 `rand::thread_rng` 函数：

```rust
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_rang(1..=100);
}
```

Rust 社群业已构造了可在 [crates.io](https://crates.io/) 上取得的许多 Rust 包，而将任意的这些包，拉取进入自己的包，都涉及到这些同样步骤：将他们列在自己包的 `Cargo.toml` 文件中，并使用 `use` 来将他们代码箱中的条目，带入到作用域中。

请注意标准库（`std`）同样是个相对本地包的外部代码箱。由于标准库是 Rust 语言本身附带的，因此就无需修改 `Cargo.toml` 文件为包含 `std`。但为了将 `std` 中的条目带入到本地包作用域，是需要以 `use` 来引用他。比如，以 `HashMap` 来说，就要使用下面这行：

```rust
use std::collections::HashMap;
```

这是一个以 `std`，即标准库代码箱名字，开头的绝对路径。


## 运用嵌套路径，清理大型 `use` 清单

**Using Nested Paths to Clean Up Large `use` Lists**


在用到定义在同一代码箱或同一模组中的多个条目时，若各自行上地列出这些条目，那么就会占据文件中的很多纵向空间。比如，清单 2-4 中的猜数游戏里，就有下面这两个 `use` 语句，他们将 `std` 中的两个条目带入到作用域：

文件名：`src/main.rs`

```rust
// --跳过--
use std::cmp::Ordering;
use std::io;
// --跳过--
```

相反，这里就可以使用嵌套路径，来在一个行中，把来自同一代码箱或包的那些条目，带入到作用域。通过指明路径的共同部分，接上一对冒号，及随后的花括号封闭包围起来的那些路径各异部分的清单，就完成了这一点，如下代码清单 7-18 所示。

文件名：`src/main.rs`

```rust
// --跳过--
use std::{cmp::Ordering, io};
// --跳过--
```

*清单 7-18：指定出嵌套路径，来将多个有着同样前缀的程序项目带入到作用域*

在更为大型的程序中，使用嵌套路径，将许多的程序项目，从同一代码箱或模组带入到作用域，可极大地减少所需的单独 `use` 语句数目！

在路径中的任何级别，都可使用嵌套路径，在对两个共用了子路径的 `use` 语句进行组合时，这是有用的。比如下面清单 7-19 就给出了两个 `use` 语句：一个将 `std::io` 带入到作用域，而另一个则是将 `std::io::Write` 带入到作用域。

文件名：`src/lib.rs`

```rust
use std::io;
use std::io::Write;
```

*清单 7-19：其中一个为另一个子路径的两个 `use` 语句*

这两个路径的共同部分，即是 `std::io`，且那就是完整的第一个路径。为将这两个路径融合为一个 `use` 语句，这里可在嵌套路径中，使用 `self` 关键字，如下清单 7-20 中所示。

文件名：`src/main.rs`

```rust
use std::io::{self, Write};
```

*清单 7-20：将清单 7-19 中的两个路径组合为一个 `use` 语句*

这行代码就将 `std::io` 与 `std::io::Write` 带入到了作用域。


## 全局操作符

**The Glob Operator**


在打算将某个路径中的 *全部，all* 公开条目，都带入到作用域时，可将那个路径，后面跟上 `*`，即全局操作符，而予以指定：

```rust
use std::collections::*;
```

这个 `use` 语句，将定义在 `std::collections` 中的全部公开项目，都带入到了当前作用域。在使用这个全局操作符时要当心！全局带入，会导致更难于分清哪些名字是作用域中，与在所编写程序中用到的名字，是在何处定义的。

通常是在测试时，要将正在测试的全部程序项目带入到 `tests` 模组，才使用这个全局操作符；在第 11 章中的 [怎样编写测试](Ch11_Writing_Automated_Tests.md#怎样编写测试) 小节，就会讲到这个问题。在序曲模式（the prelude pattern）中，有时也会用到全局操作符：请参阅 [标准库文档](https://doc.rust-lang.org/std/prelude/index.html#other-preludes)，了解有关更多序曲模式的知识。
