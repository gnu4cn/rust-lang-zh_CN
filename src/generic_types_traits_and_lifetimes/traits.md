# 以特质定义共用行为

所谓 *特质，a trait*，定义了特定类型具有的功能，并可与其他类型共用。咱们可以使用特质以抽象的方式定义共用行为。我们可以使用 *特质边界，trait bounds*，指定某一泛型类型可以是有着特定行为的任意类型。

> **注意**：特质类似于其他语言中通常称为 *接口，interfaces* 的特性，但也有一些区别。


## 定义特质

类型的行为由我们可以在该类型上调用的方法构成。当咱们可以在不同类型上调用相同的方法时，那么这些不同类型就共用了同样的行为。所谓特质定义，属于分组方法签名在一起，以定义实现某一目的所需的一组行为的方法。

例如，假设我们有着多个结构体，保存不同类别与数量的文本：`NewsArticle` 结构体保存特定地方的新闻报道，和 `SocialPost` 结构体，最多可以有 280 个字符，以及表明其是新帖子、转发还是对另一帖子的回复的元数据。

我们打算构造出一个名为 `aggregator` 的媒体聚合库代码箱，可以显示可能存储在 `NewsArticle` 或 `SocialPost` 实例中的数据的摘要。为此，咱们需要每种类型的摘要，并且我们将通过调用实例上的 `summarize` 方法请求这一摘要。下面清单 10-12 显示了表达这一行为的公开 `Summary` 特质的定义。


<a name="listing_10-12"></a>
文件名：`src/lib.rs`


```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

**清单 10-12**：一个由 `summarize` 方法提供的行为组成的 `Summary` 特质

在这里，我们使用 `trait` 关键字声明一个特质，然后是该特质的名字，在这一情形下为 `Summary`。咱们还声明该特质为 `pub`，以别依赖于这个代码箱的代码箱也可以使用这个特质，正如我们将在下面的几个示例中看到的那样。在花括号内，咱们声明了实现这个特质的类型的行为的方法签名，在这一情形下为 `fn summarize(&self) -> String`。

在方法签名之后，咱们没有提供花括号内的实现，而是使用分号。实现这个特质的每种类型都必须为方法的主体提供自己的定制行为。编译器将强制任何有着 `Summary` 特质的类型，都将有着完全以这个签名定义的 `summarize` 方法。

特质在其主体可以有多个方法：方法签名一行一个地列出，每行都以分号结束。


## 在类型上实现特质

现在咱们已经定义了 `Summary` 特质方法的所需签名，咱们可以在咱们的媒体聚合器中的类型上实现他了。下面清单 10-13 显示了 `NewsArticle` 结构体上 `Summary` 特质的实现，使用标题、作者及地点字段来创建 `summaryize` 的返回值。对于 `SocialPost` 结构体，咱们定义 `summarize` 为用户名后跟帖子全文，假设帖子内容已限制为 280 字符。

<a name="listing_10-13"></a>
文件名：`src/lib.rs`

```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format! ("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format! ("{}: {}", self.username, self.content)
    }
}
```

**清单 10-13**：在 `NewsArticle` 和 `SocialPost` 类型上实现 `Summary` 特质

在类型上实现特质与实现常规方法类似。区别在于，在 `impl` 之后，我们放置我们打算实现的特质名字，然后使用 `for` 关键字，然后指定我们打算对其实现特质的类型名字。在 `impl` 代码块内，咱们放置特质定义所定义的方法签名。我们不是在每个签名后添加分号，而是使用花括号并以咱们希望针对这一特定类型，特质方法要有的特定行为来填充方法体。

现在，这个库已对 `NewsArticle` 与 `SocialPost` 实现了 `Summary` 特质，该代码箱的用户可以如同我们调用常规方法一样，在 `NewsArticle` 与 `SocialPost` 实例上调用特质方法。唯一区别是用户必须带入特质以及类型到作用域。下面是二进制代码箱可以怎样使用我们的 `aggregator` 库代码箱的示例：


```rust
use aggregator::{SocialPost, Summary};

fn main() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "当然，跟大家已经知道的一样，朋友们",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 个新帖子: {}", post.summarize());
}
```

这段代码会打印 `1 个新帖子: horse_ebooks: 当然，跟大家已经知道的一样，朋友们`。

> **译注**：项目的 `Cargo.toml` 如下。
>
> ```toml
> [package]
> name = "aggregator"
> version = "0.1.0"
> edition = "2024"
>
> [dependencies]
> ```

依赖于 `aggregator` 代码箱的其他代码箱也可以带入 `Summary` 特质到作用域，以在他们自己的类型上实现 `Summary`。要注意的一个限制是，只有在特质或类型，或二者同时属于我们代码箱本地时，我们才能在类型上实现特质。例如，我们可以对像 `SocialPost` 这样，作为我们的 `aggregator` 代码箱功能一部分的自定义类型，实现像 `Disply` 这样的标准库特质，因为类型 `SocialPost` 属于咱们 `aggregator` 代码箱的本地类型。咱们还可以对我们 `aggregator` 代码箱中的 `Vec<T>` 实现 `Summary`，因为特质 `Summary` 属于我们 `aggregator` 代码箱的本地特质。

但我们不能对外部类型实现外部特质。例如，我们不能在我们的 `aggregator` 代码箱内，对 `Vec<T>` 实现 `Display` 特质，因为 `Display` 和 `Vec<T>` 均被定义在标准库中，而不属于咱们 `aggregator` 代码箱本地。这种限制属于名为 *内聚，coherence* 的属性的一部分，更具体地说是 *孤儿规则，the orphan rule*，如此命名是因为父类型不存在。这条规则确保其他人的代码无法破坏咱们的代码，反之亦然。若没有这条规则，两个代码箱可能为同一类型实现相同的特质，而 Rust 将不知道要使用哪个实现。


## 使用默认实现

有时，为特质中部分或全部方法提供默认行为很有用，而非要求所有类型上的全部方法实现。然后，当咱们对特定类型实现特质时，咱们可以保留或覆盖，override，每个方法的默认行为。

在下面清单 10-14 中，我们为 `Summary` 特质的 `summarize` 方法指定了个默认字符串，而非像在 [清单 10-12](#listing_10-12) 中那样只定义方法签名。

<a name="listing_10-14"></a>
文件名：`src/lib.rs`

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("阅读更多......")
    }
}
```

**清单 10-14**：以 `summarize` 方法的默认实现定义 `Summary` 特质

而要使用默认实现来汇总 `NewsArticle` 的实例，咱们要以 `impl Summary for NewsArticle {}` 指定一个空的 `impl` 代码块。

尽管我们不再直接对 `NewsArticle` 类型定义 `summarize` 方法，但咱们已经提供了默认实现并指定了 `NewsArticle` 实现 `Summary` 特质。因此，我们仍然可以在 `NewsArticle` 实例上调用 `summarize` 方法，如下面这样：

```rust
    let article = NewsArticle {
        headline: String::from("企鹅队赢得斯坦利杯锦标赛！"),
        location: String::from("美国，宾夕法尼亚州，匹兹堡"),
        author: String::from("Iceburgh"),
        content: String::from(
            "匹兹堡企鹅队再度成为美国曲棍球联盟 \
            NHL 中的最佳球队。"
        ),
    };

    println! ("有新文章可读！{}", article.summarize());
```

这段代码会打印 `有新文章可读！阅读更多......`。

创建默认实现不要求我们修改 [清单 10-13](#listing_10-13) 中 `SocialPost` 上 `Summary` 的实现的任何内容。原因是覆盖默认实现的语法，与实现没有默认实现的特质方法的语法相同。

默认实现可以调用同一特质中的其他方法，即使这些别的方法没有默认实现。通过这种方式，特质就可以提供很多有用的功能，并只要求实现者指定其中一小部分。例如，我们可以定义 `Summary` 特质为有个需要实现的 `summarize_author` 方法，然后定义一个有默认实现的 `summarize` 方法，该默认实现会调用 `summarize_author` 方法：

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format! ("（阅读更多来自 {} ......）", self.summarize_author())
    }
}
```

要使用这一版本的 `Summary`，我们只需在类型上实现该特质时定义 `summarize_author` 方法即可：

```rust
impl Summary for SocialPost {
    fn summarize_author(&self) -> String {
        format! ("@{}", self.username)
    }
}
```

定义 `summarize_author` 后，咱们就可以调用 `SocialPost` 结构体实例上的 `summarize` 方法，而 `summarize` 的默认实现将调用我们提供的 `summarize_author` 的定义。因为咱们已经实现了 `summarize_author`，所以 `Summary` 特质已经给予我们 `summarize` 方法的行为，而无需我们编写更多代码。下面是其看起来的样子：

```rust
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "当然，跟大家已经知道的一样，朋友们",
        ),
        reply: false,
        retweet: false,
    };

    println! ("1 个新帖子：{}", post.summarize());
```

这段代码会打印 `1 个新帖子：（阅读更多来自 @horse_ebooks ......）`。

请注意，从方法的重写实现调用同一方法的默认实现是不可行的。


## 将特质用作参数

既然咱们知道了怎样定义和实现特质，我们就可以探讨怎样使用特质，定义接受许多不同类型的函数。我们将使用在 [清单 10-13](#listing_10-13) 中定义在 `NewsArticle` 和 `SocialPost` 的 `Summary` 特质，来定义一个 `notify` 函数，其会调用 `item` 参数上的 `summarize` 方法，该参数是实现 `Summary` 特质的某种类型。为此，我们要使用 `impl Trait` 语法，像下面这样：

```rust
pub fn notify(item: &impl Summary) {
    println! ("突发新闻！{}", item.summarize());
}
```

对于 `item` 参数，咱们指定 `impl` 关键字和特质名字，而不是具体类型。这个参数接受实现指定特质的任何类型。在 `notify` 的函数体中，咱们可以调用 `item` 上 `Summary` 特质中的任何方法，比如 `summarize`。我们可以调用 `notify` 并传入 `NewsArticle` 或 `SocialPost` 的任何实例。以任何其他类型，比如 `String` 或 `i32` 等调用该函数的代码都将不编译，因为这些类型未实现 `Summary`。


### 特质边界语法

`impl Trait` 语法适用于简单情况，但实际上是一种称为 *特质边界，trait bound* 的较长形式的语法糖，syntax sugar；特质绑定看起来像下面这样：


```rust
pub fn notify<T: Summary>(item: &T) {
    println! ("突发新闻！{}", item.summarize());
}
```

这种较长形式等同于上一小节中的示例，但更冗长。咱们把特质边界与冒号之后的泛型类型参数声明放在一起，并放在尖括号内。

`impl Trait` 语法很方便，在简单情形下使代码更简洁，而更完整的特质边界语法可以在其他情况下表达更高的复杂度。例如，我们可能有两个实现 `Summary` 的参数。以 `impl Trait` 语法实现这种情况看起来像下面这样：

```rust
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
```

当我们希望这个函数允许 `item1` 与 `item2` 有着不同类型时（只要两种类型都实现 `Summary` ）时，那么使用 `impl Trait` 是合适的。但当我们打算强制两个参数都要有同一类型时，我们就必须使用特质边界，像下面这样：

```rust
pub fn notify<T: Summary>(item1: &T, item2: &T) {
```

指定为 `item1` 与 `item2` 参数类型的泛型类型 `T` 会约束该函数，从而作为 `item1` 和 `item2` 的实参传递的值的具体类型必须相同。


### `+` 语法下的多个特质边界

咱们还可以指定多个特质边界。假设咱们希望 `notify` 对 `item` 既使用显示格式化，又使用 `summarize`：咱们就要在 `notify` 定义中指定 `item` 必须同时实现 `Disply` 与 `Summary`。我们可以使用 `+` 语法做到这点：

```rust
pub fn notify(item: &(impl Summary + Display)) {
```

`+` 语法同样对泛型类型上的特质边界有效：


```rust
pub fn notify<T: Summary + Display>(item: &T) {
```

指定这两个特质后，`notify` 的函数体就可以 `summarize` 并使用 `{}` 来格式化 `item`。


### `where` 子句下更清楚的特质边界

使用过多的特质边界有其弊端。每个泛型都有自己的特质边界，因此有着多个泛型参数的函数就会在函数名字与参数列表之间，包含大量特质边界信息，从而使函数签名难以阅读。出于这个原因，Rust 有一种替代语法，用于在函数签名后的 `where` 子句内指定特质边界。所以，与其这样写：

```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```

我们可以使用 `where` 子句，像下面这样：

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
```

这个函数的签名就不那么杂乱无章了：函数名字、参数清单和返回值类型紧挨在一起，类似于没有大量特质边界的函数。


## 返回实现特质的类型

我们还可以在返回位置处使用 `impl Trait` 语法，返回一个实现特质的某种类型的值，如下所示：


```rust
fn return_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "当然，正如咱们或许已经知道的一样，朋友们"
        ),
        reply: false,
        retweet: false,
    }
}
```

通过将 `impl Summary` 用于返回类型，咱们指定 `returns_summarizable` 函数某种实现 `Summary` 特质的类型，而无需命名具体类型。在这一情形下，`returns_summarizable` 返回一个 `SocialPost`，但调用这个函数的代码不需要知道这点。

仅通过其实现的特质，指定返回类型的能力在闭包与迭代器的上下文中特别有用，我们会在第 13 章中讨论他们。闭包和迭代器都会创建只有编译器知道的类型，或极长而难于指定的类型。`impl Trait` 语法让咱们可以简洁地指定某种实现 `Iterator` 特质的类型，而无需写出非常长的类型。

然而，只有在返回单个类型时，咱们才能使用 `impl Trait`。比如下面这段在将返回值类型值指定为了 `impl Summary` 下，而要返回 `NewsArticle` 或 `Tweet` 的代码，就不会工作：

```rust
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from("企鹅队赢得斯坦利杯锦标赛！"),
            location: String::from("美国，宾夕法尼亚州，匹兹堡"),
            author: String::from("Iceburgh"),
            content: String::from(
                "匹兹堡企鹅队再度成为美国曲棍球联盟 \
            NHL 中的最佳球队。"
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "当然，跟大家已经清楚的一样了，朋友们",
            ),
            reply: false,
            retweet: false,
        }
    }
}
```

由于编译器中实现 `impl Trait` 语法方式方面的限制，返回 `NewsArticle` 或 `Tweet` 便是不允许的。在第 17 章的 [运用允许不同类型值的特质对象](Ch17_Object_Oriented_Programming_Features_of_Rust.md#使用允许不同类型值的特质对象) 小节，咱们就会降到如何编写有着这种行为的函数。


## 运用特质边界，有条件地实现方法

**Using Trait Bounds to Conditionally Implement Methods**


使用带有用到泛型参数 `impl` 代码块的特质边界，咱们便可根据实现了指定特质的类型，而有条件地实现方法，by using a trait bound with an `impl` block that uses generic type parameters, we can implement methods conditionally for types that implement the specified traits。比如下面清单 10-15 中的类型 `Pair<T>`，就会一直将那个 `new` 函数，实现为返回 `Pair<T>` 的新实例（回顾第 5 章的 [定义方法](Ch05_Using_Structs_to_Structure_Related_Data.md#方法的定义) 小节就知道，`Self` 就是那个 `impl` 代码块的类型别名，此示例中即 `Pair<T>`）。但在接下来的 `impl` 代码块中，若 `Pair<T>` 只在其内部类型 `T` 里，实现启用比较的 `PartialOrd` 特质，*与* 启用打印的 `Display` 特质，那么 `Pair<T>` 就只会实现 `cmp_display` 方法。


```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println! ("极大数为 x = {}", self.x);
        } else {
            println! ("极大数为 y = {}", self.y);
        }
    }
}
```

*清单 10-15：根据特质边界，在泛型上有条件地实现方法，conditionally implementing methods on a generic type depending on trait bounds*

> **注意**：这里的 `new` 是个关联函数，而非方法！只能以 `Pair::new` 形式使用。要作为方法使用，函数就必须要有一个 `self` 参数。

咱们还可对实现了另一特质的任意类型，有条件地实现某个特质。在满足这些特质边界的类型上的特质实现，被称作 *一揽子实现，blanket implementations*，在 Rust 标准库中广泛使用了一揽子实现。比如，标准库就在实现了 `Display` 特质的全部类型上，实现了 `ToString` 特质。标准库中这个 `impl` 代码块，看起来与下面的类似：

```rust
impl<T: Display> ToString for T {
    // --跳过代码--
}
```

由于标准库有着这个一揽子实现，咱们便可在实现了 `Display` 特质的全部类型上，调用由 `ToString` 特质所定义的 `to_string` 方法。比如，由于整数类型实现了 `Display` 特质，那么咱们就可以像下面这样，把整数转换为他们对应的 `String`：

```rust
let s = 3.to_string();
```

一揽子实现，会出现在特质文档的 “相关实现器，Implementors” 部分。


特质与特质边界这两个特性，允许咱们编写出运用泛型参数来减少代码重复的代码，并还向编译器指出了咱们希望该泛型有着特定行为。随后编译器就能使用特质边界信息，来检查代码用到的全部具体类型，是否提供到正确行为。在一般的动态类型语言，dynamically typed languages，中，若调用某个类型上尚未定义的方法，咱们将在运行时收到报错。但 Rust 将这些错误移到了编译时，这样在代码还不能运行的时候，咱们就被强制要求修复这些问题。此外，由于已在编译时被检查过，因此咱们就不必编写检查运行时行为的代码。这样做在提升了性能的同时，不必放弃泛型灵活性。


（End）


