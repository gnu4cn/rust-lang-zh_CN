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

回顾第 6 章中 [枚举值](../enums_and_pattern_matching/defining_an_enum.md#枚举值) 小节，我们定义的每个枚举变种的名字，也会成为一个初始化函数。我们可以将这些初始化函数作为实现闭包特质的函数指针使用，这意味着我们指定初始化函数为取闭包的方法的参数，如下清单 20-31 中所示。

<a name="listing_20-31"></a>
```rust
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
```

**清单 20-33**：对 `map` 方法使用枚举初始化器，以从数字创建 `Status` 示例

在这里，我们通过使用 `Status::Value` 的初始化函数，使用对其调用 `map` 的范围中的每个 `u32` 值创建 `Status::Value` 的实例。有些人希望这种风格，有些人则更倾向于使用闭包。两种方式会编译为同样的代码，因此请使用咱们觉得更清晰的风格。


## 返回闭包

闭包由特质表示，这意味着咱们不能直接返回闭包。在大多数咱们可能希望返回特质的情形下，咱们可以转而使用实现该特质的具体类型作为函数的返回值。然而，对于闭包咱们通常不能这样做，因为他们没有可返回的具体类型；例如，当闭包捕获了其作用域中的任何值时，咱们就不允许使用函数指针 `fn` 作为返回类型。

相反，咱们将通常使用我们在第 10 章学过的 [`impl Trait` 语法](../generic_types_traits_and_lifetimes/traits.md#对类型实现特质)。咱们可以使用 `Fn`、`FnOnce` 和 `FnMut`，返回任何函数类型。例如，以下清单 20-32 中的代码将正常编译。

<a name="listing_20-32"></a>
文件名：`projects/returning_closure/src/main.rs`

```rust
fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}
```

**清单 20-32**：使用 `impl Trait` 语法从函数返回闭包

然而，正如我们在第 13 章中 [推断与注解闭包类型](../functional_features/closures.md#推断与注解闭包类型) 小节中指出的，每个闭包本身也属于其自己的独特类型。当咱们需要处理多个有着相同签名，却有着不同实现的函数时，就将需要为他们使用特质对象。试想一下，当咱们编写像是下面清单 20-33 中所示的代码时，会发生什么。

<a name="listing_20-33"></a>

```rust
fn main() {
    let handlers = vec![returns_closure(), returns_initialized_closure(123)];
    for handler in handlers {
        let output = handler(5);
        println!("{output}");
    }
}

fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

fn returns_initialized_closure(init: i32) -> impl Fn(i32) -> i32 {
    move |x| x + init
}
```

**清单 20-33**：创建一个由返回 `impl Fn` 类型的函数定义的闭包构成的 `Vec<T>`

这里我们有两个函数：`returns_closure` 和 `returns_initialized_closure`，他们都返回 `impl Fn(i32) -> i32`。请注意，尽管他们实现了同一类型，但返回的闭包却不同。当我们尝试编译这段代码时，Rust 会让我们知道这行不通：

```console
$ cargo run
   Compiling returning_closure v0.1.0 (/home/hector/rust-lang-zh_CN/projects/returning_closure)
error[E0308]: mismatched types
  --> src/main.rs:2:44
   |
 2 |     let handlers = vec![returns_closure(), returns_initialized_closure(123)];
   |                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected opaque type, found a different opaque type
...
 9 | fn returns_closure() -> impl Fn(i32) -> i32 {
   |                         ------------------- the expected opaque type
...
13 | fn returns_initialized_closure(init: i32) -> impl Fn(i32) -> i32 {
   |                                              ------------------- the found opaque type
   |
   = note: expected opaque type `impl Fn(i32) -> i32`
              found opaque type `impl Fn(i32) -> i32`
   = note: distinct uses of `impl Trait` result in different opaque types

For more information about this error, try `rustc --explain E0308`.
error: could not compile `returning_closure` (bin "returning_closure") due to 1 previous error
```

这一报错消息告诉我们，每当我们返回 `impl Trait` 时，Rust 都会创建一个唯一的 *不透明类型，opaque type*，其中我们无法窥见 Rust 为我们构建的具体细节，也无法推测出 Rust 将生成何种类型供我们自行编写。因此，尽管这两个函数返回了实现相同特质（`Fn(i32) -> i32`） 的闭包，但 Rust 为每个闭包生成的不透明类型却是不同的。（这类似于我们在第 17 章中 [`Pin` 类型与 `Unpin` 特质](../async/async_traits.md#pin-与-unpin-特质) 中看到的，即使不同异步块有着同一输出类型，Rust 也会为他们生成不同的具体类型。）我们已经多次看到这种问题的解决方案：我们可以使用特质对象，如下清单 20-34 中所示。

<a name="listing_20-34"></a>

```rust
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn returns_initialized_closure(init: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x + init)
}
```

**清单 20-34**：创建一个 `Vec<T>`，其中包含由返回 `Box<dyn Fn>` 的函数定义的闭包，以便他们具有相同的类型

这段代码可以正常编译。有关特质对象的更多信息，请参阅第 18 章中 [使用特质对象抽象共用行为](../oop/trait_objects.md) 小节。

接下来，我们来看看宏！
