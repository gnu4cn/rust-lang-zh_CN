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


## 通过新型模式实现外部特质


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


