# 掌握 Rust 中的所有权

**Understanding Ownership**

所有权，ownership 作为 Rust 最为独特的特性，而对这门语言其余部分有着深刻影响。正是所有权，使得 Rust 在无需垃圾收集器的情况下，保证了内存安全，因此掌握所有权的工作原理，就尤为重要。在这一章，将就所有权，以及几个与所有权有关的特性：借用、切片，以及 Rust 在内存中放置数据的方式等，进行讲解。

## 何谓所有权

*所有权* 是掌管着 Rust 程序管理内存方式的一套规则（*ownership* is a set of rules that governs how a Rust program manages memory）。所有程序在其运行期间，都必须管理其运用计算机内存的方式。一些语言有着伴随着其程序运行，而持续查找不再用到内存的垃圾回收；在别的一些语言中，程序员必须显式地分配和释放内存。Rust 采用了第三条路线：经由带有编译器会加以检查的一套规则的所有权系统，内存便得到了管理。在这些规则的任何一条被违反了时，程序就不会编译。所有权的任何一个特性，都不会在程序运行期间，拖慢程序运行速度。

由于对许多程序员来说，所有权都是个新概念，因此要些时间来习惯他。好消息则是随着对 Rust 与那些所有权系统规则的愈加熟练，那么就会发现，顺其自然地开发出安全且高效的代码，会变得越来越容易。请务必坚持下去！

在掌握了所有权后，就会对那些令到 Rust 成为一门独特编程语言的特性，有扎实掌握。在本章中，将通过完成着重于甚为常见的一些数据结构：字符串的示例，而掌握到所有权。

> **内存栈与堆，the Stack and the Heap**
>
> 许多编程语言，都不要求进程考虑内存栈与堆。不过在像是 Rust 这样的系统编程语言中，某个值是在栈上还是在堆上，就会对语言的行为方式，造成影响，还会影响到不得不做出一些明确决定的理由。本章稍后将讲到的所有权的那些部分，是与内存栈和堆有关的，因此这里是关于他们的一点简要说明，作为预备知识。
>
> 内存栈和堆，都属于在运行时代码可用内存的组成部分，但他们是以不同方式架构组织起来的。栈，the stack 以其收到值的顺序，保存着一些值，并以相反的顺序，将这些值移除。这被成为 *后进先出，last in, first out*。设想有一叠盘子：在添加更多盘子时，就要把新的盘子放在盘子堆顶上，而在要用个盘子时，就要从顶上拿。从底下或中间添加或拿走盘子，都是不行的！添加数据被称为 “压入栈，pushing onto the stack”，而移除数据被称为 *弹出栈，popping off the stack*。保存在栈上的数据，必须要有已知的、固定的大小。相反，那些运行时未知大小，或大小可能会变化的数据，就必须保存在堆上。
>
> 内存堆的组织程度较低：在将数据放在堆上时，就要请求确切数量的空间。内存分配器会在堆上找到一处足够大的空白位点，将其标记为正在使用中，然后返回一个 *指针，pointer*，即那个点位的地址。此过程被称为 *堆上内存分配，allocating on the heap*，而有时会去掉“堆”，而简称为 *内存分配，allocating* （而将值压入到栈上，则不被视为内存分配）。由于到堆的指针是已知的、固定大小的，因此就可以将该指针存储在栈上，而在想要具体数据时，就必须依循该指针。请设想正坐在某个餐馆里。在进到餐馆时，就要报出跟你们组的人数，进而餐馆员工就会找出一张可以坐下所有人的空桌子，并把你们带过去。在你们组有人迟到时，他们就可以询问是坐在哪张桌子，而找到你们。
>
> 由于在把数据压到栈上时，内存分配器绝不必搜寻一个位置来存储新数据，因此相比在堆上分配空间，把数据压入栈是要快得多的；存储新数据的地方，始终是在栈顶部。与此相比，在内存堆上分配空间则需要更多工作，由于内存分配器必须先找到一块足够大空间来保存该数据，并随后还要为准备好下一次内存分配，而完成对此次分配的登记。
>
> 因为必须要循着某个指针去获取到数据，因此访问内存堆上的数据，与访问栈上的数据相比，也要慢一些。当较少地在内存中跳跃时，现代处理器会更快。延续上面的比喻，设想餐馆里的一名服务员，正在接收来自许多台餐桌的点餐。那么一次获取到一个桌子的全部点餐，再去往下一桌，无疑是最高效的。而从餐桌 A 拿到一份点餐，再从餐桌 B 拿到一份点餐，随后又从餐桌 A 拿到一份，然后又从餐桌 B 再拿到一份，这样无疑就是慢得多的过程了。经由同一令牌，如果处理器处理的数据与另一数据靠近（就像在栈上那样），而不是远离另一数据（就像在内存堆上可能的情形），那么处理器无疑会更好地完成他的工作。
>
> 在代码对某个函数进行调用时，传入到该函数的值（潜在包含了指向内存堆上数据的指针），以及该函数的本地变量，都是被压入到栈上的。在该函数结束运行后，这些值就被从栈上弹出。
>
> 对代码的哪些部分正在使用内存堆上的哪些数据进行追踪，最小化内存堆上的重复数据数量，以及对内存堆上的未使用数据进行清理而不至于耗尽内存空间等，都是所有权要解决的问题。一旦掌握了所有权，就再也不需要经常考虑栈和堆了，而清楚了所有权主要目的，是为着对内存堆进行管理，则会有助于解释所有权，为何会以他自己的方式运作。


### 所有权规则

首先，来看看这些所有权规则。在完成后面用于演示这些规则的示例时，请牢记这些规则：

- Rust 中的每个值，都有一个名为 *所有者，owner* 的变量；
- 同一时间，只能有一个所有者；
- 在其所有者超出作用域，scope 时，该值就被丢弃。


### 变量作用域（variable scope）

既然已经学了 Rust 基础语法，接下来就不会在示例中，包含整个的 `fn main() {` 代码了，那么若跟随这些示例，就要确保把接下来的这些示例，自己手动放在 `main` 函数里头。这样的结果就是，这些示例会比较精炼一点，着重于具体细节而不是那些样板代码。

作为所有权的首个示例，这里将考察一下一些变量的 *作用域，scope*。作用域是指某个项目在程序中的有效范围。以下面这个变量来说：

```rust
let s = "hello";
```

这里的变量 `s` 指向一个字符串字面值，其中的字符串的值，则是被硬编码到这个程序的文本。自变量被声明处，到当前 *作用域* 结束处，变量都是有效的。下面清单 4-1 给出了一个带有对变量 `s` 在何处有效，进行注解注释的程序：

```rust
{                       // 变量 s 在这里是无效的，他还没被声明出来
    let s = "hello";    // s 自此往下都是有效的

    // 对变量 s 执行一些操作
}                       // 此时该作用域就结束了，而变量 s 也不再有效
```

*清单 4-1：变量与其间有效的作用域，a variable and the scope in which it is valid*

换句话说，这里有两个重点：

- 当变量 `s` 一旦来到作用域，他就有效了，when `s` comes *into scope*, it is valid；
- 他会保持有效，直到 *超出作用域*，it remains valid until it goes *out of scope*。


到这里，作用域和变量何时有效二者之间的关系，与其他语言中的此类关系类似。现在就要通过引入 `String` 类型，在此理解之上建构出所有权的理解，now we'll build on top of this understanding by introducing the `String` type。

### `String` 类型

为了对所有权的那些规则进行演示，就需要比前面第 3 章的 ["数据类型"](Ch03_Common_Programming_Concepts.md#数据类型) 小节中讲到那些类型，更为复杂一些的数据类型。前面讲到的那些类型，都是已知大小、可存储在栈上的，且在他们的作用域结束时会被弹出栈，在代码另一部分需要在不同作用域中用到同一值时，这些类型还可被快速而简单地复制，而构造出新的、独立实例。不过这里要审视的是存储在内存堆上的数据，进而探讨 Rust 是如何知晓，何时要清理这些内存堆上的数据，那么 `String` 类型就是极佳的示例了。

这里将着重于 `String` 类型与所有权有关的部分。这些方面同样适用于其他的、不论是由标准库还是自己创建的复合数据类型，complex data types。在 [第 8 章](Ch08_Common_Collections.md#何为-string) 将深入讲解 `String` 类型。

前面咱们已经见到了一些字符串字面值，其中有个硬编码到程序里的字符串值。字符串字面值很方便，但对于那些打算使用文本的全部情形，他们却并不适合。一个原因是字符串字面值为不可变的。另一个原因则是，在编写代码时，并非每个字符串的值都是已知的：比如，假设要获取用户输入并存储下来呢？对于这样的情形，Rust 有着第二种字符串类型，即 `String`。这种类型对分配到内存堆上的数据加以管理，并因此而具备了存储在编译时数量未知文本的能力。使用 `String` 类型的 `from` 函数，就可以从字符串字面值，创建出一个 `String` 类型的值来，如下所示：

```rust
let s = String::from("hello");
// 变量 s 的类型为：String, 而此前字面值中的变量 s 的类型为：&str
```

其中的双冒号（`::`）运算符，实现了将这个特定 `from` 函数，置于 `String` 类型的命名空间之下，而无需使用类似于 `string_from` 这种名字了。在第 5 章的 [方法语法](Ch05_Using_Structs_to_Structure_Related_Data.md#方法语法) 小节，并在第 7 章的 [对模组树中的某个项目进行引用的路径](Ch07_Managing_Growing_Projects_with_Packages_Crates_and_Modules.md#用于引用目录树中项目的路径) 小节，对模组命名空间的介绍中，将对这种语法进行更多讲解。

这种字符串，*能* 被改变：

```rust
let mut s = String::from("hello");
s.push_str(", world!"); // push_str() 方法把字面值追加到某个字符串
println! ("{}", s); // 这将打印出 `hello, world!`
```

那么，到底字面值, `&str` 与 `String` 类型有何不同？为何 `String` 可以被改变，而字面值却不能？区别就在于，这两种类型处理内存的方式，是不同的。


### 内存与内存分配

对于字符串字面值这种情况，在编译时咱们就知道其内容，因此该文本就被直接硬编码到了最终的可执行文件。这就是为何字符串字面值快速高效的原因。然而这些属性，只是来源于字符串字面值的不可变性。不幸的是，对于那些在编译时大小未知的，且在运行期间大小可能改变的各个文本，是无法为他们而将某块内存，放入到二进制程序中的（unfortunately, we can't put a blob of memory into the binary for each piece of text whose size is unknown at compile time and whose size might change while running the program）。

在 `String` 类型下，为了支持可变、可增长的一段文本，就需要在内存堆上分配某个数量的内存，用来保存文本的那些内容，而这个数量在编译时则是未知的。这就意味着：

- 该内存必须在运行时向内存分配器请求；
- 在使用完那个 `String` 值之后，需要把这片内存交回给内存分配器的某种途径。

其中第一部分是由代码编写者完成的：在调用 `String::from` 时，这个 `from` 方法的实现，就请求了他所需的内存。在各种编程语言中，这是相当通行的做法。

然而，这第二部分就有所不同了。在带有 *垃圾收集器，garbage collector, GC* 的那些语言中，对那些不再是正被使用中的内存的追踪和清理，是由垃圾收集器完成的，对此这里无需去考虑。而在大多数不带垃圾收集器的语言，就要靠代码编写者自己，去识别内存在何时不再被使用，并像请求内存时一样，要调用代码来显式地释放他。要正确完成这样的内存释放，早已成为一个历史悠久的编程难题。若忘记了，咱们就将浪费内存。而过早地释放内存，则将造成变量失效。若执行两次，那也同样是程序错误。咱们需要严格地一个 `allocate` 对应一个 `free`。

Rust 采取了不同的路线：一旦某个变量超出了作用域，那么该变量所持有的内存空间，就被自动退回。下面是对清单 4-1 那个作用域示例，使用 `String` 而非字符串字面值的一个版本：

```rust
    {
        let s = String::from("hello");  // 变量 s 自此往下是有效的

        // 以变量 s 完成一些操作
    }                                   // 该作用域到此时结束，而变量 s
                                        // 不再有效
```

其中就存在可将那个 `String` 类型的值所需的内存，退回给内存分配器的一个天然时间点：即在变量 `s` 超出作用域时。在变量超出作用域时，Rust 就会主动调用一个特殊函数。该函数名为 `drop`，其正是 `String` 类型的编写者，放置用于内存退回的代码之处。在那个结束花括号处，Rust 会自动调用这个 `drop` 函数。

> 注意：在 C++ 中，在某程序中项目生命周期结束时，资源重分配的这种模式，有时被称为 *资源获取即初始化*（in C++, this pattern of deallocating resources at the end of an item's lifetime is sometimes called *Resource Acquisition Is Initialization, RAII*）。若曾用过 RAII 模式，那么 Rust 中的这个 `drop` 函数就会不那么陌生了。

这种模式对 Rust 代码编写方式有深远影响。在此刻他可能看起来还算简单，但在想要让多个变量，使用早先在内存堆上分配的数据，这种更为复杂情形时，代码行为就会无法被预见到。现在就来探讨一下一些这样的情况。

### 变量与数据互操作方式之一：迁移（所有权）

在 Rust 中，多个变量可以多种方式，与同一数据进行互操作。来看看下面清单 4-2 中用到整数的示例：

```rust
let x = 5;
let y = x;
```

*清单 4-2：将变量 `x` 的整数值，赋值给变量 `y`*

或许能猜到这段代码正在完成的事情：“把值 `5` 绑定到变量 `x`；随后构造一份 `x` 中值的拷贝并将其绑定到变量 `y`。” 现在就有了两个变量，`x` 与 `y`，且他们都等于 `5`。由于整数是有着已知的、固定大小的简单值，因此这实际上就是正在发生的事情，且这两个 `5` 的值都是被压入到栈上的。

> **注**：这就是下面会讲到的 [栈上数据的拷贝，copy](#唯栈数据拷贝stack-only-data-copy) 情形。


那么现在来看看 `String` 的版本：

```rust
let s1 = String::from("hello");
let s2 = s1;
```

这代码看起来与上面的非常相似，那么这里就可以假定其工作方式也是一样的：那就是，第二行将构造出一个 `s1` 中值的拷贝，并将该拷贝绑定到 `s2`。不过这并非真的是实际发生的样子。

> **注**：下面的代码将打印出 `s1 = 你好, s2 = 你好`，表示类型 `&str` （字符串切片）是存储在栈上的。

```rust
fn main() {
    let s1 = "你好";
    let s2 = s1;

    println! ("s1 = {}, s2 = {}", s1, s2);
}
```

请参阅下面的图 4-1，来搞明白在幕后 `String` 到底发生了什么。`String` 类型的值，是由三部分构成，在下图中的左边有给出：一个指向到保存该字符串内容内存的指针、一个长度，和一个该字符串的容量。这样一组数据被保存在栈上。下图的右边，即是内存堆上保存着字符串内容的内存。

![Rust 中 `String` 类型的本质](images/Ch04_01.svg)

*图 4-1：、保存着绑定到变量 `s1` 的值 `hello` 的一个 `String` 类型值在内存中的表示*

> **注**：`String` 类似属于 [灵巧指针，smart pointer](Ch15_Smart_Pointers.md)，他是个包含了指针与其他一些元数据的，带有一些方法的特别能力的结构体。

其中的长度，即为以字节计数、该 `String` 值内容正使用着的内存数量。而容量则是该 `String` 值从内存分配器处收到的、以字节计算的内存数量。长度与容量之间的区别，会相当重要，但在此情形下尚不重要，到目前未知，是可以忽略容量这个部分的。

在将 `s1` 赋值给 `s2` 时，这个 `String` 值被拷贝了，表示这里拷贝了栈上的指针、长度和容量。这里并未拷贝指针指向的、内存堆上的数据。也就是说，内存中数据的表示，如下图 4-2 所示：

![有着变量 `s1` 的指针、长度与容量拷贝的变量 `s2` 在内存中的表示](images/Ch04_02.svg)

*图 4-2：有着变量 `s1` 的指针、长度与容量拷贝的变量 `s2` 在内存中的表示*

这种表示 *不* 同于下图 4-3，那才是 Rust 对内存堆上的数据进行拷贝时，内存看起来的样子。如果 Rust 像下图 4-3 中那样做，那么当内存堆上的数据较大时， `s2 = s1` 的这个操作，将会在运行时性能开销上代价高昂。

![`s2 = s1` 操作的另一种可能：Rust 拷贝内存堆数据](images/Ch04_03.svg)

*图 4-3：`s2 = s1` 操作的另一种可能：Rust 同时拷贝内存堆数据*

早先曾讲过，在变量超出作用域后，Rust 会自动调用那个 `drop` 函数，而清理掉那个变量的堆内存。但图 4-2 则给出了两个指针都指向同一位置的情况。这就是个问题了：在 `s2` 与 `s1` 都超出作用域时，他们都将尝试去释放那同样的内存。这被称为 *双重释放，double free* 错误，是先前提到过的内存安全错误之一，one of the memory safety bugs。二次释放内存，可导致内存损坏，而内存损坏则会潜在导致安全漏洞。

为确保内存安全，Rust 在代码行 `s2 = s1` 之后，便不再认为 `s1` 是有效的了。因此，在 `s1` 超出作用域后，Rust 便不需要释放任何内存。下面就来检查一下，在 `s2` 创建出来后，去尝试使用 `s1` 会发生什么；这样做是不会工作的：

```rust
    let s1 = String::from("hello");  // 这里 s 的类型为：String
    let s2 = s1;

    println! ("{}", s1);
```

由于 Rust 阻止了对失效引用变量的使用，因此将收到一个下面这样的错误：

```console
$ cargo run
   Compiling string_demo v0.1.0 (/home/peng/rust-lang/projects/string_demo)
warning: unused variable: `s2`
 --> src/main.rs:3:9
  |
3 |     let s2 = s1;
  |         ^^ help: if this is intentional, prefix it with an underscore: `_s2`
  |
  = note: `#[warn(unused_variables)]` on by default

error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:5:21
  |
2 |     let s1 = String::from("hello");  // 这里 s 的类型为：String
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     let s2 = s1;
  |              -- value moved here
4 |
5 |     println! ("{}", s1);
  |                     ^^ value borrowed here after move
  |
  = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0382`.
warning: `string_demo` (bin "string_demo") generated 1 warning
error: could not compile `string_demo` due to previous error; 1 warning emitted
```

若在使用其他编程语言时，曾听说过 *浅拷贝（shallow copy）* 和 *深拷贝（deep copy）* 这两个说法，那么这种对指针、长度与容量的拷贝，而未拷贝数据的概念，或许听起来像是进行了一次浅拷贝。但由于 Rust 还将第一个变量进行了失效处理，因此这里就不叫浅拷贝，而叫做 *迁移（move）*。在这个示例中，就会讲，变量 `s1` 已被 *迁移* 到变量 `s2` 里了。因此真实发生的事情，就是下图 4-4 显示的那样：

![在变量 `s1` 失效后内存中的表示](images/Ch04_04.svg)

*图 4-4：在变量 `s1` 失效后内存中的表示*

这就解决了问题！在只有 `s2` 有效之下，当变量 `s2` 超出作用域后，那么就只有他会释放内存，于是就解决了双重内存释放问题。

此外，这种做法背后，还隐含着一种语言设计上的取舍：Rust 绝不会自动创建数据的 “深” 拷贝。由此，任何 *自动* 拷贝，都可认为在运行时性能开销上的影响很小（Therefore, any *automatic* copying can be assumed to be inexpensive in terms of runtime performance）。

### 变量与数据交互方式之二：克隆

在 *确实* 打算对 `String` 的内存堆数据，而非只是栈数据进行深度拷贝时，就可以使用一个常用的、名为 `clone` 的方法。在第 5 章将讨论到方法语法，而由于在众多编程语言中，方法都是共同特性，那么此前大概率是见到过方法的。

下面是一个运作中的 `clone` 方法示例：

```rust
fn main() {
    let s1 = String::from("hello");  // 这里 s 的类型为：String
    let s2 = s1.clone();

    println! ("s1 = {}, s2 = {}", s1, s2);
}
```

这段代码工作起来毫无问题，并显式地产生出图 4-3 中给出的行为，其间内存堆数据确实得以拷贝。

当看到一个对 `clone` 方法的调用时，那么就明白正有一些任性代码在被执行，且那代码可能开销高昂。对此方法的调用，是某些不寻常事情正在发生的直观指示。

### 唯栈数据：拷贝（stack-only data: copy）

尚有另一个至今还未讲到的小问题。正使用着整数的这段代码 -- 其中一部分在下面的清单 4-2 中给出了 -- 会工作并是有效代码：

```rust
let x = 5;
let y = x;

println! ("x = {}, y = {}", x, y);
```

然而这段代码，似乎与前面刚刚所掌握的相抵触：这里没有对 `clone` 的调用，但变量 `x` 依然有效，而并未迁移到变量 `y` 中去。

原因就在于，诸如整数这样的，在编译时大小已知的类型，都是被整个存储在栈上，那么构造他们具体值的拷贝是迅速的。那就意味着，在构造出变量 `y` 之后，就没有理由要去阻止变量 `x` 一直有效了。换句话说，此时的深拷贝与浅拷贝之间，是没有区别的，因此对 `clone` 进行调用，不会完成与通常的浅拷贝有任何区别的事情，进而就能忽略这个 `clone` 方法。

Rust 有着叫做 `Copy` 特质（the `Copy` trait, 在第 10 章将对特质，traits，进行更多的讲解）的，可放在像是整数这样的、存储于栈上的那些类型之上的一个特殊注解，a special annotation。在某个类型实现了 `Copy` 特质时，使用此类型的那些变量，就不会迁移，相反会轻而易举地被复制，从而在赋值给另一变量后，令到他们依然有效。

在某个类型或类型的任何部分带有 `Copy` 特质时，Rust 就不会再允许以 `Drop` 特质对其加以注解了。若某个类型需要在其值超出作用域后，还要进行某些特殊处理，而又将 `Copy` 注解添加到了那个类型，那么就会收到编译时错误（if the type needs something special to happen when the value goes out of scope and we add the `Copy` annotation to that type, we'll get a compile-time error）。要了解如何将 `Copy` 注解，添加到自己编写的类型而实现这个 `Copy` 特质，请参阅附录 C 中 [可派生特质（derivable traits）](Ch21_Appendix.md#附录-c派生特质)。

那么到底哪些类型要实现 `Copy` 特质呢？可查阅给定类型的文档，来确定相应类型是否有实现 `Copy` 特质，不过作为一般规则，任何组别的简单标量值，any group of simple scalar values，都可实现 `Copy` 特质，以及不要求分配内存堆分配，或者其他形式资源的类型，也都可以实现 `Copy` 特质（any group of simple scalar values can implement `Copy`, and nothing that requires allocation or is some form of resource can implement `Copy`）。下面就是一些实现 `Copy` 特质的类型：

- 全部的整型，比如 `u32`；
- 布尔值类型，`bool`，即值 `true` 与 `false`；
- 全部浮点数类型，比如 `f64`;
- 字符类型，`char`;
- 只包含实现 `Copy` 特质类型的元组类型。比如 `(i32, i32)` 这个元组类型，就实现了 `Copy` 特质，而 `(i32, String)` 则没有。


### 所有权与函数

将值传递给函数的语法，与将值赋值给变量的语法，是类似的。将变量传递给函数，就会进行迁移或拷贝，这与赋值所做的别无二致。下面的清单 4-3 有着一个带有一些注解的示例，对其中的变量进入和超出作用域，进行了展示。

文件名：`src/main.rs`


```rust
fn main() {
    let s = String::from("hello");  // 变量 s 进到作用域

    takes_ownership(s);             // 变量 s 的值迁移到这个函数里头......
                                    // ......进而变量 s 因此不再有效

    let x = 5;                      // 变量 x 进到作用域

    makes_copy(x);                  // 变量 x 迁移到到这个函数里，
                                    // 但由于 i32 实现了 `Copy` 特质，因此
                                    // 后面在使用变量 x 也是没问题的
}   // 到这里，变量 x 超出了作用域，接着便是变量 s。但由于变量 s 的值已被迁移，因此
    // 这里不会有特别的事情发生。

fn takes_ownership(some_string: String) {   // 变量 some_string 进到作用域
    println! ("{}", some_string);
}   // 到这里，变量 some_string 便超出作用域，而 `drop` 方法就会被调用。some_string 的
    // 内存就被释放了。

fn makes_copy(some_integer: i32) {  // 变量 some_integer 进到作用域
    println! ("{}", some_integer);
}   // 到这里，变量 some_integer 超出作用域。没有特别事情发生。
```

*清单 4-3：带所有权与作用域注解的函数*

> 注：下面的代码，仍然会报出：`use of moved value: ``some_string```错误：

```rust
fn takes_ownership(some_string: String) {
    println! ("{}", some_string);
    another_takes_ownership(some_string);
    third_takes_ownership(some_string);
}
```

在对 `takes_ownership` 的调用之后，尝试使用变量 `s` 时，Rust 就会抛出一个编译时错误。这样的静态检查，保护咱们免于出错。请将使用变量 `s` 与变量 `x` 的代码，添加到 `main` 函数中，来观察一下在哪些地方可以使用他们，以及所有权规则会怎样阻止这样做。

### 返回值与作用域（return value and scope）

返回值也会转移所有权。下面的清单 4-4 给出了一个返回了某个值的函数示例，该示例有着与清单 4-3 中的那些类似的注释。

文件名：`src/main.rs`

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership 将其返回值
                                        // 迁移到变量 s1 中

    let s2 = String::from("hello");     // 变量 s2 进入作用域

    let s3 = takes_and_gives_bake(s2);  // 变量 s2 被迁移到 takes_and_gives_back
                                        // 中，该函数又将他的返回值迁移到变量 s3 中

    println! ("{}, {}", s1, s3);
}   // 到这里，变量 s3 超出作用域而被丢弃。变量 s2 已被迁移，因此什么也不会发生。而
    // 变量 s1 则超出作用域而被丢弃。

fn gives_ownership() -> String {    // 函数 gives_ownership 将把他的返回值，迁移
                                    // 到调用他的函数中（即 main 函数）
    String::from("归你了")          // 此表达式的值将被返回，并迁出到调用函数
}

// 此函数接收一个 String 并要返回一个 String
fn takes_and_gives_bake(a_string: String) -> String {   // a_string 进入作用域
    a_string    // a_string 被返回，并迁出到调用函数
}
```

*清单 4-4：返回值的所有权转移*

变量所有权每次都依循同一模式：在将值赋给另一变量时，所有权就会迁移。包含着内存堆上数据的某个变量，在超出作用域时，除非数据所有权已被迁移至另一变量，否则该值就会被 `drop` 给清理掉。

而在此模式生效时，每个函数下的取得所有权与随后的交回所有权，就有点乏味了。在要某个函数使用某个值而不占据其所有权时，会怎样呢？如果希望再度使用传入到函数中的全部东西，并还要把他们和那些可能要返回的函数体运算结果，一起再传回来，那样就很烦人了。

如下面的清单 4-5 所示，Rust 确实允许使用一个元组，返回多个值：

文件名：`src/main.rs`

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len): (String, usize) = calculate_length(s1);

    println! ("字符串 {} 的长度为：{}", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
```

*清单 4-5：返回参数所有权*

这虽然间接实现了消除变量所有权占据下，函数的使用变量，但对于这种本应常见的概念来说，这样做就过于花哨，且带来了大量工作负担。幸运的是，Rust 有着一项使用某个值而不转移所有权，名为 *引用（references）* 的特性。


## 引用与借用（references and borrowing）

清单 4-5 中那些元组代码的问题，是因为那个 `String` 值已被迁移到 `calculate_length` 函数中，因此那里就必须将那个 `String` 值，返回给调用函数（the calling funciton, 即清单 4-5 中的 `main` 函数），进而在对 `calculate_length` 的调用之后，仍然可以使用那个 `String` 的堆上数据。相反，咱们可以提供到那个 `String` 值的引用。所谓 *引用，reference*，与指针相似的是，在引用中的是个地址，咱们循着这个地址，就可以访问保存在那个地址处的数据，而这个数据则是为某个别的变量所拥有的。与指针不同的是，在引用存活期间，其保证是指向了特定类型有效值的。

以下是应如何定义和使用，将某个对象的引用作为参数，而非占用该值所有权的方式下的 `calculate_length` 函数：

文件名：`src/main.rs`

```rust
fn main() {
    let s1 = String::from("hello");

    let length = calculate_length(&s1);

    println! ("字符串 {} 的长度为：{}", s1, length);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

首先，注意到变量声明与函数返回值中的全部元组代码都不见了。其次，留意到这里是将 `&s1` 传入到 `calculate_length` 中的，同时在该函数的定义中，采用的是 `&String` 而非 `String`。这些 `&` 符号，these ampersands，表示了 *引用，references*，他们实现了在无需占用某个值所有权的情况下，引用到该值。下图 4-5 对此概念进行了描述。

![指向 `String s1` 的 `&String s` 图示](images/Ch04_05.svg)

*图 4-5：指向 `String s1` 的 `&String s` 图示*


> 注意：这种经由使用 `&` （取地址）运算符，而得到的变量引用的反面，即为 *解引用，dereferencing*，解引用是以解引用运算符 `*` 达成的。在第 8 章中就会看到这个 [解引用运算符的使用](Ch08_Common_Collections.md#对矢量中那些值的迭代)，而在第 15 章中，则会对解引用的细节加以讨论。

来细看一下这里的函数调用：

```rust
let s1 = String::from("hello");
let len = calculate_length(&s1);
```

这种 `&s1` 语法，实现了创建出一个 *指向，refers* 到 `s1` 的值，却不占有那个值的引用变量。由于引用不占有那个值，因此在引用停止使用（超出作用域）时，其所指向值就不会被弃用。

与此类似，那个函数签名同样使用 `&` 运算符，来表明参数 `s` 的类型是个引用。下面就来添加一些说明性的注解：

```rust
fn calculate_length(s: &String) -> usize {  // 变量 s 为到某个 String 值的引用
    s.len()
}   // 到这里，变量 s 超出作用域。但由于他并没有他指向值的所有权，因此什么
    // 也不会发生。
```

变量 `s` 于其间有效的那个作用域，与所有函数参数作用域是相同的，而由于变量 `s` 不拥有经引用而指向的那个值的所有权，因此在变量 `s` 停止被使用时，那个所指向的值就不会被丢弃。在函数以引用变量，而非真实值作为参数时，由于根本就没有拥有过所有权，那么就不再需要为了交回所有权，而将那些值返回了。

咱们把这种创建出引用的行为，叫做 *借用，borrowing*。正如日常生活中，当某人拥有某个物件时，咱们就可以把这个物件从那个人那里借用一下。在使用完毕后，咱们必须将其还回。咱们是不拥有该物件的。

那么在尝试修改某个正借用的物件时，又会发生什么呢？请尝试下面清单 4-6 中的代码。提前剧透一下：那代码就不会工作！

文件名：`src/main.rs`

```rust
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world!");
}
```

*清单 4-6：尝试修改被借用值，a borrowed value*

下面就是编译器报错：

```console
$ cargo run
   Compiling ownership_demo v0.1.0 (/home/peng/rust-lang/projects/ownership_demo)
error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
 --> src/main.rs:8:5
  |
7 | fn change(some_string: &String) {
  |                        ------- help: consider changing this to be a mutable reference: `&mut String`
8 |     some_string.push_str(", world!");
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable

For more information about this error, try `rustc --explain E0596`.
error: could not compile `ownership_demo` due to previous error
```

就跟变量默认是不可变的一样，引用也是默认不可变的。不允许修改所引用的某个物件。


### 可变引用

使用 *可变引用，mutable reference*，来取代默认不可变引用，只需一些小小调整，就可将清单 4-6 的代码，修改为允许对借用值，a borrowed value 加以修改：

文件名：`src/main.rs`

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    println! ("s：{}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}
```

首先，这里将变量 `s` 改为了 `mut`。随后在调用 `change` 函数处，以 `&mut s` 创建了一个可变的引用变量，并以 `some_string: &mut String`，将那个函数签名，更新为接受一个可变引用变量（a mutable reference）。这样做就很清楚地表明了，那个 `change` 函数将修改他借用的那个值。

可变引用变量有个大的限制：在有着到某值的一个可变引用时，就不能有到那个值的其他引用了。下面尝试创建到变量 `s` 两个可变引用的代码，就会失败：

文件名：`src/main.rs`

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println! ("{}, {}", r1, r2);
}
```

下面是编译器报错：

```console
$ cargo run
   Compiling ownership_demo v0.1.0 (/home/peng/rust-lang/projects/ownership_demo)
error[E0499]: cannot borrow `s` as mutable more than once at a time
 --> src/main.rs:5:14
  |
4 |     let r1 = &mut s;
  |              ------ first mutable borrow occurs here
5 |     let r2 = &mut s;
  |              ^^^^^^ second mutable borrow occurs here
6 |
7 |     println! ("{}, {}", r1, r2);
  |                         -- first borrow later used here

For more information about this error, try `rustc --explain E0499`.
error: could not compile `ownership_demo` due to previous error
```

此错误是说，由于在某个时间，多次将 `s` 借用做可变引用，而因此这段代码是无效的。首次可变借用是在 `r1` 中，而这次借用必须持续到其在那个 `println!` 中被使用为止，但就在那个可变引用的创建与使用中间，这里还尝试了在 `r2` 中，创建另一个借用了与 `r1` 同样数据的可变引用变量。

这种阻止在同一时间，到同一数据多重可变引用的限制，是允许修改的，但要在极度受控方式下进行（the restriction preventing multiple mutable references to the same data at the same time allows for mutation but in a very controlled fashion）。由于多数语言都允许随时修改数据，而因此多重可变引用正是一些新晋 Rust 公民们纠结不已的东西。有着这个限制的好处，则是 Rust 可以在编译时，对数据竞争加以阻止。与赛跑情形类似，*数据竞争，data race* 会在下面三种现象发生出现时出现：

- 同一时间有两个以上的指针访问着同一数据（two or more pointers access the same data at the same time）；
- 这些指针中至少有一个，正被用于写那个数据（at least one of the pointers is being used to write to the data）；
- 没有使用某种机制，来同步对数据的访问（there's no mechanism being used to synchronize access to the data）。

数据竞争导致未定义行为，并在尝试于运行时对其加以追踪的时候，难于排查诊断和修复；Rust 通过拒绝编译带有数据竞争的代码，而防止了这类问题！

与往常一样，可使用花括号来创建一个新的作用域，而实现多个可变应用变量，只要不是 *同时，simultaneous* 的几个就行：

```rust
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    }   // 由于在这里变量 r1 超出了作用域，因此就可以
        // 毫无问题地构造一个新的引用变量了。

    let r2 = &mut s;
```


对于将可变与不可变引用进行结合的情况，Rust 则会强制执行类似规则。下面的代码会导致错误：

```rust
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s;

    println! ("{}, {} 与 {}", r1, r2, r3);
```

下面就是那个错误：

```console
$ cargo run
   Compiling ownership_demo v0.1.0 (/home/peng/rust-lang/projects/ownership_demo)
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:14
  |
4 |     let r1 = &s;
  |              -- immutable borrow occurs here
5 |     let r2 = &s;
6 |     let r3 = &mut s;
  |              ^^^^^^ mutable borrow occurs here
7 |
8 |     println! ("{}, {} 与 {}", r1, r2, r3);
  |                               -- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `ownership_demo` due to previous error
```

咦！在有着对某个值的不可变引用时，*也是，also* 不可以对其有可变引用的。不可变引用的用户们，并不期望他们所引用的值，在他们眼皮底下突然就变掉！不过由于仅读取数据的不可变引用，对其他读取那个数据的引用，不具备造成影响的能力，因此多个不可变引用倒是可以的。

请注意引用变量的作用域，是从引入这个变量的地方开始，而持续到那个引用变量最后一次被使用为止。举例来说，由于那个不可变引用变量最后的使用，即那个 `println!`，是在那个可变引用变量引入之前发生的，因此下面的代码将会编译：

```rust
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println! ("r1 与 r2: {}, {}", r1, r2);
    // 变量 r1 与 r2 在此点位之后便不再被使用

    let r3 = &mut s;    // 这就没问题了
    println! ("r3: {}", r3);
```

不可变引用变量 `r1` 与 `r2` 的作用域，在 `println!` 语句，即他们最后被使用的地方之后便结束，而这个地方正是那个可变引用变量 `r3` 被创建之前。这些作用域不会重叠，因此这段代码是允许的。识别出引用变量在作用域结束之前的某处，不再被使用的编译器能力，叫做 *非词法性生命周期，Non-Lexical Lifetimes, 简写做 NLL*，在 [版本手册](https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/non-lexical-lifetimes.html) 里可阅读到更多有关内容。

虽然这些所有权借用方面的错误，时常令人沮丧，但请记住这正是 Rust 编译器，于早期阶段（在编译时而非运行时）就在指出潜在错误，并表明问题准确所在。代码编写者这才不必去追踪为何数据不是先前所设想的那样。

### 悬空引用，dangling references

在有着指针的那些语言中，都容易通过在保留了到某些内存的一个指针同时，释放了那些内存，而错误地创建出 *悬空指针，a dangling pointer* -- 引用了内存中，可能已经给了其他指针的某个地址的一个指针。在 Rust 中，与此相对照，编译器会确保引用绝不会成为悬空引用：在有着到某数据的引用时，编译器会确保在到该数据的引用，超出作用域之前，被引用的数据不超出作用域。

下面就来创建一个悬空引用，看看 Rust 如何以编译器错误，来阻止悬空引用：

文件名：`src/main.rs`


```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```

下面就是报错：


```console
$ cargo run
   Compiling ownership_demo v0.1.0 (/home/peng/rust-lang/projects/ownership_demo)
error[E0106]: missing lifetime specifier
 --> src/main.rs:5:16
  |
5 | fn dangle() -> &String {
  |                ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
  |
5 | fn dangle() -> &'static String {
  |                ~~~~~~~~

For more information about this error, try `rustc --explain E0106`.
error: could not compile `ownership_demo` due to previous error
```

此错误消息提到了一个这里还没有讲到特性：生命周期（lifetimes）。在第 10 章将 [详细讨论生命周期](Ch10_Generic_Types_and_Lifetimes.md#使用生命周期对引用加以验证)。不过，忽略掉生命周期有关的那部分错误，那么该错误消息就真的包含了，这段代码为何是问题代码的关键原因：

```console
this function's return type contains a borrowed value, but there is no value
for it to be borrowed from
```

下面来细看一下，这里的 `dangle` 代码各个阶段到底发生了什么：

文件名：`src/main.rs`

```rust
fn dangle() -> &String {    // 函数 dangle 返回的是到某个 String 值的引用
    let s = String::from("hello");  // 变量 s 是个新的 String 值

    &s  // 这里返回了一个到该 String，变量 s 的引用
}   // 到这里，变量 s 超出了作用域，进而被丢弃了。他的内存就没了。
    // 危险所在！
```

由于变量 `s` 是在函数 `dangle` 内部创建的，那么在函数 `dangle` 的代码执行完毕时，变量 `s` 就将被解除内存分配（deallocated）。而这里还在尝试返回一个到他的引用。那就意味着这个引用，就会指向到一个无效的 `String`。那就不好了！Rust 是不会允许这样干的。

这里的解决办法，就是直接返回那个 `String` 值：

```rust
fn dangle() -> String {
    let s = String::from("hello");

    s
}
```

### 引用的规则

下面来对前面已经讨论过有关引用的东西，进行一下总结回顾：

- 在任意给定时间点，都 *要么* 只能有一个可变引用，*要么* 有任意数量的不可变引用（at any given time, you can have *either* one mutable reference *or* any number of immutable references）；
- 引用必须一直有效（references must always be valid）。

接下来，将看看一种不同类别的引用：切片（slices）。


## 切片类型（the slice type）

*切片（slices）* 特性，实现了对集合中一个连续元素序列，而非对整个集合的引用。切片是引用的一种类别，因此他不会持有所有权。

这里有个小的编程问题：编写一个取得字符串，而返回在那个字符串中找到的第一个单词的函数。在函数在那个字符串中未找到空格时，那么这整个字符串就一定是一个单词，因此就要返回这整个字符串了。

下面就要在不使用切片特性的情况下，来看看该怎么编写这个函数的签名，从而搞明白切片要解决的问题：

```rust
fn first_word(s: &String) -> ?
```

这个 `first_word` 函数，有着一个作为参数的 `&String` 类型。这里不想要所有权，因此这是没问题的。不过应该返回什么呢？这里实在没有一种描述字符串 *局部（part）* 的方式。不过，这里可以返回那个单词的、以一个空格表示的结尾的索引。先来试试这个，如下面清单 4-7 所示：

文件名：`src/main.rs`

```rust
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
```

*清单 4-7：返回那个 `&String` 参数中一个字节索引值的 `first_word` 函数*

因为这里需要对这个 `String` 值元素挨个遍历，进而挨个检查值是否是个空格，因此这里就将使用 `as_bytes` 方法，把这个 `String` 值转换为字节的数组：

```rust
let bytes = s.as_bytes();
```

接着，这里使用 `iter` 方法，创建了一个在该字节数组上的迭代器：

```rust
for (i, &item) in bytes.iter().enumerate() {
```

在第 13 章，将讨论到迭代器的更多细节。而现在，明白 `iter` 是个返回集合中各个元素的方法，而那个 `enumerate` 则会将 `iter` 的结果进行封装进而将各个元素作为一个元组的组成部分，进行返回即可。自 `enumerate` 返回的元组第一个元素就是索引值，而第二个元素，则是到 `iter` 返回元素的索引。相比由代码编写者自己计算索引，这就要方便一点。

由于 `enumerate` 方法返回了一个元组，因此这里就可以使用模式，来解构那个元组。在 [第 6 章](Ch06_Enums_and_Pattern_Matching.md#绑定到值的模式)，会对模式进行更多讨论。在那个 `for` 循环中，指定了一个有着用于那个元组中索引的 `i`，以及用于那个元组中单个字节的 `&item` 的模式。由于这里获得的是一个到从 `.iter().enumerate()` 获取元素的引用，因此在那个模式中使用了 `&` 运算符。

在那个 `for` 循环内部，这里通过使用字节字面值语法（the byte literal syntax），就表示空格的字节进行了搜索。在找到空格时，就返回空格的位置。否则就通过使用 `s.len()` 返回该字符串的长度。

```rust
        if item == b' ' {
            return i;
        }
    }

    s.len()
```

现在就有了一种找出字符串中第一个单词末尾索引的方法了，不过这里有个问题。这里所返回的只是个 `usize`，然而这个返回值只是在 `&String` 的语境下，才是个有意义的数字。也就是说，由于这个返回的 `usize` 类型值，是从那个 `String` 值获取到的孤立值，因此就没办法保证在以后仍然有效。关于这点，可考虑在清单 4-8 中、用到了清单 4-7 中 `first_word` 函数的这么一个程序。

文件名：`src/main.rs`

```rust
fn main() {
    let mut s = String::from("The quick brown fox jumps over the lazy dog.");

    let word = first_word(&s);  // 变量 word 将获得值 5

    s.clear();  // 这个语句会清空该字符串，令其等于 ""

    // 到这里变量 word 仍有着值 5，但已经不再有那个可将值 5 有意义的运用
    // 到的字符串了。变量 5 现在完全无用了！
}
```

*清单 4-8：将来自调用 `first_word` 函数的结果存储起来，并在随后修改那个 `String` 值的内容*

该程序会不带任何错误地编译，且同样会在调用了 `s.clear()`后使用变量 `word` 时，其仍会完成后续执行（this program compiles without any errors and would do so if we used `word` after calling `s.clear()`）。由于变量 `word` 完全未被连接到变量 `s` 的状态，因此变量 `word` 仍包含着值 `5`。这里仍可使用那个值 `5` 与变量 `s`，来尝试提取出第一个单词，但由于自将值 `5` 保存在 `word` 中以来，变量 `s` 的内容已被修改，因此这样做将是个程序错误（a bug）。

这种不可避免的担心变量 `word` 中的索引，失去与变量 `s` 中的数据同步，就会十分烦人且容易发生错误！而在要编写 `second_word` 函数时，对这些索引的管理，将更加脆弱。`second_word` 的函数签名，将务必看起来像下面这样：

```rust
fn second_word(s: &String) -> (usize, usize) {
```

现在就得对一个开始 *和* 结束索引保持跟踪，同时甚至还有更多的、要从特定状态中的数据计算出的值，而这些值又完全没有与那种状态联系起来。这样就有了三个无关的、需要同步保持的变量漂浮着。

幸运的是，Rust 有此问题的解决办法，那就是：字符串切片（string slices）。

### 字符串切片

字符串切片是到某个 `String` 类型值部分的引用，而看起来像下面这样：

```rust
    let s = String::from("The quick brown fox jumps over the lazy dog.");

    let the = &s[0..3];
    let quick = &s[4..9];
```

与到整个 `String` 值的引用 `&s` 不同，`the` 是到这个 `String` 的，在那个附加 `[0..3]` 中所指明的一部分的引用。通过指定 `[start_index..ending_index]`，而使用了在一对方括号里的一个范围，这里创建出了切片，其中的 `starting_index` 是切片中首个位置，而 `ending_index` 则是比切片中最后位置多一的位置索引。切片数据结构内部，存储着开始位置与该切片的长度，长度即 `ending_index` 减去 `starting_index`。那么在示例 `let quick = &s[4..9];` 中，`quick` 就会包含一个到变量 `s` 的索引 `4` 处字节的指针。

下图 4-6 展示对此进行了展示。

![指向一个 `String` 数据局部的字符串切片](images/Ch04_06.svg)

*图 4-6：指向一个 `String` 数据局部的字符串切片*

在 Rust 的 `..` 范围语法，the `..` range syntax 之下，在希望于索引为零处开始时，那么就可以舍弃那两个点之前的值。也就是说，写开始索引 `0` 与不写，是等价的：

```
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];
```

对同一个字符串令牌，在切片包含了那个 `String` 的最后字节时，那么就可以舍弃那结尾的数字。即意味着下面的语句是等价的：

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];
```

要取用整个字符串时，还可以把开始与结束索引都舍弃掉。那么下面的语句就是等价的了：

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[0..len];
let slice = &s[..];
```

> **注意**：这些字符串切片的范围索引值，必须出现于有效的 UTF-8 字符边界处。若在 UTF-8 多字节字符中间，尝试创建字符串切片，那么程序就会以错误退出。这里只是为介绍字符串切片目的，而假定本小节中只使用 ASCII 字符；在第 8 章的 [“以 `String` 类型值存储 UTF-8 编码的文本”](Ch08_Common_Collections.md#使用-string-存储-utf-8-编码的文本) 小节，有着对 UTF-8 字符串的更全面讨论。


对这全部字符串切片的情况了然在胸，那么下面就来将 `first_word` 重写为返回切片。表示 “字符串切片” 的类型，写做 `&str`：

文件名：`src/main.rs`

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
```

这里是以前面清单 4-7 中所做的同样方式，即查找首次出现的空格，而获取到该单词结束处的索引。在找到空格时，就运用该字符串的开头，与那个空格的索引，作为字符串切片开始与结束索引，而返回一个字符串切片。

现在当调用 `first_word` 函数时，取回的便是与所用 `String` 数据联系起来单个值。这个值是由到切片起点的引用，与切片中元素个数所组成。

这样返回切片，对于 `second_word` 函数，也是有效的：

```rust
fn second_word(s: &String) -> &str {
```

由于编译器将确保到那个 `String` 数据中引用保持有效，因此现在就有了一个简单的、相比之前那个不那么容易搞混的 API 了。还记得在清单 4-8 中那个程序里的错误吧，即那个在已经获取到首个单词结束位置的索引，而随后清除了那个字符串，因此得到的索引就不在有效的问题。那段代码虽然逻辑上不正确，但也不会立即给出什么错误来。若继续尝试使用空字符串上的首个单词结束索引，这些问题仍会出现。切片就令到这个代码错误不可能了，并实现了更快发现代码问题。使用切片版本的 `first_word` 函数，就会抛出一个编译时错误：

文件名：`src/main.rs`

```rust
fn main() {
    let mut s = String::from("The quick brown fox jumps over the lazy dog.");

    let word = first_word(&s);

    s.clear();

    println! ("首个单词为：{}", word);
}
```

下面就是那个编译器错误消息：

```console
$ cargo run                                                                      ✔
   Compiling ownership_demo v0.1.0 (/home/peng/rust-lang/projects/ownership_demo)
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:5
  |
4 |     let word = first_word(&s);
  |                           -- immutable borrow occurs here
5 |
6 |     s.clear();
  |     ^^^^^^^^^ mutable borrow occurs here
7 |
8 |     println! ("首个单词为：{}", word);
  |                                 ---- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `ownership_demo` due to previous error
```

回顾借用规则，在有着到某数据的不可变引用时，就不能同时有可变引用。由于 `clear` 方法需要清空那个 `String` 值，那么就需要得到一个可变引用。而在 `clear` 方法调用之后的 `println!`，用到了变量 `word` 里的引用，那么这个不可变引用于那个时刻，就必将仍是活跃的。Rust 不允许 `clear` 中的可变引用，与 `word` 中的不可变引用同时存在，进而编译失败。可以看出，Rust 不光令到这个 `first_word` 的 API 更易于使用，他还在运行时就消除了这一整类错误！


### 字符串字面值即切片

还记得前面讲到过，那些硬编码的、存储在二进制可执行文件内部的字符串字面值吧。现在了解了切片，那么就可以很好理解字符串字面值了：

```rust
let s = "Hello, world!";
```

这里的变量 `s` 的类型，即为 `&str`：他是个指向到二进制文件特殊点位的一个切片。这也是为何字符串字面值为不可变的原因；`&str` 类型属于不可变引用。


### 字符串切片作为函数参数

了解了咱们可在函数中，取字符串字面值的切片及 `String` 值，就引出了对 `first_word` 函数的又一项改进，而下面就是函数 `first_word` 的签名：

```rust
fn first_word(s: &String) -> &str {
```

更老道的 Rust 公民将把这个函数签名，写着像下面清单 4-9 中所展示的那样，这是因为下面这样写，就实现了在 `&String` 与 `&str` 两种类型值上，可使用同一个函数：

```rust
fn first_word(s: &str) -> &str {
```

*清单 4-9：通过对 `s` 参数的类型使用字符串切片，对 `first_word` 函数进行改进*

在咱们有着某个字符串切片时，那么就可以直接传递那个字符串切片。而在咱们有着一个 `String` 时，则可传递该 `String` 的切片，或到这个 `String` 的引用。这种灵活性，是利用了 *强制引用解除，deref coercions* 特性，在第 15 章的 [函数与方法下的隐式强制解引用](Ch05_Smart_Pointers.md#函数与方法下的隐式解引用强制转换) 小节，将讲到的一种特性。

这样定义出取字符串切片，而非到 `String` 值引用做参数的函数，令到这个 API 在不丢失任何功能的情况下，变得更为通用和有用：

文件名：`src/main.rs`

```rust
fn main() {
    let s = String::from("The quick brown fox jumps over the lazy dog.");

    // 函数 first_word 在 String 值的切片上有效，不管是部分还是全部的切片
    let word = first_word(&s[0..6]);
    println! ("{}", word);

    let word = first_word(&s[..]);
    println! ("{}", word);

    // 函数 first_word 还在 String 变量的引用上有效，而 String 变量的引用
    // 与 String 值的整个切片是等价的
    let word = first_word(&s);
    println! ("{}", word);

    let s_string_literal = "hello word";

    // 函数 first_word 在字符串字面值上有效，不论是部分还是整体
    let word = first_word(&s_string_literal[0..6]);
    println! ("{}", word);

    let word = first_word(&s_string_literal[..]);
    println! ("{}", word);

    // 由于字符串字面值已经 是 字符串切片，因此无需切片语法，这
    // 也是有效的!
    let word = first_word(s_string_literal);

    println! ("{}", word);
}
```

### 其他切片

或许已经想到，字符串切片是特定于字符串的。然而还有更多通用切片类型呢。请看下面这个数组：

```rust
let a = [1, 2, 3, 4, 5];
```

就跟要引用字符串的部分一样，也可能要引用数组的部分。那么就将像下面这样，来完成对数组一部分的引用：

```rust

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq! (slice, &[2, 3]);
```

这个切片变量 `slice` 的类型为 `&[i32]`。数组切片的原理与字符串切片一样，都是经由存储到首个元素的引用，和切片长度实现的。今后将对所有类别的其他集合，运用到这种切片。在第 8 章讲到各种矢量时，就会对这些集合加以讨论。


## 本章小结

所有权、借用及切片等概念，在编译时确保了 Rust 程序中的内存安全。Rust 语言所给到的对内存运用的掌控方式，与别的系统编程语言相同，但会让数据的所有者，在其超出作用域时，自动清理掉其数据，这就意味着咱们不必编写并调试额外代码，来实现这种控制。

所有权对 Rust 程序的许多其他部分都有影响，因此在本书其余部分，都将更进一步的涉及到这些所有权的概念。接下来就要移步第 5 章，而在结构体 `struct` 中，如何将小块数据组装起来。
