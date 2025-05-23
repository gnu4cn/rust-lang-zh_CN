# 近观异步有关的特质


在本章中，我们以各种方式用到了 `Future`、`Pin`、`Unpin`、`Stream` 及 `StreamExt` 等特质。不过，到目前为止，我们还没有深入探讨他们的工作原理，或他们相互配合的细节，这对咱们日常的 Rust 工作来说，在大多数情况下是没有问题的。但有时，咱们将遇到需要了解更多细节的情况。在这个小节中，我们将涉足这些细节，以便在那些情形下有所帮助，而 *真正的* 深入探讨，则留给其他文档。


## `Future` 特质

我们来先仔细看看 `Future` 这个特质是如何工作的。下面是 Rust 对他的定义：



```rust
use std::pin::Pin;
use std::task::{Context, Poll};

pub trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```


该特质定义包含了许多新类型，以及一些我们以前从未见过的语法，因此我们来逐一了解一下该定义。


首先，`Future` 关联的类型 `Output`，指出了该未来值会解析为什么。这类似于 `Iterator` 特质关联的类型 `Item`。其次，`Future` 还有着 `poll` 方法，该方会取一个特殊的 `Pin` 引用，作为其 `slef` 参数，以及一个到 `Context` 类型的可变引用，并返回一个 `Poll<Self::Output>`。稍后我们将进一步讨论 `Pin` 与 `Context`。现在，我们着重于该方法会返回什么，即那个 `Poll` 类型：


```rust
enum Poll<T> {
    Ready(T),
    Pending,
}
```

这个 `Poll` 类型类似于 `Option`。他有个有值的变种，即 `Ready(T)`，还有个没有值的变种 `Pending`。不过，`Poll` 所指的内容与 `Option` 完全不同！`Pending` 变种表示这个未来值仍有工作要做，所以调用者将需要稍后再检查。`Ready` 变种表示该未来值已完成其工作，`T` 值可用。


> **注意**：对于大多数未来值，调用者都不应在该未来值返回 `Ready` 后，再次调用 `poll`。若在就绪后再次轮询，许多未来值都将出现不会恢复错误。再次轮询安全的未来值，会在其文档中明确说明。这与 `Iterator::next` 的行事方式类似。



当咱们看到使用了 `await` 的代码时，Rust 会在背后将其编译为调用 `poll` 的代码。回顾 [清单 17-4](futures.md#listing-17-4)，其中我们曾打印出单个 URL 解析后的页面标题，Rust 就会将其编译成类似（但不完全）下面这样的代码：


```rust
match page_title(url).poll() {
    Ready(page_title) => match page_title {
        Some(title) => println!("The title for {url} was {title}"),
        None => println!("{url} had no title"),
    }
    Pending => {
        // But what goes here?
    }
}
```
