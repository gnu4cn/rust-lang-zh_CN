# 闭包：捕获其环境的匿名函数

**Closures: Anonymous Functions that Capture Their Environment**


Rust 的闭包，是一些咱们可将其保存在变量中，或将其作为参数传递给其他函数的匿名函数。咱们可在一处创建出闭包，随后在别处调用该闭包，而在不同上下文中执行他，evaluate it。与函数不同，闭包可捕获到他们于其中被定义作用域的值。随后咱们将演示这些闭包特性，怎样实现代码重用与行为定制，unlike functions, closures can capture values from the scope in which they're defined. We'll demonstrate how these closure features allow for code reuse and behavior customization。


## 使用闭包捕获环境

**Capturing Environment with Closures**


咱们将首先检视，咱们可怎样使用闭包来捕获他们被定义所在环境的值，以供稍后使用。场景是这样的：每隔一段时间，咱们体恤衫公司都会以促销方式，送出一些独家的、限量版的体恤衫给邮件列表上的人。邮件列表上的人可选择性地将他们偏好的颜色，添加到他们的个人资料。若被选中得到免费T恤的人，设置了他们的喜好颜色，那么他们会收到那种颜色的衣服。而若那人不曾指定喜好颜色，他们就会收到该公司当前数量最多那种颜色的体恤衫。

有许多方式实现这个业务逻辑。而对于这个示例，咱们将使用一个名为 `ShirtColor`，有着变种 `Red` 与 `Blue`（为简化目的而限制颜色数目）的枚举。咱们会以有着包含了表示当前库存中 T 恤衫颜色 `Vec<ShirtColor>` 的 `shirts` 字段的 `Inventory` 的结构体，表示该公司的当前库存。其中定义在 `Inventory` 上的方法 `giveaway`，会获取到免费体恤衫获得者的可选体恤衫颜色偏好，并返回那个人将得到的体恤衫颜色。下面清单 13-1 给出了这个设置：

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
        "选项为 {:?} 的用户，得到了 {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println! (
        "选项为 {:?} 的用户得到了 {:?}",
        user_pref2, giveaway2
    );
}
```

*清单 13-1：体恤衫公司派发情形*

定义在 `main` 中的 `store`，剩下两件蓝色 T 恤与一件红色 T 恤，用于本次限量版促销活动。咱们分别对选项为红色 T 恤，与没有偏好的两名用户，调用了 `giveaway` 方法。

再次说明，此代码可以许多方式实现，而这里，为了专注于闭包，故除了用到闭包的 `giveaway` 方法主体外，咱们都立足于已经学过的概念。在 `giveaway` 方法中，咱们以类型为 `Option<ShirtColor>` 的参数，而获取到用户偏好，并调用了 `user_preference` 上的 `unwrap_or_else` 方法。[`Option<T>` 上的 `unwrap_or_else` 方法](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or_else) 是由标准库定义的。他取一个参数：不带任何参数，返回一个值 `T` （与 `Option<T>` 的 `Some` 变种中存储的同一类型，此示例中即 `ShirtColor`）的闭包。在 `Option<T>` 为 `Some` 变种时，`unwrap_or_else` 就会返回 `Some` 里的那个值。而在 `Option<T>` 为 `None` 变种时，那么 `unwrap_or_else` 就会调用随后的闭包，并返回由该闭包所返回的值。

咱们指定了闭包表达式，closure expression， `|| self.most_stocked()`，作为 `unwrap_orelse` 的参数。这是个本身不取参数的闭包（如闭包有参数，参数就应出现在两条竖线之间）。该闭包的主体调用了 `self.most_stocked()`。咱们于此处定义出该闭包，而 `unwrap_or_else` 的实现就会在需要其结果时，执行这个闭包。

运行此代码会打印出：


```console
$ cargo run                                                         lennyp@vm-manjaro
   Compiling closure_demo v0.1.0 (/home/lennyp/rust-lang/closure_demo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.47s
     Running `target/debug/closure_demo`
选项为 Some(Red) 的用户，得到了 Red
选项为 None 的用户得到了 Blue
```

这里有个有趣的地方，即这里曾传递了一个调用了在当前 `Inventory` 实例上的 `self.most_stocked()` 的闭包。标准库并不需要明白有关这里所定义的 `Inventory` 或 `ShirtColor` 的任何事情，或者在此场景下这里要运用的那些逻辑。这个闭包捕获了到 `self` 这个 `Inventory` 实例的一个不可变引用，并将该不可变引用，传递给这里指定给那个 `unwrap_or_else` 方法的代码。与之相反，函数是无法以这种方式，捕获到他们的环境的（the closure captures an immutable reference to the `self` `Inventory` instance and pass it with the code we specify to the `unwrap_or_else` method. Functions, on the other hand, are not able to capture their environment in this way）。


## 闭包的类型推断与类型注解

**Closure Type Inference and Annotation**


函数与闭包之间，还有别的一些区别。闭包通常不要求咱们像 `fn` 函数那样，注解参数或返回值的类型。之所以在函数上要求类型注解，是因为类型是暴露给用户的显式接口的一部分。硬性要求定义出这种接口，对于确保所有人，在某个函数用到与返回的值类型上，达成一致尤为重要。而另一方面的闭包，则不是用在像这样的暴露接口中：他们被存储于变量中，而在无需对其命名及暴露给库用户下被用到。

闭包通常是短小的，并仅在较窄的上下文里，而非任何场景下都有意义。在这些受限的条件下，编译器就可以推断出参数与返回值的类型，类似于其能够推断出绝大多数变量类型的方式（同样也有极少数编译器需要闭包类型注解的情况）。

和变量一样，如果我们想增加明确性和清晰性，我们可以添加类型注释，代价是比严格意义上的必要更多的言语。注解闭包的类型，看起来会像是下面清单 13-2 中所给出的定义。在此示例中，咱们定义了一个闭包，并将其存储在变量中，而非清单 13-1 中咱们所做的，把闭包定义在咱们将其作为参数传递的地方。

文件名：`src/main.rs`

```rust
let expensive_closure = |num: u32| -> u32 {
    println! ("缓慢计算中......");
    thread::sleep(Duration::from_secs(2));
    num
}
```

*清单 13-2：在闭包中加上可选的参数与返回值类型的类型注解*


添加了类型注解后，闭包的语法看起来就更像函数的语法了。出于比较目的，这里咱们定义了把 `1` 加到参数的一个函数，与有着同样行为的一个闭包。咱们添加了一些空格，来对齐对应部分。这说明除了使用管道和大量的语法是可选的之外，闭包语法与函数语法是多么的相似：

```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 };
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

第一行给出了函数定义，而第二行给出了完整注解过的闭包定义。在第三行，咱们移除了闭包定义的类型注解。在第四行，由于闭包的主体只有一个表达式，因此咱们移出了那对可选的花括号。这些全都是在其被调用时，会产生出同样行为的有效定义。由于 `add_one_v3` 与 `add_one_v4` 中的类型将从他们的使用中推断出来，因此这两行就要求两个被执行的闭包能被编译出来。这与 `let v = Vec::new();` 需要类型注解，或需要有某种类型的值插入到这个 `Vec` 中，Rust 才能够推断出类型相似。

对于闭包定义，编译器将为其各个参数及其返回值，都推断出某种具体类型。举个例子，下面清单 13-3 给出了一个仅返回作为参数接收到的值的简短闭包定义。除这个示例外，这个闭包不是特别有用。请注意咱们没有添加任何类型注解到这个定义。由于没有类型注解，咱们可以用任何类型调用这个闭包，这里咱们第一次是以 `String` 类型调用的。若咱们随后尝试以整数调用 `example_closure` 时，就会得到一个报错。

文件名：`src/main.rs`

```rust
    let example_closure = |x| x;
    let s = example_closure(String::from(你好));
    let n = example_closure(5);
```

*清单 13-3：尝试以两种不同类型，调用类型为推断出的闭包*

编译器会给到我们如下错误：

```console
$ cargo run                                                                             lennyp@vm-manjaro
   Compiling closure-example v0.1.0 (/home/lennyp/rust-lang/closure-example)
error[E0308]: mismatched types
 --> src/main.rs:4:29
  |
4 |     let n = example_closure(5);
  |             --------------- ^- help: try using a conversion method: `.to_string()`
  |             |               |
  |             |               expected struct `String`, found integer
  |             arguments to this function are incorrect
  |
note: closure defined here
 --> src/main.rs:2:27
  |
2 |     let example_closure = |x| x;
  |                           ^^^

For more information about this error, try `rustc --explain E0308`.
error: could not compile `closure-example` due to previous error
```

咱们第一次是以 `String` 值调用的 `example_closure`，编译器便推断出该闭包的 `x` 与返回值类型均为 `String`。这些类型随后就被锁定于 `example_closure` 中的那个闭包里，而在咱们接下来尝试对同一闭包使用不同类型时，便得到了类型报错。


## 捕获引用抑或迁移所有权

**Capturing Reference or Moving Ownership**


闭包可以三种方式，捕获到其环境中的值，这直接对应了函数取得参数的三种方式：不可变地进行借用、可变地借用，与取得所有权，closures can capture values from their environment in three ways, which directly map to the three ways a function can take a parameter: borrowing immutably, borrowing mutably, and taking ownership。闭包将根据函数主体会对捕获值做些什么，而确定出要使用何种方式。

在下面清单 13-4 中，由于其只需不可变引用来打印出值，因此咱们定义了个捕获了到名为 `list` 矢量值的不可变引用的闭包：

文件名：`src/main.rs`

```rust
fn main() {
    let list = vec! [1, 2, 3];
    println! ("在定义闭包之前的：{:?}", list);

    let only_borrows = || println! ("自闭包打印出的：{:?}", list);

    println! ("在调用闭包之前：{:?}", list);
    only_borrows();
    println! ("在调用闭包之后：{:?}", list);
}
```

*清单 13-4：定义并调用捕获了不可变引用的闭包*

这个示例还演示了变量可绑定到闭包定义，且咱们随后可通过使用变量名字与圆括号对，犹如变量名是个函数名一样，调用闭包。

由于咱们可在同一时间，有着到 `list` 的多个不可变引用，因此在闭包定义前、闭包定义后而被调用前，及闭包调用后的代码中，在代码中 `list` 都是可访问的。此代码会编译、运行并打印出如下输出：

```console
$ cargo run                                                                             lennyp@vm-manjaro
   Compiling closure-example v0.1.0 (/home/lennyp/rust-lang/closure-example)
    Finished dev [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/closure-example`
在定义闭包之前的：[1, 2, 3]
在调用闭包之前：[1, 2, 3]
自闭包打印出的：[1, 2, 3]
在调用闭包之后：[1, 2, 3]
```

接下来，在下面清单 13-5 中，咱们修改了闭包主体，从而会添加一个元素到这个 `list` 矢量。闭包现在就会捕获可变引用：


文件名：`src/main.rs`

```rust
fn main() {
    let mut list = vec! [1, 2, 3];
    println! ("在定义闭包之前的：{:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println! ("在调用闭包之后：{:?}", list);
}
```

*清单 13-5：定义并调用会捕获可变引用的闭包*

此代码会编译、运行，并打印出：


```console
$ cargo run                                                                   lennyp@vm-manjaro
   Compiling closure-example v0.1.0 (/home/lennyp/rust-lang/closure-example)
    Finished dev [unoptimized + debuginfo] target(s) in 0.47s
     Running `target/debug/closure-example`
在定义闭包之前的：[1, 2, 3]
在调用闭包之后：[1, 2, 3, 7]
```

请注意在 `borrows_mutably` 的定义与调用之间，不再有 `println!`： `borrows_mutably` 被定义时，其就捕获了到 `list` 的可变引用。由于该闭包被调用后，咱们没有再使用那个闭包，因此这个可变借用就结束了。由于在有着可变借用，a mutable borrow，时不允许有其他的借用，因此在该闭包的定义与调用期间，打印那个 `list` 的不可变借用是不允许的。请尝试在那里添加一个 `println!`，来看看咱们会得到什么报错！

即使闭包主体不严格需要所有权，而咱们仍要强制闭包取得其用到的环境中值的所有权时，咱们可在参数清单前，使用 `move` 关键字。

当将闭包传递给新线程以移动数据以使其由新线程拥有时，此技术最有用。当咱们在第 16 章中讲到并发时，将详细讨论线程与为何要使用线程，而现在，咱们来粗略地探讨一下，运用一个需要 `move` 关键字的闭包，生成新线程。下面清单 13-6 给出了修改后在新线程而非主线程中，打印出矢量值的清单 13-4：

文件名：`src/main.rs`

```rust
use std::thread;

fn main() {
    let list = vec! [1, 2, 3];
    println! ("在定义闭包之前的：{:?}", list);

    thread::spawn(move || println! ("从线程打印出的：{:?}", list))
        .join()
        .unwrap();
}
```

*清单 13-6：使用 `move` 关键字，强制那个线程的闭包取得 `list` 的所有权*

咱们生成了一个新线程，给到线程一个闭包作为参数来运行，we spawn a new thread, giving the thread a closure to run as an argument。闭包的主体体会打印出清单。在代码清单 13-4 中，由于不可变引用是打印 `list` 所需的最低权限，因此闭包仅使用了不可变引用捕获 `list`。在这个示例中，即使闭包主体只需不可变引用，咱们仍需通过把 `move` 关键字放在闭包定义的开头，而指明 `list` 应被迁移到闭包中。新线程可能在主线程其余部分执行完毕前执行结束，也有可能主线程先结束。若主线程依然保有 `list` 的所有权，而主线程又在新线程结束之前就结束而弃用掉 `list`，那么新线程中的 `list` 就会成为无效。因此，编译器要求 `list` 要迁移到给新线程的闭包中，如此那个引用将有效。请尝试去掉 `move` 关键字，或在闭包被定义出后使用 `list`，来看看会得到什么样的编译器报错！


## 将捕获值迁出闭包与 `Fn` 特质

**Moving Captured Values Out of Closures and the `Fn` Traits**

一旦闭包捕获了引用，或捕获了环境中值的所有权（因此影响到被迁移 *进* 该闭包的任何物件），闭包主体中的代码，就会定义出闭包稍后被执行时，引用或值会发生什么（因此影响到被迁移 *出* 该闭包的相关项目，once a closure has captured a reference or captured ownership of a value from the environment where the closure is defined(thus affecting what, if anything, is moved *into* the closure), the code in the body of the closure defines what happens to the references or values when the closure is evaluated later(thus affecting what, if anything, is moved *out* of the closure)）。闭包主体可执行以下任意操作：

- 将捕获到的值迁移出闭包;

- 修改捕获到的值;

- 既不迁移也不修改该值;

- 或以不捕获环境中任何东西开始。

闭包捕获进而处理环境中值的方式，影响到闭包会实现哪个特质，而特质则是指函数与结构体，能指明他们可使用闭包类别的方式。依据闭包主体处理环境中值的方式，闭包会以累加样式，自动实现一个、两个，或全部三个的 `Fn` 特质，the way a closure captures and handles values from the environment affects which traits the closure implements, and traits are how functions and structs can specify what kinds of closures they can use. Closures will automatically implement one, two, or all three of these `Fn` traits, in an additive fashion, depending on how the closure's body handles the values：

1. `FnOnce` 特质适用于可被调用一次的闭包。由于全部闭包都可被调用，因此他们都至少实现了这个特质。而由于将捕获值迁移出其主体的闭包，只能被调用一次，因此这样的闭包将只实现 `FnOnce`，而不会实现其他 `Fn` 特质；

2. `FnMut` 特质适用于不会把捕获值迁出主体，但仍会修改捕获值的闭包。这些闭包可被多次调用；

3. `Fn` 则适用于不把捕获值迁出主体，且不修改捕获值的闭包，以及不从环境捕获任何东西的闭包。在不会修改其环境下，这些闭包可被多次调用，在诸如并发地多次调用闭包的情形中，这种调用方式就相当重要。

咱们来看看清单 13-1 中咱们曾用到的， `Option<T>` 上那个 `unwrap_or_else` 方法的定义：

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

回顾到 `T` 就是表示 `Optoin` 的 `Some` 变种中，值类型的泛型。类型 `T` 也是 `unwrap_or_else` 函数的返回值类型：比如，在 `Option<String>` 上调用 `unwrap_or_else` 的代码，就将得到一个 `String`。

接下来，请留意 `unwrap_or_else` 函数有个额外的泛型参数 `F`。`F` 类型是名为 `f` 的参数类型，其正是调用 `unwrap_or_else` 时，咱们提供的闭包。

泛型 `F` 上所指定的特质边界，the trait bound，为 `FnOnce() -> T`，表示 `F` 必须能被调用一次、不取参数，并要返回 `T` 类型值。在特质边界中使用 `FnOnce`，表示 `unwrap_or_else` 只会调用 `f` 最多一次的约束。在 `unwrap_or_else` 的主体中，咱们就可以看到，当 `Option` 为 `Some` 时，`f` 不会被调用。当 `Option` 为 `None` 时，`f` 就会被调用一次。由于所有闭包都实现了 `FnOnce`，`unwrap_or_else` 会接收最为广泛的闭包，而尽可能地灵活。

> 注意：函数也可实现全部三个 `Fn` 特质。当咱们打算执行的操作，不需要捕获环境中的值时，便可在需要实现了 `Fn` 特质的物件处，使用函数名字而非闭包。比如，在 `Option<Vec<T>>` 值上，若该值为 `None`，那么咱们就可以调用 `unwrap_or_else(Vec::new)` 来获取到一个新的空矢量值。

现在咱们来看看定义在切片上的标准库方法 `sort_by_key`，以看出其与 `unwrap_or_else` 有何区别，及为何 `sort_by_key` 会使用 `FnMut` 而非 `FnOnce` 作为特质边界。闭包会得到一个到正被处理切片中，当前元素引用形式的参数，并返回可被排序的类型 `K` 的值。在咱们想要以各个条目的某种特定属性，对切片进行排序时，这个函数是有用的。在下面清单 13-7 中，咱们有着一个 `Rectangle` 实例的清单，且咱们使用了 `sort_by_key`，来以 `width` 属性的升序，对 `Rectangle` 实例加以排序：

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
    println! ("以宽的升序排序：{:#?}", list);

    list.sort_by_key(|r| r.height);
    println! ("以高的升序排序：{:#?}", list);
}
```

*清单 13-7：使用 `sort_by_key` 来对矩形以宽和高分别排序*


此代码会打印出：

```console
$ cargo run                                                                                       lennyp@vm-manjaro
   Compiling closure-example v0.1.0 (/home/lennyp/rust-lang/closure-example)
    Finished dev [unoptimized + debuginfo] target(s) in 0.19s
     Running `target/debug/closure-example`
以宽的升序排序：
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
以高的升序排序：
[
    Rectangle {
        width: 10,
        height: 1,
    },
    Rectangle {
        width: 3,
        height: 5,
    },
    Rectangle {
        width: 7,
        height: 12,
    },
]
```

`sort_by_key` 被定义为取 `FnMut` 闭包的原因是，他会多次调用闭包：对切片中的每个条目调用一次。闭包 `|r| r.width` 不会捕获、修改，或从其环境迁迁出任何东西，因此其满足特质边界要求。

相比之下，清单 13-8 展示了一个只实现 FnOnce 特质的闭包的例子，因为他会把值移出环境。编译器不会让咱们在 `sort_by_key` 下使用这个闭包：

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
    let value = String::from("按照被调用到的 key");

    list.sort_by_key(|r| {
        sort_operations.push(value);
        r.width
    });
    println! ("{:#?}", list);
}
```

*清单 13-8：尝试在 `sort_by_key` 下使用 `FnOnce` 类型的闭包*

这是一个精心设计的、迂回的方法（并不奏效），试图计算排序列表时 `sort_by_key` 被调用的次数。这段代码尝试通过把 `value` -- 闭包环境中的一个 `String` -- 压入到 `sort_operations` 矢量，而完成这个计数。闭包会捕获 `value`，随后通过将 `value` 的所有权转移给 `sort_operations` 矢量，而把 `value` 迁出闭包。闭包可以被调用一次；由于 `value` 将不再位于其被再次压入到 `sort_operations` 的环境中，因此第二次尝试调用他将不会工作。那么，这个闭包就只实现了 `FnOnce`。在咱们尝试编译此代码时，就会得到由于闭包必须实现 `FnMut`，而因此 `value` 无法被迁出闭包的报错：

```console
$ cargo run                                                                                       lennyp@vm-manjaro
   Compiling closure-example v0.1.0 (/home/lennyp/rust-lang/closure-example)
error[E0507]: cannot move out of `value`, a captured variable in an `FnMut` closure
  --> src/main.rs:18:30
   |
15 |     let value = String::from("按照被调用到的 key");
   |         ----- captured outer variable
16 |
17 |     list.sort_by_key(|r| {
   |                      --- captured by this `FnMut` closure
18 |         sort_operations.push(value);
   |                              ^^^^^ move occurs because `value` has type `String`, which does not implement the `Copy` trait

For more information about this error, try `rustc --explain E0507`.
error: could not compile `closure-example` due to previous error
```

报错指向的是闭包主体中，把 `value` 迁出环境的那行。要修复这个问题，咱们需要修改闭包的主体，令其不将值迁出环境。要计算 `sort_by_key` 被调用的次数，在环境中保留一个计数器并在闭包主体中递增其值，是一种更直接的计算方法。下面清单 13-9 中的闭包，由于只捕获了到 `num_sort_operations` 计数器的可变引用，进而就可以被多次调用，其就会工作：

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
    println! ("{:#?}\n 在 {num_sort_operations} 次操作下被排序好的", list);
}
```

*清单 13-9：在 `sort_by_key` 下使用 `FnMut` 闭包是允许的*

在定义或使用用到了闭包的函数或类型时，`Fn` 特质非常重要。在下一节中，我们将讨论迭代器。许多迭代器方法，都会取闭包的参数，因此在继续学习时，请牢记这些闭包的细节！

> 注：将清单 13-8 的代码，只加入一个地址符号 `&`，而修改成下面这样，也是工作的。这就要想想是为什么了：）

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
    let value = String::from("按照被调用到的 key");

    list.sort_by_key(|r| {
        sort_operations.push(&value);
        r.width
    });
    println! ("{:#?}\n{:#?}", list, sort_operations);
}
```


（End）


