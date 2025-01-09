# 引用与借用

**References and Borrowing**


清单 4-5 中，元组代码的问题在于，我们必须将那个 `String` 返回给调用函数，以便咱们在调用 `calculate_length` 后，仍能使用这个 `String`，因为这个 `String` 已被迁移到 `calculate_length` 中。相反，我们可以提供一个到该 `String` 值的引用。*引用，reference* 与指针类似，其是个我们可以沿着他，访问存储在其中数据的地址；其中的数据为其他变量所有。与指针不同的是，可以保证某个引用在其生命周期内，始终指向某个特定类型的有效值。

下面是如何定义和使用，以对象引用作为参数，而非取得值所有权的 `calculate_length` 函数：


文件名：`src/main.rs`

```rust
fn main() {
    let s1 = String::from("hello");

    let length = calculate_length(&s1);

    println! ("字符串 '{}' 的长度为：{}", s1, length);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

首先，请注意变量声明和函数返回值中的所有元组代码都不见了。其次，请注意我们将 `&s1` 传入到 `calculate_length`，且在其定义中，我们使用了 `&String` 而不是 `String`。这些 `&` 符合表示了 *引用*，而他们允许咱们，在取得某个值所有权的情况下，对其进行引用。下图 4-5 描述了这一概念。


![指向 `String s1` 的 `&String s` 图示](images/Ch04_05.svg)

*图 4-5：指向 `String s1` 的 `&String s` 图示*


> **注意**：与使用 `&` 进行引用相反的是，*解引用，dereferencing*，他是通过解引用操作符 `*` 实现的。我们将在第 8 章，看到解引用操作符的一些用法，并在第 15 章讨论解引用的细节。


我们来仔细看看这里的函数调用：


```rust
    let s1 = String::from("hello");

    let len = calculate_length(&s1);
```


通过 `&s1` 这种语法，我们可以创建出一个指向 `s1` 值，但不拥有他的引用。由于这个引用不拥有 `s1`，因此其停止使用时，他所指向的值，不会被丢弃。

同样，那个函数的签名，使用了 `&` 来表明参数 `s` 的类型是个引用。我们来添加一些解释性注释：


```rust
fn calculate_length(s: &String) -> usize {  // s 是个到某 String 的引用
    s.len()
}   // 这里，s 会超出作用域。但由于他没有其指向值的所有权，因此该
    // 值不会被丢弃。
```


变量 `s` 有效的作用域，与任何的函数参数作用域相同，但当 `s` 停止使用时，该引用所指向的值并不会丢弃，因为 `s` 没有所有权。当函数将引用而非实际值作为参数时，我们就不再需要返回值，来归还所有权，因为我们从未拥有过所有权。

我们把这种创建出某个引用的行为，称为 *借用，borrowing*。这如同现实生活中，如果某人拥有某样东西，咱们可以向其借用这件东西。用毕时，咱们必须归还。咱们并不拥有这件东西。

那么，如果我们试图修改借来的东西，会发生什么情况呢？请尝试下面清单 4-6 中的代码。剧透一下：这不会起作用！


文件名：`src/main.rs`

```rust
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world!");
}
```

*清单 4-6：尝试修改某个借用值*


下面就是那个报错：


```console
$ cargo run
   Compiling ownership_demo v0.1.0 (C:\tools\msys64\home\Lenny.Peng\rust-lang-zh_CN\projects\ownership_demo)
error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
 --> src\main.rs:8:5
  |
8 |     some_string.push_str(", world!");
  |     ^^^^^^^^^^^ `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable
  |
help: consider changing this to be a mutable reference
  |
7 | fn change(some_string: &mut String) {
  |                         +++

For more information about this error, try `rustc --explain E0596`.
error: could not compile `ownership_demo` (bin "ownership_demo") due to previous error
```


正如变量默认是不可变的一样，引用也是如此。我们不能修改我们引用的东西。


## 可变引用

**Mutable References**


只需稍作调整，使用 *可变引用，mutable reference*，咱们即可修改某个借用值，便可修正清单 4-6 中的代码：


文件名：`src/main.rs`

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}
```


首先，我们将 `s` 改为了 `mut`。然后，我们在调用 `change` 函数处，用 `&mut s` 创建了一个可变引用，并更新了函数签名，以 `some_string：&mut String` 来接受一个可变引用。这就清楚地表明，`change` 函数将改变其所借用的值。

可变引用有个很大的限制：如果咱们有了到某个值的一个可变引用，就不能对该值有其他引用。下面这段试图创建两个到 `s` 可变引用的代码，就会失败：


文件名：`src/main.rs`

```rust
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println! ("{}, {}", r1, r2);
```


下面是报错信息：


```console
$ cargo run
   Compiling ownership_demo v0.1.0 (C:\tools\msys64\home\Lenny.Peng\rust-lang-zh_CN\projects\ownership_demo)
error[E0499]: cannot borrow `s` as mutable more than once at a time
 --> src\main.rs:5:14
  |
4 |     let r1 = &mut s;
  |              ------ first mutable borrow occurs here
5 |     let r2 = &mut s;
  |              ^^^^^^ second mutable borrow occurs here
6 |
7 |     println! ("{}, {}", r1, r2);
  |                         -- first borrow later used here

For more information about this error, try `rustc --explain E0499`.
error: could not compile `ownership_demo` (bin "ownership_demo") due to previous error
```

这个报错讲到，因为我们不能将 `s` 作为可变引用，同一实际借用多次，因此这段代码无效。第一次可变借用是在 `r1` 中，而必须持续到其在 `println!` 中被使用为止，但在这个可变引用的创建和使用之间，我们试图在 `r2` 中，创建另一个借用了与 `r1` 同一数据的可变引用。

防止在同一时间，对同一数据进行多个可变引用的这种限制，允许改变，但改变是在非常受控的方式下进行的。这也是 Rust 新手比较头疼的问题，因为大多数语言，都允许咱们随时改变。有着这种限制的好处是，Rust 可以在编译时，防止数据竞赛。*数据竞赛，data race* 类似于某种竞赛情形，会在下面这三种行为发生时出现：


- 两个以上的指针同时访问某同一数据；

- 至少有一个指针被用来写该数据；

- 没有同步访问该数据的机制。

数据竞赛会导致未定义的行为，当咱们试图在运行时跟踪他们时，会很难诊断和修复；Rust 通过拒绝编译带有数据竞赛的代码，从而避免了这个问题！

与往常一样，我们可以使用花括号，创建一个新的作用域，从而允许多个可变引用，只要不是 *同时* 的多个：


```rust
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    }   // r1 在这里超出了作用域，因此我们可以毫无问题地构造一个新的引用。

    let r2 = &mut s;
```


Rust 对组合可变引用和不可变引用，也执行类似的规则。下面这段代码会导致一个报错：


```rust
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s;

    println! ("{}, {} 与 {}", r1, r2, r3);
```


下面是那个报错：

```console
$ cargo run
   Compiling ownership_demo v0.1.0 (C:\tools\msys64\home\Lenny.Peng\rust-lang-zh_CN\projects\ownership_demo)
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> src\main.rs:6:14
  |
4 |     let r1 = &s;
  |              -- immutable borrow occurs here
5 |     let r2 = &s;
6 |     let r3 = &mut s;
  |              ^^^^^^ mutable borrow occurs here
7 |
8 |     println! ("{}, {}, {}", r1, r2, r3);
  |                             -- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `ownership_demo` (bin "ownership_demo") due to previous error
```


呼！我们 *也* 不能在有着到某个值的不可变引用的同时，有着一个到这同一值的可变引用。

某个不可变引用的使用者，不期望该值会突然在他们眼皮底下改变！不过，多个不可变引用是允许的，因为仅在读取数据的使用者，并无影响其他读取该数据使用者的能力。

请注意，引用的作用域从该引用被引入的地方开始，并持续到该引用被最后一次使用处为止。例如，下面这段代码可以编译，因为那个不可变引用的最后一次使用（`println!`），发生在那个可变引用被引入之前：


```rust
    let mut s = String::from("hello");

    let r1 = &s;    // 没有问题
    let r2 = &s;    // 没有问题
    println! ("r1 与 r2: {}, {}", r1, r2);
    // 变量 r1 与 r2 在此处之后将不会使用了

    let r3 = &mut s;    // 这就没问题了
    println! ("r3: {}", r3);
```


不可变引用 `r1` 和 `r2` 的作用域，在他们最后一次被使用的 `println!` 之后，可变引用 `r3` 被创建之前结束。这些作用域不会重叠，因此这段代码会被放行：编译器可以区分出，在作用域结束前的某个点，该引用不再被使用。


> **译注**：由于引用属于大小已知、固定不变的类型，因此他们是保存在栈上的，带有 `Copy` 特质，故上面的代码中，在 `let r3 = &mut s;` 语句之前，可以无限次使用 `r1` 和 `r2` 这两个不可变引用，他们不会被迁移到 `println!` 宏及其他函数中。


尽管借用方面的报错有时会令人沮丧，但请记住，这是 Rust 编译器在早期（编译时而不是运行时）就指出某个潜在错误，并准确地告诉咱们问题所在。这样，咱们就不必再追踪，为什么咱们的数据和咱们设想的不一样了。


## 悬空引用

**Dangling References**


在带有指针的语言中，就很容易错误地创建出 *悬空指针，dangling pointer*，即在保留了指向某处内存指针的同时，释放了该处内存，从而造成引用了内存中，可能已经给了其他代码的某个位置的指针。相比之下，在 Rust 中，编译器会保证引用，永远不会成为悬空引用：如果咱们有个到某数据的引用，编译器会确保该数据，不会在指向该数据引用超出作用域之前，超出作用域。

我们来尝试创建一个悬挂引用，看看 Rust 如何通过编译时报错，来防止他们：


文件名：`src/main.rs`

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```

下面是那个报错：


```console
$ cargo run
   Compiling ownership_demo v0.1.0 (C:\tools\msys64\home\Lenny.Peng\rust-lang-zh_CN\projects\ownership_demo)
error[E0106]: missing lifetime specifier
 --> src\main.rs:5:16
  |
5 | fn dangle() -> &String {
  |                ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
  |
5 | fn dangle() -> &'static String {
  |                 +++++++

For more information about this error, try `rustc --explain E0106`.
error: could not compile `ownership_demo` (bin "ownership_demo") due to previous error
```

这条报错信息，涉及我们尚未讲到的一项特性：生命周期。我们将在第 10 章，详细讨论生命周期。但是，如果咱们不考虑生命周期的部分，这条消息确实包含了，为什么这段代码会出现问题的关键所在：


```console
this function's return type contains a borrowed value, but there is no value
for it to be borrowed from
```


我们来仔细看看，咱们 `dangle` 代码的每个阶段，到底发生了什么：


文件名：`src/main.rs`

```rust
fn dangle() -> &String {    // dangle 返回的是个到某 String 的引用
    let s = String::from("hello");  // s 是个新的 String

    &s  // 咱们返回了一个指向那个 String，s 的引用
}   // 这里，s 超出了作用域，进而被丢弃。他的内存就没了。
    // 危险所在！
```


因为 `s` 是在 `dangle` 内部创建的，所以当 `dangle` 的代码结束时，`s` 将被解除内存分配。然而我们曾试图返回对他的引用。这意味着这个引用，将指向一个无效的 `String`。这可不行！Rust 不允许我们这么做。


这里的解决办法，是直接返回那个 `String` 值：

```rust
fn dangle() -> String {
    let s = String::from("hello");

    s
}
```

这会没有任何问题地运作。所有权会被迁出，而不会有任何东西，被解除内存分配。


## 引用的规则

**The Rules of References**


我们来回顾一下，我们已讨论过的关于引用的内容：


- 在任何时候，咱们都可以有着一个可变引用，或任意数量的不可变引用；

- 引用必须始终有效。


接下来，我们来看看另一种引用：切片。


（End）


