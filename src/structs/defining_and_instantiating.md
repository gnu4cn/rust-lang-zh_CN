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

*清单 5-4：取一个电子邮件和用户名，并返回一个 `User` 实例的 `build_user` 函数*


将函数参数命名为与结构字段相同的名字，是有意义的，但必须重复 `email` 和 `username` 两个字段名字和变量，就有点乏味了。如果结构体有更多字段，那么重复每个字段的名字，就会更加烦人。幸运的是，有种方便的简写！


## 使用字段初始化简写法

**Using the Field Init Shorthand**


由于清单 5-4 中的参数名字，和结构体字段名字完全相同，我们可以使用 *字段初始化简写法，field init shorthand* 语法，重写 `build_user`，使其行为完全相同，而不重复使用 `username` 和 `email`，如下清单 5-5 所示。


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

*清单 5-5：由于 `username` 与 `email` 两个参数有着与结构体字段同样的名字，而使用了字段初始化简写法的 `build_user` 函数*


在这里，我们正创建 `User` 结构体的一个新实例，其中有个名为 `email` 的字段。我们希望将 `email` 字段的值，设置为 `build_user` 函数的 `email` 参数中的值。因为 `email` 字段和 `email` 参数同名，所以我们只需写下 `email` 而不是 `email:email`。


## 使用结构体更新语法，从另一实例创建出实例

**Creating Instances from Other Instances with Struct Update Syntax**


创建一个其中包含另一实例中的大部分值，但要更改其中某些值的新实例，通常很有用。咱们可以使用 *结构体更新语法*，完成这一点。

首先，在下面清单 5-6 中，我们给出了在不使用这种更新语法下，如何常规地在 `user2` 中创建一个新 `User` 实例。我们为 `email` 字段设置了一个新值，但对其他字段，使用了咱们在清单 5-2 中，创建的 `user1` 中的那些同样值。


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

*清单 5-6：创建一个用到 `user1` 中值的新 `User` 实例*


如下清单 5-7 所示，使用结构体更新语法，我们可以用较少的代码，实现相同的效果。语法 `..` 指明了，其余未显式设置的字段，应有着与给定实例中字段，相同的值。


```rust
fn main() {
    // --跳过代码--

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
```

*清单 5-7：使用结构体更新语法设置某个 `User` 实例的新 `email` 值，但使用来自 `user1` 的其余值*


清单 5-7 中的代码，也创建出了一个 `user2` 中，有着不同 `email` 值，却有着来自 `user1` 的 `username`、`active` 和 `sign_in_count` 等字段同样值的实例。`..user1` 必须放在最后，以指定其余字段应从 `user1` 中的相应字段获取值，但我们可以选择以任意顺序，为任意多个字段指定值，与结构体定义中的字段顺序无关。

请注意，结构更新语法像赋值一样，用到了 `=` 操作符；这是因为他迁移了数据，正如我们在 [变量和数据相互作用：迁移](../ownership/about_ownership.md#变量与数据相互作用迁移) 小节中所看到的。在本例中，创建出 `user2` 后，我们不能再将 `user1` 作为一个整体使用，因为 `user1` 的 `username` 字段中的那个 `String`，已被迁移到了 `user2` 中。如果我们同时为 `user2` 的 `email` 和 `username`，赋予了新的 `String` 值，从而只使用 `user1` 中的 `active` 和 `sign_in_count` 两个值，那么`user1` 在创建出 `user2` 后，仍然会有效。`active` 和 `sign_in_count` 都是实现了 `Copy` 特质的类型，因此将适用我们在 [唯栈数据：拷贝](../ownership/about_ownership.md#唯栈数据拷贝) 小节中，曾讨论过的行为。


## 使用没有命名字段的元组结构体，创建不同类型

**Using Tuple Structs without Named Fields to Create Different Types**


Rust 还支持称为 *元组结构体，tuple structs* 的，看起来类似于元组的结构体。元组结构体有着结构体名称所提供的附带意义，但却没有与其字段相关联的名字；相反，他们只有字段的类型。如果咱们打算给这整个元组一个名字，并将该元组构造为不同于其他元组的类型，并且在给常规结构体中的各个字段命名会显得冗长或多余时，那么元组结构体就非常有用。

要定义元组结构体，就要以 `struct` 关键字以及该结构体的名字开头，然后是元组中的类型。例如，下面我们定义并使用了两个名为 `Color` 和 `Point` 的元组结构体：


```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);
    let origin = Point(0, 0, 0);
}
```


请注意，其中的 `black` 和 `origin` 两个值，属于不同类型，因为他们是不同元组结构体的实例。咱们定义的每个结构体，都属于其自己的类型，即使结构体中的字段，可能具有相同类型。例如，取 `Color` 类型参数的某个函数，就不能取某个 `Point` 作为参数，尽管这两种类型，都是由三个 `i32` 值组成。此外，元组结构体的实例，在咱们可以将其解构为他们的单独部分，以及咱们可以使用 `.` 后跟索引来访问单独值两个方面，类似于元组。


## 不带任何字段的类单元值结构体

**Unit-Like Structs Without Any Fields**


咱们还可以定义出，没有任何字段的结构体！这些结构体被称为 *类单元值结构体，unit-like structs*，因为他们的行为类似于我们在 [元组类型](../programming_concepts/data_types.md#元组类型) 小节中，提到的单元值类型 `()`。当咱们需要在某种类型上，实现某个特质，但又没有咱们打算存储在该类型中的任何数据时，类单元值结构体就会派上用场。我们将在第 10 章讨论特质。下面是个名为 `AlwaysEqual` 的单元值结构体的声明和实例化示例：


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
