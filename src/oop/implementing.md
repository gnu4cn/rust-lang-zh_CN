# 实现面向对象的设计模式

*状态模式，the state pattern*，属于一种面向对象的设计模式。这种模式的关键在于，我们定义某个值可能在内部具有的一套状态。这些状态由一组 *状态对象，state objects* 表示，而值的行为会根据其状态而变化。我们即将进行一个博客帖子结构体的示例，其有着一个保存其状态的字段，该字段将为 “草稿”、“审阅” 或 “已发布” 三个状态集中的一个状态对象。

状态对象共用功能：当然，在 Rust 中，我们使用结构体与特质，而非对象与继承。每个状态对象都负责自己的行为，并管理何时应转换为另一种状态。保存状态对象的值，对状态的不同行为，或何时进行状态转换一无所知。

使用状态模式的优势在于，当程序的业务需求发生变化时，我们无需修改保存状态的值的代码，或用到该值的代码。我们只需更新某个状态对象内部的代码即可更改其规则，或者添加更多状态对象。

首先，我们将以更传统的面向对象方式实现状态模式，然后，我们将使用一种在 Rust 中更自然的方式。我们来深入研究如何使用状态模式，逐步实现博客帖子的工作流。

最终的功能将看起来像下面这样：

1. 博客帖子作为空白的草稿开始；
2. 草稿完成后，对帖子的审阅是必需的；
3. 帖子被批准后，其得以发布；
4. 只有已发布的博客帖子才会返回用以打印的内容，从而未获批准的帖子不会意外发布。

对帖子尝试的任何其他修改均应无效。例如，当我们在请求审核之前尝试批准博客帖子草稿时，该帖子应保持为未发布的草稿。


## 尝试传统的面向对象风格

为了解决同一个问题，代码的结构方式有无数种，每种都有不同的权衡取舍。这一节的实现更多的是传统的面向对象风格，虽然在 Rust 中可以这样编写，但并未利用 Rust 的某些优势。稍后，我们将演示一种不同的解决方案，他虽然仍采用面向对象的设计模式，但其结构方式对于有面向对象编程经验的开发者而言，可能显得不太熟悉。我们将比较这两种方案，以体验到与其他语言的代码相比，以不同方式设计 Rust 代码的权衡取舍。

下面清单 18-11 以代码形式展示了这一工作流程：这是我们将在名为 `blog` 的库代码箱中实现的 API 的一个使用示例。这还不会编译，因为我们尚未实现 `blog` 代码箱。

<a name="listing_18-11"></a>
文件名：`src/main.rs`

```rust
use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("今天午饭我吃了沙拉。");
    assert_eq! ("", post.content());

    post.request_review();
    assert_eq! ("", post.content());

    post.approve();
    assert_eq! ("今天午饭我吃了沙拉。", post.content());
}
```

**清单 18-11**：演示我们希望 `blog` 代码箱具备的预期行为的代码

我们希望允许用户使用 `Post::new`，创建新的博客帖子草稿。我们打算允许添加文本到博客帖子。当我们尝试在审批之前立即获取帖子的内容时，我们不应得到任何文本，因为该帖子仍然是草稿。出于演示目的，我们在代码中添加了 `assert_eq!`。针对这点的理想单元测试是，断言帖子草稿会从 `content` 方法返回空字符串，但我们不会为这个示例编写测试。

接下来，我们希望启用对帖子的一次审阅请求，并且我们希望在等待审阅期间，`content` 返回空字符串。当帖子获得批准后，他应得以发布，这意味着当 `content` 被调用时，该帖子的正文将被返回。

请注意，我们与该代码箱中交互的唯一类型是 `Post` 类型。这个类型将使用状态模式，并保存一个值，该值将是三个状态对象之一，表示帖子可能出于的不同状态 -- 草稿、等待审阅或已发布。从一种状态到另一状态的更改，将在 `Post` 类型内部得以内部地管理。状态的变化，是响应于库用户对 `Post` 实例调用的方法而发生的，但用户不必直接管理状态变更。此外，用户也不会在状态方面犯错，比如在审阅前发布帖子。


### 定义 `Post` 并创建新实例

我们来开始库的实现！我们知道我们需要一个公开的 `Post` 结构体保存一些内容，因此我们将从该结构体的定义，及用于创建 `Post` 实例的关联公开 `new` 函数开始，如下清单 18-12 中所示。我们还将构造一个私有的 `State` 特质，将定义某个 `Post` 的所有状态对象必须具备的行为。

然后，`Post` 类型将在名为 `state` 的私有字段的 `Option<T>` 值内，保存 `Box<dyn State>` 的特质对象，以保存状态对象。稍后咱们就会明白为何 `Option<T>` 是必要的。

<a name="listing_18-12"></a>
文件名：`src/lib.rs`

```rust
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
}

trait State {}

struct Draft {}

impl State for Draft {}
```

**清单 18-12**：`Post` 结构体的定义，和创建新 `Post` 实例的 `new` 函数、`State` 特质，以及 `Draft` 结构体

其中 `State` 特质定义了不同帖子状态共用的行为。状态对象分别是 `Draft`、`PendingReview` 和 `Published`，他们都将实现 `State` 特质。目前，这一特质没有任何方法，且我们以仅定义 `Draft` 状态开始，因为这是我们的帖子开始的状态。

当我们创建新的 `Post` 实例时，我们设置其 `state` 字段为包含一个 `Box` 值的 `Some` 值。这个 `Box` 值指向一个 `Draft` 结构体的新实例。这确保了每当我们创建一个 `Post` 的新实例时，他都将以草稿形式开始。由于 `Post` 的 `state` 字段是私有的，因此没有办法创建处于任何其他状态的 `Post`！在 `Post::new` 函数中，我们设置 `content` 字段为一个新的、空 `String`。


### 存储帖子内容的文本

我们在清单 18-11 中看到，我们希望能够调用一个名为 `add_text` 的方法，并传递给他一个 `&str`，然后添加为博客帖子的文本内容。我们会作为方法实现这点，而不是暴露 `content` 字段为 `pub`，以便稍后我们可以实现一个方法，其将控制 `content` 字段的数据被读取的方式。`add_text` 方法相当简单，因此我们来添加下面清单 18-13 中的实现到 `impl Post` 代码块。

<a name="listing_18-13"></a>
文件名：`src/lib.rs`

```rust
impl Post {
    // -- 跳过代码 --
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}
```

**清单 18-13**：实现 `add_text` 方法，以添加文本到帖子的 `content` 字段

`add_text` 方法取到 `self` 的可变引用，因为我们正在修改我们正对其调用 `add_text` 的 `Post` 实例。然后，我们对 `content` 字段中的 `String` 值调用 `push_str` 并传递 `text` 参数，以添加到已保存的 `content`。这一行为不依赖于帖子所处的状态，因此他不是状态模式的一部分。`add_text` 方法完全不与 `state` 字段交互，但他是我们希望支持的行为的一部分。


### 确保草稿帖子的内容为空

即使我们调用了 `add_text` 并添加了一些内容到帖子，我们仍然希望 `content` 方法返回一个空的字符串切片，因为帖子仍处于草稿状态，正如清单 18-11 中的第一个 `assert_eq!` 所示。目前，我们来以将满足这一要求的最简单方式实现 `content` 方法：始终返回一个空字符串切片。一旦稍后实现修改帖子的状态以便其可被发布的能力后，我们将修改这个方法。到目前为止，贴子只能处于草稿状态，因此帖子内容应始终为空。下面清单 18-14 展示了这一占位符实现, a placehoder implementation。

<a name="listing_18-14"></a>
文件名：`src/lib.rs`

```rust
impl Post {
    // -- 跳过代码 --
    pub fn content(&self) -> &str {
        ""
    }
}
```

**清单 18-14**：为 `Post` 上 `content` 方法添加占位符实现，其始终返回一个空字符串切片

通过这个添加的 `content` 方法，清单 18-11 中直到第一个 `assert_eq!` 的所有代码都会按预期运行。


### 请求审阅，改变帖子的状态

接下来，我们需要添加请求对帖子审阅的功能，这应该将其状态从 `Draft` 更改为 `PendingReview`。下面清单 18-15 展示了这一代码。

<a name="listing_18-15"></a>
文件名：`src/lib.rs`

```rust
impl Post {
    // -- 跳过代码 --
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
```

**清单 18-15**：实现 `Post` 与 `State` 特质上的 `request_review` 方法

我们给予 `Post` 一个名为 `request_review` 的公开方法，其将取一个到 `self` 的可变引用。然后，我们对 `Post` 的当前状态调用内部的 `request_review` 方法，而这第二个 `request_review` 方法会消费当前状态并返回一个新状态。

我们添加 `request_review` 方法到 `State` 特质；所有实现这个特质的类型，现在都将需要实现 `request_review` 方法。请注意，我们没有使用 `self`、`&self` 或 `&mut self` 作为这个方法的第一个参数，而是使用 `self: Box<Self>`。这种语法意味着，这个方法仅在对包含这种类型的 `Box` 调用时才有效。这种语法会取得 `Box<Self>` 的所有权，从而使旧状态失效，以便 `Post` 的状态值可以转换为新状态。

为了消费旧的状态，`request_review` 方法需要取得状态值的所有权。这就是 `Post` 的 `state` 字段中 `Option` 发挥作用的地方：我们调用 `take` 方法，来从 `state` 字段取出 `Some` 值，并在其位置留下一个 `None`，因为 Rust 不允许我们在结构体中有着未填充的（无效或空字段) 字段。这让我们可以从 `Post` 中迁出 `state` 值，而不是借用他。然后，我们将设置帖子的 `state` 值为这一操作的结果。

为了获的 `state` 值的所有权，我们需要暂时设置 `state` 为 `None`，而不是以 `self.state = self.state.request_review();` 这样的代码直接设置他。这确保了在我们将 `Post` 转换为新的状态后，`Post` 无法再使用旧的 `state` 值。

`Draft` 上的 `request_review` 方法返回一个新的、装箱后的新 `PendingReview` 结构体实例，表示帖子等待审阅时的状态。`PendingReview` 结构体也实现了 `request_review` 方法，但不执行任何转换。相反，他会返回自身，因为当我们对已处于 `PendingReview` 状态的帖子请求审阅时，他应保持处于 `PendingReview` 状态。

现在我们可以开始看到状态模式的优势了：不论 `Post` 的 `state` 值为何，其上的 `request_review` 方法都一样。每种状态都负责自己的规则。

我们将保持 `Post` 上的 `content` 方法不变，使其返回一个空字符串切片。我们现在既可以让 `Post` 处于 `PendingReview` 状态，也可以处于 `Draft` 状态，但我们想要 `PendingReview` 状态下的同样行为。现在清单 18-11 再第二个 `assert_eq!` 前都可以正常工作了！


## 添加 `approve` 以修改 `content` 的行为

`approve` 方法将类似于 `request_review` 方法：他将设置 `state` 为当前状态规定的，再状态为 “批准” 时应具有的值，如下清单 18-16 中所示。

<a name="listing_18-16"></a>
文件名：`src/lib.rs`

```rust
impl Post {
    // -- 跳过代码 --
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    // -- 跳过代码 --

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    // -- 跳过代码 --

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
```

**清单 18-16**：实现 `Post` 与 `State` 特质上的 `approve` 方法

我们添加 `approve` 方法到 `State` 特质，并添加一个实现 `State` 的新结构体，即 `Published` 状态。

与 `PendingReview` 上 `request_review` 的工作方式类似，当我们对 `Draft` 调用 `approve` 方法时，他将不会产生效果，因为 `approve` 将返回 `self`。在我们对 `PendingReview` 调用 `approve` 时，他返回一个新的、装箱后的 `Published` 结构体实例。`Published` 结构体实现了 `State` 特质，而对于 `request_review` 及 `approve` 这两个方法，他都会返回自身，因为在这些情形下，帖子都应保持处于 `Published` 状态。

现在我们需要更新 `Post` 上的 `content` 方法。我们希望 `content` 返回的值取决于 `Post` 的当前状态，因此我们即将让 `Post` 委托给定义在其 `state` 上的 `content` 方法，如下清单 18-17 中所示。

<a name="listing_18-17"></a>
文件名：`src/lib.rs`

```rust
impl Post {
    // -- 跳过代码 --
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
    // -- 跳过代码 --
}
```

**清单 18-17**：更新 `Post` 上的 `content` 方法为，委托给 `State` 上的 `content` 方法

> **译注**：这里涉及到一个 “委托”、“委派” 的概念，updating the `content` on `Post` to delegate to a `content` medthod on `State`

由于咱们的目标是要把所有规则，都保持于实现了 `State` 的那些结构体中，因此咱们就要调用 `state` 字段中值上的 `content` 方法，并将帖子实例（那就是 `self`）作为参数加以传递。随后咱们要返回从 `state` 值上的 `content` 方法调用所返回的值。

由于咱们要的是到 `Option<T>` 内部值的一个引用，而非该值的所有权，因此咱们调用了 `Option<T>` 上的 `as_ref` 方法。由于 `state` 是个 `Option<Box<dyn State>>`，在咱们调用 `as_ref` 时，就会返回一个 `Option<&Box<dyn State>>`。而若咱们没有调用 `as_ref`，那么由于咱们无法无法把 `state` 迁移出那个借用的函数参数 `&self`，而将得到一个报错。

咱们随后调用了 `unwrap` 方法（标准库 `Option<T>` 类型上的），由于咱们清楚，`Post` 上的那些方法，会确保 `state` 将在这些方法完成时，始终包含某个 `Some` 值，因此咱们就明白，这个`unwrap` 是绝不会终止运行的。这便是第 9 章 [相比与编译器咱们掌握着更多信息的情形](Ch09_Error_Handling.md#相比于编译器代码编写者掌握了更多信息的情形) 小节所讲到的情形之一：即咱们明白某个 `Option<T>` 不可能是个 `None` 值，尽管编译器无法掌握这一点。

到 `unwrap` 方法这里，当咱们在 `&Box<dyn State>` 上调用 `content` 方法时，强制解引用转换，deref coercion 就会在那个 `&` 及 `Box` 上发挥作用，从而 `content` 方法就将在实现了 `State` 特质的类型上，最终被调用到。而那就意味着咱们需要把 `content` 添加到 `State` 特质的定义，而那正是咱们把根据咱们所有的状态，返回什么样的内容，这种逻辑要放入的地方，如下清单 17-18 中所示：

文件名：`src/lib.rs`

```rust
trait State {
    // -- 跳过代码 --
    fn content<'a>(&self, post: &'a Post) -> &'a str { "" }
}

// -- 跳过代码 --
struct Published {}

impl State for Published {
    // -- 跳过代码 --

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
```

*清单 17-18：把 `content` 方法添加到 `State` 特质*

咱们添加了返回空字符串切片的 `content` 方法默认实现。那就意味着咱们无需在 `Draft` 与 `PendingRereview` 两个结构体上实现 `content` 方法。而 `Published` 结构体则将重写这个 `content` 方法，并返回 `post.content` 中的值。

> **注**：由于 `content` 默认实现返回的是 `""` 空字符串切片，是个已知大小的值，故方才可以写默认实现。而若将 `request_review` 或 `approve` 也写为默认实现，即如下面这样：

```rust
trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State> { self }
    fn approve(self: Box<Self>) -> Box<dyn State> { self }
    fn content<'a>(&self, post: &'a Post) -> &'a str { "" }
}
```
>
> 那么将报出错误：

```console
$ cargo run                                                                                     ✔  
   Compiling simple_blog v0.1.0 (/home/peng/rust-lang/simple_blog)
error[E0277]: the size for values of type `Self` cannot be known at compilation time
  --> src/lib.rs:40:53
   |
40 |     fn approve(self: Box<Self>) -> Box<dyn State> { self }
   |                                                     ^^^^ doesn't have a size known at compile-time
   |
   = note: required for the cast from `Self` to the object type `dyn State`
help: consider further restricting `Self`
   |
40 |     fn approve(self: Box<Self>) -> Box<dyn State> where Self: Sized { self }
   |                                                   +++++++++++++++++

For more information about this error, try `rustc --explain E0277`.
error: could not compile `simple_blog` due to previous error
```
>
> 这表示 Rust 中的默认实现，需要返回值为固定大小。

请注意正如咱们曾在第 10 章中讨论过的那样，在这个方法上咱们需要生命周期注解。咱们取了到某个 `post` 的引用作为参数，并返回的是到那个 `post` 一部分的引用，因此所返回引用的生命周期，便于这个 `post` 参数生命周期相关。

而咱们就完成了 -- 清单 17-11 的全部代码现在便工作了！咱们就已实现了博客帖子工作流规则，the rules of the blog post workflow 下的状态模式。与那些规则相关的逻辑，是存在与这些状态对象中，而非散落于 `Post` 的各处，the logic related to the rules lives in the state objects rather than being scattered throughout `Post`。

> 为何不用枚举，Why Not An Enum？
>
> 你可能已经想到，为何咱们没有使用将不同帖子状态作为变种的一个 `enum`。那确实是一种可行的办法，请尝试并比较最后的结果，来发现你要选哪个方案！运用枚举的一个不足之处，便是在每个检查枚举值的地方，将都需有一个 `match` 表达式，或类似的东西来处理每种可能的变种。相比这里的特质对象方法，那就会有更多重复代码。


## 状态模式的取舍

**Trade-offs of the State Pattern**


咱们已经证明，Rust 是能够实现这种面向对象模式，来封装处于不同状态下帖子应具备的各种不同行为。`Post` 上的方法对这些各种行为毫不知情。咱们组织代码的方式，即咱们必须仅在一处查看，而获悉某个已发布帖子可以有的那些不同行为方式：这便是 `Published` 结构体上 `State` 特质的实现。

若咱们原本打算创建另一种不使用状态模式的替代实现，那么咱们可能就会在 `Post` 上的那些方法中，使用一些检查帖子状态的 `match` 表达式，并在那些 `match` 表达式处改变行为。那就意味着咱们将不得不查看多个地方，来了解某个处于已发布状态帖子的全部影响！这样做只会徒增咱们所添加的更多一些状态：每个的这些 `match` 表达式，都将需要另一支臂。

而在状态模式下，那些 `Post` 方法以及那些咱们用到 `Post` 的各处，就不需要那些 `match` 表达式，而要添加一个新状态，咱们将只需添加一个新结构体，并在那个结构体上实现那些特质方法即可。

使用状态模式的这种实现，易于添加更多功能。为发现使用状态模式维护代码的简单性，请尝试下面几条建议：

- 请添加将帖子状态从 `PendingReview` 改回到 `Draft` 的一个 `reject` 方法；

- 在状态可被改变为 `Published` 之前，要求两次到 `approve` 的调用；

- 只有在某个帖子处于 `Draft` 状态时，才允许用户添加文本内容。提示：让状态对象负责那些可能修改内容的操作，而不负责修改 `Post` 的操作。


状态模式的一个缺点则是，由于这些状态都实现那些状态间的转换，那么其中一些状态就相互耦合了。当咱们在 `PendingReview` 于 `Published` 之间，添加另一状态，比如 `Scheduled` 时，咱们将不得不把 `PendingReview` 中的代码，修改为相应地转换到 `Scheduled`。若在新状态的添加下，`PendingReview` 无需修改，那么就会少一些事情，然而那便意味着转换到另一种涉及模式了。

至于另一个缺点，便是咱们重复了一些逻辑。为消除一些重复，咱们就可能会尝试构造 `State` 特质上，返回 `self` 的 `request_review` 于 `approve` 两个方法的默认实现；然而，由于该特质不清楚那个具体的 `self` 将为何物，因此这会违反对象安全性，violate object safety。咱们希望能够将 `State` 作为特质对象使用，因此咱们就需要他的那些方法是对象安全的。

其他代码重复包括了 `Post` 上 `request_review` 与 `approve` 两个方法的一些相似实现。这两个方法都委托给了那个 `Option` 的 `state` 字段中值上的同一方法，并将 `state` 字段的值，设置到方法的结果。若咱们在 `Post` 上有着大量的遵循这种模式的方法，咱们就会考虑定义出一个宏，defining a macro，来消除这种重复（请参阅第 19 章中 ["宏，Macros"](Ch19_Advanced_Features.md#关于宏) 小节）。

经由这种完全按照面向对象模式下所定义的状态模式，来实现这种模式，咱们就没有利用上原本所能利用的 Rust 的全部优势。下面就来看看，咱们可对那个 `simple_blog` 能做出的，可将无效状态与无效状态转换，构造为编译时错误的一些改变。


### 将状态与行为编码为类型

咱们将给出如何对这种状态模式加以反思，以得到一套不同的权衡取舍。不同于对状态及状态的转换进行完全地封装，进而外部代码对他们一无所知，咱们将把那些状态编码为不同类型。于是乎，Rust 的类型检查系统，就将通过发出编译器报错，阻止在那些仅允许已发布帖子之处，使用草稿帖子的尝试。

下面来考虑一下清单 17-11 中，`main` 函数的第一部分：

文件名：`src/main.rs`

```rust
fn main() {
    let mut post = Post::new();

    post.add_text("今天午饭我吃了沙拉。");
    assert_eq! ("", post.content());
}
```

咱们仍旧使用 `Post::new`，实现了新的处于草稿状态的那些帖子的创建，并实现了将文本添加到帖子内容的能力。但与在草稿帖子上有着返回空字符串的 `content` 方法不同，咱们将把 `Post` 构造为根本就没有那个 `content` 方法。那样的话，在咱们尝试获取某个草稿帖子的内容时，就会得到告诉咱们该方法不存在的编译器报错。由此，对于生产中咱们无意地显示出帖子内容，由于那样的代码甚至都不会编译，那么这将是不可能的了。清单 17-19 给出了 `Post` 结构体的定义，以及一个 `DraftPost` 的结构体，以及各自上的一些方法：


文件名：`src/lib.rs`

```rust
pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}
```

*清单 17-19：有着 `content` 方法的 `Post` 与不带 `content` 方法的 `DraftPost`*


`Post` 与 `DraftPost` 两个结构体都有着存储了博客帖子文本的私有 `content` 字段。由于咱们正将状态编码，迁移到一些结构体类型，因此这两个结构体就不再有 `state` 字段了。`Post` 结构体将表示已发布帖子，而他便有着返回 `content` 的 `content` 方法。

咱们仍有一个 `Post::new` 函数，但不是返回 `Post` 实例，其返回的是 `DraftPost` 实例。由于 `content` 是私有的，而有没有任何返回 `Post` 的函数，那么此刻就不可能创建出 `Post` 实例。

`DraftPost` 结构体有着一个 `add_text` 方法，因此咱们就可以如同之前那样，把文本添加到 `content`，但请注意 `DraftPost` 并没有定义一个 `content` 方法！因此现在的程序确保了全部帖子都以草稿帖子开头，而草稿帖子并不会让他们的内容用于显示。任何绕过这些约束的尝试，都将导致编译器报错。


### 以到不同类型的转换，实现（状态的）转换

**Implementing Transitions as Transformations into Different Types**


那么怎样来获取到某个已发布帖子呢？咱们是打算强化某个草稿帖子在其可被发布之前，必须被审阅和批准的规则。处于等待审阅状态的帖子，应仍然不显示任何内容。下面酒类通过添加另一结构体，`PendingReviewPost`、在 `DraftPost` 上定义出返回 `PendingReviewPost` 实例的 `request_review` 方法，以及在 `PendingReviewPost` 上定义出返回 `Post` 的 `approve` 方法，实现这些约束，如下清单 17-20 中所示：

文件名：`src/lib.rs`

```rust
impl DraftPost {
    // -- 跳过代码 --

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}
```

*清单 17-20：通过在 `DraftPost` 上调用 `request_review` 而被创建出的 `PendingReviewPost` 实例，以及将 `PendingReviewPost` 转变为已发布 `Post` 的 `approve` 方法*

`request_review` 与 `approve` 两个方法，都取得了 `self` 的所有权，从而消费了 `DraftPost` 及 `PendingReviewPost` 实例，并把他们相应地转换成了 `PendingReviewPost` 与已发布的 `Post`。以这种方式，在咱们于 `DraftPost` 实例上调用了 `request_review` 之后，便不会再有任何遗存的 `DraftPost` 实例，对 `PendingReviewPost` 之类亦是如此。`PendingReviewPost` 结构体之上，并没有 `content` 方法，因此正如 `DraftPost` 一样，尝试读取其内容，会导致编译器报错。由于获取确实有定义出的 `content` 方法的已发布 `Post` 实例的唯一方式，为在某个 `PendingReviewPost` 上调用 `approve` 方法，而获取到一个 `PendingReviewPost` 的唯一方法，为在某个 `DraftPost` 上调用 `request_review` 方法，咱们现在便已将这个博客帖子工作流，编码为了类型系统。

不过咱们还必须对 `main` 做出一些小的修改。`request_review` 与 `approve` 两个方法，返回的都是一些新实例，而不再是修改他们于其上所调用的结构，因此咱们就需要添加更多 `let post = ` 遮蔽赋值语句，来保存那些返回的实例。咱们还不能断言草稿于等待审阅帖子的内容为空字符串，咱们也是不需要他们的：咱们再也不会编译，尝试使用处于这些状态下的帖子内容的代码。下面清单 17-21 中，给出了 `main` 中更新后的代码：

文件名：`src/main.rs`

```rust
use neo_simple_blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("这是一个博客帖子。");

    let post = post.request_review();
    let post = post.approve();

    assert_eq! ("这是一个博客帖子。", post.content());
}
```

*清单 17-21：为使用这个博客帖子工作流的新实现，而对 `main` 的一些修改*


咱们所需做出的对 `post` 重新复制的那些修改，表示这种实现，已不再那么严格遵循面向对象设计模式了：状态之间的转换，不再是整个地封装在 `Post` 实现里。不过，咱们的收获，则是由于类型系统，以及编译时所发生的类型检查，那些无效状态现在就不可能了！这样就确保了一些确切代码错误，比如未发布帖子内容的显示等，在其到达生产部署之前，就将被发现。

请在清单 17-21 之后的情况下，尝试在 `neo_simple_blog` 上实现本小节开头给出的那些任务，来发现你对此版本代码的设计模式有何看法。请注意其中一些任务，在这种模式下或许已被完成了。


咱们业已看到，即使 Rust 有能力实现面向对象的一些设计模式，而对于其他模式，比如将状态编码为类型系统等，在 Rust 中也都是可行的。这些模式都有着不同的取舍。尽管咱们可能对面向对象的那些模式非常熟悉，但对问题进行反思，而运用上 Rust 那些特性的优势，就可以提供到各种好处，比如在编译时阻止一些代码错误等。出于比如所有权这样的，面向对象语言所不具备的某些特性，那么在 Rust 中，面向对象的那些模式，将并不总是最佳方案。


# 本章小节

在读完这一章之后，不论咱们认为或是不认为 Rust 是门面向对象语言，现在都明白，咱们可以在 Rust 中，使用特质对象来获得一些面向对象的特性。动态调遣，dynamic dispatch 可以些许运行时性能损失，换取到咱们代码一定程度的灵活性。咱们则可运用这样的灵活性，来实现能有助于代码可维护性的一些面向对象模式。Rust 还有面向对象语言所没有的其他一些特性，比如所有权等。对于利用 Rust 各种长处方面的优势来讲，面向对象模式将不总是最佳方式，但其为一种可行选项。

接下来，咱们将看看各种模式，这是带来大量灵活性的 Rust 诸多特性的另一项。虽然贯穿本书，咱们已经粗略地看了看他们，但仍尚未见识到他们的完整能力。咱们就拭目以待吧！


（End）


