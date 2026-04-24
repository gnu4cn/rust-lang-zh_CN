# 高级特质

我们在第 10 章中 [以特质定义共用行为](../generic_types_traits_and_lifetimes/traits.md) 小节中首次介绍了特质，但我们未曾讨论一些更高级的细节。现在咱们对 Rust 有了更深入的了解，我们就可以深入探讨其中的细节了。


## 通过关联类型定义特质

所谓 *关联类型*，将类型占位符与特质连接，使得特质方法的定义可以在其签名中使用这些占位符类型。特质的实现者将针对特定实现，指定要使用的具体类型以取代占位符类型。这样，我们就可以无需在特质被实现之前确切地知道类型是什么下，定义一个使用某些类型的特质。

在本章中，我们提到大多数高级特性很少被用到。关联类型处于中间位置：他们的使用频率虽低于本书其余部分中介绍的特性，但比这一章中讨论的其他特性更为常用。

一个带有关联类型特质的示例是，标准库提供的 `Iterator` 特质。其中的关联类型名为 `Item`，代表着实现 `Iterator` 特质的类型正在迭代的值的类型。`Iterator` 特质的定义如下清单 20-13 中所示。

<a name="listing_20-13"></a>
```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

**清单 20-13**：有着关联类型 `Item` 的 `Iterator` 特质的定义

类型 `Item` 是个占位符，而 `next` 方法的定义表明他将返回类型为 `Option<Self::Item>` 的值。`Iterator` 特质的实现者将指定 `Item` 的具体类型，而 `next` 方法将返回包含一个包含该具体类型值的 `Option`。

关联类型可能看起来是个与泛型类似的概念，因为后者允许我们在不指定函数可以处理哪些类型下定义函数。为了探讨这两个概念之间的区别，我们将看看对名为 `Counter` 的类型的一种 `Iterator` 特质的实现，该实现指定 `Item` 类型为 `u32`：

```rust
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --跳过代码--
```

这种语法看起来与泛型相当。那么，为什么不直接如下清单 20-14 中所示那样，使用使用泛型定义 `Iterator` 呢？

```rust
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}
```

**清单 20-14**：使用泛型对 `Iterator` 特质的假设定义

区别在于，当使用泛型时，如清单 20-14 中那样，我们必须在每个实现中注解类型；由于我们还可以针对 `Counter` 实现 `Iterfator<String>` 或任何其他类型，因此我们可以为 `Counter` 提供多个 `Iterator` 实现。换句话说，当特质包含泛型参数时，他可以针对一个类型被实现多次，每次都修改泛型类型参数的具体类型。当我们对 `Counter` 使用 `next` 方法时，我们将必须提供类型注解，来表明我们打算使用的 `Iterator` 实现。

在关联类型下，我们不需要类型注解，因为我们无法对同一个类型多次实现某个特质。在有着使用关联类型的定义的清单 20-13 中，我们只能选择一次 `Item` 的类型，因为只能有一个 `impl Iterator for Counter`。我们不必在对 `Counter` 调用 `next` 的每个地方，都指定我们想要一个 `u32` 值的迭代器。

关联类型也成为了特质合约的一部分：特质的实现者必须提供一种类型，来代替关联类型的占位符。关联类型通常有着一个描述其使用方式的名字，并且在 API 文档中记录关联类型是一种很好的做法。


## 使用默认泛型类型参数和运算符重载

> **译注**：请参考 [Difference Between Method Overloading and Method Overriding in Java](https://www.geeksforgeeks.org/difference-between-method-overloading-and-method-overriding-in-java/) 了解 Java 中的重载与重写的区别。

在泛型类型参数时，我们可以为泛型类型指定一种默认的具体类型。当默认类型有效时，这样做消除了特质的实现者指定具体类型的需要。在声明泛型类型时，咱们可以通过 `<PlaceholderType=ConcreteType>` 语法指定默认类型。

这种技巧有用的一个很好的示例是 *运算符重载*，其中咱们可以在特定情形下自定义运算符（比如 `+`）的行为。

Rust 不允许咱们创建自己的运算符，或重载任意运算符。但咱们可以通过实现与运算符相关的特质，来重载 `std::ops` 中列出的操作和对应的特质。例如，在下面清单 20-15 中，我们重载了 `+` 运算符，以将两个 `Point` 实例相加。我们通过对 `Point` 结构体实现 `Add` 特质来实现这点。

<a name="listing_20-15"></a>
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

**清单 20-15**：实现 `Add` 特质，以针对`Point` 实例重载 `+` 运算符

其中 `add` 方法会将两个 `Point` 实例的 `x` 值和两个实例的 `y` 值相加，从而创建一个新的 `Point`。`Add` 特质有个名为 `Output` 的关联类型，确定从 `add` 方法返回的类型。

这段代码中的默认泛型类型位于 `Add` 特质内部。以下是其定义：

```rust
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
```

这段代码看起来应该熟悉：一个包含一个方法和关联类型的特质。新的部分是 `Rhs=Self`：这种语法叫做 *默认类型参数*。其中 `Rhs` 泛型类型参数（`right hand side` 的缩写），定义了 `add` 方法中 `rhs` 参数的类型。当我们在实现 `Add` 特质时没有为 `Rhs` 指定具体类型时，`Rhs` 的类型将默认为 `Self`，即我们正在对其实现 `Add` 的类型。

当咱们为 `Point` 实现 `Add` 时，由于咱们打算把两个 `Point` 实例相加，因此而使用了 `Rhs` 的默认值。接下来看看，其中咱们打算定制那个 `Rhs` 而非使用其默认值的一个 `Add` 实现示例。

我们有着两个结构体，`Millimeters` 与 `Meters`，保存不同单位的值。这种在另一类型中对现有类型的简单封装，成为 *新类型模式，newtype pattern*，我们会在 [通过新型模式实现外部特质](#通过新型模式实现外部特质) 小节对此进行更详细的说明。我们打算将以毫米为单位的值，与以米为单位的值相加，并要让 `Add` 的实现正确地进行单位转换。我们通过将 `Meters` 作为 `Rhs`，对 `Millimeters` 实现 `Add`，如下清单 20-16 中所示。

<a name="listing_20-16"></a>
```rust
use std::ops::Add;

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

**清单 20-16**：对 `Millimeters` 实现 `Add` 特质，以将 `Millimeters` 与 `Meters` 相加

为了将 `Millimeters` 和 `Meters` 相加，我们指定 `impl Add<Meters>` 以设置 `Rhs` 类型参数的值，而不是使用其默认的 `Self`。

咱们将以如下两种主要方式，使用默认类型参数：

1. 在不破坏现有代码的情况下，扩展某个类型；
2. 允许在大多数用户不需要的特定情形下进行自定义。

标准库的 `Add` 特质就是第二种用途的一个示例：通常，咱们将把两个相似的类型相加，但 `Add` 特质提供了超越这种情况的自定义能力。在 `Add` 特质中使用默认类型意味着，在大多数时候咱们不必指定额外的参数。换句话说，部分实现样板代码不再需要，从而使得更容易使用该特质。

第一个用途与第二种类似，但方向相反：当咱们打算添加类型参数到某个现有特质时，咱们可以给予他一个默认类型参数，从而在不破坏现有实现代码的情况下，扩展该特质的功能。


## 消除同名方法之间的歧义

Rust 并未禁止一个特质有着与另一特质的方法同名的方法，也不会阻止咱们对同一个类型实现这两个特质。咱们还可以对类型实现与特质中方法同名的方法。

在调用名字相同的方法时，咱们需要告诉 Rust 希望使用哪个。请考虑下面清单 20-17 中的代码，其中我们定义了两个特质 `Pilot` 和 `Wizard`，他们都包含一个名为 `fly` 的方法。然后，我们对 `Human` 类型实现了这两个特质，该类型本身已经实现了一个名为 `fly` 的方法。每个 `fly` 方法都执行不同的操作。

<a name="listing_20-17"></a>
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
        println! ("这是你们的机长在讲话。");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println! ("上升！");
    }
}

impl Human {
    fn fly(&self) {
        println! ("*疯狂地挥舞双臂*");
    }
}
```

**清单 20-17**：两个特质被定义为有着 `fly` 方法，并得以对 `Human` 类型实现，以及一个直接在 `Human` 上实现的 `fly` 方法

当我们对 `Human` 实例调用 `fly` 时，编译器默认调用直接在该类型上实现的方法，如下清单 20-18 中所示。

<a name="listing_20-18"></a>
```rust
fn main() {
    let person = Human;
    person.fly();
}
```

**清单 20-18**：对 `Human` 实例调用 `fly`

运行这段代码将打印 `*疯狂地挥舞双臂*`，表明 Rust 调用了直接定义在 `Human` 上的 `fly` 方法。

为了调用 `Pilot` 或 `Wizard` 特质中的 `fly` 方法，我们需要使用更明确的语法，来指定我们指的是哪个 `fly` 方法。下面清单 20-19 演示了这种语法。

<a name="listing_20-19"></a>
文件名：`src/main.rs`

```rust
fn main() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}
```

**清单 20-19**：指定我们打算调用哪个特质的 `fly` 方法

在方法名字前指定特质名字，可以向 Rust 澄清我们打算调用的 `fly` 的实现。我们原本可以写下 `Human::fly(&person)`，这等同于我们在清单 20-19 中使用的 `person.fly()`，但若我们不需要消除歧义，那么这种写法稍微冗长一些。

运行这段代码会打印以下内容：

```console
$ cargo run
   Compiling traits_example v0.1.0 (/home/hector/rust-lang-zh_CN/projects/traits_example)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running `target/debug/traits_example`
这是你们的机长在讲话。
上升！
*疯狂地挥舞双臂*
```

由于 `fly` 方法取一个 `self` 参数，因此当我们有两种都实现了同一个 *特质* 的 *类型* 时，Rust 可以根据 `self` 的类型来确定要使用哪个特质的实现。

<a name="fq_syntax"></a>
然而，不是方法的关联函数没有 `self` 参数。当存在多个类型或特质，以相同的函数名字定义了非方法的函数时，除非咱们使用 *完全限定语法，fully qualified syntax*，否则 Rust 并不总是知道咱们所指的是何种类型。例如，在下面清单 20-20 中，我们为动物收容所创建了一个特质，他们打算将所有狗崽取名为点点。我们以一个关联的非方法函数，构造了一个 `Animal` 特质。`Animal` 特质针对结构体 `Dog` 予以实现，我们还在 `Dog` 上直接提供了一个关联的非方法函数 `baby_name`。

<a name="listing_20-20"></a>
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
        String::from("小狗")
    }
}

fn main() {
    println! ("狗崽叫做 {}", Dog::baby_name());
}
```

**清单 20-20**：一个带有关联函数的特质，以及一个有着同名关联函数且还实现了该特质的类型

我们在于 `Dog` 上定义的 `baby_name` 关联函数中，实现了将所有狗崽取名为点点的代码。`Dog` 类型还实现了 `Animal` 特质，该特质描述了所有动物共有的特征。狗崽都叫做小狗，这一点在对 `Dog` 的 `Animal` 特质实现中，在与 `Animal` 特质关联的 `baby_name` 函数中得以表达。

在 `main` 中，我们调用了 `Dog::baby_name` 函数，他会直接调用定义在 `Dog` 上的关联函数。这段代码会打印以下输出：

```console
$ cargo run
   Compiling traits_example v0.1.0 (/home/hector/rust-lang-zh_CN/projects/traits_example)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/traits_example`
狗崽叫做 点点
```

这种输出不是我们想要的。我们希望调用我们对 `Dog` 实现的 `Animal` 特质的一部分的 `baby_name` 函数，以便代码打印 `狗崽叫做 小狗`。我们在清单 20-19 中使用的指定特质名字的技巧在这里不管用；若我们修改 `main` 为下面清单 20-21 中的代码，我们将得到一个编译报错。

<a name="listing_20-21"></a>
```rust
fn main() {
    println! ("狗崽叫做 {}", Animal::baby_name());
}
```

**清单 20-21**：尝试调用 `Animal` 特质中的 `baby_name` 函数，但 Rust 不知道要使用哪个实现

由于 `Animal::baby_name` 没有 `self` 参数，并且可能有其他类型实现了 `Animal` 特质，因此 Rust 无法计算出我们想要使用哪个 `Animal::baby_name` 实现。我们将得到以下编译器报错：


```console
$ cargo run
   Compiling traits_example v0.1.0 (/home/hector/rust-lang-zh_CN/projects/traits_example)
error[E0790]: cannot call associated function on trait without specifying the corresponding `impl` type
  --> src/main.rs:20:26
   |
 2 |     fn baby_name() -> String;
   |     ------------------------- `Animal::baby_name` defined here
...
20 |     println! ("狗崽叫做 {}", Animal::baby_name());
   |                              ^^^^^^^^^^^^^^^^^^^ cannot call associated function of trait
   |
help: use the fully-qualified path to the only available implementation
   |
20 |     println! ("狗崽叫做 {}", <Dog as Animal>::baby_name());
   |                              +++++++       +

For more information about this error, try `rustc --explain E0790`.
error: could not compile `traits_example` (bin "traits_example") due to 1 previous error
```

为了消除歧义，并告知 Rust 我们想要使用针对 `Dog` 的 `Animal` 实现，而不是其他类型的 `Animal` 实现，我们需要使用完全限定语法。下面清单 20-22 演示了怎样使用完全限定语法。

<a name="listing_20-22"></a>
文件名：`src/main.rs`

```rust
fn main() {
    println! ("小狗叫做 {}", <Dog as Animal>::baby_name());
}
```

**清单 20-22**：使用完全限定语法来指定我们希望调用对 `Dog` 实现的 `Animal` 特质中的 `baby_name` 函数

我们向 Rust 提供了尖括号内的类型注解，这表明我们希望调用对 `Dog` 实现的 `Animal` 特质中的 `baby_name` 函数，即表明我们希望针对这次函数调用，将 `Dog` 类型视为 `Animal`。这段代码现在将打印我们想要的输出：

```console
$ cargo run
   Compiling traits_example v0.1.0 (/home/hector/rust-lang-zh_CN/projects/traits_example)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/traits_example`
狗崽叫做 小狗
```

一般而言，完全限定语法定义如下：

```rust
<Type as Trait>::function(receiver_if_method, next_arg, ...);
```

对于不属于方法的关联函数，就不存在 `receiver`：只有其他参数的列表。咱们可以在调用函数或方法的任何地方使用完全限定语法。但是，对于 Rust 可以从程序中的其他信息计算出的部分，咱们可以省略这种语法的任何部分。仅在存在多个使用相同名字的实现，并且 Rust 需要帮助来时别咱们想要调用哪个实现时，咱们才需要使用这种更详细的语法。

## 使用超特质

有时，咱们可能会编写一个依赖于另一特质的特质定义：为了让某种类型实现前一个特质，咱们会希望要求该类型也实现后一个特质。咱们之所以会这样做，是为了让咱们的特质定义可以使用后一个特质中的关联项目。咱们的特质所依赖的特质，被称为咱们特质的 *超特质，supertrait*。

例如，假设我们打算构造一个带有 `outline_print` 方法的 `OutlinePrint` 特质，该方法将打印格式化的给定值，使其被星号框起来。也就是说，假设有个实现标准库特质 `Display` 以得到 `(x, y)` 的 `Point` 结构体，当我们对  `x` 为 `1`、`y` 为 `3` 的 `Point` 调用 `outline_print` 时，他应打印以下内容：

```console
**********
*        *
* (1, 3) *
*        *
**********
```

在 `outline_print` 方法的实现中，我们打算使用 `Display` 特质的功能。因此，我们需要指定 `OutlinePrint` 特质仅适用于同时实现 `Display`，且提供 `OutlinePrint` 所需功能的类型。我们可以通过指定 `OutlinePrint: Display`，在特质定义中实现这点。这种技巧类似于添加特质边界到特质。下面清单 20-23 展示了 `OutlinePrint` 特质的实现。

<a name="listing_20-23"></a>
```rust
use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println! ("{}", "*".repeat(len + 4));
        println! ("*{}*", " ".repeat(len + 2));
        println! ("* {output} *");
        println! ("*{}*", " ".repeat(len + 2));
        println! ("{}", "*".repeat(len + 4));
    }
}
```

**清单 20-23**：实现需要 `Display` 中功能的 `OutlinePrint` 特质

由于我们已指定 `OutlinePrint` 需要 `Display` 特质，因此我们可以使用 `to_string` 函数，他会针对实现 `Display` 的任何类型自动实现。若我们在没有于特质名字之后添加冒号并指定 `Display` 特质的情况下，尝试使用 `to_string`，我们就会得到一个报错，指出在当前作用域中找不到类型 `&Self` 的名为 `to_string` 的方法。

我们来看看当我们尝试对某个未实现 `Display` 的类型，比如 `Point` 结构体，实现 `OutlinePrint` 时会发生什么：


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
   Compiling traits_example v0.1.0 (/home/hector/rust-lang-zh_CN/projects/traits_example)
error[E0277]: `Point` doesn't implement `std::fmt::Display`
  --> src/main.rs:20:23
   |
20 | impl OutlinePrint for Point {}
   |                       ^^^^^ unsatisfied trait bound
   |
help: the trait `std::fmt::Display` is not implemented for `Point`
  --> src/main.rs:15:1
   |
15 | struct Point {
   | ^^^^^^^^^^^^
note: required by a bound in `OutlinePrint`
  --> src/main.rs:3:21
   |
 3 | trait OutlinePrint: fmt::Display {
   |                     ^^^^^^^^^^^^ required by this bound in `OutlinePrint`

error[E0277]: `Point` doesn't implement `std::fmt::Display`
  --> src/main.rs:24:7
   |
24 |     p.outline_print();
   |       ^^^^^^^^^^^^^ unsatisfied trait bound
   |
help: the trait `std::fmt::Display` is not implemented for `Point`
  --> src/main.rs:15:1
   |
15 | struct Point {
   | ^^^^^^^^^^^^
note: required by a bound in `OutlinePrint::outline_print`
  --> src/main.rs:3:21
   |
 3 | trait OutlinePrint: fmt::Display {
   |                     ^^^^^^^^^^^^ required by this bound in `OutlinePrint::outline_print`
 4 |     fn outline_print(&self) {
   |        ------------- required by a bound in this associated function

For more information about this error, try `rustc --explain E0277`.
error: could not compile `traits_example` (bin "traits_example") due to 2 previous errors
```

为了修复这个问题，我们对 `Point` 实现 `Display`，从而满足 `OutlinePrint` 所需的约束条件，像下面这样：

```rust
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write! (f, "({}, {})", self.x, self.y)
    }
}
```

然后，对 `Point` 实现 `OutlinePrint` 特质将成功编译，我们就可以对 `Point` 实例调用 `outline_print` 方法，将其显示在由星号构成的轮廓内。


## 通过新型模式实现外部特质

在第 10 章中 [对类型实现特质](../generic_types_traits_and_lifetimes/traits.md#对类型实现特质) 小节中，我们提到了 “孤儿规则，the orphan rule”，规定只有当特质或类型（或两者同时）属于我们代码箱本地时，才允许对类型实现特质。我们可以使用新型模式，the newtype pattern, 绕过这一限制，该模式涉及在元组结构体中创建新类型。（我们在第 5 章中 [以元组结构体创建不同类型](../structs/defining_and_instantiating.md#以元组结构体创建不同类型) 小节中讨论过元组结构体）元组结构体将包含一个字段，并且是我们打算实现某个特质的类型的轻量级包装器。然后，包装器类型对于我们的代码箱来说就属于本地的，我们可以对包装器实现该特质。所谓 *新型，newtype*，是一个 [源自 Haskell 编程语言的术语](https://wiki.haskell.org/index.php?title=Newtype)。使用这种模式不会造成运行时性能开销，并且包装器类型会在编译时省略。

举例来说，假设我们打算对 `Vec<T>` 实现 `Display`，但 “孤儿规则” 会阻止我们直接这样做，因为 `Display` 特质和 `Vec<T>` 类型均被定义在我们的代码箱外部。我们可以构造一个包含 `Vec<T>` 类型实例的 `Wrapper` 结构体；然后，我们可以对 `Wrapper` 实现 `Display` 并使用 `Vec<T>` 值，如下清单 20-24 中所示。

<a name="listing_20-24"></a>
文件名：`projects/newtype/src/main.rs`

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
    println! ("w = {w}");
}
```

**清单 20-24**：创建一个包围 `Vec<String>` 的 `Wrapper` 类型，以实现 `Display`

由于 `Wrapper` 是个元组结构体，而 `Vec<T>` 是元组中索引 `0` 处的项目，因此 `Display` 的实现使用 `self.0` 来访问内层的 `Vec<T>`。这样，我们就可以对 `Wrapper` 使用 `Display` 特质的功能。

使用这种技巧的缺点在于，`Wrapper` 是个新的类型，因此他不具备其包含的值的方法。我们将必须直接对 `Wrapper` 实现 `Vec<T>` 的所有方法，以便这些方法委托给 `self.0`，这将允许我们将 `Wrapper` 完全视为 `Vec<T>`。而若我们希望新类型具有内层类型的所有方法，那么对 `Wrapper` 实现 `Deref` 特质，以返回内层类型将是一种解决方法（我们在第 15 章中的 [将灵巧指针视为普通引用](../smart_pointers/deref-t.md) 小节中讨论过实现 `Deref` 特质）。若我们不希望 `Wrapper` 类型拥有内层类型的所有方法 -- 例如，为了限制 `Wrapper` 类型的行为 -- 我们就必须手动实现我们真正想要的方法。


即使不涉及特质，新型模式也很有用。我们来转换一下视角，看看与 Rust 类型系统交互的一些高级方式。


（End）


