# 引用模组树中项目的路径

为了展示给 Rust 于何处找到模组树中的项目，我们会使用路径，以我们在导览文件系统时我们使用路径的同一方式。为了调用函数，我们需要知道他的路径。

路径可以采用两种形式：

- *绝对路径，an absolute path*，是自代码箱根开始的完整路径；对于外部代码箱中的代码，绝对路径以代码箱名字开头，而对于当前代码箱中的代码，绝对路径以字面值 `crate` 开头；
- *相对路径，a relative path*，从当前模组开始，并使用 `self`、`super`，或当前模组中的标识符。

绝对路径和相对路径都后跟一个或多个以双冒号（`::`）分隔的标识符。

回到 [清单 7-1](./defining_modules.md#listing_7-1)，假设我们打算调用 `add_to_waitlist` 函数。这相当于询问：`add_to_waitlist` 函数的路径是什么？下面清单 7-3 包含清单 7-1，其中移除了部分模组及函数。

我们将展示从一个定义在代码箱根处的新函数 `eat_at_restaurant`，调用 `add_to_waitlist` 函数的两种方法。这两个路径都是正确的，但还存在另一个问题，将阻止这个示例按原样编译。稍后我们将解释原因。

`eat_at_restaurant` 函数属于咱们库代码箱公开 API 的一部分，因此我们以 `pub` 关键字标记他。在 [以 `pub` 关键字暴露路径](#以-pub-关键字暴露路径) 小节中，我们将深入有关 `pub` 的细节。


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

<a name="listing_7-3"></a>
**清单 7-3**：使用绝对路径与相对路径调用 `add_to_waitlist` 函数

第一次在 `eat_at_restaurant` 中调用 `add_to_waitlist` 函数时，我们使用了绝对路径。`add_to_waitlist` 函数定义在与 `eat_at_restaurant` 相同的代码箱下，这意味着我们可使用 `crate` 关键字开始一个绝对路径。然后，我们包含各个后续模组，直到到达 `add_to_waitlist`。咱们可以想象一个有着相同结构的文件系统：我们会指定路径 `/front_of_house/hosting/add_to_waitlist` 来运行 `add_to_waitlist` 程序；使用名字 `crate` 从代码箱根开始，就像在咱们的 shell 下使用 `/` 从文件系统根目录开始一样。

第二次在 `eat_at_restaurant` 中调用 `add_to_waitlist` 时，我们使用相对路径。这一路径以 `front_of_house` 开头，该模组的名字定义在模组树中与 `eat_at_restaurant` 的同一层级。这里的文件系统等效项，将是使用路径 `front_of_house/hosting/add_to_waitlist`。以模组名字开头意味着路径是相对的。

选择使用相对路径还是绝对路径，是咱们根据咱们的项目作出的决定，并取决于咱们是否更愿意将项目定义代码从使用项目的代码单独迁出，还是将二者放在一起。例如，若我们把 `front_of_house` 模组和 `eat_at_restaurant` 函数，迁移到名为 `customer_experience` 的模组中，那么我们就需要更新 `add_to_waitlist` 的绝对路径，但相对路径仍然有效。但是，若我们将 `eat_at_restaurant` 函数单独移到名为 `dining` 的模组中，那么 `add_to_waitlist` 调用的绝对路径将保持不变，但相对路径将需要更新。一般来说，我们更倾向于指定绝对路径，因为我们将打算彼此独立地迁移代码定义及项目调用的可能性更大。

我们来尝试编译清单 7-3，并找出他为何还不编译！我们得到的错误如下清单 7-4 中所示。


```console
$ cargo build
   Compiling restuarant v0.1.0 (/home/hector/rust-lang-zh_CN/projects/restuarant)
error[E0603]: module `hosting` is private
 --> src/lib.rs:9:28
  |
9 |     crate::front_of_house::hosting::add_to_waitlist();
  |                            ^^^^^^^  --------------- function `add_to_waitlist` is not publicly re-exported
  |                            |
  |                            private module
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
   |                     ^^^^^^^  --------------- function `add_to_waitlist` is not publicly re-exported
   |                     |
   |                     private module
   |
note: the module `hosting` is defined here
  --> src/lib.rs:2:5
   |
 2 |     mod hosting {
   |     ^^^^^^^^^^^

For more information about this error, try `rustc --explain E0603`.
error: could not compile `restuarant` (lib) due to 2 previous errors
```

<a name="listing_7-4"></a>
**清单 7-4**：构建清单 7-3 中代码时的编译器报错

错误消息表明模组 `hosting` 是私有的。换句话说，我们有 `hosting` 模组及 `add_to_waitlist` 函数的正确路径，但 Rust 将不允许我们使用他们，因为他没有对私有部分的权限。在 Rust 中，默认情况下所有项目（函数、方法、结构体、枚举、模组和常量等）都属于父模组私有。当咱们打算将函数或结构体等项目构造为私有时，咱们就将其放入模组中。

父模组中的项目不能使用子模组中的私有项目，但子模组中的项目可以使用其祖辈模组中的项目。这是因为子模组封装并隐藏了他们的实现细节，但子模组可以看到定义他们的上下文。继续我们的比喻，请把隐私规则想象成餐厅的后台：那里发生的事情对餐厅顾客来说属于私有，但办公室经理可以看到并执行他们运营餐厅里的一切。

Rust 选择让模组系统以这种方式运作，从而隐藏内部实现细节成为默认行为。这样，咱们就清楚咱们可以修改内部代码的哪些部分，而不会破坏外部代码。不过，Rust 确实为咱们提供了通过使用 `pub` 关键字构造项目为公开，以暴露子模组代码的内部部分给外部祖辈模组的选项。


##  以 `pub` 关键字暴露路径

我们来回到清单 7-4 中的报错，他告诉我们 `hosting` 模组是私有的。我们希望父模组中的 `eat_at_restaurant` 函数有着对子模组中 `add_too_waitlist` 函数的访问权限，因此我们以 `pub` 关键字标记 `hosting` 模组，如下清单 7-5 中所示。


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

<a name="listing_7-5"></a>
**清单 7-5**：声明 `hosting` 模组为 `pub`，以在 `eat_at_restaurant` 中使用他


不幸的是，清单 7-5 中的代码仍会导致编译器错误，如下清单 7-6 中所示。


```console
$ cargo build
   Compiling restuarant v0.1.0 (/home/hector/rust-lang-zh_CN/projects/restuarant)
error[E0603]: module `hosting` is private
 --> src/lib.rs:9:28
  |
9 |     crate::front_of_house::hosting::add_to_waitlist();
  |                            ^^^^^^^  --------------- function `add_to_waitlist` is not publicly re-exported
  |                            |
  |                            private module
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
   |                     ^^^^^^^  --------------- function `add_to_waitlist` is not publicly re-exported
   |                     |
   |                     private module
   |
note: the module `hosting` is defined here
  --> src/lib.rs:2:5
   |
 2 |     mod hosting {
   |     ^^^^^^^^^^^

For more information about this error, try `rustc --explain E0603`.
error: could not compile `restuarant` (lib) due to 2 previous errors
```

*清单 7-6：构建清单 7-5 中代码时的编译器报错*

发生了什么事？在 `mod hosting` 前添加 `pub` 关键字会构造该模组为公开。在这一修改下，当我们可以访问 `front_of_house` 时，我们也可以访问 `hosting`。但 `hosting` 的 *内容* 仍然是私有的；构造模组为公开不会构造其内容为公开。模组上的 `pub` 关键字只会让其祖辈模组中的代码可以引用他，而不是访问其内部代码。因为模组属于容器，所以仅构造模组为公开并不能做得更多；我们需要更进一步，选择构造模组内的一个或多个项目为公开。

清单 7-6 中的报错表明 `add_to_waitlist` 函数是私有的。隐私规则适用于结构体、枚举、函数和方法以及模组等。

我们还可通过在 `add_too_waitlist` 函数的定义前添加 `pub` 关键字，构造该函数为公开，如下清单 7-7 中所示。

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

<a name="listing_7-7"></a>
**清单 7-7**：添加 `pub` 关键字到 `mod hosting` 及 `fn add_too_waitlist`，让我们可以在 `eat_at_restaurant` 中调用这个函数

现在这段代码将编译！为了了解为何添加 `pub` 关键字，就让我们可以在 `add_too_waitlist` 中，在遵守隐私规则下使用这些路径，我们来看一下绝对路径和相对路径。

在绝对路径中，我们以 `crate` 开头，他是咱们代码箱的模组树的根。`front_of_house` 模组定义在代码箱根处。虽然 `front_of_house` 不是公开的，但由于 `eat_at_restaurant` 函数定义在与 `front_of_house` 模组的同一个模组中（即 `eat_at_restaurant` 和 `front_of_house` 属于同辈），我们可在 `eat_at_restaurant` 中引用 `front_of_house`。接下来是以 `pub` 标注的 `hosting` 模组。我们可以访问 `hosting` 的父模组，因此我们可以访问 `hosting`。最后，`add_to_waitlist` 函数以 `pub` 标注了，并且我们可以访问他的父模组，因此这个函数调用有效！

在相对路径中，除了第一步外，逻辑与绝对路径相同的：路径未从代码箱根开始，而是从 `front_of_house` 开始。`front_of_house` 模组定义在与 `eat_at_restaurant` 的同一个模组中，因此从 `eat_at_restaurant` 定义处的模组开始的相对路径是可行的。然后，因为 `hosting` 和 `add_to_waitlist` 都以 `pub` 标注过，因此该路径的其余部分也有效，进而这个函数调用有效！

当咱们计划共享咱们的库代码箱，以便其他项目可以使用咱们的代码时，我们的公共 API 就是咱们与咱们代码箱的用户的合约，决定了他们如何与咱们的代码交互。为了让人们更容易依赖咱们的代码箱，管理咱们 API 的变更方面需要考虑很多因素。这些考量超出了这本书的范围；若咱们对这个主题感兴趣，请参阅 [Rust API 指南](https://rust-lang.github.io/api-guidelines/)。


> **带有二进制与库的包的最佳实践**
>
> 我们曾提到一个包可同时包含 `src/main.rs` 的二进制代码箱根及 `src/lib.rs` 的库代码箱根，且默认情况下两个代码箱都将有着包的名字。通常，有着这种同时包含库与二进制代码箱模式的包，在二进制代码箱中都只有启动可执行程序的代码，该可执行程序调用定义在库代码箱中的代码。这让其他项目可受益于这个包所提供的大部分功能，因为库代码箱的代码可以共享。
>
> 模组树应定义在 src/lib.rs 中。然后，通过以包的名字开始的路径，任何公开项目都可在二进制代码箱中使用。二进制代码箱成为库代码箱的用户，就像完全外部的代码箱将使用库代码箱一样：他只能使用公开 API。这可以帮助咱们设计良好的 API；咱们不仅是作者，同时还是客户！
>
> 在 [第 12 章](../Ch12_An_IO_Project_Building_a_Command_Line_Program.md) 中，我们将以一个同时包含二进制代码箱与库代码箱的命令行程序演示这种组织实践。


## 以 `super` 关键字开始相对路径

我们可通过在路径开头使用 `super` 关键字，构造以父模组处为起点的相对路径，而不是以当前模组或代码箱根为起点。这就像以 `..` 语法开始文件系统路径，表示要前往到父目录。使用 `super` 允许我们引用我们已知位于父模组中的项目，当模组与父模组密切相关，而父模组可能会在某一天被迁移到模组树的其他地方时，这可以使重新排列模组树变得更容易。

请看下面清单 7-8 中的代码，这段代码建模了厨师修正错误菜单，并亲自将其送到顾客手中的情况。定义在 `back_of_house` 模组中的 `fix_incorrect_order` 函数，通过以 `super` 开头指定 `deliver_order` 路径，调用了定义在父模组中的 `deliver_order` 函数。


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

<a name="listing_7-8"></a>
**清单 7-8**：使用以 `super` 开头的相对路径调用函数*

`fix_incorrect_order` 函数位于 `back_of_house` 模组中，因此我们可使用 `super` 前往 `back_of_house` 的父模组，在本例中便是根模组 `crate`。从那里，我们查找 `deliver_order` 并找到了他。成功！我们认为，即是将来决定重新组织该代码箱的模组树，`back_of_house` 模组和 `deliver_order` 函数也会保持同一相互关系，并会一起迁移。因此，我们使用了 `super`，这样当这段代码被迁移到别的模组时，我们今后要更新代码的位置将更少。


## 构造结构体与枚举为公开

我们还可以使用 `pub` 关键字指定结构体和枚举为公开，但在结构体和枚举下的 `pub` 用法有一些额外细节。当我们在结构体定义前使用 `pub` 时，我们构造结构体为公开，但结构体的字段将仍是私有的。我们可根据具体情况构造每个字段为公开或不公开。在下面清单 7-9 中，我们定义了个公开的 `back_of_house::Breakfast` 结构体，有着一个公开的 `toast` 字段和一个私有的 `seasonal_fruit` 字段。这建模了餐厅中的情形，其中顾客可以选择餐食中搭配的面包类型，而厨师根据时令和库存决定搭配餐食的水果。可用的水果变化很快，因此顾客无法选择水果，甚至无法看到他们将得到哪种水果。

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
    // 点一份带黑麦土司的夏日早餐, rye, US /raɪ/, UK /rai/, n.黑麦, 黑麦粒。
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // 改变我们对想要什么面包的想法。
    meal.toast = String::from("Wheat");
    println! ("请给我一份 {} 土司", meal.toast);

    // 若我们取消注释下一行，其将不编译；
    // 我们不允许查看或修改搭配餐食的应季水果。
    // meal.seasonal_fruit = String::from("blueberries");
}
```

<a name="listing_7-9"></a>
**清单 7-9**：带有一些公开字段及一些私有字段的结构体

因为 `back_of_house::Breakfast` 结构体中的 `toast` 字段是公开的，因此在 `eat_at_restaurant` 中我们可以使用点表示法（`.`）写入和读取 `toast` 字段。请注意，我们不能在 `eat_at_restaurant` 中使用 `seasonal_fruit` 字段，因为 `seasonal_fruit` 是私有的。请尝试取消注释修改 `seasonal_fruit` 字段值的行，看看咱们得到什么错误！


```console
$ cargo build
   Compiling restaurant v0.1.0 (/home/hector/rust-lang-zh_CN/projects/restaurant)
error[E0616]: field `seasonal_fruit` of struct `Breakfast` is private
  --> src/lib.rs:32:10
   |
32 |     meal.seasonal_fruit = String::from("blueberries");
   |          ^^^^^^^^^^^^^^ private field

For more information about this error, try `rustc --explain E0616`.
error: could not compile `restaurant` (lib) due to 1 previous error
```

另外，请注意，因为 `back_of_house::Breakfast` 有个私有字段，所以该结构体需要提供一个公开的关联函数构造 `Breakfast` 的实例（我们在这里将其命名为 `summer`）。当 `Breakfast` 没有这样一个函数时，我们就无法在 `eat_at_restaurant` 中创建 `Breakfast` 的实例，因为我们无法在 `eat_at_restaurant` 中设置私有的 `seasonal_fruit` 字段的值。

相反，当我们构造枚举为公开时，那么随后他的所有变种都是公开的。我们只需要在 `enum` 关键字前加上 `pub`，如下清单 7-10 中所示。

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

<a name="listing_7-10"></a>
**清单 7-10**：指定枚举为公开会使其所有变种成为公开

因为我们已构造 `Appetizer` 为公开，所以我们可以在 `eat_at_restaurant` 中使用 `Soup` 和 `Salad` 两个变种。

除非枚举的变种是公开的，否则枚举的用处就不大；在每种情况下都必须以 `pub` 注解所有枚举变种将很烦人了，因此枚举变种的默认设置是要成为公开。结构体在其字段不公开的情况下通常就很有用，因此结构体字段遵循默认情况下所有内容都是私有，除非以 `pub` 注释的一般规则。

还有一种涉及 `pub` 的情形我们尚未介绍，那就时我们的最后一项模组系统特性：`use` 关键字。我们将首先介绍 `use` 本身，然后我们将展示如何结合 `pub` 和 `use`。


（End）


