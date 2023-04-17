# 泛型、特质与生命周期

每种编程语言都有着用于有效处理重复概念的一些工具。在 Rust 中，一种这样的工具就是 *泛型（generics）*：将一些具体类型或其他属性的替身抽象出来。对于在编译及运行代码时泛型处有着什么，咱们无需知悉就可以表达泛型的行为，或与其他泛型之间的关系（abstract stand-ins for concret types or other properties. We can express the bevavior of generics or how they relate to other generics without knowing what will be in their place when compiling and running the code）。

与函数取一些未知值，以在多个具体值上运行函数体中同样代码的方式一样，其也可以取一些泛型的参数，而非像是 `i32` 或 `String` 这样的具体类型。事实上，前面在第 6 章的 `Option<T>`，第 8 章的 `Vec<T>` 和 `HashMap<K, V>`，还有第 9 章的 `Result<T, E>` 中，就已经用到了泛型特性。本章中，将探讨怎样定义咱们自己的、带有泛型特性的类型、函数及方法！

首先，这里会回顾怎样对函数进行凝练，从而减少代码重复。随后会使用同样技巧，来将两个只是参数类型不同的函数，构造为一个泛型函数。这里还会说明，怎样在结构体与枚举定义中使用泛型。

接着就会掌握怎样使用 *特质（traits）*，来以泛型方式定义动作行为。可将特质与泛型结合，来将某个泛型约束为只接受有着特定行为的那些类型，而不再是任意类型。

最后，这里将讨论 *生命周期（lifetimes）*：给到编译器有关引用之间关系信息的各种泛型。生命周期特性实现了给到编译器有关借用值的足够信息，从而在相比于没有咱们帮助而未给到这些信息时，编译器就能够于更多的不同情形下，确保这些引用的有效性。


### 通过提取出函数，而去除重复

泛型特性允许咱们以表示多种类型方式的占位符，替换掉特定类型，而消除代码重复。在进入到泛型语法之前，咱们先来看看，怎样以不涉及泛型的，而是用表示多个值的占位符替换特定值，提取出函数的方式消除重复。随后就会把这同样技巧，应用到提取泛型函数上！通过看到如何识别出可提取到函数中的重复代码，咱们就将开始识别出可使用泛型特性的重复代码。

这里会以下面清单 10-1 中，找出清单里极大数的简短程序开始。

文件名：`src/main.rs`

```rust
fn main() {
    let number_list = vec! [34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println! ("极大数为 {}", largest);
}
```

*清单 10-1：找出某个数字清单中的极大数*

这里将一个整数清单，存储在了变量 `number_list` 中，并将到该清单中第一个数字的引用，放在一个名为 `largest` 的变量里。这里随后对那个清单中的全部数字进行迭代，并在当前数字大于存储在 `largest` 中的数字时，替换掉那个变量中的引用。而在当前数小于或等于至今所见到的极大数时，那个变量则不会改变，而代码会继续到清单中的下一个数。在对清单中的全部数字进行审视后，`largest` 就应指向那个极大数，在此示例中即为 `100`。

现在咱们接受了找出两个不同数字清单中极大数的任务。为完成这个任务，咱们可以选择重复清单 10-1 中的代码，并在程序中两个不同位置，使用那相同逻辑，如下清单 10-2 中所示。

文件名：`src/main.rs`

```rust
fn main() {
    let number_list = vec! [34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println! ("极大数为 {}", largest);

    let number_list = vec! [102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println! ("极大数为 {}", largest);
}
```

*清单 10-2：找出 **两个** 数字清单中最大数的代码*

尽管此代码工作了，但那些重复代码则是乏味且容易出错的。在修改此代码时，还必须记住在多个地方更新代码。

为消除这种重复，咱们将通过定义一个运行在于参数中所传入的任意整数清单之上的函数，来消除这种重复。此方案会令到这里的代码更清楚，并实现了找出某个清单中极大数这一概念的抽象表达。

在下面的清单 10-3 中，咱们就把找出极大数的代码，提取到了一个名为 `largest` 的函数中。随后调用了该函数来找出了清单 10-2 中两个数字清单的极大数。将来咱们还可以在可能遇到的任何其他 `i32` 值清单上，使用这个函数。

文件名：`src/main.rs`

```rust
fn largest(list: &[i32]) -> &i32 {
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
    println! ("极大数为 {}", result);

    let number_list = vec! [102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println! ("极大数为 {}", result);
}
```

*清单 10-3：抽象后的找出两个清单中极大数的代码*

这个 `largest` 函数有着一个名为 `list` 的参数，该参数表示了任意的、可能传入到该函数的一些 `i32` 值的切片。那么由此而来，在调用该函数时，该代码就会运行在所传入的那些特定值上。

总的来说，以下就是将代码从清单 10-2 修改为清单 10-3 所用的步骤：

1. 识别出重复代码；
2. 将重复代码提取到目标函数的函数体中，并在函数签名中指定重复代码的输入与输出值；
3. 将重复代码的两个实例，更新为调用这个提取出的函数。

接下来，就要在泛型下，使用这些同样步骤来降低代码重复了。与函数体可以在抽象的 `list`， 而非具体值上运作的方式一样，泛型实现了代码在抽象类型上的操作。

比如，假设说这里有两个函数：一个时在 `i32` 值的切片中，找出极大项，而另一个是在 `char` 值的切片中，找出极大项。那该怎样消除重复呢？下面就来解决这个问题！


## 通用数据类型

**Generic Data Types**

这里会使用泛型，来创建诸如函数签名或结构体等的定义，随后咱们便可以将这些定义，用于许多不同的具体数据类型。首先咱们来看看，怎样运用泛型特性来定义函数、结构体、枚举及方法等。接下来就会讨论到，泛型如何影响到代码性能。


### 函数定义方面

在定义用到泛型的函数时，就要把泛型放在咱们通常于其中，指明参数与返回值数据类型的函数签名中。这样做就会在阻止代码重复的同时，令到代码更为灵活，同时提供到更多功能给咱们函数的调用者。

继续之前的 `largest` 函数，下面清单 10-4 给出了两个均为找出某个切片中极大值的函数。这随后就要将这两个函数，合并为使用泛型特性的单个函数。

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
    println! ("极大数为 {}", result);

    let char_list = vec! ['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println! ("极大字符为 {}", result);
}
```

*清单 10-4：两个只是名字与签名中类型不同的函数*

其中的 `largest_i32` 函数，即为在清单 10-3 中所提取出的那个，找出某个切片中最大的 `i32` 函数。而这里的 `largest_char` 函数则是找出某个切片中的极大 `char`。由于这两个函数体有着同样代码，因此这里就要通过在单个函数中，引入泛型参数来消除重复。

为将新单一函数中的类型参数化，咱们需要给类型参数命名，就如同咱们对某个函数的那些实参（值参数），the value parameters，所做的那样。可将任意标识符，用作类型参数名字。不过咱们将使用 `T`，这是因为根据约定，Rust 中的参数名字都是简短的，通常只有一个字母，还因为 Rust 的类型命名约定为驼峰式大小写命名规则（CamelCase）。而 `T` 作为 “type” 的简写，其便是大多数 Rust 程序员的默认选择了。

当咱们要在函数体中，用到某个参数时，咱们必须在函数签名中声明出这个参数，如此编译器便知道那个名字表示什么。与此类似，当咱们要在函数签名中，用到某个类型参数名字时，在使用该类型参数之前，咱们必须声明出这个类型参数。要定义这个泛型的 `largest` 函数，就要把类型名字声明，放在尖括号（`<>`）里，于函数名字与参数列表之间，如下所示：

```rust
fn largest<T>(list: &<T>) -> &T {
```

咱们把这个定义读作：函数 `largest` 对某个类型 `T` 通用（the function `largest` is generic over some type `T`）。该函数有着一个名为 `list` 的参数，其为类型 `T` 值切片。`largest` 函数将返回一个到同样类型 `T` 值的引用。

下面清单 10-5 给出了这个在其签名中用到通用数据类型的合并 `largest` 函数的定义。这个清单还展示了咱们可以怎样使用 `i32` 值切片，或 `char` 值切片调用该函数。请注意此代码尚不会编译，但咱们将在本章后面修复他。

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
    println! ("极大数为 {}", result);

    let char_list = vec! ['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println! ("极大字符为 {}", result);
}
```

*清单 10-5：使用泛型参数的 `largest` 函数；此代码尚不会编译*

现在编译此代码，将得到如下错误信息：

```console
$ cargo run                                                                                      lennyp@vm-manjaro
   Compiling generics_demo v0.1.0 (/home/lennyp/rust-lang/generics_demo)
error[E0369]: binary operation `>` cannot be applied to type `&T`
 --> src/main.rs:5:17
  |
5 |         if item > largest {
  |            ---- ^ ------- &T
  |            |
  |            &T
  |
help: consider restricting type parameter `T`
  |
1 | fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
  |             ++++++++++++++++++++++

For more information about this error, try `rustc --explain E0369`.
error: could not compile `generics_demo` due to previous error
```

帮助文本消息提到 `std::cmp::PartialOrd`，其是个 *特质（trait）*，而咱们在下一小节就会讲到特质。至于现在，明白这个报错指出了，`largest` 函数体不会对所有 `T` 可能的类型工作就行。由于咱们是要在该函数体中，比较两个类型 `T` 的值，那么咱们就只能使用值可被排序的类型。为能进行比较，标准库便有这个咱们可在类型上应用的 `std::cmp::PartialOrd` 特质（请参阅附录 C 了解该特质的更多信息）。按照该帮助信息的建议，咱们就要把对 `T` 有效的类型，限制为仅那些实现了 `PartialOrd` 的类型，而由于标准库在 `i32` 与 `char` 上，均实现了 `PartialOrd` 特质，那么这个示例就会编译了。


### 在结构体定义中

咱们也可使用这种 `<>` 语法，将结构体定义为在其一个或多个字段中使用泛型参数。清单 10-6 定义了一个 `Point<T>` 的结构体，来保存任意类型的 `x` 与 `y` 坐标值。

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

*清单 10-6：保存类型 `T` 的 `x` 与 `y` 值的 `Point<T>` 结构体*

在结构体定义中使用泛型特性的语法，与在函数定义中用到的类似。首先，在紧接着结构体名字之后，咱们于尖括号内部，声明了类型参数的名字。随后咱们在原本指明具体类型的结构体定义中，用到了那个泛型。

请注意由于咱们只使用了一个泛型来定义 `Point<T>`，那么这个定义就是说，`Point<T>` 结构体对某些类型 `T` 通用，且不论那种类型为何，字段 `x` 与 `y` *均为* 那同一类型。当咱们要创建有着不同类型值的某个 `Point<T>` 时，如下面清单 10-7 中，那么咱们的代码就不会编译。

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

*清单 10-7：由于字段 `x` 与 `y` 有着同一泛型数据类型 `T`，因此他们必须为同一类型*

在此示例中，当咱们把整数值 `5` 赋值给 `x` 时，咱们就让编译器明白，这个 `Point<T>` 实例的泛型 `T` 将是个整数。随后在咱们把 `4.0` 指定给那个已被咱们定义为与 `x` 有着同一类型的 `y` 时，咱们将得到一个下面这样的类型不匹配错误：

```console
$ cargo run                                                                                      lennyp@vm-manjaro
   Compiling generics_demo v0.1.0 (/home/lennyp/rust-lang/generics_demo)
error[E0308]: mismatched types
 --> src/main.rs:7:38
  |
7 |     let wont_work = Point { x: 5, y: 4.0 };
  |                                      ^^^ expected integer, found floating-point number

For more information about this error, try `rustc --explain E0308`.
error: could not compile `generics_demo` due to previous error
```

要定义出其中 `x` 与 `y` 同时为泛型，又可以有着不同类型的 `Point` 结构体，咱们可使用多个泛型参数。比如，在下面清单 10-8 中，就将 `Point` 的定义，修改为了对类型 `T` 与 `U` 通用，其中 `x` 为类型 `T`，而 `y` 则是类型 `U`。

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

*清单 10-8：对两种类型通用的 `Point<T, U>`，进而 `x` 与 `y` 可以是不同类型的值*

现在上面给出的全部 `Point` 实例，便都是允许的了！咱们可在某个定义中，使用咱们想要泛型参数个数，不过用多了就会令到代码难于阅读。若发现代码中需要很多泛型，那就可能表示咱们的代码，需要重新组织架构为更小的片段了。


### 在枚举的定义中

如同咱们在结构体下所做的那样，咱们可定义出在其变种中，保存一些通用数据类型的枚举。咱们来换个角度看看，咱们在第 6 章中曾使用过的，标准库所提供的 `Option<T>`：

```rust
enum Option<T> {
    Some(T),
    None,
}
```

对咱们来说，这个定义现在应有着更多意涵了。可以看到，`Option<T>` 枚举对类型 `T` 是通用的，并有着两个变种：保存着一个类型 `T` 值的 `Some`，与一个不保存任何值的 `None` 变种。经由使用这个 `Option<T>` 枚举，咱们便可表达出可选值，an optional value，的抽象概念，而由于 `Option<T>` 是通用的，因此咱们就可以在无关乎该可选值为何种类型下，用到这个抽象。

枚举也可以使用多个泛型。在第 9 章中用到的 `Result` 枚举定义，就是一个示例：

```rust
enum Result <T, E> {
    Ok(T),
    Err(E),
}
```

`Result` 枚举对 `T` 和 `E` 两种类型通用，并有着两个变种：保存了一个类型 `T` 值的 `Ok`，与保存了一个类型 `E` 值的 `Err`。这个定义使得在某个操作可能成功（便返回某种类型 `T` 的一个值），或失败（便返回一个某种类型 `E` 的值）的地方，使用 `Result` 枚举方便起来。事实上，这正是咱们在清单 9-3 中，打开某个文件时所用到的，在文件被成功打开时，其中的 `T` 就以 `std::fs::File` 给填上了，而当打开那个文件时，若存在某些问题，那么其中的 `E` 就会被 `std::io::Error` 填充。

当咱们认识到咱们的代码中，有着仅在其所保存值类型方面有区别的多个结构体或枚举的情况时，咱们就可以通过使用泛型避免代码重复。


### 在方法定义中

咱们可以在结构体与枚举上实现方法（正如在第 5 章中咱们所做的），并也可以在他们定义中使用泛型。下面清单 10-9 展示了于其上实现了名为 `x` 方法的，咱们曾在清单 10-6 中定义的 `Point<T>` 结构体。


文件名：`src/main.rs`

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println! ("{}, {}", p.x(), p.y());
}
```

*清单 10-9：在 `Point<T, U>` 结构体上，实现将返回到类型 `T` 的 `x` 字段的引用的一个名为 `x` 的方法*

这里已在 `Point<T, U>` 上，定义了名为 `x` 的、返回到字段 `x` 中数据引用的一个方法。经由在 `impl` 后，将 `T` 声明为泛型，Rust 就可以识别出，`Point` 中尖括号（`<>`） 里的类型是个泛型而非具体类型。对于这个泛型参数，咱们可以选择不同于前面结构体定义中，所声明的泛型参数名字，但使用同一个名字是依照惯例的。在声明了泛型的 `impl` 里编写的方法，不论泛型最终将以何种具体类型所代替，这些方法都将定义在该类型的所有实例上。

当咱们在类型上定义方法时，咱们还可以在泛型上指定约束条件。比如，只在 `Point<f32>` 的实例，而非任意泛型的 `Point<T>` 实例上实现方法。在下面清单 10-10 中，咱们使用了具体类型 `f32`，意味着在 `impl` 之后咱们没有声明任何类型。


文件名：`src/main.rs`

```rust
impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

*清单 10-10：只适用于有着特定具体类型泛型参数 `<T, U>` 的结构体的一个 `impl` 代码块*

此代码表示类型 `Option<f32, f32>` 将有一个 `distance_from_origin` 方法；其中 `T, U` 不是 `f32` 的其他 `Option<T, U>` 实例，就不会被定义这个方法。该方法度量了咱们的点与坐标 `(0.0, 0.0)` 处点的距离，并使用了只对浮点数类型可行的数学运算。

结构体定义中的泛型参数，并不总与咱们在同一结构体方法签名中，所使用的那些泛型参数相同。为让示例更明确，下面清单 10-11 对 `Point` 结构体，使用了泛型 `T` 与 `U`，而对 `mixup` 方法签名则使用了 `X` `Y`。
这个方法使用来自 `self` `Point` 的 `x` 值（类型为 `T`），与来自传入的那个 `Point` 值的 `y` （类型为 `Y`），创建出一个新的 `Point`。

文件名：`src/main.rs`


```rust
#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<X, Y>(self, other: Point<X, Y>) -> Point<T, Y> {
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

*清单 10-11：一个使用了与其结构体定义不同泛型的方法*

在 `main` 函数中，咱们定义了一个有着 `x` 为 `i32` （值为 `5`），及 `y` 为 `f64` （值为 `10.4`）的 `Point`。变量 `p2` 是个有着 `x` 为字符串切片（值为 `Hello`），同时 `y` 为 `char` （值为 `c`）的 `Point` 结构体。以参数 `p2` 调用 `p1` 上的 `mixup`，就给到咱们 `p3`，由于 `p3` 的 `x` 来自于 `p1`，因此将有一个 `i32` 的 `x`。而由于这个变量 `p3` 的 `y` 来自于 `p2`, 因此他将有一个 `char` 的 `y`。那个 `println!` 宏调用，将打印 `p3.x = 5, p3.y = c`。

此示例的目的，是要对其中有些泛型参数是以 `impl` 来声明，而另一些泛型参数则是以方法定义来声明的情形，加以演示。由于这里的泛型参数 `T` 与 `U` 与结构体定义在一起，因此他们是在 `impl` 后声明的。而其中的泛型参数 `X` 与 `Y`，则由于他们只与方法 `mixup` 有关，所以他们就被声明在了 `fn mixup` 之后。


### 使用泛型参数代码的性能问题


咱们或许想知道，在运用了泛型参数时，是否有着运行时的开销。好消息就是，相比于使用具体类型，使用泛型不会令到咱们的程序运行得更慢。

Rust 通过在编译时，完成那些使用了泛型代码的单态化，performing monomorphization of the code using generics，达成这个目的。所谓 *单态化，monomorphization*，是指通过把编译后用到的具体类型，填入到泛型位置，而将通用代码转换为具体代码的过程。在此过程中，编译器会执行与清单 10-5 中，咱们用来创建通用函数相反的步骤：编译器会查看泛型代码被调用到的所有地方，并为那些调用到的泛型代码，生成具体类型代码。

咱们来通过使用标准库的通用 `Option<T>` 枚举，看看单态化的工作原理：

```rust
let integer = Some(5);
let float = Some(5.0);
```

在编译此代码时，Rust 就会执行单态化。在那过程中，编译器会读取这两个 `Option<T>` 实例中用到的值，并识别到两种类型的 `Option<T>`：一个为 `i32`，而另一个为 `f64`。这样一来，编译器就会把 `Option<T>` 的通用定义，展开为两个专门的 `i32` 与 `f64` 定义，由此就用这些特定类型，替换了通用定义。

单态化的代码版本，看起来与下面的类似（编译器会使用不同于这里为演示目的而使用的名字）：


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

那个通用的 `Option<T>`，就被以编译器创建的具体定义给替换掉了。由于 Rust 会把通用代码，编译到指明了各个实例中类型的代码，因此咱们就不会为运用泛型而付出运行时代价。在代码运行时，其会如同原本咱们曾重复了那些定义的代码一样执行。单态化的过程，令到 Rust 的泛型在运行时极为高效。


## 特质：定义共用行为

*特质，a trait*，定义了特定类型所具有，并可与其他类型共用的功能。咱们可使用特质，来以抽象方式定义出共用行为。而运用 *特质边界，trait bounds*，咱们便可以指明带有特定行为的任意类型的泛型，we can use *trait bounds* to specify that a generic type can be any type that has certain behavior。


> 注意：特质与其他语言中名为 *接口，interfaces* 的特性类似，虽然有一些差别。


### 定义特质

类型的行为，是由可在该类型上调用的方法，所组成的。若咱们能于不同类型上调用同样方法时，那么这些不同类型就共用了同样行为。特质定义，是为定义出完成某种目的一套必要行为，而把方法签名编组在一起的一种方式，trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose。

比如说，咱们有着保存了几种类别与数量文本的多个结构体：保存着特定地方新闻报道的 `NewsArticle` 结构体，与最多有 280 个字符、带有表明其是否为一条新推文、retweet 或另一推文回复的 `Tweet` 结构体。

而咱们则打算构造出一个，可以把可能存储于某个 `NewsArticle` 或 `Tweet` 实例中的数据的摘要信息显式出来的，名为 `aggregator` 的媒体聚合器库代码箱。要实现这个目的，咱们就需要每个类型的摘要，而咱们将通过调用实例上的 `summarize` 方法，请求摘要信息。下面清单 10-12 便给出了表达此行为的一个公开 `Summary` 特质定义。


文件名：`src/lib.rs`


```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

*清单 10-12：由 `summarize` 方法提供行为，组成的一个 `Summary` 特质*

这里咱们使用 `trait` 关键字，与随后的特质名字，即此示例中的 `Summary`，而声明出了一个特质。咱们还把该特质声明为了 `pub`，从而依赖于此代码箱的代码箱，也可利用上这个特质，如同咱们将在下面几个示例中所看到的那样。而在花括号里面，咱们要声明出，对实现了这个特质的那些类型行为加以描述的方法签名，在此示例中便是 `fn summarize(&self) -> String`。

在方法签名之后，咱们没有提供位于花括号里的方法实现，而是使用了一个分号。实现此特质的每种类型，必须为该方法的方法体，提供其自己的定制行为。编译器会强制要求，任何有着 `Summary` 特质的类型，都将要有与此签名完全一致的 `summarize` 方法定义好。

其代码体中，特质可有多个方法：一行一个地列出方法签名，同时每行都以分号结束。


### 在类型上实现某个特质

既然咱们已定义出 `Summary` 特质方法所需的签名，咱们便可以在咱们的媒体聚合器中的那些类型上实现他了。下面清单 10-13 给出了在 `NewsArticle` 结构体上，使用标题、作者以及处所字段，来创建出 `summaryize` 方法返回值的一个 `Summary` 实现。而对于 `Tweet` 结构体，咱们则把 `summarize`，定义为假定推文已被限制为 280 字符时，返回用户名加上推文的全部文字。

文件名：`src/lib.rs`

```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format! ("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format! ("{}: {}", self.username, self.content)
    }
}
```

*清单 10-13：在 `NewsArticle` 与 `Tweet` 两个类型上实现 `Summary` 特质*


在类型上实现特质类似于实现常规方法。区别在于，在 `impl` 之后，咱们放置的是咱们打算实现特质的名字，之后要使用 `for` 关键字，后面要指定咱们打算为其实现特质的类型名字。在 `impl` 代码块内，咱们要放入特质定义所定义的方法签名。咱们不再于各个签名之后添加分号，而是要使用花括号，并将咱们想要这个特质对于特定类型而所具有的方法，填充到方法体中。

既然库已在 `NewsArticle` 与 `Tweet` 上实现了 `Summary` 特质，那么库代码箱的用户，就可以如同调用常规方法的那样，调用 `NewsArticle` 与 `Tweet` 实例上的这些特质方法了。唯一区别就是，用户必须将该特质，以及那些类型，同时带入到作用域中。下面就是某个二进制代码箱，怎样能用到咱们的 `aggregator` 库代码箱的示例：


```rust
use aggregator::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "当然，跟大家已经清楚的一样了，朋友们",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 条新推文: {}", tweet.summarize());
}
```

此代码会打印 `1 条推文：horse_ebooks: 当然，跟大家已经清楚的一样了，朋友们`。

依赖于 `aggregator` 代码箱的其他代码箱，同样可以将 `Summary` 特质带入其作用域，以在他们自己的类型上实现 `Summary`。有个限制条件要注意，即只有在特质或类型二者至少有一个属于代码箱本地的时，咱们才能在类型上实现特质。比如，由于定制类型 `Tweet` 对于咱们的代码箱 `aggregator` 是本地的，因此咱们可以将比如 `Display` 这样的标准库特质，像 `aggregator` 代码箱功能的一部分那样，实现在 `Tweet` 上。由而于那个特质 `Summary` 属于 `aggregator` 代码箱本地，咱们便还可在咱们的 `aggregator` 代码箱中，将其实现在 `Vec<T>` 上。


不过咱们是无法将外部特质，实现在外部类型上的。比如，由于 `Display` 特质与 `Vec<T>` 类型，都是定义在标准库中，而均不属于咱们的 `aggregator` 代码箱，咱们就不能在 `aggregator` 代码箱里头，将 `Display` 特质实现在 `Vec<T>` 上。这种限制属于名为 *内聚，coherrnce* 的属性的一部分，更具体地说，便是 *孤儿规则，the orphan rule*，之所以这样叫法，是由于父类型缺席了，this restriction is part of a property called *coherence*, and more specifically the *orphan rule*, so named because the parent type is not present。这条规则确保了其他人的代码无法破坏咱们代码，反之亦然。若没有这条规则，两个代码箱就会对同样类型实现同一特质，那么 Rust 就不清楚要使用那个实现了。


### 默认实现

给特质中某个或全部方法以默认行为，而非在所有类型上都要求实现全部方法，有的时候会是有用的做法。这样做之后，当咱们在某个特定类型上实现特质时，咱们就可以保留或重写，override，各个方法的默认行为。

下面清单 10-14 就给 `Summary` 特质的 `summarize` 方法，指定了一个默认字符串，而非如同在清单 10-12 中咱们曾做的，只定义出方法签名。

文件名：`src/lib.rs`

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("了解更多......")
    }
}
```

*清单 10-14：定义有着 `summarize` 方法默认实现的 `Summary` 特质*

而要使用默认实现来对 `NewsArticle` 的实例进行摘要，咱们就要以 `impl Summary for NewsArticle {}`，指明一个空的 `impl` 代码块。

尽管不再直接在 `NewsArticle` 类型上定义，那个 `summarize` 方法，但咱们是提供了一个默认实现的，并已指明 `NewsArticle` 类型实现了 `Summary` 特质。由此，咱们就可以在某个 `NewsArticle` 实例上，调用这个 `summarize` 方法，如同下面这样：

```rust
    let article = NewsArticle {
        headline: String::from("企鹅队赢得斯坦利杯锦标赛！"),
        location: String::from("美国，宾夕法尼亚州，匹兹堡"),
        author: String::from("Iceburgh"),
        content: String::from(
            "匹兹堡企鹅队再度成为美国曲棍球联盟 \
            NHL 中的最佳球队。"
        ),
    };

    println! ("有新文章可读！{}", article.summarize());
```

此代码会打印出 `有新文章可读！了解更多......`。

创建默认实现，不要求咱们对清单 10-13 中，在 `Tweet` 上 `Summary` 的实现，做任何修改。原因是对某个默认实现进行重写的语法，与实现不具有默认实现的特质方法语法相同。

默认实现可调用同一特质中的其他方法，即使那些别的方法没有默认实现。以这种方式，特质就可以提供到很多有用功能，且只要求特质实现者类型，指明其的一小部分方法。比如，咱们就可以将 `Summary` 特质，定义为有着一个要求予以实现的 `summarize_author` 方法，并在随后定义了有着调用了 `summarize_author` 方法默认实现的 `summarize` 方法：

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format! ("（了解更多来自 {} ......）", self.summarize_author())
    }
}
```

而要使用此版本的 `Summary`，咱们只需在某个类型上实现该特质时，定义出 `summarize_author` 方法：

```rust
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format! ("@{}", self.username)
    }
}
```

定义出 `summarize_author` 后，咱们就可以在 `Tweet` 结构体的实例上，调用 `summarize` 方法了，而 `summarize` 的默认实现，将调用咱们所提供的 `summarize_author` 的定义。由于咱们已实现了 `summarize_author`，在不要求咱们编写任何更多代码下，`Summary` 特质就已给到 `summarize` 方法的行为。

```rust
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "当然，跟大家已经清楚的一样了，朋友们",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 条新推文: {}", tweet.summarize());
```

此代码会打印 `1 条新推文: （了解更多来自 @horse_ebooks ......）`。

请注意从方法的重写实现，调用同一方法的默认实现是不可行的。


### 作为参数的特质

既然清楚了怎样定义和实现特质，那么咱们就可以探讨一下，怎样运用特质来定义出接收不同类型参数的函数。咱们将使用之前清单 10-13 中，在 `NewsArticle` 与 `Tweet` 上曾实现过的 `Summary` 特质，来定义一个会调用其 `item` 参数上 `summarize` 方法的 `notify` 函数，而该参数便是实现了 `Summary` 特质类型的。要完成这个目的，咱们就要使用 `impl Trait` 语法，如下所示：

```rust
pub fn notify(item: &impl Summary) {
    println! ("突发新闻！{}", item.summarize());
}
```

咱们给那个 `item` 参数指定了 `impl` 关键字和特质名字，而不是具体类型。这个参数会接受实现了指定特质的任何类型。在 `notify` 的函数体中，咱们就可以在 `item` 上，调用来自 `Summary` 特质的任何方法了，比如 `summarize`。咱们可以调用 `notify`，并传入 `NewsArticle` 或 `Tweet` 的任意实例。而以任意其他类型，比如 `String` 或 `i32`，调用该函数的代码，由于那些类型没有实现 `Summary`，就不会编译。


#### 特质边界语法

这种在简单情形下工作的 `impl Trait` 语法，实际上是被称作 *特质边界，trait bound* 的较长形式的语法糖，syntax sugar；其看起来像下面这样：


```rust
pub fn notify<T: Summary>(item: &T) {
    println! ("突发新闻！{}", item.summarize());
}
```

这种较长形式与上一小节中的示例是等价的，但要更冗长一些。咱们把特质边界（`Summary`），在冒号之后，与泛型参数声明放在一起，并在一对尖括号里面。

在简单情形下，`impl Trait` 这种语法是方便的，且令到代码更为简洁，而在别的情形下，较完整的特质边界语法，则能表达出更高复杂度。比如，咱们可以有两个实现 `Summary` 的参数。以 `impl Trait` 语法实现这种情况，看起来就会像下面这样：

```rust
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
```

当咱们是要此函数允许 `item1` 与 `item2` 有着不同类型时（只要两个类型都实现了 `Summary` ），那么使用 `impl Trait` 语法便是恰当的。而当要的是强制这两个参数有着同一类型时，咱们就必须使用特质边界，像下面这样：

```rust
pub fn notify<T: Summary>(item1: &T, item2: &T) {
```

其中被指定为 `item1` 与 `item2` 两个参数类型的泛型 `T`，会对该函数加以约束，进而作为 `item1` 与 `item2` 的实参所传递值的具体类型必须相同。


#### 使用 `+` 语法，指定多个特质边界

咱们还可以指明多个特质边界。比方说咱们想要 `notify` 使用 `item` 上的 `summarize` 的同时，还要使用显示格式：咱们就要在 `notify` 定义中，指明 `item` 必须实现了 `Disply` 与 `Summary` 两个特质。使用 `+` 语法，咱们便可达到这个目的：

```rust
pub fn notify(item &(impl Summary + Display)) {
```

`+` 语法同样对泛型上的特质边界有效：


```rust
pub fn notify<T: Summary + Display>(item: &T) {
```

有了指定的这两个特质，那么 `notify` 的函数体，便可调用 `summarize` 函数，及使用 `{}` 来格式化 `item` 了。


#### 使用 `where` 子句获得更清楚的特质边界

**Clearer Trait Bounds with `where` Clauses**

使用过多的特质边界，有着其一些缺点。每个泛型都有自己的特质边界，那么有着多个泛型参数的函数，在其名字与其参数列表之间，就好包含很多特质边界信息，从而令到该函数签名难于阅读。出于这个原因，Rust 有着在函数签名之后的 `where` 子句里，指明特质边界的这种替代语法。从而与其写出下面这个签名：

```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```

咱们便可像下面这样，使用 `where` 子句：

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
```


这个函数的签名，就不那么杂乱无章了：函数名、参数清单与返回值类型紧挨在一起，类似于与不带有很多特质边界的函数。


### 实现了特质的返回值类型

咱们还也可以在返回值处，使用 `impl Trait` 语法来返回某种实现某个特质类型的值，如下所示：


```rust
fn return_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "当然，如同你或许已经知道的一样，朋友们"
        ),
        reply: false,
        retweet: false,
    }
}
```

通过对返回值类型使用 `impl Summary`，而没有命名具体类型下，咱们便指明了 `returns_summarizable` 函数，会返回实现了 `Summary` 特质的类型。在此示例中，`returns_summarizable` 函数返回的是个 `Tweet`，而调用此函数的代码，则无需知会这一点。

仅以其实现了的特质，便指明了返回值类型这种能力，在闭包与迭代器语境下尤为有用，咱们在第 13 章就会讲到他们。闭包与迭代器会创建出只有编译器清楚的类型，或指定起来极长的类型。`impl Trait` 语法，就允许咱们简明地、在无需编写出极长类型下指定出，返回实现了 `Iterator` 特质的某种类型的函数。

然而，只有在返回单个类型时，咱们才能使用 `impl Trait`。比如下面这段在将返回值类型值指定为了 `impl Summary` 下，而要返回 `NewsArticle` 或 `Tweet` 的代码，就不会工作：

```rust
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from("企鹅队赢得斯坦利杯锦标赛！"),
            location: String::from("美国，宾夕法尼亚州，匹兹堡"),
            author: String::from("Iceburgh"),
            content: String::from(
                "匹兹堡企鹅队再度成为美国曲棍球联盟 \
            NHL 中的最佳球队。"
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "当然，跟大家已经清楚的一样了，朋友们",
            ),
            reply: false,
            retweet: false,
        }
    }
}
```

由于编译器中实现 `impl Trait` 语法方式方面的限制，返回 `NewsArticle` 或 `Tweet` 便是不允许的。在第 17 章的 [运用允许不同类型值的特质对象](Ch17_Object_Oriented_Programming_Features_of_Rust.md#使用允许不同类型值的特质对象) 小节，咱们就会降到如何编写有着这种行为的函数。


### 运用特质边界来有条件地实现方法

使用带有用到泛型参数 `impl` 代码块的特质边界，咱们便可根据实现了指定特质的类型，而有条件地实现方法，by using a trait bound with an `impl` block that uses generic type parameters, we can implement methods conditionally for types that implement the specified traits。比如下面清单 10-15 中的类型 `Pair<T>`，就会一直将那个 `new` 函数，实现为返回 `Pair<T>` 的新实例（回顾第 5 章的 [定义方法](Ch05_Using_Structs_to_Structure_Related_Data.md#方法的定义) 小节就知道，`Self` 就是那个 `impl` 代码块的类型别名，此示例中即 `Pair<T>`）。但在接下来的 `impl` 代码块中，若 `Pair<T>` 只在其内部类型 `T` 里，实现启用比较的 `PartialOrd` 特质，*与* 启用打印的 `Display` 特质，那么 `Pair<T>` 就只会实现 `cmp_display` 方法。


```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println! ("极大数为 x = {}", self.x);
        } else {
            println! ("极大数为 y = {}", self.y);
        }
    }
}
```

*清单 10-15：根据特质边界，在泛型上有条件地实现方法，conditionally implementing methods on a generic type depending on trait bounds*

> **注意**：这里的 `new` 是个关联函数，而非方法！只能以 `Pair::new` 形式使用。要作为方法使用，函数就必须要有一个 `self` 参数。

咱们还可对实现了另一特质的任意类型，有条件地实现某个特质。在满足这些特质边界的类型上的特质实现，被称作 *一揽子实现，blanket implementations*，在 Rust 标准库中广泛使用了一揽子实现。比如，标准库就在实现了 `Display` 特质的全部类型上，实现了 `ToString` 特质。标准库中这个 `impl` 代码块，看起来与下面的类似：

```rust
impl<T: Display> ToString for T {
    // --跳过代码--
}
```

由于标准库有着这个一揽子实现，咱们便可在实现了 `Display` 特质的全部类型上，调用由 `ToString` 特质所定义的 `to_string` 方法。比如，由于整数类型实现了 `Display` 特质，那么咱们就可以像下面这样，把整数转换为他们对应的 `String`：

```rust
let s = 3.to_string();
```

一揽子实现，会出现在特质文档的 “相关实现器，Implementors” 部分。


特质与特质边界这两个特性，允许咱们编写出运用泛型参数来减少代码重复的代码，并还向编译器指出了咱们希望该泛型有着特定行为。随后编译器就能使用特质边界信息，来检查代码用到的全部具体类型，是否提供到正确行为。在一般的动态类型语言，dynamically typed languages，中，若调用某个类型上尚未定义的方法，咱们将在运行时收到报错。但 Rust 将这些错误移到了编译时，这样在代码还不能运行的时候，咱们就被强制要求修复这些问题。此外，由于已在编译时被检查过，因此咱们就不必编写检查运行时行为的代码。这样做在提升了性能的同时，不必放弃泛型灵活性。


## 使用生命周期验证引用

生命周期是另一种咱们前面已经用到的泛型。与确保类型有着期望行为的特质不同，生命周期确保的是引用在咱们需要他们有效期间，保持有效，lifetimes ensure that references are valid as long as we need them to be。

在第 4 章中 [引用与借用](Ch04_Understanding_Ownership.md#引用与借用references-and-borrowing) 小节，咱们未曾讨论的一个细节，即 Rust 中的每个引用，都有着 *生命周期，lifetime*，其便是引用有效的作用范围。多数时候，声明周期是隐式而被推导出来的，这正与多数时候类型是被推导出来的一样。咱们只须在可能有多个类型时注解类型。与此类似，在一些引用的生命周期，可能以几种方式存在关联时，咱们就必须注解出生命周期。为确保在运行时用到的具体类型显著有效，Rust 就会要求咱们使用泛型生命周期参数，注解出这些关系，in a similar way, we must annotate lifetimes when the lifetimes of references could be related in a few different ways. Rust requires us to annotate the relationships using generic lifetime parameters to ensure the actual references used at runtime will definitely be valid。

绝大多数别的编程语言，甚至都没有注解周期，annotating lifetimes, 这个概念，因此这会让人感到陌生。尽管咱们在这一章中不会涵盖生命周期的全部，咱们仍将讨论可能遇到的生命周期语法的一些常见方式，从而咱们就能适应这个概念。


### 使用生命周期防止悬空引用

**Preventing Dangling References with Lifetimes**

生命周期的主要目标，就是防止 *悬空引用（dangling references）*，这会导致程序引用到并非其打算要引用的数据。设想下面清单 10-16 中的程序，其有着一个外层作用范围与一个内层作用范围。


```rust
fn main() {
    let r;

    {
        let x = 5;
        r = &x;
    }

    println! {"r: {}", r};
}
```

*清单 10-16：一个使用了值已超出作用域引用的尝试*

> 注意：清单 10-16、10-17 及 10-23 中的示例，都不带初始值地声明了一些变量，那么变量名就存在于外层作用域中。咋一看，这样做似乎与 Rust 的无空值（Rust's having no `null` values）特性相抵触。不过，在尝试于赋予变量值之前使用某个变量，就会得到一个编译器时错误，这样就表示 Rust 实际上是不允许空值（`null` values） 的。


外层作用域声明了一个名为 `r` 不带初始值的变量，同时内层作用域声明了一个名为 `x` 带有初始值 `5` 的变量。在那个内层作用域里头，这里尝试了将 `r` 的值设置为到 `x` 的一个引用。随后那个内层作用域便结束了，而这里尝试打印 `r` 中的值。由于 `r` 所指向的值，在这里尝试使用之前就已超出作用域，因此此代码不会编译。下面就是错误消息：

```console
$ cargo run                                            lennyp@vm-manjaro
   Compiling lifetimes_demo v0.1.0 (/home/lennyp/rust-lang/lifetimes_demo)
error[E0597]: `x` does not live long enough
 --> src/main.rs:6:13
  |
6 |         r = &x;
  |             ^^ borrowed value does not live long enough
7 |     }
  |     - `x` dropped here while still borrowed
8 |
9 |     println! {"r: {}", r};
  |                        - borrow later used here

For more information about this error, try `rustc --explain E0597`.
error: could not compile `lifetimes_demo` due to previous error
```

变量 `x` 未 “存活足够长时间。” 原因就是在内层作用域于第 7 行结束处，变量 `x` 便超出了作用域。然而对于外层作用域，变量 `r` 仍是有效的；有望变量 `r` 的作用域要大一些，这里就讲变量 `x` 就要 “存活得长一些”。若 Rust 允许此代码工作，那么变量 `r` 就会引用到在变量 `x` 超出作用域时，已被解除分配的内存，并且任何尝试在变量 `x` 下的操作，都将不会正确工作。那么 Rust 是怎样判定此代码无效的呢？他运用了一种借用检查器。


### 借用检查器

Rust 编译器有个对作用域加以比较，而确定出全部借用是否有效的 *借用检查器（a borrow checker）*。下面清单 10-17 就给出了与清单 10-16 同样的代码，不过有着显示其中变量生命周期的注解。

```rust
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+
```

*清单 10-17：变量 `r` 与 `x` 生命周期的注解，各自取名为 `'a` 与 `'b`*

这里已使用 `'a` 与 `'b` 分别注解了变量 `r` 与 `x` 的生命周期。如同这里所看到的，内层的 `'b` 代码块，相比外层的 `'a` 声明周期代码块要小得多。在编译时，Rust 就会比较这两个生命周期的大小，并发现变量 `r` 有着 `'a` 的生命周期，但他却指向了一个 `'b` 的生命周期。由于生命周期 `'b` 比 `'a` 要短，于是该程序就被拒绝：引用物（the subject of the reference）未与引用变量，存活同样时间。

下面清单 10-18 修复了该代码，因此他就没有悬空引用，进而就无误地编译了。


```rust
fn main() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}                         // ----------+
```

*清单 10-18：由于被引用数据有着长于引用变量的生命周期，因此这是一个有效的引用*


这里变量 `x` 有着生命周期 `'b`，在此示例中是长于声明周期 `'a` 的。这就意味着由于 Rust 清楚在变量 `r` 中的引用，在变量 `x` 有效期间，将始终有效，因此变量 `r` 就可以对变量 `x` 加以引用。

既然咱们已经清楚引用的生命周期都在何处，以及 Rust 怎样对生命周期加以分析，来确保引用将始终有效，那么接下来就要探讨，函数语境下的参数与返回值的泛型生命周期了（generic lifetimes of parameters and return values in the context of functions）。


### 函数中的泛型生命周期

**Generic Lifetimes in Functions**


下面将编写一个返回两个字符串切片中较长者的函数。该函数将取两个字符串切片，并返回单个的字符串切片。在实现了这个 `longest` 函数后，清单 10-19 中的代码，就会打印 `最长的字符串为 abcd`。


文件名：`src/main.rs`


```rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println! ("最长的字符串为 {}", result);
}
```

*清单 10-19：调用 `longest` 函数来找出两个字符串切片中较长的那个的 `main` 函数*


留意到由于这里并不打算这个 `longest` 函数取得其参数的所有权，因此这里是要该函数取两个字符串切片，两个都是引用变量，而非字符串。请参考第 4 章中的 [作为函数参数的字符串切片](Ch04_Understanding_Ownership.md#字符串切片作为函数参数) 小节，了解更多为何在清单 10-19 中使用的参数，即为这里想要的参数的讨论。

在尝试如下面清单 10-20 中所示的那样，对这个 `longest` 函数加以实现时，那将仍不会编译。


文件名：`src/main.rs`

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else { y }
}
```

*清单 10-20：一种尚不会编译的返回两个字符串切片中较长者的 `longest` 函数实现*


相反，这里会得到以下的谈及生命周期的错误：


```console
$ cargo run                                                                                  lennyp@vm-manjaro
   Compiling lifetimes_demo v0.1.0 (/home/lennyp/rust-lang/lifetimes_demo)
error[E0106]: missing lifetime specifier
 --> src/main.rs:1:33
  |
1 | fn longest(x: &str, y: &str) -> &str {
  |               ----     ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
help: consider introducing a named lifetime parameter
  |
1 | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  |           ++++     ++          ++          ++

For more information about this error, try `rustc --explain E0106`.
error: could not compile `lifetimes_demo` due to previous error
```

该帮助性文字，揭示了由于 Rust 无法弄清返回的引用到底是指向 `x` 还是 `y`，因此返回值类型就需要其上的泛型声明周期函数（a generic lifetime parameter）。事实上，由于在该函数的函数体中，那个 `if` 代码块返回的时到参数 `x` 的引用，而其中的 `else` 代码块返回的是到 `y` 的引用，所以就连咱们也不清楚！

在对该函数进行定义时，是不清楚要传入到该函数的那些具体值的，因此就不清楚究竟是`if` 条件，还是 `else` 条件会被执行。这里也不清楚要传入的那些引用变量的具体声明周期，进而就无法查看如同清单 10-17 及 10-18 中所看到的那些作用域，来判断返回的引用变量是否始终有效。由于 Rust 的借用检查器不清楚其中 `x` 与 `y` 的生命周期，与返回值的生命周期有怎样的关联，因此借用检查器也无法对此做出判断。要修复这个错误，就要添加对这些引用变量之间关系进行定义的泛型生命周期参数，进而借用检查器就可以完成他的分析。


### 生命周期注解语法

**Lifetime Annotation Syntax**

生命周期注解，一点也不会改变引用变量的存活时长。而是在不影响生命周期指向，对多个引用变量生命周期之间的关系加以描述。正如函数签名指定了泛型参数时，函数可接受任意类型一样，通过在函数签名中指定泛型生命周期参数，函数就可以接受任意生命周期的引用了（just as functions can accept any type when the signatures specifies a generic type parameter, functions can accept with any lifetime by specifying a generic lifetime parameter）。

生命周期注解有着些许不同寻常的语法：生命周期参数的名字，必须以撇号（单引号，`'`）开头，通常为全部小写字母，且像泛型一样非常短。多数人会用 `'a` 作为首个生命周期的注解。是将生命周期注解，放在某个引用的 `&` 之后，使用一个空格，来将注解与该引用的类型分隔开。

下面就是一些示例：到某个 `i32` 的不带生命周期参数的引用、到某个 `i32` 的有着名为 `'a` 的生命周期参数，以及到某个 `i32` 的同样有着生命周期 `'a` 的可变引用。

```rust
&i32        // 某个引用
&'a i32     // 某个带有显式生命周期的引用
&'a mut i32 // 某个有着显式生命周期的可变引用
```

由于注解的目的是告诉 Rust （编译器），多个引用的泛型生命周期参数相互之间是怎样关联起来的，因此生命周期本身并没有什么意义。接下来就要在那个 `largest` 函数语境下，对生命周期注解相互之间怎样联系起来加以审视了。


### 函数签名中的生命周期注解

要在函数签名中使用生命周期注解，就需要在函数名字与参数列表之间，在尖括号里头对通用 *生命周期（lifetime）* 参数加以声明，正如之前对通用 *类型（type）* 参数所做的那样。

这里是要该函数签名，表达以下约束：返回的那个引用，将在这两个参数均为有效时，保持有效。这便是参数与返回值生命周期之间的关系。这里将把该生命周期命名为 `'a`，并在随后将其添加到各个引用，如下清单 10-21 中所示。

文件名：`src/main.rs`

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

*清单 10-21：指明了签名中全部引用都必须有着同一生命周期 `'a` 的 `longest` 函数*


此代码应会编译，并在清单 10-19 的 `main` 函数中使用他时，产生出这里想要的结果来。

该函数签名现在会告诉 Rust，对于生命周期 `'a`，该函数会取两个参数，这两个参数都是存活时间至少为 `'a` 的两个字符串切片。该函数签名还会告诉 Rust，从该函数返回的字符串切片，将存活至少生命周期 `'a` 那么长时间。在实践中，这就表示有这个 `longest` 函数返回的引用的生命周期，与该函数参数所引用到的值生命周期中较小的一致。这些关系，就是这里想要 Rust 在分析此代码时，要用到的关系。

当于函数中对生命周期进行注解时，这些注解是介入函数签名中，而非函数体中。这些生命周期注解，成为了该函数合约的一部分，这与签名中的类型较为相似。令到函数包含生命周期合约（the lifetime contract），就意味着 Rust 编译器所完成的分析，可以相对简单一些。在某个函数被注解的方式，或其被调用的方式存在问题时，所报出的编译器错误，就可以更精准地指向所编写代码或约束的某个部分。相反，相比于添加了生命周期注解，在 Rust 编译器要做出更多有关这里所预期生命周期关系的推断时，那么编译器可能就只能够指出，在问题原因处许多步之外，代码的某个使用了（if, instead, the Rust compiler made more inferences about what we intended the relationships of the lifetimes to be, the compiler might only be able to point to a use of our code many steps away from the cause of the problem）。

在将具体引用传递给 `longest` 时，取代 `'a` 的那个具体生命周期，即为参数 `x` 作用域与参数 `y` 作用域重叠的部分。也就是说，这个泛型生命周期 `'a` 将获取到，与 `x` 与 `y` 生命周期中较小那个相等的具体生命周期。由于这里已将那个返回的引用，注解为了同一生命周期参数 `'a`，那么那个返回的引用，就会在 `x` 与 `y` 的生命周期中较小那个的长度期间有效。

下面就来通过传入有着不同具体生命周期的引用，而看看这些生命周期注解，是怎样对这个 `longest` 函数加以限制的。下面清单 10-22 就是一个直观的示例。


文件名：`src/main.rs`

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("长字符串就是长");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println! ("最长的字符串为 {}", result);
    }
}
```

*清单 10-22：在到有着不同具体生命周期 `String` 类型值的引用下，运用这个 `longest` 函数*


在此示例中，到外层作用域结束之前，`string1` 都是有效的，而 `string2` 则是到内层作用域结束之前为有效，同时 `result` 引用了某个在内层作用域结束之前有效的东西。运行此代码，就会看到借用检查器通过了检查；此代码将编译并打印 `最长的字符串为 长字符串就是长`。

接下来，就要尝试一个展示 `result` 中引用的生命周期，必须为这两个参数生命周期中较小的那个的示例。这里将把那个 `result` 变量的声明，移到内层作用域外面而将到该 `result` 变量的赋值，仍然留在有着 `string2` 变量的作用域里头。随后将把那个用到 `result` 变量的 `println!` 语句，移出到内层作用域外面，在内层作用域结束之后。下面清单 10-23 中的代码就不会编译了。


文件名：`src/main.rs`


```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("长字符串就是长");
    let result;

    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println! ("最长的字符串为 {}", result);
}
```

*清单 10-23：尝试在 `string2` 已超出作用域之后对 `result` 加以使用*


在尝试编译此代码时，就会得到这样的错误：


```console
$ cargo run                                                                        lennyp@vm-manjaro
   Compiling lifetimes_demo v0.1.0 (/home/lennyp/rust-lang/lifetimes_demo)
error[E0597]: `string2` does not live long enough
  --> src/main.rs:15:44
   |
15 |         result = longest(string1.as_str(), string2.as_str());
   |                                            ^^^^^^^^^^^^^^^^ borrowed value does not live long enough
16 |     }
   |     - `string2` dropped here while still borrowed
17 |     println! ("最长的字符串为 {}", result);
   |                                    ------ borrow later used here

For more information about this error, try `rustc --explain E0597`.
error: could not compile `lifetimes_demo` due to previous error
```

该错误显示，要让 `result` 对那个 `println!` 语句有效，那么 `string2` 就需要在外层作用域结束之前，保持有效。Rust （编译器）之所以清楚这点，是由于这里使用了同样的生命周期参数 `'a`，对该函数的各个参数与返回值进行了注解。

而作为人类，那么就可以看看这段代码，并发现 `string1` 相较于 `string2` 的生命周期要长，进而由此 `result` 就会包含一个到 `string1` 的引用。由于 `string1` 尚未超出作用域，那么到 `string1` 的某个引用，相对 `println!` 语句仍将有效。然而编译器却无法在此示例中，发现该引用是有效的。这里已告知 Rust，有这个 `longest` 函数所返回引用的生命周期，与所传入的那些参数声明周期中较小者相同。那么，由于代码中可能有着无效的引用，故借用检查器是不允许清单 10-23 中代码的。


请尝试设计更多传入到 `longest` 函数不同值与引用生命周期，及返回引用变量使用方式不同的试验。并在编译这些试验代码前，就这些试验是否会通过借用检查器的检查，做出一些假定；随后在看看所做出的假定是否正确！


### 从生命周期角度进行思考

**Thinking in Terms of Lifetimes**


指定生命周期参数的所需方式，取决于函数是在干什么事情。比如在将 `longest` 函数的实现，修改为了始终返回第一个参数，而非那个最长的字符串切片时，那么就不需要在其中的 `y` 参数上，指定生命周期了。以下代码将会编译：

文件名：`src/main.rs`

```rust
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```


这里已将生命周期参数 `'a` 指定给了参数 `x` 与返回值类型，而由于参数 `y` 的生命周期，与 `x` 或返回值的生命周期，并无任何关系，故这里并未将 `'a` 指定给参数 `y`。

当从某个函数返回一个引用时，返回值类型的生命周期参数，就需要与某个参数的生命周期参数相匹配。而在返回的引用，*未* 指向某个参数时，那么他就必须指向在该函数内部创建的某个值。然而，由于这个函数内部创建的值，在函数结束处将超出作用域，因此这就会是个悬空引用了。请设想下面这个不会编译的尝试性 `longest` 函数实现：

文件名：`src/main.rs`

```rust
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("真正长的字符串");
    result.as_str()
}
```

这里尽管已给那个返回类型指定了生命周期参数 `'a`，但由于其中的返回值生命周期，与那些参数的生命周期毫无关系，故该实现将编译失败。下面就是得到的错误消息：

```console
$ cargo run                                                              lennyp@vm-manjaro
   Compiling lifetimes_demo v0.1.0 (/home/lennyp/rust-lang/lifetimes_demo)
error[E0515]: cannot return reference to local variable `result`
  --> src/main.rs:11:5
   |
11 |     result.as_str()
   |     ^^^^^^^^^^^^^^^ returns a reference to data owned by the current function

For more information about this error, try `rustc --explain E0515`.
warning: `lifetimes_demo` (bin "lifetimes_demo") generated 2 warnings
error: could not compile `lifetimes_demo` due to previous error; 2 warnings emitted
```

问题就是在 `longest` 函数结束处，`result` 就超出了作用域而被清理掉了。这里还在尝试从该函数返回到 `result` 的一个引用。并不存在能够指定一个会改变这个悬空引用的生命周期参数的办法，而 Rust 也不会容许创建悬空引用。在此示例中，最佳修复将是返回一个有着所有权的数据类型，而非某个引用（注：故引用是没有所有权的），进而随后由调用函数（the calling function），来负责清理该值。

最后，生命周期语法，是有关将函数各个参数的生命周期，与函数返回值的生命周期连接起来的。一旦他们连接了起来，那么 Rust 就有了足够信息，来放行一些涉及内存安全的操作，以及蓝星那些会创建出悬空指针或其他危及内存安全的操作。


### 结构体定义中的生命周期注解

**Lifetime Annotations in Struct Definitions**


到目前为止，本书中业已定义的那些结构体，都持有自己的一些类型。这里可以将结构体定义为持有一些引用，但这样的话，就需要在结构体定义中每个引用上，添加生命周期注解了。下面清单 10-24 有着一个名为 `ImportedExcerpt`、保存着一个字符串切片的结构体。

文件名：`src/main.rs`

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("请叫我伊萨梅尔。多年以前.....");
    let first_sentence = novel.split('。').next().expect("找不到一个 '。'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
```

*清单 10-24：保存了一个引用的结构体，因此就需要生命周期注解*

此结构体有着单个的、保存着一个字符串切片，因此是个引用变量的字段 `part`。与通用数据类型（generic data types）之下一样，这里在结构他名字后面，尖括号里头，对其中的通用声明周期参赛进行了声明，进而就可以在这个结构体定义代码体中，使用那个生命周期参数。这样的注解表示，`ImportantExcerpt` 的实例，无法存活过超出保存在其 `part` 字段中引用的生命早期。

这里的 `main` 函数创建了该 `ImportantExcerpt` 的一个实例，该实例保存了到其中由变量 `novel` 所拥有所有权的 `String` 第一句话的引用。`novel` 中的数据，在该 `ImportantExcerpt` 实例被创建之前，便存在了。此外，`novel` 在这个 `ImportantExcerpt` 超出作用域之前，并未超出作用域，因此在这个 `ImportantExcerpt` 实例中的引用，将有效。


### 生命周期的省略

**Lifetime Elision**

现在已经了解到每个引用都有生命周期，以及需要给使用到引用的函数与结构体，指明生命周期参数。不过，在第 4 章中，曾有一个清单 4-9 中的函数，这里再次将其展示在下面清单 10-25 中，这是个不带生命周期注解就被编译的函数。


文件名：`src/main.rs`

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
```

*清单 10-25：在清单 4-9 中曾定义的一个不带生命周期注解即被编译的函数，即使其参数与返回值均为引用变量*


此函数不带生命周期注解即会编译的原因，是历史遗留的：在 Rust 早期版本（`pre-1.0`）中，由于每个引用都需要显式生命周期，因此该代码就不会编译。那个时候，该函数签名就会被写成下面这样：

```rust
fn first_word<'a>(s: &'a str) -> &'a str {
```

在编写了许多的 Rust 代码之后，Rust 团队发现，Rust 程序员们在某些特定情形下，会一次又一次地敲入许多同样的生命周期注解。而这些特定情形，是可被预测的，并遵循了少数几种确定性的模式（a few deterministic patterns）。Rust 开发者们于是就将这些模式，编程进了编译器的代码，于是借用检查器，就可以推断出这些情形下的生命周期，而无需显式的注解了。

由于将来可能合并更多确定性的模式，并将这些模式添加到编译器，因此 Rust 的这段历史是有联系的。在将来，或许就只要求更少甚至没有生命周期注解了。

编程到 Rust 的引用分析中的那些确定性模式，被称为 *生命周期省略规则（lifetime elision rules）*。这些规则并非 Rust 程序员要遵循的规则；他们是一套编译器要考虑的特殊情形，并在咱们编写的代码符合这些情形时，就无需显式地写出生命周期（注解）。

这些省略规则，并不提供完全的推断。在 Rust 明确地应用了这些规则，但仍有着哪些引用有何种生命周期方面的模糊性时，那么编译器是不会就其余引用变量应有何种生命周期，加以猜测的。编译器将给到某个可通过添加生命周期注解，而予以消除的错误消息，而非对模糊的引用生命周期胡乱猜测。

在函数或方法参数上的生命周期，被称为 *输入生命周期（input lifetimes）*，而在返回值上的生命周期，则被称为 *输出生命周期（output lifetimes）*。

在没有显式的生命周期注解时，编译器会用到三条规则，来计算出那些引用的生命周期。首条规则应用于输入生命周期，而第二及第三条规则，则是都应用于输出生命周期。在编译器抵达这三条规则的结尾处，而仍有其未能计算出生命周期的引用时，那么编译器就会以某个错误消息而停止。这三条规则适用于 `fn` 定义，对于 `impl` 代码块也一样适用。

首条规则即为，编译器后给那些是引用的各个参数，分别指派一个生命周期参数。也就是说，带有一个参数的函数，就会获得一个生命周期参数：`fn foo<'a>(x: &'a i32)`；而有着两个参数的函数，就会得到两个单独的生命周期参数：`fn foo<'a, 'b>(x: &'a i32, &'b i32)`；如此等等。

而第二条规则，则是在确切地只有一个输入生命周期参数时，那个生命周期，就被指派给全部的输出生命周期参数：`fn foo<'a>(x: &'a i32) -> &'a i32`。

第三条规则，在有多个输入生命周期参数，但由于这是个方法，而其中之一为 `&self` 或 `&mut self` 时，那么 `self` 的生命周期就会被指派给全部输出生命周期参数。由于只有较少必要符号，因此这第三条规令到方法的阅读与编写体验更佳。

下面咱们就来扮着编译器。这里将应用这些规则，来计算出清单 10-25 中，那个 `first_word` 函数签名里各个引用的生命周期。该函数签名以不带任何与其中引用关联的生命周期开始：

```rust
fn first_word(s: &str) -> &str {
```

随后编译器就应用首条规则，这会指定各个参数获取到其各自的生命周期。这里将和平时一样，将该生命周期叫做 `'a`，那么现在该函数签名就是这样的：

```rust
fn first_word<'a>(s: &'a str) -> &str {
```

由于这里确切地是一个输入生命周期，那么第二条规则就应用了。第二条规则指出，这个输入参数的生命周期，会被指派给输出生命周期，那么现在这个函数签名就是这样的：

```rust
fn first_word<'a>(s: &'a str) -> &'a str {
```

现在这个函数签名中的全部引用，都有了生命周期，进而编译器就可以在无需程序员对此函数签名中的生命周期进行注解的情况下，而继续其分析了。

接下来就要看看另一个示例，这次要使用在清单 10-20 中，一开始编写时没有生命周期参数的那个 `longest` 函数：

```rust
fn longest(x: &str, y: &str) -> &str {
```

首先来应用第一条规则：各个参数都得到其自己的生命周期。这次有两个而非一个参数，那么这里就有两个生命周期：

```rust
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
```

可以看到，由于这里有多于一个的输入生命周期，因此第二条规则并不使用了。至于第三条规则，由于 `longest` 是个函数而非方法，那么这些参数中没有一个是 `self`，因此第三条规则也不适用。在历经了全部三条规则后，这里仍未计算出返回值类型的生命周期为何物。这就是为何在尝试编译清单 10-20 中代码时收到错误的原因：编译器历经这些生命周期省略规则，而仍未计算出该函数签名中引用的全部生命周期。

由于这第三条规则，真的只适用于方法签名中，这里接下来就要看看，在方法语境中的生命周期，从而发现这第三条规则，就意味着不必甚为频繁地在方法签名中对生命周期进行注解。


### 方法定义中的生命周期注解

前面在有着生命周期的结构体上实现方法时，就使用了与清单 10-11 中泛型参数同样的语法。其中根据生命周期是否与结构体字段，或与方法的参数及返回值相关，而声明并使用到生命周期参数。

由于这些生命周期为结构体类型的部分，因此结构体字段的生命周期名字，总是要声明在 `impl` 关键字之后，并随后要在结构体名字之后使用。

在 `impl` 代码块里头的方法签名中，这些引用可能会被捆绑到结构体字段中引用的生命周期，或这些引用也可能是独立的。此外，生命周期省略规则通常会发挥作用，从而在方法签名中，生命周期注解就不是必要的了。下面就来看看一些使用了清单 10-24 中曾定义的名为 `ImportantExcerpt` 结构体的示例。

首先，这里将使用一个名为 `level`、其唯一参数是个到 `self` 引用，且返回值是个 `i32` 、不是到任何变量引用的方法：

```rust
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
```


在 `impl` 之后的生命周期参数声明，及在类型名字之后其的使用，都是必须的，但由于那首条省略规则，这里就未要求对到 `self` 引用的生命周期进行注解。

下面是个其中适用第三条生命周期省略规则的示例：

```rust
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println! ("请注意：{}", announcement);
        self.part
    }
}
```

这里有两个输入生命周期，那么 Rust 就会适用首条生命周期省略规则，并赋予到 `&self` 与 `announcement` 其各自的生命周期。随后由于其中一个参数是 `&self`，那么返回值类型就会得到 `&self` 的生命周期，进而全部生命周期就都得到了计算。


### 静态生命周期

这里需要讨论的一种特俗生命周期，那就是 `'static`，这表示了受其影响的引用，*可以* 存活到程序整个持续时间。所有字符串字面值，都有着 `'static` 的生命周期，可将这种特性注解为下面这样：

```rust
let s: &'static str = "我有静态的生命周期。";
```

此字符串的文本，是直接存储在该程序二进制数据中的，而程序二进制数据则是一直可用的。因此，所有字符串字面值的生命周期，就是 `'static`。

在一些错误消息中，或许会看到使用 `'static` 生命周期的建议。不过在将 `'static` 指定为某个引用的生命周期之前，请想一下手头的这个引用，是不是真的会存活到整个程序的生命周期，以及是否想要这个引用存活到程序的整个生命周期。多数时候，某个建议 `'static` 生命周期的错误消息，都是由尝试创建悬空引用，或可行的生命周期不匹配造成的。在这些情况下，解决办法是修复这些问题，而不是指定这个 `'static` 生命周期。


### 泛型参数、特质边界与生命周期三位一体

**Generic Type Parameters, Trait Bounds, and Lifetimes Together**


下面就来简要地看看，在一个函数中，同时指定出全部的泛型参数、特质边界与生命周期的语法！

```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println! ("通知！{}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

这就是清单 10-21 中，返回两个字符串切片中较长者的那个 `longest` 函数。不过现在他有了个名为 `ann`、泛型 `T` 的额外参数，这个泛型 `T` 可填入任何的、实现了那个由 `where` 子句所指定的 `Display` 特质的类型。这个额外参数，将被使用 `{}` 打印出来，这就是 `Display` 特质作为必要的原因。由于生命周期是一种通用类型，因此这里的生命周期参数 `'a` 与泛型参数 `T`，就处于函数名称之后尖括号内部的同一清单里头。


## 本章小结

在这一章中，涉及到了很多东西！既然了解了泛型参数、特质与特质边界，以及通用生命周期参数，那么就算准备好编写在许多不同情形下，不带有重复的工作代码了。泛型参数实现了将代码应用于不同类型。特质与特质边界则确保了即使类型是通用的，他们仍将有着代码所需的行为。咱们还掌握了运用生命周期注解，来保证这样的灵活代码不会有任何的悬空引用。而全部的这种分析，都是发生在编译时，这样的特性未对运行时性能造成影响！

不论相信与否，本章中讨论到的这些话题，要掌握的东西远不止这些：第 17 章会讨论特质对象（trait objects），那是另一种运用特质的方式。同时则还有更多只会在甚为高级场合，会需要用到生命周期注解的复杂场景；为着这些目的，就要阅读一下 [Rust 指南](https://doc.rust-lang.org/reference/index.html)。不过接下来，就会掌握怎样编写 Rust 中的测试，这样就可以确保所咱们的代码，以其应该有的方式工作。
