# 将模组拆分为不同文件

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

请注意只需在模组树中的某处，使用一次 `mod` 声明，而将某个文件的内容加载进来。一旦编译器获悉该文件是项目的一部分（且由已将那个 `mod` 语句放置于于何处，而掌握了该代码在模组树中所处的位置），项目中的其他文件，则应如同之前 [用于引用模组树中项目的路径](#用于引用目录树中项目的路径) 小节中，曾讲到的到模组声明处的路径，来引用那个文件中的代码。也就是说，这里的 `mod` *并非* 其他编程语言有的那种 “include” 操作。

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

> **备用文件路径**
>
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

# 本章小节

Rust 实现了包拆分为多个代码箱，进而将代码箱拆分为多个模组，这样就可以从一个模组，对定义在另一模组中的程序项目加以引用。通过指明绝对或相对路径，就可以做到这点。使用 `use` 语句，就可以将这些程序项目的路径，带入到作用域，如此就可以在那个作用域中，多次用到所带入的程序项目时，使用较简短的路径。默认下模组代码是私有的，但可通过添加 `pub` 关键字，而将一些定义构造为公开的。

下一章中，就会看看，可在本地组织良好代码中，使用到的标准库中的一些集合数据结构（collection data structures）。


（End）


