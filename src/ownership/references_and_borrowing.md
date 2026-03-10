# 引用与借用

[清单 4-5](./about_ownership.md#listing_4-5) 中元组代码下的问题在于，我们必须返回 `String` 给调用函数，以便咱们在调用 `calculate_length` 后仍能使用该 `String`，因为这个 `String` 已被迁移到 `calculate_length` 中。相反，我们可提供 `String` 值的引用。所谓 *引用，reference*，就是一个指针，因为他是个地址，我们可顺着这个地址访问存储在该地址处的数据；该数据由其他变量所有。与指针不同，引用保证在该引用的声明周期内，指向特定类型的有效值。

下面是咱们将如何定义和使用 `calculate_length` 函数的方式，该函数有着到对象的引用作为参数，而非取得值的所有权：

文件名：`src/main.rs`

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println! ("字符串 '{s1}' 的长度为：{len}。");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

首先，请注意变量声明和函数返回值中的所有元组代码都没有了。其次，要注意我们传递 `&s1` 到 `calculate_length` 中，而在其定义中，我们采用 `&String` 而不是 `String`。这两个 `&` 符合表示 *引用*，他们允许咱们指向某个值而无需取得他的所有权。下图 4-6 描述了这一概念。

<a name="f_4-6"></a>
![指向 `String s1` 的 `&String s` 图示](../images/Ch04_05.svg)

**图 4-6**：`&String s` 指向 `String s1` 的图示


> **注意**：使用 `&` 进行引用的反面，即 *解引用，dereferencing*，其以解引用运算符 `*` 完成。我们将在第 8 章看到解引用运算符的一些用法，并在第 15 章讨论解引用的细节。


我们来仔细看看这里的函数调用：


```rust
    let s1 = String::from("hello");

    let len = calculate_length(&s1);
```


`&s1` 这种语法允许我们创建一个引用，*指向* `s1` 的值但不拥有他。由于引用不拥有他，所以在引用停止使用后，其所指向的值将不被丢弃。

同样，函数的签名使用 `&` 表明参数 `s` 的类型是个引用。我们来添加一些解释性注解：


```rust
fn calculate_length(s: &String) -> usize {  // s 是个到 String 的引用
    s.len()
}   // 这里，s 会超出作用域。但由于 s 没有其指向内容的所有权，因此
    // 那个 String 不会被丢弃。
```

变量 `s` 有效的作用域与任何的函数参数作用域相同，但当 `s` 停止使用时，引用所指向的值不会被丢弃，因为 `s` 没有所有权。当函数将引用而非具体值作为参数时，我们将无需为归还所有权返回值，因为我们从未拥有所有权。

我们把创建引用的操作称为 *借用，borrowing*。就像现实生活中一样，当某人拥有某样东西，咱们可以向他们借用。在用完后，咱们必须归还。咱们不拥有他。

那么，若我们试图修改借用的东西会发生什么呢？请尝试下面清单 4-6 中的代码。剧透预警：他不会工作！


<a name="listing_4-6"></a>
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

**清单 4-6**：尝试修改借用的值


下面是报错：


```console
$ cargo run
   Compiling ref_demo v0.1.0 (/home/hector/rust-lang-zh_CN/projects/ref_demo)
error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
 --> src/main.rs:8:5
  |
8 |     some_string.push_str(", world!");
  |     ^^^^^^^^^^^ `some_string` is a `&` reference, so it cannot be borrowed as mutable
  |
help: consider changing this to be a mutable reference
  |
7 | fn change(some_string: &mut String) {
  |                         +++

For more information about this error, try `rustc --explain E0596`.
error: could not compile `ref_demo` (bin "ref_demo") due to 1 previous error
```


正如变量默认是不可变的一样，引用也是如此。我们不允许修改我们引用的内容。


## 可变引用

我们可以修复清单 4-6 中的代码，只需稍作调整，转而使用 *可变引用，mutable reference*，便会允许咱们修改借用值：


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


首先，我们修改 `s` 为 `mut`。然后，在调用 `change` 函数处，我们以 `&mut s` 创建一个可变引用，并以 `some_string：&mut String` 更新函数签名为接受一个可变引用。这使 `change` 函数将改变其借用的值非常清楚。

可变引用有个很大的限制：当咱们有一个到某个值的可变引用时，咱们就不能有对该值的其他引用。下面这段尝试创建两个到 `s` 的可变引用的代码将失败：


文件名：`src/main.rs`

```rust
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println! ("{}, {}", r1, r2);
```


下面是报错：


```console
$ cargo run
   Compiling ref_demo v0.1.0 (/home/hector/rust-lang-zh_CN/projects/ref_demo)
error[E0499]: cannot borrow `s` as mutable more than once at a time
 --> src/main.rs:5:14
  |
4 |     let r1 = &mut s;
  |              ------ first mutable borrow occurs here
5 |     let r2 = &mut s;
  |              ^^^^^^ second mutable borrow occurs here
6 |
7 |     println! ("{}, {}", r1, r2);
  |                         -- first borrow later used here

For more information about this error, try `rustc --explain E0499`.
error: could not compile `ref_demo` (bin "ref_demo") due to 1 previous error
```

这个报表明这段代码无效，因为我们同一时间不能多次借用 `s` 为可变。第一次可变借用在 `r1` 处，并必须持续到其在 `println!` 中使用为止，但在这一可变引用的创建和使用之间，我们试图在 `r2` 中创建另一个可变引用，借用与 `r1` 的同一数据。

阻止同时对同一数据的多重可变引用这一限制允许改变，但以非常受控的方式进行。这是 Rust 新手比较头疼的问题，因为大多数语言都允许咱们随时修改数据。有着这一限制的好处是 Rust 可以在编译时防止数据竞争。所谓 *数据竞争，data race*，类似于竞争条件，会在下面这三种行为发生时出现, is similar to a race condition：

- 两个以上的指针同时访问同一数据；
- 至少有一个指针正用于写该数据；
- 没有用于同步访问该数据的机制。

数据竞争会导致未定义的行为，当咱们试图在运行时追踪数据竞争时，可能难于诊断和修复；Rust 通过拒绝编译带有数据竞争的代码来防止这个问题！

与往常一样，我们可使用花括号创建新的作用域，允许多重可变引用，只要不是一些 *同时* 的多重引用即可：


```rust
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    }   // r1 在这里超出作用域，因此我们可以毫无问题地构造一个新的引用。

    let r2 = &mut s;
```


针对可变引用和不可变引用的组合，Rust 会执行类似规则。下面这段代码会导致一个报错：


```rust
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s;

    println! ("{r1}, {r2} 与 {r3}");
```


下面是报错：

```console
$ cargo run
   Compiling ref_demo v0.1.0 (/home/hector/rust-lang-zh_CN/projects/ref_demo)
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:14
  |
4 |     let r1 = &s;
  |              -- immutable borrow occurs here
5 |     let r2 = &s;
6 |     let r3 = &mut s;
  |              ^^^^^^ mutable borrow occurs here
7 |
8 |     println! ("{r1}, {r2} 与 {r3}");
  |                 -- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `ref_demo` (bin "ref_demo") due to 1 previous error
```


哎呀！当我们有着到同一值的不可变引用时，我们 *也* 不能有可变引用。

不可变引用的使用者不会期望值会突然在他们眼皮底下改变！不过，多重不可变引用是允许的，因为仅仅读取数据的使用者没有能力影响其他使用者对数据的读取。

请注意，引用的作用域从其被引入之处开始，持续到该引用最后一次被使用为止。例如，下面这段代码将编译，因为不可变引用最后使用是在 `println!` 中，在可变引用被引入前：


```rust
    let mut s = String::from("hello");

    let r1 = &s;    // 没有问题
    let r2 = &s;    // 没有问题
    println! ("{r1} 与 {r2}");
    // 变量 r1 与 r2 在此处之后将不会使用

    let r3 = &mut s;    // 没有问题
    println! ("{r3}");
```


不可变引用 `r1` 和 `r2` 的作用域在他们最后被使用处的 `println!` 后结束，这在可变引用 `r3` 被创建前。这些作用域不重叠，因此这段代码会被放行（允许）：编译器可以区分出引用在作用域结束前的某个点不再被使用。


> **译注**：由于引用属于大小已知、固定不变的类型，因此他们是保存在栈上的，带有 `Copy` 特质，故上面的代码中，在 `let r3 = &mut s;` 语句之前，可以无限次使用 `r1` 和 `r2` 这两个不可变引用，他们不会被迁移到 `println!` 宏及其他函数中。


尽管借用的报错有时会令人沮丧，但请记住，这是 Rust 编译器在早期（编译时而不是运行时）指出某个潜在 bug，并向咱们准确展示问题所在。然后，咱们不必排查为何咱们的数据不是咱们认为的那样。


## 悬空引用

在有着指针的语言中，通过释放一些内存同时保留指向该内存的指针，很容易错误地创建 *悬空指针，dangling pointer* -- 引用内存中可能已经给了另一指针/变量的某个位置的指针。相比之下，在 Rust 中，编译器保证引用将绝不是悬空引用：当咱们有着到某一数据的引用时，编译器会确保该数据不会在指向该数据的引用超出作用域前超出作用域。

我们来尝试创建一个悬挂引用，看看 Rust 如何通过编译时报错来防止他们：


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

下面是报错：


```console
$ cargo run
   Compiling ref_demo v0.1.0 (/home/hector/rust-lang-zh_CN/projects/ref_demo)
error[E0106]: missing lifetime specifier
 --> src/main.rs:5:16
  |
5 | fn dangle() -> &String {
  |                ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime, but this is uncommon unless you're returning a borrowed value from a `const` or a `static`
  |
5 | fn dangle() -> &'static String {
  |                 +++++++
help: instead, you are more likely to want to return an owned value
  |
5 - fn dangle() -> &String {
5 + fn dangle() -> String {
  |

For more information about this error, try `rustc --explain E0106`.
error: could not compile `ref_demo` (bin "ref_demo") due to 1 previous error
```

这一报错信息涉及我们尚未讲到的一项特性：生命周期。我们将在第 10 章中详细讨论生命周期。但是，若咱们不考虑生命周期的部分，这一消息确实包含了为何这段代码是个问题的关键：


```console
this function's return type contains a borrowed value, but there is no value
for it to be borrowed from
```


我们来仔细看看，咱们 `dangle` 代码的每个阶段到底发生了什么：


文件名：`src/main.rs`

```rust
fn dangle() -> &String {    // dangle 返回一个到 String 的引用
    let s = String::from("hello");  // s 是个新的 String

    &s  // 我们返回一个指向 String，s 的引用
}   // 这里，s 超出作用域而被丢弃。因此他的内存没了。
    // 危险！
```


因为 `s` 是在 `dangle` 内部创建的，所以在 `dangle` 的代码完毕后，`s` 将被解分配。但我们试图返回对他的引用。这意味着这个引用将指向无效 `String`。这可不行！Rust 将不允许我们这样做。


这里的解决方案是直接返回 `String` 值：

```rust
fn dangle() -> String {
    let s = String::from("hello");

    s
}
```

这会毫无问题地工作。所有权被迁出，而没有内容被解分配。


## 引用的规则

我们来回顾一下我们讨论过的关于引用的内容：


- 在任何给定时间，咱们都可以 *要么* 有着一个可变引用，*或者* 任意数量的不可变引用；
- 引用必须始终有效。


接下来，我们将看看另一种不同的引用：切片。


（End）


