# 安装

第一步即是安装 Rust。这里将通过 `rustup` 这个用于管理 Rust 版本及相关工具的命令行工具，来下载 Rust。要下载 Rust，就需要互联网连接。

> 注意：若由于某些原因而不愿使用 `rustup`，那么请参考 [其他 Rust 安装方式页面](https://forge.rust-lang.org/infra/other-installation-methods.html) 了解更多选项。

接下来就是要按照最新的稳定版 Rust 编译器。Rust 的稳定性保证了本书中所有示例都将在较新的 Rust 版本下可持续编译。由于 Rust 经常会改进错误消息和告警，因此在不同版本之间，输出可能会略有不同。也就是说，任何使用以下步骤所安装的较新、稳定版 Rust，都将如本书内容中所期望的那样工作。

> 关于**命令行注释**
> 在本章及全书中，都会给出一些在终端中用到的命令。他们是一些应在以 `$` 开始的终端中输入的行。至于这个 `$` 字符，是无需输入的；这个字符表示每条命令的开头。那些不以 `$` 开头的行，通常给出的是上一命令的输出。此外，那些特定于 `PowerShell` 的示例中，将使用 `>` 而不是 `$`。


## 在 Linux 与 macOS 上安装 `rustup`

若使用的是 Linux 或 macOS，那么请打开一个终端，然后输入下面的命令：

```console
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

此命令会下载一个脚本并开始 `rustup` 工具的安装，而 `rustup` 将安装最新的稳定版 Rust。可能会提示输入 `sudo` 密码。在安装成功后，就会出现下面这行！

```console
Rust is isntalled now. Great!
```

这里还将需要一个连接器（linker），这是个Rust要用来将其编译好的输出，组合起来形成一个文件的程序。似乎你的电脑上以及有了一个这样的连接器了。若收到连接器错误信息，那么就应安装一个 C 语言编译器，C 编译器通常会包含着连接器的。由于一些常用 Rust 包对 C 代码有依赖且需要 C 编译器，因此 C 编译器也是有用的。

在 macOS 上，可通过运行下面的命令，获取到一个 C 编译器：

```console
$ xcode-select --install
```

Linux 用户一般都会安装 GCC 或 Clang，至于具体哪种 C 编译器，则是依据他们所用 Linux 分发版本的文档可以确定。比如若使用的是 Ubuntu，那么就可以安装 `build-essential` 软件包。


## 在 Windows 上安装 `rustup`

在 Windows 上，请前往 [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) 页面，并按照安装 Rust 的指令进行安装。在安装过程的某个时刻，将收到为何需要 Visual Studio 2013 或更新版本的 C++ 构建工具的说明。而最简单的获取到构建工具的方法，则是安装 [Visual Studio 2019 构建工具](https://visualstudio.microsoft.com/visual-cpp-build-tools/)。在询问将要安装何种工作负载（workloads）时，请确保 `C++ build tolls` 被选中，还要确保包含 Windows 10 SDK 及英语语言包。

本书接下来用到的命令，在 `cmd.exe` 与 `PowerShell` 中都可工作。若其中有特定区别，本书将会解释要用哪个。


## 更新与卸载

在通过 `rustup` 安装了 Rust 后，更新到最新版本就容易了。在 `shell` 中运行下面的更新脚本：

```console
$ rustup update
```

而要卸载 Rust 和 `rustup`，只需在 `shell` 中运行下面的卸载脚本：

```java
$ rustup self uninstall
```

## 问题排除

要检查当前是否安装了 Rust, 请开启一个 `shell` 并敲入这行命令：

```console
$ rustc --version
```

就会看到版本编号、合并哈希（`commit` hash），以及已发布的该最新稳定版本合并日期，以下面这种格式：

```console
rustc x.y.z (abcabcadc yyyy-mm-dd)
```

若看到这个信息，那么就已成功安装了 Rust！若看不到这个信息，且是在 Windows 上，那么就请在 `%PATH%` 系统变量中检查一下 Rust 在不在里面。若那一点问题都没有而 Rust 仍就不工作，那么可在数个地方需求帮助。其中最便利的就是 [Rust 官方 Discord](https://discord.gg/rust-lang) 上的 `#beginners` 频道了。在那里可与其他 Rust 公民（一种无厘头的自我称呼）聊天，他们可以帮助到你。其他不错的资源包括 [用户论坛](https://users.rust-lang.org/) 和 [Stack Overflow](https://stackoverflow.com/questions/tagged/rust)。


## 本地文档

Rust 的安装，也包含了一份本地文档，因此可离线阅读到这本地文档。运行 `rustup doc` 即可在浏览器中打开这本地文档。

在任何时候遇到标准库所提供的类型或函数，而又确定他做些什么或该怎样使用这类型或函数时，就可以使用 API 文档来搞明白他是怎么回事！
