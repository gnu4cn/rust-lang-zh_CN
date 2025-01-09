# 面向对象语言的特征

**Characteristics of Object-Oriented Languages**


在编程界，并无关于某门被视为面向对象的，而必须具有哪些特性的共识。Rust 受了许多编程范式，programming paradigms，的影响，其中就包括 OOP；比如在第 13 章中，咱们就曾探讨过，那些来自于函数式编程的特性。可以说，那些 OOP 的语言，确实是共用了一些确切的特征的，那即是对象、封装与继承等。下面就来看看这些特征各自指的是什么，以及 Rust 是否支持他们。


## 对象包含了数据及行为

**Objects Contain Data and Behavior**


Erich Gamma、Richard Helm、Ralph Johnson 及 John Vlissides 等的合著 *Design Patterns: Elements of Reusable Object-Oriented Software* （Addison-Wesley Professional, 1994），又被通俗地叫做 *The Gang of Four* 书，便是面向对象设计模式的一个目录。该书像下面这样定义了 OOP：

> 面向对象程序是由对象所组成的。*对象，an object* 同时打包了数据与运行在那数据上的过程。这些过程一般就叫做 *方法，methods* 或 *操作，operations*。

运用这个定义，Rust 便是面向对象的：结构体与枚举均有着数据，而 `impl` 块则提供了结构体与枚举上的那些方法。即使有着方法的那些结构体与枚举未*被称作* 对象，根据 The Gang of Four 的对象定义，他们提供了同样的功能。


## 隐藏了实现细节的封装

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


## 以类型系统及以代码共用的继承

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


（End）


