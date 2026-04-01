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

请注意由于咱们是以 `//!` 而非 `///` 开始的这些注释，因此在以 `//!` 开始的最后一行后，并无任何代码的，咱们是在给包含此注释的程序项目，而非紧接着此注释的程序项目编写文档。在此示例中，那个程序项目就是 `src/lib.rs` 文件，为代码箱根。这些注释描述了整个代码箱。

当咱们运行 `cargo doc --open` 时，这些注释将显示在 `cargo_features_demo` 代码箱文档的首页，he front page，位处代码箱公开项目的清单之上，如下图 14-02 中所示：

![渲染出的 `cargo_features_demo` 代码箱文档](../images/14-02.png)

*图 14-02：渲染出的 `cargo_features_demo` 代码箱文档, 包括着将该代码箱作为整体描述的注释*

程序项目里的文档注释，用于对描述代码箱及模组尤其有用。使用他们来解释容器，the container，的整体目标，有助于咱们的用户们理解代码箱的组织结构。


## 导出便利的公开 API


在咱们发布代码箱时，公开 API 的结构是主要的考量。相比与咱们，使用咱们代码箱的人们对代码箱结构的没有那么熟悉，并在咱们的代码箱有着大型模组层次结构时，难于找到他们打算使用的部分。

在第 7 章中，咱们曾讲到过怎样使用 `pub` 关键字把一些程序项目构造为公开，与怎样使用 `use` 关键字，把程序项目带入到作用域。但是，咱们在开发某个代码箱时，对咱们有意义的组织结构（模组树），对于咱们的用户则可能不那么便利。咱们会打算把代码箱结构组织为包含多个级别的层次，但随后某个想要使用定义在层次结构深处类型的人，就可能在找出那个类型是否存在上遇到麻烦。他们可能还会对必须敲入 `use cargo_features_demo::some_module::another_module::UsefulType;`，而非敲入 `use cargo_features_demo::UsefulType;` 而感到恼火。

可喜的是，若代码箱组织结构 *不* 便于其他人在另一库中使用，咱们不必重新调整代码箱的内部组织：相反，咱们可通过使用 `pub use`，重新导出程序项目，而构造出一种不同于咱们私有组织结构的公开组织结构。重新导出，re-export，会取一处的公开程序项目，而在另一处将其构造为公开，就跟这个项目是在那另一处被定义过一样。

比如说，咱们构造了用于建模美术概念的一个名为 `art` 的库。这个库里有两个模组：包含了两个名为 `PrimaryColor` 与 `SeccondaryColor` 枚举的 `kinds` 模组，与包含了名为 `mix` 函数的 `utils` 模组，如下清单 14-3 中所示：

文件名：`src/lib.rs`

```rust
//! # art
//!
//! 建模诸多美术概念的一个库。

pub mod kinds {
    /// RYB 颜色模型下的主要颜色。
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// RYB 颜色模型下的次要颜色。
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// 结合两种等量的主要颜色，创建出
    /// 某种次要颜色。
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --跳过代码--
        SecondaryColor::Purple
    }
}
```

*清单 14-3：有着组织到 `kinds` 与 `utils` 两个模组中的一些程序项目的 `art` 库*

下图 14-03 展示了由 `cargo doc` 产生出的该代码箱文档首页，看起来的样子：

![列出 `kinds` 与 `utils` 两个模组的 `art` 代码箱文档首页](../images/14-03.png)

*图 14-3：列出 `kinds` 与 `utils` 两个模组的 `art` 代码箱文档首页*


请注意 `PrimaryColor` 与 `SecondaryColor` 两个类型，及 `mix` 函数都未在首页上列出。要看到他们，咱们必须点击 `kinds` 与 `utils`。

依赖于这个库的另一代码箱，将需要把程序项目从 `art` 带入到作用域的 `use` 语句，与指明当前定义的模组结构。下面清单 14-4 给出了用到 `art` 代码箱中 `PrimaryColor` 与 `mix` 两个程序项目的代码箱示例：

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

*清单 14-4：用到 `art` 代码箱以内部组织结构导出程序项目的代码箱*

> **注**：使用本地未发布代码箱的方法，是在 `Cargo.toml` 的 `[dependencies]` 小节中，列出要使用的本地未发布代码箱。参见 [How to use a local unpublished crate?](https://stackoverflow.com/a/33025972)

文件：`Cargo.toml`

```toml
// --跳过代码--

[dependencies]
art = { path = "../art" }
```

清单 14-4 中用到 `art` 代码箱代码的作者，不得不搞清楚 `PrimaryColor` 是在 `kinds` 模组中，及 `mix` 函数是在 `utils` 模组中。`art` 代码箱的模组结构（即模组树），相比于用到该代码箱的开发者，与在 `art` 代码箱上编写代码的开发者要更为密切。对于试图搞清楚怎样使用 `art` 代码箱的人来说，其内部组织结构并未包含任何有用信息，而因为要用到他的开发者，不得不搞明白要在那里去查看，且必须在 `use` 语句中指明那些模组名字，这反而会造成混乱。

要从公开 API 中移除内部组织结构，咱们可把清单 14-3 中 `art` 代码箱的代码，修改为添加一些 `pub use` 语句，来在顶层处重导出程序项目，to re-export the items at the top level，如下清单 14-5 中所示：

文件名：`src/lib.rs`

```rust
//! # art
//!
//! 建模诸多美术概念的一个库。

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds;
pub mod utils;
```

*清单 14-5：添加 `pub use` 语句来重导出程序项目*

如下图 14-04 中所示，`cargo doc` 为此代码箱所产生出的 API 文档，现在将在首页上列出并链接到重导出项，从而令到 `PrimaryColor` 与 `SecondaryColor` 两个类型及 `mix` 函数更易于找到。

![列出了重导出项目的 `art` 代码箱文档首页](../images/14-04.png)

*图 14-4：列出重导出项的 `art` 代码箱文档首页*

`art` 代码箱的用户，依然可以像清单 14-4 中所演示的那样，发现及使用清单 14-3 的内部结构，或者他们可使用清单 14-5 中更为便利的结构，如下清单 14-6 中所示：

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

*清单 14-6：使用着 `art` 代码箱重导出项的程序*

其中有许多嵌套模组的情形下，以 `pub use` 在顶层重导出类型，可在用到该代码箱的人的体验方面，造成显著不同。`pub use` 的另一常见用途则是，为将依赖代码箱的定义构造为咱们自己代码箱公开 API 的一部分，而重导出当前代码箱中某个依赖的定义。

创建出有用的公开 API 结构，与其说是一门科学，不如说是一门艺术，而咱们可不断迭代，来找到对用户运作最佳的 API。选择 `pub use` 会给到咱们在内部组织代码箱方式上的灵活性，并解除了内部结构与呈现给代码箱用户的组织结构的耦合。请查看咱们曾安装的代码箱代码，来发现他们的内部结构，是否不同于其公开 API。


## 建立 Crates.io 帐号


在咱们能发布代码箱之前，咱们需要在 [crates.io](https://crates.io) 上创建帐号，并得到 API 令牌，an API token。而要这样做，就要访问 [crates.io](https://crates.io) 处的主页，并通过 GitHub 帐号登录。（目前 GitHub 帐号是必须的，但该站点今后可能会支持其他创建帐号途径。）在登录后，咱们就要访问 [https://crates.io/me/](https://creates.io/me/) 处的帐号设置，而获取自己的 API 密钥，API key。然后使用咱们的 API 密钥，运行 `cargo login` 命令，如下：

```console
$ cargo login abcdefghijklmnopqrstuvwxyz012345
```

此命令将告知 Cargo 咱们的 API 令牌，并在 `~/.cargo/credentials` 文件中本地存储起来。请注意此令牌是个 *秘密，secret*：不要与任何人分享。不论因何种缘故，与任何人分享了，咱们都应吊销他，并在 [crates.io](https://crates.io) 上生成新的令牌。


## 添加元数据到新代码箱

Adding Metadata to a New Crate**


假设咱们有了个打算发布的代码箱。在发布前，咱们将需要在代码箱的 `Cargo.toml` 文件的 `[package]` 小节中，添加一些元数据。

咱们的代码箱将需要一个独特的名字。当咱们在本地于代码箱上工作时，咱们可以给代码箱取任意喜欢的名字。但是，[crates.io](https://crates.io) 上代码箱的名字，则是以先到先得的原则分配的，allocated on a first-come, first-served basis。一旦某个代码箱名字已被占用，其他人就不能发布有着那个名字的代码箱。在尝试发布某个代码箱之前，咱们要检索一下打算使用的名字。若这个名字已被使用，咱们将需要找到另一名字，并编辑 `Cargo.toml` 文件中 `[package]` 小节下的 `name` 字段，来使用这个用作发布的新名字，像下面这样：

文件名：`Cargo.toml`

```toml
[package]
name = "guessing_game"
```

即使咱们已选了个独特的名字，当咱们此时运行 `cargo publish` 来发布这个代码箱时，仍将得到一条告警及随后的报错：


```console
cargo publish                                                                                          lennyp@vm-manjaro
    Updating crates.io index
warning: manifest has no description, license, license-file, documentation, homepage or repository.
See https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata for more info.
   Packaging guessing_game v0.1.0 (/home/lennyp/rust-lang/guessing_game)
   Verifying guessing_game v0.1.0 (/home/lennyp/rust-lang/guessing_game)
   Compiling libc v0.2.132
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.16
   Compiling getrandom v0.2.7
   Compiling rand_core v0.6.3
   Compiling rand_chacha v0.3.1
   Compiling rand v0.8.5
   Compiling guessing_game v0.1.0 (/home/lennyp/rust-lang/guessing_game/target/package/guessing_game-0.1.0)
    Finished dev [unoptimized + debuginfo] target(s) in 3.55s
   Uploading guessing_game v0.1.0 (/home/lennyp/rust-lang/guessing_game)
error: failed to publish to registry at https://crates.io

Caused by:
  the remote server responded with an error: missing or empty metadata fields: description, license. Please see https://doc.rust-lang.org/cargo/reference/manifest.html for how to upload metadata
```

此报错是由于咱们缺失了一些重要信息：描述及许可证是必须的，由此人们就会明白咱们的代码箱完成的什么，及在何种条件下他们可以使用他。在 `Cargo.toml` 中，由于代码箱的描述，会与咱们的代码箱一起呈现在搜索结果中，因此请添加仅仅一两句话的描述。而对于 `license` 字段，则需要提供 *某个许可证标识符值，a licence identifier value*。[Linux 基金会的软件包数据交换站，Linux Foundation's Software Package Data Exchange, SPDX，spdx.org](http://spdx.org/licenses/) 列出了可供这个值使用的标识符。比如，为指明咱们已使用 MIT 许可证，授权咱们的软件包，就要添加 `MIT` 的许可证标识符：


文件名：`Cargo.toml`

```toml
[package]
name = "guessing_game"
license = "MIT"
```

若咱们打算使用某个未出现于 SPDX 中的许可证，咱们就需要把那种许可证的文本，放置于某个文件里，把这个文件包含在咱们的项目中，并于随后使用 `license-file` 来指出那个文件的名字，而不再使用 `license` 键，the `license` key。

至于哪种许可证适合于咱们的项目方面的指南，是超出这本书的范围的。Rust 社区的许多人，都以 Rust 项目同样的方式，即采用 `MIT OR Apache-2.0` 双重许可证，授权他们的项目。这种实践表明，咱们也可以通过 `OR` 来指定出多个许可证标识符，从而让咱们的项目有着多种许可证。

在添加了独特名字、版本号、代码箱描述及许可证后，已准备好发布项目的 `Cargo.toml`文件，就会看起来像下面这样：

文件名：`Cargo.toml`

```toml
[package]
name = "guessing_game"
license = "MIT"
version = "0.1.0"
description = "一个在其中猜出计算机所选数字的有趣游戏。"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
rand = "0.8.3"
```

[Cargo 文档](https://doc.rust-lang.org/cargo/) 介绍了为确保其他人能更容易发现并使用咱们代码箱，而可指明的别的一些元数据。


## 发布到 Crates.io

既然咱们已经创建了账号，保存了 API 令牌，选择了代码箱名字，并指定了必需的元数据，那么咱们就准备好发布了！发布代码箱，会上传特定版本到 [crates.io](https://crates.io)，供其他人使用。

因为发布是 *永久性的，permanent*，因此要当心。版本绝无可能被覆盖，且代码无法被删除。[crates.io](https://crates.io) 的一个主要目标，是要充当代码的永久存档，以便依赖于 [crates.io](https://crates.io) 中代码箱的所有项目构建都将持续工作。而允许版本的删除，就会令到实现那个目标几无可能。不过，在咱们可发布的代码箱版本数目上没有限制。

再度运行 `cargo publish` 命令。现在他就应成功了：

```console
$ cargo publish                                                                        lennyp@vm-manjaro
    Updating crates.io index
warning: manifest has no documentation, homepage or repository.
See https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata for more info.
   Packaging guessing_game-xfossdotcom v0.1.0 (/home/lennyp/rust-lang/guessing_game)
   Verifying guessing_game-xfossdotcom v0.1.0 (/home/lennyp/rust-lang/guessing_game)
   Compiling libc v0.2.132
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.16
   Compiling getrandom v0.2.7
   Compiling rand_core v0.6.3
   Compiling rand_chacha v0.3.1
   Compiling rand v0.8.5
   Compiling guessing_game-xfossdotcom v0.1.0 (/home/lennyp/rust-lang/guessing_game/target/package/guessing_game-xfossdotcom-0.1.0)
    Finished dev [unoptimized + debuginfo] target(s) in 2.73s
   Uploading guessing_game-xfossdotcom v0.1.0 (/home/lennyp/rust-lang/guessing_game)
```

恭喜！现在咱们就已与 Rust 社区分享了咱们的代码，且任何人都可将咱们的代码箱，添加为他们项目的依赖。

> 注：在 Crates.io 上的账号电子邮箱未验证时，将报出如下错误：

```console
Caused by:
  the remote server responded with an error: A verified email address is required to publish crates to crates.io. Visit https://crates.io/me to set and verify your email address.
```


## 发布既有代码箱的新版本


咱们完成咱们代码箱的修改，而准备好发布新版本时，咱们要修改 `Cargo.toml` 中所指定的 `version` 值并重新发布。请运用 [语义版本控制规则，Semantic Versioning rules](http://semver.org/)，根据咱们已做出修改的类别，来确定出恰当的下一版本编号为何。然后运行 `cargo publish` 来上传新版本。


## 使用 `cargo yank` 命令弃用 Crates.io 上的版本

**Depracating Versions from Crates.io with `cargo yank`**


尽管咱们无法移除代码箱的先前版本，但咱们可以阻止任何今后的项目，将其添加为新的依赖项。这在某个代码箱版本由于某种原因，或别的问题而损坏时是有用的。在诸如此类的情形下，Cargo 支持把某个代码箱版本 *抽出来*，in such situations, Cargo supports *yanking* a crate version。

抽出某个版本，在允许所有依赖该版本的既有项目继续工作的同时，会阻止新项目依赖那个版本。本质上，一次版本抽出，表示带有 `Cargo.lock` 的全部项目不会破坏，而任何今后生成的 `Cargo.lock` 文件，都将不使用被抽出的版本。

要抽出代码箱的某个版本，就要在咱们先前已发布的代码箱目录中，运行 `cargo yank` 并指定出要抽出的版本。比如，咱们曾发布了名为 `guessing_game` 代码箱的 `0.1.0` 版本，而打算抽出他，咱们就要在 `guessing_game` 的项目目录下，运行下面的命令：

```console
$ cargo yank --vers 0.1.0                                                           4s lennyp@vm-manjaro
    Updating crates.io index
        Yank guessing_game-xfossdotcom@0.1.0
```

通过把 `--undo` 添加到这个命令，咱们还可以撤销某次抽出，而允许项目开始再度依赖于某个版本：

```console
$ cargo yank --vers 0.1.0 --undo                                                    lennyp@vm-manjaro
    Updating crates.io index
      Unyank guessing_game-xfossdotcom@0.1.0
```

抽出版本，*不会* 删除任何代码。比如，其无法删除那些不小心上传的机密信息。若发生了机密信息被上传的情况，咱们必须立即重置这些机密信息。


（End）


