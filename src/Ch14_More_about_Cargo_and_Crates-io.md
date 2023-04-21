# Cargo 的其他方面与 Crates.io

**More About Cargo and Crates.io**

到目前为止，咱们只使用了 Cargo 的一些最基本特性，来构建、运行与测试所编写的代码，而 Cargo 可以完成多得多的事情。本章中，咱们将讨论他一些别的、更为先进的特性，来展示如何完成以下的这些事情：

- 经由不同发布配置文件，定制咱们的构建，customize your build through release profiles；
- 把库发布在 [crates.io](https://crates.io) 上；
- 使用工作区来组织大型项目，organize large projects with workspaces;
- 从 [crates.io](https://crates.io) 安装库；
- 使用定制命令来扩展 Cargo 。


相比咱们在本章会讲到的功能，Cargo 甚至能完成更多，因此对于 Cargo 全部特性的完整阐释，请参阅 [他的文档](https://doc.rust-lang.org/cargo/)。


## 使用不同发布配置文件，对构建进行定制

**Customizing Builds with Release Profiles**

在 Rust 中，所谓 *发布配置文件，release profiles*，是带有实现程序员对编译代码有着更多掌控的，一些预定义及可定制的配置文件。相对其他配置文件，每个配置文件都是被独立配置的。

Cargo 有两个主要发布配置文件：运行 `cargo build` 时 Cargo 用到的 `dev` 配置文件，与运行 `cargo build --release` 时 Cargo 用到的 `release` 配置文件。`dev` 配置文件被定义为有着用于开发的一些良好默认配置，而 `release` 配置文件有着用于发布构建的良好默认配置。

从咱们构建的输出中，这些配置文件名字或许不陌生：

```console
$ cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
$ cargo build --release
    Finished release [optimized] target(s) in 0.0s
```

其中 `dev` 及 `release`，即由编译器用到的不同配置文件。

Cargo 有着在咱们在项目的 `Cargo.toml` 文件中，未曾显式添加任何 `[profile.*]` 小节时，所适用的各个配置文件的默认设置。通过添加咱们打算定制的任何配置文件的 `[profile.*]` 小节，咱们就会覆盖掉默认设置的任何子集。比如，下面是 `dev` 与 `release` 配置文件中 `opt-level` 设置的默认值：

文件名：`Cargo.toml`

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

这个 `opt-level` 设置项，控制了 Rust 将应用到咱们代码的优化数目，有着范围 `0` 到 `3` 的取值范围。运用更多优化，会延长编译时间，因此在开发过程中而频繁编译代码时，就会想要更少的优化，来更快地编译，即使产生出的代码运行较慢。因此这默认的 `opt-level` 就是 `0`。而在已准备好发布时，那么就最好用更多时间来编译。咱们将只以发布模式编译一次，但会多次运行那已编译好的程序，因此发布模式就以较长的编译时间，换取到运行较快的代码。那就是 `release` 配置文件的 `opt-level` 默认为 `3` 的原因。

通过在 `Cargo.toml` 中，给某个默认值添加不同的值，就可以覆盖掉这个默认值。比如，在打算于开发配置文件中使用优化级别 `1` 时，就可以把下面这两行，添加到项目的 `Cargo.toml`：

文件名：`Cargo.toml`

```toml
[profile.dev]
opt-level = 1
```

此代码就覆盖了默认的设置 `0`。现在运行 `cargo build`，Cargo 将使用 `dev` 配置文件的那些默认设置，加上这里对 `opt-level` 的定制修改。由于这里把 `opt-level` 设置为了 `1`，Cargo 就会应用相比于默认设置更多，但并不如发布构建那样多的优化。

若要了解这两个配置文件的完整配置项清单及他们的默认设置，请参阅 [Cargo 文档](https://doc.rust-lang.org/cargo/reference/profiles.html)。


## 将代码箱发布到 Crates.io

**Publishing a Crate to Crates.io**

前面在项目中，已经用到了 [crates.io](https://crates.io) 上的一些包，然而通过发布咱们自己的包，还可以与其他人分享咱们自己的代码。位于 [crates.io](https://crates.io) 网站的代码箱登记处，会分发咱们的包，因此 crates.io 主要保存了开放源码的代码。

Rust 与 Cargo，均有着令到咱们所发布的包，易于为其他人找到并使用的一些特性。接下来就会聊聊一些这样的特性，并讲解怎样发布某个包（how to release a package）。


### 制作一些有用的文档注释

**Making Useful Documentation Comments**

准确地为咱们的包编写文档，将帮助到其他使用者获悉怎样及何时来使用咱们的包，因此投入时间来编写文档是值得的。第 3 章中，就曾讨论过如何使用双斜杠 `//`，来注释 Rust 代码。Rust 还有用于文档的一种特殊注释，通常被称作 *文档注释（documentation comment）*，此类注释将产生出 HTML 文档。这些生成的 HTML，会将给那些想要了解怎样 *使用（use）* 咱们的代码箱，而不是咱们代码箱如何实现的程序员，准备的公开 API 的文档注释，给显示出来。

文档注释用的是三斜杠 `///`，而不是双斜杠，并支持用于格式化文本的 Markdown 写法。要把文档注释，放在他们要注释项目之前，紧接着注释项目。下面清单 14-1 给出了名为 `cargo_features_demo` 代码箱中，一个 `add_one` 函数的文档注释。

文件名：`src/lib.rs`

````rust
/// 将一加到所给数字。
/// # 示例（examples）
///
/// ```
/// let arg = 5;
/// let answer = cargo_features_demo::add_one(arg);
///
/// assert_eq! (6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
````

*清单 14-1：一个函数的文档注释*

这里对该 `add_one` 函数完成了什么进行了描述，以标题 `示例` 开始了一个小节，随后提供了演示如何使用这个 `add_one` 函数的代码。通过运行 `cargo doc` 命令，就可以生成该文档注释的 HTML 文档。`cargo doc` 命令会运行与 Rust 一起分发的 `rustdoc` 工具，并将生成的 HTML 文档，放在 `target/doc` 目录中。

为方便起见，运行 `cargo doc --open` 将构建出当前代码箱文档（以及全部代码箱依赖的文档）的 HTML，并于随后在 web 浏览器中打开得到的结果。导航到那个 `add_one` 函数，就会看到文档注释中的文本，是如何被渲染的，如下图片 1401 中所示：

![`add_one` 函数的 HTML 文档](images/14-01.png)

*图 14-01：`add_one` 函数的 HTML 文档*

#### 经常用到的一些小节

这里用到了清单 14-1 中 `# 示例（examples）` 的 Markdown 标题，来创建出生成 HTML 中，有着标题 “示例（examples）” 的一个小节。下面是代码箱编写者经常在他们文档中，用到的一些别的小节：

- **终止运行（Panics）**：正被文档注释的函数可能终止运行的情形。不愿他们的程序终止运行的那些调用者，应确保他们不会在这些情形下调用该函数；
- **报错（Errors）**：在该函数返回的是一个 `Result` 时，那么对可能发生的各种错误及何种条件下会引起这些错误被返回进行描述，就能有效帮助到调用者，以便他们可以编写出以不同方式，处理这些不同类别错误的代码来。
- **安全性（Safety）**：在该函数属于 `unsafe` 的调用时（在后面第 19 章会讨论到不安全 `unsafe`），就应有解释为何该函数属于不安全，以及对该函数所期望的调用者要坚守哪些不变因素进行说明一个小节（if the funciton is `unsafe` to call(we discuss unsafety in Chapter 19), there should be a section explaining why the function is unsafe and covering the invariants that the function expects callers to uphold）。


多数的文档注释，并不需要全部的这些小节，但这仍不失为提醒咱们，代码使用者将有兴趣了解咱们代码哪些方面的一个不错的检查单。


#### 作为测试的文档注释

**Documentation Comments as Tests**

在文档注释中添加一些代码块，可有助于演示怎样使用咱们的库，而这样做有着一项额外收获（an additional bonus）：运行 `cargo test` 将以测试方式，运行文档中的那些代码示例！没有什么比带有示例的文档更好的了。然而比起由于在文档写好后，代码已被修改而造成的示例不工作，也没有什么更糟糕的了。在清单 14-1 中 `add_one` 函数的文档下，运行 `cargo test` 时，就会在测试结果中看到这样一个小节：

```console
   Doc-tests cargo_features_demo

running 1 test
test src/lib.rs - add_one (line 7) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.15s
```

而现在要么修改那个函数，要么修改那个示例，总之要让示例中的 `assert_eq!` 终止运行，并再次运行 `cargo tset`，就会看到文档测试（the doc tests）捕获了那个示例，同时 `add_one` 的实现代码，与文档注释中的代码，便失去了相互的同步！

> **注**：此状况下的输出为：

```console
   Doc-tests cargo_features_demo

running 1 test
test src/lib.rs - add_one (line 7) ... FAILED

failures:

---- src/lib.rs - add_one (line 7) stdout ----
Test executable failed (exit status: 101).

stderr:
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `6`,
 right: `7`', src/lib.rs:7:1
stack backtrace:
   0:     0x5620cf499480 - std::backtrace_rs::backtrace::libunwind::trace::h32eb3e08e874dd27
                               at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/../../backtrace/src/back                             trace/libunwind.rs:93:5
   // ...
  36:                0x0 - <unknown>



failures:
    src/lib.rs - add_one (line 7)

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.15s

error: doctest failed, to rerun pass `--doc`
```

> **注**：执行 `cargo test --doc`，将只运行文档注释中的示例代码。

**对其所在的程序项目进行文档注释（Commenting Contained Items）**

`//!` 这种样式的文档注释，会把文档添加到包含注释的条目，而非那些跟随这些注释的条目。在代码箱根文件中（依惯例即 `src/lib.rs`），或在对代码箱编写文档的模组，抑或在作为整体的模组中，通常会用到这些文档注释（the style of doc comment `//!` adds documentation to the item contains the comments rather than to the items following the comments. We typically use these doc comments inside the crate root file(`src/lib.rs` by convention) or inside a module to document the crate or the module as a whole）。

比如，为添加对包含 `add_one` 函数的这个 `cargo_features_demo` 代码箱的目的加以描述的文档，这里就要把以 `//!` 开头的一些文档注释，添加到那个 `src/lib.rs` 文件的开头，如下清单 14-2 中所示：

文件：`src/lib.rs`

```rust
//! # Cargo 特性示例代码箱
//!
//! `cargo_features_demo` 是令到执行某些确切计算更便利
//! 的一些工具的集合。
//!

/// 将一加到所给数字。
// --跳过代码--
```

*清单 14-2：整体下 `cargo_features_demo` 代码箱的文档*

请注意由于这里是以 `//!` 而非 `///` 开始的这些注释，因此在以 `//!` 开始的最后一行之后，是没有任何代码的，这里是在给包含了此注释的程序项目，而非紧接着此注释的程序项目编写文档。在此示例中，那个程序项目就是 `src/lib.rs` 文件，这正是代码箱根。这些注释描述了整个的代码箱。

在运行 `cargo doc --open` 时，这些注释就会显示在 `cargo_features_demo` 代码箱文档的首页（the front page），位于该代码箱那些公开项目清单之上，如下图 14-02 中所示：

![渲染后的 `cargo_features_demo` 代码箱的文档](images/14-02.png)

*图 14-02：渲染后的 `cargo_features_demo` 代码箱的文档, 包括了将该代码箱作为整体进行描述的注释*

程序项目里的文档注释，用于对代码箱及模组等进行描述，尤其有用。使用它们来解释该容器（the container） 的整体目标，就有助于咱们代码箱的使用者，理解该代码箱的组织。


### 使用 `pub use` 导出好用的公开 API

**Exporting a Convinient Public API with `pub use`**

在发布某个代码箱时，其公开 API 结构是主要的考量。用到咱们代码箱的人们，相比咱们自己，对代码箱结构的没有那么熟悉，进而在咱们的代码箱有着大型模组层次结构时，就会难于找到他们打算使用的部分。

在第 7 章中，就曾讲到过怎样使用 `pub` 关键字，把一些程序项目构造为公开，以及使用 `use` 关键字，把一些程序项目带入到某个作用域。尽管如此，在开发某个代码箱过程中，对咱们有意义的组织结构（模组树），对于咱们的用户，就可能不那么便利。咱们可能打算把代码箱结构，组织为包含多个级别的层次，但随后打算使用已被定义在该层次结构深处某个类型的人，就可能在查明那个类型是否存在上，遇到麻烦。他们可能还会对必须敲入 `use cargo_features_demo::some_module::another_module::UsefulType;`，而非敲入 `use cargo_features_demo::UsefulType;` 而感到恼火。

可喜的是，在代码箱组织结构 *不* 便于其他人在别的库中使用时，咱们并不必须重新调整代码箱的内部组织：相反，咱们可通过使用 `pub use`，重新导出程序项目，而构造出一种不同于咱们私有组织结构的公开组织结构。重新导出（re-export）会取位于某处的一个公开程序项目，并在另一处将其构造为公开，就跟这个项目是在这另一处被定义过一样。

比如，假设这里构造了用于对一些美术概念建模的一个名为 `art` 的库。这个库里头有两个模组：包含了两个分别名为 `PrimaryColor` 与 `SeccondaryColor` 枚举的 `kinds` 模组与包含了一个名为 `mix` 函数的 `utils` 模组，如下清单 14-3 中所示：

文件名：`src/lib.rs`

```rust
//! # 美术
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

*清单 14-3：带有组织在 `kinds` 与 `utils` 模组中一些程序项目的 `art` 库*

下图 14-03 给出了由 `cargo doc` 产生出的该代码箱文档首页的样子：

![列出 `kinds` 与 `utils` 两个模组的 `art` 代码箱文档首页](images/14-03.png)

*图 14-3：列出 `kinds` 与 `utils` 两个模组的 `art` 代码箱文档首页*


请注意 `PrimaryColor` 与 `SecondaryColor` 两个类型，并未在首页上列出，那个 `mix` 函数也没有。要看到他们，就必须点击 `kinds` 与 `utils`。

依赖于这个库的另一代码箱，就需要使用将一些将程序项目从 `art` 带入到作用域的 `use` 语句，指定当前所定义的这种模组结构。下面清单 14-4 给出了用到这个 `art` 代码箱中 `PrimaryColor` 与 `mix` 两个项目的代码箱示例：

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

*清单 14-4：以内部组织结构导出方式下，使用 `art` 代码箱的程序项目*

> **注**：使用本地未发布代码箱的方法，是在 `Cargo.toml` 的 `[dependencies]` 小节中，要使用的本地未发布代码箱列出。参见 [How to use a local unpublished crate?](https://stackoverflow.com/a/33025972)

文件：`Cargo.toml`

```toml
// --跳过代码--

[dependencies]
art = { path = "../art" }
```

清单 14-4 中，使用了 `art` 代码箱代码的作者，就必须找出那个 `PrimaryColor` 是在 `kinds` 模组里，以及 `mix` 函数是在 `utils` 模组里。那个 `art` 代码箱的模组结构（即模组树），和要用到该代码箱的开发者相比，与那些在 `art` 代码箱上编写代码的开发者要更为密切。对于那些试图搞清楚怎样使用这个 `art` 代码箱的人来说，他的内部组织结构并未包含任何有用信息，而由于用到他开发者，必须搞明白要在那里去查看，并必须在 `use` 语句中指明那些模组名字，因此他的内部组织结构，反而会造成混乱。

要从公开 API 中消除内部的组织结构，咱们可以将清单 14-3 中那个 `art` 代码箱的代码，修改为在其顶层出添加上一些 `pub use` 语句，来重新导出那些程序项目，如下清单 14-5 中所示：

文件名：`src/lib.rs`

```rust
//! # 美术
//!
//! 建模诸多美术概念的一个库。

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds;
pub mod utils;
```

*清单 14-5：添加一些 `pub use` 语句来重新导出程序项目*

如下图 14-04 中所示，`cargo doc` 为此代码箱所产生出的 API 文档，现在就会在首页上，列出并链接到那些重导出的程序项目，令到 `PrimaryColor` 与 `SecondaryColor` 两个类型，以及那个 `mix` 函数更易于找到。

![列出了重导出项目的 `art` 代码箱文档首页](images/14-04.png)

*图 14-4：列出了重导出项目的 `art` 代码箱文档首页*

这个 `art` 代码箱的用户，仍然能象下面清单 14-4 中所演示的那样，看到并使用清单 14-3 中的内部结构，抑或他们可以使用清单 14-5 中那种更为便利的结构，如下清单 14-6 中所示：

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

*清单 14-6：使用 `art` 代码箱中那些重导出程序项目的程序*

在那些其中有着许多嵌套模组的情形下，于代码箱结构的顶部，使用 `pub use` 重导出一些类型，可在那些用到该代码箱的人体验上，造成显著不同。`pub use` 的另一常见用途，则是重导出当前代码箱中某项依赖的一些定义，而把那个依赖代码箱的一些定义，构造为咱们自己代码箱的公开 API。

创建出一种有用的公开 API 结构，与其说是一门科学，不如说是一门艺术，同时可以不断迭代，来找出对于代码箱用户运作最佳的那种 API。而选择 `pub use` 则会给到在内部组织代码箱方式上的灵活性，同时解偶了内部结构，与呈现给代码箱用户的组织结构。请查看曾安装过的一些代码箱的代码，来检视他们内部结构，是否不同于他们的公开 API。


### 建立一个 Crates.io 帐号

在能够发布代码箱之前，咱们需要在 [crates.io](https://crates.io) 上创建一个帐号，并获取到一个 API 令牌（an API token）。而要这样做，就要访问 [crates.io](https://crates.io) 处的主页，并经由一个 GitHub 帐号登录。（GitHub 帐号目前是必须的，但该站点后面可能会支持创建帐号的其他途径。）在登录之后，就要访问 [https://crates.io/me/](https://creates.io/me/) 处的帐号设置，并找回自己的 API 密钥（API key）。然后使用这个 API 密钥，运行 `cargo login` 命令，如下：

```console
$ cargo login abcdefghijklmnopqrstuvwxyz012345
```

此命令将告知 Cargo 咱们的 API 令牌，并将其存储在本地的 `~/.cargo/credentials` 文件中。请注意此令牌是个 *秘密（secret）*：不要与其他任何人分享这个秘密。不论以何种方式，让任何人知道了API 令牌，那么就都应该吊销这个 API 令牌，并在 [crates.io](https://crates.io) 上重新生成一个新的令牌。

### 给新的代码箱添加元数据

**Adding Metadata to a New Crate**

好比说有个打算发布的代码箱。在发布之前，就将需要在该代码箱的 `Cargo.toml` 文件的 `[package]` 小节中，添加一些元数据。

咱们的代码箱将需要一个独特的名字。尽管在本地编写某个代码箱时，可以给代码箱取任意喜欢的名字。但是，[crates.io](https://crates.io) 上代码箱的名字，则是以先到先得的原则分配的（allocated on a first-come, first-served basis）。一旦某个代码箱名字被使用，其他人就不能发布有着那个名字的代码箱了。在尝试发布某个代码箱之前，就要检索一下咱们打算使用的那个名字。在这个名字已被使用时，就需要找到另一个名字，并编辑 `Cargo.toml` 文件中 `[package]` 小节下的 `name` 字段，以在使用那个用于发布的新名字，就像下面这样：

文件名：`Cargo.toml`

```toml
[package]
name = "guessing_game"
```

即使咱们以及选出了一个独特的名字，在此时运行 `cargo publish` 来发布这个代码箱时，仍将得到一条告警并接着一个报错：


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

这些错误是由于咱们缺失了一些重要信息：代码箱的描述及许可证是必须的，由此人们就会明白咱们的代码箱完成的什么，以及在何种条件下他们可以使用咱们的代码箱。在 `Cargo.toml` 中，由于代码箱的描述，会与咱们的代码箱一起呈现在搜索结果中，因此请添加仅仅一两句话的描述。而对于那个 `license` 字段，则需要提供 *某个许可证标识符的值（a licence identifier value）*。[Linux 基金会的软件包数据交换站（Linux Foundation's Software Package Data Exchange, SPDX），spdx.org](http://spdx.org/licenses/) 就列出了可供这个值使用的那些标识符。比如，为指明咱们已使用 MIT 许可证，授权咱们的软件包，那么就要添加那个 `MIT` 的许可证标识符：


文件名：`Cargo.toml`

```toml
[package]
name = "guessing_game"
license = "MIT"
```

在打算使用某个未出现于 SPDX 中的许可证时，那么就需要把那种许可证的文本放置于某个文件里，把这个文件包含在咱们的项目中，并于随后使用 `license-file` 而非 `license` 键（the `license` key），来指出那个文件的名字。

有关哪种许可证适合于你的项目方面的指南，是不在这本书的范围的。Rust 社区的许多人，都以 Rust 项目同样的方式，即采用 `MIT OR Apache-2.0` 双重许可证，授权他们的项目。这种项目授权的实践，表明咱们是也可以通过 `OR`，来指定多个许可证标识符，从而让咱们的项目有着多种许可证。

有了一个独特的名字、版本号、代码箱描述，并添加了某个许可证，那么这个准备好发布项目的 `Cargo.toml`，就会看起来像下面这样的：

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

[Cargo 文档](https://doc.rust-lang.org/cargo/) 介绍了可以指定来确保其他人，能更容易发现并使用你的代码箱的其他元数据。


### 发布到 Crates.io

既然前面已经创建了账号，保存了 API 令牌，选择了代码箱的名字，并指定了必需的元数据，那么就准备好发布了！发布某个代码箱，就会上传某个特定版本到 [crates.io](https://crates.io)，供其他人使用。

因为发布是 *永久性的（permanent）*，因此要当心。其中的版本绝无可能被覆盖，同时代码无法删除。[crates.io](https://crates.io) 的一个主要目标，是要充当代码的永久存档，以便依赖 [crates.io](https://crates.io) 处代码箱的全部项目构建，将持续工作。允许版本删除，就会领导实现那个目标几无可能。好在咱们可发布的代码箱版本数目上，没有限制。

再度运行这个 `cargo publish` 命令。他现在就应成功了：

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

祝贺！现在咱们与 Rust 社区，分享了咱们的代码，同时其他人就可以将咱们的代码箱，作为他们项目的一项依赖而加以添加了。

> **注**：在 Crates.io 上的账号电子邮箱未验证时，将报出如下错误：

```console
Caused by:
  the remote server responded with an error: A verified email address is required to publish crates to crates.io. Visit https://crates.io/me to set and verify your email address.
```

### 发布既有代码箱的新版本

在修改了咱们的代码箱，并准备好发表一个新版本时，就要修改那个于 `Cargo.toml` 中所指明的 `version` 值并重新发布。请使用 [语义版本控制规则（Semantic Versioning rules）](http://semver.org/) 来依据曾作出的修改类别，确定出适当的下一版本编号。然后就运行 `cargo publish` 来上传这个新版本。


### 使用 `cargo yank` 弃用 Crates.io 上的一些版本

**Depracating Versions from Crates.io with `cargo yank`**

尽管咱们无法移除某个代码箱的一些先前版本，但咱可以阻止任何今后的项目，将他们添加作新的依赖项。在某个代码箱版本由于某种原因或别的问题而损坏时，这样做是有用的。在诸如此类的情形中，Cargo 是支持将某个代码箱版本 *抽出来* 的（in such situations, Cargo supports *yanking* a crate version）。

抽出某个版本，就会在允许所有依赖该版本的项目继续工作的同时，阻止新的项目依赖那个版本。本质上，一次版本抽出，就表示有着 `Cargo.lock` 的全部项目不会破坏，同时任何今后生成的 `Cargo.lock` 文件，都不会使用这个被抽出的版本了。

要抽出代码箱的某个版本，就要在先前已发布的那个代码箱目录中，运行 `cargo yank` 并指定要抽出哪个版本。比如，在曾发布了一个名为 `guessing_game` 代码箱的 `0.1.0` 版本，并打算抽出他时，那么就要在 `guessing_game` 的项目目录下，运行下面的命令：

```console
$ cargo yank --vers 0.1.0                                                           4s lennyp@vm-manjaro
    Updating crates.io index
        Yank guessing_game-xfossdotcom@0.1.0
```

通过添加 `--undo` 到这个命令，咱们还可以撤销某次抽出，而运行一些项目再度开始依赖于某个版本：

```console
$ cargo yank --vers 0.1.0 --undo                                                    lennyp@vm-manjaro
    Updating crates.io index
      Unyank guessing_game-xfossdotcom@0.1.0
```

抽出某个版本，*不会* 删除任何代码。比如，此操作就无法删除那些不小心上传的机密信息。若发生了机密信息被上传的情况，那么就必须立即重置这些机密信息。


## Cargo 工作区

**Cargo Workspaces**

在第 12 章中，曾构建了包含一个二进制代码箱和一个库代码箱的包（a package）。随着项目的不可开发，就会发现那个库代码箱会持续变大，而咱们就会想要将咱们的包，进一步拆分为多个库代码箱。Cargo 提供了叫做 *工作区（workspace）* 的特性，可帮助管理多个先后开发的相关包。

> ***注***：总结 Rust 开发的层次结构如下：工作区（workspace） -> 包（package） -> 代码箱（crate） -> 模组（module） -> 语句（statement）。

### 创建工作区

*工作区*（a *workspace*）是共享了同一 `Cargo.lock` 文件及输出目录的一个包集合。下面就来构造一个用到工作区的项目 -- 这里将使用一些简单代码，这样咱们就可以着重于该工作区的结构上。组织工作区有多种方式，因此这里将只给出一种常用的方式。这里将有着包含一个二进制代码箱，及两个库代码箱的工作区。其中的二进制代码箱，将提供依赖于那两个库代码箱的 `main` 功能。其中一个库代码箱，将提供一个 `add_one` 函数，而另一个则会提供 `add_two` 函数。这三个代码箱，都将是同一工作区的组成部分。这里将以创建该工作区的目录开始：

```console
$ mkdir add
$ cd add
```

接着，在那个 `add` 目录中，就要创建一个将对整个工作区加以配置的 `Cargo.toml` 文件了。这个文件不会有 `[package]` 小节。相反，他会以一个 `[workspace]` 小节打头，这将允许咱们通过指定有着这里二进制代码箱的那个包的路径，而把一些成员添加到这个工作区；在此情形下，那个路径就是 `adder`:

文件名：`Cargo.toml`

```toml
[workspace]
members = [
    "adder",
]
```

再接着，这里将通过在这个 `add` 目录里头，运行 `cargo new` 创建出那个 `adder` 二进制代码箱：

```console
$ cargo new adder
     Created binary (application) `adder` package
```

到这里，就可以通过运行 `cargo build`，构造这个工作区了。这个 `add` 目录下的那些文件，看起来应像下面这样：

```console
.
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
├── Cargo.lock
├── Cargo.toml
└── target
```

这个工作区在顶级有个 `target` 目录，那些编译好的物件（the compiled artifacts）就会放入到其中；那个 `adder` 包则并无其自己的 `target` 目录。即使在 `adder` 目录内部运行 `cargo build`，那些编译出的物件，仍会以位处于 `add/target` 而告终，而不会在 `add/adder/target` 目录里。Cargo 之所以像这样来架构这个 `target` 目录，是因为工作区中的那些代码箱，是为了依赖于彼此。若各个代码箱都有其自己的 `target` 目录，那么各个代码箱为了把编译成的物件放在自己的 `target` 目录中，而不得不重新编译工作区中的各个其他代码箱。通过共用一个 `target` 目录，这些代码箱就可以避免不必要的重构建。


### 创建工作区中的第二个包

**Creating the Second Package in the Workspace**

接下来，就要创建出工作区中的另一个成员包，并将其叫做 `add_one`。请修改顶层的 `Cargo.toml`，在其中的 `members` 清理里指明 `add_one` 的路径：

文件名：`Cargo.toml`

```toml
[workspace]

members = [
    "adder",
    "add_one",
]
```

随后生成一个名为 `add_one` 的新库代码箱：

```console
$ cargo new add_one --lib                                                                        lennyp@vm-manjaro
     Created library `add_one` package
```

这个 `add` 目录现在应该有这些目录与文件：

```console
.
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
├── add_one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── Cargo.lock
├── Cargo.toml
└── target
```

在那个 `add_one/src/lib.rs` 文件中，给添加一个 `add_one` 函数：

文件名：`add_one/src/lib.rs`

```rust
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

现在咱们就可以让有着这里二进制代码箱的 `adder` 包，依赖于这个有着这里的库的 `add_one` 包了。首先，这里将需要把有关 `add_one` 的路径依赖（a path dependency），添加到 `adder/Cargo.toml`。

文件名：`adder/Cargo.toml`

```toml
[dependencies]
add_one = { path = "../add_one" }
```

Cargo 不会假定工作区中的代码箱将各自相互依赖，因此这里需要显示说明这些依赖关系。

接下来，就要在 `adder` 代码箱中，使用那个 `add_one` 函数（位于 `add_one` 代码箱中）。打开 `adder/src/main.rs` 文件，并在其顶部使用一行 `use`，带入那个新的 `add_one` 库代码箱到作用域中。随后修改其中的 `main` 函数，来调用那个 `add_one` 函数，如下清单 14-7 中所示。

文件名：`adder/src/main.rs`

```rust
use add_one::add_one;

fn main() {
    let num = 10;
    println!("你好，世界！{num} 加一为 {}!", add_one(num));
}
```

*清单 14-7：在 `adder` 代码箱中使用 `add_one` 库代码箱*

下面就来通过在 `add` 目录的顶层，运行 `cargo build` 构建出这个工作区！

```console
$ cargo build                                                                                 lennyp@vm-manjaro
   Compiling add_one v0.1.0 (/home/lennyp/rust-lang/add/add_one)
   Compiling adder v0.1.0 (/home/lennyp/rust-lang/add/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 0.40s
```

要在 `add` 目录运行这个二进制代码箱，是可以通过使用 `cargo run` 的 `-p` 命令行参数及包名字，指定出要运行工作区中哪个包的：

```console
$ cargo run -p adder                                                                          lennyp@vm-manjaro
   Compiling adder v0.1.0 (/home/lennyp/rust-lang/add/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 0.35s
     Running `target/debug/adder`
你好，世界！
        10 加 1 为 11!
```

这会运行 `adder/src/main.rs` 中的代码，其依赖于那个 `add_one` 代码箱。


### 于工作区中依赖外部代码箱

**Depending on an External Package in a Workspace**

请注意工作区在其顶层，只有一个 `Cargo.lock` 文件，而非在各个代码箱目录中都有 `Cargo.lock`。这确保了工作区的全部代码箱，都使用着同一版本的所有依赖。若这里把 `rand` 包分别添加到 `adder/Cargo.toml` 及 `add_one/Cargo.toml` 文件，Cargo 就会就这两个版本的 `rand` 加以解析，并将其记录在这一个的 `Cargo.lock` 中。

令到工作区中全部代码箱使用同样的那些依赖，就意味着这些代码箱将始终相互兼容。下面就来把 `rand` 代码箱添加到 `add_one/Cargo.toml` 文件的 `[dependencies]` 小节，从而就可以在 `add_one` 代码箱中使用这个 `rand` 代码箱：

文件名：`add_one/Cargo.toml`

```toml
rand = "0.8.3"
```

现在就可以添加 `use rand;` 到 `add_one/src/lib.rs` 文件了，而通过在 `add` 目录运行 `cargo build` 构建这整个工作区，就会带入并编译那个 `rand` 代码箱。由于这里并未引用那个已带入到作用域中的 `rand`，因此这里会收到一条告警：

```console
$ cargo build                                                                                 lennyp@vm-manjaro
    Updating crates.io index
  Downloaded rand_core v0.6.4
  Downloaded ppv-lite86 v0.2.17
  Downloaded getrandom v0.2.8
  Downloaded libc v0.2.137
  Downloaded 4 crates (681.6 KB) in 1.29s
   Compiling libc v0.2.137
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.17
   Compiling getrandom v0.2.8
   Compiling rand_core v0.6.4
   Compiling rand_chacha v0.3.1
   Compiling rand v0.8.5
   Compiling add_one v0.1.0 (/home/lennyp/rust-lang/add/add_one)
warning: unused import: `rand`
 --> add_one/src/lib.rs:1:5
  |
1 | use rand;
  |     ^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: `add_one` (lib) generated 1 warning

   Compiling adder v0.1.0 (/home/lennyp/rust-lang/add/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 6.76s
```

那个顶层的 `Cargo.lock`，现在就包含了有关 `add_one` 对 `rand` 的依赖信息。但是，及时 `rand` 在该工作区中的某处被用到，在未将 `rand` 添加到其他代码箱的 `Cargo.toml` 文件之前，是不能在其他代码箱中使用他的。比如，若这里把 `use rand;` 添加到 `adder` 包的 `adder/src/main.rs` 文件，就会得到一个报错：

```console
$ cargo build                                                                                 lennyp@vm-manjaro
   --跳过前面的告警--
   Compiling adder v0.1.0 (/home/lennyp/rust-lang/add/adder)
error[E0432]: unresolved import `rand`
 --> adder/src/main.rs:1:5
  |
1 | use rand;
  |     ^^^^ no external crate `rand`

For more information about this error, try `rustc --explain E0432`.
error: could not compile `adder` due to previous error
```

要修正整个错误，就要编辑 `adder` 包的 `Cargo.toml` 文件，并也表明 `rand` 是其的一个依赖项。构建这个 `adder` 包就会把 `rand`，添加到 `Cargo.lock` 中 `adder` 的依赖项清单，但不会有额外的 `rand` 拷贝会被下载。Cargo 已确保工作区中用到这个 `rand` 包每个包中的每个代码箱，都将使用同一版本，从而节省了空间，并确保了工作区中的那些代码箱都将兼容于彼此。


### 添加测试到工作区

**Adding a Test to a Workspace**

为说明另一项改进，下面来添加 `add_one` 代码箱里头，`add_one::add_one` 函数的一个测试：

文件名：`add_one/src/lib.rs`

```rust
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_one(2);
        assert_eq!(result, 3);
    }
}
```

此时在顶层的 `add` 目录运行 `cargo test`。在像这样组织的工作区中运行 `cargo test`，就会运行工作区中全部代码箱的那些测试：

```console
$ cargo test                                                                                                           lennyp@vm-manjaro
   Compiling add_one v0.1.0 (/home/lennyp/rust-lang/add/add_one)
   Compiling adder v0.1.0 (/home/lennyp/rust-lang/add/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.68s
     Running unittests src/lib.rs (target/debug/deps/add_one-837c2ad0efe6b80c)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/adder-2277ab1084738161)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

输出的首个部分，显示 `add_one` 代码箱中那个 `it_works` 测试通过了。接下来的小节显示，在 `adder` 代码箱中找到零个测试，而随后那个最后小节，显示在 `add_one` 代码箱中找到零个文档测试。（*注*：二进制代码箱中不会有文档测试？）

这里还可以通过使用 `-p` 命令行标志及指定要测试的代码箱名字，在顶层目录处运行工作区中某个特定代码箱的那些测试：


```console
$ cargo test -p add_one                                                                                                lennyp@vm-manjaro
    Finished test [unoptimized + debuginfo] target(s) in 0.01s
     Running unittests src/lib.rs (target/debug/deps/add_one-837c2ad0efe6b80c)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

此输出展示了只运行那个 `add_one` 代码箱测试，而未运行 `adder` 代码箱测试的 `cargo test`。

在将工作区中的代码箱发布到 `crates.io` 时，工作区中的各个代码箱，则将需要被单独发布。与 `cargo test` 类似，可通过使用 `-p` 命令行标志，并指明打算发布的那个代码箱名字，发布工作区中的某个特定代码箱。

作为附加练习，请以与 `add_one` 代码箱类似方式，把 `add_two` 添加到这个工作区！

在项目日渐增长时，就要考虑使用工作区：相比于一大块代码，搞明白较小的、单独的组件，要容易一些。再者，把代码箱保持在一个工作区中，当工作区中的那些代码箱经常同时被修改时，就能令到他们之间的协作更容易。


## 使用 `cargo install` 安装 Crates.io 上的二进制代码箱

**Installing Binaries from Crates.io with `cargo install`**

`cargo install` 命令允许咱们在本地安装和使用二进制的代码箱。这种用法的目的，不是要替换系统包（system packages）；其宗旨是为 Rust 开发者提供安装其他人已在 [crates.io](https://crates.io) 上分享工具的一种便利方式。请注意咱们只能安装有着二进制目标的那些包。所谓 *二进制目标*（a *binary target*），即与本身为非可运行，而适合于在其他程序中包含的库目标（a libary target）相反的，在代码箱有着一个 `src/main.rs` 文件，或被指定为二进制的另一文件时，所创建出的那个可以运行的程序。通常，代码箱会在 `README` 文件中，有着关于其是否为库代码箱，还是有着二进制目标，或二者皆具方面的信息。

使用 `cargo install` 安装的全部二进制程序文件，都是被存储在安装根的 `bin` 文件中（in the installation root's `bin` folder）。在使用 `rustup.rs` 安装的 Rust，且未有做任何定制配置时，那么这个目录将是 `$HOME/.cargo/bin`。为了能够运行那些使用 `cargo install` 安装的程序，就要确保那个目录是在 `$PATH` 中。

> *注*：可在任意位置运行 `cargo install` 命令来安装某个 Crates.io 上的 Rust 二进制程序，这些程序都将被安装在 `$HOME/.cargo/bin` 之下。若已安装了某个 Rust 程序后再安装他，那么就会有如下输出：

```console
$ cargo install ripgrep                                                                                              1m 4s lennyp@vm-manjaro
    Updating crates.io index
     Ignored package `ripgrep v13.0.0` is already installed, use --force to override
```

比如，在第 12 章中，曾提到有个名为 `ripgrep` 用于检索文件的 `grep` 的 Rust 实现。要安装 `ripgrep`，就可以运行如下命令：

```console
$ cargo install ripgrep                                                                                                       lennyp@vm-manjaro
    Updating crates.io index
  Installing ripgrep v13.0.0
   Compiling memchr v2.5.0
   Compiling cfg-if v1.0.0
   Compiling libc v0.2.137
   Compiling log v0.4.17
   Compiling proc-macro2 v1.0.47
   Compiling lazy_static v1.4.0
   Compiling regex-automata v0.1.10
   Compiling quote v1.0.21
   Compiling unicode-ident v1.0.5
   Compiling bstr v0.2.17
   Compiling syn v1.0.103
   Compiling aho-corasick v0.7.20
   Compiling regex-syntax v0.6.28
   Compiling serde_derive v1.0.147
   Compiling encoding_rs v0.8.31
   Compiling serde v1.0.147
   Compiling regex v1.7.0
   Compiling grep-matcher v0.1.5
   Compiling serde_json v1.0.89
   Compiling unicode-width v0.1.10
   Compiling fnv v1.0.7
   Compiling same-file v1.0.6
   Compiling once_cell v1.16.0
   Compiling thread_local v1.1.4
   Compiling globset v0.4.9
   Compiling textwrap v0.11.0
   Compiling encoding_rs_io v0.1.7
   Compiling memmap2 v0.5.8
   Compiling bitflags v1.3.2
   Compiling crossbeam-utils v0.8.14
   Compiling bytecount v0.6.3
   Compiling itoa v1.0.4
   Compiling ryu v1.0.11
   Compiling strsim v0.8.0
   Compiling termcolor v1.1.3
   Compiling clap v2.34.0
   Compiling grep-searcher v0.1.10
   Compiling atty v0.2.14
   Compiling base64 v0.13.1
   Compiling grep-printer v0.1.6
   Compiling grep-cli v0.1.6
   Compiling grep-regex v0.1.10
   Compiling ripgrep v13.0.0
   Compiling walkdir v2.3.2
   Compiling ignore v0.4.18
   Compiling grep v0.2.10
   Compiling num_cpus v1.14.0
    Finished release [optimized + debuginfo] target(s) in 1m 09s
  Installing /home/lennyp/.cargo/bin/rg
   Installed package `ripgrep v13.0.0` (executable `rg`)
```

输出的最后两行，显示了那个已安装二进制 Rust 程序的位置与名字，在 `ripgrep` 这个示例中，名字即为 `rg`。而由于正如前面提到的那样，该安装目录是在 `$PATH` 中，因此随后就可以运行 `rg --help`，进而启动一个用于检索文件的更快、更具 Rust 风格的工具了！


## 使用定制命令对 Cargo 进行扩展

**Extending Cargo with Custom Commands**

Cargo 被设计为在无需修改 Cargo 下，就可以使用一些新的子命令，对其加以扩展。当 `$PATH` 中有着一个名为 `cargo-something` 的二进制程序时，那么就可通过运行 `cargo something`，将其作为某个 Cargo 的子命令一样运行他。像这样的定制命令，还会在运行 `cargo --list` 被列出来。这种使用 `cargo install` 来安装扩展，并在随后就跟运行内建的 Cargo 工具一样运行他们，正是 Cargo 之设计的一项超级便利的好处！


## 本章小节

运用 Cargo 与 [crates.io](https://crates.io) 进行代码的分享，正是令到 Rust 生态对于许多不同任务都有用的一个方面。Rust 的标准库是小型且稳定的，但在不同于语言本身的时间线上，代码箱则是易于共享、运用以及改进的。请不要羞于在 [crates.io](https://crates.io) 上分享对自己有用的代码；那些代码或许同样对其他人也是有用的！
