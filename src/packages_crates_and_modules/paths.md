#  用于引用模组树中项目的路径

**Paths for Referring to an Item in the Module Tree**


为告诉 Rust 在模组树中的何处，可以找到某个项目，我们就像在文件系统中一样，用到了路径。要调用某个函数，我们需要知道他的路径。

路径有两种形式：

- *绝对路径，an absolute path*，是从代码箱根开始的完整路径；对于外部代码箱中的代码，绝对路径从代码箱名字开始，而对于当前代码箱中的代码，绝对路径从字面的 `crate` 开始；

- *相对路径，a relative path* 从当前模组开始，并用到 `self`、`super` 关键字，或当前模组中的某个标识符。


绝对路径和相对路径，后面都有一或多个用双冒号（`::`）分隔的标识符。

回到清单 7-1，假设我们要调用 `add_too_waitlist` 函数。这等同于在询问：`add_to_waitlist` 函数的路径是什么？下面清单 7-3 包含了去掉了部分模组及函数的清单 7-1。

我们将展示两种从代码箱根处，定义的新函数 `eat_at_restaurant`，调用 `add_too_waitlist` 函数的方法。这两个路径都是正确的，但还存在另一将导致本示例无法按原样编译的问题。我们稍后会解释原因。

`eat_at_restaurant` 函数，是咱们库代码箱公共 API 的一部分，因此我们使用了 `pub` 关键字对其进行标记。在 [使用 pub 关键字暴露路径](#使用-pub-关键字暴露路径) 小节，我们将详细介绍 `pub`。


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

*清单 7-3：使用绝对与相对路径，调用 `add_to_waitlist` 函数*


第一次调用 `eat_at_restaurant` 中的 `add_to_waitlist` 函数时，我们使用的是绝对路径。`add_too_waitlist` 函数与 `eat_at_restaurant`，定义在同一个代码箱中，这意味着我们可以使用 `crate` 关键字，来开始绝对路径。然后，我们逐个包含后续模组，直到找到 `add_to_waitlist`。咱们可以想象某种具有相同结构的文件系统：我们指定 `/front_of_house/hosting/add_to_waitlist` 路径，来运行 `add_to_waitlist` 程序；使用 `crate` 这个名字，从代码箱根目录开始，就像在 shell 中，使用 `/` 从文件系统根目录开始一样。

第二次在 `eat_at_restaurant` 中调用 `add_too_waitlist` 时，我们使用了相对路径。该路径以 `front_of_house` 开头，`front_of_house` 是与 `eat_at_restaurant` 定义在模组树同一级别处，模组的名字。在这里，文件系统等价的做法，是使用路径 `front_of_house/hosting/add_to_waitlist`。以模组名字开头，就意味着路径是相对的。

选择使用相对路径还是绝对路径，取决于咱们的项目，也取决于咱们更倾向于将项目定义代码，与使用项目的代码分开移动，还是一起移动。例如，如果我们将 `front_of_house` 模组和 `eat_at_restaurant` 函数，移到名为 `customer_experience` 的模组中，我们就需要更新 `add_too_waitlist` 的绝对路径，但相对路径仍然有效。但是，如果我们将 `eat_at_restaurant` 函数单独移到名为 `dining` 的模组中，那么 `add_too_waitlist` 调用的绝对路径将保持不变，但相对路径则需要更新。一般来说，我们更倾向于指定绝对路径，因为我们更有可能希望，独立地移动项目的代码定义和项目的调用。

我们来试着编译清单 7-3，看看他为什么还不能编译！我们得到的错误信息，如清单 7-4 所示。


```console
$ cargo build
   Compiling restuarant v0.1.0 (/home/hector/restuarant)
error[E0603]: module `hosting` is private
 --> src/lib.rs:8:28
  |
8 |     crate::front_of_house::hosting::add_to_waitlist();
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
  --> src/lib.rs:10:21
   |
10 |     front_of_house::hosting::add_to_waitlist();
   |                     ^^^^^^^  --------------- function `add_to_waitlist` is not publicly re-exported
   |                     |
   |                     private module
   |
note: the module `hosting` is defined here
  --> src/lib.rs:2:5
   |
2  |     mod hosting {
   |     ^^^^^^^^^^^

For more information about this error, try `rustc --explain E0603`.
error: could not compile `restuarant` (lib) due to 2 previous errors
```

*清单 7-4：构建清单 7-3 中代码时的编译器报错*


错误消息表明，模组 `hosting` 是私有的。换句话说，我们有了 `hosting` 模组和 `add_too_waitlist` 函数的正确路径，但 Rust 不允许我们使用他们，因为其无法访问私有部分。在 Rust 中，所有项目（函数、方法、结构体、枚举、模组和常量），默认都是父模组私有的。如果咱们打算将函数或结构体等项目私有化，可以将其放入模组中。

父模组中的项目，不能使用子模组中的私有项目，但子模组中的项目，却可以使用其祖辈模组中的项目。这是因为子模组封装并隐藏了他们的实现细节，但子模组可以看到定义他们的上下文。继续我们的比喻，请把隐私规则，想象成某家餐厅的后台办公室：里面发生的事情，对餐厅顾客来说是隐私，但办公室经理，却可以看到执行做他们所经营餐厅里的一切事情。

Rust 选择让模组系统以这种方式运行，以便在默认情况下，隐藏内部实现细节。这样，咱们就明白，在不破坏外部代码的情况下，可以修改内部代码的哪些部分。不过，Rust 确实提供了选项，让咱们可以通过使用 `pub` 关键字，将子模组的内部代码，公开给外部的祖辈模组。


##  使用 `pub` 关键字暴露路径

**Exposing Paths with the `pub` Keyword**

我们回到清单 7-4 中的报错，该报错告诉我们，`hosting` 模组是私有的。我们希望父模组中的 `eat_at_restaurant` 函数，能访问子模组中的 `add_too_waitlist` 函数，因此我们在 `hosting` 模组中，标记了 `pub` 关键字，如清单 7-5 所示。


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

*清单 7-5：将 `hosting` 模组声明为 `pub`，以便在 `eat_at_restaurant` 中使用他*


不幸的是，如下清单 7-6 所示，清单 7-5 中的代码仍会导致报错。


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

*清单 7-6：构建清单 7-5 中代码时出现的编译器报错*


发生了什么？在 `hosting` 模组前添加 `pub` 关键字后，该模组就变成了公共模组。有了这个改动，如果我们能访问 `front_of_house`，也就能访问 `hosting`。但是，`hosting` 的 *内容* 仍然是私有的；将该模组构造为公开，并不会使其内容公开。模组上的 `pub` 关键字，只能让其先辈模组中的代码引用他，而不能访问其内部代码。因为模组是个容器，所以只将模组公开并不能做什么；我们需要更进一步，选择将其模组中的一或多个项目也公开。

清单 7-6 中的报错表明，`add_too_waitlist` 函数是私有的。隐私规则适用于结构体、枚举、函数和方法以及模组。

我们还可以在 `add_too_waitlist` 函数的定义前，添加 `pub` 关键字，使其成为公共函数，如清单 7-7 所示。

<a name="list_7-7"></a>
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

*清单 7-7：在 `mod hosting` 和 `fn add_too_waitlist` 中添加 `pub` 关键字后，我们就可以在 `eat_at_restaurant` 中调用了这个函数*

现在代码可以编译了！要了解为何添加 `pub` 关键字后，我们就可以在 `add_too_waitlist` 中，在遵守隐私规则下使用这些路径，我们来看看其中的绝对路径和相对路径。

在绝对路径中，我们从 `crate` 开始，他是咱们代码箱模组树的根。`front_of_house` 模组就定义在代码箱根中。虽然 `front_of_house` 不是公有的，但由于 `eat_at_restaurant` 函数与 `front_of_house` 模组定义在同一个模组中（也就是说，`eat_at_restaurant` 和 `front_of_house` 属于姊妹关系），我们可以在 `eat_at_restaurant` 中引用 `front_of_house`。接下来是标有 `pub` 的 `hosting` 模组。我们可以访问 `hosting` 的父模组，因此可以访问 `hosting`。最后，`add_to_waitlist` 函数被标记为 `pub`，我们可以访问他的父模组，因此这个函数调用是有效的！

在相对路径中，除了第一步外，逻辑与绝对路径是相同的：路径不从代码箱根开始，而是从 `front_of_house` 开始。`front_of_house` 模组与 `eat_at_restaurant`，定义在同一个模组中，因此从定义出 `eat_at_restaurant` 的模组处开始的相对路径就是可行的。然后，由于 `hosting` 和 `add_to_waitlist` 被标记为了 `pub`，因此该路径的其余部分也有效，这个函数调用也就有效了！

若咱们计划分享出咱们的库代码箱，以便其他项目可以使用咱们的代码，那么我们的公共 API，就是咱们与代码箱用户之间的合约，他决定了用户如何与咱们的代码交互。而在管理公开 API 变更方面，为使对咱们代码箱有依赖的人们更容易一些，则需要考虑诸多因素。这些考量超出了本书的范围；如果你对这个主题感兴趣，请参阅 [Rust API 指南](https://rust-lang.github.io/api-guidelines/)。


> **带有一个二进制与一个库代码箱的 Rust 软件包最佳实践**
>
> **Best Practice for Packages with a Binary and a Library**
>
> 我们曾提到过，一个软件包可以同时包含一个 `src/main.rs` 的二进制代码箱根，和一个 `src/lib.rs` 库代码箱根，且默认情况下这两个代码箱都将有着这个软件包的名字。通常情况下，以这种同时包含一个库和一个二进制代码箱模式的软件包，都会在二进制代码箱中，加入启动一个会调用到库代码箱的可执行文件的足够少代码。这样，其他项目就能从这个软件包所提供的大部分功能中获益，因为库代码箱的代码可以共用。
>
> 模组树应定义在 src/lib.rs 中。然后，二进制代码箱并可通过以软件包名字开头的路径，使用任何公共项目。二进制代码箱就成为库代码箱的一个用户，就像会用到库代码箱的一个纯粹外部代码箱一样：他只能使用公开 API。这可以帮助咱们设计出良好的应用程序接口；咱们不仅是作者，同时也是客户！
>
> 在 [第 12 章](../Ch12_An_IO_Project_Building_a_Command_Line_Program.md)，我们将以同时包含一个二进制代码箱，与一个库代码箱的命令行程序，演示这种代码组织实践。


## 以 `super` 关键字开始相对路径

**Starting Relative Paths with `super`**

我们可以通过在路径开头，使用 `super` 关键字构建从父模组，而不是当前模组或代码箱根开始的相对路径。这就像用 `..` 语法，开始文件系统路径一样。当该模组与父模组关系密切，使用 `super` 就能让我们引用父模组中的项目，而父模组有可能在某一天，被移到模组树的其他地方时，使重新排列模组树变得更容易。


请看以下清单 7-8 中的代码，该代码建模了厨师修改错误订单，并亲自将其送到顾客手中的情况。`back_of_house` 模组中定义的 `fix_incorrect_order` 函数，通过指定从 `super` 开始的 `deliver_order` 路径，调用了父模组中定义的 `deliver_order` 函数。


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

`fix_incorrect_order` 函数位于 `back_of_house` 模组中，因此我们可以使用 `super` 进入 `back_of_house` 的父模组，在本例中就是根模组 `crate`。在那里，我们查找 `deliver_order`，并找到了他。成功！我们认为，如果我们决定重新组织该代码箱的模组树，`back_of_house` 模组和 `deliver_order` 函数基本会保持相同的关系，而会在重新组织时被一起移动。因此，我们使用了 `super`，这样将来如果这些代码被移动到不同模组，我们就可以减少更新代码的地方。


## 将结构体与枚举构造为公开

**Making Structs and Enums Public**


我们也可以使用 `pub` 关键字，将结构体和枚举指定为公开，但结构体和枚举的 `pub` 用法时，有一些额外细节。如果我们在结构体定义之前使用 `pub`，那么该结构体就会被公开，但结构体的字段仍然是私有的。我们可以根据具体情况，决定是否公开每个字段。在下面清单 7-9 中，我们定义了一个公开的 `back_of_house::Breakfast` 结构体，其中有个公开的 `toast` 字段，但却有个私有的 `seasonal_fruit` 字段。这就建模了某家餐厅的情况：顾客可以选择配餐的面包类型，但厨师会根据当季和库存情况，决定配餐的水果。可用的水果变化很快，因此顾客无法选择水果，甚至无法看到他们会吃到哪种水果。

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
    // 点一份带黑麦土司的夏日早餐, rye, US /raɪ/, UK /rai/, n.黑麦, 黑麦粒
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println! ("请给我一份 {} 土司", meal.toast);

    // 若不把接下来这行注释掉，那么就不会编译；这里不允许查看或修改
    // 这份餐搭配的应季水果
    // meal.seasonal_fruit = String::from("blueberries");
}
```

*清单 7-9：带有一些公开字段与一些私有字段的结构体*


由于 `back_of_house::Breakfast` 结构体中的 `toast` 字段是公开的，因此在 `eat_at_restaurant` 中，我们可以使用点符号（`.`）写入和读取这个 `toast` 字段。请注意，我们不能在 `eat_at_restaurant` 中，使用 `seasonal_fruit` 字段，因为 `seasonal_fruit` 是私有的。请尝试取消注释那个修改 `seasonal_fruit` 字段值的行，看看会出现什么错误！


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

另外，请注意，由于 `back_of_house::Breakfast` 有个私有字段，因此该结构体就需要提供一个构造 `Breakfast` 实例的公开关联汉斯（我们在这里将其命名为 `summer`）。如果 `Breakfast` 结构体没有一个这样的函数，我们就无法在 `eat_at_restaurant` 中，创建出 `Breakfast` 结构体的实例，因为我们无法在 `eat_at_restaurant` 中设置那个私有 `seasonal_fruit` 字段值。


相反，如果我们将某个枚举构造为公开，那么他的所有变种都会是公开的。我们只需要在 `enum` 关键字前，加上 `pub` 关键字，如下清单 7-10 所示。

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

*清单 7-10：将某个枚举指定为公开，会使其所有变种公开*

因为我们将这个 `Appetizer` 枚举构造为了公开，所以我们可以在 `eat_at_restaurant` 中，使用 `Soup` 和 `Salad` 两个变种。

除非枚举变种是公开的，否则枚举的用处就不大；如果每次都要给所有枚举变种注释上 `pub`，那就太烦人了，所以枚举变种默认是公开的。通常结构体在其字段不公开的情况下也很有用，因此结构体字段遵循了一般规则，即除非被注释为 `pub`，否则默认情况下所有字段，都是私有的。


（End）


