# 高级类型

**Advanced Types**


Rust 的类型系统有着一些到目前为止咱们曾提到过但尚未讨论过的特性。咱们将以一般意义上检视新型模式作为类型为何有用，而讨论新型模式开始。随后咱们将移步到类型别名，一项与新型模式类似，不过有着些许不同语义的特性。咱们还将讨论 `!` 类型与动态大小的类型。


## 为类型安全与抽象而运用新型模式

**Using the Newtype Pattern for Type Safety and Abstraction**


> **注意**：此小节假定你已读过早先的 [“使用新型模式来再外层类型上实现外层的特质”](#使用新型模式在外层类型上实现外层的特质") 小节。

对于那些超出到目前为止咱们曾讨论过的任务，包括静态强制要求值绝不会混淆，以及表明某个值的单位等等，新型模式同样是有用的。在清单 19-15 中，咱们就曾看到一个使用新型，表明单位的一个示例：回顾到 `Millimeters` 与 `Meters` 两个结构体，都曾将 `u32` 值封装在新型中。而若咱们编写了带有一个类型 `Millimeters` 参数的函数，那么咱们就无法编译某个偶然尝试以类型 `Meters` 或普通 `u32` 的值，调用那个函数的程序。


咱们还可以使用新型模式，来抽象出某个类型的一些实现细节：新的类型可暴露处不同意私有内部类型 API 的一个公开 API。

新类型还可以隐藏内部实现。比如，咱们可提供一个 `People` 类型，来封装一个存储着某人与其名字关联的 ID 的 `HashMap<i32, String>`。使用 `People` 的代码，只需与咱们提供的公开 API，比如某个将名字字符串添加到 `People` 集合的方法交互；那些代码将不需要知悉咱们在内部分配了`i32` 的 ID 给那些名字。新型模式是达成，咱们曾在第 17 章讨论过的 [“隐藏实现细节的封装”](Ch17_Object_Oriented_Programming_Features_of_Rust.md#隐藏了实现细节的封装) 的一种轻量方式。


## 使用类型别名创建类型同义词

**Creating Type Synonyms with Type Aliases**


Rust 提供给到既有类型另一个名字的声明 *类型别名，type alias* 的能力。为此，咱们要使用 `type` 关键字。比如，咱们可以像下面这样，创建到 `i32` 的别名 `Kilometers`：

```rust
type Kilometers = i32;
```

现在，别名 `Kilometers` 便是 `i32` 的同义词了；与在清单 19-15 中咱们曾创建的 `Millimeters` 与 `Meters` 两个类型不同，`Kilometers` 不是个单独的、新类型。有着类型 `Kilometers` 的那些值，将与类型 `i32` 的那些值做同样对待：


```rust
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    assert_eq! (x, y);
```

由于 `Kilometers` 与 `i32` 为同样类型，因此咱们可将这两种类型的值相加，且咱们可将 `Kilometers` 值传递给取 `i32` 参数的那些函数。但是，在使用这种方法时，咱们不会获得咱们早先所讨论的新型模式中的类型检查的那些益处。换句话说，当咱们在一些地方混淆了 `Kilometers` 与 `i32` 时，编译器将不会给到咱们一个报错。

类型同义词的一种主要用例，是为减少重复。比如，咱们可能有下面这样一个冗长的类型：


```rust
Box<dyn Fn() + Send + 'static>
```


在函数签名中，以及在全部代码中作为类型注解编写这种冗长类型，就会令人疲倦而容易出错。设想有个全部是下面清单 19-24 中代码的项目：

```rust
    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println! ("嗨"));

    fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
        // --跳过代码--
    }

    fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
        // --跳过代码--
    }
```

*清单 19-24：在多处使用长类型*


类型别名通过降低重复，而令到这样的代码更为可管理。在下面清单 19-25 中，咱们为那个冗长类型，引入了一个名为 `Thunk` 的别名，从而便可以使用这个更简短的别名 `Thunk`，替换全部的该种类型。

```rust
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println! ("嗨"));

    fn takes_long_type(f: Thunk) {
        // --跳过代码--
    }

    fn returns_long_type() -> Thunk {
        // --跳过代码--
    }
```

*清单 19-25：引入类型别名 `Thunk` 来减少重复*

这样的代码，阅读和编写起来要容易得多！给类型别名选择有意义的名字，也可以有助于表达咱们的意图（ *形实替换，thunk* 是个表示会在稍后被计算执行，因此对于会被存储的闭包，其是个恰当的名字）。

类型别名，还普遍用于 `Result<T, E>` 下的消除重复。设想标准库中的 `std::io` 模组。I/O 操作经常会返回一个 `Result<T, E>`，以处理操作失效时的情况。这个库有个表示了所有可能 I/O 错误的 `std::io::Error` 结构。`std::io` 中的许多函数，都会在那个 `E` 为 `std::io::Error` 下，返回 `Result<T, E>`，比如 `Write` 特质中的这些函数：

```rust
use std::fmt;
use std::io::Error;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn flush(&mut self) -> Result<(), Error>;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
}
```

其中的 `Result<..., Error>` 就被重复了很多。由此，`std::io` 便有了下面这样的类型别名声明：

```rust
type Result<T> = std::result::Result<T, std::io::Error>;
```

由于这种声明是在 `std::io` 模组中，因此咱们就可以使用完全合格的别名 `std::io::Result<T>`；那即是，带有 `E` 被填充为 `std::io::Error` 的 `Result<T, E>`。那个 `Write` 特质的函数签名，最终看起来就像下面这样了：

```rust
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<();
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}
```

类型别名以这两种方式发挥作用：其令到代码更易于编写 *并* 在整个 `std::io` 层面给到咱们一个一致的接口。由于其为一个别名，因此他仅是另一个 `Result<T, E>`，这意味着咱们可以与其一道使用那些全部工作于 `Result<T, E>` 上的方法，以及诸如 `?` 运算符那样的特殊语法。


## 永不返回的永不类型

**The Never Type that Never Returns**


Rust 有着一种因其没有值，而因此在类型理论术语中，叫做 *空类型，empty type* 的名为 `!` 的类型。因为在某个函数绝不会返回值时，这个类型立于返回值类型处，所以咱们称其为 *永不类型，never type*。下面是个示例：

```rust
fn bar() -> ! {
    // --跳过代码--
}
```

此代码读作 “函数 `bar` 返回永不。” 返回永不的函数被称为 *发散函数，diverging functions*。咱们无法创建出类型 `!` 的值，因此 `bar` 就永不会有可能返回值。


然而一种咱们永不能创建出值的类型，到底有什么用处呢？回顾到清单 2-5 中，作为那个猜数游戏一部分的代码；咱们已在在下面清单 19-26 中，重现了他的一点点：

```rust
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
```

*清单 19-26：有着一个以 `continue` 结束支臂的 `match` 表达式*

那个时候，咱们跳过了此代码的一些细节。而在第 6 章中的 [“`match` 控制流运算符”](Ch06_Enums_and_Pattern_Matching.md#match-控制流结构) 小节，咱们曾讨论了 `match` 支臂必须全部返回同一类型。那么，比如说，下面的代码就不会工作：


```rust
    let guess = match guess.trim().parse() {
        Ok(_) => 5,
        Err(_) => "你好",
    }
```

此代码中的 `guess` 类型，将必须为整数与字符串，而 Rust 要求 `guess` 只有一种类型。那么 `continue` 到底返回的是什么呢？到底是怎样咱们才在清单 19-26 中，曾被允许从一个支臂返回一个 `u32`，并有着以 `continue` 结束另一个支臂的呢？

描述这种行为的正式方式，即类型 `!` 的表达式，可被强制转换为任何别的类型。由于 `continue` 不会返回值，因此咱们就被允许以 `continue` 结束这个 `match` 支臂；相反，这个 `match` 支臂将控制移回到该循环的顶部，因此在 `Err` 情形下，咱们就绝不会赋给 `guess` 一个值。

在 `panic!` 宏下，这个永不类型也是有用的。回顾到咱们在 `Option<T>` 值上调用 `unwrap` 函数来生成一个值，或在此定义下中止运行：


```rust
impl<T> Option<T> {
    pub fn unwrap(self) -> {
        match self {
            Some(val) => val,
            None => panic! ("在 `None` value 上调用了 `Option::unwrap()`"),
        }
    }
}
```

此代码中，与清单 19-26 中那个 `match` 同样的事情发生了：Rust 会发现那个 `val` 有着类型 `T`，且 `panic!` 有着类型 `!`，因此整个 `match` 表达式的结果便是 `T`。此代码之所以有效，是由于 `panic!` 不会产生值；他会终止这个程序。在 `None` 情形下，咱们不会从 `unwrap` 返回值，所以此代码是有效的。

最后一个有着类型 `!` 的表达式，则是一个 `loop`：

```rust
    print! ("永永 ");

    loop {
        print! ("远远 ");
    }
```

这里，那个循环永不会结束，因此 `!` 便是该表达式的值。但是，若咱们包含了一个 `break`，由于这个循环会在其到达 `break` 时终止，因此这就不再成立了。


## 动态大小的类型与 `Sized` 特质

**Dynamically Sized Types and the `Sized` Trait**


Rust 需要知道其类型的确切情况，比如给某种特定类型值分配多少的内存空间。在一开始这就给其类型系统的一个角落留下了一点混乱：那便是 *动态大小类型，dynamically sized types* 这个概念。此概念有时被称为 DSTs 或 *未知大小类型，unsized types*，这些类型让咱们编写出，使用了仅在运行时才知道其大小值的代码来。

下面来深入到名为 `str`，贯穿这本书咱们一直都在使用一个的动态大小类型细节。那正是 `str`，而非 `&str`，确实是个 DST。在运行时之前，咱们是无法掌握字符串有多长，就是说咱们无法创建出一个类型 `str` 的变量，也无法取类型 `str` 的参数。设想下面的这段无法工作的代码：

```rust
    let s1: str = "致以问候！";
    let s2: str = "最近过得怎么样？";
```

Rust 需要清楚，要给特定类型的任何值分配多少内存，且某种类型的所有值，都必须使用同样数量的内存。若 Rust 运行咱们编写此代码，那么这两个 `str` 值就将需要占据同样数量的内存空间。但他们有着不同长度：`s1` 需要 15 字节的存储，而 `s2` 需要 `24` 字节。这就是为何创建保存动态大小类型值的变量不可行的原因。

那么咱们要怎么做呢？在这种情况下，咱们就已经知道答案了：咱们要令到 `s1` 与 `s2` 的类型为 `&str` 而非 `str`。从第 4 章的 [“字符串切片”](Ch04_Understanding_Ownership.md#字符串切片) 小节，回顾到切片数据结构，只会存储其开始位置和切片的长度。因此尽管 `&T` 是存储了 `T` 所处内存地址的单个值，而一个 `&str` 则是 *两个* 值：`str` 的地址与其长度。如此，咱们就知道某个 `&str` 在编译时的大小了：其为 `uszie` 长度的两倍。那便是，咱们总是清楚 `&str` 的大小，而不管他所指向的字符串有多长。一般来说，这就是 Rust 中动态大小类型被运用的方式：他们有着存储了动态信息大小的额外的一点元数据。动态大小类型的黄金法则，就是咱们必须始终把那些动态大小类型的值，放置某种指针之后。

咱们可将 `str` 与所有类别的指针结合：比如，`Box<str>` 或 `Rc<str>`。事实上，之前咱们就已经见到过这样的，只不过是在一种不同的动态大小类型下：那便是特质。每个特质都是咱们可以通过使用特质名字而加以引用的动态大小类型。在第 17 章中的 [“使用允许不同类型值的特质对象”](Ch17_Object_Oriented_Programming_Features_of_Rust.md#使用允许不同类型值的特质对象) 小节，咱们曾提到为了将特质用作特质对象，咱们就必须将其放在指针之后，比如 `&dyn Trait` 或 `Box<dyn Trait>` （`Rc<dyn Trait>` 也应生效）。

为处理 DSTs 相关问题，Rust 提供了 `Sized` 特质，来判断在编译时某个类型的大小是否已知。在运行时大小已知的全部物件，都已自动实现了这个特质。此外，Rust 会隐式地将 `Sized` 上的边界，添加到每个泛型函数。那就是说，像下面的一个泛型函数：

```rust
fn generic<T>(t: T) {
    // --跳过代码--
}
```

实际上会被如咱们像下面写的这样被对待：


```rust
fn generic<T: Sized>(t: T) {
    // --跳过代码--
}
```

默认情况下，泛型函数只将在那些编译时有着已知大小的类型上工作。但是，咱们可以使用下面的特殊语法来解除这种限制：


```rust
fn generic<T: ?Sized>(t: &T) {
    // --跳过代码--
}
```

在 `?Sized` 上的特质边界，表示 “`T` 可能是也可能不是 `Sized` 的”，而这样的注解就会重写泛型在编译时务必要有已知大小的默认限制。有着这种意义的 `?Trait` 语法，只对 `Sized` 可用，对其他任何特质都是不可用的。

还要注意咱们已将那个参数 `t` 的类型，从 `T` 更换为了 `&T`。由于这个类型可能不是 `Sized`，因此咱们就需要在某种指针之后使用他。在这种情况下，咱们选择了一个引用。

接下来，咱们将谈谈函数与闭包！


（End）


