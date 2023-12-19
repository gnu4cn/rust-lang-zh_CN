# 定义控制作用域和隐私的模组

**Defining Modules to Control Scope and Privacy**


在本节中，我们将讨论模组和模组系统的其他部分，即允许咱们给项目取名的 *路径，paths*、将路径引入作用域的 `use` 关键字，以及将项目构造为公开的 `pub` 关键字等。我们还将讨论 `as` 关键字、外部包和 `glob` 操作符。

首先，我们将以一份方便咱们今后组织咱们代码的规则清单开始。然后，我们将详细解释每一条规则。


## 模组速查表

**Modules Cheat Sheet**


这里我们提供了有关模组、路径、`use` 关键字和 `pub` 关键字，在编译器中如何运作，以及大多数开发人员如何组织代码的快速参考。我们将在本章节中，逐个讨论这些规则的示例，但这是个很到的，作为模组工作原理的提醒的地方。

- **从代码箱根开始**：编译代码箱时，编译器首先会在代码箱根文件（通常是库代码箱的 `src/lib.rs`，或二进制代码箱的 `src/main.rs`）中，查找要编译的代码；

+ **声明模组**：在代码箱根文件中，咱们可以声明出新的模组；比如，咱们以 `mod garden;`，声明一个 ”花园“ 模组。编译器将在下面这些地方，查找该模组的代码：
    - 内联式，在替换 `mod garden` 后分号的花括号内；
    - 文件 `src/garden.rs` 中；
    - 文件 `src/garden/mod.rs` 中。

+ **声明子模组**：在除代码箱根外的任何文件中，都可以声明子模组。例如，咱们可能会在 `src/garden.rs` 中，声明 `mod vegetables;`。编译器将在下面这些地方的父模组目录中，查找该子模组的代码：
    - 内联式，直接跟在 `mod vegetables` 之后，代替分号的花括号内；
    - 文件 `src/garden/vegetables.rs` 中；
    - 文件 `src/garden/vegetables/mod.rs` 中。

- **模组中代码的路径**：一旦某个模组成为咱们代码箱的一部分，只要隐私规则允许，咱们就可以在同一代码箱的任何其他地方，使用代码的路径来引用该模组中的代码。例如，花园蔬菜模组中的 `Asparagus` （芦笋）类型，就可以在 `crate::garden::vegetables::Asparagus` 处找到。

- **私有与公开**：模组内的代码默认对其父模组是私有的。要将某个模组构造为公开，可使用 `pub mod` 代替 `mod` 声明该模组。要将某个公开模组中的项目，也构造为公开，请在其声明前使用 `pub`。

- **`use` 关键字**：在作用域中，`use` 关键字会创建出到项目的快捷方式，以减少长路径的重复。在任何可以引用 `crate::garden::vegetables::Asparagus` 的作用域中，咱们都可以使用 `use crate::garden::vegetables::Asparagus;` 创建出一个快捷方式，并在那以后，咱们就只需写下 `Asparagus`，就可以在该作用域中使用这个类型。

在此，我们创建了一个说明这些规则的名为 `backyard` 的二进制代码箱。这个代码箱的目录也称为 `backyard`，其中包含下面这些文件和目录：


```console
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```


本例中的代码箱根文件是 `src/main.rs`，他包含：


文件名：`src/main.rs`

```rust
use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println! ("I'm growing {:?}!", plant);
}
```


`pub mod garden;` 这行告诉编译器，要包含其在 `src/garden.rs` 中找到的代码，即:


文件名：`src/garden.rs`

```rust
pub mod vegetables;
```


在这里，`pub mod vegetables;` 表示也包含了 `src/garden/vegetables.rs` 中的代码。这些代码是:


```rust
#[derive(Debug)]
pub struct Asparagus {}
```


现在就来进入到这些规则的细节，并在实际操作中对他们进行演示吧！


## 在模组中把有关联的代码组织起来

**Grouping Related Code in Modules**

*模组（modules）* 实现了为代码可读性与易于重用目的，而将代码，组织在代码箱里。由于模组里的代码，默认是私有的，因此模组还实现了各个项目 *隐私（privacy）* 的控制。私有项目是一些对外部用途不可用的内部实现细节。可将模组及模组中的那些程序项目，构造为公开，这样就把他们暴露出来，从而允许外部代码使用及依赖于他们。

举例来说，这里要编写一个提供饭馆功能的库代码箱。那么就会定义出一些函数签名，不过要将这些函数的函数体留作空白，而非在代码中具体实现一个饭馆出来，以专注于代码组织。

在餐饮行业，饭馆的一些部分被称作 *前台（front of house）*，而其余部分则被称作 *后厨（back of house）*。前台是顾客们所在的地方；这是饭馆领台给食客安排位置、服务员拿到菜单和买单，以及调酒师制作饮品的地方。而后厨则是大厨和厨师们在厨房做菜、洗碗工做清洁工作，以及经理们完成行政工作的地方。

为了以此种方式架构起这里代码，那么就可以将其函数，组织进一些嵌套模组中。通过运行 `cargo new --lib restaurant` 命令，创建出一个新的、名为 `restaurant` 的库；然后把下面清单 7-1 中的代码，敲入到文件 `src/lib.rs` 里，而定义出一些模组与函数签名。下面是饭馆前台部分：

文件名：`src/lib.rs`

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```

*清单 7-1：包含着别的一些、又包含了一些函数的模组的一个 `front_of_house` 模组（a `front_of_house` module containing other modules that then contain functions）*

这里使用跟着模组名字（此示例中，即 `front_of_house`）的关键字 `mod`，定义的一个模组。随后就是位处一对花括号中的模组代码体（the body of the module）。在模组内部，可以有其他模组，如同此示例中的 `hosting` 与 `serving` 模组。模组还可以驻留一些别的项目，诸如结构体、枚举、常量、特质（traits），以及 -- 如同在清单 7-1 中那样的 -- 一些函数等等。

经由模组的使用，就可以将有关联的一些定义，组织在一起，并以他们因何相关而取个名字。使用此代码的程序员们，就可以根据这些分组，而非通读全部的这些定义，来浏览代码，那么在找到他们想要使用的那些定义时，就会容易一些。而对于要往该代码增加新功能的那些程序员，就清楚在哪里放置代码，来保持程序组织有序。

早先曾提到 `src/main.rs` 与 `src/lib.rs` 都叫做代码箱根（crate root）。他们之所以有着这样的名字，是由于这两个文件的内容，都形成了位处该代码箱的模组结构（the root of the crate's module structure），又称为 *模组树（module tree）*根部处，名为 `crate` 的模组。

以下清单 7-2 给出了清单 7-1 中结构的模组树（模组结构）：

```console
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

*清单 7-2：清单 7-1 中代码的模组树*

该树展示了一些模组是怎样嵌套在另一模组内部的（比如，`hosting` 就嵌套在 `front_of_house` 里头）。该树还展示了一些模组与其他模组互为 *姊妹关系（siblings）*，即他们是定义在同一模组中的（`hosting` 与 `serving` 都是定义在 `front_of_house` 模组中）。继续以家族作比喻，那么在模组 A 为包含在模组 B 里头时，就说模组 A 是模组 B 的 *子模组（child）*，而模组 B 即为模组 A 的 *父模组（parent）*。请注意这里的整个模组树，都是以那个隐式的、名为 `crate` 模组，作为的根。

模组树或许会令人想到计算机上文件系统的目录树；这可是一个极为恰当的类比！就跟文件系统中的目录一样，使用模组是为对代码进行组织。而正如目录中的那些文件，这里需要一种找到那些模组的方法。
