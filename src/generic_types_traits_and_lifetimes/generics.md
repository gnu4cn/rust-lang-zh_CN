# 通用数据类型

我们使用泛型来创建函数签名或结构体等项目的定义，然后咱们可以对许多不同的具体数据类型使用他们。咱们来首先看看怎样使用泛型定义函数、结构体、枚举及方法等。然后，我们将讨论到泛型会怎样影响代码性能。


## 函数定义方面

在定义使用泛型的函数时，我们放置泛型于函数的签名中，我们通常将于其中指定参数与返回值的数据类型。这样做使我们的代码更灵活，并在防止代码重复的同时，提供更多功能给咱们函数的调用者。

继续我们的 `largest` 函数，下面清单 10-4 显示了两个函数，均为找出切片中最大值。我们随后将合并这两个函数为使用泛型的单个函数。

<a name="listing_10-4"></a>
文件名：`src/main.rs`

```rust
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec! [34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println! ("最大数为 {result}");

    let char_list = vec! ['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println! ("最大字符为 {result}");
}
```

**清单 10-4**：仅在名字和签名类型上不同的两个函数

`largest_i32` 函数是我们在 [清单 10-3](../Ch10_Generic_Types_Traits_and_Lifetimes.md#listing_10-3) 中提取的函数，会找出切片中最大的 `i32` 函数。`largest_char` 函数会找出切片中的最大 `char`。两个函数体有着同样的代码，因此让我们通过在单个函数中引入泛型参数来消除重复。

为了参数化新的单个函数中的类型，咱们需要命名类型参数，就像我们对函数的值参数，the value parameters，所做的那样。咱们可以使用任何标识符作为类型参数的名字。但咱们将使用 `T`，因为根据约定，Rust 中的类型参数名字要短，通常只有一个字母，并且 Rust 的类型命名约定，type-naming convention，为驼峰式大小写命名规则，UpperCamelCase。`T` 为 *type* 的缩写，便是大多数 Rust 程序员的默认选择。

当我们在函数体中使用参数时，我们必须在签名中声明出参数名字，以便编译器知道该名字表示什么。同样，当咱们在函数签名中使用类型参数名字时，我们必须在使用他之前声明出类型参数名字。为了定义通用的 `largest` 函数，我们就要放置类型名字声明于函数名字与参数列表之间的尖括号，`<>` 内，如下所示：

```rust
fn largest<T>(list: &<T>) -> &T {
```

我们把这个定义读作：“函数 `largest` 对某一类型 `T` 通用，the function `largest` is generic over some type `T`”。这个函数有着一个名为 `list` 的参数，其为类型 `T` 的值的切片。`largest` 函数将返回一个对同一类型 `T` 的值的引用。

下面清单 10-5 显示了在其签名中使用通用数据类型合并后的 `largest` 函数的定义。这个清单还展示了咱们可以怎样以一个 `i32` 的值或 `char` 的值的切片调用该函数。请注意这段代码尚不会编译。

<a name="listing_10-5"></a>
文件名：`src/main.rs`

```rust
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec! [34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println! ("最大数为 {result}");

    let char_list = vec! ['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println! ("最大字符为 {result}");
}
```

**清单 10-5**：使用泛型参数的 `largest` 函数；这段代码尚不编译

当我们现在编译这段代码，我们将得到下面这个报错：

```console
$ cargo run
   Compiling generic_func v0.1.0 (/home/hector/rust-lang-zh_CN/projects/generic_func)
error[E0369]: binary operation `>` cannot be applied to type `&T`
 --> src/main.rs:5:17
  |
5 |         if item > largest {
  |            ---- ^ ------- &T
  |            |
  |            &T
  |
help: consider restricting type parameter `T` with trait `PartialOrd`
  |
1 | fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
  |             ++++++++++++++++++++++

For more information about this error, try `rustc --explain E0369`.
error: could not compile `generic_func` (bin "generic_func") due to 1 previous error
```

帮助文本提到了 `std::cmp::PartialOrd`，其是个特质，而我们即将在下一小节中讨论特质。现在只需知道，这个报错指出 `largest` 的主体将不工作于 `T` 可能的所有可能类型。因为我们打算在函数主体中比较类型 `T` 的值，所以我们只能使用值可以排序的类型。为了支持比较操作，标准库提供了 `std::cmp::PartialOrd` 特质，咱们可以对类型实现他（请参阅 [附录 C](../appendix/derivable_traits.md#用于排序比较的-partialord-与-ord) 了解有关这一特质的更多信息）。要修复清单 10-5，我们可以按照帮助文本的建议，限制 `T` 的有效类型为仅实现 `PartialOrd` 的类型。该清单随后将编译，因为标准库在 `i32` 与 `char` 上均实现了 `PartialOrd`。


## 结构体定义方面

我们也可以使用 `<>` 语法，定义结构体为在一个或多个字段中使用泛型参数。下面清单 10-6 定义了一个 `Point<T>` 结构体来保存任意类型的 `x` 与 `y` 坐标值。

<a name="listing_10-6"></a>
文件名：`src/main.rs`

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
```

**清单 10-6**：保存类型 `T` 的 `x` 与 `y` 值的 `Point<T>` 结构体

在结构体定义中使用泛型的语法与在函数定义中用到的类似。首先，我们在紧接着结构体名字后的尖括号内声明类型参数的名字。然后，咱们在结构体定义中，原本指定具体类型之处使用泛型类型。

请注意，由于咱们仅使用了一种泛型类型来定义 `Point<T>`，所以这一定义表明，`Point<T>` 结构体对某一类型 `T` 是通用的，并且字段 `x` 和 `y` 均为同一类型，无论该类型为何。当我们创建一个有着不同类型值的 `Point<T>`，如下面清单 10-7 中所示，我们的代码将不编译。

<a name="listing_10-7"></a>
文件名：`src/main.rs`

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let wont_work = Point { x: 5, y: 4.0 };
}
```

**清单 10-7**：字段 `x` 与 `y` 必须是同一类型，因为他们都有着相同的通用数据类型 `T`

在这个示例中，当咱们指派整数值 `5` 给 `x` 时，咱们就让编译器知道了，这个 `Point<T>` 实例的泛型类型 `T` 将是个整数。然后，当我们为咱们已定义其与 `x` 有着同一类型的 `y` 指定 `4.0` 时，咱们将得到下面这样的类型不匹配错误：

```console
$ cargo run
   Compiling generic_struct v0.1.0 (/home/hector/rust-lang-zh_CN/projects/generic_struct)
error[E0308]: mismatched types
 --> src/main.rs:7:38
  |
7 |     let wont_work = Point { x: 5, y: 4.0 };
  |                                      ^^^ expected integer, found floating-point number

For more information about this error, try `rustc --explain E0308`.
error: could not compile `generic_struct` (bin "generic_struct") due to 1 previous error
```

要定义一个 `Point` 结构体，其中 `x` 与 `y` 均为泛型但可以有着不同类型，我们可以使用多个泛型类型参数。比如，在下面清单 10-8 中，我们修改 `Point` 的定义为对类型 `T` 与 `U` 泛型，其中 `x` 为类型 `T`，`y` 为类型 `U`。

<a name="listing_10-8"></a>
文件名：`src/main.rs`

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
```

**清单 10-8**：对两种类型泛型的 `Point<T, U>`，从而 `x` 与 `y` 可以是不同类型的值

现在展示的所有 `Point` 实例都是允许的！咱们可在定义中使用任意数量的泛型类型参数，但使用过多会使咱们的代码难于阅读。当咱们发现代码中需要大量泛型时，则可能表明咱们的代码需要重新组织为一些更小的部分。


## 枚举定义方面

正如我们对结构体所做的那样，咱们可以定义枚举为他们的变种中保存通用数据类型。咱们再来看看标准库提供的 `Option<T>` 枚举，咱们曾在第 6 章中使用过他：

```rust
enum Option<T> {
    Some(T),
    None,
}
```

这个定义现在对咱们来说应该更有意义了。正如咱们所见，`Option<T>` 枚举对类型 `T` 是通用的，并有着两个变种：`Some`，保存着类型 `T` 的一个值，以及 `None`，不保存任何值。通过使用 `Option<T>` 枚举，咱们可以表达可选值的抽象概念，the abstract concept of an optional value，而由于 `Option<T>` 是泛型的，所以无论可选值是何种类型，咱们都可以使用这种抽象。

枚举也可以使用多个泛型类型。我们曾在第 9 章中用到的 `Result` 枚举的定义就是一个示例：

```rust
enum Result <T, E> {
    Ok(T),
    Err(E),
}
```

`Result` 枚举对 `T` 和 `E` 两种类型通用，并有着两个变种：`Ok`，保存类型 `T` 的值，以及 `Err`，保存类型 `E` 的值。这一定义使得在我们有个可能成功（返回某种类型 `T` 的值）或失败（返回某种类型 `E` 的值）的任何地方，使用 `Result` 枚举都很方便。事实上，这就是我们在 [清单 9-3](../error_handling/result.md#listing_9-3) 中用于打开文件的，其中当文件成功打开时， `T` 被填充以 `std::fs::File`，而当打开文件存在问题时，`E` 就被填充以 `std::io::Error`。

当咱们发现咱们的代码中，多个结构体或枚举仅在他们保存的值的类型方面不同的情况时，咱们可通过使用泛型类型来避免代码重复。


## 方法定义方面

我们可以在结构体及枚举上实现方法（正如我们在 [第 5 章中](../structs/method_syntax.md) 咱们所做的那样，*译注：及 [第 6 章](../enums_and_pattern_matching/defining_an_enum.md)*），并也可以在他们定义中使用泛型类型。下面清单 10-9 展示了我们在 [清单 10-6](#listing_10-6) 中定义的 `Point<T>` 结构体，带有一个定义在其上的名为 `x` 的方法。


<a name="listing_10-9"></a>
文件名：`src/main.rs`

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```

**清单 10-9**：在 `Point<T>` 结构体上实现一个名为 `x` 的方法，将返回对类型 `T` 的 `x` 字段的引用

在这里，我们已在 `Point<T>` 上定义了个名为 `x` 的方法，返回到字段 `x` 中的数据的引用。

请注意，我们必须在 `imple` 之后声明 `T`，以便我们可以使用 `T` 来指定我们正在实现类型 `Point<T>` 上的方法。通过在 `impl` 后声明 `T` 为泛型类型，Rust 就可以识别 `Point` 中尖括号里的类型是泛型类型而非具体类型。我们可以为这个泛型参数选择一个不同的泛型参数名字，相比在结构体定义中声明的泛型参数名字，但使用同一个名字是惯例。当咱们在声明了泛型类型的 `impl` 内编写某个方法时，那么该方法将定义在这一类型的任何实例上，无论最终替换泛型类型的何种具体类型。

在定义泛型类型上的方法时，我们还可以指定泛型类型上的约束。例如，我们可以仅对 `Point<f32>` 的实例实现方法，而非对有着任意泛型类型的 `Point<T>` 实例。在下面清单 10-10 中，我们使用具体类型 `f32`，这意味着我们未在 `impl` 之后声明任何类型。


<a name="listing_10-10"></a>
文件名：`src/main.rs`

```rust
impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

**清单 10-10**：一个 `impl` 代码块，仅适用于有着针对泛型类型参数 `<T, U>` 的特定具体类型的结构体

这段代码表示类型 `Option<f32>` 将有着 `distance_from_origin` 方法；而其中 `T` 不是 `f32` 的其他 `Option<T>` 实例将不会被定义这个方法。该方法测量我们的点距坐标 `(0.0, 0.0)` 处点的有多远，并使用了仅对浮点类型可用的数学运算。

结构体定义中的泛型类型参数，并不总与咱们在同一结构体的方法签名中使用的泛型类型参数相同。下面清单 10-11 对 `Point` 结构体使用泛型类型 `X1` 与 `Y1`，而对 `mixup` 方法签名使用 `X2` 和 `Y2`，以使示例更清晰。这个方法以 `self` `Point` 中的 `x` 值（类型为 `X1`）与传入的 `Point` 中的 `y` （类型为 `Y2`），创建一个新的 `Point` 实例。

<a name="listing_10-11"></a>
文件名：`src/main.rs`

```rust
#[derive(Debug)]
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println! ("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```

**清单 10-11**：使用与其结构体定义不同的泛型类型的方法

在 `main` 函数中，咱们定义了个 `Point`，他有着 `i32` 的 `x`（值为 `5`），及 `f64` 的 `y`（值为 `10.4`）。变量 `p2` 是个 `Point` 结构体，有着字符串切片的 `x`（值为 `Hello`）和 `char` 的 `y`（值为 `c`）。以参数 `p2` 对 `p1` 调用 `mixup` 给到我们 `p3`，其将有着 `i32` 的 `x`，因为 `x` 来自 `p1`。变量 `p3` 将有个 `char` 的 `y`，因为 `y` 来自 `p2`。`println!` 宏调用将打印 `p3.x = 5, p3.y = c`。

这个示例的目的是演示其中一些泛型参数以 `impl` 声明，而另一些以方法定义声明的情形。这里，泛型参数 `X1` 和 `Y1` 声明于 `imple` 之后，因为他们与结构体的定义有关。泛型参数 `X2` 与 `Y2` 声明于 `fn mixup` 之后，因为他们仅与该方法有关。


## 用到泛型的代码的性能问题

咱们可能想知道使用泛型类型参数时，是否存在运行时开销。好消息就是，使用泛型类型将不会使咱们的程序运行得比其在具体类型下更慢。

Rust 通过在编译时对使用泛型的代码执行单态化达成这点。所谓 *单态化，monomorphization* <sup>1</sup>，是指通过在编译时填入用到的具体类型，从而将泛型代码转换为具体代码的过程。在这个过程中，编译器执行与 [清单 10-5](#listing_10-5) 中我们用于创建泛型函数相反的步骤：编译器查看泛型代码被调用的所有位置，并针对泛型代码被调用时的具体类型生成代码。

> **译注**
>
> 参考：[Wikipedia: Monomorphization](https://en.wikipedia.org/wiki/Monomorphization)

咱们来通过使用标准库的通用 `Option<T>` 枚举看看其工作原理：

```rust
let integer = Some(5);
let float = Some(5.0);
```

当 Rust 编译这段代码时，他会执行单态化。在此过程中，编译器读取 `Option<T>` 实例中使用的值，并识别出两种 `Option<T>`：一种是 `i32`，另一种是 `f64`。因此，他展开 `Option<T>` 的泛型定义为特定于 `i32` 和 `f64` 的两个定义，从而以具体定义替换泛型定义。

这段代码的单态化版本类似于以下内容（编译器会使用与我们在这里用于演示的不同名字）：


文件名：`src/main.rs`

```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

泛型 `Option<T>` 被编译器创建的具体定义替换。由于 Rust 会将泛型代码编译为指定每个实例中的类型的代码，因此咱们不会付出使用泛型的运行时开销。当代码运行时，他只会如同我们手动重复了每个定义时原本那样执行。单态化的过程使 Rust 的泛型在运行时极为高效。


（End）

