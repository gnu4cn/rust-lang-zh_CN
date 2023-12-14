# 定义与实例化结构体

**Defining and Instantiating Structs**


结构体与 [元组类型](../programming_concepts/data_types.md#元组类型) 小节中，讨论过的元组类似，他们都保存了多个相关的值。与元组一样，结构体中的数据也可以是不同的类型。与元组不同的是，在结构体中，我们会为每条数据命名，这样就能清楚地知道，这些值的含义。添加这些名字，意味着结构体比元组更灵活：咱们不必依赖数据的顺序，来指定或访问某个实例的值。

要定义某个结构体，我们要输入关键字 `struct` 并为这整个结构体命名。结构体的名称，应描述被组合在一起的这些数据的意义。然后，在花括号内，咱们要定义出我们称之为 *字段，fields* 的，这些数据的名字和类型。例如，下面清单 5-1 给出了一个，存储了一名用户账户信息的结构体。


```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}
```

*清单 5-1：`User` 结构体的定义*


要在咱们定义了某个结构体后使用他，我们就要通过为每个字段，指定出具体的值，创建出该结构体的一个 *实例，instance*。我们通过指明该结构体的名字，然后添加一对，包含着一些其中键是字段的名字，值为我们要存储在这些字段中数据的 *key: value* 键值对的花括号。我们不必按照在结构体中声明字段的同样顺序，指定这些字段。换句话说，结构体的定义，就像是该类型的通用模板，而实例则将以特定数据，填充该模板，以创建出该类型的值来。例如，我们可以如下清单 5-2 所示，声明出一个特定的用户。


文件名：`src/main.rs`

```rust
fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    }
}
```

*清单 5-2：创建一个 `User` 结构体的实例*


要从结构体中获取某个特定值，我们要使用点表示法。例如，要访问这名用户的电子邮件地址，我们会使用 `user1.email`。如果该实例是可变的，我们可以通过使用这种点表示法，而将赋值到某个特定字段，来更改某个值。下面清单 5-3 给出了，如何更改某个可变 `User` 实例的 `email` 字段中的值。


文件名：`src/main.rs`

```rust
fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    }

    user1.email = String::from("anotheremail@example.com");
}
```

*清单 5-3：修改某个 `User` 实例的 `email` 字段中的值*


请注意，整个实例必须是可变的；Rust 不允许我们，只将某些字段标记为可变。与任何表达式一样，我们可以将结构体的新实例，构造为函数体的最后一个表达式，从而隐式地返回该新实例。

下面清单 5-4，给出了一个以给定电子邮件和用户名，返回一个 `User` 实例的 `build_user` 函数。`active` 字段会得到值 `true`，`sign_in_count` 会得到值 `1`。


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


## 运用字段初始化的简写法

**Using the Field Init Shorthand**


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


## 使用结构体更新语法，从其他实例创建实例

**Creating Instances from Other Instances with Struct Update Syntax**


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

请注意结构体更新语法，像赋值一样使用了 `=`；这是由于结构体更新语法迁移了数据，就跟在之前的 ["变量与数据互动方式：迁移"](Ch04_Understanding_Ownership.md#变量与数据互操作方式之一迁移所有权) 小节中看到的那样。在此示例中，在创建了 `user2` 之后，由于变量 `user1` 中的 `username` 字段中的 `String` 值，已被迁移到 `user2` 中了，因此就再也不能使用变量 `user1` 了。若给到 `user2` 的 `email` 及 `username` 字段都是新的 `String` 值，而因此只使用来自 `user1` 的 `active` 和 `sign_in_count` 值，那么在创建了 `user2` 之后，`user1` 仍将是有效的。因为 `active` 和 `sign_in_count` 的类型，都是实现了 `Copy` 特质的类型，因此就会应用在 [唯栈数据：拷贝](Ch04_Understanding_Ownership.md#唯栈数据拷贝stack-only-data-copy) 小节中的行为表现。


## 使用不带命名字段的元组结构体，来创建不同类型

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


## 不带任何字段的类单元结构体

**Unit-Like Structs Without Any Fields**


还可以定义没有任何字段的结构体！由于这些没有任何字段的结构体，与曾在 [元组类型](Ch03_Common_Programming_Concepts.md#元组类型) 小节提到过的单元类型 `()` 表现类似，因此他们叫做 *类单元结构体（unit-like structs）*。当需要在某类型上实现某个特质（trait），却又不希望将任何数据存储在那个类型自身里面时，类单元结构体就就有用（unit-like structs can be useful when you need to implement a trait on some type but don't have any data that you want to store in the type itself）。在第 10 章就会讨论到特质。下面是一个声明和初始化名为 `AlwaysEqual` 的单元结构体的示例：

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
