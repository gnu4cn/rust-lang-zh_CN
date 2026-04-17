# 面向对象语言的特征

在编程界，对于某门语言必需具备哪些特征才能被视为面向对象并无定论。Rust 受了许多编程范式，programming paradigms，的影响，包括 OOP；例如，我们在第 13 章中探讨了来自函数式编程的特性。可以说，OOP 语言共用一些共同特征 -- 即对象、封装和继承。我们来看看这些特征各自的含义，以及 Rust 是否支持他们。


## 对象包含数据和行为

Erich Gamma、Richard Helm、Ralph Johnson 及 John Vlissides 等的合著 *[Design Patterns: Elements of Reusable Object-Oriented Software](https://en.wikipedia.org/wiki/Design_Patterns)* （Addison-Wesley Professional, 1994），又被通俗地叫做 *The Gang of Four* 书，是面向对象设计模式的汇编。该书是这样定义 OOP 的：

> “面向对象的程序对象所组成。**对象** 封装了数据和操作该数据的过程。这些过程通常称为 **方法** 或 **操作**”。

使用这个定义，那么 Rust 是面向对象的：结构体和枚举包含数据，而 `impl` 代码块提供结构体和枚举上的方法。尽管带有方法的结构体与枚举未 *被称作* 对象，但根据 The Gang of Four 对对象的定义，他们提供了同样的功能。


## 隐藏实现细节的封装

通常与 OOP 相关的另一方面是，*封装，encapsulation* 的思想，这意味着使用对象的代码无法访问对象的实现细节。因此，与对象交互的唯一方式是，通过对象的公开 API；使用对象的代码不应能够深入对象内部，并直接更改数据或行为。这使得程序员能够修改和重构对象的内部结构，而无需更改使用对象的代码。

我们在第 7 章中讨论过怎样控制封装：我们可以使用 `pub` 关键字来决定代码中哪些模组、类型、函数与方法等应为公开的，默认情况下其他所有项目都是私有的。例如，我们可以定义一个 `AveragedCollection` 结构体，有着一个包含 `i32` 值矢量的字段。该结构体还可以有一个包含矢量中值的平均数的字段，这意味着表示在任何人需要时都不必按需计算平均值。换句话说，`AveragedCollection` 将为我们缓存计算出的平均值。下面清单 18-1 提供了 `AveragedCollection` 结构体的定义。

<a name="listing_18-1"></a>
文件名：`src/lib.rs`

```rust
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}
```

**清单 18-1**：`AveragedCollection` 结构体，维护一个整数列表和集合中项目的平均数

该结构体被标记为 `pub`，以便其他代码可以使用他，但结构体内的字段仍保持私有。在这一情形下这一点很重要，因为我们希望确保每当有值添加到列表或从列表中移除时，平均数也会随之更新。我们通过在该结构体上实现 `add`、`remove` 和 `average` 方法来实现这一点，如下清单 18-2 中所示。

<a name="listing_18-2"></a>
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

**清单 18-2**：`AveragedCollection` 上的公开方法 `add`、`remove` 与 `average` 的实现

`add`、`remove` 与 `average` 这三个公开方法 ，是访问或修改 `AveragedCollection` 实例中数据的唯一方式。当使用 `add` 方法添加项目到 `list`，或使用 `remove` 方法移除项目时，两个方法调用的实现还会调用私有的`update_average` 方法，其会负责更新 `average`字段。

我们把 `list` 和 `average` 字段保留为私有，以便外部代码无法直接向 `list` 字段添加项目或从中移除项目；否则，当 `list` 发生变化时，`average` 字段就会失去同步。`average` 方法返回 `average` 字段中的值，允许外部代码读取 `average` 但不能修改他。

由于我们已经封装了结构体 `AveragedCollection` 的实现细节，因此将来可以轻松地修改数据结构等方面。例如，我们可以对 `list` 字段使用 `HashSet<i32>` 而非 `Vec<i32>`。只要 `add`、`remove` 及 `average` 三个公开方法的签名保持不变，使用 `AveragedCollection` 的代码就无需修改。若我们转而构造 `list` 为公开，情况就未必如此：`HashSet<i32>` 与 `Vec<i32>` 有着添加和移除项目的不同方法，因此如果外部代码直接修改 `list`，很可能就必须修改。

当封装是某门语言被视为面向对象的必要条件，那么 Rust 满足这一要求。对代码的不同部分使用或不使用 `pub` 的选项，实现了对实现细节的封装。


## 作为类型系统和代码共用的继承

*继承，inheritance* 属于一种机制，一个对象可以从另一对象的定义继承元素，从而获得父对象的数据和行为，而无需再次定义他们。

若某门语言必须具备继承，才能被称为面向对象语言，那么 Rust 就不是这样的语言。在不使用宏的情况下，就无法定义继承父结构体的字段和方法实现的结构体。

不过，若咱们于在编程工具箱中使用继承，那么在 Rust 中也可以采用其他解决方案，具体取决于咱们最初选择继承的原因。

咱们选择继承出于两个主要原因。一是为了代码的重用：咱们可以为一种类型实现特定的行为，而继承使咱们可以针对不同的类型重用该实现。咱们可以使用默认特质方法实现来有限地实现这一点，正如咱们在 [清单 10-14](../generic_types_traits_and_lifetimes/traits.md#listing_10-14) 中所见，当时我们对 `Summary` 特质添加了 `summarize` 方法的默认实现在。任何实现 `Summary` 特质的类型，都将于其上有着可用的 `summarize` 方法，而无需更多代码。这类似于父类有着某个方法的实现，继承子类也会有着该方法的实现。当我们实现 `Summary` 特质时，也可以重写 `summarize` 方法的默认实现，这类似于子类重写从父类继承的方法实现。

使用继承的另一个原因与类型系统相关：使子类型可以在父类型相同的位置使用。这也称为 *多态，polymorphism*，这意味着当多个对象共用某些特征时，咱们可以在运行时相互替换多个对象。

> 关于 **多态**
>
> 对许多人来说，多态与继承同义。但他实际上是个更普遍的概念，指的是可以处理多种数据类型的代码。在继承中，这些类型通常属于子类。
>
> Rust 则使用泛型来抽象各种可能的类型，并使用特质边界对这些类型必须提供的内容施加约束。这有时被称为 *有界参数多态性，bounded parametric polymorphism*。

Rust 通过不提供继承，做出了不同的权衡取舍。继承往往存在共用过多代码的风险。子类并不总是应该共用父类的所有特征，但在继承下却要这样做。这会降低程序设计的灵活性。由于某些方法并不适用于子类，他还引入了调用子类上不合理的方法，或引发错误的方法的可能性。此外，一些语言仅支持 *单一继承*（即子类只能继承自一个类），这进一步限制了程序设计的灵活性。

出于这些原因，Rust 采取了不同的方法，即使用特质对象而不是继承，来实现运行时的多态性。我们来看看特质对象的工作原理。



