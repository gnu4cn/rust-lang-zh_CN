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


## 把有关联的代码分组在模组中

**Grouping Related Code in Modules**


*模组，modules* 让我们可以把代码组织在代码箱中，以提高可读性并方便重复使用。模组还允许我们控制程序项目的私密性，因为某个模组内的代码默认为私有。私有项目是一些内部的实现细节，不供外部使用。我们可以选择将模组及其中的项目公开，这样就可以让外部代码使用和依赖他们。

例如，我们来编写一个提供餐厅功能的库代码箱。我们将定义函数的签名，但函数体留空，以专注于代码的组织，而不是实现餐厅的实现。

在餐饮业，餐厅的一些部分被称为 *前厅，front of house*，其他部分被称为 *后厨，back of house*。前厅是顾客所在的地方；包括餐厅领台为顾客安排座位、服务员接受点单和付款，以及调酒师调制饮料的地方。后厨是厨师和厨工在厨房工作、洗碗工进行清洁和经理进行行政工作的地方。

为了以这种方式架构我们的代码箱，我们可以将其函数，组织成一些嵌套的模组。请运行 `cargo new restaurant --lib`，创建一个名为 `restaurant` 的新库；然后在 `src/lib.rs` 中，输入下面清单 7-1 中的代码，定义一些模组和函数签名。下面是前台部分：


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

*清单 7-1：包含着其他包含了函数模组的 `front_of_house` 模组*


我们使用后跟模组名字（本例中为 `front_of_house`）的 `mod` 关键字，定义某个模组。随后模组的主体，位于花括号内。在模组内部，我们可以放置其他模组，就像本例中 `hosting` 和 `serving` 模组。模组还可以包含其他项目的定义，如结构体、枚举、常量、特质，以及如同清单 7-1 中的函数。

经由模组的使用，就可以将有关联的一些定义，组织在一起，并以他们因何相关而取个名字。使用此代码的程序员们，就可以根据这些分组，而非通读全部的这些定义，来浏览代码，那么在找到他们想要使用的那些定义时，就会容易一些。而对于要往该代码增加新功能的那些程序员，就清楚在哪里放置代码，来保持程序组织有序。

前面我们曾提到，`src/main.rs` 和 `src/lib.rs` 被称为代码箱根。之所以叫这两个文件为代码箱根，是因为这两个文件的内容，构成了位于该代码箱结构（即 *模组树，module tree*）根部，名为 `crate` 的模组。

清单 7-2 显示了清单 7-1 中结构的模组树。

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

该树显示了其中一些模组是如何嵌套在另一模组中的；例如，`hosting` 嵌套在 `front_of_house` 中。该树还显示其中一些模组彼此属于 *同辈，siblings*，即他们被定义在同一模组中；`hosting` 和 `serving` 是 `front_of_house` 中定义的同辈份模组。如果模组 A 包含在模组 B 中，我们就说模组 A 是模组 B 的子模组，而模组 B 是模组 A 的父模组。请注意，整个模组树的根，都位于名为 `crate` 的隐式模组下。

模组树可能会让咱们联想到电脑上的文件系统目录树；这是个非常恰当的比较！就像文件系统中的目录一样，咱们可以使用模组来组织代码。而就像目录中的文件一样，我们需要一种找到咱们模组的方法。


（End）


