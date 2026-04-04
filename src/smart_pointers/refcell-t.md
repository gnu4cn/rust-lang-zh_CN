# `RefCell<T>` 与内部可变性模式

所谓 *内部可变性，interior mutability*，属于 Rust 中的一种设计模式，在即使存在对数据的不可变引用时，也允许咱们修改该数据；通常情况下，这种操作是借用规则所不允许的。为了修改数据，这种模式在数据结构内部使用 `unsafe` 代码，绕过 Rust 管理修改和借用的通常规则。不安全代码向编译器表明，我们正在手动检查规则，而非依赖编译器为咱们检查；我们将在第 20 章中进一步讨论不安全代码。

尽管编译器无法保证借用规则在运行时会得以遵守，但只要我们可以确保这点，我们就可以使用用到内部可变性模式的类型。此时，所涉及的不安全代码会被封装于安全的 API 中，而外层类型仍然是不可变的。

咱们来通过研究遵循内部可变性模式的 `RefCell<T>` 类型，探讨这一概念。


## 在运行时强制执行借用规则检查

与 `Rc<T>` 不同，`RefCell<T>` 类型表示对其包含的数据的单一所有权。那么，是什么使 `RefCell<T>` 不同于 `Box<T>` 这样的类型呢？回顾咱们在第 4 章中学到的 [借用规则](../ownership/references_and_borrowing.md#引用的规则)：

- 在任何给定时间，咱们都可以 *要么* 有着一个可变引用，*或者* 任意数量的不可变引用（但不能两者兼有）；
- 引用必须始终有效。

在引用与 `Box<T>` 下，借用规则的这两项不变性在编译时强制执行。而在 `RefCell<T>` 下，这两项不变性在运行时强制执行。对于引用，当咱们破坏了这两条规则，咱们将得到编译器报错。而在 `RefCell<T>` 下，当咱们破坏这两条规则时，咱们的程序将终止运行并退出。

在编译时检查借用规则的好处是，错误将在开发过程中被尽早捕获，并且由于所有分析都已提前完成，而没有对运行时性能的影响。出于这些原因，在编译时检查借用规则在大多数情况下都是最佳选择，这就是为什么这是 Rust 的默认设置。

相反，在运行时检查借用规则的优势在于，此时某些内存安全的场景被允许，而这些场景不会被编译时的检查所允许。像 Rust 编译器这样的静态分析，本质上是保守的。代码的某些属性是无法通过分析代码来检测的：最著名的例子就是 [停机问题](https://zh.wikipedia.org/wiki/%E5%81%9C%E6%9C%BA%E9%97%AE%E9%A2%98), 虽然这超出了本书的范围，但却是个值得研究的有趣主题。

由于有些分析是不可能的，当 Rust 编译器无法确定代码是否符合所有权规则时，他可能会拒绝某个正确的程序；从这方面讲，他是保守的。若 Rust 编译器接受了某个错误的程序，用户就无法信任 Rust 做出的保证。然而，当 Rust 拒绝某个正确的程序时，虽然程序员会感到不便，但不会发生灾难性的后果。当咱们确信代码遵循了借用规则，而编译器却无法理解和保证这点时，`RefCell<T>` 类型非常有用。

与 `Rc<T>` 类似，`RefCell<T>` 仅适用于单线程的场景，并会在咱们试图于多线程的上下文中使用他时给出编译时报错。我们将在第 16 章中讨论怎样在多线程的程序中获得 `RefCell<T>` 的功能。

下面是对选择 `Box<T>`、`Rc<T>` 或 `RefCell<T>` 的原因的回顾：

+ 所有者数量方面
    - `Rc<T>` 允许同一数据可以有多个所有者；
    - `Box<T>` 和 `RefCell<T>` 都只有单一的所有者。
+ 借用规则检查方面
    - `Box<T>` 允许不可变或可变的借用在编译时检查；
    - `Rc<T>` 只允许不可变借用在编译时检查；
    - `RefCell<T>` 允许不可变或可变的借用在运行时检查。
- 由于 `RefCell<T>` 允许可变借用在运行时检查，因此即使 `RefCell<T>` 是不可变的，咱们也可以改变 `RefCell<T>` 内部的值。

改变不可变值内部的值，即为 *内部可变性模式*。我们来看一种其下内部可变性有用的情形，并研究这是如何可行的。


## 使用内部可变性

借用规则的一个结果是，当咱们有个不可变值时，咱们就无法以可变方式借用他。例如，下面这段代码不会编译：

```rust
fn main() {
    let x = 5;
    let y = &mut x;
}
```

当咱们尝试编译这段代码时，将得到以下报错：

```console
$ cargo run
   Compiling borrowing v0.1.0 (/home/hector/rust-lang-zh_CN/projects/borrowing)
error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
 --> src/main.rs:3:13
  |
3 |     let y = &mut x;
  |             ^^^^^^ cannot borrow as mutable
  |
help: consider changing this to be mutable
  |
2 |     let mut x = 5;
  |         +++

For more information about this error, try `rustc --explain E0596`.
error: could not compile `borrowing` (bin "borrowing") due to 1 previous error
```


然而，在某些情况下，值在其方法中改变自身，却对其他代码表现为不可变，会非常有用。值的方法外部的代码将无法修改该值。使用 `RefCell<T>` 是获得具备内部可变性能力的一种方式，但 `RefCell<T>` 并未完全绕过借用规则：编译器中的借用检查器会放行这种内部可变性，而代之以在运行时检查借用规则。当咱们违反这些规则时，咱们将得到一次 `panic!` 而不是编译器报错。

咱们来通过一个实际示例，其中我们可以使用 `RefCell<T>` 来改变某个不可变值，看看为什么这很有用。


## 以模拟对象测试

在测试过程中，为了观察特定行为并断言其是否得以正确实现，程序员有时会使用一种类型代替另一类型。这种占位类型称为 *测试替身*。可以把他想象成电影制作中的特技替身，有一个人顶替演员，完成特别棘手的某场戏。当我们运行测试时，测试替身会代替其他类型。所谓 *模拟对象*，属于测试替身的具体类型，他会记录测试过程中发生的情况，以便咱们可以断言是否发生了正确的操作。

Rust 没有其他语言有的意义上的对象，并且 Rust 没有一些其他语言有的内建于标准库中的模拟对象功能。但是，咱们完全可以创建一个结构体，将发挥与模拟对象同样的作用。

以下是我们将测试的情景：我们将创建一个库，跟踪某个值与最大值的关系，并根据当前值与最大值的接近程度发送消息。例如，这个库可用于跟踪用户允许进行的 API 调用数量的配额。

我们的库将仅提供跟踪某个值接近最大值的程度，以及于何时发出什么消息的功能。使用我们的库的应用，需自行提供发送消息的机制：应用可以直接展示消息给用户、发送电子邮件、发送短信或执行其他操作。库无需了解这些细节。他所需的只是实现我们将提供的一个特质，名为 `Messenger`。以下清单 15-20 展示了该库的代码：

<a name="listing_15-20"></a>
文件名：`src/lib.rs`

```rust
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: 你已超出配额!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: 你已用完 90% 的配额！");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: 你已用完 75% 的配额！");
        }
    }
}
```

**清单 15-20**：跟踪某个值与最大值接近程度，并在值处于特定水平时告警的一个库

这段代码的一个重要部分在于，`Messenger` 特质有个名为 `send` 的方法，他会取对 `self` 的不可变引用，以及消息文本。这个特质是我们的模拟对象需要实现的接口，以便模拟对象可以像真实对象一样使用。另一个重要的部分是，我们希望测试 `LimitTracker` 上的 `set_value` 方法的行为。我们可以改变我们传入的 `value` 参数的内容，但 `set_value` 不会返回任何内容供我们进行断言。我们希望能够表达，当咱们以实现 `Messenger` 特质的项目与特定的 `max` 值创建一个 `LimitTracker` 时，消息发送器会在我们为 `value` 传递不同数字时，被告知要发送相应的消息。

> **译注**：the interface, 借鉴了 Java 语言的叫法，参见 [使用接口来拯救！](https://java.xfoss.com/Ch08_Interfaces_and_Abstract_Classes.html#%E4%BD%BF%E7%94%A8%E6%8E%A5%E5%8F%A3%E6%9D%A5%E6%8B%AF%E6%95%91)。而这种内部可变性模式用到的数据结构，则类似于 Java 中的内部类。

我们需要这样一个模拟对象，当我们调用 `send` 时，他不会发送电子邮件或短信，而只将跟踪他被告知要发送的消息。我们可以

- 创建一个这种模拟对象的新实例、
- 创建一个使用该模拟对象的 `LimitTracker` 实例、
- 调用 `LimitTracker` 上的 `set_value` 方法，
- 然后检查该模拟对象是否具有我们期望的消息。


下面清单 15-21 展示了一种实现一个模拟对象来做到这点的尝试，但借用检查器不允许这样做。

<a name="listing_15-21"></a>
文件名：`src/lib.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        sent_messages: Vec<String>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: vec![],
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.len(), 1);
    }
}
```

**清单 15-21**：尝试实现借用检查器不允许的 `MockMessenger`

这段测试代码定义了一个 `MockMessenger` 结构体，有着一个 `sent_messages` 字段，该字段带有一个 `String` 值的 `Vec` 值，用于跟踪其被告知要发送的消息。我们还定义了个关联函数 `new`，以便于创建以空消息列表开头的新 `MockMessenger` 值。然后，我们为 `MockMessenger` 实现 `Messenger` 特质，以便我们可以提供 `MockMessenger` 给 `LimitTracker`。在 `send` 方法的定义中，我们取作为参数传入的消息，并存储在 `MockMessenger` 的 `sent_messages` 列表中。

在测试中，我们正在测试当 `LimitTracker` 被告知设置 `value` 为大于最大值的 75% 的某个值时会发生什么。首先，我们创建一个新的 `MockMessenger`，他将以一个空的消息列表开始。然后，我们创建一个新的 `LimitTracker` 并给予他一个对新的 `MockMessenger` 的引用和 `100` 的 `max` 值。我们以值 `80`，其大于 75%，调用 `LimitTracker` 上 `set_value` 方法。然后，我们断言 `MockMessenger` 跟踪的消息列表中现在应该有一条消息。

但是，如下所示，这个测试存在一个问题：

```console
$ cargo test
   Compiling limit-tracker v0.1.0 (/home/hector/rust-lang-zh_CN/projects/limit-tracker)
error[E0596]: cannot borrow `self.sent_messages` as mutable, as it is behind a `&` reference
  --> src/lib.rs:59:13
   |
59 |             self.sent_messages.push(String::from(message));
   |             ^^^^^^^^^^^^^^^^^^ `self` is a `&` reference, so it cannot be borrowed as mutable
   |
help: consider changing this to be a mutable reference in the `impl` method and the `trait` definition
   |
 2 ~     fn send(&mut self, msg: &str);
 3 | }
...
57 |     impl Messenger for MockMessenger {
58 ~         fn send(&mut self, message: &str) {
   |

For more information about this error, try `rustc --explain E0596`.
error: could not compile `limit-tracker` (lib test) due to 1 previous error
```

我们无法修改 `MockMessenger` 来记录消息，因为 `send` 方法取对 `self` 的不可变引用。我们也无法采纳错误文本中的建议，在 `impl` 方法和特质定义中都使用 `&mut self`。我们不希望仅仅为了测试而修改 `Messenger` 特质。我们需要找到一种方法，让测试代码在现有设计下正常运行。

这正是内部可变性可以发挥作用的情形！我们将存储 `sent_messages` 于 `RefCell<T>` 中，然后 `send` 方法就能够修改 `sent_messages` 以存储我们看到的消息。下面清单 15-22 展示了他的样子：

<a name="listing_15-22"></a>
文件名：`src/lib.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        // -- 跳过代码 --

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
```

**清单 15-22**：使用 `RefCell<T>` 在外部值被视为不可变的同时修改内层值

`sent_messages` 字段现在的类型为 `RefCell<Vec<String>`，而不是 `Vec<String>`。在 `new` 函数中，我们围绕空矢量值创建了个新的 `RefCell<Vec<String>` 实例。

对于 `send` 方法的实现，第一个参数仍然是 `self` 的不可变借用，这与特质的定义一致。我们对 `self.sent_messages` 中的 `RefCell<Vec<String>` 调用了 `borrow_mut`，以获取对 `RefCell<Vec<String>` 内部的值，即那个矢量值的可变引用。然后，我们就可以调用到该矢量值的可变引用上的 `push`，来记录测试过程中发送的消息。

我们必须要做的最后一个更改是在断言中：为了查看内层矢量值中有多少个项目，我们对 `RefCell<Vec<String>>` 调用 `borrow`，以获取对该矢量值的不可变引用。

现在咱们已经了解了怎样使用 `RefCell<T>`，我们来深入了解他的工作原理！


## 在运行时跟踪借用

在创建不可变与可变的引用时，我们分别使用 `&` 和 `&mut` 语法。而对于 `RefCell<T>`，我们使用 `borrow` 和 `borrow_mut` 方法，他们是属于 `RefCell<T>` 的安全 API 的一部分。

- `borrow` 方法返回灵巧指针类型 `Ref<T>`，
- 而 `borrow_mut` 返回灵巧指针类型 `RefMut<T>`。

这两种类型都实现了 `Deref`，因此我们可以像对待普通引用一样对待他们。

`RefCell<T>` 会跟踪当前有多少个活动的 `Ref<T>` 和 `RefMut<T>` 灵巧指针。每次我们调用 `borrow` 时，`RefCell<T>` 都会增加其活动的不可变借用的计数。当某个 `Ref<T>` 值超出作用域时，不可变借用计数会减少 1。就像编译时的借用规则一样，`RefCell<T>` 允许我们在任何时候都可以有多个不可变借用，或者一个可变借用。

当我们试图违反这些规则时，与我们在引用下将得到编译器报错不同，`RefCell<T>` 的实现将在运行时终止运行。下面清单 15-23 展示了清单 15-22 中的 `send` 实现的一种修改。我们故意尝试为同一作用域创建两个可变借用，以说明 `RefCell<T>` 会阻止咱们在运行时这样做。

<a name="listing_15-23"></a>
文件名：`src/lib.rs`

```rust
    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            let mut borrow_one = self.sent_messages.borrow_mut();
            let mut borrow_two = self.sent_messages.borrow_mut();

            one_borrow.push(String::from(message));
            two_borrow.push(String::from(message));
        }
    }
```

**清单 15-23**：在同一作用域中创建两个可变引用，以验证 `RefCell<T>` 将终止运行

我们为从 `borrow_mut` 返回的 `RefMut<T>` 灵巧指针创建了个变量 `one_borrow`。然后，我们以同样的方式在变量 `two_borrow` 中创建了另一个可变借用。这会在同一作用域中构造两个可变引用，而这是不被允许的。当我们运行库的测试时，清单 15-23 中的代码将不带任何报错地编译，但测试将失败：

```console
$ cargo test
   Compiling limit-tracker v0.1.0 (/home/hector/rust-lang-zh_CN/projects/limit-tracker)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running unittests src/lib.rs (target/debug/deps/limit_tracker-97fd84dd2716b118)

running 1 test
test tests::it_sends_an_over_75_percent_warning_message ... FAILED

failures:

---- tests::it_sends_an_over_75_percent_warning_message stdout ----

thread 'tests::it_sends_an_over_75_percent_warning_message' (678390) panicked at src/lib.rs:61:53:
RefCell already borrowed
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::it_sends_an_over_75_percent_warning_message

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`
```

请注意，该代码以消息 `RefCell already borrowed` 终止运行。这正是 `RefCell<T>` 处理运行时违反借用规则的方式。

像我们在这里这样，选择在运行时而非编译时捕获借用错误，意味着咱们可能会在开发过程后期，才发现代码中的错误：甚至可能直到代码部署到生产环境中才会发现。此外，由于在运行时而不是编译时跟踪借用，咱们的代码会因此出现较小的运行时性能损失。但是，使用 `RefCell<T>` 使编写模拟对象可行，其可以修改本身以跟踪他看到的消息，而咱们是在只允许不可变值的上下文中使用他。尽管需要权衡，咱们仍可使用 `RefCell<T>` 来获得比普通引用提供的更多功能。


## 允许可变数据的多个所有者

使用 `RefCell<T>` 的一种常见方式，是与 `Rc<T>` 结合使用。回顾一下，`Rc<T>` 允许咱们有着某一数据的多个所有者，但他只提供对数据的不可变访问。当咱们有个包含 `RefCell<T>` 的 `Rc<T>` 时，就可以得到一个可以有多个所有者，*并且* 咱们可以改变的值！

例如，回顾清单 15-18 中的构造列表示例，其中我们使用了 `Rc<T>` 来允许多个列表共用另一列表的所有权。由于 `Rc<T>` 仅包含不可变值，因此一旦创建了列表后，我们就无法更改列表中的任何值。我们来加入 `RefCell<T>`，以获得其修改列表中值的能力。下面清单 15-24 显示了通过在 `Cons` 定义中使用 `RefCell<T>`，我们可以修改存储在所有列表中的值：

<a name="listing_15-24"></a>
文件名：`src/main.rs`

```rust
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a 随后 = {a:?}");
    println!("b 随后 = {b:?}");
    println!("c 随后 = {c:?}");
}
```

**清单 15-24**：使用 `Rc<RefCell<i32>>` 创建我们可以改变的 `List`

我们创建一个值 `Rc<RefCell<i32>>` 的实例的值，并存储在名为 `value` 的变量中，以便稍后可以直接访问他。然后，我们以包含 `value` 的一个 `Cons` 变种创建一个 `a` 中的 `List`。我们需要克隆 `value`，以便 `a` 和 `value` 都拥有内部的值 `t` 的所有权，而不是从 `value` 转移所有权到 `a`，或者让 `a` 从 `value` 借用。

我们将列表 `a` 包装在 `Rc<T>` 中，这样当我们创建列表 `b` 和 `c` 时，他们都可以引用 `a`，这就是我们在 [清单 15-18](./rc-t.md#listing_15-18) 中所做的。

在创建了 `a`、`b` 和 `c` 中的列表后，我们打算加 10 到 `value` 中的值。我们通过对 `value` 调用 `borrow_mut` 来做到这点，他使用我们在第 5 章中 [`->` 操作符在哪里？](../structs/method_syntax.md#auto_deref) 处讨论的自动解引用特性，解引用 `Rc<T>` 为内层的 `RefCell<T>` 值。`borrow_mut` 方法返回一个 `RefMut<T>` 灵巧指针，我们对其使用解引用运算符并修改内层值。

当我们打印 `a`、`b` 与 `c` 时，可以看到他们都有了修改后的值 `15` 而不是 `5`：

```console
$ cargo run
   Compiling cons-list v0.1.0 (/home/hector/rust-lang-zh_CN/projects/cons-list)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/cons-list`
a 随后 = Cons(RefCell { value: 15 }, Nil)
b 随后 = Cons(RefCell { value: 3 }, Cons(RefCell { value: 15 }, Nil))
c 随后 = Cons(RefCell { value: 4 }, Cons(RefCell { value: 15 }, Nil))
```

这种技巧非常巧妙! 通过使用 `RefCell<T>`，我们有了个表面上不可变的 `List` 值。但我们可以使用 `RefCell<T>` 上，提供对其内部可变性的访问的方法，以便可以在需要时修改数据。运行时对借用规则的检查，可以保护我们免受数据竞争的影响，有时以牺牲一点速度，换取这种数据结构方面的灵活性是值得的。请注意，`RefCell<T>` 不适用于多线程代码! `Mutex<T>` 是 `RefCell<T>` 的线程安全版本，我们将在第 16 章中讨论 `Mutex<T>`。


（End）


