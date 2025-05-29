# 使用 `Drop` 特质运行清理代码

**Running Code on Cleanup with `Drop` Trait**


对于灵巧指针模式来讲，第二个重要的特质是 `Drop`，他允许咱们定制某个值即将超出作用域时发生的事情。咱们可以在任何类型上，为 `Drop` 特质提供一个实现，实现代码可用于释放文件或网络连接等资源。

咱们之所以在灵巧指针上下文中引入 `Drop` 特质，是由于 `Drop` 特质的功能在实现某个灵巧指针时几乎都会用到。比如，当 `Box<T>` 被弃用时，他将释放该匣子指向的堆上的内存空间。

在某些语言中，对于某些类型，程序员必须在每次使用完这些类型的实例后，调用代码来释放内存或资源。这方面的例子包括文件句柄、套接字或锁，file handles, sockets, or locks。如果他们忘记了，系统可能会变得过载并崩溃。在 Rust 中，咱们可以指定在值超出范围时运行一段特定的代码，编译器将自动插入这段代码。因此，咱们无需小心地将清理代码，放在程序中某个特定类型的实例完成后的任何地方 -- 咱们仍然不会泄露资源!

咱们通过实现 `Drop` 特质，来指定当一个值超出作用域时要运行的代码。`Drop` 特质要求咱们实现一个名为 `drop` 的方法，他需要一个对 `self` 的可变引用。为了了解Rust 何时调用 `drop`，现在咱们就来用 `println!` 语句实现 `drop`。

下面清单 15-14 给出了仅有着一项定制功能，即在其实例超出作用域时打印出 `正在弃用 CustomSmartPointer！` 的一个 `CumstomSmartPointer` 结构体，以展示出 Rust 在何时运行这个 `drop` 函数。

文件名：`src/main.rs`

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println! ("正在使用数据 `{}` 弃用 CustomSmartPointer！", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("c - 我的事情"),
    };
    let d = CustomSmartPointer {
        data: String::from("d - 其他事情"),
    };
    println! ("已创建出一些 CustomSmartPointer 实例");
}
```

*清单 15-14：一个实现了 `Drop` 特质的 `CustomSmartPointer` 结构体，咱们将把咱们的清理代码放在这里*

`Drop` 特质包含在前奏中，included in the prelude，所以我们不需要把他带入作用域。我们在 `CustomSmartPointer` 上实现了 `Drop` 特质，并为 `drop` 方法提供到一个调用了 `println!` 的实现。`drop` 函数的主体是在咱们的类型的实例超出作用域时，打算运行的任何逻辑的地方。咱们在这里打印一些文本来直观地演示 Rust 何时调用 `drop`。

在 `main` 中，我们创建了两个 `CustomSmartPointer` 的实例，然后打印`已创建出一些 CumstomSmartPointer 实例`。在 `main` 的结尾，我们的 `CustomSmartPointer` 实例将超出作用域，Rust 将调用我们放在 `drop` 方法中的代码，打印我们的最终信息。注意，我们不需要显式地调用 `drop` 方法。

当我们运行这个程序时，我们会看到以下输出：

```console
$ cargo run
   Compiling sp_demos v0.1.0 (/home/lennyp/rust-lang/sp_demos)
    Finished dev [unoptimized + debuginfo] target(s) in 0.50s
     Running `target/debug/sp_demos`
已创建出一些 CustomSmartPointer 实例
正在使用数据 `d - 其他事情` 弃用 CustomSmartPointer！
正在使用数据 `c - 我的事情` 弃用 CustomSmartPointer！
```

当我们的实例超出作用域时，Rust 自动为我们调用了 `drop`，从而调用我们指定的代码。变量的弃用顺序与其创建顺序相反，因此 `d` 在 `c` 之前被弃用。这个例子的目的是给咱们一个直观了解 `drop` 方法如何工作的直观指引；通常咱们会指定咱们类型需要运行的清理代码，而不是打印消息。


## 使用 `std::mem::drop` 提前弃用值

**Drop a Value Early with `std::mem::drop`**


不幸的是，要禁用自动 `drop` 功能并不简单。通常情况下，禁用 `drop` 功能是没有必要的；`Drop` 特质的全部意义在于他是自动处理的。然而在少数情况下，咱们可能想要提前清理一个值。一个例子便是在运用管理锁的灵巧指针时：咱们可能想要强制使用释放锁的 `drop` 方法，这样同一作用域内的其他代码就可以获得锁。Rust 不允许咱们手动调用 `Drop` 特质的 `drop` 方法；相反，如果咱们打算强制一个值在其作用域结束前被弃用，咱们必须调用标准库提供的 `std::mem::drop` 函数。

若我们试图通过修改清单 15-14 中的 `main` 函数来手动调用 `Drop` 特质的 `drop` 方法，如清单15-15所示，我们会得到一个编译器报错：

文件名：`src/main.rs`

```rust
fn main() {
    let c = CustomSmartPointer {
        data: String::from("一些数据"),
    };
    println! ("已创建出一个 CustomSmartPointer 实例。");
    c.drop();
    println! ("在 main 结束之前这个 CustomSmartPointer 已被弃用。")
}
```

*清单 15-15：尝试调用 `Drop` 特质的 `drop` 方法来提前清理*

当我们试图编译这段代码时，我们会得到这样的报错：

```console
$ cargo run
   Compiling sp_demos v0.1.0 (/home/lennyp/rust-lang/sp_demos)
error[E0040]: explicit use of destructor method
  --> src/main.rs:17:7
   |
17 |     c.drop();
   |     --^^^^--
   |     | |
   |     | explicit destructor calls not allowed
   |     help: consider using `drop` function: `drop(c)`

For more information about this error, try `rustc --explain E0040`.
error: could not compile `sp_demos` due to previous error
```

这个错误信息指出，我们不允许显式调用 `drop`。这条错误信息使用了术语 “解构函数，destructor”，这是清理实例的函数的通用编程术语。解构函数类似于 *构造函数，constructor*，后者创建一个实例。Rust 中的 `drop` 函数就是一个特殊的解构函数。

Rust 之所以不允许咱们显式地调用 `drop`，是因为 Rust 仍然会在 `main` 函数结尾处自动调用值上的 `drop`，这将导致 *双重释放，double free* 的错误，由于 Rust 会试图对同一个值进行两次清理。

当值超出作用域时，我们无法禁用 `drop` 的自动插入，也无法显式调用 `drop` 方法。所以，如果我们需要强制一个值提前被清理，我们就使用 `std::mem::drop` 函数。

`std::mem::drop` 函数与 `Drop` 特质中的 `drop` 方法不同。咱们通过把咱们想要强制弃用的值作为参数传递来调用他。这个函数在前奏中，所以我们可以修改清单 15-15 中的 `main` 来调用 `drop` 函数，如清单 15-16 所示：

文件名：`src/main.rs`

```rust
fn main() {
    let c = CustomSmartPointer {
        data: String::from("我的事情"),
    };
    println! ("已创建出一个 CustomSmartPointer 实例。");
    drop(c);
    println! ("在 main 结束之前这个 CustomSmartPointer 已被弃用。")
}
```

*清单 15-16：调用 `std::mem::drop` 在值超出作用域前，显式地弃用该值*

运行这段代码将打印出以下内容：

```console
$ cargo run
   Compiling sp_demos v0.1.0 (/home/lennyp/rust-lang/sp_demos)
    Finished dev [unoptimized + debuginfo] target(s) in 0.40s
     Running `target/debug/sp_demos`
已创建出一个 CustomSmartPointer 实例。
正在使用数据 `一些数据` 弃用 CustomSmartPointer!
在 main 结束之前这个 CustomSmartPointer 已被弃用。
```

文本 ``正在使用数据 `一些数据` 弃用 CustomSmartPointer!`` 被打印在 `已创建出一个 CustomSmartPointer 实例。` 与 `在 main 结束之前这个 CustomSmartPointer 已被弃用。` 之间，显示 `drop` 方法在这个时间点被调用来弃用 `c`。

咱们可以通过多种方式使用 `Drop` 特质实现中指定的代码，来方便和安全地进行清理：例如，咱们可以用他来创建咱们自己的内存分配器！ 有了 `Drop` 特质和 Rust 的所有权系统，咱们不需要记得清理内存，因为 Rust 会自动完成。

咱们也不必担心因意外清理仍在使用的值而导致的问题：确保引用始终有效的所有权系统，还确保在值不再被使用时，`drop` 只被调用一次。

现在我们已经研究了 `Box<T>` 和灵巧指针的一些特性，让我们看看标准库中定义的其他几个灵巧指针。


（End）


