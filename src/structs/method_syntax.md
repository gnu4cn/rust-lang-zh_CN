# 方法

方法与函数类似：我们以 `fn` 关键字和名字声明他们，他们可以有参数和返回值，并且他们包含一些在方法于别处被调用时运行的代码。与函数不同，方法定义在结构体（或枚举或特质对象，我们将在 [第 6 章](../Ch06_Enums_and_Pattern_Matching.md) 和 [第 18 章](../oop/trait_objects.md) 分别介绍他们）的上下文中，并且他们的第一个参数始终时 `self`，表示方法正于其上被调用的结构体实例。


## 方法语法

我们来修改以 `Rectangle` 实例为参数的 `area` 函数，转而构造一个定义在 `Rectangle` 结构上的 `area` 方法，如下清单 5-13 中所示。

<a name="listing_5-13"></a>
文件名：`src/main.rs`

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

**清单 5-13**：定义 `Rectangle` 结构体上的 `area` 方法

为了在 `Rectangle` 的上下文中定义函数，我们开始了个 `Rectangle` 的 `impl`（实现，implementation）代码块。这个 `impl` 代码块中的所有内容都将与 `Rectangle` 类型关联。然后，我们将 `area` 函数移入 `impl` 的花括号中，并把函数签名中的第一个（本示例中的唯一）参数修改为 `self`， 并修改函数体中的各处。在 `main` 中，在我们曾调用 `area` 函数并将 `rect1` 作为参数传递的地方，我们便可使用 *方法语法，method syntax*，调用咱们 `Rectangle` 实例上的 `area` 方法。方法语法位于实例之后：我们添加一个点，后跟方法名称、圆括号及任何的参数。

在 `area` 的签名中，我们使用 `&self` 而不是 `rectangle: &Rectangle`。`&self` 实际上是 `self：&Self` 的缩写。在 `impl` 代码块中，类型 `Self` 是该 `impl` 代码块所针对类型的别名。方法必须以名为 `self` 的类型 `Self` 参数作为其第一个参数，因此 Rust 允许咱们在第一个参数处可仅以名字 `self` 缩写这一参数。请注意，我们仍然需要在 `self` 简写前使用 `&`，表明这个方法借用 `Self` 实例，就像在 `rectangle: &Rectangle` 中所做的那样。方法可以取得 `self` 的所有权；或不可变地借用 `self`，就像我们在这里所做的那样；或者可变地借用 `self`，就像他们可以借用任何别的参数那样。


> **译注**：
>
> - `&self` - 不可变借用；
> - `&mut self` 可变借用；
> - `self` - 取得所有权，发生所有权转移，`self` 所指向的内存堆上值，原先那个在栈上的变量将失效。


出于我们在函数版本中使用 `&Rectangle` 的同一原因，我们在这里选择 `$self`：我们不打算取得所有权，我们只想读取结构体中的数据，而不是写入他。若作为方法执行内容的一部分，咱们打算修改我们于其上调用方法的实例时，我们就要使用 `&mut self` 作为第一个参数。通过只使用 `self` 作为第一个参数让方法取得实例的所有权很少见；这种做法通常会在方法会将 `self` 转换成其他内容，而咱们打算阻止调用者在转换后使用原始实例时用到。

除了提供方法语法与不必在每个方法的签名中重复 `self` 的类型外，使用方法而不是函数的主要原因是为了组织。我们把所有能对类型实例做的事情都放在一个 `impl` 块中，而不是让我们代码的未来用户，在我们提供的库中不同地方检索 `Rectangle` 的能力。

请注意，我们可以选择将方法取名为与结构体字段之一同样的名字。例如，我们可以在 `Rectangle` 上定义一个也叫做 `width` 的方法：


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
        println! ("矩形的宽度不为零；其为 {}", rect1.width);
    }
}
```


在这里，我们选择将这个 `width` 方法构造为当实例的 `width` 字段中的值大于 `0` 时返回 `true`，并当该值为 `0` 时返回 `false`：我们可出于任何目的在与字段同名的方法中使用该字段。在 `main` 中，当我们在 `rect1.width` 后加上括号时，Rust 就知道我们指的是 `width` 这个方法。在我们未使用括号时，Rust 知道我们指的是 `width` 这个字段。

通常，但并非总是，当我们给方法取了与字段同样的名字时，我们就会希望他只返回该字段中的值，而不做其他任何事情。像这样的方法称为 *获取器，getter*，而 Rust 并未像一些别的语言那样自动对结构字段实现他们。获取器很有用，因为我们可以将字段构造为私有，而将方法构造为公开，从而作为类型公开 API 的一部分，实现对字段的只读访问。我们将在 [第 7 章](../packages_crates_and_modules/paths.md#使用-pub-关键字对路径进行暴露) 中讨论什么是公开和私有，以及如何将字段或方法指定为公开或私有。


<a name="auto_deref"></a>
> **`->` 操作符在哪里？**
>
> 在 C 和 C++ 中，有两种不同的运算符用于调用方法：当咱们直接在对象上调用方法时，咱们会使用 `.`；而当咱们在指向某个对象的指针上调用方法，且需要首先解引用该指针时，咱们就要使用 `->`。换句话说，当 `object` 是个指针时，那么 `object->something()` 就类似于 `(*object).something()`。
>
> Rust 没有 `->` 的等价物；相反，Rust 有一项称为 *自动引用及解引用，automatic referencing and dereferencing* 的特性。调用方法属于 Rust 中有着这一行为的少数地方之一。
>
> 下面是其工作原理：当咱们以 `object.something()` 调用方法时，Rust 会自动添加 `&`、`&mut` 或 `*`，以便 `object` 与方法的签名匹配。换句话说，下面两行是相同的：
>
> ```rust
> p1.distance(&p2);
> (&p1).distance(&p2);
> ```
>
> 第一行看起来更简洁。这种自动引用行为之所以有效，是因为方法有个明确的接收者 -- 类型 `self`。给定方法的接收者和名字后，Rust 就可以明确地计算出该方法是在读取 (`&self`)、改变 (`&mut self`) 还是消费 (`self`)。Rust 将方法的接收者的借用构造为隐式这一事实，是令到所有权在实践中符合人机工程学的重要部分。


## 有着更多参数的方法

我们来通过在 `Rectangle` 结构体上实现第二个方法练习使用方法。这次我们希望 `Rectangle` 实例取另一 `Rectangle` 实例，当第二个 `Rectangle` 可完全容纳在 `self`（第一个 `Rectangle`）内时返回 `true`；否则，他应返回 `false`。也就是说，一旦我们定义了 `can_hold` 方法，我们就希望能够编写出下面清单 5-14 中所示的程序。


<a name="listing_5-14"></a>
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

**清单 5-14**：使用尚未编写的 `can_hold` 方法

预期输出将如下，因为 `rect2` 两条边都小于 `rect1` 的两条边，而 `rect3` 宽于 `rect1`：


```console
rect1 可以容纳 rect2 吗？true
rect1 可以容纳 rect3 吗？false
```

我们知道咱们是要定义一个方法，因此他将位于 `impl Rectangle` 代码块内。方法的名字将是 `can_hold`，同时他将取另一 `Rectangle` 的不可变借用作为参数。通过查看调用该方法的代码我们可以区分这一参数的类型：`rect1.can_hold(&rect2)` 传入 `&rect2`，这是个对 `Rectangle` 实例 `rect2` 的不可变借用。这是有道理的，因为我们只需要读取 `rect2`（而不是写入，那将意味着我们需要可变借用），并且我们希望 `main` 保留对 `rect2` 的所有权，以便我们就可以在调用 `can_hold` 方法后再次使用他。`can_hold` 的返回值将是个布尔值，并其实现将分别检查 `self` 宽度和高度是否大于另一 `Rectangle` 的宽度和高度。咱们来添加这个新的 `can_hold` 方法到清单 5-13 中的 `impl` 代码块，如下清单 5-15 中所示。


<a name="listing_5-15"></a>
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
**清单 5-15**：在 `Rectangle` 上实现 `can_hold` 方法，他会取另一 `Rectangle` 实例作为参数

当我们以清单 5-14 中的 `main` 函数运行这段代码时，我们将得到我们想要的输出。方法可以取我们于 `self` 参数后添加到签名的多个参数，而这些参数会与函数中的参数一样生效。


> **译注**：实际上，这个 `can_hold` 的实现有错误，因为其没有考虑到矩形倒转后可以容纳的情况，改进后的 `can_hold` 如下。
>
> ```rust
>     fn can_hold(&self, other: &Rectangle) -> bool {
>         (self.width > other.width && self.height > other.height)
>             || (self.width > other.height && self.height > other.width)
>     }
> ```


## 关联函数

定义在 `impl` 代码块内的所有函数都称为 *关联函数，associated function*，因为他们与 `impl` 后命名的类型相关联。我们可以定义不以 `self` 作为其第一个参数的关联函数（而因此不属于方法），因为他们不需要使用该类型的某个实例。我们已经用到过一个这样的函数：定义在 `String` 类型上的 `String::from` 函数。

不属于方法的关联函数通常用于构造器，将返回结构体的新实例。这些通常叫做 `new`，但 `new` 并非一个特殊名字而未被内置在这门语言中。例如，我们可选择提供一个名为 `square` 的关联函数，该函数将只有一个边长参数，并将其同时用作宽度和高度，从而使创将正方形的 `Rectangle` 更容易，而不必指定同一个值两次：


文件名：`src/main.rs`

```rust
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
```


这个函数中返回类型和函数体中的两个 `Self` 关键字，是在 `impl` 关键字后出现的类型的别名，在本例中即为 `Rectangle`。

为了调用这个关联函数，我们要对结构体名字使用 `::` 语法；`let sq = Rectangle::square(3);` 就是个例子。这个函数被纳入到了该结构体的命名空间：`::` 语法同时用于关联函数，及由模组创建的命名空间。我们将在 [第 7 章](../packages_crates_and_modules/defining_modules.md) 中讨论模组。


## 多个 `impl` 代码块

每个结构体都允许有多个 `impl` 代码块。例如，[清单 5-15](#listing_5-15) 相当于下面清单 5-16 中的代码，其有着都在自己 `impl` 块中的各个方法。


<a name="listing_5-16"></a>
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

**清单 5-16**：使用多个 `impl` 代码块重写清单 5-15


这里没有理由将这些方法分离为多个 `impl` 块，但这属于有效语法。我们将在第 10 章中我们讨论泛型与特质处，看到多个 `impl` 代码块很有用的情形。


# 本章小结


结构体让咱们能够创建针对咱们领域有意义的自定义类型。通过使用结构体，咱们可以将相关的数据片段相互连接，并为每个片段命名以使咱们的代码清晰明了。在 `impl` 代码块中，咱们可以定义与咱们的类型关联的函数，而方法属于一种关联函数，让咱们可指定咱们的结构体实例所具有的行为。

但结构体并非咱们可创建自定义类型的唯一方式：咱们来转向 Rust 的枚举特性，添加另一种工具到咱们的工具箱。


（End）


