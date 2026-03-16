# 拆分模组为不同文件

到目前为止，这一章的所有示例都在一个文件中定义了多个模组。当模组变大时，咱们就可能希望迁移他们的定义到单独文件，使代码更易于导览。

例如，我们来从 [清单 7-17](./the_use_keyword.md#listing_7-17) 中有着多个餐厅模组的代码开始。我们将提取模组到文件中，而不是让全部模组都定义在代码箱的根文件中。在这一情形下，代码箱的根文件为 `src/lib.rs`，但这一过程也适用于根文件为 `src/main.rs` 的二进制代码箱。

首先，我们将提取 `front_of_house` 模组到他自己的文件。请移除 `front_of_house` 模组花括号内的代码，只留下 `mod front_of_house;` 声明，以便 `src/lib.rs` 文件包含下面清单 7-21 中所示的代码。请注意，在我们创建清单 7-22 中的 `src/front_of_house.rs` 文件前，这段代码将不编译。


<a name="listing_7-21"></a>
文件名：`src/lib.rs`

```rust
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

**清单 7-21**：声明`front_of_house` 模组，其主体将位于 `src/front_of_house.rs`

接下来，将曾在那个花括符中的代码放入名为 `src/front_of_house.rs` 的新文件中，如下清单 7-22 中所示。编译器知道要在这个文件中查找，因为他在代码箱根中遇到了名为 `front_of_house` 的模组声明。


<a name="listing_7-22"></a>
文件名：`src/front_of_house.rs`

```rust
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```

**清单 7-22**：`src/front_of_house.rs` 中 `front_of_house` 模组内的定义

请注意，咱们只需在模组树中使用 `mod` 声明加载文件 *一次*。一旦编译器知道了该文件属于项目的一部分（并由于咱们放置 `mod` 语句之处而知道代码在模组树中的何处），咱们项目中的其他文件，应使用其被声明处的路径来引用已加载文件的代码，如同 [“引用模组树中项目的路径”](./paths.md) 小节中所介绍的。换句话说，`mod` *不是* 咱们可能已在其他编程语言中看到的 “包含，include” 操作。

接下来，我们将提取 `hosting` 模组到他自己的文件。这个过程略有不同，因为 `hosting` 是 `front_of_house`，而非根模组的子模组。我们将把 `hosting` 的文件放在一个新的目录中，该目录将以模组树中他的父辈命名，在此情形下即为 `src/front_of_house`。

要开始迁移 `hosting`，我们修改 `src/front_of_house.rs` 为只包含 `hosting` 模组的声明：

文件名：`src/front_of_house.rs`

```rust
pub mod hosting;
```

然后，我们创建一个 `src/front_of_house` 目录及一个 `hosting.rs` 文件，来包含构造于 `hosting` 模组中的定义：


文件名：`src/front_of_house/hosting.rs`

```rust
pub fn add_to_waitlist() {}
```

相反若我们放置 `hosting.rs` 于 `src` 目录下，编译器就会期望 `hosting` 模组中的 `hosting.rs` 代码被声明于代码箱根中，而不是声明为 `front_of_house` 模组的子模组。编译器关于 “检查哪些文件以获取哪些模组的代码” 规则，意味着目录和文件会紧密地匹配模组树。

> **备用文件路径**
>
> 到目前为止，我们已介绍了 Rust 编译器用到的最惯用的文件路径，但 Rust 还支持一种较早的文件路径样式。对于声明于代码箱根中名为 `front_of_house` 的模组，编译器将在以下位置查找该模组的代码：
>
> - `src/front_of_house.rs`（我们介绍的）；
> - `src/front_of_house/mod.rs`（较早样式，仍受支持的路径）。
>
> 对于名为 `hosting` 属于 `front_of_house` 子模组的模组，编译器将在以下位置查找该模组的代码：
>
> - `src/front_of_house/hosting.rs`（我们介绍的）；
> - `src/front_of_house/hosting/mod.rs`（较早样式，仍受支持的路径）。
>
> 若咱们对同一模组使用两种样式，咱们将得到一个编译器报错。虽然在同一项目中针对不同模组混合使用两种样式是允许的，但这可能会让浏览咱们项目的人感到困惑。
>
> 使用名为 `mod.rs` 文件的样式的主要缺点是，咱们的项目最终会有大量名为 `mod.rs` 的文件，当咱们同时在编辑器中打开他们时，这会造成混淆。

我们已经迁移各个模组的代码到单独文件，而模组树保持不变。`eat_at_restaurant` 中的函数调用在无需任何修改下仍将有效，即使定义位于不同文件。这一技巧允许咱们在模组大小增加时，迁移他们到新的文件。

请注意，`src/lib.rs` 中的 `pub use crate::front_of_house::hosting` 同样未曾改变，`use` 语句既对作为代码箱一部分编译的文件也无任何影响。`mod` 关键字声明模组，而 Rust 会在与模组同名的文件中查找进入该模组的代码。


# 本章小节

Rust 允许咱们将包拆分为多个代码箱，并拆分代码箱为模组，以便咱们可以在一个模组中引用定义在另一模组中的项目。咱们可通过指定绝对路径或相对路径做到这点。这些路径可在 `use` 语句下带入作用域，以便咱们可以针对项目在该作用域中的多次引用，而使用较短的路径。默认情况下模组代码是私有的，但咱们可通过添加 `pub` 关键字，构造定义为公开。

在下一章中，我们将探讨标准库中的一些集合数据结构，咱们可在咱们良好组织的代码中使用他们。


（End）


