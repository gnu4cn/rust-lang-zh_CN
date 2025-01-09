# 实现一种面向对象设计模式

**Implementing an Object-Oriented Design Pattern**

*状态模式，the state pattern* 属于一种面向对象设计模式。这种模式的核心，便是咱们要定义某个值在其内部可能有的一套各种状态。这些状态是由一套 *状态对象，state objects* 所表示的，同时该值的行为，会根据其状态而改变。接下来咱们会完成有着保存其状态字段，该字段将有着“草稿，draft”、“审阅，review” 或“已发布，published” 三种状态集合的状态对象，的一个博客帖子结构体示例。

状态对象共用着功能：当然，在 Rust 中咱们会使用结构体与特质，而非对象与继承。每个状态对象负责其自己的行为，以及在其应变换为另一状态时自身的治理。保存着状态对象的值，对这些状态的不同行为，或这些状态之间何时变换就毫不知情。

运用状态模式的优势在于，当程序的业务需求改变时，咱们将不需要修改该值保存状态的那些代码，也不需要修改用到该值的那些代码。咱们只需更新某个状态对象内部的那些代码，来改变其规则，或是添加别的一些状态对象。

首先，咱们将要以更传统的面向对象方式，实现这种状态模式，随后咱们将使用对于 Rust 中，更自然一些的方法。下面就来深入到使用状态模式，逐步实现一个博客帖子工作流。

最终功能看起来将像下面这样：


1. 博客帖子以一个空的草稿开始；

2. 在草稿写好后，该帖子就要求审阅一下；

3. 在帖子被批准后，其就会被发布；

4. 只有发布了的帖子，才会返回要打印的内容，因此那些未获批准的帖子就不会被无故发布。


所有别的在帖子上的尝试修改，都应无效。比如，在完成审阅之前，若咱们尝试批准博客帖子草稿，那么该帖子应保持为一个未发布的草稿。

下面清单 17-11 给出了代码形式的这个工作流：此为咱们将在一个名为 `simple_blog` 的库代码箱中，实现的这个 API 的一个示例用法。由于咱们尚未实现该 `simple_blog` 代码箱，因此这段代码尚不会编译。

文件名：`src/main.rs`

```rust
use simple_blog::Post;

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

*清单 17-11：验证咱们打算这个 `simple_blog` 代码箱要有的必要功能的代码*

咱们打算运行用户使用 `Post::new` 创建出一个新的帖子草稿。咱们打算允许将一些文本添加到博客帖子。当咱们在审批之前，尝试立即获取到帖子的内容时，由于该帖子仍为一个草稿，因此咱们就不应得到任何文本。出于验证目的，咱们已在该代码中添加了 `assert_eq!`。而为此目的的良好单元测试，就应断言帖子草稿会从那个 `content` 方法返回空字符串，而咱们并未打算为此示例编写一些测试。

接下来，咱们打算开启该帖子的审阅请求，同时咱们系统在等待审阅期间，`content` 返回一个空字符串。在该帖子得到审批时，他就应得以发布了，表示在 `content` 被调用时，该帖子的文本将被返回。

请注意咱们与 `simple_blog` 代码箱交互的唯一类型，便是 `Post` 这个类型。此类型将用到状态模式，并将保存着将为表示帖子可能状态 -- 草稿、等待审阅或已发布，的三个状态对象之一的一个值。从一种状态改变为另一状态，将在该 `Post` 类型里得以内部管理。这些状态，会因应着库用户在 `Post` 实例上的方法调用而改变，但库用户们却不必直接管理这些状态改变。同样，用户们是无法在这些状态上犯下错误的，比如在帖子未审阅前发布帖子。


## 定义出 `Post` 并创建出一个草稿状态的新实例

**Defining `Post` and Creating a New Instance in the Draft State**


下面就来开始这个库的实现！咱们清楚咱们需要保存着一些内容的一个公开的 `Post` 结构体，因此咱们将以这个结构体的定义，及创建出 `Post` 实例的一个关联的公开 `new` 函数开始，如下清单 17-12 中所示。咱们还将构造出将定义 `Post`的全部状态对象，所必须有的行为的一个私有 `State` 特质。

随后 `Post` 类型将在名为 `state` 的私有字段的 `Option<T>` 值内部，保存 `Box<dyn State>` 类型的一个特质对象，来保存状态对象。过一会儿，咱们就会看到为何那个 `Option<T>` 是必要的。

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

*清单 17-12：`Post` 结构体的定义，以及创建新 `Post` 实例的 `new` 函数、`State` 特质，以及 `Draft` 结构体*

其中的 `State` 特质会定义出由不同帖子状态共用的行为。这些状态对象分别是 `Draft`、`PendingReview` 及 `Published`，同时他们都将实现 `State` 特质。至于现在，这个特质并无任何方法，而由于 `Draft` 状态是咱们想要帖子开始的状态，因此咱们将以仅定义出这个状态开始。

在创建出新的 `Post` 实例时，咱们将其 `state` 自动设置为了保存着一个 `Box` 值的 `Some` 值。这个 `Box` 会只想 `Draft` 结构体的一个新实例。这会确保不能在咱们何时创建出一个 `Post` 的新实例，其都将作为一篇草稿开始。由于 `Post` 的 `state` 字段是私有的，因此就没有办法创建出其他任何状态的一个 `Post`！在 `Post::new` 函数中，咱们把 `content` 字段设置为了一个新的、空 `String`。


## 存储帖子内容文本

**Storing the Text of the Post Content**


在 17-11 中，咱们曾看到咱们希望能调用一个名为 `add_text` 的方法，并传递给他随后被作为博客帖子内容而添加的一个 `&str`。咱们将这实现为一个方法，而不是把 `content` 作为 `pub` 暴露出来，如此稍后咱们就可以实现一个将控制 `content` 字段数据如何被读取的方法。这个 `add_text` 方法是相当直截了当的，那么接下就来在清单 17-13 中，添加这个实现到 `impl Post` 代码块：

文件名：`src/lib.rs`

```rust
impl Post {
    // -- 跳过代码 --

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}
```

*清单 17-13：实现将文本添加到帖子 `content` 字段的 `add_text` 方法*

由于咱们是在修改咱们正于其上调用 `add_text` 的 `Post` 实例，因此这个 `add_text` 方法便取了到 `self` 的可变引用。咱们随后调用了 `content` 字段中 `String` 类型上的 `push_str`，并传递 `text` 参数来添加到那个已保存的 `content`。此行为不依赖帖子所处的状态，因此其并非状态模式的一部分。这个 `add_text` 方法完全不与 `state` 字段交互，但其为咱们打算支持行为的一部分。


## 确保帖子草稿的内容为空

**Ensuring the Content of a Draft Post Is Empty**


即使咱们已调用了 `add_text` 并把一些内容添加到了咱们的帖子，但由于该帖子仍处于草稿状态，故咱们仍想要那个 `content` 方法返回空字符串切片，an empty string slice，正如清单 17-11 中第 7 行所给出的那样。那么现在，就来用将满足此要求的最简单物件，实现这个 `content` 方法：即总是返回一个空字符串切片。稍后一旦咱们实现修改帖子状态的能力，从而帖子可被发布，咱们就会修改这个方法。到目前为止，贴子就只能处于草稿状态，因此帖子内容应始终为空。下面清单 17-14 给出了这种占位的实现：

文件名：`src/lib.rs`

```rust
impl Post {
    // -- 跳过代码 --

    pub fn content(&self) -> &str {
        ""
    }
}
```

*清单 17-14：添加始终返回空字符串切片的 `Post` 上 `content` 方法的一个占位实现，a placeholder implementation*

有了添加的这个 `content` 方法，清单 17-11 中到第 7 行为止的那些代码就都将如预期那样工作了。


## 请求帖子审阅改变其状态

**Requesting a Review of the Post Changes Its State**


接下来，咱们就需要添加请求帖子审阅的功能了，帖子审阅应将其状态从 `Draft` 改变为 `PendingReview`。下面清单 17-15 给出了这样的代码：


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

*清单 17-15：实现 `Post` 与 `State` 特质上的 `request_review` 方法*

咱们给了 `Post` 名为 `request_review` 的一个公开方法，其将取到 `self` 的一个可变引用。随后咱们调用了 `Post` 当前状态上的内部 `request_review` 方法，而这第二个 `request_review` 方法就会消费当前状态并返回一个新的状态。

咱们把那个 `request_review` 方法添加到了 `State` 特质；所有实现了这个特质的类型，现在都将需要实现这个 `request_review` 方法。请注意这里用的不再是 `self`、`&self` 或 `&mut self` 作为该方法的首个参数，这里用的是 `self: Box<Self>`。这样的语法表示，在有在某个保存了该类型的 `Box` 上调用时，这个方法才是有效的。这种语法会取得 `Box<Self>` 的所有权，令到原有状态失效，进而 `Post` 的状态值就可以转换到一种新的状态。

为了消费原有状态，这个 `request_review` 方法就需要取得该状态值的所有权。这便是 `Post` 的那个 `state` 字段中的 `Option` 进入之处：咱们调用了 `take` 方法（属于标准库的 `Option` 类型），来从 `state` 字段取出那个 `Some` 的值，并由于 Rust 不允许咱们在结构体中有无效或空字段，Rust doesn't let us have unpopulated fields in structs，而在 `state` 字段中留下一个 `None`。这样就允许咱们把其中的 `state` 值，迁移出 `Post`而非借用他。随后咱们将把帖子的 `state` 值，设置为此操作的结果。

为了获取到 `state` 值的所有权，咱们就需要暂时将 `state` 设置为 `None`，而非直接使用像是 `self.state = self.state.request_review();` 这样的代码设置他。这样做确保了在咱们已将 `Post` 转换为新状态后，其无法使用原先的 `state` 值。

`Draft` 上的 `request_review` 方法返回的是个新的、新加入的 `PendingReview` 装箱过的实例，其表示了帖子等待审阅时的状态。那个 `PendingReview` 结构体同样实现了 `request_review` 方法，但并未进行任何转换。而是，由于在咱们于某个已处于 `PendingReview` 状态的帖子上，请求审阅时，帖子应保持处于 `PendingReview` 状态，因此 `PendingReview` 的 `request_review` 方法调用返回的是他自己。

现在咱们就可以开始看到状态模式的优势了：不论 `Post` 的 `state` 值为何，其上的 `request_review` 方法都是一样的。每种状态都负责着其自己的那些规则。

咱们将保留 `Post` 上的 `content` 方法如其现在这样，即返回一个空字符串切片。现在咱们就可以让某个 `Post`，处于 `PendingReview` 状态抑或 `Draft` 状态了，不过咱们想要 `PendingReview` 状态中的同样行为。现在清单 17-11 到第 10 行便工作了！


## 添加 `approve` 来修改 `content` 的行为

**Adding `approve` to Change the Behavior of `content`**


`approve` 方法将与 `request_review` 方法类似：他将把 `state` 设置为在帖子状态为 “批准” 时，当前状态所应表明的值，如下清单 17-16 中所示：

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

*清单 17-16：实现 `Post` 与 `State` 特质上的 `approve` 方法*

咱们把这个 `approve` 方法，添加到了 `State` 特质，并添加了一个实现了 `State` 的新结构体，即 `Published` 状态。

与 `PendingReview` 上的 `request_review` 工作方式类似，在咱们调用 `Draft` 上的 `approve` 方法时，由于 `approve` 将返回 `self`，因此他将没有效果。当咱们在 `PendingReview` 上调用 `approve` 时，他返回的是一个新的、装箱过后的 `Published` 结构体实例。这个 `Published` 结构体实现了 `State` 特质，而由于帖子在`request_review` 及 `approve` 两个方法下，都应保持处于 `Published` 状态，因此对于这两个方法，他都会返回他本身。

现在咱们就需要更新 `Post` 上的那个 `content` 方法了。咱们希望从 `content` 返回的值，取决于 `Post` 的当前状态，因此咱们就让 `Post`，委托给定义在其 `state` 上的一个 `content` 方法，如下清单 17-17 中所示：

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

*清单 17-17：将 `Post` 上的 `content` 方法，更新为委托给 `State` 上的 `content` 方法*

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

**Encoding States and Behavior as Types**


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


