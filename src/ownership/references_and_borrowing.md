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


变量 `s` 于其间有效的那个作用域，与所有函数参数作用域是相同的，而由于变量 `s` 不拥有经引用而指向的那个值的所有权，因此在变量 `s` 停止被使用时，那个所指向的值就不会被丢弃。在函数以引用变量，而非真实值作为参数时，由于根本就没有拥有过所有权，那么就不再需要为了交回所有权，而将那些值返回了。

咱们把这种创建出引用的行为，叫做 *借用，borrowing*。正如日常生活中，当某人拥有某个物件时，咱们就可以把这个物件从那个人那里借用一下。在使用完毕后，咱们必须将其还回。咱们是不拥有该物件的。

那么在尝试修改某个正借用的物件时，又会发生什么呢？请尝试下面清单 4-6 中的代码。提前剧透一下：那代码就不会工作！

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

*清单 4-6：尝试修改被借用值，a borrowed value*

下面就是编译器报错：

```console
$ cargo run
   Compiling ownership_demo v0.1.0 (/home/peng/rust-lang/projects/ownership_demo)
error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
 --> src/main.rs:8:5
  |
7 | fn change(some_string: &String) {
  |                        ------- help: consider changing this to be a mutable reference: `&mut String`
8 |     some_string.push_str(", world!");
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable

For more information about this error, try `rustc --explain E0596`.
error: could not compile `ownership_demo` due to previous error
```

就跟变量默认是不可变的一样，引用也是默认不可变的。不允许修改所引用的某个物件。


## 可变引用

**Mutable References**

使用 *可变引用，mutable reference*，来取代默认不可变引用，只需一些小小调整，就可将清单 4-6 的代码，修改为允许对借用值，a borrowed value 加以修改：

文件名：`src/main.rs`

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    println! ("s：{}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}
```

首先，这里将变量 `s` 改为了 `mut`。随后在调用 `change` 函数处，以 `&mut s` 创建了一个可变的引用变量，并以 `some_string: &mut String`，将那个函数签名，更新为接受一个可变引用变量（a mutable reference）。这样做就很清楚地表明了，那个 `change` 函数将修改他借用的那个值。

可变引用变量有个大的限制：在有着到某值的一个可变引用时，就不能有到那个值的其他引用了。下面尝试创建到变量 `s` 两个可变引用的代码，就会失败：

文件名：`src/main.rs`

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println! ("{}, {}", r1, r2);
}
```

下面是编译器报错：

```console
$ cargo run
   Compiling ownership_demo v0.1.0 (/home/peng/rust-lang/projects/ownership_demo)
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
error: could not compile `ownership_demo` due to previous error
```

此错误是说，由于在某个时间，多次将 `s` 借用做可变引用，而因此这段代码是无效的。首次可变借用是在 `r1` 中，而这次借用必须持续到其在那个 `println!` 中被使用为止，但就在那个可变引用的创建与使用中间，这里还尝试了在 `r2` 中，创建另一个借用了与 `r1` 同样数据的可变引用变量。

这种阻止在同一时间，到同一数据多重可变引用的限制，是允许修改的，但要在极度受控方式下进行（the restriction preventing multiple mutable references to the same data at the same time allows for mutation but in a very controlled fashion）。由于多数语言都允许随时修改数据，而因此多重可变引用正是一些新晋 Rust 公民们纠结不已的东西。有着这个限制的好处，则是 Rust 可以在编译时，对数据竞争加以阻止。与赛跑情形类似，*数据竞争，data race* 会在下面三种现象发生出现时出现：

- 同一时间有两个以上的指针访问着同一数据（two or more pointers access the same data at the same time）；
- 这些指针中至少有一个，正被用于写那个数据（at least one of the pointers is being used to write to the data）；
- 没有使用某种机制，来同步对数据的访问（there's no mechanism being used to synchronize access to the data）。

数据竞争导致未定义行为，并在尝试于运行时对其加以追踪的时候，难于排查诊断和修复；Rust 通过拒绝编译带有数据竞争的代码，而防止了这类问题！

与往常一样，可使用花括号来创建一个新的作用域，而实现多个可变应用变量，只要不是 *同时，simultaneous* 的几个就行：

```rust
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    }   // 由于在这里变量 r1 超出了作用域，因此就可以
        // 毫无问题地构造一个新的引用变量了。

    let r2 = &mut s;
```


对于将可变与不可变引用进行结合的情况，Rust 则会强制执行类似规则。下面的代码会导致错误：

```rust
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s;

    println! ("{}, {} 与 {}", r1, r2, r3);
```

下面就是那个错误：

```console
$ cargo run
   Compiling ownership_demo v0.1.0 (/home/peng/rust-lang/projects/ownership_demo)
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:14
  |
4 |     let r1 = &s;
  |              -- immutable borrow occurs here
5 |     let r2 = &s;
6 |     let r3 = &mut s;
  |              ^^^^^^ mutable borrow occurs here
7 |
8 |     println! ("{}, {} 与 {}", r1, r2, r3);
  |                               -- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `ownership_demo` due to previous error
```

咦！在有着对某个值的不可变引用时，*也是，also* 不可以对其有可变引用的。不可变引用的用户们，并不期望他们所引用的值，在他们眼皮底下突然就变掉！不过由于仅读取数据的不可变引用，对其他读取那个数据的引用，不具备造成影响的能力，因此多个不可变引用倒是可以的。

请注意引用变量的作用域，是从引入这个变量的地方开始，而持续到那个引用变量最后一次被使用为止。举例来说，由于那个不可变引用变量最后的使用，即那个 `println!`，是在那个可变引用变量引入之前发生的，因此下面的代码将会编译：

```rust
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println! ("r1 与 r2: {}, {}", r1, r2);
    // 变量 r1 与 r2 在此点位之后便不再被使用

    let r3 = &mut s;    // 这就没问题了
    println! ("r3: {}", r3);
```

不可变引用变量 `r1` 与 `r2` 的作用域，在 `println!` 语句，即他们最后被使用的地方之后便结束，而这个地方正是那个可变引用变量 `r3` 被创建之前。这些作用域不会重叠，因此这段代码是允许的。识别出引用变量在作用域结束之前的某处，不再被使用的编译器能力，叫做 *非词法性生命周期，Non-Lexical Lifetimes, 简写做 NLL*，在 [版本手册](https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/non-lexical-lifetimes.html) 里可阅读到更多有关内容。

虽然这些所有权借用方面的错误，时常令人沮丧，但请记住这正是 Rust 编译器，于早期阶段（在编译时而非运行时）就在指出潜在错误，并表明问题准确所在。代码编写者这才不必去追踪为何数据不是先前所设想的那样。


## 悬空引用

**Dangling References**



在有着指针的那些语言中，都容易通过在保留了到某些内存的一个指针同时，释放了那些内存，而错误地创建出 *悬空指针，a dangling pointer* -- 引用了内存中，可能已经给了其他指针的某个地址的一个指针。在 Rust 中，与此相对照，编译器会确保引用绝不会成为悬空引用：在有着到某数据的引用时，编译器会确保在到该数据的引用，超出作用域之前，被引用的数据不超出作用域。

下面就来创建一个悬空引用，看看 Rust 如何以编译器错误，来阻止悬空引用：

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

下面就是报错：


```console
$ cargo run
   Compiling ownership_demo v0.1.0 (/home/peng/rust-lang/projects/ownership_demo)
error[E0106]: missing lifetime specifier
 --> src/main.rs:5:16
  |
5 | fn dangle() -> &String {
  |                ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
  |
5 | fn dangle() -> &'static String {
  |                ~~~~~~~~

For more information about this error, try `rustc --explain E0106`.
error: could not compile `ownership_demo` due to previous error
```

此错误消息提到了一个这里还没有讲到特性：生命周期（lifetimes）。在第 10 章将 [详细讨论生命周期](Ch10_Generic_Types_and_Lifetimes.md#使用生命周期对引用加以验证)。不过，忽略掉生命周期有关的那部分错误，那么该错误消息就真的包含了，这段代码为何是问题代码的关键原因：

```console
this function's return type contains a borrowed value, but there is no value
for it to be borrowed from
```

下面来细看一下，这里的 `dangle` 代码各个阶段到底发生了什么：

文件名：`src/main.rs`

```rust
fn dangle() -> &String {    // 函数 dangle 返回的是到某个 String 值的引用
    let s = String::from("hello");  // 变量 s 是个新的 String 值

    &s  // 这里返回了一个到该 String，变量 s 的引用
}   // 到这里，变量 s 超出了作用域，进而被丢弃了。他的内存就没了。
    // 危险所在！
```

由于变量 `s` 是在函数 `dangle` 内部创建的，那么在函数 `dangle` 的代码执行完毕时，变量 `s` 就将被解除内存分配（deallocated）。而这里还在尝试返回一个到他的引用。那就意味着这个引用，就会指向到一个无效的 `String`。那就不好了！Rust 是不会允许这样干的。

这里的解决办法，就是直接返回那个 `String` 值：

```rust
fn dangle() -> String {
    let s = String::from("hello");

    s
}
```

## 引用的规则

**The Rules of References**

下面来对前面已经讨论过有关引用的东西，进行一下总结回顾：

- 在任意给定时间点，都 *要么* 只能有一个可变引用，*要么* 有任意数量的不可变引用（at any given time, you can have *either* one mutable reference *or* any number of immutable references）；
- 引用必须一直有效（references must always be valid）。

接下来，咱们将看看，一种不同类别的引用：切片（slices）。
