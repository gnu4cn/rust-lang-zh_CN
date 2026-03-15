# 在模组下控制作用域及隐私

在这一小节中，我们将讨论模组和模组系统的其他部分，即

- *路径，path*，他允许咱们给项目命名；
- 将路径带入作用域的 `use` 关键字；
- 以及将项目构造为公开的 `pub` 关键字等。

我们还将讨论 `as` 关键字、外部包及 `glob` 操作符。


## 模组速查表

在深入模组与路径的细节前，我们在此提供一个快速参考，有关模组、路径、`use` 关键字及 `pub` 关键字在编译器中的工作原理，以及大多数开发人员组织他们的代码的方式。尽管我们将在这一章中逐一讲解这些规则的示例，但这里也是回顾模组工作原理的绝佳之处。

- **从代码箱根开始**：编译代码箱时，编译器首先在代码箱根文件（通常是库代码箱的 `src/lib.rs`及二进制代码箱的 `src/main.rs`）中查找要编译的代码；

+ **声明模组**：在代码箱根文件中，咱们可声明新的模组；假设咱们以 `mod garden;` 声明了个 ”花园“ 模组。编译器将在以下位置查找该模组的代码：
    - 内联式，在花括号内，其替换 `mod garden` 后分号；
    - 在文件 `src/garden.rs` 中；
    - 在文件 `src/garden/mod.rs` 中。

+ **声明子模组**：在除代码箱根外的任何文件中，咱们都可以声明子模组。例如，咱们可在 `src/garden.rs` 中声明 `mod vegetables;`。编译器将在以下位置的命名为父模组的目录下，查找该子模组的代码：
    - 内联式，直接跟在 `mod vegetables` 之后，代替分号的花括号内；
    - 在文件 `src/garden/vegetables.rs` 中；
    - 在文件 `src/garden/vegetables/mod.rs` 中。

- **到模组中代码的路径**：一旦模组成为咱们代码箱的一部分，只要隐私规则允许，咱们就可以使用到代码的路径，从同一代码箱的任何其他地方引用该模组中的代码。例如，花园蔬菜模组中的 `Asparagus` （芦笋）类型，将于 `crate::garden::vegetables::Asparagus` 处找到。

- **私有与公开**：默认情况下，模组内的代码对其父模组属于私有的。要构造模组为公开，就要以 `pub mod` 而不是 `mod` 声明他。要将公开模组中的项目也构造为公开，就要在他们的声明前使用 `pub`。

- **`use` 关键字**：在某一作用域内，`use` 关键字创建项目的快捷方式以减少长路径的重复。在任何可以引用 `crate::garden::vegetables::Asparagus` 的作用域中，咱们都可以 `use crate::garden::vegetables::Asparagus;` 创建快捷方式，从那以后，咱们就只需写下 `Asparagus` 即可在这一作用域中使用该类型。

在这里，我们创建一个名为 `backyard` 的二进制代码箱说明这些规则。这个代码箱的目录同样名为 `backyard`，包含以下这些文件及目录：


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


这一情形下，代码箱根文件为 `src/main.rs`，他包含：


文件名：`src/main.rs`

```rust
use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println! ("I'm growing {:?}!", plant);
}
```

其中 `pub mod garden;` 行告诉编译器，要包含其在 `src/garden.rs` 中找到的代码，即:


文件名：`src/garden.rs`

```rust
pub mod vegetables;
```


在这里，`pub mod vegetables;` 表示 `src/garden/vegetables.rs` 中的代码也会包含。该代码是:


```rust
#[derive(Debug)]
pub struct Asparagus {}
```

现在我们来深入这些规则的细节并在操作中演示他们！


## 以模组组织相关代码

所谓 *模组，module*，让我们可以组织代码箱内的代码，出于可读性及重用目的。模组还允许我们控制程序项目（译注：变量、类型及函数等）的 *隐私*，因为模组内的代码默认是私有的。私有项目属于内部的实现细节，对外部使用不可用。我们可选择构造模组及其内项目公开，这会暴露他们以允许外部代码使用并依赖他们。

举个例子，我们来编写一个库代码箱，提供餐厅功能。我们将定义函数的签名，但将他们的主体留空，以专注于代码的组织而不是餐厅的实现。

在餐饮业，餐厅的一些部分称为前厅，front of house，其他部分称为后厨，back of house。*前厅* 是顾客所在的地方；包括餐厅领台为顾客安排座位、服务员接受点单和付款，以及调酒师调制饮料的地方。*后厨* 是厨师和厨工在厨房工作、洗碗工清洁以及经理进行行政工作的地方。

为了以这种方式架构我们的代码箱，我们可将其功能组织成嵌套的模组。请通过运行 `cargo new restaurant --lib` 创建一个名为 `restaurant` 的新库；然后输入下面清单 7-1 中的代码到 `src/lib.rs` 中，定义一些模组与函数签名。这段代码属于前台部分。


<a name="listing_7-1"></a>
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

**清单 7-1**：`front_of_house` 模组包含其他模组，而其他模组包含函数


我们以后跟模组名字（在这一情形下，即 `front_of_house`）的 `mod` 关键字定义一个模组。该模组的主体随后位于花括号内。在模组内部，我们可以放置其他模组，就像本例中的 `hosting` 和 `serving` 模组。模组还可以容纳其他程序项目的定义，比如结构体、枚举、常量、特质，以及清单 7-1 中的函数等。

通过使用模组，我们可将相关定义组织在一起并表明他们相关的原因。使用这段代码的程序员可以根据分组浏览代码，而不必通读所有定义，从而更容易找到与他们攸关的定义。往这段代码增加新功能的程序员将清楚要将代码放在何处以保持程序组织有序。

早先我们曾提到 `src/main.rs` 与 `src/lib.rs` 称为代码箱根。他们名字的原因是这两个文件的内容构成了代码箱模组结构，称为 *模组树，module tree* 根处名为 `crate` 的模组。

清单 7-2 展示了清单 7-1 中结构的模组树。

<a name="listing_7-2"></a>
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

**清单 7-2**：清单 7-1 中代码的模组树

该树显示了其中一些模组如何嵌套别的模组中；例如，`hosting` 嵌套在 `front_of_house` 内。该树还显示了一些模组属于 *同辈，siblings*，意味着他们定义在同一模组下；`hosting` 和 `serving` 就属于定义在 `front_of_house` 内的同辈模组。当模组 A 包含在模组 B 内时，我们就说模组 A 是模组 B 的 *子模组*，而模组 B 是模组 A 的 *父模组*。请注意，整个模组树的根位于名为 `crate` 的隐式模组处。

模组树可能会让咱们想起咱们计算机上文件系统的目录树；这是个非常恰当的比较！就像文件系统中的目录一样，咱们可使用模组来组织代码。而就像目录下的文件一样，我们需要一种找到咱们模组的方法。


（End）


