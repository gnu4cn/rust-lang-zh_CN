# 将模组拆分为不同文件

**Separating Modules into Different Files**


到目前为止，本章的所有示例都把多个模组，定义在一个文件中。当模组变大时，咱们就会想要把他们的定义，移入到单独文件中，这样代码更容易浏览。

例如，我们来从 [清单 7-17](./the_use_keyword.md#listing-7-17) 中，包含了多个餐厅模组的代码开始。我们将把这些模组提取到一些文件中，而不是把所有模组都定义在代码箱的根文件里。在本例中，代码箱的根文件为 `src/lib.rs`，但这一过程也适用于根文件为 `src/main.rs` 的二进制代码箱。

首先，我们将把 `front_of_house` 这个模组提取到他自己的文件中。请移除 `front_of_house` 模组的花括号内的代码，只留下 `mod front_of_house;` 这个声明，这样 `src/lib.rs` 文件就包含着下面清单 7-21 所示的代码。请注意，在创建出清单 7-22 中的 `src/front_of_house.rs` 文件前，这段代码将不会编译。


文件名：`src/lib.rs`

```rust
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

*清单 7-21：声明出其模组代码主体，将位于 `src/front_of_house.rs` 中的 `front_of_house` 模组*

接下来，将原先花括符中的代码，放入一个名为 `src/front_of_house.rs` 的新文件中，如下清单 7-22 所示。编译器之所以知道要在这个文件中查找，是因为他在代码箱根中，看到了名为 `front_of_house` 的模组声明。


文件名：`src/front_of_house.rs`

```rust
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```

*清单 7-22：`src/front_of_house.rs` 中 `front_of_house` 模组内部的定义*

请注意，在咱们的模组树中，咱们只需使用 `mod` 声明加载某个文件 *一次*。一旦编译器知道了该文件是项目的一部分（且编译器会因咱们放置 `mod` 语句位置，而知道该代码位处模组树中何处），咱们项目中的其他文件，就应能使用 [“引用模组树中某个项目的路径”](./paths.md) 小节中介绍的到其声明处的路径，引用该加载文件的代码。换句话说，`mod` 并 *不是* 咱们在其他编程语言中，可能会看到的 “包含” 操作。

接下来，我们将把 `hosting` 模组提取到他自己的文件中。这个过程略有不同，因为 `hosting` 是 `front_of_house` 的一个子模组，而不是根模组。我们将把 `hosting` 的文件，放到一个模组树中的以其祖先命名的新目录中，在此情形下即为 `src/front_of_house`。

要开始迁移 `hosting`，我们就要将 `src/front_of_house.rs`，修改为只包含 `hosting` 模组的声明：

文件名：`src/front_of_house.rs`

```rust
pub mod hosting;
```

然后，我们创建一个 `src/front_of_house` 目录及一个存放 `hosting` 模组中所定义内容的 `hosting.rs` 文件：

文件名：`src/front_of_house/hosting.rs`

```rust
pub fn add_to_waitlist() {}
```

如果我们将 `hosting.rs` 文件放在 `src` 目录下，编译器就会期望 `hosting.rs` 的代码，属于在代码箱根中声明的 `hosting` 模组，而非作为 `front_of_house` 模组子模组声明的。确定哪些文件属于哪个模组代码的编译器规则，意味着目录和文件结构，会与模组树更加一致。

> **备用文件路径**
>
> 到目前为止，我们已经介绍了 Rust 编译器所使用的最常见文件路径，但 Rust 还支持一种较旧的文件路径风格。对于代码箱根中声明的一个名为 `front_of_house` 模组，编译器将在以下位置查找该模组的代码：

- `src/front_of_house.rs`（即这里讲到的）；
- `src/front_of_house/mod.rs`（较早样式，仍被支持的路径）。

> 对于 `front_of_house` 子模组的一个名为 `hosting` 的模组，编译器将在以下位置查找该模组的代码：

- `src/front_of_house/hosting.rs`（即这里讲到的）；
- `src/front_of_house/hosting/mod.rs`（较早样式，仍被支持的路径）。

> 若咱们对同一模组同时使用这两种样式，咱们将得到一个编译器报错。在同一个项目中对不同模组混合使用这两种样式是允许的，但这可能会让浏览咱们项目的人感到困惑。
>
> 使用名为 `mod.rs` 文件样式的主要缺点，是咱们的项目可能会产生大量名为 `mod.rs` 的文件，当咱们在咱们的编辑器中同时打开这些文件时，这就可能造成混淆。

我们已将各个模组的代码移至独立文件中，同时模组树保持不变。`eat_at_restaurant` 中的函数调用，在无需任何修改下就将正常工作，尽管这些函数的定义现在位于别的文件中。随着模组规模扩大，这种技术允许咱们将其迁移至新文件。

请留意 `src/lib.rs` 中的 `pub use crate::front_of_house::hosting` 这个语句没有改变，且 `use` 语句对哪些文件被作为该代码箱的部分而被编译也没有任何影响。其中的 `mod` 关键字声明了这些模组，同时 Rust 会在与模组同名的文件中，查找要放入该模组的代码。


# 本章小节

Rust 允许咱们将包拆分为多个代码箱，并将代码箱拆分为模组，这样咱们就可以从一个模组，引用另一模组中所定义的那些项目。通过指定绝对路径或相对路径，咱们就可以做到这点。使用 `use` 语句，这些路径就被带入作用域，这样咱们就可以在该作用域中，对项目的多次引用使用一个更短的路径。模组代码默认是私有的，但咱们可通过添加 `pub` 关键字，将他们的定义构造为公开。

在下一章中，我们将探讨标准库中的一些集合数据结构，在咱们良好组织的代码中，咱们就可以使用这些集合数据结构。


（End）


