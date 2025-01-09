# 高级函数与闭包

**Advanced Functions and Closures**


这个小节会探讨一些与函数和闭包有关的高级特性，包括函数指针与作为返回值的闭包，function pointers and returning closures。


## 函数指针

**Function Pointers**


咱们已讲到了怎样把闭包传递给函数；咱们也可以把常规函数传递给函数！在咱们打算传递一个咱们已定义的函数，而非定义出一个新闭包时，这种技巧便是有用的。这些函数会强制转换到类型 `fn` （有着小写的 `f`），而不会与那个 `Fn` 闭包特质混淆。这个 `fn` 类型，被称为 *函数指针，funciton pointer*。使用函数指针的传递函数，将实现把函数作为其他函数参数而运用。

指明某个函数是个函数指针的语法，与参数是个闭包的语法类似，如下清单 19-27 中所示，其中咱们定义了一个往其参数加一的函数 `add_one`。函数 `do_twice` 则会取两个参数：到任何的取一个 `i32` 参数，并返回 `i32` 值函数的函数指针，以及一个 `i32` 值。这个 `do_twice` 函数会调用函数 `f` 两次，传递给他那个 `arg` 值，随后把这两次函数调用的结果相加在一起。`main` 函数使用了参数 `add_one` 与 `5` 调用 `do_twice`。


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

    println! ("答案为：{}", answer);
}
```


*清单 19-27：使用 `fn` 类型来以参数方式接收函数指针*


此代码会打印出 `答案为：12`。咱们指明了 `do_twice` 中的参数 `f` 是取一个类型 `i32` 参数，并返回一个 `i32` 的 `fn`。最后咱们便可以在 `do_twice` 函数体中调用 `f` 了。在 `main` 中，咱们可以将名为 `add_one` 的函数，作为首个参数传递给 `do_twice`。


与闭包不同，`fn` 是种类型而非一个特质，因此咱们将 `fn` 直接指定为参数类型，而非使用 `Fn` 特质之一，作为特质边界声明一个泛型参数。


函数指针实现了全部三个闭包特质（`Fn`、`FnMut` 与 `FnOnce`），意味着咱们可以一直将某个函数，作为期望得到一个闭包的函数的参数而加以传递。编写出使用了一个泛型及闭包特质之一的函数，是最佳做法，如此咱们的函数就既可以接收函数，也可以接收闭包了。

那就是说，一种咱们只想接收 `fn` 而不想接收闭包的情况，便是与并无闭包的外部代码相交互时：C 语言函数可以参数方式接收函数，但 C 语言是没有闭包的。


而作为既可以使用内联定义的闭包，又可以使用命名函数的一种情况，下面就来看看标准库中 `Iterator` 特质所提供的 `map` 函数的一种用法。要使用 `map` 函数来将某个一些数字构成的矢量值，转换为字符串的矢量，咱们可以使用一个闭包，如下面这样：

```rust
    let list_of_numbers = vec! [1, 2, 3];
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(|i| i.to_string()).collect();
```

或者咱们可以命名一个作为给 `map` 参数的函数，而非那个闭包，如下面这样：

```rust
    let list_of_numbers = vec! [1, 2, 3];
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(ToString::to_string).collect();
```

请注意由于有着多个可用的名为 `to_string` 函数，因此咱们就必须使用早先在 [“高级特质”](#高级特质) 小节中讲到的完全合格语法。这里咱们使用了那个标准库已对任何实现了 `Display` 类型，实现过了的 `ToString` 特质中的 `to_string` 函数。

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


