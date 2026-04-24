# 高级函数和闭包

这一小节探讨与函数和闭包相关的一些高级特性，包括函数指针以及返回闭包。


## 函数指针

我们已经讨论了怎样传递闭包给函数；咱们也可以传递常规函数给函数！当咱们打算传递某个已定义好的函数，而不是定义新的闭包时，这种技巧非常有用。函数会强制转换为 `fn` 类型（`f` 小写），而不会与 `Fn` 的闭包特质混淆。`fn` 类型被称为 *函数指针，funciton pointer*。通过函数指针传递函数将允许咱们把函数作为其他函数的参数使用。

指定参数为函数指针的语法与闭包的语法类似，如下清单 20-28 中所示，其中我们定义了个名为 `add_one` 的函数，会将其参数加 1。函数 `do_twice` 取两个参数：一个指向任何取 `i32` 的参数并返回 `i32` 值的函数的函数指针，以及一个 `i32` 值。`do_twice` 函数调用函数 `f` 两次，向其传递 `arg` 值，然后将两次函数调用的结果相加。`main` 函数以参数 `add_one` 与 `5` 调用 `do_twice`。

<a name="listing_20-28"></a>
文件名：`src/main.rs`

```rust
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println! ("答案为：{answer}");
}
```

**清单 20-28**：使用 `fn` 类型接受作为参数的函数指针

这段代码打印 `答案为：12`。我们指定 `do_twice` 中的参数 `f` 是个 `fn`，取 `i32` 类型的参数并返回一个 `i32`。然后我们可以在 `do_twice` 的函数体中调用 `f`。在 `main` 中，我们可以作为第一个参数传递函数名 `add_one` 给 `do_twice`。

与闭包不同，`fn` 是一种类型而非特质，因此我们可以直接指定 `fn` 为参数类型，而不是以 `Fn` 的特质之一作为特质边界，声明一个泛型类型参数。

函数指针实现了所有三个闭包特质（`Fn`、`FnMut` 和 `FnOnce`），这意味着咱们始终可以作为参数，传递函数指针给期望闭包的函数。最好使用泛型类型和闭包特质之一编写函数，以便咱们的函数既可以接受函数，也可以接受闭包。

也就是说，咱们只希望接受 `fn` 而不接受闭包的一种示例，是与不支持闭包的外部代码交互时：C 函数可以接受函数作为参数，但 C 不支持闭包。

作为咱们既可以使用内联定义的闭包，也可以使用命名函数的示例，我们来看看标准库中 `Iterator` 特质提供的 `map` 方法的用法。要使用 `map` 方法将一个数字矢量转换为字符串矢量，我们可以使用闭包，如下清单 20-29 中所示。

<a name="listing_20-29"></a>
```rust
    let list_of_numbers = vec! [1, 2, 3];
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(|i| i.to_string()).collect();
```

**清单 20-29**：对 `map` 方法使用闭包，以转换数字为字符串

或者，我们可以将一个函数作为 `map` 的参数代替闭包。下面清单 20-30 展示了这种做法的样子。

<a name="listing_20-30"></a>
```rust
    let list_of_numbers = vec! [1, 2, 3];
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(ToString::to_string).collect();
```

**清单 20-30**：对 `map` 方法使用 `String::to_string` 函数，以转换数字为字符串

请注意，由于存在名为 `to_string` 的可用函数，因此我们必须使用在 [高级特质](./adv_traits.md#fq_syntax) 小节中提到的完全限定语法。

在这里， 我们使用的是定义在 `ToString` 特质中的 `to_string` 函数，标准库已针对任何实现 `Display` 特质的类型实现了这一特质。

自第 6 章 [“枚举取值”](Ch06_Enums_and_Pattern_Matching.md#枚举取值) 小节，回顾咱们所定义的各个枚举变种名字，也会成为一个初始化函数。咱们可以将这些初始化函数，作为实现了那些闭包特质的函数指针使用，这就意味着咱们可以把这些初始化函数，指定为取闭包的方法的参数，像下面这样：

```rust
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
```

这里咱们运用了那些经由使用 `Status::Value` 的初始化函数，于其上调用了 `map` 的那个范围中各个 `u32` 值，而创建出了一些 `Status::Value` 的实例。有的人会首选这种方式，而别的人则首选闭包。他们会编译到同样的代码，因此请使用你认为更清晰的风格。


## 返回闭包

**Returning Closures**


闭包是由特质表示的，这就意味着咱们不能直接返回闭包。在多数咱们可能打算返回特质的情形中，咱们都可以转而使用实现了该特质的具体类型，作为函数的返回值。但是，由于闭包没有可返回的具体类型，因此对于闭包是不能这样做的；就好比咱们是不被允许将函数指针作为返回值类型。


下面的代码尝试直接返回一个闭包，但其不会编译：


```rust
fn returns_closure() -> dyn Fn(i32) -> i32 {
    |x| x + 1
}
```

编译器报错如下：

```console
$ cargo build
   Compiling returning_closure v0.1.0 (/home/lenny.peng/rust-lang/returning_closure)
error[E0746]: return type cannot have an unboxed trait object
 --> src/main.rs:1:25
  |
1 | fn returns_closure() -> dyn Fn(i32) -> i32 {
  |                         ^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
  |
  = note: for information on `impl Trait`, see <https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits>
help: use `impl Fn(i32) -> i32` as the return type, as all return paths are of type `[closure@src/main.rs:2:5: 2:8]`, which implements `Fn(i32) -> i32`
  |
1 | fn returns_closure() -> impl Fn(i32) -> i32 {
  |                         ~~~~~~~~~~~~~~~~~~~

For more information about this error, try `rustc --explain E0746`.
error: could not compile `returning_closure` due to previous error
```

这个报错再度指向了那个 `Sized` 特质！Rust 不清楚他将需要多少内存空间来存储这个闭包。早先咱们就已见到了对这个问题的解决办法了。咱们可以使用一个特质对象：

```rust
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
```

这段代码可以很好地编译。有关特质对象的更多内容，请参考第 17 章中的 [“使用特质对象实现不同类型值”](Ch17_Object_Oriented_Programming_Features_of_Rust.md#使用允许不同类型值的特质对象) 小节。


接下来，咱们就要看看宏了！


（End）


