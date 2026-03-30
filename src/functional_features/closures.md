# 闭包

Rust 的闭包属于匿名函数，咱们可将其保存在变量中，或作为参数传递给其他函数。咱们可在一处创建闭包，然后在其他地方调用该闭包，以在不同的上下文中对其求值。与函数不同，闭包可以捕获定义他们的作用域中的值。我们将演示这些闭包特性怎样实现代码重用与行为定制。


## 捕获环境

咱们将首先探讨如何使用闭包捕获定义他们的环境中的值以供随后使用。场景如下：作为促销活动，我们的 T 恤衫公司会不定期向邮件列表中的某人赠送一件独家限量版的 T 恤衫。邮件列表中的人们可以选择添加他们偏好的颜色到他们的个人资料。当被选中获得免费 T 恤的人设置了喜好颜色时，他们会获得该颜色的 T 恤。当此人没有指定喜好颜色时，他们会获得公司当前有的数量最多的颜色。

实现这一逻辑的方式有很多。在这个示例中，我们将使用一个名为 `ShirtColor` 的枚举，有着变种 `Red` 与 `Blue`（出于简单目的，限制了可选颜色的数量）。我们以 `Inventory` 结构体表示公司的库存，其有一个名为 `shirts` 的字段，包含一个表示当前库存中 T 恤衫颜色的 `Vec<ShirtColor>` 值。定义在 `Inventory` 上的方法 `giveaway` 会获取免费 T 恤衫中奖者的可选体恤衫颜色偏好，并返回那个人将获得的体恤衫颜色。这一设置如下面清单 13-1 中所示：

<a name="listing_13-1"></a>
文件名：`src/main.rs`

```rust
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec! [ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println! (
        "选项为 {:?} 的用户得到 {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println! (
        "选项为 {:?} 的用户得到 {:?}",
        user_pref2, giveaway2
    );
}
```

**清单 13-1**：体恤衫公司赠品情况

定义在 `main` 中的 `store`，还有两件蓝色 T 恤和一件红色 T 恤可供本次限量版促销活动分发。我们分别对一名偏好红色的用户和已为没有偏好的用户调用了 `giveaway` 方法。

同样，这段代码可以多种方式实现，在这里，为了专注于闭包，我们坚持使用咱们已经学过的概念，除了用到闭包的 `giveaway` 方法主体外。在 `giveaway` 方法中，我们以类型为 `Option<ShirtColor>` 的参数获取用户偏好，并调用 `user_preference` 上的 `unwrap_or_else` 方法。[`Option<T>` 上的 `unwrap_or_else` 方法](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or_else) 由标准库定义。他取一个参数：一个不带任何参数、返回值 `T` （存储在 `Option<T>` 的 `Some` 变种中的同一类型，这一情形下即 `ShirtColor`）的闭包。当 `Option<T>` 为 `Some` 变种时，`unwrap_or_else` 返回 `Some` 内的值。当 `Option<T>` 为 `None` 变种时，`unwrap_or_else` 会调用闭包，并返回闭包返回的值。

我们指定闭包表达式 `|| self.most_stocked()` 为 `unwrap_or_else` 的参数。这是个本身不取参数的闭包（当闭包有参数时，他们会出现在两条竖线之间）。该闭包的主体调用了 `self.most_stocked()`。咱们于此处定义了该闭包，而 `unwrap_or_else` 的实现将在随后需要结果时对这个闭包求值。

运行这段代码会打印以下内容：


```console
$ cargo run
   Compiling shirt-company v0.1.0 (/home/hector/rust-lang-zh_CN/projects/shirt-company)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/shirt-company`
选项为 Some(Red) 的用户得到 Red
选项为 None 的用户得到 Blue
```

这里一个有趣的方面是，我们传递了一个闭包，其会在当前的 `Inventory` 实例上调用 `self.most_stocked()`。标准库无需了解我们定义的 `Inventory` 或 `ShirtColor` 类型，或者我们打算在此场景中使用的逻辑。这个闭包捕获了到 `self` 这个 `Inventory` 实例的不可变引用，并将其与我们指定的代码一起，传递给 `unwrap_or_else` 方法。另一方面，函数无法以这种方式捕获他们的环境。


## 推断与注解闭包类型

函数与闭包之间还存在更多差异。闭包通常不要求咱们像 `fn` 函数那样注解参数或返回值的类型。类型注解对函数是需要的，因为类型是暴露给咱们用户的显式接口的一部分。严格定义这一接口，对于确保所有人都对函数使用的值类型和返回的值类型达成一致至关重要。另一方面，闭包并不用于这样的暴露接口：他们存储在变量中，并且他们在无需命名及无需暴露给库的用户的情况下被使用。

闭包通常很简短，且仅在狭窄的上下文中相关，而非在任意场景中。在这些受限的上下文中，编译器可以推断参数与返回值的类型，类似于其能够推断大多数变量类型的方式（在极少数情况下，编译器也需要闭包类型注解）。

与变量一样，当我们想要提高显式性和清晰度时，可以添加类型注解，但代价是比严格必要的更加冗长。注解闭包的类型会看起来像下面清单 13-2 中所示的定义。在这个示例中，我们定义了个闭包并将其存储在一个变量中，而不是如同清单 13-1 中咱们所做的，在作为参数传递闭包处定义闭包。

<a name="listing_13-2"></a>
文件名：`src/main.rs`

```rust
let expensive_closure = |num: u32| -> u32 {
    println! ("缓慢计算中......");
    thread::sleep(Duration::from_secs(2));
    num
}
```

**清单 13-2**：添加闭包中参数与返回值的可选类型注解

添加类型注解后，闭包的语法看起来与函数的语法更加相似。下面，我们出于比较目的，定义了一个加 1 到其参数的函数，以及有着同样行为的闭包。咱们添加了一些空格来对齐相关部分。这说明了闭包语法与函数语法的相似之处，除了管道的使用和部分可选的语法外。

```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 };
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

第一行展示了一个函数定义，第二行展示了一个完全注解的闭包定义。在第三行中，我们移除了闭包定义中的类型注解。在第四行中，我们移除了花括号，由于该闭包主体只有一个表达式，因此这是可选的。这些都属于有效的定义，在被调用时将产生相同的行为。`add_one_v3` 与 `add_one_v4` 这两行代码需要闭包被求值才能编译，因为类型将根据他们的用法得以推断。这类似于 `let v = Vec::new();` 需要类型注解或向该 `Vec` 插入某一类型的值，Rust 才能够推断类型。

对于闭包定义，编译器将为每个参数与其返回值都推断出一种具体类型。例如，下面清单 13-3 展示了一个简短闭包的定义，仅返回其作为参数接收到的值。除了这个示例的目的外，这个闭包并不是很有用。请注意，我们没有向该定义添加任何类型注解。因为没有类型注解，我们可以任何类型调用这个闭包，我们在这里第一次是以 `String` 类型调用的。当我们随后尝试以整数调用 `example_closure` 时，我们将得到一个报错。

<a name="listing_13-3"></a>
文件名：`src/main.rs`

```rust
    let example_closure = |x| x;
    let s = example_closure(String::from(你好));
    let n = example_closure(5);
```

**清单 13-3**：尝试以两种不同类型，调用一个其类型为推断出的闭包

编译器给予我们下面这个报错：

```console
$ cargo run
   Compiling closure-example v0.1.0 (/home/hector/rust-lang-zh_CN/projects/closure-example)
error[E0425]: cannot find value `你好` in this scope
 --> src/main.rs:4:42
  |
4 |     let s = example_closure(String::from(你好));
  |                                          ^^^^ not found in this scope

error[E0308]: mismatched types
 --> src/main.rs:5:29
  |
5 |     let n = example_closure(5);
  |             --------------- ^ expected `String`, found integer
  |             |
  |             arguments to this function are incorrect
  |
note: expected because the closure was earlier called with an argument of type `String`
 --> src/main.rs:4:29
  |
4 |     let s = example_closure(String::from(你好));
  |             --------------- ^^^^^^^^^^^^^^^^^^ expected because this argument is of type `String`
  |             |
  |             in this closure call
note: closure parameter defined here
 --> src/main.rs:2:28
  |
2 |     let example_closure = |x| x;
  |                            ^
help: try using a conversion method
  |
5 |     let n = example_closure(5.to_string());
  |                              ++++++++++++

Some errors have detailed explanations: E0308, E0425.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `closure-example` (bin "closure-example") due to 2 previous errors
```

我们第一次以 `String` 值调用 `example_closure` 时，编译器会推断出该闭包的 `x` 与返回值类型为 `String`。这些类型随后就被锁定在 `example_closure` 中的闭包中，而当我们下次尝试对同一闭包使用不同类型时，便得到一个类型报错。


## 捕获引用抑或迁移所有权

闭包可以三种方式捕获其环境中的值，这直接对应于函数取得参数的三种方式：

- 不可变地借用；
- 可变地借用；
- 与取得所有权。

闭包将根据函数主体对捕获值执行的操作来决定使用何种方式。

在下面清单 13-4 中，由于其只需不可变引用来打印出值，因此咱们定义了个捕获了到名为 `list` 矢量值的不可变引用的闭包：

<a name="listing_13-4"></a>
文件名：`src/main.rs`

```rust
fn main () {
    let list = vec! [1, 2, 3];
    println! ("定义闭包前：{list:?}");

    let only_borrows = || println! ("闭包中的：{list:?}");

    println! ("调用闭包前：{list:?}");
    only_borrows();
    println! ("调用闭包后：{list:?}");
}
```

**清单 13-4**：定义并调用捕获不可变引用的闭包

这个示例还说明，变量可以绑定到闭包定义，并且我们稍后可通过使用变量名字和圆括号调用该闭包，就像变量名是函数名一样。

因为我们可以同时有着多个对 `list` 的不可变引用，所以 `list` 在闭包定义前、闭包定义后但被调用前，及闭包调用后的代码中仍然是可访问的。这段代码会编译、运行，并打印如下输出：

```console
$ cargo run
   Compiling closure-example v0.1.0 (/home/hector/rust-lang-zh_CN/projects/closure-example)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/closure-example`
在定义闭包之前的：[1, 2, 3]
在调用闭包之前：[1, 2, 3]
闭包中的：[1, 2, 3]
在调用闭包之后：[1, 2, 3]
```

接下来，在下面清单 13-5 中，我们修改了闭包主体，使其添加一个元素到 `list` 矢量。闭包现在捕获了一个可变引用。

<a name="listing_13-5"></a>
文件名：`src/main.rs`

```rust
fn main () {
    let list mut = vec! [1, 2, 3];
    println! ("定义闭包前：{list:?}");

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println! ("调用闭包后：{:?}", list);
}
```

**清单 13-5**：定义并调用捕获可变引用的闭包

这段代码会编译、运行，并打印：

```console
$ cargo run
   Compiling closure-example v0.1.0 (/home/hector/rust-lang-zh_CN/projects/closure-example)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/closure-example`
定义闭包前：[1, 2, 3]
调用闭包后：[1, 2, 3, 7]
```

请注意，在 `borrows_mutably` 闭包的定义与调用之间不再有 `println!`： `borrows_mutably` 被定义时，其就捕获了到 `list` 的可变引用。闭包被调用后我们未再使用该闭包，因此可变借用结束。在闭包定义与闭包调用之间，打印目的的不可变借用不被允许，因为当存在可变借用时，不允许其他借用。请尝试在那里添加一个 `println!`，看看咱们会得到什么错误消息！

> **译注**：在闭包定义和闭包调用之间放入一个不可变借用时，错误消息如下。
>
> ```console
>    Compiling closure-example v0.1.0 (/home/hector/rust-lang-zh_CN/projects/closure-example)
> error[E0502]: cannot borrow `list` as immutable because it is also borrowed as mutable
>  --> src/main.rs:7:23
>   |
> 5 |     let mut borrows_mutably = || list.push(7);
>   |                               -- ---- first borrow occurs due to use of `list` in closure
>   |                               |
>   |                               mutable borrow occurs here
> 6 |
> 7 |     println! ("调用闭包前：{list:?}");
>   |                             ^^^^ immutable borrow occurs here
> 8 |     borrows_mutably();
>   |     --------------- mutable borrow later used here
>   |
>   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
>
> For more information about this error, try `rustc --explain E0502`.
> error: could not compile `closure-example` (bin "closure-example") due to 1 previous error
> ```

当咱们打算强制闭包取得其用到的环境中的值的所有权，即使闭包主体并不严格需要所有权时，咱们也可以在参数列表前使用 `move` 关键字。

在传递闭包给新线程，以迁移数据使其归新线程所有，这一技巧最有用。咱们将在第 16 章中讨论并发时，详细讨论线程以及为何咱们将希望使用他们，而现在，咱们来简要地探讨一下，使用需要 `move` 关键字的闭包生成一个新线程。下面清单 13-6 展示了修改后的 [清单 13-4](#listing_13-4)，以在新线程而不是主线程中打印那个矢量值。

<a name="listing_13-6"></a>
文件名：`src/main.rs`

```rust
use std::thread;

fn main () {
    let mut list = vec! [1, 2, 3];
    println! ("定义闭包前：{list:?}");

    thread::spawn(move || println! ("线程中：{list:?}")).join().unwrap();
}
```

**清单 13-6**：使用 `move` 强制线程的闭包取得 `list` 的所有权

咱们生成一个新线程，给予该线程一个闭包作为参数运行。闭包主体体打印出列表。在清单 13-4 中，闭包仅使用了不可变引用捕获 `list`，因为这是打印 `list` 所需的最低权限。在这个示例中，即使闭包主体仍然只需要一个不可变引用，我们也需要通过放置 `move` 关键字在闭包定义的开头，指定 `list` 应被迁移到闭包中。若在对新线程调用 `join` 之前主线程执行了更多操作，新线程可能在主线程其余部分执行完成之前完成，或者主线程可能先完成。若主线程保留了 `list` 的所有权，而在新线程结束之前结束并弃用 `list`，则线程中的不可变引用将成为无效。因此，编译器要求，`list` 要迁移到提供给新线程的闭包中，以便该引用将有效。请尝试移除 `move` 关键字，或在闭包定义处之后使用 `list`，看看咱们会得到什么样的编译器报错！


## 从闭包中迁出捕获值

一旦闭包从其被定义的环境中捕获了某个引用，或捕获了环境中某个值的所有权后（从而影响被迁移 *进入* 闭包中的内容，在有内容被迁入时），闭包主体中的代码定义在稍后闭包被求值时，对这些引用或值会发生什么操作（从而影响从闭包迁移 *出去* 的内容，在有内容被迁出时）。

闭包主体可以执行以下任一操作：

- 从闭包迁出捕获值、
- 修改捕获值、
- 既不移动也不修改值，
- 或者从一开始就不捕获环境中的任何内容。

闭包捕获并处理环境值的方式，会影响闭包实现的特质，而特质是函数与结构体可以指定他们可以使用何种闭包的方式。闭包将根据其主体处理值的方式，以累加的方式自动实现以下一个、两个或全部三个 `Fn` 特质：

1. `FnOnce`，适用于只能调用一次的闭包。所有闭包都至少实现了这一特质，因为所有闭包都可被调用。会从其主体中迁出捕获值的闭包，将仅实现 `FnOnce` 而不会实现其他 `Fn` 特质，因为他只能被调用一次；
2. `FnMut`，适用于不会从其主体迁出捕获值，但可能修改捕获值的闭包。这些闭包可被多次调用；
3. `Fn`，适用于不会从其主体迁出捕获值，也不修改捕获值的闭包，以及不从环境中捕获任何内容的闭包，这在诸如并发地多次调用闭包的情形下非常重要。

我们来看看我们在 [清单 13-1](#listing_13-1) 中使用的 `Option<T>` 上的 `unwrap_or_else` 方法的定义：

```rust
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
```

回想一下，`T` 属于泛型类型，表示 `Optoin` 的 `Some` 变种中值的类型。该类型 `T` 也是 `unwrap_or_else` 函数的返回类型：例如，对 `Option<String>` 调用 `unwrap_or_else` 的代码将得到一个 `String`。

接下来，请注意 `unwrap_or_else` 函数有着额外的泛型类型参数 `F`。类型 `F` 是名为 `f` 的参数的类型，这是我们在调用 `unwrap_or_else` 时提供的闭包。

在泛型类型 `F` 上指定的特质边界为 `FnOnce() -> T`，这意味着 `F` 必须能够被调用一次、不取参数，并返回一个 `T` 值。在特质边界中使用 `FnOnce`，表达了 `unwrap_or_else` 不会调用 `f` 多次的约束。在 `unwrap_or_else` 的主体中，我们可以看到当 `Option` 为 `Some` 时，`f` 不会被调用。当 `Option` 为 `None` 时，`f` 将被调用一次。因为所有闭包都实现 `FnOnce`，所以 `unwrap_or_else` 接收所有三种闭包，而尽可能地灵活。

> 注意：当我们打算执行的操作，不需要捕获环境中的值时，我们可以在需要某种实现 `Fn` 特质之一的项目处，使用函数的名字而不是闭包的名字。例如，对于某个 `Option<Vec<T>>` 值，当该值为 `None` 时，我们可以调用 `unwrap_or_else(Vec::new)` 来得到一个新的空矢量值。

现在我们来看看定义在切片上的标准库方法 `sort_by_key`，了解他与 `unwrap_or_else` 有何区别，以及为何 `sort_by_key` 会使用 `FnMut` 而不是 `FnOnce` 作为特质边界。闭包以到正在处理的切片中的当前元素的引用的形式得到一个参数，并返回一个可排序的类型 `K` 的值。当咱们打算根据每个项目的某一特定属性对切片进行排序时，这个函数非常有用。在下面清单 13-7 中，我们有个 `Rectangle` 实例的列表，我们使用 `sort_by_key` 根据其 `width` 属性从低到高升序对他们排序：

<a name="listing_13-7"></a>
文件名：`src/main.rs`

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|r| r.width);
    println! ("{list:#?}");
}
```

**清单 13-7**：使用 `sort_by_key` 根据宽度排序矩形

这段代码会打印：

```console
$ cargo run
   Compiling rectangles v0.1.0 (/home/hector/rust-lang-zh_CN/projects/rectangles)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/rectangles`
[
    Rectangle {
        width: 3,
        height: 5,
    },
    Rectangle {
        width: 7,
        height: 12,
    },
    Rectangle {
        width: 10,
        height: 1,
    },
]
```

`sort_by_key` 被定义为取一个 `FnMut` 闭包的原因是，他会多次调用该闭包：对切片中的每个条目调用一次。闭包 `|r| r.width` 不会捕获、修改或从其环境迁迁出任何项目，因此他满足特质边界要求。

相比之下，下面清单 13-8 展示了个仅实现 `FnOnce` 特质的闭包示例，因为他会从环境中迁出值。编译器不会让咱们与 `sort_by_key` 一起使用这个闭包：

<a name="listing_13-8"></a>
文件名：`src/main.rs`

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut sort_operations = vec! [];
    let value = String::from("闭包被调用");

    list.sort_by_key(|r| {
        sort_operations.push(value);
        r.width
    });
    println! ("{list:#?}");
}
```

**清单 13-8**：尝试对 `sort_by_key` 使用 `FnOnce` 类型的闭包

这是一种做作的、复杂的方式（无法正常工作），试图计算 `sort_by_key` 在排序 `list` 时调用闭包的次数。这段代码尝试通过压入 `value` -- 闭包的环境中的一个 `String` -- 到 `sort_operations` 矢量来完成这一计数。该闭包会捕获 `value`，随后通过转移 `value` 的所有权给 `sort_operations` 矢量值，从闭包迁出 `value`。这个闭包可以被调用一次；尝试第二次调用他是行不通的，因为 `value` 将不再位于环境中，无法再次被压入 `sort_operations`！因此，这个闭包仅实现了 `FnOnce`。当我们尝试编译这段代码时，我们会得到这个报错：`value` 无法从闭包中迁出，因为该闭包必须实现 `FnMut`：

```console
$ cargo run
   Compiling rectangles v0.1.0 (/home/hector/rust-lang-zh_CN/projects/rectangles)
error[E0507]: cannot move out of `value`, a captured variable in an `FnMut` closure
  --> src/main.rs:18:30
   |
15 |     let value = String::from("闭包被调用");
   |         -----   -------------------------- move occurs because `value` has type `String`, which does not implement the `Copy` trait
   |         |
   |         captured outer variable
16 |
17 |     list.sort_by_key(|r| {
   |                      --- captured by this `FnMut` closure
18 |         sort_operations.push(value);
   |                              ^^^^^ `value` is moved here
   |
help: `Fn` and `FnMut` closures require captured values to be able to be consumed multiple times, but `FnOnce` closures may consume them only once
  --> /rustc/01f6ddf7588f42ae2d7eb0a2f21d44e8e96674cf/library/alloc/src/slice.rs:249:12
help: consider cloning the value if the performance cost is acceptable
   |
18 |         sort_operations.push(value.clone());
   |                                   ++++++++

For more information about this error, try `rustc --explain E0507`.
error: could not compile `rectangles` (bin "rectangles") due to 1 previous error
```

报错指向闭包主体中从环境中迁出 `value` 的行。要解决这个问题，我们需要修改闭包的主体，以便他不会从环境中迁出值。在环境中保留一个计数器，并在闭包主体中递增其值，是计算闭包被调用次数的更直接的方法。下面清单 13-9 中的闭包之所以能与 `sort_by_key` 一起工作，是因为他仅捕获到 `num_sort_operations` 计数器的可变引用，因此可被多次调用。

<a name="listing_13-9"></a>
文件名：`src/main.rs`

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println! ("{list:#?}，在 {num_sort_operations} 次操作后被排序好");
}
```

**清单 13-9**：与 `sort_by_key` 一起使用 `FnMut` 闭包是允许的

在定义或使用用到了闭包的函数或类型时，`Fn` 特质非常重要。在下一小节中，我们将讨论迭代器。许多迭代器方法都会取闭包参数，因此在我们继续学习时请牢记这些闭包细节！

> 译注：将 [清单 13-8](#listing_13-8) 中的代码，只加入一个地址符号 `&`，而修改成下面这样，也是工作的。这就要想想是为什么了：）
>
> ```rust
> #[derive(Debug)]
> struct Rectangle {
>     width: u32,
>     height: u32,
> }
>
> fn main() {
>     let mut list = [
>         Rectangle { width: 10, height: 1 },
>         Rectangle { width: 3, height: 5 },
>         Rectangle { width: 7, height: 12 },
>     ];
>
>     let mut sort_operations = vec! [];
>     let value = String::from("闭包被调用");
>
>     list.sort_by_key(|r| {
>         sort_operations.push(&value);
>         r.width
>     });
>    println!("
>        {list:#?}
>        {sort_operations:#?}
>        ");
> }
> ```


（End）


