# `RefCell<T>` 与内部可变性模式

**`RefCell<T>` and the Interior Mutability Pattern**

*内部可变性，interior mutability* 属于 Rust 中的一种设计模式，他实现了即使在有着到数据的一些不可变引用之下，对数据加以改变；一般情况下，这样的行为是借用规则所不允许的。为了改变数据，这种模式便运用了数据结构内部的一些 `unsafe` 代码，来改变了 Rust 监管可变性与借用的一些一般规则。这些不安全代码向编译器表明，咱们自己在手动检查那些规则，而非依赖于编译器为咱们检查那些规则；在第 19 章将进一步讨论这些不安全代码。

只有当我们可以确保在运行时遵循借用规则时，我们才能使用使用内部可变性模式的类型，即使编译器不能保证这一点。然后将涉及的不安全代码包装在安全的 API 中，并且外部类型仍然是不可变的。

咱们来通过检视遵循内部可变性模式的 `RefCell<T>` 类型来探讨这个概念。


## 使用 `RefCell<T>` 在运行时执行借用规则检查

**Enforcing Borrowing Rules at Runtime with `RefCell<T>`**


与 `Rc<T>` 不同，`RefCell<T>` 类型表示对其所持有的数据的单一所有权。那么，是什么使 `RefCell<T>` 与 `Box<T>` 这样的类型不同呢？回顾咱们在第四章学到的借用规则：

- 在任何给定时间，咱们都可以有着 *要么* （而非同时） 一个可变引用，要么任意数量的不可变引用；
- 引用必须始终有效。


对于引用与 `Box<T>`，借用规则的不变性，the borrowing rules' invariants, 是在编译时强制执行的。对于 `RefCell<T>`，这些不变性则是在运行时强制执行的。对于引用，如果咱们破坏了这些规则，咱们会得到编译器报错。而在 `RefCell<T>` 中，如果咱们破坏了这些规则，咱们的程序将终止运行。

在编译时检查借用规则的好处是在开发过程中会更早地发现错误，而且对运行时性能没有影响，因为所有分析都是事先完成的。由于这些原因，在大多数情况下，在编译时检查借用规则是最好的选择，这就是为什么这是 Rust 的默认设置。

相反，在运行时检查借用规则的优点是允许某些内存安全的场景，而编译时检查则不会允许这些场景。与 Rust 编译器一样，静态分析，static analysis，本质上是保守的。代码的某些属性无法通过分析代码来检测：最著名的例子是停机问题，the Halting Problem, 它超出了本书的范围，但却是一个值得研究的有趣主题。

由于某些分析是不可行的，那么如果 Rust 编译器不能确定代码符合所有权规则，他可能会拒绝某个正确的程序；从这方面讲，他是保守的。如果 Rust 编译器接受了错误的程序，用户就无法相信 Rust 做出的保证。然而，如果 Rust 拒绝了某个正确的程序，编程者会感到不便，但又不会发生什么灾难性的事情。在咱们确定咱们的代码遵循借用规则，而编译器无法理解和保证时，`RefCell<T>` 类型就很有用。

与 `Rc<T>` 类似，`RefCell<T>` 只适用于单线程场景，如果咱们试图在多线程环境下使用它，会出现编译时错误。我们将在第 16 章讨论如何在多线程程序中获得 `RefCell<T>` 的功能。

下面是对选择 `Box<T>`、`Rc<T>` 或 `RefCell<T>` 理由的总结：

- `Rc<T>` 使同一数据有多个所有者；`Box<T>` 和 `RefCell<T>` 有单一所有者;
- `Box<T>` 允许在编译时检查不可变或可变的借用；`Rc<T>` 只允许在编译时检查不可变的借用；`RefCell<T>` 允许在运行时检查不可变或可变的借用；
- 因为 `RefCell<T>` 允许在运行时检查可变的借用，所以即使 `RefCell<T>` 是不可变的，咱们也可以改变 `RefCell<T>` 中的值。

改变不可变值内部的值，就是 *内部可变性模式，the interior mutablity pattern*。让我们看一下内部可变性有用的一种情况，并检视其如何可行。


## 内部可变性：对不可变值的可变借用

**Interior Mutability: A Mutable Borrow to an Immutable Value**


借用规则的一种后果是，当咱们有一个不可变的值时，咱们不能以可变方式借用他。比如，下面这段代码就不能编译：

```rust
fn main() {
    let x = 5;
    let y = &mut x;
}
```

如果咱们试图编译这段代码，咱们会得到以下错误：

```console
$ cargo run
   Compiling sp_demos v0.1.0 (/home/lennyp/rust-lang/sp_demos)
error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
 --> src/main.rs:3:13
  |
2 |     let x = 5;
  |         - help: consider changing this to be mutable: `mut x`
3 |     let y = &mut x;
  |             ^^^^^^ cannot borrow as mutable

For more information about this error, try `rustc --explain E0596`.
error: could not compile `sp_demos` due to previous error;
```
然而，在有些情况下，值在其方法中改变自身，但对其他代码来说却显得不可改变，这将是非常有用的。在该值的方法之外的代码将不能改变该值。使用 `RefCell<T>` 是获得内部可变性能力的一种方法，但是 `RefCell<T>` 并没有完全绕过借用规则：编译器中的借用检查器会放行这种内部可变性，而代之以在运行时借用规则得以检查。如果咱们违反了这些规则，咱们会得到一个 `pani!` 而不是一个编译器报错。

咱们来通过一个其中咱们可以使用 `RefCell<T>` 改变一个不可变的值的实际示例，看看为什么这很有用。


## 内部可变性的用例：模拟对象

**A Use Case for Interior Mutability: Mock Objects**


有时在测试过程中，程序员会使用一个类型来代替另一类型，以便观察特定的行为并断定其实现是正确的。这种占位符类型被称为 *测试替身，test double*。请从电影制作中的“特技替身，stunt double”的角度来考虑他，某人代替一名演员来完成特别棘手的一场戏。当咱们运行测试时，测试替身代表其他类型。*模拟对象，mock objects* 是特定类型的测试替身，他记录了测试过程中发生的事情，因此咱们可以断言发生了正确的动作。

Rust 没有像其他语言那样拥有对象，Rust 也没有像其他一些语言那样在标准库中内置模拟对象功能。但是，咱们绝对可以创建一个与模拟对象具有相同用途的结构。

下面是我们要测试的情景：我们将创建一个库，跟踪某个数值与最大值的关系，并根据当前数值与最大值的接近程度发送消息。例如，这个库可以用来跟踪用户允许调用的 API 数量配额。

这个库将提供跟踪某个值接近最大值的程度，及在什么时刻发出什么消息的功能。使用这个库的应用，将被期望提供发送消息的机制：应用可以在应用中放置消息、发送电子邮件、发出手机短信或其他东西。库不需要知道这个细节。他所需的只是实现了咱们将提供的名为 `Messenger` 特质的东西。以下清单 15-20 给出库的代码：


文件名：`src/lib.rs`

```rust
{{#include ../../projects/limit_tracker/src/lib.rs::35}}
```

*清单 15-20：跟踪某个值与最大值接近程度，并在值处于不同水平时发出告警的库*

这段代码的一个重要部分是 `Messenger` 特质有个叫做 `send` 的方法，其接收一个不可变 `self` 的引用和消息文本。这个特质是咱们模拟对象需要实现的接口，这样模拟对象就可以和真实对象一样被使用。另一个重要的部分是，我们要测试 `LimitTracker` 上 `set_value` 方法的行为。我们可以改变我们传入的 `value` 参数，但 `set_value` 并没有返回任何东西让我们做断言。我们希望能够表达出，若咱们用实现了 `Messenger` 特质的东西，与 `max` 的一个特定值创建了一个 `LimitTracker`，当我们为 `value` 传递不同的数字时，`messenger` 就会被告知要发送相应的消息。

> 注：the interface, 借鉴了 Java 语言的叫法，参见 [使用接口来拯救！](https://java.xfoss.com/ji-cheng-he-duo-tai-ji-zhi/ch08_interfaces_and_abstract_classes#interface_rescue)。而这种内部可变性模式用到的数据结构，则类似于 Java 中的内部类。

我们需要一个模拟对象，他不会在我们调用 `send` 时发送电子邮件或文本消息，而只会记录他被告知要发送的消息。我们可以创建模拟对象的一个新实例，创建一个使用该模拟对象的 `LimitTracker` 实例，调用 `LimitTracker` 实例的 `set_value` 方法，然后检查该模拟对象是否有我们期望的消息。清单 15-21 给出了一个实现模拟对象的尝试，来就这样做，但借用检查器不允许这样做：

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

*清单 15-21：试图实现一个借用检查器不允许的 `MockMessenger`*

这段测试代码定义了一个 `MockMessenger` 结构体，他有一个 `send_messages` 字段，里面有一个 `String` 值的 `Vec`，用来记录他被告知要发送的消息。我们还定义了一个关联函数 `new`，以方便创建新的 `MockMessenger` 值，该值以一个空的消息列表开始。然后我们为 `MockMessenger` 实现了 `Messenger` 特质，这样我们就可以给 `LimitTracker` 一个 `MockMessenger`。在 `send` 方法的定义中，我们将传入的消息作为参数，并将其存储在 `MockMessenger` 的 `send_messages` 列表中。

在测试中，我们正在测试当 `LimitTracker` 被告知将 `value` 设置为超过最大值的 75% 时会发生什么。首先，我们创建一个新的 `MockMessenger`，他将以一个空的消息列表开始。然后我们创建一个新的 `LimitTracker` 并为其提供对新 `MockMessenger` 的引用和最大值 `100`。我们在 `LimitTracker` 上用一个大于 75% 的值 `80` 调用 `set_value` 方法。然后我们断言 `MockMessenger` 正在跟踪的消息列表现在应有一条消息。

但是，此测试存在一个问题，如下所示：

```console
$ cargo test
   Compiling limit_tracker v0.1.0 (/home/lennyp/rust-lang/limit_tracker)
error[E0596]: cannot borrow `self.sent_messages` as mutable, as it is behind a `&` reference
  --> src/lib.rs:58:13
   |
2  |     fn send(&self, msg: &str);
   |             ----- help: consider changing that to be a mutable reference: `&mut self`
...
58 |             self.sent_messages.push(String::from(message));
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `self` is a `&` reference, so the data it refers to cannot be borrowed as mutable

For more information about this error, try `rustc --explain E0596`.
error: could not compile `limit_tracker` due to previous error
warning: build failed, waiting for other jobs to finish...
```

我们不能修改 `MockMessenger` 来记录消息，因为 `send` 方法需要一个对 `self` 的不可变的引用。我们也不能采纳错误文本中的建议，使用 `&mut self` 来代替，因为那样的话，`send` 的签名就无法与 `Messenger` 特质定义中的签名相匹配（请随意尝试，看看咱们会得到什么样的报错消息）。

这种情况下，内部可变性可以起到帮助作用！我们将把 `send_messages` 存储在一个 `RefCell<T>` 中，然后 `send` 方法将能够修改 `send_messages` 来存储我们所看到的信息。清单 15-22 显示了这是什么样子：

文件名：`src/lib.rs`

```rust
{{#include ../../projects/limit_tracker/src/lib.rs:37:}}
```

*清单 15-22：使用 `RefCell<T>` 来改变内层值，而外部值被认为是不可变的*

`sent_messages` 字段现在的类型是 `RefCell<Vec<String>`，而不是 `Vec<String>`。在 `new` 函数中，我们围绕空向量创建一个新的 `RefCell<Vec<String>` 实例。

对于 `send` 方法的实现，第一个参数仍然是 `self` 的不可变借用，这与特质定义相匹配。我们对 `self.send_messages` 中的 `RefCell<Vec<String>` 调用 `borrow_mut`，以获得 `RefCell<Vec<String>` 中值的可变引用，也就是那个矢量。然后，我们可以对该矢量的可变引用调用 `push`，以记录测试期间发送的消息。

我们必须做的最后一个更改是在断言中：为了查看内层矢量中有多少个条目，我们在 `RefCell<Vec<String>>` 上调用 `borrow` 以获得对该矢量的不可变引用。

现在咱们已经看到了如何使用 `RefCell<T>`，咱们来深入了解其工作原理！


## 使用 `RefCell<T>` 在运行时记录借用

**Keeping Track of Borrows at Runtime with `RefCell<T>`**


当创建不可变和可变引用时，我们分别使用 `&` 和 `&mut` 语法。而对于 `RefCell<T>`，我们使用 `borrow` 和 `borrow_mut` 方法，他们属于 `RefCell<T>` 安全 API 的一部分。`borrow` 方法返回灵巧指针类型 `Ref<T>`，而 `borrow_mut` 返回灵巧指针类型 `RefMut<T>`。这两种类型都实现了 `Deref`，所以我们可以像对待普通引用一样对待他们。

`RefCell<T>` 会记录当前有多少个 `Ref<T>` 和 `RefMut<T>` 灵巧指针是活动的。每次我们调用 `borrow`，`RefCell<T>` 都会增加他的计数，即有多少个不可变借用是活动的。当一个 `Ref<T>` 值超出作用域时，不可变借用的计数就会减少一个。就像编译时的借用规则一样，`RefCell<T>` 允许我们在任何时候有许多不可变借用或一个可变的借用。

在咱们尝试违反这些规则时，与在引用下咱们会得到编译器报错不同，`RefCell<T>` 的实现将在运行时终止运行。下面清单 15-23 给出了清单 15-22 中那个 `send` 实现的修改。咱们故意为同一作用域创建了两个可变借用，以演示 `RefCell<T>` 在运行时阻止咱们这样做。

文件名：`src/lib.rs`

```rust
    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            let mut borrow_one = self.sent_messages.borrow_mut();
            let mut borrow_two = self.sent_messages.borrow_mut();

            borrow_one.push(String::from(message));
            borrow_two.push(String::from(message));
        }
    }
```

*清单 15-23：在同一作用域中创建两个可变引用，以发现 `RefCell<T>` 会终止运行*

我们为从 `borrow_mut` 返回的 `RefMut<T>` 智能指针创建了一个变量 `one_borrow`。然后我们以同样的方式在变量 `two_borrow` 中创建了另一个可变的借用。这就在同一作用域中产生了两个可变引用，这是不允许的。当我们运行咱们库的测试时，清单 15-23 中的代码将被不带任何报错地编译，但测试将失败：

```console
$ cargo test
   Compiling limit_tracker v0.1.0 (/home/peng/rust-lang/limit_tracker)
    Finished test [unoptimized + debuginfo] target(s) in 0.46s
     Running unittests src/lib.rs (target/debug/deps/limit_tracker-98d6159d1b15eb72)

running 1 test
test tests::it_sends_an_over_75_percent_waring_message ... FAILED

failures:

---- tests::it_sends_an_over_75_percent_waring_message stdout ----
thread 'tests::it_sends_an_over_75_percent_waring_message' panicked at 'already borrowed: BorrowMutError', src/lib.rs:60:53
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::it_sends_an_over_75_percent_waring_message

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`
```

请注意该代码是以消息 `already borrowed: BorrowMutError` 终止运行的。这正是 `RefCell<T>` 处理运行时违反借用规则的方式。

选择在运行时而不是编译时捕获借用错误，正如我们在这里所做的那样，意味着咱们可能会在开发过程后期，才发现代码中的错误：可能直到咱们的代码部署到生产环境中才发现。此外，由于在运行时而不是编译时记录借用，咱们的代码会招致小的运行时性能损失。但是，使用 `RefCell<T>` 可以编写出模拟对象，该对象可以修改自身，来记录在咱们仅允许不可变值的上下文中使用他时，其所见到的消息。尽管 `RefCell<T>` 为获得比常规引用所提供的更多功能而有所取舍，咱们可以使用他。


## 通过结合 `Rc<T>` 与 `RefCell<T>`，实现可变数据的多个所有者

**Having Multiple Owners of Mutable Data by Combining `Rc<T>` and `RefCell<T>`**


使用 `RefCell<T>` 的一种常见方式是与 `Rc<T>` 结合使用。回顾一下，`Rc<T>` 实现了某个数据的多个所有者，但只提供对数据的不可变访问。如果咱们有一个持有 `RefCell<T>` 的 `Rc<T>`，咱们可以得到一个可以有着多个所有者，*且* 咱们可以改变的值。

比如，回顾清单 15-18 中的构造列表示例，咱们使用 `Rc<T>` 来实现多个列表共用另一列表所有权。由于 `Rc<T>` 只保存不可变值，因此一旦咱们创建出列表中的任何值，咱们就再也不能改变他们。咱们来加入 `RefCell<T>`，以获得修改列表中值的能力。下面清单 15-24 显示，通过在 `Cons` 定义中使用 `RefCell<T>`，咱们可以修改所有列表中存储的值：

文件名：`src/main.rs`

```rust
{{#include ../../projects/cons_list_demo/src/main.rs}}
```

*清单 15-24：使用 `Rc<RefCell<i32>>` 创建一个咱们可改变的 `List`*

我们创建了一个值，他是 `Rc<RefCell<i32>>` 的一个实例，并将其存储在一个名为 `value` 的变量中，以便我们稍后可以直接访问。然后我们以持有 `value` 的一个 `Cons` 变种，在 `a` 中创建了一个 `List`。我们需要克隆 `value`，以便 `a` 和 `value` 都拥有内部值 `5` 的所有权，而不是将所有权从 `value` 转移到 `a` 或让 `a` 从 `value` 借用。

我们将列表 `a` 包装在 `Rc<T>` 中，这样当我们创建列表 `b` 和 `c` 时，他们都可以引用 `a`，这就是我们在示例 15-18 中所做的。

在我们创建了 `a`、`b` 和 `c` 中的列表后，我们打算在 `value` 中的值上加 `10`。我们通过在 `value` 上调用 `borrow_mut` 来实现这一目的，他使用了我们在第 `5` 章中讨论过的自动解引用功能，the automatic dereferencing feature，（参见 [`->` 操作符去哪儿了？](Ch05_Using_Structs_to_Structure_Related_Data.md#--操作符the---operator哪去了呢) 小节），将 `Rc<T>` 解引用到内部的 `RefCell<T>` 值。`borrow_mut` 方法返回一个 `RefMut<T>` 灵巧指针，我们对其使用解引用操作符，并改变内部值。

在打印 `a`、`b` 与 `c` 时，就可以看到他们都有了修改后的值 `15` 而非 `5`：

```console
$ cargo run                                                       lennyp@vm-manjaro
   Compiling cons_list_demo v0.1.0 (/home/lennyp/rust-lang/cons_list_demo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/cons_list_demo`
之后的 a = Cons(RefCell { value: 15 }, Nil)
之后的 b = Cons(RefCell { value: 3 }, Cons(RefCell { value: 15 }, Nil))
之后的 c = Cons(RefCell { value: 4 }, Cons(RefCell { value: 15 }, Nil))
```

这个技巧非常整洁! 通过使用 `RefCell<T>`，我们有一个对外不可变的 `List` 值。但是我们可以使用 `RefCell<T>` 上提供对其内部可变性访问的方法，这样我们就可以在需要的时候修改我们的数据。借用规则的运行时检查可以保护我们不受数据竞赛的影响，有时值得用一点速度来换取我们数据结构中的这种灵活性。请注意，`RefCell<T>` 对多线程代码不起作用! `Mutex<T>` 是 `RefCell<T>` 的线程安全版本，我们将在第 16 章讨论 `Mutex<T>`。


（End）


