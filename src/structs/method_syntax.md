# 方法语法

**Method Syntax**


*方法* 与函数类似：我们用 `fn` 关键字和一个名字，来声明方法，方法可以有参数和返回值，同时他们还包含一些代码，这些代码会在从其他地方调用该方法时运行。与函数不同的是，方法是在某个结构体（或某个枚举或特质对象，我们将在 [第 6 章](../Ch06_Enums_and_Pattern_Matching.md) 和 [第 17 章](../oop/trait_objects.md) 分别介绍）的上下文中定义的，而且方法的第一个参数，总是表示该方法被调用所在的结构体实例本身的 `self`。


## 定义出方法

**Defining Methods**


如下清单 5-13 所示，我们来修改以 `Rectangle` 实例为参数的那个 `area` 函数，转而构造出一个定义在 `Rectangle` 结构上的 `area` 方法。


```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println! ("该矩形的面积为 {} 平方像素。",
        rect1.area()
    );
}
```

*清单 5-13：在 `Rectangle` 结构体上定义一个 `area` 方法*


为了在 `Rectangle` 上下文中定义这个函数，我们为 `Rectangle` 创建了一个 `impl`（implementation，实现）代码块。该 `impl` 代码块中的所有内容，都将与 `Rectangle` 这个类型相关联。然后，我们将那个 `area` 函数，移入 `impl` 的花括号中，并把函数签名中的首个（且本示例中唯一的）参数修改为 `self`， 同时修改函数体中的各处。在 `main` 中，在我们曾调用过 `area` 函数并将 `rect1` 作为参数传递的地方，我们便可以使用 *方法语法，method syntax*，在咱们的 `Rectangle` 实例上，调用 `area` 方法。方法语法位于某个实例之后：我们要添加一个后跟方法名称、圆括号和任何参数的一个点。

在 `area` 的签名中，我们使用了 `&self` 而不是 `rectangle: &Rectangle`。`&self` 实际上是 `self：&Self` 的缩写。在 `impl` 代码块中，`Self` 这个类型，是 `impl` 代码块，所针对的那个类型的别名。方法必须将名为 `self`，类型为 `Self` 的参数，作为其第一个参数，因此 Rust 允许咱们，在第一个参数处，将其缩写为仅 `self` 这个名字。请注意，就像在 `rectangle: &Rectangle` 中一样，我们仍然需要在 `self` 这个简写前面，使用 `&` 来表明该方法借用了 `Self` 这个实例。方法可以取得 `self` 的所有权，也可以不可变地借用 `self`（就像我们在这里所做的），还可以可变地借用 `self`（就像借用其他参数一样）。


> **译注**：`&self` - 不可变借用；`&mut self` 可变借用；`self` - 取得所有权，发生所有权转移，`self` 所指向的内存堆上值，原先那个在栈上的变量将失效。


我们在这里选择 `&self` 的原因，与我们在函数那个版本中使用 `&Rectangle` 的原因相同：我们不打算取得所有权，我们只想读取该结构体中的数据，而不是向其写数据。如果我们打算修改调用方法的实例（作为该方法的一部分），我们可以使用 `&mut self` 作为第一个参数。只使用 `self` 作为第一个参数，来取得实例所有权的方法并不多见；这种方法通常用于该方法会将 `self` 转换成其他东西，且咱们想要防止调用者，在这种转换后继续使用原始实例的时候。

除了提供方法语法和不必在每个方法的签名中，重复 `self` 的类型外，使用方法而不是函数的主要原因，是为了组织。我们把所有能用类型实例做的事情，都放在一个 `impl` 块中，而不是让我们代码的未来用户，在我们提供的库中不同地方，检索 `Rectangle` 的功能。

请注意，我们可以选择将某个方法，命名为与结构体的某个字段同名。例如，我们可以为 `Rectangle` 定义一个名为 `width` 的方法：


文件名：`src/main.rs`

```rust
impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect1.width() {
        println! ("该矩形的宽度不为零；其为 {}", rect1.width);
    }
}
```


在这里，我们选择将这个 `width` 方法，构造为在实例的 `width` 字段中值大于 `0` 时，返回 `true`；如果该值为 `0`，则返回 `false`：我们可以在与某个字段的同名方法中，使用这个字段来达到任何目的。在 `main` 中，当我们在 `rect1.width` 后加上括号时，Rust 就知道我们指的是 `width` 这个方法。当我们不使用括号时，Rust 知道我们指的是 `width` 字段。

通常（但不总是），当我把某个方法，命名为与一个字段同名时，我们就希望他只返回字段中的值，而不做其他任何事情。这样的方法称为 *获取器，getter*，而 Rust 并未像其他语言那样，自动为结构的字段实现获取器。获取器之所以有用，是因为我们可以将该字段，构造为私有，而将该方法构造为公开，从而将对该字段的只读访问，实现为该类型公开 API 的一部分。我们将在 [第 7 章](../packages_crates_and_modules/paths.md#使用-pub-关键字对路径进行暴露) 讨论什么是公开，public 和私有，private，以及如何将字段或方法，指定为公开或私有。


> **`->` 操作符在哪里？**
>
> 在 C 和 C++ 中，有两种不同的操作符用于调用方法：如果是直接调用对象上的方法，咱们会使用 `.`；如果是调用到对象的某个指针上的方法，并且需要首先解引用该指针时，则要使用 `->`。换句话说，如果 `object` 是个指针，则 `object->something()` 类似于 `(*object).something()`。
>
> Rust 没有与 `->` 等价的运算符；相反，Rust 有一项称为 *自动引用和解引用，automatic referencing and dereferencing* 的特性。在 Rust 中，调用方法是少数几个具有这种行为的地方之一。
>
> 其工作原理如下：当咱们以 `object.something()` 调用某个方法时，Rust 会自动加入 `&`、`&mut` 或 `*`，以便 `object` 与该方法的签名相匹配。换句话说，下面两个方法是相同的：
>
```rust
p1.distance(&p2);
(&p1).distance(&p2);
```
>
> 第一种看起来更简洁。这种自动引用行为之所以有效，是因为方法有明确的接收者 -- 类型 `self`。有了方法的接收者和名字，Rust 就能明确确定，该方法是在读取 (`&self`)、改变 (`&mut self`) 还是消费 (`self`)。Rust 将方法的接收者，构造为隐式借用，这一事实，是在实践中，令到所有权符合人机工程学的重要部分。


## 带有更多参数的方法

**Methods with More Parameters**


我们来通过在 `Rectangle` 结构体上，实现第二个方法，来练习运用方法。这一次，我们希望 `Rectangle` 的实例，取另一 `Rectangle` 实例，在第二个 `Rectangle` 可以完全容纳在 `self`（第一个 `Rectangle`）中时，则返回 `true`；否则，返回 `false`。也就是说，只要我们已定义出 `can_hold` 方法，我们就可以编写下面清单 5-14 中所示的程序。


文件名：`src/main.rs`

```rust
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println! ("rect1 可以容纳 rect2 吗？{}", rect1.can_hold(&rect2));
    println! ("rect1 可以容纳 rect3 吗？{}", rect1.can_hold(&rect3));
}
```

*清单 5-14：使用尚未编写的 `can_hold` 方法*


由于 `rect2` 两个边都小于 `rect1` 的两个边，而 `rect3` 则宽于 `rect1`，因此预期输出结果如下：


```console
rect1 可以容纳 rect2 吗？true
rect1 可以容纳 rect3 吗？false
```

我们知道咱们是要定义某个方法，因此其将位于那个 `impl Rectangle` 代码块中。方法的名称，将是 `can_hold`，同时他将取另一 `Rectangle` 的不可变借用作为参数。通过查看调用该方法的代码，我们可以判断出该参数的类型：`rect1.can_hold(&rect2)` 传入了 `&rect2`，其为对 `Rectangle` 实例 `rect2` 的不可变借用。这是有道理的，因为我们只需要读取 `rect2`（而不是写入，那意味着我们需要一个可变借用），而且我们希望 `main` 保留对 `rect2` 的所有权，这样我们就可以在调用 `can_hold` 方法后，再次使用他。`can_hold` 的返回值，将是个布尔值，其实现将检查 `self` 宽度和高度，是否分别大于另一 `Rectangle` 的宽度和高度。咱们来将这个新的 `can_hold` 方法，添加到清单 5-13 中的 `impl` 代码块中，如下清单 5-15 所示。


文件名：`src/main.rs`

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        (self.width > other.width && self.height > other.height) ||
            (self.width > other.height && self.height > other.width)
    }
}
```

*清单 5-15：在 `Rectangle` 上实现这个会取另一 `Rectangle` 实例作为参数的 `can_hold` 方法*


当我们使用清单 5-14 中的 `main` 函数，运行这段代码时，就会得到我们想要的输出。方法可以取我们在 `self` 参数后，添加到签名的多个参数，而这些参数的作用，就跟函数中的参数一样。


> **译注**：实际上，这个 `can_hold` 的实现有错误，因为其没有考虑到矩形倒转后可以容纳的情况，改进后的 `can_hold` 如下。


```rust
    fn can_hold(&self, other: &Rectangle) -> bool {
        (self.width > other.width && self.height > other.height)
            || (self.width > other.height && self.height > other.width)
    }

```


## 关联函数

**associated functions**


由于定义在 `impl` 代码块内部的全部函数，都是与那个在 `impl` 关键字之后命名的类型相关联的，因此他们都叫做 *关联函数（associated functions）*。因为一些关联函数不需要用到该类型的实例，因此可把这些函数定义为不将 `self` 作为首个参数的关联函数（而这样的话，这些函数就不是方法了）。前面就已用到过这样的一个关联函数：`String::from` 函数就是定义在 `String` 类型上的。

非方法的关联函数，通常用于将会返回一个该结构体新实例的构造函数。比如，这里就可提供有着一维参数，并将该一维参数同时用作宽和高的这么一个关联函数，如此就令到相比于两次指定同样值，而更容易创建除正方形的 `Rectangle`。

文件名：`src/main.rs`

```rust
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
```

要调用这个关联函数，就要使用带有结构体名字的 `::` 语法；`let sq = Rectangle::square(3);` 就是一个示例；该函数是是在那个结构体的命名空间之下的：`::` 语法，同时用于关联函数，与由模组创建出的命名空间。在第 7 章会讨论到 Rust 的模组概念。


## 多个 `impl` 代码块

**Multiple `impl` Blocks**


所有结构体都允许有多个 `impl` 代码块。比如前面的清单 5-15 就与下面清单 5-16 给出的代码等价，其中各个方法都在各自的 `impl` 代码块中：

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        (self.width > other.width && self.height > other.height) ||
            (self.width > other.height && self.height > other.width)
    }
}
```

*清单 5-16：使用多个 `impl` 代码块对清单 5-15 进行重写*

虽然这里并无将这些方法分开到多个 `impl` 代码块中的理由，不过这样做也是有效的语法。在第 10 章讨论到泛型和特质时，就会看到多个 `impl` 代码块是有用的情形。

# 本章小节

结构体实现了创建对于特定领域有意义的定制类型。通过运用结构体，就可以将有关联的数据片段相互连接起来，并给各个数据取名字来让代码清晰。在 `impl` 代码块中，可定义与类型关联的函数，而方法则是一类实现了指定结构体实例所拥有行为的关联函数。

然而结构体并非能够创建定制类型的唯一方式：加下了就要转向到 Rust 的枚举特性，将另一工具加入到编程工具箱。
