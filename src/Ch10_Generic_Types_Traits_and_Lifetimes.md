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

当在函数体中运用某个参数时，就必须在函数签名中声明这个参数，如此编译器就知道那个名字表示什么。与此类型，在函数签名中使用某个类型参数时，就必须在使用该类型参数之前，对这个类型参数进行声明。要定义这个同样的 `largest` 函数，就要那些类型名字声明，放在尖括号（`<>`）内部，位于函数名字与参数列表之间，如下所示：

```rust
fn largest<T>(list: &<T>) -> &T {
```

要将这个定义读作：函数 `largest` 对某些类型 `T` 通用（the function `largest` is generic over some type `T`）。该函数有着一个名为 `list` 的参数，即类型 `T` 值的一个切片。`largest` 函数将返回一个到同样类型 `T` 值的引用。

下面清单 10-5 给出了这个在其签名中，运用通用数据类型的合并 `largest` 函数定义。这个清单还展示了怎样使用 `i32` 值切片，或 `char` 值切片，调用该函数。请注意此代码尚不会编译，但本章后面就会修复他。

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

现在即刻编译此代码，将得到如下错误信息：

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

该帮助文本消息，提供了 `std::cmp::PartialOrd`，这是一个 *特质（trait）*，而在下一小节，就会讲到特质。至于现在，明白该错误表示，`largest` 函数体，由于 `T` 可以是的那些全部可能类型而不会工作就行。由于这里是要在该函数体中，比较那些类型 `T` 的值，因此这里就只能使用值可被排序的那些类型。而要让这些值的比较可行，标准库就有着这个可应用在类型上的 `std::cmp::PartialOrd` 特质（请参阅附录 C 了解该特质的更多信息）。按照该帮助信息的建议，这里就要将这些类型限制为对 `T` 有效的、仅那些实现了 `PartialOrd` 的类型，而由于标准库在 `i32` 与 `char` 上，均实现了 `PartialOrd` 特质，那么这个示例就会编译了。


### 在结构体定义中

这里还可以将结构体，定义为在其一个或多个字段中，运用 `<>` 语法来使用泛型参数。清单 10-6 定义了一个 `Point<T>` 的结构体，来保存任意类型的 `x` 与 `y` 坐标值。

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

在结构体定义中使用泛型特性的语法，与在函数定义中用到的类型。首先，这里在紧接着结构体名字之后的尖括号内部，声明了类型参数名字。随后在结构体定义中，在哪些原本会指明具体数据类型的地方，使用了该泛型。

请注意由于这里之用的一个泛型来定义 `Point<T>`，此定义是说，这个 `Point<T>` 结构体对某些类型 `T` 通用，且字段 `x` 与 `y` *均为* 那种同样类型，而不论那种类型可能是何种类型。在创建一个有着不同类型值的 `Point<T>` ，即下面清单 10-7 中那样，那么这里的代码就不会编译。

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

在此示例中，当这里将整数值 `5` 赋值给 `x` 时，那么就让编译器明白了该 `Point<T>` 实例的泛型 `T`，将是个整数。那么随后在将 `4.0` 指定给那个已被定义为与 `x` 有着同一类型的 `y` 时，就会得到一个像下面这样的类型不匹配错误：

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

要定义一个其中 `x` 与 `y` 同时为泛型，而又可以有着不同类型的 `Point` 结构体，就可以使用多个泛型参数。比如，在下面清单 10-8 中，就将 `Point` 的定义，修改为了对类型 `T` 与 `U` 通用，其中 `x` 为类型 `T`，而 `y` 则是类型 `U`。

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

现在上面给出的全部 `Point` 实例，都是允许的了！在某个定义中，可以使用想要的泛型参数个数，不过多使用几个就会令到代码难于阅读。在发现代码中需要很多种泛型时，那就可能表示所编写的代码，需要重构为更小的片段了。


### 在枚举的定义中

如同对结构体所做的那样，可将枚举定义为在他们的变种中，保存一些通用数据类型。下面就来换个角度看看，标准库提供的那个曾在第 6 章中用到的 `Option<T>`：

```rust
enum Option<T> {
    Some(T),
    None,
}
```

这个定义，现在应有着更多意涵了。可以看到，这个 `Option<T>` 枚举对类型 `T` 是通用的，并有两个变种：保存着一个类型 `T` 值的 `Some`，与一个不保存任何值的 `None` 变种。通过使用这个 `Option<T>` 枚举，就可以表达可选值的抽象概念，并由于 `Option<T>` 是通用的，因此就可以在无关乎该可选值为何种类型下，使用这种抽象概念。

枚举也可以使用多个泛型。在第 9 章中用到的 `Result` 枚举定义，就是一个示例：

```rust
enum Result <T, E> {
    Ok(T),
    Err(E),
}
```

这个 `Result` 对两种类型通用，`T` 与 `E`，并有着两个变种：保存了一个类型 `T` 值的 `Ok`，与保存了一个类型 `E` 值的 `Err`。这个定义使得在某个操作可能成功（便返回某种类型 `T` 的一个值），或失败（便返回一个某种类型 `E` 的值）的地方，使用这个 `Result` 枚举方便起来。事实上，这就是在清单 9-3 中打开某个文件时所用到的，在文件被成功打开时，其中的 `T` 就用 `std::fs::File` 填充上了，而在打开那个文件时存在某些问题时，那么其中的 `E` 就以 `std::io::Error` 填充。

在意识到代码中有着多个仅在其所保存值类型上，有区别的结构体或枚举，这样的一些情况时，就可以通过使用泛型，而避免代码重复。


### 在方法定义中

在结构体与枚举上，可以实现一些方法（正如在第 5 章中所做的那样），并也可以在这些方法的定义中使用泛型。下面清单 10-9 展示了在清单 10-6 中所定义的那个 `Point<T>` 结构体，其上实现了个名为 `x` 的方法。


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

*清单 10-9：在 `Point<T, U>` 结构体上实现一个将返回到类型 `T` 的 `x` 字段引用的名为 `x` 的方法*

这里已在 `Point<T, U>` 上定义了一个名为 `x` 的、返回到字段 `x` 中数据一个引用的方法。通过在 `impl` 后将 `T` 声明为泛型，Rust 就可以识别到 `Point` 中的尖括号（`<>`） 里的类型为一个泛型，而非具体类型。对于这个泛型参数，这里是可以选择一个不同于前面结构体定义中的泛型参数的，但使用同样的名字在这里是惯例性的。在某个声明了泛型的 `impl` 里头编写的那些方法，不论泛型最终将以何种具体类型所代替，这些方法都将被定义在该类型的任意实例上。

在将一些方法定义在类型上时，还可以在这些泛型上指明一些约束条件。比如这里就可以只在 `Point<f32>`，而不是有着任意泛型的 `Point<T>` 实例上实现方法。在下面清单 10-10 中，就使用了具体类型 `f32`，即指在 `impl` 之后没有声明任何类型。

文件名：`src/main.rs`

```rust
impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

*清单 10-10：一个只适用于有着对于泛型参数 `<T, U>` 的某种特定具体类型结构体的 `impl` 代码块*

此代码表示类型 `Option<f32, f32>` 将有着一个 `distance_from_origin` 方法；别的其中 `T, U` 不是 `f32` 的 `Option<T, U>` 实例，就不会有这个定义的方法。该方法度量了这个点与坐标 `(0.0, 0.0)` 处点的距离，并使用了只对浮点数类型可以的数学运算。

结构体定义中的泛型参数，并不总是与在同一结构体方法签名中所用的那些泛型参数相同。下面清单 10-11 就对 `Point` 结构体使用了泛型 `T` 与 `U`，对 `mixup` 方法签名，则使用了 `X Y`，来让这个示例更明显。该方法以来自 `self` `Point` 的 `x` 值（类型为 `T`），与来自传入的 `Point` 值的 `y` （类型为 `Y`），创建了一个新的 `Point`。

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

在 `main` 函数中，这里已定义了一个有着 `x` 为 `i32` （值为 `5`），及 `y` 为 `f64` （值为 `10.4`）的 `Point`。其中的变量 `p2` 是个有着 `x` 为字符串切片（值为 `Hello`），同时 `y` 为 `char` （值为 `c`）的 `Point` 结构体。以 参数 `p2` 调用 `p1` 上的 `mixup`，给到了 `p3`，由于 `p3` 的 `x` 来自于 `p1`，因此将有一个 `i32` 的 `x`。而由于这个变量 `p3` 的 `y` 来自于 `p2`, 因此他将有一个 `char` 的 `y`。那个 `println!` 宏调用，将打印 `p3.x = 5, p3.y = c`。

此示例的目的，是要对其中有些泛型参数是以 `impl` 来声明，而另一些泛型参数则是以方法定义来声明的一种情形，加以演示。由于这里的泛型参数 `T` 与 `U`与结构体定义在一起，因此他们是在 `impl` 之后声明的。而其中的泛型参数 `X` 与 `Y`，由于他们只与那个方法 `mixup` 有关，因此他们就被声明在了 `fn mixup` 之后。


### 使用泛型参数代码的性能问题


这里或许想了解，在运用了泛型参数时，是否有着运行时的开销。好消息就是，相比于使用具体类型，使用泛型并不会令到程序运行得更慢。

Rust 通过在编译时，完成那些使用了泛型代码的单态化（performing monomorphization of the code using generics），实现了这一点。所谓 *单态化（monomorphization）*，即通过将在编译后用到的具体类型填入进去，而将通用代码转换为具体代码的过程。在此过程中，编译器会执行与清单 10-5 中曾创建通用函数相反的步骤：编译器会查看所有泛型代码被调用到的地方，并生成调用到泛型代码的那些具体类型的代码。

下面就来通过使用标准库的通用 `Option<T>` 枚举，看看单态化的工作原理：

```rust
let integer = Some(5);
let float = Some(5.0);
```

在 Rust 编译此代码时，他就会执行单态化。在那个过程中，编译器会读取已在这两个 `Option<T>` 实例中用到的值，并识别到两种类型的 `Option<T>`：一个为 `i32`，同时另一个为 `f64`。就这样，编译器会将 `Option<T>` 的通用定义，展开为两个专门用于 `i32` 与 `f64` 的定义，由此就用这些特定类型，对通用定义进行了替换。

单态化后版本的该代码，看起来与下面的类似（编译器会使用不同于这里为演示目的而使用的名字）：


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

这个通用的 `Option<T>`，就被以编译器创建的具体定义给替换掉了。由于 Rust 将通用代码，编译为指明了各个实例中类型的代码，那么就不会为运用泛型，而付出运行时代价。在运行的时候，代码就只是会与原本早先手写的各个重复定义一样执行。单态化的过程，令到 Rust 的泛型特性，在运行时极为高效。


## 特质：定义共用行为

*特质（a trait）*，定义了某种特定类型所具有的，并可与其他类型共用的功能。使用特质，就可以抽象方式，来定义共用行为。而使用 *特质边界（trait bounds）*，就可以指明具有特定行为的任意类型的泛型（we can use *trait bounds* to specify that a generic type can be any type that has certain behavior）。


> **注意**：特质与其他语言中名为 *接口（interfaces）* 的特性类似，不过有着一些不同之处。


### 定义一个特质

某个类型的行为，是由那些可在该类型上调用的方法，所组成的。在能够于不同类型之上，调用一些同样方法时，那么这些不同类型，就共用了同样的行为。特质定义，就是为了完成某种目标，而定义一套必要行为，为此而将一些方法签名组织在一起的一种方式（trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose）。

比如说，这里有着保存了多种类别与数量文本的多个结构体：一个 `NewsArticle` 结构体，保存着归档于特定位置的新闻故事，而一个 `Tweet` 结构体，则可以有着最多 280 个字符，并带有表明其是否为一则新 tweet、或 retweet，抑或是到另一 tweet 答复的元数据。

这里打算构造一个名为 `aggregator` 的媒体聚合器库代码箱，可以显示出可能存储在某个 `NewsArticle` 或 `Tweet` 实例中数据的一些摘要信息来。要完成这个目的，就需要每种类型的摘要，并将通过调用实例上的 `summarize` 方法，请求到摘要信息。下面清单 10-12 给出了表示此行为的一个公共 `Summary` 特质。


文件名：`src/lib.rs`


```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

*清单 10-12：由 `summarize` 方法提供的行为所组成的一个 `Summary` 特质*

这里使用 `trait` 关键字，及接下来该特质的名字，即此示例中的 `Summary`，声明了一个特质。这里同时将该特质声明为了 `pub`，如此就会跟在接下来将看到的几个示例中那样，那些依赖于此代码箱的其他代码箱，也可利用上这个特质。在花括号里面，就要声明对那些实现了此特质的类型行为加以描述的方法签名，在此示例中，那些方法签名即为 `fn summarize(&self) -> String`。

在方法签名之后，与提供出花括号里的方法实现不同，这里使用了一个分号。实现此特质的各个类型，都必须提供其自己的定制行为，作为该方法的方法体。编译器会强制要求，有着这个 `Summary` 特质的任意类型，都将有着与这个以该签名所定义的，完全一致的 `summarize` 方法。

特质在其代码体中，可以由多个方法：这些方法签名一行一个地列出来，同时每行都已分号结束。


### 在类型上实现某个特质

既然前面已经定义了所需的那个 `Summary` 特质的那些方法的签名，那么就可以将其在此处媒体聚合器中的类型上，加以实现了。下面清单 10-13 就给出了这个 `Summary` 特质，在 `NewsArticle` 结构体上，使用标题、作者以及处所字段来创建 `summaryize` 方法返回值的一种实现。而对于 `Tweet` 结构体，这里是将 `summarize` 定义作在假定推文已被限制为 280 字符情况下，返回用户名加上推文的全部文字。

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

*清单 10-13：在 `NewsArticle` 与 `Tweet` 上对 `Summary` 进行实现*


在某个类型上实现一个特质，与实现常规方法类似。不同之处为，在 `impl` 关键字后面，要放置那个打算实现的特质名字，然后要使用 `for` 关键字，并随后指定那个打算为其实现特质的类型名字。在这个 `impl` 代码块内部，就要放入前面特质定义所定义出的那些方法签名。这里不再是在各个签名之后添加分号，而是使用花括号，并将这里打算要特定类型所具有该特质的方法，填充到该方法的方法体中。

既然这个库已经在 `NewsArticle` 与 `Tweet` 上实现了那个 `Summary` 特质，那么该库代码箱的用户，就可以调用常规方法的同样方式，调用 `NewsArticle` 与 `Tweet` 实例上的这些特质方法了。唯一区别就是，用户必须将该特质，与相关类型，同时带入到作用域中。下面就是某个二进制代码箱，如何使用这里的 `aggregator` 库代码箱的示例：


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

依赖于 `aggregator` 代码箱的其他代码箱，同样可以将 `Summary` 特质带入其作用域，来将 `Summary` 实现在他们自己的类型上。
要注意有个限制，就是只有在特质或类型属于本地代码箱时，才能将这个特质应用于这个类型上。比如，由于 `Tweet` 这个类型属于本地代码箱 `aggregator`，那么因此就可以将像是标准库的特质 `Display`，应用于像是定制类型 `Tweet` 上，而作为这个 `aggregator` 代码箱功能的一部分。由于那个 `Summary` 属于 `aggregator` 代码箱本地，因此在这里的 `aggregator` 代码箱中，还可将 `Summary` 应用在 `vec<T>` 上。


但这里是无法将外部特质，应用在外部类型上的。比如，由于 `Display` 特质与 `vec<T>` 类型都是定义在标准库中，而均不属于 `aggregator` 代码箱本地，那么在这里的 `aggregator` 代码箱里头，就不能够将 `Display` 特质，应用在 `vec<T>` 上。这种限制属于一种名为 *内聚性（coherrnce）* 属性的一部分，更具体地讲，就是 *孤儿规则（the orphan rule）*，之所以这样称呼，是由于未父类型缺席了（this restriction is part of a property called *coherence*, and more specifically the *orphan rule*, so named because the parent type is not present）。此规则确保其他人的代码无法破坏自己的代码，反之亦然。在没有这条规则下，两个代码箱就会将同一特质，对同一类型加以实现，那么 Rust 就不清楚要使用那个实现了。


### 默认实现

在有些时候，给特质中一些或全部方法以默认行为，而非要求在每种类型上实现全部方法，会是有用的做法。随后，在某个特定类型上实现该特质时，就可以保留或重写各个方法的默认行为。

下面清单 10-14 就给那个 `Summary` 特质的 `summarize` 方法，指定了一个默认字符串，而非如同在清单 10-12 中所做的，仅定义该方法签名。

文件名：`src/lib.rs`

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("了解更多......")
    }
}
```

*清单 10-14：定义一个带有 `summarize` 方法默认实现的 `Summary` 特质*

而要使用默认实现来对 `NewsArticle` 的实例进行摘要，就要以 `impl Summary for NewsArticle {}`，指明一个空的 `impl` 代码块。

即便这里不再于 `NewsArticle` 类型上，直接定义那个 `summarize` 方法，这里仍已提供到一个默认实现，并已指明了 `NewsArticle` 类型实现了 `Summary` 特质。由此，这里就可以在某个 `NewsArticle` 实例上，调用这个 `summarize` 方法，如同下面这样：

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

默认实现的创建，不要求对清单 10-13 中在 `Tweet` 上 `Summary` 的实现，做任何的修改。原因就是对某个默认实现进行重写这种语法，与实现某个不具有默认实现的特质方法的语法，是相同的。

默认实现可调用同一特质中的其他方法，即使这些别的方法没有默认实现。以这样的方式，特质就可以提供到很多有用功能，而只要求实现者指明特质的一小部分。比如这里就可以将这个 `Summary` 特质，定义为有着一个要求予以实现的 `summarize_author` 方法，并随后定义了一个有着调用了该 `summarize_author` 方法的一个默认实现 `summarize` 方法：

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format! ("（了解更多来自 {} ......）", self.summarize_author())
    }
}
```

而要使用此版本的 `Summary`，就只需在某个类型上实现该特质时，对 `summarize_author` 加以定义：

```rust
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format! ("@{}", self.username)
    }
}
```

在定义了 `summarize_author` 之后，就可以在 `Tweet` 结构体的实例上调用 `summarize` 方法，同时 `summarize` 的默认实现，将调用这里已经提供到的 `summarize_author` 的定义。由于这里已经实现了 `summarize_author`，那么在无需要求咱们编写任何其他代码之下，这个 `Summary` 特质就已给到 `summarize` 方法的行为了。

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

此代码会打印出 `1 条新推文: （了解更多来自 @horse_ebooks ......）`。

请注意从某个方法的重写实现，是无法访问该同一方法的默认实现的。


### 作为参数的特质

既然咱们清楚了怎样定义和实现特质，那么就可以探索一下，怎样使用特质来定义接受不同类型参数的函数了。这里将使用之前清单 10-13 中，定义在 `NewsArticle` 与 `Tweet` 上的那个 `Summary` 特质，来定义一个调用其 `item` 参数上 `summarize` 方法的 `notify` 函数，其中 `item` 参数是某个实现了 `Summary` 特质的类型。要完成这一点，就要使用到 `impl Trait` 语法，如下所示：

```rust
pub fn notify(item: &impl Summary) {
    println! ("突发新闻！{}", item.summarize());
}
```

这里给那个 `item` 参数指定了 `impl` 关键字和特质名字，而不再是具体类型。此参数将接受实现了指定特质的任何类型。在 `notify` 的函数体中，就可以调用 `item` 上来此 `Summary` 特质的全部方法，比如这里的 `summarize`。这里便可以对 `notify` 加以调用，并传入任意的 `NewsArticle` 或 `Tweet` 实例了。以任意其他类型，比如 `String` 或 `i32`，由于这些类型没有实现 `Summary`，那么对该函数进行调用的代码，就不会编译。


### 特质边界语法

这种适用于简单案例的 `impl Trait` 语法，实际上是一种被称作 *特质边界（a trait bound）* 较长形式的语法糖；而特质边界看起来像下面这样：


```rust
pub fn notify<T: Summary>(item: &T) {
    println! ("突发新闻！{}", item.summarize());
}
```

这种较长形式与上一小节中的示例是等价的，但要更冗长一些。这里是将特质边界（`Summary`），在一个冒号之后，与泛型参数声明放在了一起，并在一对尖括号里面。

`impl Trait` 这种语法，在简单情形下，是方便的，且令到代码更为简洁，而在别的情形下，较完整的特质边界语法，则能够对更高复杂度进行表达。比如，这里可以有着两个实现了 `Summary` 的参数。以 `impl Trait` 语法实现这种情况，看起来就像下面这样：

```rust
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
```

在想要此函数容许 `item1` 与 `item2` 有着不同类型（只要这两种类型都实现了 `Summary` 即可）。而在要强制这两个参数有着同一类型时，那么就必须使用特质边界，像下面这样：

```rust
pub fn notify<T: Summary>(item1: &T, item2: &T) {
```

这里被指定为 `item1` 与 `item2` 两个参数类型的泛型 `T`，对该函数进行了约束，进而作为传递给 `item1` 与 `item2` 的参数值具体类型，就必须相同了。


### 使用 `+` 语法，指定多个特质边界

这里还可以指定多于一个的特质边界。比方说这里打算的是 `notify` 要在 `item` 上使用 `summarize` 方法的同时，还会用到一些显示的格式化：那么就会在 `notify` 的定义中，指明 `item` 必须同时实现了 `Disply` 与 `Summary` 两个特质。使用 `+` 语法，就可以达到这个目的：

```rust
pub fn notify(item &(impl Summary + Display)) {
```

`+` 语法同样对泛型上的特质边界有效：


```rust
pub fn notify<T: Summary + Display>(item: &T) {
```

在制定了这两个特质下，`notify` 的函数体，就可以调用 `summarize` 函数，并可以使用 `{}` 来对 `item` 进行格式化了。


### 运用 `where` 子句获得更明确的特质边界

**Clearer Trait Bounds with `where` Clauses**

过多特质边界的使用，有着其负面性。每个泛型都有其自己的特质边界，因此带有多个泛型参数的函数，就会在函数名字与其参数列表之间，包含很多的特质边界信息，从而令到该函数签名难于阅读。由于这个原因，Rust 就有了在函数签名之后的一个 `where` 子句里头，指定特质边界的这样一种替代性语法。从而与其像下面这样编写函数签名：

```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```

就可以使用 `where` 子句，写成下面这样：

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
```


这样的函数签名，就不那么杂乱无章了：函数名、参数列表与返回值类型紧挨在一起，与未带有很多特质边界的某个函数类似。


### 实现了特质的返回值类型

在返回某种实现了某个特质的类型值的返回值处，也可以使用 `impl Trait` 语法，如下所示：


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

这里通过在返回值类型上使用 `impl Summary`，在没有命名具体类型下，而指明了这个 `returns_summarizable` 函数，会返回某种实现了 `Summary` 特质的类型。在此示例中，`returns_summarizable` 函数返回的是一个 `Tweet`，而调用此函数的代码，则无需知会这一点。

仅由返回值类型所实现的特质，指定处返回值类型的这种能力，在闭包与迭代器，在第 13 章就会涉及到这两个特性，的语境下尤为有用。闭包与迭代器创建了仅编译器知晓的一些类型，或一些长度极大而无法指定的类型。这种 `impl Trait` 语法，实现了简明地指定处返回实现了 `Iterator` 特质的某种类型，而无需编写出那非常长的类型。

然而，只能在返回单个类型时，才能使用 `impl Trait`。比如，下面有着将返回值类型值指定为了 `impl Summary`，而既要返回 `NewsArticle` 又要返回 `Tweet` 的代码，就不会工作：

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

这里不允许既返回 `NewsArticle` 又返回 `Tweet`，是由于有关这个 `impl Trait` 语法，在编译器中实现方式的限制。在第 17 章的 [运用允许不同类型值的特质对象](Ch17_Object_Oriented_Programming_Features_of_Rust.md#使用允许不同类型值的特质对象) 小节，就会讲到怎样编写有着这种行为的函数。


### 运用特质边界来有条件地实现方法

经由运用有着一个使用泛型参数的 `impl` 代码块的特质边界，就可以根据实现了指定特质的类型，而实现不同方法（by using a trait bound with an `impl` block that uses generic type parameters, we can implement methods conditionally for types that implement the specified traits）。比如下面清单 10-15 中的类型 `Pair<T>`，就一直会将那个 `new` 函数，实现为返回 `Pair<T>` 的一个新实例（回顾第 5 章的 [定义方法](Ch05_Using_Structs_to_Structure_Related_Data.md#方法的定义) 小节就知道，`Self` 就是那个 `impl` 代码块的类型别名，在这个示例中即为 `Pair<T>`）。但在接下来的 `impl` 代码块中，在 `Pair<T>` 的内部类型 `T` 实现了启用比较的 `PartialOrd` 特质，*与* 启用打印的 `Display` 特质时，那么 `Pair<T>` 就只会实现 `cmp_display` 方法。


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

*清单 10-15：根据特质边界，而有条件地实现泛型上的方法（conditionally implementing methods on a generic type depending on trait bounds）*

> **注意**：这里的 `new` 是个关联函数，而非方法！只能以 `Pair::new` 形式使用。要作为方法使用，函数就必须要有一个 `self` 参数。

还可以对任意实现了另一特质的类型，有条件地实现某个特质。在任意满足某些特质边界的类型上，某个特质的实现，被称作 *一揽子实现（blanket implementations）*，这样的做法在 Rust 标准库中有广泛使用。比如，标准库就在实现了 `Display` 特质的全部类型上，实现了 `ToString` 这个特质。标准库中这个 `impl` 代码快，看起来与下面的类似：

```rust
impl<T: Display> ToString for T {
    // --跳过代码--
}
```

由于标准库有着这样的一揽子实现，那么就可以在实现了 `Display` 特质的全部类型上，调用由 `ToString` 特质所定义的 `to_string` 方法。比如，由于正是实现了 `Display` 特质，那么这里就可以像下面这样，把整数转换为他们对应的 `String`：

```rust
let s = 3.to_string();
```

一揽子实现，会出现在特质文档的 “相关实现器（Implementors）” 部分。


### 小结

特质与特质边界这两个特性，允许咱们编写运用了泛型参数的代码，从而在减少代码重复的同时，还向编译器指出希望该泛型有着特定行为。随后编译器就会使用特质边界信息，来就代码所用到的全部具体类型，是否提供到正确行为进行检查。在一般的动态类型语言（dynamically typed languages）中，若调用某个类型上尚未定义的方法，那么就会在运行时收到错误。但 Rust 将这些错误，移到了编译时，这样就要在代码还不能运行的时候，就被强制要求修复这些问题。此外，由于已在编译时检查过这些问题，那么就不必编写对运行时行为进行检查的代码了。在不必放弃泛型灵活性之下，就提升了程序性能。


## 使用生命周期对引用加以验证

生命周期是另一种前面已经用到的泛型。与确保某种类型有着期望行为的特质不同，生命周期确保的是引用在需要他们有效期间，保持有效（lifetimes ensure that references are valid as long as we need them to be）。

在第 4 章中的 [引用与借用](Ch04_Understanding_Ownership.md#引用与借用references-and-borrowing) 小节，未曾讨论到的一个细节，就是在 Rust 中的全部引用都有着 *生命周期（lifetime）*，即引用有效的作用范围。多数时候，声明周期都是隐式的，且是被推导出来的，这正与多数时候类型是被推导出来的一样。在可能有多个类型时，仅务必对类型加以注解即可。与这种注解类型的方式类似，在引用的生命周期与少数几种不同方式相关时，就必须对生命周期加以注解。为确保在运行时用到的具体类型显著有效，Rust 就要求使用泛型生命周期参数，对这些关系加以注解（in a similar way, we must annotate lifetimes when the lifetimes of references could be related in a few different ways. Rust requires us to annotate the relationships using generic lifetime parameters to ensure the actual references used at runtime will definitely be valid）。

绝大多数别的编程语言，甚至都没有声明周期注解这个概念，那么这就会感觉陌生了。尽管本章不会涵盖生命周期的全部，这里仍会对可能遇到的生命周期语法的一些常见方式进行讨论，如此就会对此概念感到不那么违和。


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
