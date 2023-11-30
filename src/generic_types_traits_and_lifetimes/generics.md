# 通用数据类型

**Generic Data Types**

这里会使用泛型，来创建诸如函数签名或结构体等的定义，随后咱们便可以将这些定义，用于许多不同的具体数据类型。首先咱们来看看，怎样运用泛型特性来定义函数、结构体、枚举及方法等。接下来就会讨论到，泛型如何影响到代码性能。


## 函数定义方面

**In Function Definitions**


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


## 结构体定义方面

**In Struct Definitions**


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


## 枚举定义方面

**In Enum Definitions**


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


## 方法定义方面

**In Method Definitions**


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


## 用到泛型代码的性能问题

**Performance of Code Using Generics**


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



