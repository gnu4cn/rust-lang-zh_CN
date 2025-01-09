# 高级特质

在第 10 章 [“特质：定义共用行为”](Ch10_Generic_Types_Traits_and_Lifetimes.md#特质定义共用行为) 小节中，咱们曾首先涉及到特质，但咱们不曾讨论更为高级的那些细节。现在咱们对 Rust 有了更多了解，咱们就可以深入本质，get into the nitty-gritty。


## 使用关联类型指定出特质定义中的一些占位性类型

**Specifying placeholder types in trait definitions with associated types**


*关联类型* 将类型占位符与特质加以结合，从而那些特质方法的定义，就可以在他们的签名中，使用这些占位符类型。特质的实现者，将为其特定实现，指明占位符类型所要使用的具体类型。如此一来，咱们便可以在特质被实现之前，无需准确获悉特质用到的类型下，定义出用到这些类型的特质。

在本章中，咱们已经介绍了绝大多数极少需要用到的高级特性。而关联类型则是位于这些高级特性中部的一种：其相较本书其余部分降到的那些特性，用得尤其少见，但相较这一章中讨论到的其他特性，则其要更常用一些。

带有关联类型特质的一个示例，便是标准库所提供的 `Iterator` 特质。其中的关联类型名为 `Item`，且代表着实现了这个 `Iterator` 特质的类型所迭代的那些值的类型。`Iterator` 特质的定义如下清单 19-12 中所示。


```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

*清单 19-12：有着关联类型 `Item` 的 `Iterator` 特质的定义*

其中的类型 `Item` 便是个占位符，而那个 `next` 方法的定义，则显示其将返回类型类型为 `Option<Self::Item>` 的值。`Iterator` 的实现者，将指明 `Item` 的具体类型，同时 `next` 方法将返回包含那个具体类型值的一个 `Option`。

从泛型允许咱们在不指明函数可处理何种类型下，而定义出某个函数上看，关联类型可能看起来是个与泛型类似类似的概念。为检视这两个概念的不同，咱们将看看在指定了 `Item` 类型为 `u32` 的一个名为 `Counter` 的类型上的 `Iterator` 实现：

```rust
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --跳过代码--
```

这种语法似乎可与泛型的那种语法相比。那么为何没有只使用泛型定义 `Iterator`，如下清单 19-13 中所示的那样呢？

```rust
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}
```

*清单 19-13：使用泛型的一种 `Iterator` 特质的定义假设*

不同之处在于，如同清单 19-13 中使用泛型时，咱们必须注解每个实现中的那些类型；由于咱们还可以实现 `Iterfator<String> for Counter` 或任何其他类型，咱们可以有对 `Counter` 的多个 `Iterator` 实现。也就是说，在特质有着泛型参数时，他就可以对某个类型被实现多次，每次都修改泛型参数的具体类型。当在 `Counter` 上使用 `next` 方法时，咱们就将不得不提供类型注解，来表明咱们想要使用哪个 `Iterator` 实现。

而在关联类型下，由于我们无法在一个类型上多次实现某个特质，因此就无需注解类型。在上面有着用到关联类型定义的清单 9-12 中，由于只能有一个 `impl Iterator for Counter`，所以咱们就只能就`Item` 为何选择一次。咱们不必在 `Counter` 上调用 `next` 的每个地方，指定咱们所要的是个 `u32` 值的迭代器。


关联类型还成了特质合约的一部分：特质的实现着必须提供一种类型，来顶替那个关联类型占位符。关联类型通常会有个描述该类型将被如何使用的名字，而在 API 文档中对关联类型编写文档，则是良好的做法。


## 默认泛型参数与运算符的重载

**Default Generic Type Parameters and Operator Overloading**


> **注**：请参考 [Difference Between Method Overloading and Method Overriding in Java](https://www.geeksforgeeks.org/difference-between-method-overloading-and-method-overriding-in-java/) 了解 Java 中的重载与重写区别。

在咱们用到泛型参数时，咱们可以给泛型指定默认具体类型。在所指定的默认类型就有效时，这样做消除了实现者指定具体类型的需求。在声明泛型时使用 `<PlaceholderType=ConcreteType>` 语法，指定出默认类型。


这种技巧有用处情形的一个了不起示例，便是 *运算符重载，operator overloading*，咱们可以其在某些情形下，定制某个运算符（比如 `+`）的行为。


Rust 不允许咱们创建自己的运算符，或重载任意运算符。但咱们可以通过实现与运算符相关的特质，而重载那些运算及于 `std::ops` 中所列出的相应特质。比如，在下面清单 19-14 中，咱们就将 `+` 运算符过载为把两个 `Point` 实例加在一起。咱们是通过在 `Point` 结构体上实现 `Add` 特质完成这一点的。


文件名：`src/main.rs`

```rust
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq! (
        Point { x: 1, y: 0 } + Point { x: 2, y: 3},
        Point { x: 3, y: 3 }
    );
}
```

*清单 19-14：实现 `Add` 特质来为 `Point` 实例过载 `+` 运算符*

这里的 `add` 方法，将两个 `Point` 实例的 `x` 值及两个实例的 `y` 值相加，而创建出一个新的 `Point`。这里的 `Add` 特质有着一个确定自其中的 `add` 方法返回类型，名为的 `Output` 关联类型。

此代码中的默认泛型，是在 `Add` 特质里。以下便是其定义：


```rust
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
```

此代码看起来应相当熟悉：有着一个方法与关联类型的特质。其中新的部分为 `Rhs=Self`：这种语法叫做 *默认类型参数，default type parameters*。其中的 `Rhs` 泛型参数（是 `right hand side` 的缩写），定义了 `add` 方法中 `rhs` 参数的类型，当咱们实现这个 `Add` 特质，而没有指明 `Rhs` 的类型时，`Rhs` 的类型将默认为 `Self`，其将是咱们在其上实现 `Add` 的类型。

当咱们为 `Point` 实现 `Add` 时，由于咱们打算把两个 `Point` 实例相加，因此而使用了 `Rhs` 的默认值。接下来看看，其中咱们打算定制那个 `Rhs` 而非使用其默认值的一个 `Add` 实现示例。


咱们有着两个结构体，`Millimeters` 与 `Meters`，保存着不同单位的一些值。这种将某个既有类型，封装在另一结构体的瘦封装，thin wrapping，就叫做 *新类型模式，newtype pattern*，在后面的 [“使用新型模式在外部类型上实现外部特质”](#使用新型模式在外层类型上实现外层的特质) 小节，咱们会对其进行更深入讨论。咱们打算把毫米值与以米计数的值相加，并要让 `Add` 的实现，正确完成单位转换。咱们可在将 `Meters` 作为 `Rhs` 下，对 `Millimeters` 实现 `Add`，如下清单 19-15 中所示。


```rust
#[derive(Debug, Copy, Clone, PartialEq)]
struct Millimeters(u32);

#[derive(Debug, Copy, Clone, PartialEq)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
```

*清单 19-15：在 `Millimeters` 上实现 `Add` 特质，以将 `Millimeters` 与 `Meters` 相加*

为了将 `Millimeters` 与 `Meters` 相加，咱们指明了 `impl Add<Meters>` 来设置那个 `Rhs` 类型参数，而非使用其默认的 `Self`。

咱们将以如下两种主要方式，使用默认的类型参数：

- 在不破坏既有代码之下，扩展某个类型；
- 为实现绝大多数不会需要的特定情形下的定制，to allow customization in specific cases most users won't need。


标准库的 `Add` 特质，便是第二种目的的一个示例：通常，咱们将把两个相似类型相加，但 `Add` 特质提供了定制超出那种情况的能力。在 `Add` 特质中使用默认类型，就意味着咱们不必在多数时候指定额外的参数。换句话说，并不需要一点点的实现样板，从而令到使用这个特质更为容易。

第一个目的与第二个类似，不过是反过来的：在咱们打算将类型参数添加到某个既有特质时，就可以给到其一个默认值，从而在不破坏既有那些实现代码下，实现该特质功能的扩展。


## 用于消除歧义的完全合格语法：以同一名字调用方法

**Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name**


Rust 中没有什么可以阻止某个特质有着，与另一特质的方法同样名字的方法，Rust 也不会阻止咱们在一个类型上实现这两种特质。至于直接在类型上，以来自不同特质方法的同样名字实现方法，也是可行的。

在以同一名字调用这些方法时，咱们将需要告诉 Rust 打算使用哪一个。设想下面清单 19-16 中，定义了两个特质，`Pilot` 与 `Wizard`，两个特质都有一个叫做 `fly` 的代码。咱们随后在已在其上实现了一个名为 `fly` 方法的类型 `Human` 上，实现了这两个特质。每个 `fly` 都完成不同的事情。

```rust
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println! ("机长在此发言。");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println! ("飞起来！");
    }
}

impl Human {
    fn fly(&self) {
        println! ("*愤怒地挥动双臂*");
    }
}
```

*清单 19-16：两个被定义作有 `fly` 方法的特质并都在 `Human` 类型上被实现，且在 `Human` 上直接实现了一个 `fly` 方法*

当咱们在 `Human` 实例上调用 `fly` 时，编译器默认为调用直接在该类型上实现的那个方法，如下清单 19-17 中所示。

```rust
fn main() {
    let person = Human;
    person.fly();
}
```

*清单 19-17：调用 `Human` 实例上的 `fly`*

运行此代码将打印出 `*愤怒地挥动双臂*`，显示 Rust 调用了直接在 `Human` 上实现的那个 `fly` 方法。

为了调用 `Pilot` 或 `Wizard` 特质上的 `fly` 方法，咱们需要使用更为显式的语法，来指明我们所指的是那个 `fly` 方法。下面清单 19-18 对此语法进行了演示。

文件名：`src/main.rs`

```rust
fn main() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}
```

*清单 19-18：指明咱们打算调用哪个特质的 `fly` 方法*


在方法名字前指明特质名字，就向 Rust 澄清了咱们打算调用 `fly` 的哪个实现。咱们本来也可以写下 `Human::fly(&person)`，这与咱们曾在清单 19-18 中所使用的 `person.fly()` 等级，但若咱们无需消除歧义，这样写起来就些许有些长了。


运行此代码会打印以下输出：


```console
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/disambiguation`
机长在此发言。
飞起来！
*愤怒地挥动双臂*
```

由于 `fly` 方法取了一个 `self` 参数，那么当咱们有着实现了一个 *特质* 的两个 *类型* 时，Rust 就可以根据 `self` 的类型，找出要使用特质的哪个实现。

然而，不是方法的那些关联函数，是没有 `self` 参数的。当存在以同样函数名字，定义了非方法函数的类型或特质时，除非咱们使用了 *完全合格语法，fully qualified syntax*，否则 Rust 就不会总是清楚咱们所指的是何种类型。比如，在下面清单 19-19 中，咱们创建了一个用于动物收容所的特质，其中打算将所有狗崽都命名为 `点点`。咱们构造了带有关联的非方法函数 `baby_name` 的一个 `Animal` 特质。对结构体 `Dog` 实现了这个 `Animal` 特质，在 `Dog` 上咱们还直接提供了一个关联的非方法函数 `baby_name`。

```rust
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("点点")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("Puppy")
    }
}

fn main() {
    println! ("狗崽叫做 {}", Dog::baby_name());
}
```

*清单 19-19：有着一个关联函数的特质以及一个有着同样函数名字关联函数、还实现了那个特质的类型*

咱们是在那个定义在 `Dog` 上的关联函数里，实现的将全部狗仔命名为点点的代码。`Dog` 类型还实现了特质 `Animal`，该特质描述了全部动物都有的特征。小狗都叫做狗崽，且这一点是在 `Dog` 上的 `Animal` 特质中，与 `Animal` 特质关联的 `baby_name` 函数中得以表达的。

在 `main` 函数中，咱们调用了那个 `Dog::baby_name` 函数，这就会调用直接定义在 `Dog` 上的那个关联函数。此代码会打印下面的输出：

```console
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/disambiguation`
狗崽叫做 点点
```

此输出不是咱们想要的。咱们想要调用作为咱们曾在 `Dog` 上实现过的 `Animal` 特质一部分的那个 `baby_name` 函数，从而代码会打印出 `小狗叫做 狗崽`。咱们曾在清单 19-18 中用到的指定特质名字的技巧，这里就不管用了；而若咱们将 `main` 修改为下面清单 19-20 中的代码，咱们就将收到一个编译报错。

```rust
fn main() {
    println! ("小狗叫做 {}", Animal::baby_name());
}
```

*清单 19-20：尝试调用 `Animal` 特质中的那个 `baby_name` 函数，但 Rust 不清楚要使用那个实现*

由于 `Animal::baby_name` 没有 `self` 参数，且这里可能有别的实现了 `Animal` 特质的类型，因此 Rust 就无法计算出咱们想要的那个 `Animal::baby_name` 实现。咱们将得到下面这个编译器错误：


```console
$ cargo run
   Compiling disambiguation v0.1.0 (/home/lenny.peng/rust-lang/disambiguation)
error[E0790]: cannot call associated function on trait without specifying the corresponding `impl` type
  --> src/main.rs:20:26
   |
2  |     fn baby_name() -> String;
   |     ------------------------- `Animal::baby_name` defined here
...
20 |     println! ("小狗叫做 {}", Animal::baby_name());
   |                              ^^^^^^^^^^^^^^^^^ cannot call associated function of trait
   |
help: use the fully-qualified path to the only available implementation
   |
20 |     println! ("小狗叫做 {}", <Dog as Animal>::baby_name());
   |                              +++++++       +

For more information about this error, try `rustc --explain E0790`.
error: could not compile `disambiguation` due to previous error
```

为消除歧义并告知 Rust 咱们打算使用 `Dog` 的那个 `Animal` 实现，而非某种其他类型的 `Animal` 实现，咱们需要使用完全合格语法。下面清单 19-21 演示了怎样使用完全合格语法。

文件名：`src/main.rs`

```rust
fn main() {
    println! ("小狗叫做 {}", <Dog as Animal>::baby_name());
}
```

*清单 19-21：使用完全合格语法来指明，咱们是要调用实现在 `Dog` 上的 `Animal` 特质中的那个 `baby_name` 函数*

通过讲出咱们希望将 `Dog` 类型，针对这个 `baby_name` 函数调用而作为 `Animal` 对待，从而表明咱们打算调用实现在 `Dog` 上的 `Animal` 特质中的 `baby_name` 方法，这样位处那尖括号中的类型注解，提供给 Rust。此代码现在将打印出咱们想要的输出：


```console
$ cargo run
   Compiling disambiguation v0.1.0 (/home/lenny.peng/rust-lang/disambiguation)
    Finished dev [unoptimized + debuginfo] target(s) in 0.18s
     Running `target/debug/disambiguation`
小狗叫做 狗崽
```

一般来讲，完全合格语法是像下面这样定义的：


```rust
<Type as Trait>::function(receiver_if_method, next_arg, ...);
```

对于那些不是方法的语法，此处就不会有 `receiver`：这里将只有其他参数的清单。在调用函数或方法的所有地方，咱们都可以使用完全合格语法。不过，在 Rust 能够从程序中另外的信息计算出（要调用哪个函数或方法）时，那么这种语法便是可以省略的。咱们只需在有着多个使用了同一名字的实现，且 Rust 需要帮助来识别出咱们打算调用哪个实现时，才需要使用这种更为冗长的语法。


## 在一个特质里运用超特质寻求另一特质的功能

**Using Supertraits to Require One Trait's Functionality Within Another Trait**


有的时候，咱们可能会编写依赖于另一特质的特质：对于要实现前一个特质的类型，咱们希望寻求那个类型也实现后一个特质。为了咱们的特质定义，可以利用后一个特质的那些关联项目，咱们就会实现这一点。咱们的特质所依赖的那个特质，被称为咱们特质的 *超特质，supertrait*。

比方说，咱们打算构造一个带有将所给的值格式化，从而其被星号框起来的 `outline_print` 方法，这样一个 `OutlinePrint` 特质。而那个所给的值则是，一个实现了标准库特质 `Display` 来得到 `(x, y)` 的 `Point` 结构体，即当咱们在有着 `x` 为 `1` `y` 为 `3` 的 `Point` 上调用 `outline_print` 时，其将打印以下输出：


```console
**********
*        *
* (1, 3) *
*        *
**********
```

在 `outline_print` 方法的实现中，咱们打算使用 `Display` 特质的功能。因此，咱们就需要指明，这个 `OutlinePrint` 特质将只对那些同时实现了 `Display` 生效，且提供了 `OutlinePrint` 所需的功能。咱们可以通过指明 `OutlinePrint: Display`，在该特质定义中实现那一点。这种技巧类似于给特质添加特质边界。下面清单 19-22 给出了这个 `OutlinePrint` 特质的一种实现。


```rust
use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();

        println! ("{}", "*".repeat(len + 4));
        println! ("*{}*", " ".repeat(len + 2));
        println! ("* {} *", output);
        println! ("*{}*", " ".repeat(len + 2));
        println! ("{}", "*".repeat(len + 4));
    }
}
```

*清单 19-22：需要 `Display` 中功能的 `OutlinePrint` 特质实现*

由于咱们已指明 `OutlinePrint` 需要 `Display` 特质，因此咱们就可以使用那个任何实现了 `Display` 类型上均已实现了的 `to_string` 函数。若咱们在没有于特质名字之后加上冒号并指明 `Display` 特质，便尝试使用 `to_string`，咱们就会得到一个声称当前作用域中的类型 `&Self` 下，未找到名为 `to_string` 的方法的报错。

下面来看看当咱们尝试在某个未实现 `Display` 的类型，比如 `Point` 结构体上，实现 `OutlinePrint` 时会发生什么：


```rust
struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point {}
```

咱们会得到一个声称要求 `Display` 当其未实现的报错：

```console
$ cargo run
   Compiling supertrait v0.1.0 (/home/lenny.peng/rust-lang/supertrait)
error[E0277]: `Point` doesn't implement `std::fmt::Display`
  --> src/main.rs:21:23
   |
21 | impl OutlinePrint for Point {}
   |                       ^^^^^ `Point` cannot be formatted with the default formatter
   |
   = help: the trait `std::fmt::Display` is not implemented for `Point`
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
note: required by a bound in `OutlinePrint`
  --> src/main.rs:3:21
   |
3  | trait OutlinePrint: fmt::Display {
   |                     ^^^^^^^^^^^^ required by this bound in `OutlinePrint`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `supertrait` due to previous error
```

为修复这个问题，咱们就要在 `Point` 上实现 `Display` 并满足 `OutlinePrint` 所需的约束，如下面这样：

```rust
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write! (f, "({}, {})", self.x, self.y)
    }
}
```

随后在 `Point` 上实现 `OutlinePrint` 就将成功编译，而咱们就可以在 `Point` 实例上调用 `outline_print` 来将其实现在星号轮廓里了。


## 使用新型模式在外层类型上实现外层的特质

**Using the Newtype Pattern to Implement External Traits on External Types**


第 10 章中的 [“在类型上实现特质”](Ch10_Generic_Types_Traits_and_Lifetimes.md#在类型上实现某个特质) 小节，咱们曾提到，指明只有当特质或类型二者之一，属于代码本地的时，咱们才被允许在类型上实现特质的孤儿规则，the orphan rule。而使用涉及到在元组结构体中创建出一个新类型的 *新型模式，newtype pattern*，那么绕过这种限制便是可行的了。（咱们曾在第 5 章的 [“使用不带命名字段的元组结构体来创建不同类型”](Ch05_Using_Structs_to_Structure_Related_Data.md#使用不带命名字段的元组结构体来创建不同类型) 小节，谈到过元组结构体）这种元组结构体讲有一个字段，且将是围绕咱们要实现某个特质的类型的一个瘦封装，a thin wrapper。随后这个封装类型，便是咱们代码箱的本地类型了，而咱们就可以在这个封装上实现那个特质了。所谓 *新型，newtype*，是源自 Haskell 编程语言的一个术语。使用这种模式没有运行时性能代码，同时那个封装类型在编译时会被略去。

作为一个示例，就说咱们打算在 `Vec<T>` 上实现 `Display`，而由于 `Display` 特质与 `Vec<T>` 类型，均被定义在咱们代码箱外部，因此孤儿规则会阻止咱们直接这样做。咱们可以构造一个保存着 `Vec<T>` 类型实例的 `Wrapper`；随后咱们就可以在 `Wrapper` 上实现 `Display`，并使用那个 `Vec<T>` 值，如下清单 19-23 中所示。


文件名：`src/main.rs`


```rust
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write! (f, "[{}]", self.0.join(", "))
    }
}
fn main() {
    let w = Wrapper(vec! [String::from("你好"), String::from("世界")]);
    println! ("w = {}", w);
}
```

*清单 19-23：创建一个围绕 `Vec<String>` 的 `Wrapper` 类型来实现 `Display`*

由于 `Wrapper` 是个元组结构体，且 `Vec<T>` 是该元组中位于索引 `0` 处的项目，因此其中 `Display` 的实现，便使用了 `self.0` 来方法那个内部的 `Vec<T>`。随后咱们就可以在 `Wrapper` 上使用 `Display` 的功能了。

使用这种技巧的缺点，则是那个 `Wrapper` 是个新的类型，因此其没有他所保存值的那些方法。咱们讲必须直接在 `Wrapper` 上，实现 `Vec<T>` 的全部方法，即委托给 `self.0` 的那些方法，这就会允许咱们将 `Wrapper` 完全当作 `Vec<T>` 那样对待了。而若咱们想要这个新的类型，有着那个内部类型所有的全部方法，那么在 `Wrapper` 上实现 `Deref` 特质（曾在第 15 章的 [“运用 `Deref` 特质将灵巧指针像常规引用那样对待”](Ch15_Smart_Pointers.md#通过实现-deref-特质而像引用那样对待某个类型) 小节讨论过），来返回那个内部类型，将是一种办法。而若咱们不打算 `Wrapper` 类型有着内部类型的所有方法 -- 比如，为限制 `Wrapper` 的行为 -- 咱们就必须手动实现仅咱们想要的那些方法了。


即使不牵涉到特质，这种新型模式也是有用的。接下来就要转换一下视角，而看看与 Rust 的类型系统交互的一些高级方式。


（End）


