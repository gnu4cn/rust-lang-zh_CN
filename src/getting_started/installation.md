# 安装

第一步是安装 Rust。我们将通过一个管理 Rust 版本与相关工具的命令行工具 `rustup`，下载 Rust。下载时咱们将需要互联网连接。

> 注意：若由于某些原因而不愿使用 `rustup`，那么请参考 [其他 Rust 安装方式页面](https://forge.rust-lang.org/infra/other-installation-methods.html) 了解更多选项。

以下步骤将安装最新稳定版本的 Rust 编译器。Rust 的稳定性，保证了本书中所有编译成功的示例，都能继续在较新的 Rust 版本中编译。不同版本的输出，可能略有不同，因为 Rust 经常会改进错误信息和告警。换句话说，使用这些步骤安装的任何较新的、稳定的 Rust 版本，都应能与本书内容所预期的一样运行。


> 关于 **命令行注解**
>
> 在本章和全书中，我们将介绍一些在终端中使用的命令。咱们要敲入到某个终端中的那些行，都以 `$` 开头。咱们无需键入那个 `$`；他属于命令行提示符，用于表示每条命令的开始。不以 `$` 开头的行，通常会显示前一条命令的输出。此外，特定于 `PowerShell` 的示例，会使用 `>` 而不是 `$`。


## 在 Linux 与 macOS 上安装 `rustup`


如果咱们使用的是 Linux 或 macOS，请打开终端并输入以下命令：


```console
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

这条命令会下载一个脚本，并启动 `rustup` 这个工具的安装，而 `rustup` 就会安装最新的 Rust 稳定版本。可能会提示输入密码。如果安装成功，会出现下面一行：


```console
Rust is isntalled now. Great!
```

咱们还需要一个 Rust 用于将其编译输出，连接为一个文件的 *链接器，linker*，a program that Rust uses to join its compiled outputs into one file。咱们很可能已经有了这个链接器。如果出现链接器错误，那么咱们就应安装一个，通常包含了某个链接器的 C 编译器。C 编译器也很有用，因为一些常见的 Rust 软件包，均依赖于 C 代码，而因此需要 C 编译器。

在 macOS 上，咱们可以通过运行：


```console
$ xcode-select --install
```

获得一个 C 编译器。

Linux 用户一般应根据其发行版的文档，安装 GCC 或 Clang。例如，如果咱们使用 Ubuntu，则可以安装 `build-essential` 软件包。


### 使用国内 Crates、rustup 源


使用字节提供的 Rust 镜像源 [https://rsproxy.cn](https://rsproxy.cn/)，可提升 Rust 安装及 Crates 下载速度。


- 在 `~/.zshrc` 或 `~/.bashrc` 中加入以下内容


```bash
export RUSTUP_DIST_SERVER="https://rsproxy.cn"
export RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup"
```


- 创建/修改 `~/.cargo/config`，加入以下内容


```conf
[source.crates-io]
replace-with = 'rsproxy'

[source.rsproxy]
registry = "https://rsproxy.cn/crates.io-index"

[registries.rsproxy]
index = "https://rsproxy.cn/crates.io-index"

[net]
git-fetch-with-cli = true
```


- 首次安装 `rustup`


```console
# export the env above first
curl --proto '=https' --tlsv1.2 -sSf https://rsproxy.cn/rustup-init.sh | sh
```


> **参考**：[Rust使用国内Crates 源、 rustup源 |字节跳动新的 Rust 镜像源以及安装rust](https://blog.csdn.net/inthat/article/details/106742193)


## 在 Windows 上安装 `rustup`


在 Windows 上，请前往 https://www.rust-lang.org/tools/install 并按照说明安装 Rust。在安装过程中的某个时刻，咱们会收到一条信息，说明咱们还需要 Visual Studio 2013，或更高版本的 MSVC 构建工具。

而要获得构建工具，咱们需要安装 Visual Studio 2022。当被问及要安装哪些工作负载时，要包括下面这些：


- “使用 C++ 进行桌面开发”

- Windows 10 或 11 SDK

- 英语语言包组件，以及咱们所选的任何其他语言包


本书的其余部分使用了，可同时在 *cmd.exe* 和 PowerShell 中运行的命令。如果存在某些具体差异，我们将解释要使用哪个。


### 设置镜像加速


以管理员权限启动 Powershell，执行以下命令设置 `RUSTUP_DIST_SERER` 和 `RUSTUP_UPDATE_ROOT` 两个环境变量。


```powershell
[environment]::SetEnvironmentvariable("RUSTUP_DIST_SERVER", "https://mirrors.ustc.edu.cn/rust-static", "User")
[environment]::SetEnvironmentvariable("RUSTUP_UPDATE_ROOT", "https://mirrors.ustc.edu.cn/rust-static/rustup", "User")
```

保持这个 Powershell 窗口打开。


执行下载到的 `rustup-init.exe` 程序，将以上述设置的两个环境变量安装 Rust。

打开普通 Powershell 窗口，执行以下命令，创建镜像的 Cargo 配置。


```powershell
"[source.crates-io]
registry = 'https://github.com/rust-lang/crates.io-index'
replace-with = 'ustc'
[source.ustc]
registry = 'https://mirrors.ustc.edu.cn/crates.io-index/'
"|Out-File -Encoding utf8 $home\.cargo\config
```


> **参考**：[Windows 安装 Rust 并设置镜像加速](https://www.cnblogs.com/timefiles/p/17930394.html)


## 问题排除

要检查 Rust 安装是否正确，请打开 shell 并输入这一行：


```console
$ rustc --version
```

就会看到版本编号、合并哈希（`commit` hash），以及已发布的该最新稳定版本合并日期，以下面这种格式：

```console
rustc x.y.z (abcabcadc yyyy-mm-dd)
```

> 在 Windows 上的 MSYS2 环境下的输出为：

```console
~ rustc --version
rustc 1.74.0 (79e9716c9 2023-11-13)
```

如果看到此信息，说明咱们已成功安装 Rust！如果没有看到此信息，请检查 Rust 是否在咱们的 `%PATH%` 系统变量中，如下所示。

在 Windows 的 CMD 中，请使用：


```cmd
> echo %PATH%
```

在 PowerShell 中，请使用：

```powershell
> echo $env:Path
```

在 Linux 及 macOS 中，请使用：

```console
$echo $PATH
```

如果这些都正确无误，而 Rust 仍无法正常工作，那么有很多咱们可以寻求帮助的地方。请访问 [社区页面](https://www.rust-lang.org/community)，了解如何与其他 Rustaceans（我们给自己起的昵称）取得联系。

## 更新与卸载

通过 `rustup` 安装 Rust 后，更新到新近发布的版本，就很容易了。请在 shell 中运行以下更新脚本：


```console
$ rustup update
```

要卸载 Rust 和 `rustup`，请在 shell 中运行以下卸载脚本：


```console
$ rustup self uninstall
```

## 本地文档

Rust 的安装，还包含了一份本地文档，以便咱们离线阅读。请运行 `rustup doc`，在咱们的浏览器中打开本地文档。

如果咱们不确定，某个标准库提供的类型或函数的作用，或使用方法，请使用应用程序编程接口（API）文档来搞清楚！


（End）


