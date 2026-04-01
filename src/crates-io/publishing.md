# 发布代码箱到 Crates.io

我们已将 [crates.io](https://crates.io) 上的包用作过项目的依赖项，但我们也可以通过发布咱们自己的包，与其他人分享自己的代码。位于 [crates.io](https://crates.io) 网站的代码箱登记簿，会分发咱们包的源码，因此他主要托管开源代码。

Rust 与 Cargo 都有着一些让我们发布的包，更容易被人们找到并使用的特性。接下来我们将讨论其中一些特性，然后讲解怎样发布包。


## 制作有用的文档注释

准确地为咱们的包编写文档，将帮助到其他用户了解怎样及何时来使用他们，因此投入时间编写文档是非常值得的。在第 3 章中，我们讨论过怎样使用双斜杠 `//` 来注释 Rust 代码。Rust 还有针对文档的一种特别注释，通常称为 *文档注释，documentation comment*，将生成 HTML 文档。生成的 HTML 会显示针对公开 API 项目的文档注释内容，是为有兴趣了解怎样 *使用* 咱们的代码箱，而不是对其 *实现* 感兴趣的程序员准确的。

文档注释使用三斜杠 `///` 而非双斜杠，并支持 Markdown 表示法来格式化文本。要放置文档注释于他们说明的项目正上方。下面清单 14-1 展示了名为 `my_crate` 的代码箱中 `add_one` 函数的文档注释。

<a name="listing_14-1"></a>
文件名：`src/lib.rs`

~~~rust
/// 加一到给定的数字。
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq! (6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
~~~

**清单 14-1**：函数的文档注释

在这里，我们描述了 `add_one` 函数执行的操作，以标题 `Examples` 开始了一个小节，然后提供了演示怎样使用 `add_one` 函数的代码。我们可以通过运行 `cargo doc` 从这一文档注释生成 HTML 文档。这条命令运行随 Rust 一起分发的 `rustdoc` 工具，并放置生成的 HTML 文档于 `target/doc` 目录中。

出于便利目的，运行 `cargo doc --open` 将针对咱们的当前代码箱的文档（以及所有咱们的代码箱的依赖项的文档）构建 HTML，并在 web 浏览器中打开结果。导航到 `add_one` 函数，咱们将看到文档注释中的文本是如何渲染的，如下图 14-01 中所示：

<a name="f_14-1">
![`add_one` 函数的 HTML 文档](../images/14-01.png)

**图 14-1**：`add_one` 函数的 HTML 文档


### 常用小节

我们在清单 14-1 中使用了 `# Examples` 这个 Markdown 标题，以标题 “Examples” 创建了 HTML 中的一个小节。以下是代码箱作者在他们的文档中常用的一些别的小节：

- `# Panics`：这些属于被文档注释的函数可能终止运行的情形。不希望他们的程序终止运行的调用者，应确保在这些情形下不要调用该函数；
- `# Errors`：当函数返回一个 `Result` 时，描述可能发生的错误类别以及可能导致返回这些错误的条件，会对调用者很有帮助，以便他们可以编写以不同方式处理不同类别错误的代码;
- `# Safety`：当函数的调用属于 `unsafe` （我们会在第 20 章讨论不安全）时，就应有一个小节解释为何该函数不安全，并涵盖该函数期望调用者遵守的不变量。

大多数文档注释都不需要所有这些小节，但这是个很好的检查清单，可以提醒咱们代码使用者有兴趣了解的各方面。


### 作为测试的文档注释

在文档注释中添加示例代码块，可以帮助演示怎样使用咱们的库，并且还有个额外的好处：运行 `cargo test` 将作为测试运行文档中的代码示例！没有什么比带有示例的文档更好的了。但最糟糕的也莫过于示例无法运行，只因文档编写后代码已被修改。当咱们对清单 14-1 中的 `add_one` 函数的文档运行 `cargo test` 时，我们将看到测试结果中下面这样的一个小节：

```console
   Doc-tests my_crate

running 1 test
test src/lib.rs - add_one (line 5) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

现在，当我们修改函数或者示例，从而让示例中的 `assert_eq!` 终止运行，并再次运行 `cargo tset` 时，我们将发现文档测试检测到示例和代码之间未相互保持一致！

> **译注**：此时运行测试的输出为：
>
> ```console
>    Doc-tests my_crate
>
> running 1 test
> test src/lib.rs - add_one (line 5) ... FAILED
>
> failures:
>
> ---- src/lib.rs - add_one (line 5) stdout ----
> Test executable failed (exit status: 101).
>
> stderr:
>
> thread 'main' (462614) panicked at /tmp/rustdoctestw0cEJx/doctest_bundle_2024.rs:9:1:
> assertion `left == right` failed
>   left: 6
>  right: 7
> note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
>
>
>
> failures:
>     src/lib.rs - add_one (line 5)
>
> test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
> ```
>
> 执行 `cargo test --doc`，将只运行文档注释中的示例代码。


### 包含的程序项目注释


`//!` 样式的文档注释，会添加文档到 *包含* 注释的项目，而非注释 *之后* 的项目。我们通常在代码箱根文件（依惯例为 `src/lib.rs`）内，或模组内添加这些文档注释，来将代码箱或模组作为整体，而为其编写文档。

例如，为了添加描述包含 `add_one` 函数的 `my_crate` 代码箱用途的文档，我们就要添加以 `//!` 开头的文档注释，至 `src/lib.rs` 文件的开头，如下清单 14-2 中所示：

<a name="listing_14-2"></a>
文件：`src/lib.rs`

```rust
//! # 我的代码箱
//!
//! `my_crate` 是个实用工具集，旨在让执行
//! 某些计算更加便捷。

/// 加一到给定的数字。
// --跳过代码--
```

**清单 14-2**：`my_crate` 代码箱作为一个整体的文档

请注意，以 `//!` 开头的最后一行之后没有任何代码。由于我们以 `//!` 而不是 `///` 开始注释，因此我们是在给包含这种注释的程序项目，而非紧接着这种注释之后的程序项目编写文档。在这一情形下，该项目是 `src/lib.rs` 文件，是代码箱根。这些注释描述了整个代码箱。

当我们运行 `cargo doc --open` 时，这些注释将显示在 `my_crate` 代码箱文档的首页，在代码箱的公开项目列表上方，如下图 14-2 中所示：

<a name="f_14-2"></a>
![渲染出的 `cargo_features_demo` 代码箱文档](../images/14-02.png)

**图 14-02**：`my_crate` 渲染后的文档, 包括作为整体描述该代码箱的注释

项目内的文档注释对于描述代码箱及模组尤其有用。请使用他们说明容器的总体用途，以帮助咱们的用户理解代码箱的组织结构。


## 导出便捷的公开 API

在发布代码箱时，公开 API 的结构属于主要考量。相比咱们自己，使用代码箱的人对代码箱结构的没有那么熟悉，当咱们的代码箱有着庞大的模组层次结构时，他们可能在找到他们打算使用的部分时遇到困难。

在第 7 章中，咱们介绍了怎样使用 `pub` 关键字构造项目为公开，以及怎样以 `use` 关键字带入项目到作用域。然而，在开发代码箱时对咱们有意义的组织结构（模组树），对于咱们的用户可能并不方便。咱们可能打算组织咱们的结构为包含多个级别的层次结构，但后来打算使用某个咱们定义在层次结构深处的类型的人，可能在找出该类型是否存在时遇到麻烦。他们可能还会因为不得不输入 `my_crate::some_module::another_module::UsefulType;`，而不是输入 `use my_crate::UsefulType;` 而感到恼火。

好消息是，当组织结构 *不* 便于其他人在另一库中使用时，咱们不必调整咱们的内部组织结构：相反，咱们可以通过使用 `pub use` 重新导出程序项目，以构造一种不同于咱们的私有组织结构的公开组织结构。所谓 *重新导出*，会取位于一处的某个公开程序项目，并构造其为在另一处公开，就像他被定义在另一处一样。

例如，假设我们出于建模美术概念目的，构造了一个名为 `art` 的库。这个库内有两个模组：包含两个名为 `PrimaryColor` 与 `SeccondaryColor` 枚举的 `kinds` 模组，和包含名为 `mix` 函数的 `utils` 模组，如下清单 14-3 中所示：

<a name="listing_14-3"></a>
文件名：`src/lib.rs`

```rust
//! # 美术
//!
//! 用于建模美术概念的库。

pub mod kinds {
    /// 根据 RYB 颜色模型的原色。
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// 根据 RYB 颜色模型的间色。
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// 等量组合两种原色以创建
    /// 一种间色。
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --跳过代码--
        SecondaryColor::Purple
    }
}
```

**清单 14-3**：`art` 库，有着组织在 `kinds` 与 `utils` 两个模组中的程序项目

下图 14-03 展示了由 `cargo doc` 生成的这个代码箱的文档的首页的样子：

<a name="f_14-3"></a>
![列出 `kinds` 与 `utils` 两个模组的 `art` 代码箱文档首页](../images/14-03.png)

**图 14-3**：`art` 库的文档的首页，列出了 `kinds` 与 `utils` 两个模组

请注意，`PrimaryColor` 与 `SecondaryColor` 两个类型并未在首页上列出，`mix` 函数也如此。我们必须点击 `kinds` 与 `utils` 才能看到他们。

依赖于这个库的另一代码箱将需要 `use` 语句，带入 `art` 中的项目到作用域，指定当前定义的模组结构。下面清单 14-4 展示了一个代码箱示例，使用 `art` 代码箱中的 `PrimaryColor` 与 `mix` 两个程序项目：

<a name="listing_14-4"></a>
文件名：`src/main.rs`

```rust
use art::kinds::PrimaryColor;
use art::utils::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
```

**清单 14-4**：使用 `art` 代码箱的项目的代码箱，`art` 代码箱的内部组织结构已导出

> **译注**：使用本地未发布代码箱的方法，是在 `Cargo.toml` 的 `[dependencies]` 小节中，列出要使用的本地未发布代码箱。参见 [How to use a local unpublished crate?](https://stackoverflow.com/a/33025972)
>
> 文件：`Cargo.toml`
>
> ```toml
> // --跳过代码--
>
> [dependencies]
> art = { path = "../art" }
> ```

清单 14-4 中使用 `art` 代码箱的代码的作者，必须搞清楚 `PrimaryColor` 位于 `kinds` 模组中，而 `mix` 位于 `utils` 模组中。`art` 代码箱的模组结构（即模组树），相比使用他的人，对于编写 `art` 代码箱的开发者更为密切。对于试图了解怎样使用 `art` 代码箱的人来说，这一内部组织结构并未包含任何有用信息，反而会造成混淆，因为使用他的开发者必须弄清楚在哪里去查看，并且必须在 `use` 语句中指定模组名字。

为了移除公开 API 中的内部组织结构，我们可以修改 [清单 14-3](#listing_14-3) 中 `art` 代码箱的代码，为添加一些 `pub use` 语句，以在顶层重新导出程序项目，如下清单 14-5 中所示：

<a name="listing_14-5"></a>
文件名：`src/lib.rs`

```rust
//! # art
//!
//! 建模诸多美术概念的一个库。

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    // -- 跳过代码 --
}

pub mod utils {
    // -- 跳过代码 --
}
```

**清单 14-5**：添加 `pub use` 语句以重新导出程序项目

如下图 14-04 中所示，`cargo doc` 对这个代码箱生成的 API 文档，现在将在首页上列出并链接到重导出项，从而让 `PrimaryColor` 与 `SecondaryColor` 两个类型以及 `mix` 函数更容易找到。

<a name="f_14-4"></a>
![列出了重导出项目的 `art` 代码箱文档首页](../images/14-04.png)

**图 14-4**：`art` 代码箱文档的首页，列出了重新导出项目

`art` 代码箱的用户仍然可以如同 [清单 14-4](#listing_14-4) 中演示的那样，看到并使用 [清单 14-3](#listing_14-3) 中的内部结构，或者他们可使用清单 14-5 中的更方便的结构，如下清单 14-6 中所示：

<a name="listing_14-6"></a>
文件名：`src/main.rs`

```rust
use art::mix;
use art::PrimaryColor;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
```

**清单 14-6**：使用 `art` 代码箱中重新导出项目的程序

在存在许多嵌套模组的情形下，以 `pub use` 在顶层重新导出类型，可以对使用代码箱的人的体验造成显著差异。`pub use` 的另一个常见用途是重新导出当前代码箱中的依赖项的定义，以使该代码箱的定义，成为咱们的代码箱的公开 API 的一部分。

创建有用的公开 API 结构，与其说是一门科学，不如说是一门艺术，而咱们可不断迭代，找到最适合用户的 API。选择 `pub use` 赋予了咱们在内部组织代码箱方式上的灵活性，并解耦了内部结构与呈现给用户的结构。请查看咱们已安装的一些代码箱的代码，看看发现他们的内部结构是否与其公开 API 有所不同。


## 建立 Crates.io 帐号

在发布任何代码箱之前，咱们需要在 [crates.io](https://crates.io) 上创建一个账户并获取 API 令牌。为此，请访问 [crates.io](https://crates.io) 的主页，并通过 GitHub 帐号登录。（目前需要 GitHub 账户，但该站点今后可能会支持其他创建帐号的方式。）登录后，请访问 [https://crates.io/me/](https://creates.io/me/) 处的帐号设置，并获取咱们的 API 密钥。然后，运行 `cargo login` 命令并在出现提示时粘贴咱们的密钥，如下所示：

```console
$ cargo login --registry crates-io
please paste the token found on https://crates.io/me below
abcdefghijklmnopqrstuvwxyz012345
       Login token for `crates-io` saved
```

这条命令将告知 Cargo 咱们的 API 令牌，并存储在本地的 `~/.cargo/credentials` 文件中。请注意，这个令牌属于机密信息：请不要与任何人分享。当咱们出于任何原因与任何人分享了时，咱们都应在 [crates.io](https://crates.io) 上吊销他并生成一个新的令牌。

> **译注**：原文这里是仅运行 `cargo login`，但译者已将登记簿修改为国内镜像，因此要加上 `--registry crates-io` 命令行选项。
>
> ```console
> $ cargo login
> error: crates-io is replaced with non-remote-registry source registry `ustc`;
> include `--registry crates-io` to use crates.io
> ```
>
> 要从 crates.io 登出，运行 `cargo logout` 即可。
>
> ```console
> $ cargo logout
>       Logout token for `crates-io` has been removed from local storage
> note: This does not revoke the token on the registry server.
>     If you need to revoke the token, visit <https://crates.io/me> and follow the instructions there.
> ```


## 添加元数据到新代码箱

假设咱们有个打算发布的代码箱。在发布前，咱们将需要在该代码箱的 `Cargo.toml` 文件的 `[package]` 小节中添加一些元数据。

咱们的代码箱将需要一个独特的名字。在本地开发代码箱时，咱们可以给代码箱取随意命名。但是，[crates.io](https://crates.io) 上的代码箱名字，是按照先到先得的原则分配的。一旦某个名字已被占用，其他人就不能以那个名字发布代码箱。在尝试发布代码箱之前，请县检索咱们打算使用的名字。当名字已被使用时，咱们将需要找到另一个名字，并编辑 `Cargo.toml` 文件中 `[package]` 小节下的 `name` 字段，以使用新的名字进行发布，像下面这样：

文件名：`Cargo.toml`

```toml
[package]
name = "guessing_game"
```

即使咱们选择了个独特的名字，当咱们此时运行 `cargo publish` 来发布代码箱时，仍将受到一条告警，然后一条报错：


```console
$ cargo publish --registry crates-io
    Updating crates.io index
warning: manifest has no description, license, license-file, documentation, homepage or repository
  |
  = note: see https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata for more info
-- 跳过输出 --
error: failed to publish guessing_game-xfossdotcom v0.1.2 to registry at https://crates.io

Caused by:
  the remote server responded with an error (status 400 Bad Request): missing or empty metadata fields: description, license. Please see https://doc.rust-lang.org/cargo/reference/manifest.html for more information on configuring these fields
```

这导致了一个报错，因为咱们缺少一些关键信息：描述信息及许可证是必需的，由此人们才会知道咱们的代码箱做什么，以及可以在什么条款下使用他。在 `Cargo.toml` 中，添加以两句话的描述信息，因为他会与咱们的代码箱一起出现在搜索结果中。对于 `license` 字段，咱们需要提供 *许可证标识符值*。[Linux 基金会的软件包数据交换](http://spdx.org/licenses/) 列出了咱们可以针对该值使用的标识符。例如，要指定咱们已使用 MIT 许可证授权咱们的代码箱，请添加 `MIT` 标识符：


文件名：`Cargo.toml`

```toml
[package]
name = "guessing_game"
license = "MIT"
```

当咱们打算使用某种未出现在 SPDX 中的许可证时，咱们就需要放置该许可证的文本于文件中，在咱们的项目中包含该文件，然后使用 `license-file` 来指定该文件的名字，而不是使用 `license` 键。

关于哪种许可证适合咱们的项目方面的指南超出了这本书的范围。Rust 社区的许多人都以与 Rust 项目相同方式，使用 `MIT OR Apache-2.0` 双重许可证授权他们的项目。这种做法表明，咱们也可以指定由 `OR` 分隔的多个许可证标识符，有着针对咱们项目的多种许可证。

添加了唯一名字、版本号、描述信息及许可证后，某个已准备好发布的项目的 `Cargo.toml`文件可能看起来像下面这样：

文件名：`Cargo.toml`

```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2024"
description = "一个有趣的游戏，咱们需要猜出电脑选择了哪个数字。"
license = "MIT OR Apache-2.0"

[dependencies]
```

[Cargo 的文档](https://doc.rust-lang.org/cargo/) 描述了其他咱们可以指定的元数据，以确保其他人可以更轻松地发现和使用咱们的代码箱。


## 发布到 Crates.io

现在咱们已经创建了账号，保存了 API 令牌，为代码箱选择了名字，并指定了所需的元数据，咱们就可以开始发布了！发布代码箱会上传特定版本到 [crates.io](https://crates.io)，供他人使用。

请小心！因为发布是 *永久性的*。该版本永远无法被覆盖，除特殊情况外，代码也无法被删除。Crates.io 的一个主要目标是充当代码的永久存档，以便依赖于 [crates.io](https://crates.io) 上的代码箱的所有项目的构建都将持续工作。允许版本删除将使该目标变得不可能。不过，咱们可以发布的代码箱版本数量没有限制。

再次运行 `cargo publish` 命令。现在应该成功了：

```console
$ cargo publish --registry crates-io
    Updating crates.io index
   Packaging guessing_game-xfossdotcom v0.1.2 (/home/hector/rust-lang-zh_CN/projects/guessing_game)
    Updating `ustc` index
    -- 跳过输出 --
    Packaged 7 files, 5.9KiB (2.6KiB compressed)
   Verifying guessing_game-xfossdotcom v0.1.2 (/home/hector/rust-lang-zh_CN/projects/guessing_game)
   Compiling guessing_game-xfossdotcom v0.1.2 (/home/hector/rust-lang-zh_CN/projects/guessing_game/target/package/guessing_game-xfossdotcom-0.1.2)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 36.15s
   Uploading guessing_game-xfossdotcom v0.1.2 (/home/hector/rust-lang-zh_CN/projects/guessing_game)
    Uploaded guessing_game-xfossdotcom v0.1.2 to registry `crates-io`
note: waiting for guessing_game-xfossdotcom v0.1.2 to be available at registry `crates-io`
help: you may press ctrl-c to skip waiting; the crate should be available shortly
   Published guessing_game-xfossdotcom v0.1.2 at registry `crates-io`
```

恭喜！咱们现在已经与 Rust 社区分享了咱们的代码，任何人都可以轻松地添加咱们的代码箱为他们项目的依赖项。

> **译注**：在 Crates.io 上的账号电子邮箱未验证时，将报出如下错误：
>
> ```console
> Caused by:
>   the remote server responded with an error: A verified email address is required to publish crates to crates.io. Visit https://crates.io/me to set and verify your email address.
> ```


## 发布现有代码箱的新版本

当咱们对代码箱的进行了修改，而准备发布新版本时，咱们要修改在 `Cargo.toml` 中指定的 `version` 值并重新发布。请根据咱们所做的修改类型，使用 [语义化版本控制规则](http://semver.org/) 来确定合适的下一个版本编号。然后，运行 `cargo publish` 上传新的版本。


## 弃用 Crates.io 上的版本

尽管咱们无法移除代码箱的较早版本，但咱们可以阻止任何今后的项目添加他们为新的依赖项。当代码箱版本出于某种原因被破坏时，这一特性非常有用。在这种情形下，Cargo 支持 *抽出* 代码箱版本。

*抽出* 版本会防止新项目依赖于该版本，同时允许所有依赖他的现有项目继续正常运行。本质上，抽出意味着所有带有 `Cargo.lock` 的项目都不会中断，并且任何今后生成的 `Cargo.lock` 文件都将不会使用抽出的版本。

要抽出代码箱的某个版本，就要在咱们先前发布的代码箱目录下，运行 `cargo yank` 并指定咱们打算抽出的版本。例如，当咱们已发布一个名为 `guessing_game` 版本 `0.1.0` 的代码箱，而打算抽出他时，那么我们就要在 `guessing_game` 的项目目录下运行以下命令：

```console
$ cargo yank --vers 0.1.0 --registry crates-io
    Updating crates.io index
        Yank guessing_game-xfossdotcom@0.1.0
```

通过添加 `--undo` 到这个命令，咱们还可以撤销抽出，而允许项目再次依赖于某个版本：

```console
$ cargo yank --vers 0.1.0 --undo --registry crates-io
    Updating crates.io index
      Unyank guessing_game-xfossdotcom@0.1.0
```

抽出版本 *不会* 删除任何代码。例如，他无法删除意外上传的机密信息。当发生这种情况时，咱们必须立即重置这些机密信息。


（End）


