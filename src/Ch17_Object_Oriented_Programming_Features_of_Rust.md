# Rust 的面向对象编程特性

**Object Oriented Programming Features of Rust**

面向对象编程方法，object-oriented programming, OOP, 是建模程序的一种方法。对象是在 20 世纪 60 年代，在编程语言 Simula 中所引入的一个程序化概念。正是那些对象，影响了 Alan Kay 的编程架构，其中对象会相互传递消息。为描述这种架构，他在 1967 年创造了面向对象编程这个术语。有许多互相竞争的定义，都描述了 OOP 是什么，而根据其中一些定义，Rust 属于面向对象的，但根据另一些，Rust 则不属于面向对象的。在本章中，咱们将探讨通常被看作是面向对象的一些特征，以及这些特征怎样被转译为 Rust 的习惯说法。随后咱们将给出在 Rust 怎样实现面向对象的设计模式，并讨论在这样做，与相反采用 Rust 的一些长处来实现解决方案，之间的权衡取舍。


## 面向对象语言的特征

**Characteristics of Object-Oriented Languages**


在编程界，并无关于某门被视为面向对象的，而必须具有哪些特性的共识。Rust 受了许多编程范式，programming paradigms，的影响，其中就包括 OOP；比如在第 13 章中，咱们就曾探讨过，那些来自于函数式编程的特性。可以说，那些 OOP 的语言，确实是共用了一些确切的特征的，那即是对象、封装与继承等。下面就来看看这些特征各自指的是什么，以及 Rust 是否支持他们。


### 对象包含了数据及行为

**Objects Contain Data and Behavior**


Erich Gamma、Richard Helm、Ralph Johnson 及 John Vlissides 等的合著 *Design Patterns: Elements of Reusable Object-Oriented Software* （Addison-Wesley Professional, 1994），又被通俗地叫做 *The Gang of Four* 书，便是面向对象设计模式的一个目录。该书像下面这样定义了 OOP：

> 面向对象程序是由对象所组成的。*对象，an object* 同时打包了数据与运行在那数据上的过程。这些过程一般就叫做 *方法，methods* 或 *操作，operations*。

运用这个定义，Rust 便是面向对象的：结构体与枚举均有着数据，而 `impl` 块则提供了结构体与枚举上的那些方法。即使有着方法的那些结构体与枚举未*被称作* 对象，根据 The Gang of Four 的对象定义，他们提供了同样的功能。


### 隐藏了实现细节的封装

**Encapsulation that Hides Implementation Details**


通常与 OOP 相关的另一方面的 *封装，encapsulation*，是指对于用到该对象的代码，对象实现细节是不可访问的。由此，与对象交互的唯一方式，便是经由该对象的公开 API；运用对象的代码，不应具备到达该对象内部，而直接改变数据或行为的能力。这实现了程序员在无需修改用到对象的那些代码之下，修改或重构对象的那些内部代码。

在第 7 章中，咱们曾讨论过怎样控制封装：咱们可以使用 `pub` 关键字，来决定咱们代码中，哪些模组、类型、函数与方法等应为公开的，而默认其他所有项目都是私有的。比如，咱们就可以定义有着包含 `i32` 值矢量的一个字段的 `AveragedCollection` 结构体。这个字段也可以有包含着那个矢量中值的平均数的一个字段，表示在有人需要该平均值时，不必按需计算出该平均值。换句话说，`AveragedCollection` 将为咱们缓存这个计算出的平均值。下面清单 17-1 便有着这个 `AveragedCollection` 结构体的定义：

文件名：`src/lib.rs`

```rust
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}
```

*清单 17-1：维护着一个整数清单及该集合中项目平均数的 `AveragedCollection` 结构体*

该结构体被标记为 `pub`，从而其他代码就可以使用他，而该结构体内部的那些字段保持着私有。由于咱们打算不管何时在有某个值被添加到清单，或从清单移除时，其中的平均数也要同时被更新，因此在这个示例中这样的封装就很重要。咱们是通过实现下面清单 17-2 中所给出的 `add`、`remove` 及 `average` 方法，做到这一点的。

文件名：`src/lib.rs`

```rust
impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}
```

*清单 17-2：`AveragedCollection` 上公开方法 `add`、`remove` 与 `average` 的实现*

这些公开方法 `add`、`remove` 与 `average`，是仅有的访问或修改 `AveragedCollection` 实例中数据的方式。在使用 `add` 方法或 `remove` 方法，添加或移除某个条目时，其各自的实现，就同时会调用处理更新 `average` 字段的私有 `update_average` 方法。

咱们把 `list` 与 `average` 自动留着私有，从而外部代码就无法直接添加项目到那个 `list`，或直接从那个 `list` 移除项目；不然的话，在那个`list` 变化时，`average` 字段就可能失去同步。其中的 `average` 方法，返回的是 `average` 字段中的值，这实现了外部代码读取那个 `average` 而不会修改他。

由于咱们已封装了结构体 `AveragedCollection` 的实现细节，因此咱们就可以在将来轻易地修改各个方面，诸如数据结构等。比如，咱们可以对其中的 `list` 字段，使用 `HashSet<i32>` 而非 `Vec<i32>`。只要 `add`、`remove` 及 `average` 三个公开方法的签名保持不变，那些使用 `AveragedCollection` 的代码就无需改变。而相反若咱们把 `list` 构造为公开，就未必如此了：`HashSet<i32>` 与 `Vec<i32>` 有着添加和一处条目的不同方法，由此在外部代码直接修改 `list` 时，就大概率不得不修改了。

若封装是某门语言被视为面向对象的要件，你们 Rust 是满足那种要求的。对代码的不同部分，使用抑或不使用 `pub` 的选项，实现了实现细节的封装。


### 以类型系统及以代码共用的继承

**Inheritance as a Type System and as Code Sharing**


*继承，inheritance*，乃籍以实现对象从另一对象继承一些元素，从而在不必再度定义这些元素之下，获得父辈对象数据与行为的一种机制。

若某们语言务必要有着继承，方能成为一门面向对象语言，那么 Rust 就不算是面向对象语言。在不使用宏，a macro 之下，没有定义出继承父辈结构体字段与方法实现的结构体的方法。

然而，若在编程工具箱中惯于使用继承，那么依据咱们将继承作为头等大事的自身理由，是可以运用 Rust 中别的一些办法的。

之所以选用继承，大致有两种原因。一个是代码的重用：咱们可以对一个类型实现一些特定行为，而继承就让咱们可以对另一类型重用那些实现。咱们可以使用一些默认的特质方法实现，即咱们曾在清单 10-14 中，将 `summarize` 方法的一个默认实现，添加到 `Summary` 特质上时所见到的那样，在 Rust 代码中以一种受限方式做到这点。任何实现了这个 `Summary` 特质的类型，在无需更多代码之下，都将在其上有着这个 `summarize` 方法。这与父类有着某个方法的实现，同时集成的子类也会有着该方法的实现是类似的。在实现这个 `Summary` 特质时，咱们也可以重写 `summarize` 方法的默认实现，这与子类重新继承自父类的方法实现类似。

而使用与类型系统相关继承的另一原因：即为了实现在与父类型的同一地方，使用子类型。这又被成为 *多态，polymorphism*，是指在多个对象共用了一些确切特征时，咱们可以相互替换使用他们。

> **关于多态**
>
> 对许多人来讲，多态等同于继承。但他实际上指的是代码可工作于多个类型数据之下的一个宽泛概念。而对于继承，这些类型则是通用的一些子类。
>
> Rust 则运用了泛型，来对各异的各种可能类型加以抽象，并使用特质边界来强化这些类型所必须提供的那些约束。有时这样的做法，又被叫做 *有边界的参数化多态，bounded parametric polymorphism*。

由于继承通常有着共用了超出必要代码的风险，时至今日，其已在许多编程语言中，作为编程设计模式而失宠了。子类本不应共用其父类的全部特征，但在继承父类时却会这样做。这就会造成程序的设计有较低的灵活性。由于子类从父类继承的一些方法并不适用于子类，因此继承还会引入调用子类上无意义或引发错误方法的可能。此外，一些语言还只将运行单一继承（即子类只能从一个类继承），这进一步限制了程序设计的灵活度。

由于这些原因，Rust 便采取了运用特质对象，而非继承的方法。接下来就要看看特质对象是如何实现 Rust 中的多态。


## 使用允许不同类型值的特质对象

**Using Trait Objects That Allow for Values of Different Types**

> **注**：这类似于 Java 语言中，解决死亡钻石问题（DDD）的 [接口](https://java.xfoss.com/Ch08_Interfaces_and_Abstract_Classes.html#%E4%BD%BF%E7%94%A8%E6%8E%A5%E5%8F%A3%E6%9D%A5%E6%8B%AF%E6%95%91)。

在第 8 章中，咱们就提到过矢量值的一个局限，便是他们只能存储一种类型的元素。在清单 8-9 中咱们创建出了一种变通方案，其中定义了有着分别保存整数、浮点数与文本变种的 `SpreadsheetCell` 枚举。这就意味着咱们可在各个单元格中存储不同类型的数据，而仍旧有了表示这些单元格所组成行的一个矢量值。这对于在咱们的代码被编译时，就已经清楚这些可交换项目，为固定类型集的情况，这确实是一种相当不错的解决办法。

然而，有时咱们会想要咱们库的用户，能够扩展这个于某种特定情形下有效的类型集。为展示咱们将怎样达成这个目的，接下来咱们将创建对一个条目清单加以迭代的示例性图形用户界面，graphical user interface，GUI 工具 -- 对于 GUI 工具来讲这可是一项常见技能。咱们将创建包含 GUI 库架构的名为 `gui` 的一个库代码箱。此代码箱会包含给人类使用的一些类型，比如 `Button` 或 `TextField`。此外，`gui` 的用户将希望创建出他们自己的能被绘制出来的类型：比如，某个程序员要添加一个 `Image`，而另一程序员则要添加一个 `SelectBox`。

对于这个示例，咱们不会实现一个完全成熟的 GUI 库，而是会给出这些部分将怎样一起配合起来。在编写这个库时，咱们没法了解而定义出其他那些程序员可能想要创建的全部类型。但咱们肯定清楚 `gui` 需要追踪各种不同类型的许多不同值，同时他还需要调用这些不同类型值上的 `draw` 方法。其无需明白在咱们调用该 `draw` 方法时，具体会发生什么，他只需知道那个值会让那个方法可被咱们调用。

在有着继承的某门语言中要做到这点，咱们可能会定义其上有着名为 `draw` 的方法的一个名为 `Component` 类。至于其他类，比如 `Button`、`Image` 与 `SelectBox` 等，将从 `Component` 基础并因此继承这个 `draw` 方法。他们可以分别重写这个 `draw` 方法，来定义他们的定制行为，而框架则可以将全部这些类型，当作 `Component` 的实例对待而调用他们之上的 `draw`。但由于 Rust 并无继承，因此咱们需要另一种方法，来架构这个 `gui` 库，来允许用户以新类型来扩展他。


### 定义用于共同行为的特质

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


### 实现该特质

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


### 特质对象执行动态调遣

**Trait Object Perform Dynamic Dispatch**

回顾第 10 章中 [“运用了泛型的代码性能问题”](Ch10_Generic_Types_Traits_and_Lifetimes.md#使用泛型参数代码的性能问题) 小节中，在泛型之上运用特质边界时，咱们关于由编译器所完成的单一化过程，the monomorphization process 的讨论：编译器会为咱们在泛型参数处，用到的各个具体类型，而产生出非通用的函数及方法实现。单一化过程所产生的代码，便是在进行 *静态调遣，static dispatch*，这是编译器清楚，咱们在编译时调用的为哪个方法时的情况。这与 *动态调遣，dynamic dispatch* 是相反的，动态调遣是编译器在编译时，无法区分出咱们所调用的为何方法时的情况。在动态调遣情况下，编译器产生出将在运行时，得出要调用方法的代码。

在咱们运用特质对象时，Rust 就必须使用动态调遣。对于全部可能与用到特质对象代码一起使用的类型，编译器并无掌握，因此他就不明白要调用何种类型上的哪个方法。相反，在运行时，Rust 会使用特质对象内部的指针，来掌握要调用哪个方法。这种做法会导致静态调遣下所不会发生的运行时开销。动态调遣还会阻止编译器内联某个方法代码的抉择，这就相应地阻止了一些优化。然而，咱们却真切地获得了，如同咱们在清单 17-5 中所编写的代码那样的灵活性，同时才能够支持清单 17-9 中那样的情况，如此其便是一种需要考量的取舍了。when we use trait objects, Rust must use dynamic dispatch. The compiler doesn't know all the types that might be used with the code that's using trait objects, so it doesn't know which method implemented on which type to call. Instead, at runtime, Rust uses the pointers inside the trait object to know which method to call. This lookup incurs a runtime cost that doesn't occur with static dispatch. Dynamic dispatch also prevents the compiler from choosing to inline a method's code, which in turn prevents some optimizations. However, we did get extra flexibility in the code that we wrote in Listing 17-5 and were able to support in Listing 17-9, so it's a trade-off to consider.


## 实现一种面向对象设计模式

**Implementing an Object-Oriented Design Pattern**

*状态模式，the state pattern* 属于一种面向对象设计模式。这种模式的核心，便是咱们要定义某个值在其内部可能有的一套各种状态。这些状态是由一套 *状态对象，state objects* 所表示的，同时该值的行为，会根据其状态而改变。接下来咱们会完成有着保存其状态字段，该字段将有着“草稿，draft”、“审阅，review” 或“已发布，published” 三种状态集合的状态对象，的一个博客帖子结构体示例。

状态对象共用着功能：当然，在 Rust 中咱们会使用结构体与特质，而非对象与继承。每个状态对象负责其自己的行为，以及在其应变换为另一状态时自身的治理。保存着状态对象的值，对这些状态的不同行为，或这些状态之间何时变换就毫不知情。

运用状态模式的优势在于，当程序的业务需求改变时，咱们将不需要修改该值保存状态的那些代码，也不需要修改用到该值的那些代码。咱们只需更新某个状态对象内部的那些代码，来改变其规则，或是添加别的一些状态对象。

首先，咱们将要以更传统的面向对象方式，实现这种状态模式，随后咱们将使用对于 Rust 中，更自然一些的方法。下面就来深入到使用状态模式，逐步实现一个博客帖子工作流。

最终功能看起来将像下面这样：

1. 博客帖子以一个空的草稿开始；

2. 在草稿写好后，该帖子就要求审阅一下；

3. 在帖子被批准后，其就会被发布；

4. 只有发布了的帖子，才会返回要打印的内容，因此那些未获批准的帖子就不会被无故发布。

所有别的在帖子上的尝试修改，都应无效。比如，在完成审阅之前，若咱们尝试批准博客帖子草稿，那么该帖子应保持为一个未发布的草稿。

下面清单 17-11 给出了代码形式的这个工作流：此为咱们将在一个名为 `simple_blog` 的库代码箱中，实现的这个 API 的一个示例用法。由于咱们尚未实现该 `simple_blog` 代码箱，因此这段代码尚不会编译。

文件名：`src/main.rs`

```rust
use simple_blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("今天午饭我吃了沙拉。");
    assert_eq! ("", post.content());

    post.request_review();
    assert_eq! ("", post.content());

    post.approve();
    assert_eq! ("今天午饭我吃了沙拉。", post.content());
}
```

*清单 17-11：验证咱们打算这个 `simple_blog` 代码箱要有的必要功能的代码*

咱们打算运行用户使用 `Post::new` 创建出一个新的帖子草稿。咱们打算允许将一些文本添加到博客帖子。当咱们在审批之前，尝试立即获取到帖子的内容时，由于该帖子仍为一个草稿，因此咱们就不应得到任何文本。出于验证目的，咱们已在该代码中添加了 `assert_eq!`。而为此目的的良好单元测试，就应断言帖子草稿会从那个 `content` 方法返回空字符串，而咱们并未打算为此示例编写一些测试。

接下来，咱们打算开启该帖子的审阅请求，同时咱们系统在等待审阅期间，`content` 返回一个空字符串。在该帖子得到审批时，他就应得以发布了，表示在 `content` 被调用时，该帖子的文本将被返回。

请注意咱们与 `simple_blog` 代码箱交互的唯一类型，便是 `Post` 这个类型。此类型将用到状态模式，并将保存着将为表示帖子可能状态 -- 草稿、等待审阅或已发布，的三个状态对象之一的一个值。从一种状态改变为另一状态，将在该 `Post` 类型里得以内部管理。这些状态，会因应着库用户在 `Post` 实例上的方法调用而改变，但库用户们却不必直接管理这些状态改变。同样，用户们是无法在这些状态上犯下错误的，比如在帖子未审阅前发布帖子。


### 定义出 `Post` 并创建出一个草稿状态的新实例

**Defining `Post` and Creating a New Instance in the Draft State**


下面就来开始这个库的实现！咱们清楚咱们需要保存着一些内容的一个公开的 `Post` 结构体，因此咱们将以这个结构体的定义，及创建出 `Post` 实例的一个关联的公开 `new` 函数开始，如下清单 17-12 中所示。咱们还将构造出将定义 `Post`的全部状态对象，所必须有的行为的一个私有 `State` 特质。

随后 `Post` 类型将在名为 `state` 的私有字段的 `Option<T>` 值内部，保存 `Box<dyn State>` 类型的一个特质对象，来保存状态对象。过一会儿，咱们就会看到为何那个 `Option<T>` 是必要的。

文件名：`src/lib.rs`

```rust
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
}

trait State {}

struct Draft {}

impl State for Draft {}
```

*清单 17-12：`Post` 结构体的定义，以及创建新 `Post` 实例的 `new` 函数、`State` 特质，以及 `Draft` 结构体*

其中的 `State` 特质会定义出由不同帖子状态共用的行为。这些状态对象分别是 `Draft`、`PendingReview` 及 `Published`，同时他们都将实现 `State` 特质。至于现在，这个特质并无任何方法，而由于 `Draft` 状态是咱们想要帖子开始的状态，因此咱们将以仅定义出这个状态开始。

在创建出新的 `Post` 实例时，咱们将其 `state` 自动设置为了保存着一个 `Box` 值的 `Some` 值。这个 `Box` 会只想 `Draft` 结构体的一个新实例。这会确保不能在咱们何时创建出一个 `Post` 的新实例，其都将作为一篇草稿开始。由于 `Post` 的 `state` 字段是私有的，因此就没有办法创建出其他任何状态的一个 `Post`！在 `Post::new` 函数中，咱们把 `content` 字段设置为了一个新的、空 `String`。


### 存储帖子内容文本

**Storing the Text of the Post Content**

在 17-11 中，咱们曾看到咱们希望能调用一个名为 `add_text` 的方法，并传递给他随后被作为博客帖子内容而添加的一个 `&str`。咱们将这实现为一个方法，而不是把 `content` 作为 `pub` 暴露出来，如此稍后咱们就可以实现一个将控制 `content` 字段数据如何被读取的方法。这个 `add_text` 方法是相当直截了当的，那么接下就来在清单 17-13 中，添加这个实现到 `impl Post` 代码块：

文件名：`src/lib.rs`

```rust
impl Post {
    // -- 跳过代码 --

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}
```

*清单 17-13：实现将文本添加到帖子 `content` 字段的 `add_text` 方法*

由于咱们是在修改咱们正于其上调用 `add_text` 的 `Post` 实例，因此这个 `add_text` 方法便取了到 `self` 的可变引用。咱们随后调用了 `content` 字段中 `String` 类型上的 `push_str`，并传递 `text` 参数来添加到那个已保存的 `content`。此行为不依赖帖子所处的状态，因此其并非状态模式的一部分。这个 `add_text` 方法完全不与 `state` 字段交互，但其为咱们打算支持行为的一部分。


### 确保帖子草稿的内容为空

**Ensuring the Content of a Draft Post Is Empty**

即使咱们已调用了 `add_text` 并把一些内容添加到了咱们的帖子，但由于该帖子仍处于草稿状态，故咱们仍想要那个 `content` 方法返回空字符串切片，an empty string slice，正如清单 17-11 中第 7 行所给出的那样。那么现在，就来用将满足此要求的最简单物件，实现这个 `content` 方法：即总是返回一个空字符串切片。稍后一旦咱们实现修改帖子状态的能力，从而帖子可被发布，咱们就会修改这个方法。到目前为止，贴子就只能处于草稿状态，因此帖子内容应始终为空。下面清单 17-14 给出了这种占位的实现：

文件名：`src/lib.rs`

```rust
impl Post {
    // -- 跳过代码 --

    pub fn content(&self) -> &str {
        ""
    }
}
```

*清单 17-14：添加始终返回空字符串切片的 `Post` 上 `content` 方法的一个占位实现，a placeholder implementation*

有了添加的这个 `content` 方法，清单 17-11 中到第 7 行为止的那些代码就都将如预期那样工作了。


### 请求帖子审阅改变其状态

**Requesting a Review of the Post Changes Its State**

接下来，咱们就需要添加请求帖子审阅的功能了，帖子审阅应将其状态从 `Draft` 改变为 `PendingReview`。下面清单 17-15 给出了这样的代码：


文件名：`src/lib.rs`

```rust
impl Post {
    // -- 跳过代码 --

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
```

*清单 17-15：实现 `Post` 与 `State` 特质上的 `request_review` 方法*

咱们给了 `Post` 名为 `request_review` 的一个公开方法，其将取到 `self` 的一个可变引用。随后咱们调用了 `Post` 当前状态上的内部 `request_review` 方法，而这第二个 `request_review` 方法就会消费当前状态并返回一个新的状态。

咱们把那个 `request_review` 方法添加到了 `State` 特质；所有实现了这个特质的类型，现在都将需要实现这个 `request_review` 方法。请注意这里用的不再是 `self`、`&self` 或 `&mut self` 作为该方法的首个参数，这里用的是 `self: Box<Self>`。这样的语法表示，在有在某个保存了该类型的 `Box` 上调用时，这个方法才是有效的。这种语法会取得 `Box<Self>` 的所有权，令到原有状态失效，进而 `Post` 的状态值就可以转换到一种新的状态。

为了消费原有状态，这个 `request_review` 方法就需要取得该状态值的所有权。这便是 `Post` 的那个 `state` 字段中的 `Option` 进入之处：咱们调用了 `take` 方法（属于标准库的 `Option` 类型），来从 `state` 字段取出那个 `Some` 的值，并由于 Rust 不允许咱们在结构体中有无效或空字段，Rust doesn't let us have unpopulated fields in structs，而在 `state` 字段中留下一个 `None`。这样就允许咱们把其中的 `state` 值，迁移出 `Post`而非借用他。随后咱们将把帖子的 `state` 值，设置为此操作的结果。

为了获取到 `state` 值的所有权，咱们就需要暂时将 `state` 设置为 `None`，而非直接使用像是 `self.state = self.state.request_review();` 这样的代码设置他。这样做确保了在咱们已将 `Post` 转换为新状态后，其无法使用原先的 `state` 值。

`Draft` 上的 `request_review` 方法返回的是个新的、新加入的 `PendingReview` 装箱过的实例，其表示了帖子等待审阅时的状态。那个 `PendingReview` 结构体同样实现了 `request_review` 方法，但并未进行任何转换。而是，由于在咱们于某个已处于 `PendingReview` 状态的帖子上，请求审阅时，帖子应保持处于 `PendingReview` 状态，因此 `PendingReview` 的 `request_review` 方法调用返回的是他自己。

现在咱们就可以开始看到状态模式的优势了：不论 `Post` 的 `state` 值为何，其上的 `request_review` 方法都是一样的。每种状态都负责着其自己的那些规则。

咱们将保留 `Post` 上的 `content` 方法如其现在这样，即返回一个空字符串切片。现在咱们就可以让某个 `Post`，处于 `PendingReview` 状态抑或 `Draft` 状态了，不过咱们想要 `PendingReview` 状态中的同样行为。现在清单 17-11 到第 10 行便工作了！


### 添加 `approve` 来修改 `content` 的行为

**Adding `approve` to Change the Behavior of `content`**

`approve` 方法将与 `request_review` 方法类似：他将把 `state` 设置为在帖子状态为 “批准” 时，当前状态所应表明的值，如下清单 17-16 中所示：

文件名：`src/lib.rs`

```rust
impl Post {
    // -- 跳过代码 --

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    // -- 跳过代码 --

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    // -- 跳过代码 --

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
```

*清单 17-16：实现 `Post` 与 `State` 特质上的 `approve` 方法*

咱们把这个 `approve` 方法，添加到了 `State` 特质，并添加了一个实现了 `State` 的新结构体，即 `Published` 状态。

与 `PendingReview` 上的 `request_review` 工作方式类似，在咱们调用 `Draft` 上的 `approve` 方法时，由于 `approve` 将返回 `self`，因此他将没有效果。当咱们在 `PendingReview` 上调用 `approve` 时，他返回的是一个新的、装箱过后的 `Published` 结构体实例。这个 `Published` 结构体实现了 `State` 特质，而由于帖子在`request_review` 及 `approve` 两个方法下，都应保持处于 `Published` 状态，因此对于这两个方法，他都会返回他本身。

现在咱们就需要更新 `Post` 上的那个 `content` 方法了。咱们希望从 `content` 返回的值，取决于 `Post` 的当前状态，因此咱们就让 `Post`，委托给定义在其 `state` 上的一个 `content` 方法，如下清单 17-17 中所示：

文件名：`src/lib.rs`

```rust
impl Post {
    // -- 跳过代码 --

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    // -- 跳过代码 --
}
```

*清单 17-17：将 `Post` 上的 `content` 方法，更新为委托给 `State` 上的 `content` 方法*

由于咱们的目标是要把所有规则，都保持于实现了 `State` 的那些结构体中，因此咱们就要调用 `state` 字段中值上的 `content` 方法，并将帖子实例（那就是 `self`）作为参数加以传递。随后咱们要返回从 `state` 值上的 `content` 方法调用所返回的值。

由于咱们要的是到 `Option<T>` 内部值的一个引用，而非该值的所有权，因此咱们调用了 `Option<T>` 上的 `as_ref` 方法。由于 `state` 是个 `Option<Box<dyn State>>`，在咱们调用 `as_ref` 时，就会返回一个 `Option<&Box<dyn State>>`。而若咱们没有调用 `as_ref`，那么由于咱们无法无法把 `state` 迁移出那个借用的函数参数 `&self`，而将得到一个报错。

咱们随后调用了 `unwrap` 方法（标准库 `Option<T>` 类型上的），由于咱们清楚，`Post` 上的那些方法，会确保 `state` 将在这些方法完成时，始终包含某个 `Some` 值，因此咱们就明白，这个`unwrap` 是绝不会终止运行的。这便是第 9 章 [相比与编译器咱们掌握着更多信息的情形](Ch09_Error_Handling.md#相比于编译器代码编写者掌握了更多信息的情形) 小节所讲到的情形之一：即咱们明白某个 `Option<T>` 不可能是个 `None` 值，尽管编译器无法掌握这一点。

到 `unwrap` 方法这里，当咱们在 `&Box<dyn State>` 上调用 `content` 方法时，强制解引用转换，deref coercion 就会在那个 `&` 及 `Box` 上发挥作用，从而 `content` 方法就将在实现了 `State` 特质的类型上，最终被调用到。而那就意味着咱们需要把 `content` 添加到 `State` 特质的定义，而那正是咱们把根据咱们所有的状态，返回什么样的内容，这种逻辑要放入的地方，如下清单 17-18 中所示：

文件名：`src/lib.rs`

```rust
trait State {
    // -- 跳过代码 --
    fn content<'a>(&self, post: &'a Post) -> &'a str { "" }
}

// -- 跳过代码 --
struct Published {}

impl State for Published {
    // -- 跳过代码 --

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
```

*清单 17-18：把 `content` 方法添加到 `State` 特质*

咱们添加了返回空字符串切片的 `content` 方法默认实现。那就意味着咱们无需在 `Draft` 与 `PendingRereview` 两个结构体上实现 `content` 方法。而 `Published` 结构体则将重写这个 `content` 方法，并返回 `post.content` 中的值。

> **注**：由于 `content` 默认实现返回的是 `""` 空字符串切片，是个已知大小的值，故方才可以写默认实现。而若将 `request_review` 或 `approve` 也写为默认实现，即如下面这样：

```rust
trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State> { self }
    fn approve(self: Box<Self>) -> Box<dyn State> { self }
    fn content<'a>(&self, post: &'a Post) -> &'a str { "" }
}
```
>
> 那么将报出错误：

```console
$ cargo run                                                                                     ✔  
   Compiling simple_blog v0.1.0 (/home/peng/rust-lang/simple_blog)
error[E0277]: the size for values of type `Self` cannot be known at compilation time
  --> src/lib.rs:40:53
   |
40 |     fn approve(self: Box<Self>) -> Box<dyn State> { self }
   |                                                     ^^^^ doesn't have a size known at compile-time
   |
   = note: required for the cast from `Self` to the object type `dyn State`
help: consider further restricting `Self`
   |
40 |     fn approve(self: Box<Self>) -> Box<dyn State> where Self: Sized { self }
   |                                                   +++++++++++++++++

For more information about this error, try `rustc --explain E0277`.
error: could not compile `simple_blog` due to previous error
```
>
> 这表示 Rust 中的默认实现，需要返回值为固定大小。

请注意正如咱们曾在第 10 章中讨论过的那样，在这个方法上咱们需要生命周期注解。咱们取了到某个 `post` 的引用作为参数，并返回的是到那个 `post` 一部分的引用，因此所返回引用的生命周期，便于这个 `post` 参数生命周期相关。

而咱们就完成了 -- 清单 17-11 的全部代码现在便工作了！咱们就已实现了博客帖子工作流规则，the rules of the blog post workflow 下的状态模式。与那些规则相关的逻辑，是存在与这些状态对象中，而非散落于 `Post` 的各处，the logic related to the rules lives in the state objects rather than being scattered throughout `Post`。

> 为何不用枚举，Why Not An Enum？
>
> 你可能已经想到，为何咱们没有使用将不同帖子状态作为变种的一个 `enum`。那确实是一种可行的办法，请尝试并比较最后的结果，来发现你要选哪个方案！运用枚举的一个不足之处，便是在每个检查枚举值的地方，将都需有一个 `match` 表达式，或类似的东西来处理每种可能的变种。相比这里的特质对象方法，那就会有更多重复代码。


### 状态模式的取舍

**Trade-offs of the State Pattern**

咱们已经证明，Rust 是能够实现这种面向对象模式，来封装处于不同状态下帖子应具备的各种不同行为。`Post` 上的方法对这些各种行为毫不知情。咱们组织代码的方式，即咱们必须仅在一处查看，而获悉某个已发布帖子可以有的那些不同行为方式：这便是 `Published` 结构体上 `State` 特质的实现。

若咱们原本打算创建另一种不使用状态模式的替代实现，那么咱们可能就会在 `Post` 上的那些方法中，使用一些检查帖子状态的 `match` 表达式，并在那些 `match` 表达式处改变行为。那就意味着咱们将不得不查看多个地方，来了解某个处于已发布状态帖子的全部影响！这样做只会徒增咱们所添加的更多一些状态：每个的这些 `match` 表达式，都将需要另一支臂。

而在状态模式下，那些 `Post` 方法以及那些咱们用到 `Post` 的各处，就不需要那些 `match` 表达式，而要添加一个新状态，咱们将只需添加一个新结构体，并在那个结构体上实现那些特质方法即可。

使用状态模式的这种实现，易于添加更多功能。为发现使用状态模式维护代码的简单性，请尝试下面几条建议：

- 请添加将帖子状态从 `PendingReview` 改回到 `Draft` 的一个 `reject` 方法；

- 在状态可被改变为 `Published` 之前，要求两次到 `approve` 的调用；

- 只有在某个帖子处于 `Draft` 状态时，才允许用户添加文本内容。提示：让状态对象负责那些可能修改内容的操作，而不负责修改 `Post` 的操作。


状态模式的一个缺点则是，由于这些状态都实现那些状态间的转换，那么其中一些状态就相互耦合了。当咱们在 `PendingReview` 于 `Published` 之间，添加另一状态，比如 `Scheduled` 时，咱们将不得不把 `PendingReview` 中的代码，修改为相应地转换到 `Scheduled`。若在新状态的添加下，`PendingReview` 无需修改，那么就会少一些事情，然而那便意味着转换到另一种涉及模式了。

至于另一个缺点，便是咱们重复了一些逻辑。为消除一些重复，咱们就可能会尝试构造 `State` 特质上，返回 `self` 的 `request_review` 于 `approve` 两个方法的默认实现；然而，由于该特质不清楚那个具体的 `self` 将为何物，因此这会违反对象安全性，violate object safety。咱们希望能够将 `State` 作为特质对象使用，因此咱们就需要他的那些方法是对象安全的。

其他代码重复包括了 `Post` 上 `request_review` 与 `approve` 两个方法的一些相似实现。这两个方法都委托给了那个 `Option` 的 `state` 字段中值上的同一方法，并将 `state` 字段的值，设置到方法的结果。若咱们在 `Post` 上有着大量的遵循这种模式的方法，咱们就会考虑定义出一个宏，defining a macro，来消除这种重复（请参阅第 19 章中 ["宏，Macros"](Ch19_Advanced_Features.md#关于宏) 小节）。

经由这种完全按照面向对象模式下所定义的状态模式，来实现这种模式，咱们就没有利用上原本所能利用的 Rust 的全部优势。下面就来看看，咱们可对那个 `simple_blog` 能做出的，可将无效状态与无效状态转换，构造为编译时错误的一些改变。


#### 将状态与行为当作类型编码

**Encoding States and Behavior as Types**


咱们将给出如何对这种状态模式加以反思，以得到一套不同的权衡取舍。不同于对状态及状态的转换进行完全地封装，进而外部代码对他们一无所知，咱们将把那些状态编码为不同类型。于是乎，Rust 的类型检查系统，就将通过发出编译器报错，阻止在那些仅允许已发布帖子之处，使用草稿帖子的尝试。

下面来考虑一下清单 17-11 中，`main` 函数的第一部分：

文件名：`src/main.rs`

```rust
fn main() {
    let mut post = Post::new();

    post.add_text("今天午饭我吃了沙拉。");
    assert_eq! ("", post.content());
}
```

咱们仍旧使用 `Post::new`，实现了新的处于草稿状态的那些帖子的创建，并实现了将文本添加到帖子内容的能力。但与在草稿帖子上有着返回空字符串的 `content` 方法不同，咱们将把 `Post` 构造为根本就没有那个 `content` 方法。那样的话，在咱们尝试获取某个草稿帖子的内容时，就会得到告诉咱们该方法不存在的编译器报错。由此，对于生产中咱们无意地显示出帖子内容，由于那样的代码甚至都不会编译，那么这将是不可能的了。清单 17-19 给出了 `Post` 结构体的定义，以及一个 `DraftPost` 的结构体，以及各自上的一些方法：


文件名：`src/lib.rs`

```rust
pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}
```

*清单 17-19：有着 `content` 方法的 `Post` 与不带 `content` 方法的 `DraftPost`*


`Post` 与 `DraftPost` 两个结构体都有着存储了博客帖子文本的私有 `content` 字段。由于咱们正将状态编码，迁移到一些结构体类型，因此这两个结构体就不再有 `state` 字段了。`Post` 结构体将表示已发布帖子，而他便有着返回 `content` 的 `content` 方法。

咱们仍有一个 `Post::new` 函数，但不是返回 `Post` 实例，其返回的是 `DraftPost` 实例。由于 `content` 是私有的，而有没有任何返回 `Post` 的函数，那么此刻就不可能创建出 `Post` 实例。

`DraftPost` 结构体有着一个 `add_text` 方法，因此咱们就可以如同之前那样，把文本添加到 `content`，但请注意 `DraftPost` 并没有定义一个 `content` 方法！因此现在的程序确保了全部帖子都以草稿帖子开头，而草稿帖子并不会让他们的内容用于显示。任何绕过这些约束的尝试，都将导致编译器报错。


#### 以到不同类型的转换，实现（状态的）转换

**Implementing Transitions as Transformations into Different Types**


那么怎样来获取到某个已发布帖子呢？咱们是打算强化某个草稿帖子在其可被发布之前，必须被审阅和批准的规则。处于等待审阅状态的帖子，应仍然不显示任何内容。下面酒类通过添加另一结构体，`PendingReviewPost`、在 `DraftPost` 上定义出返回 `PendingReviewPost` 实例的 `request_review` 方法，以及在 `PendingReviewPost` 上定义出返回 `Post` 的 `approve` 方法，实现这些约束，如下清单 17-20 中所示：

文件名：`src/lib.rs`

```rust
impl DraftPost {
    // -- 跳过代码 --

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}
```

*清单 17-20：通过在 `DraftPost` 上调用 `request_review` 而被创建出的 `PendingReviewPost` 实例，以及将 `PendingReviewPost` 转变为已发布 `Post` 的 `approve` 方法*

`request_review` 与 `approve` 两个方法，都取得了 `self` 的所有权，从而消费了 `DraftPost` 及 `PendingReviewPost` 实例，并把他们相应地转换成了 `PendingReviewPost` 与已发布的 `Post`。以这种方式，在咱们于 `DraftPost` 实例上调用了 `request_review` 之后，便不会再有任何遗存的 `DraftPost` 实例，对 `PendingReviewPost` 之类亦是如此。`PendingReviewPost` 结构体之上，并没有 `content` 方法，因此正如 `DraftPost` 一样，尝试读取其内容，会导致编译器报错。由于获取确实有定义出的 `content` 方法的已发布 `Post` 实例的唯一方式，为在某个 `PendingReviewPost` 上调用 `approve` 方法，而获取到一个 `PendingReviewPost` 的唯一方法，为在某个 `DraftPost` 上调用 `request_review` 方法，咱们现在便已将这个博客帖子工作流，编码为了类型系统。

不过咱们还必须对 `main` 做出一些小的修改。`request_review` 与 `approve` 两个方法，返回的都是一些新实例，而不再是修改他们于其上所调用的结构，因此咱们就需要添加更多 `let post = ` 遮蔽赋值语句，来保存那些返回的实例。咱们还不能断言草稿于等待审阅帖子的内容为空字符串，咱们也是不需要他们的：咱们再也不会编译，尝试使用处于这些状态下的帖子内容的代码。下面清单 17-21 中，给出了 `main` 中更新后的代码：

文件名：`src/main.rs`

```rust
use neo_simple_blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("这是一个博客帖子。");

    let post = post.request_review();
    let post = post.approve();

    assert_eq! ("这是一个博客帖子。", post.content());
}
```

*清单 17-21：为使用这个博客帖子工作流的新实现，而对 `main` 的一些修改*


咱们所需做出的对 `post` 重新复制的那些修改，表示这种实现，已不再那么严格遵循面向对象设计模式了：状态之间的转换，不再是整个地封装在 `Post` 实现里。不过，咱们的收获，则是由于类型系统，以及编译时所发生的类型检查，那些无效状态现在就不可能了！这样就确保了一些确切代码错误，比如未发布帖子内容的显示等，在其到达生产部署之前，就将被发现。

请在清单 17-21 之后的情况下，尝试在 `neo_simple_blog` 上实现本小节开头给出的那些任务，来发现你对此版本代码的设计模式有何看法。请注意其中一些任务，在这种模式下或许已被完成了。


咱们业已看到，即使 Rust 有能力实现面向对象的一些设计模式，而对于其他模式，比如将状态编码为类型系统等，在 Rust 中也都是可行的。这些模式都有着不同的取舍。尽管咱们可能对面向对象的那些模式非常熟悉，但对问题进行反思，而运用上 Rust 那些特性的优势，就可以提供到各种好处，比如在编译时阻止一些代码错误等。出于比如所有权这样的，面向对象语言所不具备的某些特性，那么在 Rust 中，面向对象的那些模式，将并不总是最佳方案。


## 本章小节

在读完这一章之后，不论咱们认为或是不认为 Rust 是门面向对象语言，现在都明白，咱们可以在 Rust 中，使用特质对象来获得一些面向对象的特性。动态调遣，dynamic dispatch 可以些许运行时性能损失，换取到咱们代码一定程度的灵活性。咱们则可运用这样的灵活性，来实现能有助于代码可维护性的一些面向对象模式。Rust 还有面向对象语言所没有的其他一些特性，比如所有权等。对于利用 Rust 各种长处方面的优势来讲，面向对象模式将不总是最佳方式，但其为一种可行选项。

接下来，咱们将看看各种模式，这是带来大量灵活性的 Rust 诸多特性的另一项。虽然贯穿本书，咱们已经粗略地看了看他们，但仍尚未见识到他们的完整能力。咱们就拭目以待吧！
