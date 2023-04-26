# 函数式编程语言特性：迭代器与闭包

Rust 的设计曾受到许多现有的语言和技术的启发，而一个显著的影响，便是 *函数式编程，functional programming*。以函数式风格编程，通常包括了通过把函数传入到参数中，或从其他函数返回函数，及将函数赋值给变量以便稍后执行等等，而将函数当作值使用，programming in a functional style often includes using functions as values by using functions as values by passing them in arguments, returning them from another functions, assigning them to variables for later execution, and so forth。

本章中，咱们不会讨论函数式编程是什么或不是什么的问题，而将讨论与许多通常被指为函数式编程语言中特性类似的 Rust 特性。

更具体地说，咱们将讲到：

- *闭包，closures*，可存储在变量中、类似函数的结构体;
- *迭代器，iterators*，处理元素序列的方式，a way of processing a series of elements;
- 如何使用闭包与迭代器，来改进第 12 章中的那个 I/O 项目；
- 闭包与迭代器的性能问题（剧透警告：他们比咱们可能想的要快！）。

咱们已经讲到过其他的一些 Rust 特性，诸如模式匹配与枚举等，也是受函数式编程影响的。由于掌握闭包与迭代器，是编写惯用、快速 Rust 代码的重要方面，因此咱们将把这整章，都用来讲解他们。


## 闭包：捕获其环境的匿名函数

**Closures: Anonymous Functions that Capture Their Environment**

Rust 的闭包，是一些咱们可将其保存在变量中，或将其作为参数传递给其他函数的匿名函数。咱们可在一处创建出闭包，随后在别处调用该闭包，而在不同上下文中执行他，evaluate it。与函数不同，闭包可捕获到他们于其中被定义作用域的值。随后咱们将演示这些闭包特性，怎样实现代码重用与行为定制，unlike functions, closures can capture values from the scope in which they're defined. We'll demonstrate how these closure features allow for code reuse and behavior customization。


### 使用闭包捕获环境

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


### 闭包的类型推断与注解

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


### 捕获引用抑或迁移所有权

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


### 将捕获值迁出闭包与 `Fn` 特质

**Moving Captured Values Out of Closures and the `Fn` Traits**

一旦闭包捕获了某个引用，或捕获了其被定义处环境中某个值的所有权（从而影响到被迁移 *进* 该闭包的相关项目），该闭包函数体中的代码，就会定义出闭包稍后被执行时，对这些引用或值会发生什么（因此而影响到那些被迁移 *出* 该闭包的相关项目，once a closure has captured a reference or captured ownership of a value from the environment where the closure is defined(thus affecting what, if anything, is moved *into* the closure), the code in the body of the closure defines what happens to the references or values when the closure is evaluated later(thus affecting what, if anything, is moved *out* of the closure)）。闭包函数体可执行以下的任意操作：将捕获到的值迁移出闭包、修改捕获到的值、既不迁移也不修改值，或以不从其环境捕获任何东西开始。

闭包捕获及处理环境中的值的方式，影响到闭包实现了哪个特质，而特质则是指函数与结构体，能够以怎样的方式，指明他们可以使用的闭包类别。依据闭包函数体处理环境中值的不同方式，闭包会以累加方式，自动实现一个、两个，或全部三个的这些 `Fn` 特质（the way a closure captures and handles values from the environment affects which traits the closure implements, and traits are how functions and structs can specify what kinds of closures they can use. Closures will automatically implement one, two, or all three of these `Fn` traits, in an additive fashion, depending on how the closure's body handles the values）：

1. `FnOnce` 特质适用于那些可被调用一次的闭包。由于全部闭包都可被调用，因此他们都起码实现了这个特质。而由于将捕获到的值迁移出其函数体的闭包，只能被调用一次，因此这样的闭包将只要实现 `FnOnce`，而不会实现其他的那些 `Fn` 特质；

2. `FnMut` 特质适用于不将捕获值迁移出函数体，但仍会修改捕获值的那些闭包。这些闭包可被多次调用；

3. `Fn` 则适用于不将捕获值迁移出函数体的闭包，与不修改捕获值的闭包，以及那些不从其环境捕获任何东西的闭包。这些闭包在不会修改其环境之下，可被多次调用，在诸如并发地多次调用某个闭包情形中，这种的调用方式就相当重要。

下面来看看清单 13-1 中用到的， `Option<T>` 上那个 `unwrap_or_else` 方法的定义：

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

回顾到那个 `T` 即为表示某个 `Optoin` 的 `Some` 变种中值类型的泛型。那个类型 `T` 同样是这个 `unwrap_or_else` 函数的返回值类型：比如，在某个 `Option<String>` 上调用 `unwrap_or_else` 的代码，就会得到一个 `String`。

接着，请注意这个 `unwrap_or_else` 函数有个额外的泛型参数 `F`。这个 `F` 类型为名为 `f` 的参数类型，而这个参数 `f`，则正是在调用 `unwrap_or_else` 时，所提供到的那个闭包。

在这个泛型 `F` 上所指定的特质边界（the trait bound），为 `FnOnce() -> T`，就表示 `F` 必须能被调用一次、不取参数，并要返回一个 `T` 类型值。在特质边界中使用 `FnOnce`，表示 `unwrap_or_else` 只会调用 `f` 至多一次这样的约束。而在 `unwrap_or_else` 的函数体中，就可以看到在那个 `Option` 为 `Some` 时，`f` 不会被调用。在 `Option` 为 `None` 时，`f` 就会被调用一次。由于所有闭包都实现了 `FnOnce`，因此 `unwrap_or_else` 就会接收最为广泛的闭包，进而有着其最大的灵活性。

> **注意**：函数也可以实现全部三个 `Fn` 特质。在打算执行的操作，不需要捕获环境中某个值时，就可以在需要某个实现了 `Fn` 特质处，使用某个函数名字而非闭包。比如，在某个 `Option<Vec<T>>` 值上，若这个值为 `None`，那么就可以调用 `unwrap_or_else(Vec::new)` 来获取到一个新的空矢量值。

现在来检视一下定义在切片上的那个标准库方法 `sort_by_key`，来看看其与 `unwrap_or_else` 有何区别，以及为何 `sort_by_key` 会使用 `FnMut` 而非 `FnOnce` 作为特质边界。其中的闭包，获得的是一个到正被处理切片中当前元素的引用形式的参数，并返回一个可被排序类型 `K` 的一个值。在想要以各个条目的某种特定属性，对切片进行排序时，此函数是有用的。在下面清单 13-7 中，就有一个 `Rectangle` 实例的清单，进而这里使用了 `sort_by_key`，来以 `width` 属性的升序对这些 `Rectangle` 实例加以排序：

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

`sort_by_key` 之所以被定义为取一个 `FnMut` 闭包，是由于他多次调用了这个闭包：对那个切片中的每个条目运行一次。那个闭包 `|r| r.width` 并不会捕获、修改，或是从其环境迁移任何东西，因此他就满足了 `FnMut` 这个特质边界的那些要求。

作为对照，下面清单 13-8 给出一个仅实现了 `FnOnce` 特质的闭包示例，正是因为该闭包将值迁移出了其环境。编译器不会允许咱们在 `sort_by_key` 下使用这个闭包：

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

*清单 13-8：尝试在 `sort_by_key` 下使用某个 `FnOnce` 类型的闭包*

这是一种人为杜撰的、错综复杂（而不会工作）的，来尝试计算 `sort_by_key` 在对 `list` 进行排序时，被调用次数的方法。此代码试图通过将该闭包所处环境中的 `value` -- 这样一个 `String`，压入到那个 `sort_operations` 矢量，而完成这样的计数。该闭包会捕获到 `value`，随后通过将 `value` 的所有权转移给 `sort_operations` 矢量，而将 `value` 迁移出这个闭包。这个闭包是可以调用一次的；由于 `value` 将不再位于那个其被再次压入到 `sort_operations` 的环境中，因此第二次的尝试调用他，就不会工作了。因此这个闭包就只实现了 `FnOnce`。在尝试编译此代码时，就会得到由于那个闭包必须实现 `FnMut`，因此 `value` 无法被迁移出那个闭包的错误：

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

错误指向的是那个闭包函数体中，将 `value` 迁移出环境的那一行。为了修复这个问题，这里就需要对该闭包的函数体加以修改，从而令其不会将值迁移出环境。为对 `sort_by_key` 被调用次数加以计数，就要在环境中保留一个计数器，并在闭包函数体中增加该计数器的值，就是计算 `sort_by_key` 被调用次数的一种更为直接了当的方式。下面清单 13-9 中的闭包，由于其只捕获了到那个 `num_sort_operations` 计数器的可变引用，进而由此就可以被多次调用，那么这个闭包就会工作：

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

*清单 13-9：在 `sort_by_key` 下使用一个 `FnMut` 闭包是允许的*

在定义那些运用到闭包的函数或类型时，这些 `Fn` 特质是相当重要的。在下一小节中，就会讨论到迭代器。许多迭代器的方法，都取的是闭包参数，因此请在继续学习时，牢记这些闭包的细节！

> **注**：将清单 13-8 的代码，只加入一个地址符号 `&`，而修改成下面这样，也是工作的。这就要想想是为什么了：）

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


## 使用迭代器对条目系列进行处理

**Processing a Series of Items with Iterators**

迭代器模式，实现了在条目序列上依次执行某些任务。迭代器负责着对各个条目进行遍历，及判断序列在什么时候结束的逻辑。在运用迭代器时，就不必自己再实现那样的逻辑了。

在 Rust 中，迭代器是 *惰性的（lazy）*，即在调用那些消费该迭代器以用完他之前，他们是没有效果的。比如，下面清单 13-10 中的代码就通过调用定义在 `Vec<T>` 上的 `iter` 方法，而创建出了一个在那个矢量 `v1` 中各个条目上的迭代器。此代码本身并不会执行任何有用的事情。

```rust
    let v1 = vec! [1, 2, 3];
    let v1_iter = v1.iter();
```

*清单 13-10：创建出一个迭代器*

其中的迭代器，是存储在那个 `v1_iter` 变量中的。一旦创建出了某个迭代器，就可以多种方式对其加以使用了。在第 3 章中的清单 3-5 中，就曾使用了一个 `for` 循环，对一个数组进行过迭代，而在该数组的各个条目上执行一些代码。而在使用 `for` 循环这表象之下，则是隐式地创建出了一个迭代器，并于随后消费了这个迭代器，不过在本小节之前，只是并未提及其确切工作原理。

下面清单 13-11 中的示例里，就把迭代器的创建，从那个 `for` 循环中迭代器的使用分离开来了。在使用 `v1_iter` 中的迭代器，调用那个 `for` 循环时，在那个迭代器中的各个元素，就在那个循环的一个个迭代中，被使用了，而这就会打印出各个值。

```rust
    let v1 = vec! [1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println! ("得到了：{}", val);
    }
```

*清单 13-11：在 `for` 循环中使用迭代器*

在那些不具备由语言标准库所提供迭代器的编程语言中，就会大概率要通过一个从 `0` 开始的索引变量，而使用那个变量，来进入到该矢量值中，以获取到一个值，并在一个循环中对这个索引变量值进行递增，直到该索引变量达到该矢量中全部条目数为止。

迭代器为咱们处理了这全部的逻辑，从而减少了可能潜在会出错的那些重复代码。对于许多不同类别的序列，而不光是那些可以索引进去的数据结构，比如矢量值，迭代器都能给到运用这同样逻辑的更多灵活性。接下来就要检视一下迭代器是怎样做到这样的。

### `Iterator` 特质与 `next` 方法

所有迭代器都实现了一个定义在标准库中、名为 `Iterator` 的特质。该特质的定义看起来像下面这样：


```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // 这里省略了那些有着默认实现的方法
}
```

请注意此定义使用了一些新的语法：`type Item` and `Self::Item`，他们定义着这个特质的一个 *关联类型（associated type）*。将在后面的第 19 章中，深入讲到关联类型。至于现在，要了解的全部，即此代码表明了这个 `Iterator` 特质的实现，有个还要定义一个 `Item` 类型的前提要求，且这个 `Item` 类型，会被用在其中的 `next` 方法返回值类型中。也就是说，这个 `Item` 类型将是该迭代器所返回的类型。

`Iterator` 特质只要求实现者（implementors）定义一个方法：即这个 `next` 方法，他会一次返回一个，封装在 `Some` 中该迭代器的条目，且在迭代完毕时，就返回 `None`。

是可以直接调用迭代器上的这个 `next` 方法的；下面清单 13-12 演示了，重复调用创建自该矢量的那个迭代器上的 `next` 方法，会返回些什么样的值。

文件名：`src/lib.rs`

```rust
#[test]
fn iterator_demonstration() {
    let v1 = vec! [1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq! (v1_iter.next(), Some(&1));
    assert_eq! (v1_iter.next(), Some(&2));
    assert_eq! (v1_iter.next(), Some(&3));
    assert_eq! (v1_iter.next(), None);
}
```

*清单 13-12：对迭代器上的 `next` 方法进行调用*


请注意这里需要将 `v1_iter` 构造为可变的：调用某个迭代器上的 `next` 方法，会修改那个迭代器用于追踪其本身位于其关联序列何处的内部状态。也就是说，此代码 *消费（consumes）*，或用完（use up）了这个迭代器。每次对 `next` 的调用，都会从这个迭代器吃掉一个条目。之所以之前在使用 `for` 循环时，未曾需要将 `v1_iter` 构造为可变，是由于那个循环占据了 `v1_iter` 的所有权，而在幕后将其构造为了可变。

还要注意这里从到 `next` 的调用所获取到的那些值，都是到那个矢量中值的不可变引用。`iter` 方法产生出的，是一个不可变引用的迭代器。而在打算创建一个会取得 `v1` 所有权的迭代器，并返回自有数据时，则可以调用 `into_iter` 而非 `iter`。与此类似，当需要可变引用的迭代时，那么就可以调用 `iter_mut` 而非 `iter`。


### 消费迭代器的一些方法

`Iterator` 特质有着带有由标准库提供了默认实现的几个方法；通过查阅 `Iterator` 特质的标准库 API 文档，就可以找到这些方法。他们中的一些，就在他们的定义中调用了这个 `next` 方法，这正是在实现这个 `Iterator` 特质时，不要求实现这个 `next` 方法的原因。

由于调用这些调用了 `next` 方法，会耗尽迭代器，因此他们被称为 *消费适配器（consuming adaptors）*。一个例子便是 `sum` 方法，他会取得迭代器的所有权，并通过重复调用 `next`，遍历所有条目，由此消费该迭代器。在其遍历期间，他就会把各个条目，加到一个运行中的总和，并在遍历完成时返回这个总和。下面清单 13-13，有着这个 `sum` 方法运用的测试演示：

文件名：`src/lib.rs`

```rust
#[test]
fn iterator_sum() {
    let v1 = vec! [1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq! (total, 6);
}
```

*清单 13-13：调用 `sum` 方法来获取迭代器中全部项目的总和*

由于 `sum` 会取得那个在其上调用他的迭代器的所有权，因此在到 `sum` 的调用之后，是不允许使用 `v1_iter` 的。


### 产生其他迭代器的方法

*迭代器适配器（iterator adaptors）* 是一些定义在 `Iterator` 特质上，不会消费迭代器的方法。相反，他们会通过改变初始迭代器的某些方面，而产生出别的一些迭代器。

下面清单 13-17 给出了一个调用迭代器适配器方法 `map` 的示例，该方法会取得在迭代器条目被遍历时在各个条目上调用的闭包。这个 `map` 方法会返回一个产生出修改后的那些条目的一个新迭代器。此示例中的闭包，会创建一个其中原矢量的各个条目被增加 `1` 了的新迭代器：

文件名：`src/main.rs`

```rust
    let v1 = vec! [1, 2, 3];

    v1.iter().map(|x| x + 1);
```

*清单 13-14：调用迭代器适配器 `map` 来创建出一个新迭代器*

然而，此代码会产生一条告警：

```console
$ cargo run                                                                                      lennyp@vm-manjaro
   Compiling iterator_demo v0.1.0 (/home/lennyp/rust-lang/iterator_demo)
warning: unused `Map` that must be used
 --> src/main.rs:4:5
  |
4 |     v1.iter().map(|x| x + 1);
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_must_use)]` on by default
  = note: iterators are lazy and do nothing unless consumed

warning: `iterator_demo` (bin "iterator_demo") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.36s
     Running `target/debug/iterator_demo`
```

清单 13-14 中的代码，并未执行任何事情；那里所指定的闭包不曾被调用过。那条告警提示了缘由：迭代器适配器是惰性的，进而在此就需要消费该迭代器。

为修正此告警而消费那个迭代器，这里将使用 `collect` 方法，之前曾在第 12 章的清单 12-1 中，对 `env::args` 用到过该方法。此方法会消费那个迭代器，并将那些结果值，收集到一个集合数据类型中。

下面清单 13-15 中，就把在那个自 `map` 的调用所返回的迭代器遍历的结果，收集到了一个矢量值中。这个矢量将以包含原始矢量的各个条目增加 `1` 后的各个条目。

文件名：`src/main.rs`

```rust
    let v1 = vec! [1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq! (v2, vec! [2, 3, 4]);
```

*清单 13-15：调用 `map` 方法来创建出一个新迭代器，并于随后调用 `collect` 方法来消费该新迭代器而创建出一个矢量值*

由于 `map` 取了一个闭包，因此这里就可以指定咱们要在各个条目上执行的任何操作了。对于阐明闭包在重用 `Iterator` 特质所提供的遍历行为的同时，怎样实现定制一些行为来讲，这可是一个极佳的示例了。

可将多个调用，链接到迭代器适配器，来以可读方式，执行一些复杂操作。但由于所有迭代器都是惰性的，因此就必须调用一个消费适配器方法（one of the consuming apdaptor methods），来获取到迭代器适配器调用的结果。


### 使用捕获了其所在环境的闭包

**Using Closures that Capture Their Environment**

相当多的迭代器适配器，都会将闭包取作参数，且通常作为参数指定给迭代器适配器的那些闭包，都将是些会捕获他们环境的闭包。

这里将使用会取一个闭包的 `filter` 方法，来作为这方面的示例。改闭包会从其所在迭代器获取到一个条目，并返回要给 `bool`。在闭包返回 `true` 时，那个条目值就将被包含在由 `filter` 产生出的迭代中。在该闭包返回 `false` 时，那个条目值则不会被包含。

下面清单 13-16 中，使用了有着捕获其环境中 `shoe_size` 变量的一个闭包的 `filter` 方法，来对一个 `Shoe` 结构体实例的集合进行迭代。他将只返回特定尺码的那些鞋子。

文件名：`src/lib.rs`

```rust
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn filter_by_size() {
        let shoes = vec! [
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq! (
            in_my_size,
            vec! [
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
            ]
        );
    }
}
```

*清单 13-16：使用带有一个捕获 `shoe_size` 闭包的 `filter` 方法*

这里的 `shoes_in_size` 函数将一个 `shoes` 矢量的所有权，和一个 `shoe_size` 取作参数。他会返回只包含特定尺码鞋子的一个矢量。

而在 `shoes_in_size` 的函数体中，这里调用了 `into_iter`，来创建出取得那个矢量所有权的迭代器。随后这里调用了 `filter` 来将那个迭代器，适配为只包含令到那个闭包返回 `true` 那些元素的一个新迭代器。

其中的闭包，就从其环境中捕获到他的 `shoe_size` 参数，并将该值于各鞋子的尺码比较，而只保留特定尺码的那些鞋子。最后，这里调用 `collect` 方法，将由那个已适配出的迭代器返回的 `Shoe` 类型值，收集到一个矢量中，`shoes_in_size` 这个函数返回的，便是这个矢量值。

这个测试显示，在调用 `shoes_in_size` 时，取回的仅是那些有着与所指定值同样尺寸的鞋子。


## 改进那个 I/O 项目

有了迭代器这方面的新知识，这里就可以经由迭代器，改进第 12 章中的那个 I/O 项目，使得代码各处更清楚并更简洁。下面就来看看，迭代器可以怎样改进其中 `Config::build` 函数与 `search` 函数的实现。


### 使用迭代器消除 `clone`

代码清单 12-6 中，那里曾添加了取 `String` 值的切片，并通过索引到那个切片中并克隆出一些值，从而创建出一个允许拥有这些值的 `Config` 结构体实例的代码。下面清单 13-17 中，就重现了正如清单 12-23 中，那个 `Config::build` 函数的实现：

文件名：`src/lib.rs`

```rust
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("参数数量不足");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
```

*清单 13-17：清单 12-23 中 `Config::build` 函数的重现*

那个时候，就曾说过不要对其中的那两个低效率 `clone` 调用担心，因为将来会把他们消除掉。那么这个时候就是消除他们的时机！

之所以这里需要 `clone` 方法，是由于这里在参数 `args` 中，有着由一些 `String` 元素构成的切片，而这个 `build` 函数并不拥有 `args`。为返回某个 `Config` 实例的所有权，那里就不得不克隆 `Config` 结构体的 `query` 与 `filename` 字段，如此该 `Config` 实例就能拥有他的这些值。

有了关于迭代器的新知识，那么就可以将这个 `build` 函数，修改为取得某个迭代器的所有权作为其参数，而不再是借用一个切片。与其使用对切片长度加以检查，以及所有进入到特定位置的那些代码，这里将运用迭代器功能。由于迭代器将访问到那些值，那么这样就将厘清这个 `Config::build` 函数，正在完成些什么事情。

一旦 `Config::build` 取得那个迭代器的所有权，而停止使用借用到的索引操作，你们这里就可以把那个迭代器中的那些 `String` 值，迁移到 `Config` 里去，而非调用 `clone` 方法并构造一个新的内存分配了。


### 直接使用返回的迭代器

请打开之前 I/O 项目的 `src/main.rs` 文件，他看起来应是这样的：

文件名：`src/main.rs`


```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln! ("解析参数时遇到问题：{err}");
        process::exit(1);
    });

    // --跳过代码--
}
```

这里将首先把在清单 12-24 中的那个 `main` 函数的开头，修改为下面清单 13-18 中，使用迭代器的样子。同样，在更新 `Config::build` 之前，这不会编译。

文件名：`src/main.rs`

```rust
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln! ("解析参数时遇到问题：{err}");
        process::exit(1);
    });

    // --跳过代码--
}
```

*清单 13-18：把 `env::args` 的返回值传递给 `Config::build`*

这个 `env::args` 函数，返回的是个迭代器！不再是将迭代器的值收集到某个矢量，而在随后把一个切片传递给 `Config::build` 了，现在这里直接吧返回自 `env::args` 的迭代器所有权，传递给 `Config::build`。

接下来，这里就需要更新 `Config::build` 的定义。在这个 I/O 项目的 `src/data_structures.rs` 文件中，接下来就要想下面这样，修改 `Config::build` 的函数签名。由于尚需更新该函数的函数体，因此这仍不会编译。

文件名：`src/lib.rs`

```rust
impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        // --跳过代码--
```

*清单 13-19：将 `Config::build` 的函数签名，更新为期待得到一个迭代器*

`env::args` 函数的标准库文档显示，其所返回的迭代器类型为 `std::env::Args`，而那个类型是实现了 `Iterator` 特质的，同时返回的是一些 `String` 值。

这里已经更新了这个 `Config::build` 函数的签名，那么参数 `args`，就有了一个带有特质边界 `impl Iterator<Item = String>` 的泛型，而不再是 `&[String]` 类型了。第 10 章 [作为参数的特质](Ch10_Generic_Types_Traits_and_Lifetimes.md#作为参数的特质) 小节曾讨论过的这种 `impl Trait` 语法的用法，表示 `args` 可以是任何实现了 `Iterator` 类型，并返回一些 `String` 条目的类型。

由于这里取得了 `args` 的所有权，且这里通过对 `args` 进行迭代，而将对其进行修改，因此这里可把 `mut` 关键字，添加到这个 `args` 参数的说明中，来将其构造为可变的。


### 使用 `Iterator` 特质的方法取代原先的索引操作

**Using `Iterator` Trait Methods Instead of Indexing**

接下来，这里将修正 `Config::build` 的函数体。由于 `args` 实现了 `Iterator` 特质，因此就明白这里可以在他上面调用 `next` 方法！下面清单 13-20 将清单 12-23 中的代码，更新为了使用 `next` 方法：

文件名：`src/data_structures.rs`

```rust
impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("未曾获取到查询字串"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("未曾获取到文件路径"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
```

*清单 13-20：将 `Config::build` 的函数体，修改为使用迭代器方法*

请记住 `env::args` 返回值中的第一个，是程序的名字。这里是要忽略那个值，而到下一值处，因此这里首先调用了 `next` 并对该返回值什么也没做。其次，这里调用了 `next` 来获取到这里打算将其放入 `Config` 的 `query` 字段的那个值。在 `next` 返回的是一个 `Some` 时，这里使用了一个 `match` 来提取该值。在其返回的是 `None` 时，就表示没有给到足够的参数，同事这里及早地返回了一个 `Err` 值。对于 `filename` 值，这里进行了同样的处理。


### 使用迭代器适配器令到代码更清晰

**Making Code Clearer with Iterator Adaptors**

在这个 I/O 项目的 `search` 函数中，也可以利用到迭代器的优势，该函数重现于下面清单 13-21 中，如同其曾在清单 12-19 中那样：

文件名：`src/lib.rs`

```rust
pub fn search<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
```

*清单 13-21：清单 12-19 中 `search` 函数的实现*

这里可使用迭代器适配器方法，以更精练方式编写出此代码。这样做还实现了避免使用一个可变的中间 `results` 矢量。函数式编程风格（the functional programming style），偏好于将可变状态的数量最小化，从而令到代码更简明。移除掉这个可变状态，可开启使得搜索以并行方式进行的一项未来功能增强，这是由于这里将不再必须对到这个 `results` 矢量的并发访问加以管理。下面清单 13-22 给出了这一修改：

文件名：`src/lib.rs`

```rust
pub fn search<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
```

*清单 13-22：在 `search` 函数中使用迭代器适配器方法*

回顾这个 `search` 函数的目的，即为返回 `contents` 中所有包含 `query` 的那些行。与清单 13-16 中的 `filter` 示例类型，此代码使用了这个 `filter` 适配器，来只保留 `line.contains(query)` 返回 `true` 的那些行。这里随后使用 `collect()`，收集那些匹配的行。这就简单多了！请在 `search_case_insensitive` 函数中，也完成这同样的改造。

> 函数 `search_case_insenstitive` 修改后如下所示：


```rust
pub fn search_insensitive<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}
```

### 选择循环还是迭代器

**Choosing Between Loops or Iterators**

接着合乎逻辑的问题，便是在所编写代码中，应选取何种样式以及为什么要选择那种样式：是要清单 13-21 原先的实现，还是要清单 13-22 中用到迭代器的版本。绝大多数 Rust 程序员，都首选使用迭代器的样式。开始的时候要掌握这种诀窍有点难，不过一旦摸清各种迭代器适配器以及他们完成的事情，那么迭代器就能较容易地掌握。与其摆弄循环的各种东西，并构建出一些新的矢量值，运用迭代器的代码，关注的则是循环的高级别目标。那么运用迭代器就把一些常见代码，给抽象了出来，如此就更易于看出，特定于该代码的一些概念了，比如迭代器中的各个元素，所必须通过的那种过滤条件。

然而这两种实现真的等价吗？直觉上的假定，可能是其中更低级别的循环，将会更快。那么下面就来讨论性能吧。


## 性能比较：循环与迭代器

**Comparing Performance: Loops vs. Iterators**

为确定出是要是要循环，还是使用迭代器，咱们需要搞清楚以下哪种实现更快：带有显式 `for` 循环的 `search` 函数，还是有着迭代器的那个版本。

这里通过加载 Sir Arthur Conan Doyle 写的整部 *福尔摩斯历险记（The Adventures of Sherlock Holmes）* 到一个 `String` 中，并在这些内容里查找单词 *the*，运行了一个基准测试。下面就是这个基准测试，分别在使用 `for` 循环版本，与使用迭代器版本的 `search` 函数上的测试结果：

```console
test bench_search_for  ... bench:  19,620,300 ns/iter (+/- 915,700)
test bench_search_iter ... bench:  19,234,900 ns/iter (+/- 657,200)
```

迭代器版本就要稍微快一些！由于这里关键不是要证实这两个版本等价，而是要对怎样从性能方面对两种实现加以比较，有个粗略认知，因此这里不会对这个基准测试代码加以解释（we won't explain the benchmark code here, because the point is not to prove that the two versions are equivalent but to get a general sense of how these two implementations compare performance-wise）。

对于更为全面的基准测试，那么就应使用不同大小的各种文本作为其中的 `contents`，不同单词与不同长度单词作为那个 `query`，以及所有类别的其他差异。关键在于这里：尽管迭代器属于高级别的抽象，但他们会被向下编译为如同咱们自己所编写的低级别代码。迭代器是 Rust 的那些 *无代价抽象（zero-cost abstractions）* 之一，这就意味着这种抽象的使用，并不会承担额外的运行时开销。这与 C++ 最初设计者与实现者 Bjarne Stroustrup 在 “Foundation of C++”（2012） 那本书中，所定义的 *无开销（zero-overhead）* 概念类似：

> 总的来说，C++ 的众多实现，都遵循了无开销原则：用不到的东西，就无需付出代价。并更进了一步：即使用到，亦不会手搓出更良好的代码（the zero-overhead principle: What you don't use, you don't pay for. And further: What you do use, you couldn't hand code any better）。

作为另一个示例，以下代码取自某个音频解码器。其中的解码算法，使用了线性预测的数学运算，来根据先前的一些样本，估算出后面的值。此代码使用了迭代器链（an iterator chain），来完成作用域中三个变量：数据的一个 `buffer` 切片，12 个 `coeffecients` 的一个数组，及存储在 `qlp_shift` 中对数据进行偏移的数量，上的一些数学计算。这里已声明出了该示例中的那些变量，但并未给到他们任何值；尽管此代码在其上下文外部并无多少意义，但他仍不失为 Rust 如何将高级别的一些概念，翻译为低级别代码的一个简练的、真实世界下的示例。

```rust
let buffer: &mut [i32];
let coefficients: [i64; 12];
let qlp_shift: i16;

for i in 12..buffer.len() {
    let prediction = coefficients.iter()
                                 .zip(&buffer[i - 12..i])
                                 .map(|&c, &s| c * s as i64)
                                 .sum::<i64>() >> qlp_shift;
    let delta = buffer[i];
    buffer[i] = prediction as i32 + delta;
}
```

为了计算出 `prediction` 的值，此代码对 `coefficients` 中 12 个值的每个都进行了迭代，并使用 `zip` 方法将这些系数值与 `buffer` 中的前 12 个值配对起来。随后对这个每个数值对，这里将他们放一起做乘法，对这些结果求和，并将和数中的那些二进制位，往右偏移了 `qlp_shift` 个二进制位。

像是音频解码器这样得应用中的那些计算，通常要将性能放在最高的优先级。这里，就创建出了一个迭代器，使用了两个适配器，并在随后消费了那个值。而这段代码会编译到什么样的汇编代码呢？当然，在这本书编写时，他会向下编译到由手写的同样汇编代码。注意相应于对 `coefficients` 中那些值的迭代，是完全没有循环的：Rust 清楚那里有 12 次迭代，因此他（Rust 编译器）就会 “解开（unrolls）” 其中那个循环。所谓 “解开（unrolling）”，是消除循环控制代码方面的开销，而以生成该循环历次迭代的重复代码予以取代的一种优化（*unrolling* is an optimization that removes the overhead of the loop controlling code and instead generates repetitive code for each iteration of the loop）。

全部的这些系数，都是被存储在寄存器中的，这意味着访问这些值是极为快速的。运行时是没有数组上的边界检查的。这些 Rust 所能运用的全部优化，就令到最终代码极为高效。既然现在获悉到这一点，那么就可以毫无顾忌的使用迭代器和闭包了！他们使得代码看起来像是在较高的层级，但这样做并不会造成运行时性能下降。



## 本章小结

闭包与迭代器，是 Rust 的两项受函数式编程概念启发的特性。他们带来了 Rust 的有着底层代码性能、但以高级语言清晰表达概念的能力。闭包与迭代器的实现，不会影响到运行时性能。这正是 Rust 致力于提供到无代价抽象（zero-cost abstractions），这一目标的一个方面。

既然现在已经改进了这个 I/O 项目的表现力，那么接下来就要看看，`cargo` 的一些别的、将帮助咱们与外界分享这个项目的一些特性了。
