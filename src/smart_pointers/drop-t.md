# 通过 `Drop` 特质在清理时运行代码

对于灵巧指针模式而言，第二个重要的特质是 `Drop`，他允许咱们定制在值即将超出作用域时发生的事情。咱们可以为任何类型上的 `Drop` 特质提供实现，并且该代码可用于释放文件或网络连接等资源。

咱们之所以在灵巧指针的语境下介绍 `Drop` 特质，是因为在实现灵巧指针时，`Drop` 特质的功能几乎总是会用到。例如，当某个 `Box<T>` 被弃用时，他将解分配（释放）匣子指向的堆上空间。

在某些语言中，对于某些类型，程序员必须在每次使用完这些类型的实例后，调用代码释放内存或资源。示例包括文件句柄、套接字以及锁等。如果程序员忘记了，那么系统会过载并崩溃。在 Rust 中，咱们可以指定每当值超出作用域时要运行的一段特定代码，编译器将自动插入这段代码。因此，咱们无需小心地在程序中特定类型的实例结束的每个地方，都放置清理代码 -- 咱们仍然不会泄露资源!

咱们通过实现 `Drop` 特质，指定值超出作用域时要运行的代码。`Drop` 特质要求咱们实现一个名为 `drop` 的方法，该方法取对 `self` 的可变引用。为了了解 Rust 何时会调用 `drop`，现在我们来以 `println!` 语句实现 `drop`。

下面清单 15-14 展示了个 `CustomSmartPointer` 结构体，其唯一的定制功能是，他将在实例超出作用域时打印 `正在弃用 CustomSmartPointer！`，以展示 Rust 会于何时运行 `drop` 函数。

<a name="listing_15-14"></a>
文件名：`src/main.rs`

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println! ("正在以数据 `{}` 弃用 CustomSmartPointer！", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("我的事情"),
    };
    let d = CustomSmartPointer {
        data: String::from("其他事情"),
    };
    println! ("一些 CustomSmartPointer 实例已创建");
}
```

**清单 15-14**：`CustomSmartPointer` 结构体，实现了 `Drop` 特质，我们将于其中放置清理代码

`Drop` 特质包含在前奏中，因此我们无需带入他到作用域。我们对 `CustomSmartPointer` 实现 `Drop` 特质，并为 `drop` 方法提供了一个调用 `println!` 的实现。`drop` 方法的主体，是咱们将放置当咱们的类型的实例超出作用域时，咱们打算运行的任何逻辑之处。我们在这里打印一些文本，来直观地演示 Rust 将于何时调用 `drop`。

在 `main` 中，我们创建了两个 `CustomSmartPointer` 实例，然后打印 `一些 CustomSmartPointer 实例已创建`。在 `main` 结束处，我们的 `CustomSmartPointer` 实例将超出作用域，Rust 将调用我们放在 `drop` 方法中的代码，打印我们的最终信息。请注意，我们不需要显式地调用 `drop` 方法。

当我们运行这个程序时，我们会看到以下输出：

```console
$ cargo run
   Compiling drop-example v0.1.0 (/home/hector/rust-lang-zh_CN/projects/drop-example)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/drop-example`
一些 CustomSmartPointer 实例已创建
正在以数据 `其他事情` 弃用 CustomSmartPointer！
正在以数据 `我的事情` 弃用 CustomSmartPointer！
```

当我们的实例超出作用域后，Rust 自动为我们调用了 `drop`，从而调用我们指定的代码。变量按照他们创建的相反顺序被弃用，因此 `d` 先于 `c` 被弃用。这个实例的目的是，给予咱们 `drop` 方法工作原理的直观指引；通常，咱们会指定咱们的类型需要的清理代码，而不是打印消息。

遗憾的是，禁用自动的 `drop` 功能并不简单。禁用 `drop` 通常并无必要；`Drop` 特质的核心，就在于他是自动处理的。不过，有时咱们会希望提前清理某个值。一个示例便是使用管理锁的灵巧指针：咱们可能打算强制调用释放锁的 `drop` 方法，以便同一作用域内的其他代码可以获取该锁。Rust 不允许咱们手动调用 `Drop` 特质的 `drop` 方法；相反，当咱们打算在值的作用域结束前强制弃用该值时，咱们必须调用标准库提供的 `std::mem::drop` 函数。

如下清单 15-15 中所示，尝试通过修改清单 15-14 中的 `main` 函数，来手动调用 `Drop` 特质的 `drop` 方法是行不通的。

<a name="listing_15-15"></a>
文件名：`src/main.rs`

```rust
fn main() {
    let c = CustomSmartPointer {
        data: String::from("一些数据"),
    };
    println! ("CustomSmartPointer 实例已创建");
    c.drop();
    println! ("CustomSmartPointer 在 main 结束前被弃用");
}
```

**清单 15-15**：尝试手动调用 `Drop` 特质中的 `drop` 方法以提前清理

当我们尝试编译这段代码时，我们将得到下面这个报错：

```console
$ cargo run
   Compiling drop-example v0.1.0 (/home/hector/rust-lang-zh_CN/projects/drop-example)
error[E0040]: explicit use of destructor method
  --> src/main.rs:16:7
   |
16 |     c.drop();
   |       ^^^^ explicit destructor calls not allowed
   |
help: consider using `drop` function
   |
16 -     c.drop();
16 +     drop(c);
   |

For more information about this error, try `rustc --explain E0040`.
error: could not compile `drop-example` (bin "drop-example") due to 1 previous error
```

这个报错指出，我们不允许显式调用 `drop`。这条错误信息使用了术语 *析构函数*，这是用于清理实例的函数的通用编程术语。*析构函数* 好比 *构造函数*，后者创建实例。Rust 中的 `drop` 函数属于一个特殊的析构函数。

Rust 之所以不允许我们显式地调用 `drop`，是因为 Rust 仍将在 `main` 函数结束处自动对值调用 `drop`。这会导致 *双重释放* 的错误，因为 Rust 会尝试清理同一个值两次。

我们无法禁用值超出作用域时 `drop` 的自动插入，并且无法显式调用 `drop` 方法。因此，当我们需要强制某个值被提前清理时，我们要使用 `std::mem::drop` 函数。

`std::mem::drop` 函数不同于 `Drop` 特质中的 `drop` 方法。我们通过作为参数传递我们打算强制弃用的值来调用他。这个函数位于前奏中，因此我们可以修改清单 15-15 中的 `main` 来调用 `drop` 函数，如下清单 15-16 中所示：

<a name="listing_15-16"></a>
文件名：`src/main.rs`

```rust
fn main() {
    let c = CustomSmartPointer {
        data: String::from("一些数据"),
    };
    println! ("已创建出一个 CustomSmartPointer 实例。");
    drop(c);
    println! ("在 main 结束之前这个 CustomSmartPointer 已被弃用。")
}
```

**清单 15-16**：调用 `std::mem::drop` 在值超出作用域前，显式地弃用该值

运行这段代码将打印以下内容：

```console
$ cargo run
   Compiling drop-example v0.1.0 (/home/hector/rust-lang-zh_CN/projects/drop-example)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/drop-example`
CustomSmartPointer 实例已创建
正在以数据 `一些数据` 弃用 CustomSmartPointer！
CustomSmartPointer 在 main 结束前被弃用
```

文本 ``正在以数据 `一些数据` 弃用 CustomSmartPointer！`` 打印在 `CustomSmartPointer 实例已创建` 与 `CustomSmartPointer 在 main 结束前被弃用` 之间，表明 `drop` 方法的代码于该处被调用以弃用 `c`。

咱们可以多种方式使用指定于 `Drop` 特质实现中的代码，以使资源清理方便且安全：例如，咱们可以用他创建自己的内存分配器！ 在 `Drop` 特质和 Rust 的所有权系统下，咱们不必记得要清理资源，因为 Rust 会自动完成。

咱们也不必担心因意外清理仍在使用的值而导致的问题：确保引用始终有效的所有权系统，同时还保证 `drop` 只会在值不再被使用时被调用一次。

现在我们已经探讨了 `Box<T>` 和灵巧指针的一些特征，我们来看看定义在标准库中的其他几个灵巧指针。


（End）


