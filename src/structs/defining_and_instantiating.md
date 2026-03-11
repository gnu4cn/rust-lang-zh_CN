# 定义与实例化结构体

结构体类似于 [元组类型](../programming_concepts/data_types.md#元组类型) 小节中讨论的元组，因为二者都保存多个相关值。与元组一样，结构体的各个部分可以是不同的类型。与元组不同，在结构体中，我们将为每条数据命名，从而值的含义就很清楚。添加这些名字意味着结构体比元组更灵活：咱们不必为了指定或访问实例的值而依赖数据的顺序。

为了定义结构体，我们要输入关键字 `struct` 并命名整个结构体。结构体的名称应描述这些分组在一起数据的意义。然后，在花括号内，咱们定义那些数据片段的名字与类型，我们称之为 *字段，field*。例如，下面清单 5-1 展示了个存储用户账户信息的结构体。


<a name="listing_5-1"></a>
```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}
```

**清单 5-1**：`User` 结构体定义


在咱们定义了结构体后要使用他，我们就要通过为每个字段指定具体值，创建该结构体的 *实例，instance*。我们通过指明结构体的名字创建实例，然后添加包含着一些 `key: value` 对的花括号，其中值为字段的名字，值为我们打算存储在这些字段中的数据。我们不必以我们在结构体中声明字段的同一顺序指定这些字段。换句话说，结构体的定义就像类型的通用模板，而实例会以特定数据填充该模板创建该类型的值。例如，我们可以声明某名特定用户，如下清单 5-2 中所示。


<a name="listing_5-2"></a>
文件名：`src/main.rs`

```rust
fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
}
```

**清单 5-2**：创建 `User` 结构体的实例


要获取结构体中的某个特定值，我们使用点表示法。例如，要访问这名用户的电子邮件地址，我们使用 `user1.email`。当实例是可变的时，我们可通过使用点表示法并赋值到某个特定字段更改值。下面清单 5-3 展示了如何更改某个可变 `User` 实例的 `email` 字段中的值。


<a name="listing_5-3"></a>
文件名：`src/main.rs`

```rust
fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}
```

**清单 5-3**：修改某个 `User` 实例的 `email` 字段中的值

要注意整个实例必须是可变的；Rust 不允许我们只将一些字段标记为可变。与任何表达式一样，我们可将结构体的新实例构造为函数体中的最后一个表达式，从而隐式地返回该新实例。

下面清单 5-4 展示了一个 `build_user` 函数，以给定的电子邮件和用户名返回一个 `User` 实例。`active` 字段会得到值 `true`，`sign_in_count` 会得到值 `1`。


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

*清单 5-4：取电子邮件和用户名，并返回一个 `User` 实例的 `build_user` 函数*

以与结构体字段同样的名字命名函数参数是有意义的，但必须重复 `email` 和 `username` 两个字段名字和变量有点乏味。当结构体有更多字段时，那么重复每个名字就会变得更加烦人。幸运的是，有一种方便的简写！


## 运用字段初始化简写

由于清单 5-4 中的参数名字和结构体字段名字完全相同，我们可以使用 *字段初始化简写，field init shorthand* 语法重写 `build_user`，使其行为完全相同却不会重复 `username` 和 `email`，如下清单 5-5 中所示。


<a name="listing_5-5"></a>
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

**清单 5-5**：由于 `username` 与 `email` 两个参数有着与结构体字段同样的名字，而使用了字段初始化简写的 `build_user` 函数


在这里，我们正创建 `User` 结构体的一个新实例，其有个名为 `email` 的字段。我们希望将 `email` 字段的值设置为 `build_user` 函数 `email` 参数中的值。因为 `email` 字段和 `email` 参数有着相同的名字，所以我们只需写下 `email` 而不是 `email: email`。


## 以结构体更新语法创建实例

创建包含另一实例中的大部分值，而更改其中一些值的新实例通常很有用。咱们可以使用 *结构体更新语法，struct update syntax* 完成这点。

首先，在下面清单 5-6 中，我们展示如何以常规方式，在没有更新语法下，于 `user2` 中创建一个新的 `User` 实例。我们设置了 `email` 的新值，但但其余值则使用咱们在 [清单 5-2](#listing_5-2) 中创建的 `user1` 中的相同值。


<a name="listing_5-6"></a>
```rust
fn main() {
    // --跳过代码--

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("hector.peng@rust-lang.xfoss.com"),
        sign_in_count: user1.sign_in_count,
    };
}
```

**清单 5-6**：创建一个用到 `user1` 中除一个值外的所有值的新 `User` 实例


使用结构体更新语法，我们可以用更少代码实现相同效果，如下清单 5-7 中所示。`..` 这种语法表示未显式设置的其余字段，应有着与给定实例中的字段相同的值。


<a name="listing_5-7"></a>
```rust
fn main() {
    // --跳过代码--

    let user2 = User {
        email: String::from("hector.peng@rust-lang.xfoss.com"),
        ..user1
    };
}
```

*清单 5-7：使用结构体更新语法设置 `User` 实例的新 `email` 值，但使用 `user1` 中的其余值*

清单 5-7 中的代码同样在 `user2` 中创建了个实例，其有着一个 `email` 的不同值，但有着 `user1` 中 `username`、`active` 和 `sign_in_count` 等字段的相同值。`..user1` 必须放在最后，以指定全部其余字段都应从 `user1` 中的相应字段获取他们的值，但我们可以选择以任意顺序指定咱们想要数量字段的值，而无关乎结构体定义中的字段顺序。

请注意，结构更新语法会像赋值一样使用 `=`；这是因为他迁移了数据，正如我们在 [变量和数据相互作用：迁移](../ownership/about_ownership.md#变量与数据相互作用迁移) 小节中所看到的那样。在这个示例中，我们在创建 `user2` 后就不能再使用 `user1` 了，因为 `user1` 的 `username` 字段中的 `String` 已被迁移到了 `user2` 中。若我们同时赋予了 `user2` 的 `email` 和 `username` 新的 `String` 值，而因此用到 `user1` 中的 `active` 和 `sign_in_count` 两个值，那么`user1` 在创建 `user2` 后将仍然有效。`active` 和 `sign_in_count` 都是实现了 `Copy` 特质的类型，因此我们在 [唯栈数据：拷贝](../ownership/about_ownership.md#唯栈数据拷贝) 小节中讨论过的行为将适用。在这个示例中，我们还仍然可以使用 `user1.email`，因为他的值没有从 `user1` 中迁出。


## 以元组结构体创建不同类型

Rust 还支持类似于元组的结构体，称为 *元组结构体，tuple structs*。元组结构体有着结构体名称所提供的附带含义，却没有与其字段关联的名字；相反，他们只有字段的类型。当咱们打算给整个元组取个名字而将元组构造为不同于其他元组的类型，并且在像常规结构体中那样，命名各个字段会冗长或多余时，那么元组结构体就非常有用。

要定义元组结构体，就要以 `struct` 关键字以及结构体名字开头，后跟元组中的类型。例如，下面我们定义并使用两个名为 `Color` 和 `Point` 的元组结构体：


```rust
struct Color (i32, i32, i32);
struct Point (i32, i32, i32);

fn main() {
    let black = Color (0, 0, 0);
    let white = Color (255, 255, 255);
    let origin = Point (0, 0, 0);
}
```


请注意，其中 `black` 和 `origin` 两个值属于不同类型，因为他们是不同元组结构体的实例。咱们定义的每个结构体都属于其自己的类型，即使结构体内部的字段可能具有相同类型。例如，取某个 `Color` 类型形参的函数，就不能取一个 `Point` 作实参，尽管这两种类型都是由三个 `i32` 值组成。除此以外，元组结构体的实例类似于元组，在咱们可将其解构为他们的单独部分，以及咱们可使用 `.` 后跟索引访问单独值两方面。与元组不同，元组结构体要求咱们在结构他们时指定结构体的类型。例如，我们将写下 `let Point(x, y, z) = origin;` 来将 `origin` 中的值解构为名字分别为 `x`、`y` 及 `z` 的变量。


## 定义类单元值的结构体

咱们还可以定义没有任何字段的结构体！这些被称为 *类单元值结构体，unit-like structs*，因为他们的行为类似于 `()`，我们在 [元组类型](../programming_concepts/data_types.md#元组类型) 小节中提到的单元值。当咱们需要在某种类型上实现某个特质，却没有咱们打算存储在该类型本省中的任何数据时，类单元值结构体就会很有用。我们将在第 10 章中讨论特质。下面是声明和实例化一个名为 `AlwaysEqual` 单元值结构体的示例：


```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```


为定义 `AlwaysEqual`，我们使用 `struct` 关键字、我们想要的名字，然后一个分号。无需花括号或圆括号！然后，我们可以类似方式得到 `subject` 变量中的一个 `AlwaysEqual` 实例：使用我们定义的名字，无需任何花括号或圆括号。设想稍后我们将实现这种类型的行为，以便 `AlwaysEqual` 的每个实例总是等于任何其他类型的每个实例，也许是出于测试目的而要有个已知结果。我们将不需要任何数据实现该行为！咱们将在第 10 章中看到如何定义特质并在任何类型上实现他们，包括类单元值结构体。

> **结构体数据的所有权**
>
> 在 [清单 5-1](#listing_5-1) 中的 `User` 结构体定义中，我们使用了自有（所有权）的 `String` 类型而不是 `&str` 字符串切片类型。这是有意为之，因为我们希望这个结构体的各个实例都拥有其所有数据，并且会在整个结构有效期间始终有效。
>
> 结构体存储指向由其他变量持有的数据的引用也是可行的，但这样做要用到 *生命周期，lifetime*，我们将在第 10 章讨论。生命周期会确保由结构体引用的数据，在结构体存在期间一直有效。假设咱们尝试在未指定声明周期下，在结构体中存储某个引用，就像下面在 `src/main.rs` 中这样；这是行不通的：
>
> 文件名：`src/main.rs`
>
> ```rust
> struct User {
>     active: bool,
>     username: &str,
>     email: &str,
>     sign_in_count: u64,
> }
>
> fn main() {
>     let user1 = User {
>         email: "someone@example.com",
>         username: "someusername123",
>         active: true,
>         sign_in_count: 1,
>     };
> }
> ```
>
> 编译器将抱怨他需要生命周期说明符，lifetime specifier：
>
> ```console
> $ cargo run
>    Compiling struct_n_lifetime_demo v0.1.0 (/home/hector/rust-lang-zh_CN/projects/struct_n_lifetime_demo)
> error[E0106]: missing lifetime specifier
>  --> src/main.rs:3:15
>   |
> 3 |     username: &str,
>   |               ^ expected named lifetime parameter
>   |
> help: consider introducing a named lifetime parameter
>   |
> 1 ~ struct User<'a> {
> 2 |     active: bool,
> 3 ~     username: &'a str,
>   |
>
> error[E0106]: missing lifetime specifier
>  --> src/main.rs:4:12
>   |
> 4 |     email: &str,
>   |            ^ expected named lifetime parameter
>   |
> help: consider introducing a named lifetime parameter
>   |
> 1 ~ struct User<'a> {
> 2 |     active: bool,
> 3 |     username: &str,
> 4 ~     email: &'a str,
>   |
>
> For more information about this error, try `rustc --explain E0106`.
> error: could not compile `struct_n_lifetime_demo` (bin "struct_n_lifetime_demo") due to 2 previous errors
> ```
>
> 在第 10 章中，我们将讨论如何修复这些报错，以便咱们可在结构体中存储引用，但现在，我们将使用像是 `String` 的自有类型而不是像是 `&str` 的引用修复这类报错。


（End）


