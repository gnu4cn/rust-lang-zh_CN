# 运用结构体来组织相关数据

**Using Structs to Structure Related Data**

*结构体（struct）*，或者说 *结构（structure）*，实现了将多个相关值打包在一起，并取个名字，而构成一个有意义的组别。在熟悉面向对象语言的情况下，那么 *结构体* 就像是对象的那些数据属性。在本章中，将把元组与结构体加以比照，从而在既有认识之上，构建出对结构体的认识，并对使用结构体作为一种更佳的数据组织方式的时机，进行演示。这里会对如何定义及初始化结构体进行演示。还会讨论如何定义关联函数，尤其是那种叫做 *方法* 的关联函数，来指明与某个结构体类型相关联的行为。结构体与枚举（将在第 6 章讨论到），这两种数据结构，是充分利用 Rust 的编译时类型检查特性，在程序域中创建新类型的构件。

## 结构体的定义及初始化

结构体与之前 [元组类型](Ch03_Common_Programming_Concepts.md#the-tuple-type) 小节中讨论过的元组数据结构类似，二者都保存着多个相关数据。和元组一样，结构体的各个数据片段可以是不同类型。与原则不同的是，在结构体中将给各个数据片段命名，如此各个值表示什么就清楚了。加上这些名字，就意味着相比于元组更为灵活了：不必为了给某个实例指定他的那些值，或要访问实例的那些值，而对实例数据的顺序有所依赖了。

要定义出一个结构体，就要敲入关键字 `struct`，及整个结构体的名字。结构体名字，应对安排在一起的这些数据片段的意义加以描述。随后，就要这一对花括号里头，定义出各个数据片段的名称与类型，这些数据片段，就叫做 *字段（fields）*。比如，下面的清单 5-1 就给出了一个保存用户账号信息的结构体。

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}
```

*清单 5-1：`User` 结构体的定义*

在定义出了结构体后，要用上这个结构体，就要通过给各个字段指定具体值，创建出那个结构体的 *实例（instance）* 来。通过指明结构的名字，并随后加上包含了 `key: value` 键值对的一对花括号，这样创建出一个实例来。键值对中的那些键，就是那些字段的名字，而其中的那些值，则是打算保存在这些字段中的数据。不必按照在结构体中声明那些字段的顺序，来对这些字段进行指明（we don't have to specify the fields in the same order in which we declared them in the struct）。也就是说，结构体定义就如同该类型的通用模板，而实例则将特定数据填充到那个木板中，从而创建出这个类型的值来。比如，就可如下面清单 5-2 中所展示的那样，声明出一个特定的用户来：

```rust
fn main() {
    let user1 = User {
        email: String::from("rust@xfoss.com"),
        username: String::from("unisko"),
        active: true,
        sign_in_count: 1
    };
}
```

*清单 5-2：创建出结构体 `User` 的一个实例来*

而要从结构体中获取到指定值，就要使用点表示法（`.`）。在要的仅是该用户的电子邮件地址时，就可以在那些要用到这个值的地方，使用 `user1.email` 。而在该实例为可变时，那么就可以通过使用点表示法，进而给特定字段赋值，而对某个值加以修改。下面的清单 5-3 展示了如何来修改某个可变 `User` 实例 `email` 字段中的值。

文件名：`src/main.rs`

```rust
fn main() {
    let mut user1 = User {
        email: String::from("rust@xfoss.com"),
        username: String::from("unisko"),
        active: true,
        sign_in_count: 1
    };

    user1.email = String::from("java@xfoss.com");
}
```

*清单 5-3：对某个 `User` 实例中的 `email` 字段进行修改*

请注意这整个实例必须是可变的；Rust 不允许仅将一些字段标记为可变。与所有表达式一样，可以函数体中最后的表达式形式，构造出结构体的新实例，来隐式地返回那个新实例（as with any expression, we can construct a new instance of the struct as the last expression in the function body to implicity return that new instance）。

下面的清单 5-4，展示了一个以给定电子邮件和用户名，返回一个 `User` 实例的 `build_user` 函数。其中的 `active` 字符会得到值 `true`，而那个 `sign_in_count` 则会得到值 `1`。

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
```

*清单 5-4：一个取得电子邮件和用户名，并返回一个 `User` 实例的 `build_user` 函数*

将函数参数命名为与结构体字段同样的名字，是有意义，但由此而不得不重复那 `email` 与 `username` 的字段名字与变量，就有点烦人了。在结构体有更多字段时，这样重复各个名字就会变得更加烦人。幸运的是，有种方便的简写法！


### 使用字段初始化简写法

由于在清单 5-4 中的参数名字与结构体字段名字完全一样，因此就可以 *字段初始化简写（field init shorthand）* 语法，来重写 `build_user` 方法，如此一来，`build_user` 函数在没有 `email` 与 `username` 重复的情况下，也有与之前版本同样的表现，如下清单 5-5 所示：

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

*清单 5-5：由于 `email` 与 `username` 参数与结构体字段有着同样名字，而使用了字段初始化简写的 `build_user` 函数*

在这里，正创建一个 `User` 结构体的新实例，该结构体有一个名为 `email` 的字段。这里打算将 `email` 字段的值，设置为 `build_user` 函数的 `email` 参数中的值。由于 `email` 字段与 `email` 参数有着同样的名字，因此只就需写下 `email`，而非 `email: email`。


### 使用结构体更新语法，从其他实例创建出实例

创建出包含另一实例绝大部分值，而修改一些值的新实例，通常是有用的做法。而使用 *结构体更新语法（struct update syntax）* 就能做到这点。

首先，在下面的清单 5-6 中展示了如何按常规，不使用更新语法的情况下，创建出在 `user2` 中的一个新 `User` 实例。这里给 `email` 设置了一个新的值，而在其他方面，则使用了来自之前在清单 5-1 中创建的 `user1` 的那些同样值。

```rust
fn main() {
    // --跳过代码--

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("java@xfoss.com"),
        sign_in_count: user1.sign_in_count,
    };
}
```

*清单 5-6：使用一个 `user1` 的值创建出一个新的 `User` 实例*

而使用结构体更新语法，就可以较少代码，达成同样效果，如下面的清单 5-7 中所给出的那样。其中的 `..` 语法，指明了未显式设置的其余字段，将有着与所给实例中的字段同样的值。

```rust
fn main() {
    // --跳过代码--

    let user2 = User {
        email: String::from("java@xfoss.com"),
        ..user1
    };
}
```

*清单 5-7：使用结构体更新语法来设置 `User` 实例的 `email` 字段值，而使用来自 `user1` 的其余值*

清单 5-7 中的代码同样创建了在变量 `user2` 中，一个有着 `email` 的不同值，但有着来自 `user1` 的 `username`、`active` 及 `sign_in_count` 同样值。其中的 `..user1` 必须要在最后，这样来指明全部剩余字段都应从 `user1` 中的相应字段获取值，但对于其他字段值的指定，则可选择所要的任意字段，以任意顺序进行，而不论在结构体定义中这些字段的顺序为何（the `..user1` must come last to specify that any remaining fields should get their values from the corresponding fields in `user1`, but we can choose to specify values for as many fields as we want in any order, regardless of the order of the fields in the struct's definition）。

请注意结构体更新语法，像赋值一样使用了 `=`；这是由于结构体更新语法迁移了数据，就跟在之前的 ["变量与数据互动方式：迁移"](Ch04_Understanding_Ownership.md#ways-variables-and-data-interact-move) 小节中看到的那样。在此示例中，在创建了 `user2` 之后，由于变量 `user1` 中的 `username` 字段中的 `String` 值，已被迁移到 `user2` 中了，因此就再也不能使用变量 `user1` 了。若给到 `user2` 的 `email` 及 `username` 字段都是新的 `String` 值，而因此只使用来自 `user1` 的 `active` 和 `sign_in_count` 值，那么在创建了 `user2` 之后，`user1` 仍将是有效的。因为 `active` 和 `sign_in_count` 的类型，都是实现了 `Copy` 特质的类型，因此就会应用在 [唯栈数据：拷贝](Ch04_Understanding_Ownership.md#stack-only-data-copy) 小节中的行为表现。


### 使用不带命名字段的元组结构体来创建不同类型

**Using Tuple Structs without Named Fields to Create Different Types**

Rust 还支持看起来像元组的结构体，叫做 *元组结构体（tuple structs）*。元组结构体这一类型，多了类型名称中结构体这一部分所提供的意义，却并没有与各字段相关联的名字；而是，元组结构体他们那些字段的类型。在要给予整个元组一个名字，并令到元组成为不同于其他元组的一种类型，且在如同在常规结构体中那样，给各个字段取名字是多余的等等，在这样的情况下，元组结构体就会有用。

要定义一个元组结构体，就要以 `struct` 关键字和该结构体的名字开头，接着是一些在元组中的类型。比如，下面分别定义和使用了两个元组结构体 `Color` 与 `Point`:

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);
    let origin = Point(0, 0, 0);
}
```

请注意，由于这里的 `black` 与 `origin` 两个值是不同元组结构体的实例，因此他们属于不同类型。尽管结构体里的那些字段有着同样类型，对于所定义每个结构体，都是其自身的类型。比如，某个接收类型 `Color` 参数的函数，就无法接收 `Point` 值做参数，尽管这两种类型都是由三个 `i32` 值构成的。除此之外，元组结构体的实例，与元组表现一样：可将他们解构为三个独立部分，可使用 `.` 后面跟上索引，来访问单独值，等等。


### 没有字段的类单元结构

**Unit-Like Structs Without Any Fields**

还可以定义没有任何字段的结构体！由于这些没有任何字段的结构体，与曾在 [元组类型](Ch03_Common_Programming_Concepts.md#the-tuple-type) 小节提到过的单元类型 `()` 表现类似，因此他们叫做 *类单元结构体（unit-like structs）*。当需要在某类型上实现某个特质（trait），却又不希望将任何数据存储在那个类型自身里面时，类单元结构体就就有用（unit-like structs can be useful when you need to implement a trait on some type but don't have any data that you want to store in the type itself）。在第 10 章就会讨论到特质。下面是一个声明和初始化名为 `AlwaysEqual` 的单元结构体的示例：

```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

要定义出 `AlwaysEqual`，就要使用 `struct` 关键字、想要的名字，随后一个分号即可。是不需要花括号或圆括号的！随后就可以类似方式，得到一个在 `subject` 变量中的 `AlwaysEqual` 的示例了：使用定义的名字，不带任何花括弧或原括弧。设想稍后就要将此类型的表现，实现为每个 `AlwaysEqual` 的实例，总是等于任何其他类型的每个实例，这样做或许是为测试目的，而要有这样的已知结果（imagine that later we'll implement behavior for this type such that every instance of `AlwaysEqual` is always equal to every instance of any other type, perhaps to have a known result for testing purposes）。对于这样的行为表现，是不需要任何数据的！在第 10 章就会看到怎样定义特质，以及在包括类单元结构体在内的任何类型上，怎样实现特质。

> **结构体数据的所有权**
>
> 在前面清单 5-1 中的 `User` 结构体定义里，使用的是带有所有权的 `String` 类型，而非 `&str` 字符串切片类型。由于那里是要该结构体的各个实例拥有他自己的数据，且是要在整个结构体有效期间，实例数据有效，因此那里使用 `String` 类型而非 `&str` 类型就是有意而为之的了。
>
> 结构体存储到其他变量持有数据的引用，也是可能的，但这样做就需要用到 *生命周期（lifetimes）*，而生命周期则是会在后面的第 10 章会讨论到的一个 Rust 特性。生命周期确保某个结构体引用到的数据，会在该结构体有效期间保持有效。譬如说如同下面这样，在尝试在某个结构体中存储不带生命周期的引用时；这就不会工作：
>
> 文件名：`src/main.rs`

```rust
struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
}
```

> 编译器会抱怨他需要生命周期说明符：

```console
$ cargo run
   Compiling structs_demo v0.1.0 (/home/peng/rust-lang/projects/structs_demo)
error[E0106]: missing lifetime specifier
 --> src/main.rs:3:15
  |
3 |     username: &str,
  |               ^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
1 ~ struct User<'a> {
2 |     active: bool,
3 ~     username: &'a str,
  |

error[E0106]: missing lifetime specifier
 --> src/main.rs:4:12
  |
4 |     email: &str,
  |            ^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
1 ~ struct User<'a> {
2 |     active: bool,
3 |     username: &str,
4 ~     email: &'a str,
  |

For more information about this error, try `rustc --explain E0106`.
error: could not compile `structs_demo` due to 2 previous errors
```

> 在第 10 章中，就会讨论怎样来修复这些错误，尔后就可以在结构体中存储引用变量了，而至于现在，则只会使用像是 `String` 这样的具有所有权的类型，而避开使用像是 `&str` 这样的引用，来解决这个问题。


## 一个使用结构体的示例程序

为搞明白何时会想要使用结构体，下面就来编写一个计算矩形面积的程序。这里会先从使用单个变量开始，并在随后对这个程序进行重构，直到使用结构体为止。

下面就来以 `Cargo` 构造一个名为 `rectangles` 的新二进制项目，该项目将取得以像素指定的矩形宽和高，并计算出该矩形的面积。下面的清单 5-8 给出了一个简短的程序，该程序正是有着在这个项目的 `src/main.rs` 中的做法：

```rust
fn main() {
    let width1 = 30;
    let height1 = 50;

    println! (
        "该矩形的面积为 {} 平方像素。",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
```

*清单 5-8：计算由单独宽和高变量指明的矩形面积*

现在，使用 `cargo run` 允许这个程序：

```console
$ cargo run
   Compiling rectangles v0.1.0 (/home/peng/rust-lang/projects/rectangles)
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/rectangles`
该矩形的面积为 1500 平方像素。
```

这段代码通过以两个边长调用 `area` 函数，而成功计算出了该矩形的面积，不过还可以进一步让这段代码更为清晰已读。

这段代码的问题，体现在 `area` 函数签名中：

```rust
fn area(width: u32, height: u32) -> u32 {
```

`area` 函数是要计算某个矩形面积的，但这里编写的该函数，有着两个参数，同时在这个程序中，并未清楚表明那两个参数是有联系的。将宽和高组织在一起，代码就会更具易读性，且更具可管理性。在第 3 章的 [元组类型](Ch03_Common_Programming_Concepts.md#the-tuple-type) 小节，就已讨论过一种可能那样做的方式：使用元组。


### 以元组进行重构

下面的清单 5-9 给出了使用了元组的另一版本的这个程序。

文件名：`src/main.rs`

```rust
fn main() {
    let rect1 = (30, 50);

    println! (
        "该矩形的面积为 {} 平方像素。",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```

*清单 5-9：以一个元组来对矩形的宽和高进行指定*

一方面，这个程序更好了。元组实现了一些代码结构的加入，且现在传递的只有了一个参数。但在另一方面，这个版本变得更不清楚了：元组不会给他的各个元素命名，因此就不得不索引到该元组的各部分，从而令到这里的计算不那么直观了。

将宽和高混合起来对于面积计算并不重要，但在要将这个矩形绘制在屏幕上时，那就会有影响了！那时就必须要记住元组中索引 `0` 的是 `width`，而 `height` 是索引 `1`。这对那些将要使用到这代码的其他人来说，将会更难。由于没有在代码中传达数据的意义，因此现在更易于引入错误。

### 以结构体进行重构：加入更多意义

这里要使用结构体，通过给数据打上标签，来加入更多意义。可将这里正在使用的元组，以给整体命名，同时还给那些部分命名，而转换成为一个结构体。如下清单 5-10 所示。

文件名：`src/main.rs`

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println! (
        "该矩形的面积为 {} 平方像素。",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

*清单 5-10：定义一个 `Rectangle` 结构体*

这里就已定义了一个结构体，并将其命名为了 `Rectangle`。在那对花括弧内部，以 `width` 和 `height` 定义了两个字段，两个字段都具有 `u32` 类型。随后在 `main` 函数中，创建出了 `Rectangle` 的一个宽为 `30`，高为 `50` 的特定实例。

现在的 `area` 函数被定义为带有一个参数，该参数被命名为 `rectangle`，其类型是结构体 `Rectangle` 实例的不可变借用。如同在第 4 章中提到的那样，这里是要借用那个结构体，而非要取得那个结构体的所有权。在此方式下，`main` 函数仍保留着那个结构体实例的所有权，进而可继续使用变量 `rect1`，这就是在函数 `area` 签名与函数调用中，使用 `&` 符号的原因。

`area` 函数会访问那个 `Rectangle` 实例的 `width` 和 `height` 字段。`area` 的函数签名现在表达的正是这里想要的了：使用 `Rectangle` 的 `width` 和 `height` 字段，计算出他的面积。这就传达出了这里的宽与高是相互关联，同时这样做还给到了这些值描述性的名称，而非使用之前元组的索引 `0` 和 `1` 了。这在代码清晰上得了一分。


### 使用派生特质加入有用功能

**Adding Useful Functionality with Derived Traits**

如果能在调试程序期间打印出 `Rectangle` 的实例，并查看到所有字段的值，那就会派上用场。下面的清单 5-11 尝试了使用之前各章已经用到 [`println!` 宏](https://doc.rust-lang.org/std/macro.println.html)。不过这段代码不会工作。

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println! ("rect1 为：{}", rect1);
}
```

*清单 5-11：尝试打印出一个 `Rectangle` 实例*

在编译这段代码时，会得到有着以下核心消息的错误：

```console
error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
```

`println!` 宏可完成许多种类的格式化，而默认情况下，那对花括号告诉 `println!` 的是，要使用名为 `Display` 的格式化操作：即用于最终用户直接消费的输出（the `println!` macro can do many kinds of formatting, and by default, the curly brackets tell `println!` to use formatting known as `Display`: output intended for direct end user consumption）。因为在要将一个 `1` 或其他任何原生类型，展示给用户时，都只有唯一的一种方式，因此，对于至今为止已见到过的那些原生类型来说，默认都是实现了 `Display` 的。而对于结构体来说，由于存在更多的显示可能：是要逗号还是不要？要打印出那对花括号吗？所有字段都要展示出来吗？因此 `println!` 对输出进行格式化的方式，就不那么清楚了。正是因为这种模棱两可，Rust 于是就不尝试猜测代码编写者想要的样子，而结构体也就没有一个事先提供的、与 `println!` 和 `{}` 占位符一起使用的 `Display` 实现了。

在继续阅读该错误消息时，就会发现下面这个有用注解：

```console
   = help: the trait `std::fmt::Display` is not implemented for `Rectangle`
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
```

来试一下！这个 `println!` 的宏调用，现在看起来是这样 `println! ("rect1 为 {:?}", rect1);`。将说明符 `:?` 放在那对花括号里头，就会告诉 `println!`，这里是要使用一个名为 `Debug` 的输出。而 `Debug` 特质就令到这里可将那个结构体，以对开发者有用的方式打印出来，如此就可以在对代码进行调试时，看到那个结构体的值了。

在此改变下，对该代码进行编译。见鬼！还是得到个错误：

```console
error[E0277]: `Rectangle` doesn't implement `Debug`
```

不过编译器再度给到一个帮助性注释：

```console
   = help: the trait `Debug` is not implemented for `Rectangle`
   = note: add `#[derive(Debug)]` to `Rectangle` or manually `impl Debug for Rectangle`
```

Rust *确实* 带有打印输出调试信息的功能，不过这里必须显式地选择上那功能，从而使得那功能对这个结构体可用。而要实现这个目的，就要在紧接着结构体定义之前，加上外层属性 `#[derive(Debug)]`（the outer attribute `#[derive(Debug)`），如下面的清单 5-12 所示。

文件名：`src/main.rs`

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println! ("rect1 为：{:?}", rect1);
}
```

*清单 5-12：加入派生 `Debug` 特质的属性，进而运用调试格式化将那个 `Rectangle` 实例打印出来*

此时在运行这个程序时，就不会收到任何错误了，且会看到下面的输出：

```console
$ cargo run
   Compiling rectangles v0.1.0 (/home/peng/rust-lang/projects/rectangles)
    Finished dev [unoptimized + debuginfo] target(s) in 0.20s
     Running `target/debug/rectangles`
rect1 为：Rectangle { width: 30, height: 50 }
```

很棒！这虽不是最漂亮的输出，但他给出了该实例全部字段的值，这无疑在调试期间会有帮助。在有着较大的结构体时，让输出更容易阅读一点就会有用；对于那些更大结构体的情形，就可在 `println!` 中使用 `{:#?}` 而非 `{:?}`。而在这个示例中，使用 `{:#?}` 样式将输出：

```console
cargo run
   Compiling rectangles v0.1.0 (/home/peng/rust-lang/projects/rectangles)
    Finished dev [unoptimized + debuginfo] target(s) in 0.18s
     Running `target/debug/rectangles`
rect1 为：Rectangle {
    width: 30,
    height: 50,
}
```

使用 `Debug` 格式化将某个值打印出来的另一种方式，就是使用 [`dbg!` 宏](https://doc.rust-lang.org/std/macro.dbg.html)，这个 `dbg!` 宏会占据某个表达式的所有权，而将那个 `dbg!` 宏调用出现在代码中所在的文件与行号，与那个表达式的结果值一并打印出来，同时返回结果值的所有权（another way to print out a value using the [`dbg!` macro](https://doc.rust-lang.org/std/macro.dbg.html), which takes ownership of an expression, prints the file and line number of where that `dbg!` macro call occurs in your code along with the resulting value of that expression, and returns ownership of the value）。

> 注意：对 `dbg!` 宏的调用，会打印到标准错误控制台流（the standard error console stream, `stderr`），这与 `println!` 宏打印到标准输出控制台流（the standard output console stream, `stdout`）相反。在第 12 章中的 [将错误消息写到标准错误而非标准输出](Ch12_An_I_O_Project_Building_a_Command_Line_Program.md#writing-error-messages-to-standard-error-instead-of-standard-output) 小节，将讲到更多有关 `stderr` 与 `stdout` 的内容。

以下是个其中对赋值给 `width` 字段，以及在变量 `rect1` 中的整个结构体的值感兴趣的示例：

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;

    let rect1 = Rectangle {
        width: dbg! (30 * scale),
        height: 50,
    };

    dbg! (&rect1);
}
```

这里可将 `dbg!` 放在表达式 `30 * scale` 附近，同时由于 `dbg!` 返回了该表达式值的所有权，因此 `width` 字段将获取到与不在此处调用 `dbg!` 同样的值。由于这里不想要 `dbg!` 取得 `rect1` 的所有权，因此在下一个对 `dbg!` 的调用中，使用到到 `rect1` 的引用。下面就是这个示例输出的样子：

```console
cargo run
   Compiling rectangles v0.1.0 (/home/peng/rust-lang/projects/rectangles)
    Finished dev [unoptimized + debuginfo] target(s) in 0.22s
     Running `target/debug/rectangles`
[src/main.rs:11] 30 * scale = 60
[src/main.rs:15] &rect1 = Rectangle {
    width: 60,
    height: 50,
}
```

这里就可以看到，输出的第一部分来自 `src/main.rs` 文件的第 10 行，正是对表达式 `30 * scale` 进行调式的地方，而该表达式的结果值即为 `60`（在整数原生值上实现的 `Debug` 格式化只打印他们的值）。在 `src/main.rs` 第 14 行上的 `dbg!` 调用，输出了 `rect1`，即那个 `Rectangle` 结构体的值。这个输出使用了 `Rectangle` 类型的良好 `Debug` 格式化。在尝试搞清楚代码在做什么时，这个 `dbg!` 宏真的会相当有用！

除 `Debug` 特质外，Rust 业已提供了数个与 `derive` 属性一起使用的其他特质，这些特质把有用的行为表现，添加到那些定制类型。Rust 提供的那些特质及其行为，在 [附录 C](Ch21_Appendix.md#c-derivable-traits) 小节中有列出。在第 10 章中，就会涉及到怎样去实现这些有着定制行为的特质，以及怎样创建自己的特质。除了 `derive` 之外，同样还有许多别的属性；有关属性的更多信息，请参阅 [Rust 参考手册的 “属性” 小节](https://doc.rust-lang.org/reference/attributes.html)。

这里的 `area` 函数，是相当专用的：他只会计算矩形的面积。由于 `area` 方法不会在其他任何类型上工作，因此将此行为与这里的 `Rectangle` 结构体更紧密的联系起来，就会变得有帮助。接下来就要看看，怎样通过将这个 `area` 函数，转变成一个定义在这里的 `Rectangle` 类型上的方法，而继续重构这段代码。


## 方法语法

*方法* 与函数类似：是以 `fn` 关键字和一个名称，来声明出方法，方法可以有参数和返回值，同时包含了在某个地方方法被调用时，运行的一些代码。与函数不同的地方在于，方法是在结构体（或者枚举或特质对象，关于枚举即特质对象，将分别在第 6 和 17 章讲到）的语境里面定义的，且方法的首个参数将始终是 `self`，这个 `self` 表示方法被调用的那个结构体实例本身。


### 方法的定义

下面就来将那个将一个 `Rectangle` 实例作为参数的 `area` 函数，修改为定义在 `Rectangle` 结构体上的 `area` 方法，如下清单 5-13 所示：

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println! ("该矩形的面积为 {} 平方像素。",
        rect1.area()
    );
}
```

*清单 5-13：在 `Rectangle` 结构体上定义一个 `area` 方法*


为定义 `Rectangle` 上下文中的函数，这里开启了一个 `Rectangle ` 的 `impl` （implementation）代码块。此 `impl` 代码块里头的所有物件，都会与那个 `Rectangle` 类型相关联。随后这里就把原来那个 `area` 函数，移入到这个 `impl` 的花括弧里，并将函数签名中的首个（而在此情形下，也是唯一的）参数，及函数体中的各处，均修改为 `self`。在 `main` 函数，即原先调用 `area` 函数与将 `rect1` 作为参数传递的地方，现在就可以使用 *方法语法* 来调用那个在 `Rectangle` 实例上的 `area` 方法了。方法语法（the method syntax）是在实例之后：添加一个带着方法名字、括号及全部参数的点。

在 `area` 的签名中，使用了 `&self` 而不再是 `rectangle: &Rectangle`。这个 `&self` 实际上是 `self: &Self` 的简写。在 `impl` 代码块内部，类型 `Self` 就是该 `impl` 代码块所针对的类型。方法必定有着这么一个名为 `self` 类型为 `Self` 的参数，作为他们的首个参数，因此 Rust 这才允许将首个参数位置上的该参数，简写为只是他的名称 `self`。请注意这里仍然需要在 `self` 简写前使用 `&` 运算符，来表示此方法借用了 `Self` 类型的实例，这就跟 `rectangle: &Rectangle` 一样。方法可以取得 `self` 的所有权的、不可变地借用 `self` 变量，或者可变地借用 `self` 变量，对于方法的其他参数，也是这样的。

> `&self` - 不可变借用；`&mut self` 可变借用；`self` - 取得所有权，发生所有权转移，`self` 所指向的内存堆上的值原来的所有值将失效。

这里选择了 `&self`，有着与方法版本中使用 `&Rectangle` 有着同样理由：那就是不打算取得所有权，同时只打算读取结构体中的数据，而不打算写入。在作为方法要执行的一部分，要修改方法调用所在实例时，就要使用 `&mut self` 作为首个参数了。通过仅使用 `self` 作为首个参数，而取得实例所有权的情况，就非常少见了；通常在要将 `self` 转换为其他类型的数据，而要在这样的转换之后，阻止其他调用者使用原先的实例时，会用到这样的技巧。

使用方法而不是函数的主要原因，除了提供到方法语法及不必在每个方法签名中重复 `self` 的类型外，那就是为了代码的组织了。这里已将可由某个类型实例完成的事情，放在一个 `impl` 代码块中，而不是要那些后来的代码使用者，在这里提供的库的各个地方去找寻 `Rectangle` 的那些能力。

请注意可选择给方法一个与结构体字段相同的名字。比如，这里就可以在 `Rectangle` 上定义一个同样命名为 `width` 的方法：

文件名：`src/main.rs`

```rust
impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect1.width() {
        println! ("该矩形的宽不为零；他的宽为 {}", rect1.width);
    }
}
```

这里，就选择了让 `width` 方法，在实例的 `width` 字段中的值大于 `0` 时返回 `true`，在值为 `0` 时返回 `false`：在名称与某个字段相同的方法里面，可将该字段用于任何目的。在 `main` 方法中，当这里在 `rect1.width` 后跟上一对圆括号时，那么 Rust 就明白这里指的是方法 `width`了。而在没有使用一对圆括号时，Rust 就知道那里表示的是字段 `width`。

通常，但并非总是这样，在给到方法与某个字段同样名字时，要的是让那个方法返回与其同名字段中的值，而不会去干别的事情。像这样的方法，就叫做 *获取器（getters）*，而 Rust 并未像其他语言所做的那样，自动实现结构体字段的获取器。由于可将字段构造为私有，而将方法构造为公开，而由此实现对作为类型的公开 API一部分的字段的只读访问。在第 7 章中就会讨论到何为公开与私有，以及怎样将字段或方法指定为公开或私有。

#### `->` 操作符（the `->` operator）哪去了呢？

> 在 C 和 C++ 中，方法调用会用到两个操作符：直接调用在对象上的方法时，要用到 `.`，而在对象的指针上调用方法时，则要用 `->` 操作符，这时还先要对该指针解除引用。换句话说，在 `object` 是个指针时，`object -> something()` 是类似于 `(*object) -> something()` 的。
> Rust 并无 `->` 操作符的等价操作符；相反，Rust 有着一项名为 *自动引用与解引用（automatic referencing and dereferencing）* 的特性。而方法调用就是 Rust 中有着这种行为表现的少数几个地方之一。
>
> 以下就是该特性的工作原理：在以 `object.something()` 调用某个方法时，Rust 会自动加上 `&`、`&mut` 或 `*`，这样 `object` 就会匹配上该方法的签名。换句话说，下面的语句是一致的：
>
```rust
p1.distance(&p2);
(&p1).distance(&p2);
```
>
> 第一个语句看起来要清楚不少。由于方法有着明确的接收者 -- 即 `self` 的类型，因此这种自动引用的行为会生效。在给定了接收者和方法名字后，Rust 就可明确地确定出该方式到底是在读取（`&self`）、改变（`&mut self`），或者是在消费（`self`）。Rust 实现了方法接收者的隐式借用这一事实，是为实现所有权系统在编程实践中符合人机交互，而所做努力的较大部分（the fact that Rust makes borrowing implicit for method receivers is a big part of making ownership ergonomic in practice）。

### 有多个参数的方法

下面就来通过在 `Rectangle` 结构体上实现另一个方法，练习一下方法的运用。这次就要 `Rectangle` 的一个实例，去取得另一个 `Rectangle` 的实例，并在第二个 `Rectangle` 完全能放入到 `self` （即第一个 `Rectangle` ）里头时返回 `true`；否则这个方法就会返回 `false`。也就是，一旦定义了这个 `can_hold` 方法，就要能够编写下面清单 5-14 中的那个程序。

文件名：`src/main.rs`

```rust
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println! ("rect1 可以装下 rect2 吗？{}", rect1.can_hold(&rect2));
    println! ("rect1 可以装下 rect3 吗？{}", rect1.can_hold(&rect3));
}
```

*清单 5-14：对尚未成文的 `can_hold` 方法进行使用*

由于 `rect2` 的两个边都小于 `rect1` 的两个边，而 `rect3` 的两个边都要长于 `rect1` 的两个边，因此预期的输出将看起来像下面这样：

```console
rect1 可以装下 rect2 吗？true
rect1 可以装下 rect3 吗？false
```

这里知道要定义的是个方法，因此那将会在 `impl Rectangle` 代码块内部。而方法的名称将是 `can_hold`，同时他会取得作为参数的另一 `Rectangle` 值的不可变借用。通过观察调用该方法的代码，就可以得出那个参数的类型了：`rect1.can_hold(&rect2)` 传入的是 `&rect2`，正是到变量 `rect2` 的不可变借用，而 `rect2` 又是 `Rectangle` 的一个实例。由于这里只需要读取 `rect2`（而非写入，那就意味着将需要一个可变借用了），同时这里是想要 `main` 函数保留 `rect2` 的所有权，这样就可以在 `can_hold` 方法调用之后，还可以再度使用 `rect2`，因此这样做是有理由的。`can_hold` 方法的返回值，将是个布尔值，而该方法的实现会检查 `self` 的宽和高，相应地是否都大于另一个 `Rectangle` 的宽和高。下面就把这个新的 `can_hold` 方法，加入到清单 5-13 的 `impl` 代码块，如下清单 5-15 所示。

文件名：`src/main.rs`

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        (self.width > other.width && self.height > other.height) ||
            (self.width > other.height && self.height > other.width)
    }
}
```

*清单 5-15：对在 `Rectangle` 上的、取另一 `Rectangle` 实例作为参数的 `can_hold` 方法进行实现*

在以清单 5-14 中的 `main` 函数来运行此代码是，就会得到想要的输出。方法可取得在 `self` 参数之后添加到其签名的多个参数，同时这些参数就像函数中的参数一样生效。

### 关联函数（associated functions）


由于定义在 `impl` 代码块内部的全部函数，都是与那个在 `impl` 关键字之后命名的类型相关联的，因此他们都叫做 *关联函数（associated functions）*。因为一些关联函数不需要用到该类型的实例，因此可把这些函数定义为不将 `self` 作为首个参数的关联函数（而这样的话，这些函数就不是方法了）。前面就已用到过这样的一个关联函数：`String::from` 函数就是定义在 `String` 类型上的。

非方法的关联函数，通常用于将会返回一个该结构体新实例的构造函数。比如，这里就可提供有着一维参数，并将该一维参数同时用作宽和高的这么一个关联函数，如此就令到相比于两次指定同样值，而更容易创建除正方形的 `Rectangle`。

文件名：`src/main.rs`

```rust
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
```

要调用这个关联函数，就要使用带有结构体名字的 `::` 语法；`let sq = Rectangle::square(3);` 就是一个示例；该函数是是在那个结构体的命名空间之下的：`::` 语法，同时用于关联函数，与由模组创建出的命名空间。在第 7 章会讨论到 Rust 的模组概念。

### 多个 `impl` 代码块

所有结构体都允许有多个 `impl` 代码块。比如前面的清单 5-15 就与下面清单 5-16 给出的代码等价，其中各个方法都在各自的 `impl` 代码块中：

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        (self.width > other.width && self.height > other.height) ||
            (self.width > other.height && self.height > other.width)
    }
}
```

*清单 5-16：使用多个 `impl` 代码块对清单 5-15 进行重写*

虽然这里并无将这些方法分开到多个 `impl` 代码块中的理由，不过这样做也是有效的语法。在第 10 章讨论到泛型和特质时，就会看到多个 `impl` 代码块是有用的情形。

## 本章小节

结构体实现了创建对于特定领域有意义的定制类型。通过运用结构体，就可以将有关联的数据片段相互连接起来，并给各个数据取名字来让代码清晰。在 `impl` 代码块中，可定义与类型关联的函数，而方法则是一类实现了指定结构体实例所拥有行为的关联函数。

然而结构体并非能够创建定制类型的唯一方式：加下了就要转向到 Rust 的枚举特性，将另一工具加入到编程工具箱。
