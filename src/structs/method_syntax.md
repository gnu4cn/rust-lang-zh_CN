# 方法语法

**Method Syntax**

*方法* 与函数类似：是以 `fn` 关键字和一个名称，来声明出方法，方法可以有参数和返回值，同时包含了在某个地方方法被调用时，运行的一些代码。与函数不同的地方在于，方法是在结构体（或者枚举或特质对象，关于枚举即特质对象，将分别在第 6 和 17 章讲到）的语境里面定义的，且方法的首个参数将始终是 `self`，这个 `self` 表示方法被调用的那个结构体实例本身。


## 定义出方法

**Defining Methods**


下面就来将那个将一个 `Rectangle` 实例作为参数的 `area` 函数，修改为定义在 `Rectangle` 结构体上的 `area` 方法，如下清单 5-13 所示：

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


为定义 `Rectangle` 上下文中的函数，这里开启了一个 `Rectangle ` 的 `impl` （implementation）代码块。此 `impl` 代码块里头的所有物件，都会与那个 `Rectangle` 类型相关联。随后这里就把原来那个 `area` 函数，移入到这个 `impl` 的花括弧里，并将函数签名中的首个（而在此情形下，也是唯一的）参数，及函数体中的各处，均修改为 `self`。在 `main` 函数，即原先调用 `area` 函数与将 `rect1` 作为参数传递的地方，现在就可以使用 *方法语法* 来调用那个在 `Rectangle` 实例上的 `area` 方法了。方法语法（the method syntax）是在实例之后：添加一个带着方法名字、括号及全部参数的点。

在 `area` 的签名中，使用了 `&self` 而不再是 `rectangle: &Rectangle`。这个 `&self` 实际上是 `self: &Self` 的简写。在 `impl` 代码块内部，类型 `Self` 就是该 `impl` 代码块所针对的类型。方法必定有着这么一个名为 `self` 类型为 `Self` 的参数，作为他们的首个参数，因此 Rust 这才允许将首个参数位置上的该参数，简写为只是他的名称 `self`。请注意这里仍然需要在 `self` 简写前使用 `&` 运算符，来表示此方法借用了 `Self` 类型的实例，这就跟 `rectangle: &Rectangle` 一样。方法可以取得 `self` 的所有权的、不可变地借用 `self` 变量，或者可变地借用 `self` 变量，对于方法的其他参数，也是这样的。

> `&self` - 不可变借用；`&mut self` 可变借用；`self` - 取得所有权，发生所有权转移，`self` 所指向的内存堆上的值原来的所有值将失效。

这里选择了 `&self`，有着与方法版本中使用 `&Rectangle` 有着同样理由：那就是不打算取得所有权，同时只打算读取结构体中的数据，而不打算写入。在作为方法要执行的一部分，要修改方法调用所在实例时，就要使用 `&mut self` 作为首个参数了。通过仅使用 `self` 作为首个参数，而取得实例所有权的情况，就非常少见了；通常在要将 `self` 转换为其他类型的数据，而要在这样的转换之后，阻止其他调用者使用原先的实例时，会用到这样的技巧。

使用方法而不是函数的主要原因，除了提供到方法语法及不必在每个方法签名中重复 `self` 的类型外，那就是为了代码的组织了。这里已将可由某个类型实例完成的事情，放在一个 `impl` 代码块中，而不是要那些后来的代码使用者，在这里提供的库的各个地方去找寻 `Rectangle` 的那些能力。

请注意可选择给方法一个与结构体字段相同的名字。比如，这里就可以在 `Rectangle` 上定义一个同样命名为 `width` 的方法：

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
        println! ("该矩形的宽不为零；他的宽为 {}", rect1.width);
    }
}
```

这里，就选择了让 `width` 方法，在实例的 `width` 字段中的值大于 `0` 时返回 `true`，在值为 `0` 时返回 `false`：在名称与某个字段相同的方法里面，可将该字段用于任何目的。在 `main` 方法中，当这里在 `rect1.width` 后跟上一对圆括号时，那么 Rust 就明白这里指的是方法 `width`了。而在没有使用一对圆括号时，Rust 就知道那里表示的是字段 `width`。

通常，但并非总是这样，在给到方法与某个字段同样名字时，要的是让那个方法返回与其同名字段中的值，而不会去干别的事情。像这样的方法，就叫做 *获取器（getters）*，而 Rust 并未像其他语言所做的那样，自动实现结构体字段的获取器。由于可将字段构造为私有，而将方法构造为公开，而由此实现对作为类型的公开 API一部分的字段的只读访问。在第 7 章中就会讨论到何为公开与私有，以及怎样将字段或方法指定为公开或私有。

> **`->` 操作符（the `->` operator）哪去了呢？**
>
> 在 C 和 C++ 中，方法调用会用到两个操作符：直接调用在对象上的方法时，要用到 `.`，而在对象的指针上调用方法时，则要用 `->` 操作符，这时还先要对该指针解除引用。换句话说，在 `object` 是个指针时，`object -> something()` 是类似于 `(*object) -> something()` 的。
> Rust 并无 `->` 操作符的等价操作符；相反，Rust 有着一项名为 *自动引用与解引用（automatic referencing and dereferencing）* 的特性。而方法调用就是 Rust 中有着这种行为表现的少数几个地方之一。
>
> 以下就是该特性的工作原理：在以 `object.something()` 调用某个方法时，Rust 会自动加上 `&`、`&mut` 或 `*`，这样 `object` 就会匹配上该方法的签名。换句话说，下面的语句是一致的：
>
```rust
p1.distance(&p2);
(&p1).distance(&p2);
```
>
> 第一个语句看起来要清楚不少。由于方法有着明确的接收者 -- 即 `self` 的类型，因此这种自动引用的行为会生效。在给定了接收者和方法名字后，Rust 就可明确地确定出该方式到底是在读取（`&self`）、改变（`&mut self`），或者是在消费（`self`）。Rust 实现了方法接收者的隐式借用这一事实，是为实现所有权系统在编程实践中符合人机交互，而所做努力的较大部分（the fact that Rust makes borrowing implicit for method receivers is a big part of making ownership ergonomic in practice）。

## 有着更多参数的方法

下面就来通过在 `Rectangle` 结构体上实现另一个方法，练习一下方法的运用。这次就要 `Rectangle` 的一个实例，去取得另一个 `Rectangle` 的实例，并在第二个 `Rectangle` 完全能放入到 `self` （即第一个 `Rectangle` ）里头时返回 `true`；否则这个方法就会返回 `false`。也就是，一旦定义了这个 `can_hold` 方法，就要能够编写下面清单 5-14 中的那个程序。

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

    println! ("rect1 可以装下 rect2 吗？{}", rect1.can_hold(&rect2));
    println! ("rect1 可以装下 rect3 吗？{}", rect1.can_hold(&rect3));
}
```

*清单 5-14：对尚未成文的 `can_hold` 方法进行使用*

由于 `rect2` 的两个边都小于 `rect1` 的两个边，而 `rect3` 的两个边都要长于 `rect1` 的两个边，因此预期的输出将看起来像下面这样：

```console
rect1 可以装下 rect2 吗？true
rect1 可以装下 rect3 吗？false
```

这里知道要定义的是个方法，因此那将会在 `impl Rectangle` 代码块内部。而方法的名称将是 `can_hold`，同时他会取得作为参数的另一 `Rectangle` 值的不可变借用。通过观察调用该方法的代码，就可以得出那个参数的类型了：`rect1.can_hold(&rect2)` 传入的是 `&rect2`，正是到变量 `rect2` 的不可变借用，而 `rect2` 又是 `Rectangle` 的一个实例。由于这里只需要读取 `rect2`（而非写入，那就意味着将需要一个可变借用了），同时这里是想要 `main` 函数保留 `rect2` 的所有权，这样就可以在 `can_hold` 方法调用之后，还可以再度使用 `rect2`，因此这样做是有理由的。`can_hold` 方法的返回值，将是个布尔值，而该方法的实现会检查 `self` 的宽和高，相应地是否都大于另一个 `Rectangle` 的宽和高。下面就把这个新的 `can_hold` 方法，加入到清单 5-13 的 `impl` 代码块，如下清单 5-15 所示。

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

*清单 5-15：对在 `Rectangle` 上的、取另一 `Rectangle` 实例作为参数的 `can_hold` 方法进行实现*

在以清单 5-14 中的 `main` 函数来运行此代码是，就会得到想要的输出。方法可取得在 `self` 参数之后添加到其签名的多个参数，同时这些参数就像函数中的参数一样生效。


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
