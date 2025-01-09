# 使用允许不同类型值的特质对象

**Using Trait Objects That Allow for Values of Different Types**


> **注**：这类似于 Java 语言中，解决死亡钻石问题（DDD）的 [接口](https://java.xfoss.com/Ch08_Interfaces_and_Abstract_Classes.html#%E4%BD%BF%E7%94%A8%E6%8E%A5%E5%8F%A3%E6%9D%A5%E6%8B%AF%E6%95%91)。

在第 8 章中，咱们就提到过矢量值的一个局限，便是他们只能存储一种类型的元素。在清单 8-9 中咱们创建出了一种变通方案，其中定义了有着分别保存整数、浮点数与文本变种的 `SpreadsheetCell` 枚举。这就意味着咱们可在各个单元格中存储不同类型的数据，而仍旧有了表示这些单元格所组成行的一个矢量值。这对于在咱们的代码被编译时，就已经清楚这些可交换项目，为固定类型集的情况，这确实是一种相当不错的解决办法。

然而，有时咱们会想要咱们库的用户，能够扩展这个于某种特定情形下有效的类型集。为展示咱们将怎样达成这个目的，接下来咱们将创建对一个条目清单加以迭代的示例性图形用户界面，graphical user interface，GUI 工具 -- 对于 GUI 工具来讲这可是一项常见技能。咱们将创建包含 GUI 库架构的名为 `gui` 的一个库代码箱。此代码箱会包含给人类使用的一些类型，比如 `Button` 或 `TextField`。此外，`gui` 的用户将希望创建出他们自己的能被绘制出来的类型：比如，某个程序员要添加一个 `Image`，而另一程序员则要添加一个 `SelectBox`。

对于这个示例，咱们不会实现一个完全成熟的 GUI 库，而是会给出这些部分将怎样一起配合起来。在编写这个库时，咱们没法了解而定义出其他那些程序员可能想要创建的全部类型。但咱们肯定清楚 `gui` 需要追踪各种不同类型的许多不同值，同时他还需要调用这些不同类型值上的 `draw` 方法。其无需明白在咱们调用该 `draw` 方法时，具体会发生什么，他只需知道那个值会让那个方法可被咱们调用。

在有着继承的某门语言中要做到这点，咱们可能会定义其上有着名为 `draw` 的方法的一个名为 `Component` 类。至于其他类，比如 `Button`、`Image` 与 `SelectBox` 等，将从 `Component` 基础并因此继承这个 `draw` 方法。他们可以分别重写这个 `draw` 方法，来定义他们的定制行为，而框架则可以将全部这些类型，当作 `Component` 的实例对待而调用他们之上的 `draw`。但由于 Rust 并无继承，因此咱们需要另一种方法，来架构这个 `gui` 库，来允许用户以新类型来扩展他。


## 定义用于共同行为的特质

**Defining a Trait for Common Behavior**


为了实现咱们想要 `gui` 所拥有的行为，咱们将定义将有着一个名为 `draw` 方法的名为 `Draw` 特质。随后咱们就可以定义取 *特质对象，a trait object* 的一个矢量。特质对象会同时指向实现了这个指定特质的某个类型，以及用于在运行时查找那个类型上特质方法的一张表。咱们是通过指定某种指针，比如某个 `&` 的引用，或某个 `Box<T>` 的灵巧指针，接着便是 `dyn` 关键字，以及随后指明相关特质，创建出特质对象。（在第 19 章的 [“动态大小类型与 `Sized` 特质”](Ch19_Advanced_Features.md#动态大小的类型与-sized-特质) 小节咱们将讲到特质对象必须使用指针的原因。）在泛型或具体类型处，咱们就可以使用特质对象。而不论在何处使用特质对象，Rust 的类型系统都会确保在编译时，在那样的上下文中的任何值，都将实现该特质对象的特质。于是，咱们就无需掌握编译时的所有可能类型了。


咱们已经提到过，在 Rust 中，咱们避免将结构体与枚举称为 “对象”，是为了将二者与其他语言中的对象区别开来。在结构体或枚举中，结构体字段中的数据，与 `impl` 代码块中的行为是分开的，而在其他语言中，数据与行为被结合为通常被标称为对象的这么一个概念。然而，特质对象由于其结合了数据与行为，而 *真的* 更像其他语言中的对象。但从无法添加数据到特质对象上看，特质对象是不同于传统的对象的。特质对象并不如其他语言中的对象那样普遍的有用：其特定用途为实现共用行为的抽象。

下面清单 17-3 给出了怎样定义有着一个名为 `draw` 方法的一个名为 `Draw` 的特质：


文件名：`src/lib.rs`

```rust
pub trait Draw {
    fn draw(&self);
}
```

*清单 17-3：`Draw` 特质的定义*

这种语法应与在第 10 章中关于定义特质的方式看起来类似。接下来便有了一种新的语法：下面清单 17-4 定义了保存着一个名为 `components` 矢量的一个名为 `Screen` 的结构体。该矢量为类型 `Box<dyn Draw>` 的，而 `Box<dyn Draw>` 便是一个特质对象；`Box<dyn Draw>` 是 `Box` 里头实现了 `Draw` 特质的全部类型的代名词。

文件名：`src/lib.rs`

```rust
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}
```

*清单 17-4：带有保存着一个实现了 `Draw` 特质的特质对象矢量的 `components` 字段的 `Screen` 结构体的定义*

在这个 `Screen` 结构体上，咱们将定义将调用其 `components` 各条目上 `draw` 方法的一个名为 `run` 的方法，如下清单 17-5 中所示：

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

*清单 17-5：`Screen` 上会调用各组件上 `draw` 方法的一个 `run` 方法*


这与定义出用到带有特质边界泛型参数的结构体，原理是不同的。泛型参数在某个时间只能用一种具体类型替换，而特质对象则允许在运行时填入多种具体类型。比如，咱们本可以像在下面清单 17-6 中那样，将这个 `Screen` 结构体定义为使用泛型与特质边界：

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

*清单 17-6：其 `run` 方法用到泛型与特质边界的 `Screen` 结构体的一种替代实现*

这种写法就会将咱们限制到有着全是类型 `Button` 或全是类型 `TextField` 组件清单的某个 `Screen` 实例。在咱们将仅有着同质集合，homogeneous collections，时，由于那些定义在编译时，为使用具体类型而将被单一化，那么此时使用泛型与特质边界便是更可取的做法。

另一方面，有了使用特质对象的方法，一个 `Screen` 实例便可以保存包含着 `Box<Button>` 以及 `Box<TextField>` 的 `Vec<T>` 了。下面就来看看其工作原理，并于随后讲讲运行时的性能影响。


## 实现该特质

**Implementing the Trait**


现在咱们将添加实现了这个 `Draw` 特质的一些类型。咱们将提供到这个 `Button` 类型。再次声明，具体实现一个 GUI 库超出了本书的范围，因此这个 `draw` 方法在其函数体中不会有任何有用的实现。为设想其实现可能的样子，那么 `Button` 结构体就可能有着 `width`、`height` 与 `label` 等字段，如下清单 17-7 中所示：

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

*清单 17-7：实现了 `Draw` 特质的 `Button` 结构体*


`Button` 上的 `width`、`height` 与 `label` 字段，将不同于其他组建上的字段；比如，`TextField` 类型就可能有着这些字段外加一个 `placeholder` 字段。各个咱们打算绘制在屏幕上的这些类型，都将实现这个 `Draw` 特质，但会在 `draw` 方法中使用不同代码，来定义出绘制特定类型的方式，正如这里的 `Button` 所拥有的那样（如前面提到的，并无具体代码）。而比如这个 `Button` 类型，则可能包含了在用户点击按钮时，相关方法的一个额外 `impl` 代码块。这些类别的方法，就不会应用到如同 `TextField` 的那些类型。

在使用咱们库的某人，决定要实现有着 `width`、`height` 及 `options` 字段的 `SelectBox` 时，他们也要在 `SelectBox` 类型上的 `Draw` 特质，如下清单 17-8 中所示：

文件名：`src/lib.rs`


```rust
use simple_gui::Draw;

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

*清单 17-8：使用 `simple_gui` 并在 `SelectBox` 结构体上实现 `Draw` 特质的另一代码箱*

咱们库的用户，现在便可以编写他们的 `main` 函数，来创建出 `Screen` 实例。通过将各个 `SelectBox` 与 `Button` 放入到 `Box<T>` 中，而成为特质对象，他们便可以把这些 `SelectBox` 与 `Button` 添加到 `Screen` 实例了。随后他们便可以调用 `Screen` 实例上的 `run` 方法，而其将调用各个组件上的 `draw` 方法。下面清单 17-9 给出了这样的实现：

文件名：`src/main.rs`

```rust
use simple_gui::{Button, Screen};

pub fn main() {
    let screen = Screen {
        components: vec! [
            Box::new(SelectBox {
                width: 25,
                height: 30,
                options: vec! [
                    String::from("选项 A"),
                    String::from("选项 B"),
                    String::from("选项 C"),
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

*清单 17-9：使用特质对象，来存储实现了同一特质的不同类型值*

在咱们编写该库时，咱们是不知道有人会添加这个 `SelectBox` 类型的，但由于 `SelectBox` 实现了 `Draw` 特质，这就表示他实现了那个 `draw` 方法，因此咱们的 `Screen` 实现，就能运作于这个新类型之上而绘制出他来。

这一概念 -- 即尽考虑消息的所要应对的某个值，而非该值的具体类型 -- 与一些动态类型语言中 *鸭子类型，duck typing* 概念类似：若某物像鸭子那样走动，并像鸭子那样呱呱叫，那么他就一定是只鸭子！在清单 17-5 中 `Screen` 上的 `run` 实现中，`run` 不需要掌握各个组件的具体类型为何。他不会检查某个组件是个 `Button` 还是 `SelectBox`，他只会调用那个组件上的 `draw` 方法。通过把 `Box<dyn Draw>` 指定为 `component` 矢量中那些值的类型，咱们就已将 `Screen` 定义为需要咱们可在其上调用 `draw` 方法的一些值了。

运用特质对象与 Rust 的类型系统，来编写出与运用了鸭子类型的代码相类似代码的优势，便是咱们再也不必检查，某个值在运行时是否实现了某个特定方法，也再也不必担心在某个值未实现某个方法，而咱们又调用了该方法时会收到报错了。若值未实现特质对象所需的那些特质，那么 Rust 就不会编译咱们的代码。

比如，下面清单 17-10 便给出了在咱们尝试以一个 `String` 作为组件，创建出一个 `Screen` 时会发生什么：

文件名：`src/main.rs`

```rust
use simple_gui::Screen;

pub fn main() {
    let screen = Screen {
        components: vec! [Box::new(String::from("你好"))],
    };

    screen.run();
}
```

*清单 17-10：尝试使用未实现特质对象之特质的一个类型*

由于 `String` 为实现那个 `Draw` 特质，因此咱们将得到下面这个报错：

```console
$ cargo run                                                                                      ✔  
   Compiling simple_gui v0.1.0 (/home/peng/rust-lang/simple_gui)
error[E0277]: the trait bound `String: Draw` is not satisfied
  --> src/main.rs:23:27
   |
23 |         components: vec! [Box::new(String::from("你好"))],
   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Draw` is not implemented for `String`
   |
   = help: the following other types implement trait `Draw`:
             Button
             SelectBox
   = note: required for the cast from `String` to the object type `dyn Draw`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `simple_gui` due to previous error
```

此报错让咱们明白，要么咱们传递给 `Screen` 了某个不是咱们想要传递的东西，那么就应传递另一个类型，要么咱们应在 `String` 上实现 `Draw`，从而 `Screen` 便可以调用其上的 `draw` 方法。


## 特质对象执行动态调遣

**Trait Object Perform Dynamic Dispatch**


回顾第 10 章中 [“运用了泛型的代码性能问题”](Ch10_Generic_Types_Traits_and_Lifetimes.md#使用泛型参数代码的性能问题) 小节中，在泛型之上运用特质边界时，咱们关于由编译器所完成的单一化过程，the monomorphization process 的讨论：编译器会为咱们在泛型参数处，用到的各个具体类型，而产生出非通用的函数及方法实现。单一化过程所产生的代码，便是在进行 *静态调遣，static dispatch*，这是编译器清楚，咱们在编译时调用的为哪个方法时的情况。这与 *动态调遣，dynamic dispatch* 是相反的，动态调遣是编译器在编译时，无法区分出咱们所调用的为何方法时的情况。在动态调遣情况下，编译器产生出将在运行时，得出要调用方法的代码。

在咱们运用特质对象时，Rust 就必须使用动态调遣。对于全部可能与用到特质对象代码一起使用的类型，编译器并无掌握，因此他就不明白要调用何种类型上的哪个方法。相反，在运行时，Rust 会使用特质对象内部的指针，来掌握要调用哪个方法。这种做法会导致静态调遣下所不会发生的运行时开销。动态调遣还会阻止编译器内联某个方法代码的抉择，这就相应地阻止了一些优化。然而，咱们却真切地获得了，如同咱们在清单 17-5 中所编写的代码那样的灵活性，同时才能够支持清单 17-9 中那样的情况，如此其便是一种需要考量的取舍了。when we use trait objects, Rust must use dynamic dispatch. The compiler doesn't know all the types that might be used with the code that's using trait objects, so it doesn't know which method implemented on which type to call. Instead, at runtime, Rust uses the pointers inside the trait object to know which method to call. This lookup incurs a runtime cost that doesn't occur with static dispatch. Dynamic dispatch also prevents the compiler from choosing to inline a method's code, which in turn prevents some optimizations. However, we did get extra flexibility in the code that we wrote in Listing 17-5 and were able to support in Listing 17-9, so it's a trade-off to consider.


（End）


