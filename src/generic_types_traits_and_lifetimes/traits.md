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

依赖于 `aggregator` 代码箱的其他代码箱，同样可以将 `Summary` 特质带入其作用域，以在他们自己的类型上实现 `Summary`。有个限制条件要注意，即只有在特质或类型二者至少有一个属于代码箱本地的时，咱们才能在类型上实现特质。比如，由于定制类型 `Tweet` 对于咱们的代码箱 `aggregator` 是本地的，因此咱们可以将比如 `Display` 这样的标准库特质，像 `aggregator` 代码箱功能的一部分那样，实现在 `Tweet` 上。由而于那个特质 `Summary` 属于 `aggregator` 代码箱本地，咱们便还可在咱们的 `aggregator` 代码箱中，将其实现在 `Vec<T>` 上。


不过咱们是无法将外部特质，实现在外部类型上的。比如，由于 `Display` 特质与 `Vec<T>` 类型，都是定义在标准库中，而均不属于咱们的 `aggregator` 代码箱，咱们就不能在 `aggregator` 代码箱里头，将 `Display` 特质实现在 `Vec<T>` 上。这种限制属于名为 *内聚，coherrnce* 的属性的一部分，更具体地说，便是 *孤儿规则，the orphan rule*，之所以这样叫法，是由于父类型缺席了，this restriction is part of a property called *coherence*, and more specifically the *orphan rule*, so named because the parent type is not present。这条规则确保了其他人的代码无法破坏咱们代码，反之亦然。若没有这条规则，两个代码箱就会对同样类型实现同一特质，那么 Rust 就不清楚要使用那个实现了。


## 默认实现

**Default Implementions**


给特质中某个或全部方法以默认行为，而非在所有类型上都要求实现全部方法，有的时候会是有用的做法。这样做之后，当咱们在某个特定类型上实现特质时，咱们就可以保留或重写，override，各个方法的默认行为。

下面清单 10-14 就给 `Summary` 特质的 `summarize` 方法，指定了一个默认字符串，而非如同在清单 10-12 中咱们曾做的，只定义出方法签名。

文件名：`src/lib.rs`

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("了解更多......")
    }
}
```

*清单 10-14：定义有着 `summarize` 方法默认实现的 `Summary` 特质*

而要使用默认实现来对 `NewsArticle` 的实例进行摘要，咱们就要以 `impl Summary for NewsArticle {}`，指明一个空的 `impl` 代码块。

尽管不再直接在 `NewsArticle` 类型上定义，那个 `summarize` 方法，但咱们是提供了一个默认实现的，并已指明 `NewsArticle` 类型实现了 `Summary` 特质。由此，咱们就可以在某个 `NewsArticle` 实例上，调用这个 `summarize` 方法，如同下面这样：

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

此代码会打印出 `有新文章可读！了解更多......`。

创建默认实现，不要求咱们对清单 10-13 中，在 `Tweet` 上 `Summary` 的实现，做任何修改。原因是对某个默认实现进行重写的语法，与实现不具有默认实现的特质方法语法相同。

默认实现可调用同一特质中的其他方法，即使那些别的方法没有默认实现。以这种方式，特质就可以提供到很多有用功能，且只要求特质实现者类型，指明其的一小部分方法。比如，咱们就可以将 `Summary` 特质，定义为有着一个要求予以实现的 `summarize_author` 方法，并在随后定义了有着调用了 `summarize_author` 方法默认实现的 `summarize` 方法：

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format! ("（了解更多来自 {} ......）", self.summarize_author())
    }
}
```

而要使用此版本的 `Summary`，咱们只需在某个类型上实现该特质时，定义出 `summarize_author` 方法：

```rust
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format! ("@{}", self.username)
    }
}
```

定义出 `summarize_author` 后，咱们就可以在 `Tweet` 结构体的实例上，调用 `summarize` 方法了，而 `summarize` 的默认实现，将调用咱们所提供的 `summarize_author` 的定义。由于咱们已实现了 `summarize_author`，在不要求咱们编写任何更多代码下，`Summary` 特质就已给到 `summarize` 方法的行为。

```rust
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "当然，跟大家已经清楚的一样了，朋友们",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 条新推文: {}", tweet.summarize());
```

此代码会打印 `1 条新推文: （了解更多来自 @horse_ebooks ......）`。

请注意从方法的重写实现，调用同一方法的默认实现是不可行的。


## 作为参数的特质

**Traits as Parameters**


既然清楚了怎样定义和实现特质，那么咱们就可以探讨一下，怎样运用特质来定义出接收不同类型参数的函数。咱们将使用之前清单 10-13 中，在 `NewsArticle` 与 `Tweet` 上曾实现过的 `Summary` 特质，来定义一个会调用其 `item` 参数上 `summarize` 方法的 `notify` 函数，而该参数便是实现了 `Summary` 特质类型的。要完成这个目的，咱们就要使用 `impl Trait` 语法，如下所示：

```rust
pub fn notify(item: &impl Summary) {
    println! ("突发新闻！{}", item.summarize());
}
```

咱们给那个 `item` 参数指定了 `impl` 关键字和特质名字，而不是具体类型。这个参数会接受实现了指定特质的任何类型。在 `notify` 的函数体中，咱们就可以在 `item` 上，调用来自 `Summary` 特质的任何方法了，比如 `summarize`。咱们可以调用 `notify`，并传入 `NewsArticle` 或 `Tweet` 的任意实例。而以任意其他类型，比如 `String` 或 `i32`，调用该函数的代码，由于那些类型没有实现 `Summary`，就不会编译。


### 特质边界语法

**Trait Bound Syntax**


这种在简单情形下工作的 `impl Trait` 语法，实际上是被称作 *特质边界，trait bound* 的较长形式的语法糖，syntax sugar；其看起来像下面这样：


```rust
pub fn notify<T: Summary>(item: &T) {
    println! ("突发新闻！{}", item.summarize());
}
```

这种较长形式与上一小节中的示例是等价的，但要更冗长一些。咱们把特质边界（`Summary`），在冒号之后，与泛型参数声明放在一起，并在一对尖括号里面。

在简单情形下，`impl Trait` 这种语法是方便的，且令到代码更为简洁，而在别的情形下，较完整的特质边界语法，则能表达出更高复杂度。比如，咱们可以有两个实现 `Summary` 的参数。以 `impl Trait` 语法实现这种情况，看起来就会像下面这样：

```rust
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
```

当咱们是要此函数允许 `item1` 与 `item2` 有着不同类型时（只要两个类型都实现了 `Summary` ），那么使用 `impl Trait` 语法便是恰当的。而当要的是强制这两个参数有着同一类型时，咱们就必须使用特质边界，像下面这样：

```rust
pub fn notify<T: Summary>(item1: &T, item2: &T) {
```

其中被指定为 `item1` 与 `item2` 两个参数类型的泛型 `T`，会对该函数加以约束，进而作为 `item1` 与 `item2` 的实参所传递值的具体类型必须相同。


### 使用 `+` 语法，指定多个特质边界

**Specifying Multiple Trait Bounds with the `+` Syntax**


咱们还可以指明多个特质边界。比方说咱们想要 `notify` 使用 `item` 上的 `summarize` 的同时，还要使用显示格式：咱们就要在 `notify` 定义中，指明 `item` 必须实现了 `Disply` 与 `Summary` 两个特质。使用 `+` 语法，咱们便可达到这个目的：

```rust
pub fn notify(item &(impl Summary + Display)) {
```

`+` 语法同样对泛型上的特质边界有效：


```rust
pub fn notify<T: Summary + Display>(item: &T) {
```

有了指定的这两个特质，那么 `notify` 的函数体，便可调用 `summarize` 函数，及使用 `{}` 来格式化 `item` 了。


#### 使用 `where` 子句，获得更清楚的特质边界

**Clearer Trait Bounds with `where` Clauses**


使用过多的特质边界，有着其一些缺点。每个泛型都有自己的特质边界，那么有着多个泛型参数的函数，在其名字与其参数列表之间，就好包含很多特质边界信息，从而令到该函数签名难于阅读。出于这个原因，Rust 有着在函数签名之后的 `where` 子句里，指明特质边界的这种替代语法。从而与其写出下面这个签名：

```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```

咱们便可像下面这样，使用 `where` 子句：

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
```

这个函数的签名，就不那么杂乱无章了：函数名、参数清单与返回值类型紧挨在一起，类似于与不带有很多特质边界的函数。


## 实现了特质的返回值类型

**Returning Types that Implement Traits**


咱们还也可以在返回值处，使用 `impl Trait` 语法来返回某种实现某个特质类型的值，如下所示：


```rust
fn return_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "当然，如同你或许已经知道的一样，朋友们"
        ),
        reply: false,
        retweet: false,
    }
}
```

通过对返回值类型使用 `impl Summary`，而没有命名具体类型下，咱们便指明了 `returns_summarizable` 函数，会返回实现了 `Summary` 特质的类型。在此示例中，`returns_summarizable` 函数返回的是个 `Tweet`，而调用此函数的代码，则无需知会这一点。

仅以其实现了的特质，便指明了返回值类型这种能力，在闭包与迭代器语境下尤为有用，咱们在第 13 章就会讲到他们。闭包与迭代器会创建出只有编译器清楚的类型，或指定起来极长的类型。`impl Trait` 语法，就允许咱们简明地、在无需编写出极长类型下指定出，返回实现了 `Iterator` 特质的某种类型的函数。

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


