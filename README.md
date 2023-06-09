原著：[The Rust Programming Language](https://doc.rust-lang.org/book/)

*原作者：Steve Klabnik 与 Carol Nichols, 及 Rust 社区*


此版本的教材，假定安装了 Rust `1.67.1` （发布于 2023-02-09）或更新版本。请参阅 [第 1 章的 “安装” 小节](docs/Ch01_Getting_Started.md#Installation) 进行安装，或对已安装的 Rust 进行升级。

```console
$ rustc --version
rustc 1.68.0 (2c8cc3432 2023-03-06)
```


在线阅读: [rust-lang.xfoss.com](https://rust-lang.xfoss.com)

本地阅读：[`mdbook` 本地运行](./src/local_serving.md)

---

# 前言和简介

虽然这样说有些含糊其辞，但基本上可说 Rust 编程语言，是一种 *赋能（empowerment）*：不管你当前在用哪种语言编写代码，Rust 都可以赋予你更大能力，在编程之路上走得更远，在你先前的各种领域，更具信心地编写程序。

就拿涉及到底层的内存管理、数据表示及并发等 “系统层面” 的工作来讲。传统上，这类编程都被认为是深奥的、只由极少数花费多年时间掌握了避开其间臭名昭著陷阱的程序员来完成。而即使这些人，在写这类代码时，仍然是小心翼翼，以免他们写出的代码留下漏洞利用、崩溃或不当。

通过消除原有各种陷阱，以及提供到友好、全新工具集，Rust 破除了编写这类苛刻程序中的诸多障碍。那么那些需要“深入”到底层控制的程序员们，现在就可以运用 Rust，在不必承担一直以来的崩溃或者安全漏洞的情况下，同时还无须去深入细致地掌握那变化无常工具链，就可以达成他们的目标了。更了不起的是，在设计这门语言时，就贯彻了引导使用他的程序员编写出可靠又高效的代码，体现在运行速度及内存使用上。

正在进行底层代码编写的程序员们，可运用 Rust 来提升他们的雄心壮志。比如说，在Rust 中引入并行机制，是相对低风险的操作：编译器会为你捕获到那些经典错误。同时还可以在确信不会带来程序崩溃或漏洞利用之下，大胆进行更多的优化。

然而 Rust 并非局限于底层系统的编写。对于构造命令行应用、web 服务器及其他类别的代码来说，Rust 的表现力和人机工程设计，也可以让这些编写起来相当愉悦 -- 在本书后面，就会发现这样的示例。运用 Rust 可实现将一个领域中学到的技能，迁移到另一领域；通过编写 web 应用，就可以掌握 Rust, 然后将这同样的技能应用到树梅派 app 的编写上。

本书充分接纳到 Rust 给其使用者赋能的潜力。这本书友好而恰当，试图帮助你不光提升有关Rust的知识，还在一般方面提升你编程的水平和信心。那么就请继续阅读下去，欢迎来到 Rust 社区！

-- *Nicholas Matsakis 与 Aaron Turon*

## 简介

欢迎来到 *Rust 编程语言*，一本 Rust 的介绍性书籍。Rust 编程语言帮助更快地编写出更可靠软件。在程序语言设计中，上层人机交互与底层控制，通常是不可调和的；Rust 挑战了这对矛盾。经由强力的技术能力与了不起的开发者体验，Rust 带来了对底层细节（譬如内存的使用）控制的同时，免去了传统上底层控制带来的一大堆麻烦。

## Rust 适用于哪些人群

对于相当多的人群，Rust 因为各种原因，都是理想选择。下面就来看看那些最重要群体中的一些人的情况。

### 开发者团队

对于有着不同水平系统编程知识的开发者团队的协同来讲，Rust 正被证明是一种生产力工具。底层代码倾向于出现各种细微错误，这样的细微错误，对于其他编程语言，则只能经由广泛测试，和经验老道的开发者细致代码评审才能捕获到。而在 Rust 中，编译器通过拒绝编译这些难以捉摸的错误，包括并发错误，而扮演着看门人角色。通过与编译器一道工作，团队就可以将他们的时间，集中用在程序逻辑，而不是找寻错误上。

Rust 还带给了系统编程世界，一些现代开发者工具：

- Cargo，Rust 所包含的依赖管理器与构建工具，让整个 Rust 生态中添加依赖、编译与管理依赖，变得愉快并具一致性，`Cargo`, the included dependency manager and build tool, makes adding, compiling, and managing dependecies painless and consistant across the Rust ecosystem；
- Rustfmt 格式化工具，the Rustfmt formatting tool，确保不同开发者之间有着一致的编码风格；
- Rust 语言服务器，the Rust Language Server, 驱动了用于代码补全与行内错误消息的集成开发环境。

通过使用这些开发者工具，及其他一些 Rust 生态中的工具，开发者就可以在编写系统级代码时，颇具生产力了。

### 学生

Rust 是为学生及那些对掌握系统概念感兴趣的人所准备的。运用 Rust，许多人都掌握了像是操作系统开发这样的知识点。Rust 社区非常欢迎并乐于回答学生们提出的问题。通过像是本书这样的努力，Rust 团队本身是要让更多人，尤其是那些刚开始编程的人们，可获取到系统概念。


### 商业公司

已有上千家规模或大或小的商业公司，在生产中，为着不同任务使用着 Rust。这些任务包括了命令行工具、web 服务、运维工具、嵌入式装置、音视频分析与转码、加密货币、生物信息学、搜索引擎、物联网应用、机器学习，甚至Firefox web浏览器的主要部分等等。

### 开放源代码开发者

Rust 是为着那些想要构建 Rust 编程语言本身、Rust 社区、Rust 开发者工具和库而准备的。我们希望你为 Rust 语言做出贡献。

### 看重运行速度与稳定性的人们

Rust 是为那些渴求某门语言所提供速度与稳定性的人们准备的。这里说的运行速度，指的是使用 Rust 可创建出程序的运行速度，以及 Rust 所能达到的编写这些程序速度。Rust 编译器的检查，确保了功能补充与重构的稳定性。这稳定性是与那些不具备这些检查语言中的脆弱老旧代码相比得出的，开发者通常害怕去修改那些脆弱老旧代码。通过争取实现零代价的抽象，就有了那些在手动编写时，就立即编译到底层代码的上层特性，Rust 致力于实现在构造安全代码的同时，还取得了快速的代码编写与程序运行。

Rust 语言也希望带给众多其他用户以支持；这里提到的只是一些最大的相关群体。总体来讲，Rust 最伟大的抱负，是要消除程序员们在过去数十年来，业已被迫接受的在安全性与生产力、开发和运行速度及人机交互上的妥协。请给 Rust 一个机会，然后看看 Rust 的选择是否适合于你。

## 本书读者群体

本书假定你曾编写过其他某种编程语言的代码，至于何种编程语言并不重要。本书作者已尽力让其中的教学材料适合到有着宽泛编程背景的读者。这里不会花大量时间来解释编程为何物，以及该怎么来看待编程。若对编程一窍不通，那么最好找一本编程入门的书先看看。

## 怎样使用本书

大体上，本书假定是要以从前往后的顺序进行阅读。后续章节是建立在较早章节的概念之上，而较早的章节不会深入某个话题的细节；在后续章节通常会回顾到那个话题。

本书有两种章节：概念性章节与项目性章节。在概念章节，将对 Rust 某个方面的有所了解。而在项目性章节，就会构建出一些在一起的小程序，这些小程序运用了概念性章节中学到的东西。第 2、12 和 20 章，就是项目性章节；而剩下的，全都是概念性章节。

第 1 章讲了怎样安装 Rust、怎样编写出 “Hello, world!” 程序，还有怎样使用 Rust 的包管理器及构建工具 `Cargo`。第 2 章是 Rust 语言的一个实操介绍。这里涵盖了上层的一些概念，而在后续章节则会提供到进一步的细节。若要立即动手编写代码，那么第 2 章就可以开始了。一开始你或许想要跳过第 3 章，这一章涵盖了与那些其他编程语言类似的一些 Rust 特性，而要直接前往到第 4 章，去了解 Rust 的所有权系统。不过若要细致了解 Rust，就应详细掌握 Rust 的每个细节设计之后，在移步到下一章节，或许也会跳过第 2 章直接到第 3 章，然后在想要将学到的细节知识应用到项目时，再回到第 2 章。

第 5 章讨论了结构体和方法，同时第 6 章涵盖了枚举、`match` 表达式，和 `if let` 控制流结构。在构造 Rust 中的定制类型时，就会用到这些结构体和枚举。

第 7 章将了解到 Rust 的模组系统，以及代码组织的隐私规则，还有 Rust 的公共应用编程接口（Application Programming Interface, API）。第 8 章讨论了一些常用的、由标准库提供的集合数据结构，诸如矢量、字符串及哈希图。第 9 章探索了 Rust 的错误处理思想与技巧。

第 10 章涉及了范型、特质（traits） 与生命周期，他们赋予了定义出应用多种类型代码的能力。第 11 章全都是关于测试的内容，即便有着 Rust 的安全性保证，对于确保程序逻辑正确，测试仍是不可缺少的。第 12 章将构建一个我们自己的、用于在文件中搜索文本的 `grep` 命令行工具功能子集的版本。到这里，就会用到先前章节中所讨论的众多概念了。

第 13 章对闭包（closures）和迭代进行了探索：闭包和迭代属于 Rust 来自函数式编程语言的特性。第 14 章，将更深入地对 `Cargo` 加以检视，并就如何与他人分享库的最佳实践进行探讨。第 15 章讨论标准库提供的一些灵巧指针，还有实现不同功能的 Rust 特质（traits）。

第 16 章，将遍数并发编程的各种不同模型，并探讨 Rust 如何对大胆进行多线程编程的帮助。第 17 章将或许你所熟知面向对象编程的那些原则，与 Rust 下编程的习惯加以对比。

第 18 章是模式与模式匹配的一个参考，这在 Rust 程序中，属于是概念表达的强大方式。第 19 章包含了一个诸多感兴趣的话题大杂烩，包括不安全的 Rust、宏，以及更多有关生命周期、特质（traits）、类型、函数与闭包等等的细节。

第 20 章，将完成一个其中实现了底层线程化的 web 服务器！

最后，还有一些包含了这门语言的有用信息的附录，这些附录则更多的像是参考的形式。附录 A 涵盖了 Rust 的关键字，附录 B 涵盖了 Rust 的运算符和符号，附录 C 涵盖了由标准库所提供的那些派生特质（derivable traits），附录 D 涵盖了一些有用的开发工具，还有附录 E 对 Rust 版本进行了解释。

阅读本书并无定法：你要跳着去读，也是可以的！在遇到疑惑时，或许就不得不跳回去看看了。你只要怎么有效就行了。

掌握 Rust 过程中的一个重要部分，就是要学会怎样去读那些编译器给出的错误消息：这些错误消息将引导你得到运作的代码。由此，本书将提供到许多不编译的示例，以及在各种情况下编译器将给出的错误消息。请知悉在进入到某个随机示例并加以运行时，示例代码可能会不编译！请确保要阅读这些示例周围的文字，来了解正尝试运行的示例，是不是有错误。本书中的虚拟人物 `Ferris` 也会帮助你识别代码是否会工作的：

| Ferris | 意义 |
| :-: | :- |
| ![不会编译](images/Ch00_01.svg) | 此代码不会编译！ |
| ![不会运行](images/Ch00_02.svg) | 此代码不会运行! |
| ![不会产生期望的行为](images/Ch00_03.svg) | 此代码不会产生出期望的行为。 |

*表 1 - Ferris 表情含义*

多数情况下，这里都会给出不会编译代码的正确版本。

## 本书的源码

本书所产生的源码，可在 [Github: gnu4cn/rust-lang](https://github.com/gnu4cn/rust-lang-zh_CN/releases/tag/v0.2.0) 下载到。
