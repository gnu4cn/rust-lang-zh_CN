# 使用特质对象来抽象共用行为

> **译注**：这类似于 Java 语言中，解决死亡钻石问题（DDD）的 [接口](https://java.xfoss.com/Ch08_Interfaces_and_Abstract_Classes.html#%E4%BD%BF%E7%94%A8%E6%8E%A5%E5%8F%A3%E6%9D%A5%E6%8B%AF%E6%95%91)。

在第 8 章中，我们提到矢量值的一个限制是，他们只能存储一种类型的元素。我们在 [清单 8-9](../common_collections/vectors.md#listing_8-9) 中创建了一种变通方案，其中定义了一个 `SpreadsheetCell` 枚举，有着分别保存整数、浮点数与文本的变种。这就意味着我们可以在每个单元格中存储不同类型的数据，并且仍然有个表示一行单元格的矢量值。当我们的可互换项在代码编译时就属于已知的固定类型时，这是一种非常好的解决方案。

然而，有时我们希望库的用户，能够扩展这一在特定情形下有效的类型集。为了展示我们如何实现这一点，我们将创建一个示例图形用户界面（GUI）工具，其将遍历项目列表，对每个项目调用 `draw` 方法来绘制项目到屏幕 -- 这属于 GUI 工具的常见技术。我们将创建一个名为 `gui` 的库，包含 GUI 库的架构。这个代码箱可能包含一些供人们使用的类型，例如 `Button` 或 `TextField`。此外，`gui` 的用户将希望创建自己的可绘制类型：例如，某名程序员可能会添加一个 `Image`，而另一程序员可能会添加一个 `SelectBox`。

在编写这个库时，我们无法知道并定义其他程序员可能想要创建的所有类型。但我们确实清楚 `gui` 需要追踪许多不同类型的值，并且他需要对每个这些不同类型的值都调用 `draw` 方法。他无需确切知道调用 `draw` 方法时将发生什么，只需知道值将具有可供我们调用的方法即可。

要在有着继承的语言中实现这一点，我们可以定义一个名为 `Component` 类，有着名为 `draw` 的方法。其他类，比如 `Button`、`Image` 与 `SelectBox` 等，将继承自 `Component`，从而继承 `draw` 方法。他们每个都可以重写 `draw` 方法，来定义他们的定制行为，而框架可以将所有类型都视为 `Component` 的实例，并对他们调用 `draw` 方法。但由于 Rust 没有继承，我们需要另一种方式来架构 `gui` 库，以允许用户创建与该库兼容的新类型。


## 为共同行为定义特质

为了实现我们希望 `gui` 具有的行为，我们将定义一个名为 `Draw` 的特质，他将有个名为 `draw` 方法。然后，我们可以定义一个取特质对象的矢量值。所谓 *特质对象*，既指向实现我们指定特质的类型的实例，也指向用于在运行时查找该类型上特质方法的数据表。我们通过指定某种指针，比如引用或 `Box<T>` 灵巧指针，接着是 `dyn` 关键字，然后指定相关特质创建特质对象。（我们将第 20 章中 [动态大小类型与 `Sized` 特质](../advanced_features/adv_types.md#动态大小的类型与-sized-特质) 小节中，讨论特质对象必须使用指针的原因。）我们可以使用特质对象来代替泛型或具体类型。无论我们在何处使用特质对象，Rust 的类型系统都会在编译时确保该上下文中使用的任何值，都将实现该特质对象的特质。因此，我们不需要在编译时就了解所有可能类型。

我们已经提到过，在 Rust 中，我们避免称结构体和枚举为 “对象”，以将二者与其他语言中的对象区分开来。在结构体或枚举中，结构体字段中的数据，与 `impl` 代码块中的行为是分开的，而在其他语言中，组合为一个概念的数据与行为，通常被标记为对象。特质对象不同于其他语言中的对象，在于我们无法添加数据到特质对象。特质对象不如其他语言中的对象具有普遍的实用性：他们的特定用途是允许对共用行为抽象。

下面清单 18-3 展示了怎样以一个名为 `draw` 方法，来定义一个名为 `Draw` 的特质。

<a name="listing_18-3"></a>
文件名：`src/lib.rs`

```rust
pub trait Draw {
    fn draw(&self);
}
```

**清单 18-3**：`Draw` 特质的定义

根据我们在第 10 章中对怎样定义特质的讨论，这种语法应该看起来很熟悉。接下来是一些新的语法：下面清单 18-4 定义了一个名为 `Screen` 的结构体，其中包含一个名为 `components` 的矢量值。这个矢量值的类型为 `Box<dyn Draw>`，这是个特质对象；他代表 `Box` 内任何实现 `Draw` 特质的类型。

<a name="listing_18-4"></a>
文件名：`src/lib.rs`

```rust
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}
```

**清单 17-4**：`Screen` 结构体的定义，有着一个包含矢量值的 `components` 字段，而矢量值由实现 `Draw` 特质的特质对象构成

在 `Screen` 结构体上，我们将定义一个名为 `run` 的方法，他将调用每个 `components` 上的 `draw` 方法，如下清单 18-5 中所示。

<a name="listing_18-5"></a>
文件名：`src/lib.rs`

```rust
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

**清单 18-5**：`Screen` 上的 `run` 方法，会调用每个组件上的 `draw` 方法

这与定义一个使用带有特质边界的泛型类型参数定义结构体的工作方式不同。泛型类型参数一次只能以一种具体类型替换，而特质对象允许在运行时由多种具体类型填充。例如，我们本可以像下面清单 18-6 中那样，使用泛型类型和特质边界定义 `Screen` 结构体。

<a name="listing_18-6"></a>
文件名：`src/lib.rs`

```rust
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl <T> Screen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

**清单 18-6**：使用泛型和特质边界的 `Screen` 结构体及其 `run` 方法的替代实现

这会限制我们为某个 `Screen` 有着全部是 `Button` 类型，或全部是 `TextField` 类型的组件列表。当咱们将仅有着同质集合，homogeneous collections，时，那么最好使用泛型和特质边界，因为定义将在编译时被单态化，monomorphized，以使用具体类型。

另一方面，在使用特质对象方式下，一个 `Screen` 实例可以保存包含 `Box<Button>` 以及 `Box<TextField>` 的 `Vec<T>`。我们来看看其工作原理，然后我们将讨论运行时性能的影响。


## 实现特质

现在我们将添加一些实现 `Draw` 特质的类型。我们将提供 `Button` 类型。再次强调，具体实现一个 GUI 库超出了这本书的范围，因此 `draw` 方法在其主体中不会有任何有用的实现。为了想象该实现可能的样子，`Button` 结构体可能有着 `width`、`height` 与 `label` 等字段，如下清单 18-7 中所示。

<a name="listing_18-7"></a>
文件名：`src/lib.rs`

```rust
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // 具体绘制按钮的代码
    }
}
```

**清单 18-7**：实现 `Draw` 特质的 `Button` 结构体

`Button` 上的 `width`、`height` 与 `label` 字段，将不同于其他组建上的字段；例如，`TextField` 类型可能有着这些同样字段，外加一个 `placeholder` 字段。我们打算在屏幕上绘制的每种类型，都将实现 `Draw` 特质，但在 `draw` 方法中将使用不同代码，来定义如何绘制该特定类型，就像这里的 `Button` 所做的那样（如前面提到的，并无实际的 GUI 代码）。例如，`Button` 类型可能有个额外的 `impl` 代码块，包含与用户点击按钮时发生的行为相关的方法。此类方法不适用于 `TextField` 等类型。

当使用我们库的人决定实现一个有着 `width`、`height` 及 `options` 字段的 `SelectBox` 结构体时，他们也将在 `SelectBox` 类型上实现 `Draw` 特质，如下清单 18-8 中所示。

<a name="listing_18-8"></a>
文件名：`src/main.rs`

```rust
use gui::Draw;

pub struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // 具体绘制复选框的代码
    }
}
```

**清单 18-8**：另一个使用 `gui` 并在 `SelectBox` 结构体上实现 `Draw` 特质的代码箱

我们库的用户现在可以编写他们的 `main` 函数，来创建 `Screen` 实例。通过将 `SelectBox` 与 `Button` 放入 `Box<T>` 中以成为特质对象，他们便可以添加这两个组件到 `Screen` 实例。然后，他们可以调用 `Screen` 实例上的 `run` 方法，该方法将依次调用每个组件上的 `draw` 方法。下面清单 18-9 展示了这一实现。

<a name="listing_18-9"></a>
文件名：`src/main.rs`

```rust
use gui::{Button, Screen};

pub fn main() {
    let screen = Screen {
        components: vec! [
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec! [
                    String::from("是"),
                    String::from("可能"),
                    String::from("否"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
```

**清单 18-9**：使用特质对象来存储实现同一特质的不同类型值

在编写库时，我们并不知道有人可能会添加 `SelectBox` 类型，但我们的 `Screen` 实现能够处理这一新类型并绘制他，因为 `SelectBox` 实现了 `Draw` 特质，这意味着他实现了 `draw` 方法。

这一概念 -- 即只关心值响应的消息，而非值的具体类型 -- 与动态类型语言中 *鸭子类型，duck typing* 概念类似：若他像鸭子一样行走，并且像鸭子一样嘎嘎叫，那么他一定就是鸭子！在清单 18-5 中 `Screen` 的 `run` 方法实现中，`run` 不需要每个组件的具体类型。他不会检查某个组件是 `Button` 还是 `SelectBox` 的实例，他只会调用组件上的 `draw` 方法。通过指定 `Box<dyn Draw>` 作为 `component` 矢量中值的类型，我们定义了 `Screen` 为需要我们可以调用其上的 `draw` 方法的值。

使用特质对象和 Rust 的类型系统，来编写与使用鸭子类型类似的代码的优点是，我们无需在运行时检查某个值是否实现了特定方法，也不必担心当某个值未实现某个方法，而我们无论如何又调用了他时会引发错误。当值未实现特质对象所需的特质时，Rust 将不编译我们的代码。

例如，下面清单 18-10 展示了当我们尝试以一个 `String` 作为组件，创建 `Screen` 时会发生什么。

<a name="listing_18-10"></a>
文件名：`src/main.rs`

```rust
use gui::Screen;

pub fn main() {
    let screen = Screen {
        components: vec! [Box::new(String::from("你好"))],
    };

    screen.run();
}
```

**清单 18-10**：尝试使用未实现特质对象的特质的类型

我们将得到下面这个报错，因为 `String` 没有实现 `Draw` 特质：

```console
$ cargo run
   Compiling gui v0.1.0 (/home/hector/rust-lang-zh_CN/projects/gui)
error[E0277]: the trait bound `String: Draw` is not satisfied
  --> src/main.rs:5:27
   |
 5 |         components: vec! [Box::new(String::from("你好"))],
   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Draw` is not implemented for `String`
   |
help: the trait `Draw` is implemented for `Button`
  --> /home/hector/rust-lang-zh_CN/projects/gui/src/lib.rs:23:1
   |
23 | impl Draw for Button {
   | ^^^^^^^^^^^^^^^^^^^^
   = note: required for the cast from `Box<String>` to `Box<dyn Draw>`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `gui` (bin "gui") due to 1 previous error
```

这个报错让我们知道，要么我们正在向 `Screen` 传递一些我们不想传递的内容，因此应该传递不同的类型，要么我们应该在 `String` 上实现 `Draw`，以便 `Screen` 可以在其上调用 `draw` 方法。


## 执行动态分派

回顾第 10 章中 [使用泛型的代码的性能问题](../generic_types_traits_and_lifetimes/generics.md#用到泛型的代码的性能问题) 中，我们对编译器对泛型执行的单态化过程，the monomorphization process 的讨论：编译器会针对我们用来代替泛型类型参数的每个具体类型，生成函数和方法的非泛型实现。单态化产生的代码，就是在执行 *静态分派，static dispatch*，即编译器在编译时就知道咱们正在调用哪个方法。这与 *动态分派，dynamic dispatch* 相反，动态分派是指编译器在编译时无法确定咱们正在调用哪个方法。在动态分派的情况下，编译器会生成在运行时才确定调用哪个方法的代码。

当我们使用特质对象时，Rust 必须采用动态分派。编译器不知道可能与使用特质对象的代码一起使用的所有类型，因此不知道要调用哪个类型上实现的哪个方法。相反，在运行时，Rust 会使用特质对象内部的指针，来获悉要调用哪个方法。这种查找会产生静态分派不会发生的运行时开销。动态分派还会阻止编译器选择将方法代码内联，进而阻碍某些优化，并且 Rust 对于在哪里可以和不能使用动态分派有一些规则，称为 *`dyn` 兼容性*。<a name="dyn_rules"></a>这些规则超出了本讨论的范围，但咱们可以在 [参考文档中](https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility) 了解更多相关内容。不过，我们在清单 18-5 中编写，并且在清单 18-9 中得以支持的代码中，确实获得了额外的灵活性，因此这是一种需要权衡的取舍。

（End）


