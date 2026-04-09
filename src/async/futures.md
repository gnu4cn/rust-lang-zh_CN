# 未来值与异步语法

Rust 中异步编程的关键要素是 *未来值* 和 Rust 的 `async` 与 `await` 关键字。

所谓 *未来值*，是现在可能尚未准备就绪，但会在未来某个时刻准备好的值。(同样的概念在许多编成语言中都有体现，有时会以其他名称出现，比如 *任务，task* 或 *承诺，promise* 等）。Rust 提供了 `Future` 特质作为构建块，以便不同的异步操作可以虽然不同，却带有共同的接口的数据结构实现。在 Rust 中，未来值属于实现 `Future` 特质的类型。每个未来值都包含自己的有关已完成进度的信息，以及 “就绪” 的含义。

咱们可以应用 `async` 关键字到代码块和函数，以指定他们可以被中断和恢复。在异步代码块或异步函数内部，咱们可以使用 `await` 关键字来 *等待某个未来值*（即等待其就绪）。在异步代码块或函数内部，任何咱们等待未来值的位置，都是该代码块或函数暂停及恢复的潜在位置。检查未来值以了解其值是否已可用的过程，称为 *轮询*。

一些别的语言，比如 C# 和 JavaScript，也将 `async` 和 `await` 关键字用于异步编程。若咱们熟悉这些语言，就会注意到 Rust 在处理这种语法的方式上的一些显著差异。正如我们将看到的，这自有其道理！

在编写异步 Rust 时，我们大多数时候都会使用 `async` 与 `await` 关键字。Rust 会将二者编译为使用 `Future` 特质的等效代码，就像他将 `for` 循环编译为使用 `Iterator` 特质的等效代码一样。不过，由于 Rust 提供了 `Future` 特质，因此在需要时，咱们也可以给自己的数据类型实现该特质。我们将在这一章中看到的许多函数，都会返回带有他们自己的 `Future` 实现的类型。我们将在本章最后回到这一特质的定义，并深入探讨其工作原理，但这已经足够让我们继续前进了。

这可能感觉有点抽象，所以我们来编写第一个异步程序：一个小的 web 爬虫。我们将从命令行传入两个 URL，并发地获取这两个 URL，并返回先完成的那个的结果。这个示例将有不少新语法，但不用担心 -- 我们将在过程中解释咱们需要了解的一切。


# 我们的第一个异步程序

为了将这一章的重点放在学习异步而不是处理该生态的各个部分上，我们创建了 `trpl` 代码箱（`trpl` 是 “The Rust Programming Language” 的缩写）。他重导出了咱们需要的所有类型、特质及函数，他们主要来自 `futures` 和 `tokio` 代码箱。`futures` 代码箱是 Rust 异步代码实验的正式场所，并且他实际上是 `Future` 特质的最初设计地。[Tokio](https://tokio.rs/) 是如今 Rust 中使用最广泛的异步运行时，特别是对于 web 应用。当然还有其他很棒的运行时，而他们可能更适合咱们的目的。我们之所以在 `trpl` 的底层使用 `tokio` 代码箱，是因为他经过充分测试且被广泛使用。

在某些情况下，`trpl` 还重命名或封装了原始 API，以让咱们专注于与这一章相关的细节。若咱们想了解这个代码箱的用途，建议查看其 [源代码](https://github.com/rust-lang/book/tree/main/packages/trpl)。咱们将能能够到每个重导出来自哪个代码箱，而且我们留下了大量注释，解释这个代码箱的作用。


请创建一个名为 `hello-async` 的新二进制项目，并作为依赖项添加 `trpl` 代码箱：


```console
cargo new hello-async
cd hello-async
cargo add trpl
```

现在我们可以使用 `trpl` 提供的各个部分来编写我们的第一个异步程序了。我们将构建出一个小型命令行工具，他将获取两个 web 页面，提取每个页面中的 `<title>` 元素，并打印出首先完成整个过程的页面的标题。


## 定义 `page_title` 函数

我们以编写一个函数开始，他将取一个页面的 URL 作为参数，向其发出请求，并返回 `<title>` 元素的文本（见清单 17-1）。

<a name="listing_17-1"></a>
文件名：`src/main.rs`

```rust
use trpl::Html;

async fn page_title(url: &str) -> Option<String> {
    let response = trpl::get(url).await;
    let response_text = response.text().await;
    Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html())
}
```

**清单 17-1**：定义一个异步函数，以获取某个 HTML 页面中的标题元素

首先，我们定义一个名为 `page_title` 的函数，并以 `async` 关键字标记他。然后，我们使用 `trpl::get` 函数获取传入的任何 URL，并添加 `await` 关键字来等待响应。为了获取 `response` 的文本，我们调用其 `text` 方法，并再次以 `await` 关键字等待他。这两个步骤都是异步的。对于 `get` 函数，我们必须等待服务器发回其响应的第一部分，这部分将包含 HTTP 头部、cookie 等等，并且这部分可与响应正文分开投送。特别是当响应正文非常大时，其全部到达就会需要一些时间。因为我们必须等待响应的 *全部内容* 到达，所以 `text` 方法也是异步的。

我们就必须显式地等待这两个未来值，因为 Rust 中的未来值是 *懒惰的，lazy*：除非咱们以 `await` 关键字要求他们，否则他们不会执行任何操作（事实上，当咱们未使用某个未来值时，Rust 将显示编译器告警。）这可能会让咱们想起第 13 章中 [以迭代器处理一系列项目](../functional_features/iterators.md) 小节中对迭代器的讨论。除非咱们调用迭代器的 `next` 方法，否则迭代器不会执行任何操作 -- 无论是直接调用还是通过使用 `for` 循环，或比如在底层使用 `next` 的 `map` 方法等。同样，除非咱们显式地要求他们，未来值也不会执行任何操作。这种 “懒惰” 使得 Rust 可以在实际需要之前，避免运行异步代码。


> **注意**：这不同于我们在第 16 章中 [“以 `spawn` 创建新线程”](../concurrency/threads.md#以-spawn-创建新线程) 小节中，使用 `thread::spawn` 时看到的行为，当时我们传递给另一线程的闭包立即开始了运行。这也不同于许多其他语言处理异步的方式。但这对 Rust 很重要能够提供其性能保证至关重要，正如迭代器的情况一样。

一旦有了 `response_text`，我们就可以使用 `Html::parse`，将其解析为 `Html` 类型的实例。我们现在有了一种数据类型而不是原始字符串，可以作为一种更丰富的数据结构处理 HTML。特别是，我们可以使用 `select_first` 方法，找到给定 CSS 选择器的首个实例。通过传递字符串 `"title"`，我们将获取文档中的第一个 `<title>` 元素，当存在一个时。由于可能不存在任何匹配的元素，`select_first` 会返回一个 `Option<ElementRef>`。最后，我们使用 `Option::map` 方法，当 `Option` 中的项目存在时，该方法允许我们处理项目，并在不存在时什么也不做。(我们也可以在这里使用 `match` 表达式，但 `map` 更符合惯例。）在我们提供给 `map` 的函数体中，我们对 `title` 调用 `inner_html` 以获取他的内容，这是个 `String`。总而言之，我们有个 `Option<String>`。

请注意，Rust 的 `await` 关键字位于咱们正在等待的表达式 *之后*，而不是之前。也就是说，他是个 *后缀* 关键字，a *postfix* keyword。若咱们在别的语言中使用过 `async`，这可能与咱们习惯的有所不同，但在 Rust 中，他使方法链，chains of methods，更容易使用。因此，我们可以修改 `page_title` 的主体为通过 `trpl::get` 和 `text` 函数调用之间的 `await`，链接二者在一起，如下清单 17-2 中所示。

<a name="listing_17-2"></a>
文件名：`src/main.rs`


```rust
    let response_text = trpl::get(url).await.text().await;
```

**清单 17-2**：`await` 关键字下的链式调用

至此，我们已成功编写了第一个异步函数！在我们于 `main` 中添加一些代码调用他前，我们先进一步探讨一下我们编写的内容及其含义。

当 Rust 看到标有 `async` 关键字标记的 *代码块* 时，会将其编译为一种独特的，实现 `Future` 特质的匿名数据类型。当 Rust 看到标有 `async` 的 *函数* 时，会将其编译为一个非异步函数，其主体即为一个异步代码块。异步函数的返回，即为编译器为该异步代码块创建的匿名数据类型。

因此，写下 `async fn` 等同于编写一个返回 *未来值* 的返回类型的函数。对于编译器而言，诸如清单 17-1 中的 `async fn page_title` 的函数定义，大致等同于如下定义的非异步函数：


```rust
use std::future::Future;
use trpl::Html;

fn page_title(url: &str) -> impl Future<Output = Option<String>> {
    async move {
        let text = trpl::get(url).await.text().await;
        Html::parse(&text)
            .select_first("title")
            .map(|title| title.inner_html())
    }
}
```


我们来逐一分析转换后版本的各个部分：


- 他使用了我们在第 10 章中 [将特质用作参数](../generic_types_traits_and_lifetimes/traits.md#将特质用作参数) 小节中讨论过的 `impl Trait` 语法；
- 返回的值以关联类型 `Outpu` 实现 `Future`。请注意，`Output` 类型为 `Option<String>`，这与 `async fn` 版本的 `page_title` 的原始返回值类型相同；
- 原始函数主体中调用的所有代码都封装在 `async move` 代码块中。请记住，代码块都属于表达式。这一整个代码块便是从该函数返回的表达式；
- 如上所述，这个异步代码块会生成一个 `Option<String>` 类型的值。该值与返回类型中的 `Output` 类型匹配。这就像咱们见过的其他代码块一样；
- 由于其使用 `url` 参数的方式，新的函数体是个 `async move` 代码块。(我们将在本章稍后详细讨论 `async` 与 `async move` 的区别。）
- 该函数的新版本，在输出类型中有种我们以前从未见过的生命周期：`'_`。由于该函数返回了一个指向某个引用的未来值 -- 本例中，引用来自 `url` 参数 -- 因此我们就需要告诉 Rust，我们希望该引用要被包含。在这里，我们不必命名这个生命周期，因为 Rust 足够聪明，知道只有一个可能涉及到的引用，但我们 *确实* 必须明确指出，得到的那个未来值受该生命周期的约束。


现在我们可以在 `main` 中调用 `page_title`。


## 通过运行时执行异步函数

首先，我们将获取单个页面的标题，如下清单 17-3 中所示。不幸的是，这段代码还不会编译。

<a name="listing_17-3"></a>
文件名：`src/main.rs`

```rust
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let url = &args[1];
    match page_title(url).await {
        Some(title) => println!("{url} 的标题是 {title}"),
        None => println!("{url} 没有标题"),
    }
}
```

**清单 17-3**：以一个用户提供的参数，在 `main` 中调用 `page_title` 函数

我们沿用了我们在第 12 章 [接受命令行参数](../io_project/accepting_cli_arguments.md) 小节中，用于获取命令行参数的同一模式。然后，我们传递 URL 参数给 `page_title` 并等待结果。由于未来值生成的值是个 `Option<String>`，因此我们使用 `match` 表达式来打印不同的信息，以区分页面是否有 `<title>`。

我们唯一可以使用 `await` 关键字的地方，是在异步函数或异步代码块中，并且 Rust 不允许我们标记特殊的 `main` 函数为 `async`。

```console
error[E0752]: `main` function is not allowed to be `async`
 --> src/main.rs:3:1
  |
3 | async fn main() {
  | ^^^^^^^^^^^^^^^ `main` function is not allowed to be `async`
```

`main` 不能标记为 `async` 的原因，是异步代码需要 *运行时*：管理执行异步代码细节的某个 Rust 代码箱。程序的 `main` 函数可以 *初始化* 运行时，但 *他本身* 并不是运行时。（稍后我们将详细了解为什么会出现这种情况。）每个执行异步代码的 Rust 程序，都至少有一处，设置执行未来值的运行时的地方。

大多数支持异步的语言都捆绑了运行时，但 Rust 没有。相反，存在许多可用的不同异步运行时，每种都针对其目标用例做出了不同的恰当取舍。例如，有着众多 CPU 核心及大量 RAM 的高吞吐量 web 服务器，与仅有单个核心、少量 RAM 且不具备堆分配能力的微控制器 MCU 相比，就有着截然不同的需求。提供这些运行时的代码箱，通常还会提供常用功能的异步版本，比如文件或网络 I/O 等。

在这里，以及本章的其余部分中，我们将使用 `trpl` 代码中的 `block_on` 函数，他会取一个未来值作为参数，并阻塞当前线程，知道该未来值运行完成。在幕后，调用 `block_on` 会使用 `tokio` 设置运行时，用于运行传入的未来值（`trpl` 代码箱的 `block_on` 的行为类似于其他代码箱的 `block_on` 函数）。一旦该未来值完成，`block_on` 就会返回该未来值产生的任何值。

我们可以直接传递 `page_title` 返回的未来值给 `block_on`，并在其完成后，就可以像在清单 17-3 中尝试的那样，匹配得到的 `Option<String>`。但是，对于这一章中的大多数示例中（以及现实世界中的大多数异步代码），我们都将执行不止一个异步函数调用，因此我们将传递一个 `async` 代码块，并显式等待 `page_title` 调用的结果，如下清单 17-4 中所示。

<a name="listing_17-4"></a>
文件名：`src/main.rs`

```rust
fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::block_on(async {
        let url = &args[1];
        match page_title(url).await {
            Some(title) => println!("{url} 的标题是 {title}"),
            None => println!("{url} 没有标题"),
        }
    })
}
```

**清单 17-4**：通过 `trpl:block_on` 等待异步代码块

当我们运行这段代码时，我们得到了最初预期的行为：

```console
$ cargo run -- https://rust-lang.xfoss.com
   Compiling hello-async v0.1.0 (/home/hector/rust-lang-zh_CN/projects/hello-async)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.58s
     Running `target/debug/hello-async 'https://rust-lang.xfoss.com'`
https://rust-lang.xfoss.com 的标题是 序和前言 - Yet another Chinese rust-lang book.
```

呼--我们终于有了一些可以工作的异步代码！不过，但在添加代码让两个网站互相竞赛之前，我们先来简要回顾一下未来值的工作原理。

每个 *等待点，await point* -- 也就是代码中使用 `await` 关键字的每处 -- 都代表控制权交还给运行时的位置。为了做到这一点，Rust 需要跟踪异步代码块中涉及的状态，以便运行时可以启动一些其他工作，然后在准备好再次尝试推进第一项工作时返回。这属于不可见（隐形）的状态机，an invisible state machine，就好像咱们编写了个如下的枚举，来保存每个等待点处的当前状态一样：


```rust
enum PageTitleFuture<'a> {
    Initial { url: &'a str },
    GetAwaitPoint { url: &'a str },
    TextAwaitPoint { response: trpl::Response },
}
```

然而，手动编写代码以在各个状态之间转换既繁琐又容易出错，尤其是在日后咱们需要添加更多功能和状态时。幸运的是，Rust 编译器会自动创建和管理异步代码的状态机数据结构。围绕数据结构的正常借用和所有权规则仍然适用，并且令人高兴的是，编译器还会为我们检查这些检查，并提供有用的错误消息。我们将在这一章后面讨论其中一些方面。

最终，必须有某种机制来执行这个状态机，而这个机制就是运行时（这就是为什么咱们在研究运行时时，可能会遇到 *执行器，executors* 的提法：所谓执行器，是运行时中负责执行异步代码的部分。）

现在咱们就能明白，为什么在清单 17-3 中，编译器阻止我们构造 `main` 本身为一个异步函数。若 `main` 是个异步函数，针对`main` 返回的未来值，就需要由其他组件来管理状态机，但 `main` 是程序的起点！因此，我们在 `main` 中调用 `trpl::block_on` 函数来设置运行时，并运行 `async` 代码块返回的未来值，知道其完成。

> **注意**：某些运行提供了宏，以便咱们 *可以* 编写异步的 `main` 函数。这些宏会重写 `async fn main() { ... }` 为普通的 `fn main`，这与我们在清单 17-4 中手动完成的相同：调用一个函数，该函数会像 `trpl::run` 那样运行未来值至完成。

现在，我们来把这部分放在一起，看看我们可以怎样编写并发代码。


## 让两个 URL 并发地相互竞争

在下面的清单 17-5 中，我们从命令行以两个传入的两个不同 URL 调用 `page_title`，并通过选择先完成的未来值让他们竞争。

<a name="listing_17-5"></a>
文件名：`src/main.rs`

```rust
use trpl::{Either, Html};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let fut_result = async {
        let title_fut_1 = page_title(&args[1]);
        let title_fut_2 = page_title(&args[2]);

        let (url, maybe_title) =
            match trpl::select(title_fut_1, title_fut_2).await {
                Either::Left(left) => left,
                Either::Right(right) => right,
            };

        println! ("{url} 先返回");
        match maybe_title {
            Some(title) => println! ("其页面标题为： '{title}'"),
            None => println! ("他没有标题。"),
        }
    };

    trpl::block_on(fut_result)
}

async fn page_title(url: &str) -> (&str, Option<String>) {
    let response_text = trpl::get(url).await.text().await;
    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)
}
```

**清单 17-5**：对两个 URL 调用 `page_title` 以查看那个先返回

我们以对用户提供的每个 URL 调用 `page_title` 开始。我们保存得到的未来值为 `title_fut_1` 与 `title_fut_2`。请记住，这些还不会执行任何操作，因为未来值是懒惰的，且我们尚未对其进行等待。然后我们传递未来值给 `trpl::select`，他会返回一个值，表明传递给他的未来值中哪个先完成。


> **注意**：在底层，`trpl::select` 是基于定义在 `futures` 代码箱中的一个更通用的 `select` 函数构建的。`futures` 代码箱的 `select` 函数可以完成 `trpl::select` 函数无法完成的许多事情，但他也有一些额外的复杂性，目前我们可以跳过。

由于两个未来值都可以合法地 “胜出”，因此返回 `Result` 并不合理。相反，`trpl::select` 返回一种我们以前从未见过的类型 `trpl::Either`。`Either` 类型与 `Result` 有点相似，因为他有两种情况。但与 `Result` 不同的是，`Either` 中没有成功或失败的概念。相反，他使用 `Left` 与 `Right` 来表示 “二者之一”：


```Rust
enum Either<A, B> {
    Left(A),
    Right(B),
}
```

当第一个参数获胜时，`select` 函数返回带有该未来值的输出的 `Left`，当第二个未来值参数获胜时，则返回带有 *该* 未来值的输出的 `Right`。这与调用该函数时参数出现的顺序一致：第一个参数位于第二个参数的左侧。

我们还更新了 `page_title` 为返回所传入的同一 URL。这样，当首先返回的页面没有我们可以解析的 `<title>` 时，我们仍然可以打印出一条有意义的消息。有了这些信息，我们最后更新 `println!` 的输出为既表明哪个 URL 先完成，也表明该 URL 处的 web 页面的 `<title>`，在有该元素时。

现在，咱们已经构建了个可运行的小型 web 爬虫！请选择几个 URL 并运行这个命令行工具。咱们可能会发现，一些网站始终比其他网站快，而在其他情况下，较快的网站每次运行会有所不同。更重要的是，咱们已经掌握了使用未来值的基础知识，因此现在我们可以更深入地探索在异步下我们可以做些什么。

（End）


