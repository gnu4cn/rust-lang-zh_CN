#  用于引用模组树中项目的路径

**Paths for Referring to an Item in the Module Tree**


为告诉 Rust 在模块树中的何处，可以找到某个项目，我们就像在文件系统中一样，用到了路径。要调用某个函数，我们需要知道他的路径。

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


第一次调用 `eat_at_restaurant` 中的 `add_to_waitlist` 函数时，我们使用的是绝对路径。`add_too_waitlist` 函数与 `eat_at_restaurant`，定义在同一个代码箱中，这意味着我们可以使用 `crate` 关键字，来开始绝对路径。然后，我们逐个包含后续模块，直到找到 `add_to_waitlist`。咱们可以想象某种具有相同结构的文件系统：我们指定 `/front_of_house/hosting/add_to_waitlist` 路径，来运行 `add_to_waitlist` 程序；使用 `crate` 这个名字，从代码箱根目录开始，就像在 shell 中，使用 `/` 从文件系统根目录开始一样。

第二次在 `eat_at_restaurant` 中调用 `add_too_waitlist` 时，我们使用了相对路径。该路径以 `front_of_house` 开头，`front_of_house` 是与 `eat_at_restaurant` 定义在模组树同一级别处，模组的名字。在这里，文件系统等价的做法，是使用路径 `front_of_house/hosting/add_to_waitlist`。以模组名字开头，就意味着路径是相对的。

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


##  使用 `pub` 关键字暴露路径

**Exposing Paths with the `pub` Keyword**


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


## 使用 `super` 关键字开始相对路径

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


## 将结构体与枚举构造为公共项目

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
