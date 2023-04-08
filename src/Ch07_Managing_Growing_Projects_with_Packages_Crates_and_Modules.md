# 用代码包、代码箱与模组来对日益增长的项目进行管理

在编写大型程序时，由于在头脑里对整个程序保持追踪已成为不可能，因此代码组织就尤为重要。通过将相关功能分组，并以截然不同的特性而将代码加以分离，就会搞清楚在哪里去找到实现了某个特定特性的代码，以及在哪里去修改某项特性的运作方式。

到目前为止，这里所编写的程序，都是在一个模组的一个文件中的。而随着项目的增长，就可以通过将项目分解为多个模组及多个文件，来对代码加以组织。一个代码包，可以包含多个二进制的代码箱，并可有选择地包含一个库代码箱。本章会涵盖到所有的这些技巧。对于那些极为大型、有着一套互相关联而共同演化的项目，Cargo 工具提供了工作区（workspaces）概念，关于工作区，将在第 14 章的 [Cargo 工作区](Ch14_More_about_Cargo_and_Crates_io.md#cargo-workspaces)中讲到。

除了实现功能上的分组（grouping functionality）外，对功能实现细节的封装，还实现了更高层次上的代码重用：一旦实现了某个操作，其他代码就可以在无需掌握其实现原理的情况下，通过该代码的公共接口，对该实现代码加以调用。编写代码的方式，就定义了哪些部分是公开给其他代码使用的，哪些部分是私有实现细节而对其修改权力有所保留。这是对那些必须保留在头脑中细节实现数量，而有所限制的另一种方式（in addition to grouping functionality, encapsulating implementation details lets you reuse code at a higher level: once you've implemented an operation, other code can call that code via the code's pulic interface without knowing how the implementation works. The way you write code defines which part are public for other code to use and which parts are private implementation details that you reserve the right to change. This is another way to limit the amount of detail you have to keep in your head）。

而与此相关的概念，便是作用域（scope）：代码被编写出处的嵌套上下文，有着定义在所谓 “在作用域中（in scope）” 的一套名字。在读、写及编译代码时，程序员与编译器，二者都需要掌握，在某个特定点位处的某个特定名字，是否是指向了某个变量、函数、结构体、枚举、模组、常量或别的项目，以及该名字所指向项目的意义。创建作用域，及将一些名字的在或不在某个作用域加以修改，都是可行的。在同一作用域中，不能有两个名字相同的项目；有一些工具，可用于找出名字冲突。

对于包括哪些细节被暴露、哪些细节为私有，以及程序中各个作用域中有哪些名字等的代码组织，Rust 有着数种特性实现对其的管理。Rust 的这些有关代码组织的特性，有时被统称为 *模组系统（module system）*，包括了：

- **代码包（packages）**：实现代码箱（crates）的构建、测试与分享的 Cargo 特性；
- **代码箱（crates）**：产生出库或可执行文件的模组树（a tree of modules that produces a library or executable）；
- **模组（modules）** 与 **`use`关键字**：实现对代码组织、作用域及路径私有的控制（let you control the organization, scope, and privacy of paths）；
- **路径（paths）**：对结构体、函数或模组等进行命名的方式（a way of naming an item, such as a struct, function, or module）。

在本章中，就要涉及到这些特性，讨论他们之间互动的原理，以及如何运用这些特性，来对作用域加以管理。在本章结束时，就会对 Rust 的模组系统有扎实掌握，并能够像专业 Rust 程序员那样，以作用域来编写程序！

## 代码包与代码箱

这里将讲到的 Rust 模组系统的头几个部分，即为代码包与代码箱。


*代码箱（a crate）* 是 Rust 编译器一次识别到的最低数量的代码（a *crate* is the smallest amount of code that the Rust compiler considers as a time）。即使运行 `rustc` 而非 `cargo`，并传递单个源码文件（就如同在第 1 章 [“编写并运行一个 Rust 程序”](Ch01_Getting_Started.md#writing-and-running-a-rust-program) 小节中曾干过的），编译器也会将那个文件，视为一个代码箱。代码箱可以包含一些模组，而这些模组则会被定义在其他的、与该代码箱一同被编译的一些文件中，就如同在接下来的小节中将看到的那样。

代码箱有着两种形式：二进制代码箱（a binary crate），或库代码箱(a library crate)。*二进制代码箱（binary crates）* 是一些可编译为能够运行的可执行程序的一些程序，譬如命令行程序或服务器。二进制代码箱必须有着一个叫做 `main` 的、定义了在可执行文件运行时所发生事情的函数。到目前为止本书中创建的全部代码箱，都是二进制代码箱。

*库代码箱* 是没有 `main` 函数的，且他们不会编译到可执行文件。相反，他们定义的是计划在多个项目下共用的功能。比如在 [第二章](Ch02_Programming_a_Guessing_Game.md#generating-a-random-number) 中用到的 `rand` 代码箱，就提供了生成随机数的功能。在多数时候当 Rust 公民提到 “代码箱（crate）” 时，他们指的就是库代码箱，并且他们将 “代码箱（crate）” 与一般编程概念中的 “库（library）” 互换使用。

*代码箱根（crate root）* 是个 Rust 编译器开始之处的源文件，并构成了代码箱的根模组（the *crate root* is a source file that the Rust compiler starts from and makes up the root module of your crate. 后面在 [定义控制作用域和私有化的模组](#defining-modules-to-control-scope-and-privacy) 小节，将深入探讨到模组概念）。

*包（a package）* 即为提供了一套功能的一个或多个代码箱的捆绑包（a *package* is a bundle of one or more crates that provides a set of functionality）。包，包含了描述如何构建那些代码箱的一个 `Cargo.toml` 文件。Cargo 本身实际上就是，包含了前面曾用于构建代码的命令行工具二进制代码箱的包。Cargo 包还包含了一个该二进制代码箱所依赖的库代码箱。别的项目便可依靠这个 Cargo 库代码箱，来运用与 Cargo 命令行工具，所用到的同样逻辑。

代码包能包含些什么，是由数条规则所确定的。一个代码包，可包含尽可能多的二进制代码箱，但却只能包含至多一个的库代码箱。一个代码包必须包含至少一个代码箱，不管是库或二进制代码箱。

下面就来看看在创建代码包时，会发生些什么。首先，这里要敲入命令 `cargo new`:

```console
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project                                                                            ✔ 
Cargo.toml  src
$ ls my-project/src                                                                        ✔ 
main.rs
```

在运行了 `cargo new` 之后，这里便使用 `ls` 来查看 Cargo 创建了些什么。在该项目目录下，有着一个 `Cargo.toml` 文件，这就给到咱们一个代码包。其中还有一个包含了 `main.rs` 的 `src` 目录。在文本编辑器中打开 `Cargo.toml` 文件，就会注意到其中并未提及 `src/main.rs`。Cargo 遵循了 `src/main.rs` 即为与该代码包同名二进制代码箱箱根，这样一条约定。与此类似，Cargo 还知道，在代码包目录包含了 `src/lib.rs` 时，那么这个代码包就包含了与该包同名的一个库代码箱，而那个 `src/lib.rs` 就是该库代码箱的箱根。Cargo 会将代码箱根文件，传递给 `rustc`，来构建出相应的库或二进制程序。

这里有一个只包含了 `src/main.rs` 的代码包，意味着他只包含了名为 `my-project` 的一个二进制代码箱。而在代码包同时包含了 `src/main.rs` 与 `src/lib.rs` 时，他就会有两个代码箱：一个二进制和一个库代码箱，二者都有着与该代码包同样的名字。通过将一些文件放入到 `src/bin` 目录，Rust 包就可以有多个二进制代码箱：其中的每个文件，都将是单独的二进制代码箱。


## 定义控制作用域和隐私的模组

在本小节中，这里会讲到模组与模组系统的其他部分，分别是实现对各种项目（items, 变量、函数、结构体、枚举、模组、常量或别的项目）命名的 *路径（paths）*；将某个路径引入到作用域的 `use` 关键字；以及将那些项目构造为公共项目的 `pub` 关键字。这里还会讨论到 `as` 关键字、外部代码包，以及全局操作符（the glob operator）等等。

首先，这里将以今后在对代码进行组织时，易于参考的一个规则列表开始。随后就会对这些规则详细解释。

### 模组备忘单（modules cheat sheet）

下面就是模组、路径、`use` 关键字与 `pub` 关键字在编译器中工作原理的快速参考，以及多数开发者组织他们代码的方式。贯穿这一整章，都将逐一介绍这些规则，而这也是作为理解 Rust 模组工作原理的极佳之处。

- **自代码箱根开始（start from the crate root）**：在编译代码箱时，编译器首先看的是代码箱根文件（对于库代码箱，通常为 `src/lib.rs`，或者二进制代码箱的 `src/main.rs`）中，要编译的代码；

+ **模组的声明（declaring modules）**：在代码箱根文件中，就可声明一些新的模组；比方说，使用 `mod gargen;`，而声明出一个 `garden` 模组。编译器将在以下位置，查找该模组的代码：
    - 内联代码（inline），位于紧随 `mod garden` 之后，取代分号的花括号里；
    - 文件 `src/garden.rs` 中；
    - 文件 `src/garden/mod.rs` 中；

+ **子模组的声明（declaring submodules）**：在任何非代码箱根文件中，都可声明出一些子模组来。比如，或许就要在 `src/garden.rs` 中，声明出 `mod vegetables；`；编译器将在那个以父模组命名的目录里的以下地方，查找那些子模组的代码：
    - 内联代码，直接跟在 `mod vegetables` 之后，位处取代分号的花括号中；
    - 文件 `src/garden/vegetables.rs` 中；
    - 文件 `src/garden/vegetables/mod.rs` 中。

- **模组中代码的路径（paths to code in modules）**：一旦模组成为代码箱的一部分，就可以在这同一个代码箱中的任何地方，在隐私规则允许的情况下，运用代码路径，对那个模组中的代码加以引用。比如，那个 “garden” “vegetables” 模组中的 `Asparagus` 类型，就可在 `crate::garden::vegetables::Asparagus` 处找到。

- **私有与公共（private vs public）**：模组里的代码，默认对该模组的父模组是私有的。要令到模组成为公共的，就要使用 `pub mod` 而非 `mod` 来声明该模组。而要令到公共模组里的各个项目也成为公共的，就要在这些项目的声明之前，使用 `pub` 关键字。

- **`use` 关键字**：在某个作用域里，`use` 关键字创建出到项目的快捷方式，以减少长路径的重复。在任何能够引用到 `crate::garden::vegetables::Asparagus` 的作用域中，都可以使用 `use crate::garden::vegetables::AspAragus;` 语句，创建出一个快捷方式，并于随后只需写出 `Asparagus`，就可在该作用域中，使用那个类型。


下面是个名为 `backyard`、对这些规则加以演示的二进制代码箱。该代码箱的目录，也叫做 `backyard`，包含了下面这些文件与目录：

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

该代码箱的根文件，在此实例中即为 `src/main.rs`，包含下面的代码：

文件名：`src/main.rs`

```rust
use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println! ("I'm growing {:?}!", plant);
}
```

语句 `pub mod garden;`，表示编译器会包含他在 `src/garden.rs` 中找到的代码，也就是：

文件名：`src/garden.rs`

```rust
pub mod vegetables;
```

而语句 `pub mod vegetables;` 表示在 `src/garden/vetables.rs` 中的代码也会被编译器包含：

```rust
#[derive(Debug)]
pub struct Asparagus {}
```

现在就来进入到这些规则的细节，并在实际操作中对他们进行演示吧！


### 在模组中把有关联的代码组织起来

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


##  用于引用目录树中项目的路径

**Paths for Referring to an Item in the Module Tree**

这里以与在对文件系统进行导航时，所用到路径的同样方式，使用路径来给 Rust 指出，在何处找到模组树中的某个项目。要调用某个函数，那么就需要获悉他的路径。

而路径则可以有两种形式：

- *绝对路径（an absolute path）* 是从代码箱根开始的完整路径。对于外部代码箱的代码，绝对路径是以代码箱名字开头，而对于当前代码箱的代码，则是以字面值 `crate` 开头；
- *相对路径（a relative path）* 是从当前模组开始，并使用了 `self`、`super` 关键字，或当前模组中的某个标识符。

绝对与相对路径，后面跟着的都是以双冒号（`::`）分隔的一个或多个标识符。

回到清单 7-1 中的示例，比方说这里打算调用那个 `add_to_waitlist` 函数。这就跟问及：“那个 `add_to_waitlist` 函数的路径为何？“ 是同样的。下面的清单 7-3 包含清单 7-1，不过移除了一些模组与函数。

这里将给出从定义在该代码箱根部的一个新函数 `eat_at_restaurant`，调用那个 `add_to_waitlist` 函数的两种方式。其中那些路径都是正确的，但由于存在别的问题，而将阻止此示例如往常那样编译。这里会稍加解释为何会这样。

其中的 `eat_at_restaurant` 函数，是这里的库代码箱公共 API 的一部分，因此要将其以 `pub` 关键字进行标记。在后面的 [使用 `pub` 关键字对路径进行暴露](#exposing-paths-with-the-pub-keyword) 小节，深入到更多有关 `pub` 关键字的细节。

文件名：`src/lib.rs`

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径方式
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径方式
    front_of_house::hosting::add_to_waitlist();
}
```

*清单 7-3：使用绝对与相对路径两种方式，调用 `add_to_waitlist` 函数*

在 `eat_at_restaurant` 中，第一次调用那个 `add_to_waitlist` 函数使用的是绝对路径。由于这个 `add_to_waitlist` 函数，是定义在与 `eat_at_restaurant` 同一个代码箱中，这意味着此处可以使用 `crate` 关键字，来开始一个绝对路径。随后这里包括了到那个 `add_to_waitlist` 为止的各个后续模组。可以设想有着这同样结构的一个文件系统：即要指明路径 `/front_of_house/hosting/add_to_waitlist`，来运行那个 `add_to_waitlist` 程序；使用 `crate` 字面值名字，而自该代码箱根部开始，就跟使用 `/` 来从 shell 中文件系统根部开始类似。

在 `eat_at_restaurant` 里第二次调用 `add_to_waitlist` 时，这里使用了绝对路径。该路径是以 `front_of_house`，即那个与 `eat_at_restaurant` 定义在模组树的同一级别的模组名字，开始的。此处文件系统的等价物，将是使用路径 `front_of_house/hosting/add_to_waitlist`。以模组名字开始，就意味着该路径是绝对的。

至于究竟要选择相对路径，还是绝对路径，是要基于手头项目，而将作出的决定，并取决于是更倾向于把程序项目定义代码迁移到单独的地方，还是要把他们和要用到他们的代码放在一起。比如，在将 `front_of_house` 模组与 `eat_at_restaurant` 函数，移入到一个名为 `customer_experience` 的模组中时，那么就需要更新那个到 `add_to_waitlist` 的绝对路径，但那个相对路径则仍将有效。但如果将 `eat_at_restaurant` 函数单独移入到一个名为 `dining` 的模组，那么到 `add_to_waitlist` 函数的绝对路径就会依旧保持那样，但那个相对路径则需要被更新。由于今后多半要把代码定义和项目调用，迁移为各自独立，因此总体上偏好是要指明绝对路径。

接下来就要尝试编译清单 7-3，并找出他为何不编译的原因！所得到的错误如下清单 7-4 所示。

```console
$ cargo build                                                                ✔ 
   Compiling restaurant v0.1.0 (/home/peng/rust-lang/restaurant)
error[E0603]: module `hosting` is private
 --> src/lib.rs:9:28
  |
9 |     crate::front_of_house::hosting::add_to_waitlist();
  |                            ^^^^^^^ private module
  |
note: the module `hosting` is defined here
 --> src/lib.rs:2:5
  |
2 |     mod hosting {
  |     ^^^^^^^^^^^

error[E0603]: module `hosting` is private
  --> src/lib.rs:12:21
   |
12 |     front_of_house::hosting::add_to_waitlist();
   |                     ^^^^^^^ private module
   |
note: the module `hosting` is defined here
  --> src/lib.rs:2:5
   |
2  |     mod hosting {
   |     ^^^^^^^^^^^

For more information about this error, try `rustc --explain E0603`.
error: could not compile `restaurant` due to 2 previous errors
```

*清单 7-4：构建清单 7-3 中代码时的编译器错误*

该错误消息讲到，模组 `hosting` 是私有的。也就是说，这里的 `hosting` 模组与 `add_to_waitlist` 函数的路径都是正确的，但由于 Rust 没有到私有部分的访问权限，而不会允许咱们使用他们。在 Rust 中，全部程序项目（函数、方法、结构体、枚举、模组，以及常量等），默认对父模组都是私有的。在打算将某个项目，比如函数或结构体，构造为私有时，就要将其放在某个模组里。

父模组中的项目，是无法使用子模组内部的那些私有项目的，但子模组中的项目，则可以使用他们祖辈模组中的项目。这是由于子模组封装并隐藏了他们的实现细节，但子模组却可以看到他们被定义处的上下文（the context in which they're defined）。继续之前的那个比喻，请把这些隐私规则，想做是饭馆后台（the back office of a restaurant）：那里所发生的事情对饭馆顾客是隐私的，但后台经理们却可以看到并完成他们所运营饭馆里的全部事情。

Rust 选择了让模组系统以这种方式发挥作用，从而默认就将内部实现细节给隐藏了。如此一来，就清楚可修改内部代码的哪些部分，而不会破坏外层代码。尽管如此，Rust 还是提供了通过使用 `pub` 关键字，把某个程序项目构造为公共项目，而将子模组代码的内层部分，暴露给外层祖辈模组的选项。

###  使用 `pub` 关键字对路径进行暴露

下面回到清单 7-4 中，告知 `hosting` 模组为私有的那个错误。这里希望在父模组中的 `eat_at_restaurant` 函数，有着到那个 `hosting` 子模组中的 `add_to_waitlist` 函数的访问权限，因此就要将该模组，以 `pub` 关键字标记起来，如下面清单 7-5 中所示。

文件名：`src/lib.rs`

```rust
mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}
```

*清单 7-5：将 `hosting` 模组声明为 `pub` 以在 `eat_at_restaurant` 中使用他*

不幸的是，清单 7-5 中的代码仍以错误告终，如下清单 7-6 中所示：

```console
$ cargo build
   Compiling restaurant v0.1.0 (/home/peng/rust-lang/restaurant)
error[E0603]: function `add_to_waitlist` is private
 --> src/lib.rs:9:37
  |
9 |     crate::front_of_house::hosting::add_to_waitlist();
  |                                     ^^^^^^^^^^^^^^^ private function
  |
note: the function `add_to_waitlist` is defined here
 --> src/lib.rs:3:9
  |
3 |         fn add_to_waitlist() {}
  |         ^^^^^^^^^^^^^^^^^^^^

error[E0603]: function `add_to_waitlist` is private
  --> src/lib.rs:12:30
   |
12 |     front_of_house::hosting::add_to_waitlist();
   |                              ^^^^^^^^^^^^^^^ private function
   |
note: the function `add_to_waitlist` is defined here
  --> src/lib.rs:3:9
   |
3  |         fn add_to_waitlist() {}
   |         ^^^^^^^^^^^^^^^^^^^^

For more information about this error, try `rustc --explain E0603`.
error: could not compile `restaurant` due to 2 previous errors
```

*清单 7-6：构造清单 7-5 中代码时的编译器错误*


怎么回事呢？将 `pub` 关键字加在 `mod hosting` 前面，是把该模组构造为公共模组。有了此修改，那么在能够访问 `front_of_house` 时，就能够访问 `hosting`。但 `hosting` 模组的 *内容（contents）* 仍是私有的；将模组构造为公开，并不会将其内容构造为公开。在模组上的 `pub` 关键字，只是让其祖辈模组中的代码可以引用到他，而不是访问其内层代码。由于模组是些容器，因此仅将模组构造为公开，是做不了什么的；这就需要更进一步，而选择将模组里的一个或更多的程序项目，也构造为公开。

清单 7-6 中的错误说到，那个 `add_to_waitlist` 函数是私有的。适用于结构体、枚举、函数即方法等的隐私规则，与适用于模组的一样。

下面就来通过把 `pub` 关键字，添加在 `add_to_waitlist` 函数定义之前，而将其构造为公开函数，如下清单 7-7 所示。

文件名：`src/lib.rs`

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}
```

*清单 7-7：把 `pub` 关键字添加到 `mod hosting` 与 `fn add_to_waitlist`，实现从 `eat_at_restaurant` 调用该函数*

现在该代码就会编译了！接下来看看其中绝对与相对路径，以弄明白为何添加 `pub` 关键字，就实现了在遵循隐私规则之下，使用到 `add_to_waitlist` 中的这些路径的原因。

在那个绝对路径中，是以这里代码箱模组树的根、字面值 `crate` 开始的。那个 `front_of_house` 模组，即为被定义在该代码箱根中。尽管 `front_of_house` 模组不是公开的，但由于 `eat_at_restaurant` 函数被定义在与 `front_of_house` 同一模组中（即 `eat_at_restaurant` 与 `front_of_house` 是姊妹关系），因此是可以从 `eat_at_restaurant` 函数引用 `front_of_house` 的。接下来就是那个被标记了 `pub` 的 `hosting` 模组了。由于这里可以访问 `hosting` 的父模组，因此就可以访问 `hosting`。最后，由于那个 `add_to_waitlist` 函数被 `pub` 标记过，且这里可以访问他的父模组，因此该函数调用就生效了！

在那个相对路径中，除了第一步外，其中的逻辑与绝对路径相同：与从代码箱根开始不同，该相对路径是从 `front_of_house` 处开始的。这个 `front_of_house` 模组，是定义在与 `eat_at_restaurant` 函数同样的模组中，那么从 `eat_at_restaurant` 定义所在处的这个模组开始的相对路径，就是有效的。随后由于 `hosting` 与 `add_to_waitlist` 都是以 `pub` 关键字标记过，那么该路径其余部分就都工作了，同时此函数调用就是有效的了！

在计划分享库代码箱，进而其他项目可使用到其代码时，公开 API 即是与该代码箱用户的合约，定下了与库代码箱代码互动的方式。在管理对公共 API 的修改方面，则有着诸多考量，以让人们更易于依赖到咱们的代码箱。这些考量超出了本书的范围；若对这方面感兴趣，那么请参阅 [Rust API 指南](https://rust-lang.github.io/api-guidelines/)。


> **带有一个二进制与一个库的 Rust 代码包最佳实践（Best Practice for Packages with a Binary and a Library）**
>
> 前面提到过 Rust 包可以同时包含一个 `src/main.rs` 二进制代码箱根，与一个 `src/lib.rs` 库代码箱根，且这两个代码箱都将默认有着该 Rust 包的名字。一般来说，这种同时包含了一个库及二进制代码箱模式下的包，都会在二进制代码箱中，仅有着足够启动一个会调用到库代码箱代码的可执行程序的少量代码。由于库代码箱的代码可被共享，因此这就实现了别的项目，受益于该 Rust 包所提供的绝大部分功能。
>
> 模组树应定义在 `src/lib.rs` 中。随后，全部的公开程序项目，都可通过以该包名字开头的路径，在那个二进制代码箱中被使用。这个二进制代码箱，就像是个将用到那个库代码箱的完整外部箱，成了库代码箱的一名用户：他只能使用公共 API。这样做有助于设计出良好的 API；你不仅是库代码箱的作者，还是一名库代码箱的客户了！
>
> 在 [第 12 章](Ch12_An_I_O_Project_Building_a_Command_Line_Program.md)，将以一个会同时包含二进制代码箱与库代码箱的命令行程序，对这种代码组织方式实践加以演示。


### 使用 `super` 关键字开始相对路径

**Starting Relative Paths with `super`**

通过在路径开头使用 `super` 关键字，就可以构建出在父模组处，而非当前模组或代码箱根处开始的相对路径。这与以 `..` 语法开始的文件系统路径相似。使用 `super` 实现了对已知在父模组中某个程序项目的引用，在模组与其父模组密切相关，但该父模组在某个时候可能会被迁移到模组树中别的地方时，这种使用 `super` 关键字的相对路径，就能让模组树的重新安排更为容易。

设想下面清单 7-8 中，建模了一位大厨修正某个不正确点餐，并亲自将其交给顾客的代码。其中定义在 `back_of_house` 模组中的函数 `fix_incorrect_order`，通过以 `super` 作为开头指明的 `deliver_order` 路径，调用了定义在父模组中的该 `deliver_order` 函数：

文件名：`src/lib.rs`

```rust
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
```

*清单 7-8：使用以 `super` 开头的相对路径调用某个函数*


这个 `fix_incorrect_order` 函数是在 `back_of_house` 模组中，因此就可以使用 `super` 关键字，前往到 `back_of_house` 的父模组，那就是此示例中的 `crate`，亦即代码箱根。在那里，就会查找 `deliver_order` 进而找到他。大功告成！这里把 `back_of_house` 模组与 `deliver_order` 函数，设想作可能维持这同样关系，并在今后决定要对这个代码箱的模组树，进行重新组织时，他们会一起被移动。因此，这里使用了 `super`，从而今后在此代码被移入到别的模组时，要更新代码的地方就会少一些。

### 将结构体与枚举构造为公共项目

**Making Structs and Enums Public**

这里还可以使用 `pub` 关键字，来将结构体与枚举，指定为公开项目，但结构体与枚举下 `pub` 的用法，有着几个额外情况。在结构体定义前使用 `pub` 关键字时，就将该结构体构造为了公开，但该结构体的那些字段，仍将是私有。可根据具体情况，把各个字段构造为公开或不公开。在下面清单 7-9 中，就定义了有着公开 `toast` 字段，和私有 `seasonal_fruit` 字段的一个公开的 `back_of_house::Breakfast` 结构体。这就对在某个饭馆中，顾客可在何处挑选与正餐搭配的面包类型，而主厨则会根据当季及仓库里有些什么，而决定由哪些水果来搭配正餐，这种情形进行了建模。可用的水果变化很快，因此顾客就无法对水果进行选择，甚至他们看不到会得到什么样的水果。

文件名：`src/lib.rs`

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // 点下一份带有黑麦土司的夏日早餐, rye, US /raɪ/, UK /rai/, n.黑麦, 黑麦粒
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println! ("请给我一份 {} 土司", meal.toast);

    // 若不把接下来的行注释掉，那么就不会编译；这里不允许查看或修改
    // 餐食搭配的应季水果
    // meal.seasonal_fruit = String::from("blueberries");
}
```

*清单 7-9：有着一些公共字段与私有字段的一个结构体*

由于 `back_of_house::Breakfast` 结构体中的 `toast` 字段是公开的，因此在 `eat_at_restaurant` 中就可以使用点符号（`.`），对 `toast` 字段进行写入与读取。请注意由于 `seasonal_fruit` 是私有的，因此这里不能在 `eat_at_restaurant` 中使用那个 `seasonal_fruit` 字段。尝试将那个对 `seasonal_fruit` 字段值进行修改的行解除注释，看看将得到什么样的错误！


```console
$ cargo build
   Compiling restaurant v0.1.0 (/home/peng/rust-lang/restaurant)
error[E0616]: field `seasonal_fruit` of struct `Breakfast` is private
  --> src/lib.rs:25:10
   |
25 |     meal.seasonal_fruit = String::from("blueberries");
   |          ^^^^^^^^^^^^^^ private field

For more information about this error, try `rustc --explain E0616`.
error: could not compile `restaurant` due to previous error
```

还请留意由于 `back_of_restaurant::Breakfast` 有个私有字段，那么该结构体就需要提供一个公开的、构造出`Breakfast` 实例的关联函数（这里将该函数命名为了 `summer`）。若 `Breakfast` 没有这样一个函数，那么由于在 `eat_at_restaurant` 中无法设置那个私有 `seasonal_fruit` 字段的值，因此就没法在 `eat_at_restaurant` 中创建处一个 `Breakfast` 的实例来。

与此相比，在将枚举构造为公开时，该枚举的全部变种此时都是公开的。这里就只需在 `enum` 关键字前的 `pub` 关键字，如下清单 7-10 中所示。

文件名：`src/lib.rs`

```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```

> appetizer, US/ˈæpəˌtaɪzər/, UK/ˈæpəˌtaɪzə(r)/ n.（餐前的）开胃品

*清单 7-10：将枚举指定为公开，则会将其全部变种构造为公开*

由于这里将那个 `Appetizer` 枚举构造为了公开，因此就可以在 `eat_at_restaurant` 中使用 `Soup` 与 `Salad` 变种。除非枚举的各个变种是公开的，否则枚举就不是非常有用了；若在所有场合，都必须以 `pub` 关键字来对全部枚举变种进行注解，那就会让人觉得烦恼不已，因此默认枚举变种就是公开的。而结构体则通常无需其字段保持公开就有用处，因此结构体的那些字段，就遵循了除非以 `pub` 关键字注释，而默认全部为私有的一般规则。

还有一个尚未讲到的涉及 `pub` 关键字的情况，那也是最后的一项模组系统特性：`use` 关键字。后面会先讲 `use` 本身，然后再给出怎样结合 `pub` 与 `use`。

## 使用 `use` 关键字将路径带入作用域

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

### 创建惯用 `use` 路径

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

### 使用 `as` 关键字提供新名字

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


### 使用 `pub use` 将名字重新导出

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

在所编写代码的内部结构，与调用代码的程序员们对该领域有着不同设想时，重导出就是有用的。比如，在这个饭馆的比喻中，运营该饭馆的人设想的是“前厅”与“后厨”。但造访饭馆的食客，或许不会用这样的词汇，来认识饭馆的这些部位。有了 `pub use`，就可以一种结构来编写代码，而以另一种结构将代码暴露出来。这样做就让这个库，对于在该库上编写代码的程序员，与调用这个库的程序员，均具备良好的组织。在第 14 章的 [“运用 `pub use` 导出便利的公共 API”](Ch14_More_about_Cargo_and_Crates_io.md#exporting-a-convenient-public-api-with-pub-use) 小节，将会看到另一个 `pub use` 的示例，并了解他是怎样影响到代码箱的文档。


### 使用外部 Rust 包

在第 2 章中，那里曾编写了用到名为 `rand` 外部包来获取一个随机数的猜数游戏项目。为了在项目中使用 `rand`，那里曾添加下面这行到 `Cargo.toml` 文件：

文件名：`Cargo.toml`

```toml
rand = `0.8.3`
```

将 `rand` 作为依赖项添加到 `Cargo.toml`，就告诉 Cargo，去 [crates.io](https://crates.io/) 下载那个 `rand` 包和任何的依赖项，而令到 `rand` 对此项目可用。

随后为了将 `rand` 的一些定义，带入到所编写的包中，这里添加了以代码箱名字，`rand`，开头，并列出了打算要带入到作用域中的那些条目的一个 `use` 行。回顾第 2 章中的 [“生成一个随机数”](Ch02_Programming_a_Guessing_Game.md#generating-a-random-number) 小节，那里就将那个 `Rng` 特质，带入到了作用域，并调用了 `rand::thread_rng` 函数：

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

### 运用嵌套路径来清理大量的 `use` 清单

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


### 全局操作符

在打算将某个路径中的 *全部（all）* 公开条目，都带入到作用域时，可将那个路径，后面跟上 `*`，即全局操作符，而予以指定：

```rust
use std::collections::*;
```

这个 `use` 语句，将定义在 `std::collections` 中的全部公开项目，都带入到了当前作用域。在使用这个全局操作符时要当心！全局带入，会导致更难于分清哪些名字是作用域中，与在所编写程序中用到的名字，是在何处定义的。

通常是在测试时，要将正在测试的全部程序项目带入到 `tests` 模组，才使用这个全局操作符；在第 11 章中的 [怎样编写测试](Ch11_Writing_Automated_Tests.md#how-to-write-tests) 小节，就会讲到这个问题。在序曲模式（the prelude pattern）中，有时也会用到全局操作符：请参阅 [标准库文档](https://doc.rust-lang.org/std/prelude/index.html#other-preludes)，了解有关更多序曲模式的知识。


## 将模组拆分为不同文件

**Separating Modules into Different Files**

到目前为止，本章的全部示例，都是将多个模组定义在一个文件中的。在模组变得大起来时，就会打算将他们的定义，迁移到单独文件，从而令到代码易于导览。

比如，这里就从清单 7-17 中的代码开始，并将那些模组提取到文件中，而非将所有那些模组，都定义在那个代码箱根文件里。在此情况下，代码箱根文件为 `src/lib.rs`，但这个过程同样对那些根文件为 `src/main.rs` 的二进制代码箱有效。

首先，会将那个 `front_of_house` 模组，提取到他自己的文件。要移除 `front_of_house` 模组花括号里头的代码，而仅留下 `mod front_of_house;` 语句声明，这样那个 `src/lib.rs` 就会包含如下清单 7-21 中展示的代码了。请注意在创建出后面清单 7-22 中的 `src/front_of_house.rs` 文件之前，这是不会编译的。

文件名：`src/lib.rs`

```rust
mod front_of_house; 

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

*清单 7-21：声明出其模组代码体将在 `src/front_of_house.rs` 中的 `front_of_house` 模组*

接下来，就要把原先在花括号中的代码，放入到一个新的名为 `src/front_of_house.rs` 文件中，如下清单 7-22 中所示。由于编译器在该代码箱根中，找到了名字 `front_of_house`，因此他就明白要在这个文件中看看。

文件名：`src/front_of_house.rs`

```rust
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```

*清单 7-22：文件 `src/front_of_house.rs` 中 `front_of_house` 模组内部的定义*

请注意只需在模组树中的某处，使用一次 `mod` 声明，而将某个文件的内容加载进来。一旦编译器获悉该文件是项目的一部分（且由已将那个 `mod` 语句放置于于何处，而掌握了该代码在模组树中所处的位置），项目中的其他文件，则应如同之前 [用于引用模组树中项目的路径](#paths-for-referring-to-an-item-in-the-module-tree) 小节中，曾讲到的到模组声明处的路径，来引用那个文件中的代码。也就是说，这里的 `mod` *并非* 其他编程语言有的那种 “include” 操作。

接下来，就要将那个 `hosting` 模组，提取到他自己的文件了。而由于 `hosting` 是 `front_of_house` ，而非根模组，的子模组，因此过程略有不同。这里将把 `hosting` 模组的文件，放在模组树中以其父辈命名的一个新目录中，此示例中即为 `src/front_of_house`。

这里要将 `src/front_of_house.rs` 文件，修改为只包含 `hosting` 模组声明，以开始对 `hosting` 的迁移：

文件名：`src/front_of_house.rs`

```rust
pub mod hosting;
```

随后就要创建一个 `src/front_of_house` 的目录，和一个文件 `src/front_of_house/hosting.rs`，来包含在 `hosting` 模组中构造的那些定义：

文件名：`src/front_of_house/hosting.rs`

```rust
pub fn add_to_waitlist() {}
```

相反如果将 `hosting.rs` 放在 `src` 目录，那么编译器就会以为 `hosting.rs` 的代码，是在声明于代码箱根部的 `hosting` 模组中的，而不是那个 `front_of_house` 模组的子模组中的。为了获取模组代码，而要查看那些文件方面的编译器规则，就表明这些目录与文件，甚为紧密地于模组树结构相匹配。

####  备用文件路径

>
> 本小节讲的是 Rust 编译器所用到的最惯用的文件路径；但较早的文件路径仍被支持。
>
> 对于定义在代码箱根部的名为 `front_of_house` 模组，编译器会在下面这些地方查找该模组的代码：

- `src/front_of_house.rs` （即这里讲到的）；
- `src/front_of_house/mod.rs` （较早的，仍被支持的路径）。

> 而对于作为 `front_of_house` 的子模组的名为 `hosting` 的模组，编译器会在以下地方查找该模组的代码：

- `src/front_of_house/hosting.rs` （即这里讲到的）；
- `src/front_of_house/hosting/mod.rs` （较早的，仍被支持的路径）。

> 对于同一模组，若同时使用这两种文件路径，那么就会得到一个编译器错误。而对同一项目中的不同模组，采用不同方式的文件路径是被允许的，只是会对那些导览项目的人造成困扰。
>
> 使用名为 `mod.rs` 文件方式的主要缺点，即那样的话，项目最终会以许多名为 `mod.rs` 文件而终结，在代码编辑器中，同时打开这些 `mod.rs` 文件，那么就会感到混乱。

将各个模组的代码，移入到单独文件现在就完成了，而模组树还是保持原来那样。尽管模组定义存在于不同文件中，但是无需任何修改，那个在 `eat_at_restaurant` 中的函数调用仍会工作。这种技巧，就实现了在模组大小增长时，将其迁移到新的文件中。

请注意 `src/lib.rs` 中的那个 `pub use crate::front_of_house::hosting;` 语句，同样不曾改变，而那个 `use` 也不会对哪些文件作为代码箱的部分，而被编译有任何的影响。`mod` 关键字定义了模组，而 Rust 则会在与该模组有着同样名字的文件中，查找要进到那个模组中的代码。

## 总结

Rust 实现了包拆分为多个代码箱，进而将代码箱拆分为多个模组，这样就可以从一个模组，对定义在另一模组中的程序项目加以引用。通过指明绝对或相对路径，就可以做到这点。使用 `use` 语句，就可以将这些程序项目的路径，带入到作用域，如此就可以在那个作用域中，多次用到所带入的程序项目时，使用较简短的路径。默认下模组代码是私有的，但可通过添加 `pub` 关键字，而将一些定义构造为公开的。

下一章中，就会看看，可在本地组织良好代码中，使用到的标准库中的一些集合数据结构（collection data structures）。
