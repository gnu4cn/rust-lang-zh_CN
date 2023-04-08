# 高级特性

**Advanced Features**

到此时，咱们业已学习了 Rust 编程语言的那些最为常用部分。在第 20 章中咱们完成另一个项目之前，将看看咱们可能会偶尔碰到，但却不会每天用到的这门语言的一些方面。咱们可将这一章，当作在今后遇到一些不明白之处时的一份参考。这里所涵盖的特性，在一些非常特定情形下是有用的。尽管咱们可能不会经常碰到这些情形，咱们还是希望，能掌握 Rust 所提供到的全部特性。

咱们将在这一章，涵盖以下内容：

- 不安全的 Rust, unsafe Rust: 怎样选择不使用 Rust 的一些保证，而由程序员亲自负责维持这些保证；
- 高级特质，advanced traits: 关联类型，associated types、默认类型参数，default type parameters、完全合格语法，fully qualified syntax、超特质，supertraits 及与特质相关的新型模式，the newtype pattern in relation to traits;
- 高级类型：更多有关新型模式的内容、类型别名，type aliases、永恒类型，the never type 以及动态大小的类型，dynamically sized types；
- 高级函数与高级闭包：函数指针与返回的闭包，function pointers and returning closures；
- 宏，macros：那些在编译时定义了别的代码的代码定义方式，ways to define code that defines more code at compile time。


本章是给每个人应该了解的，一整套 Rust 特性！咱们就开始吧！


## 不安全的 Rust

**Unsafe Rust**

到目前为止，本书所讨论的全部代码，都曾在编译时，将 Rust 的内存安全保证进行了强制执行。然而，Rust 内部有着另一种不强制进行这些内存安全保证的语言：他被叫做 *不安全的 Rust，unsafe rust*，而其与常规 Rust 工作类似，只不过赋予了咱们额外的超能力。

不安全 Rust 之所以存在，是因为静态分析，static analysis 天生是保守的。在编译器尝试判断出代码是否维持了那些保证时，相比接受一些无效程序，则退回一些有效程序会更佳。尽管代码 *可能* 没有问题，在 Rust 编译器没有足够信息对代码有信心时，他就会退回该代码。在这些情况下，咱们就可以使用不安全代码特性，来告诉编译器，“请相信我，我明白我在做什么。”但请当心，使用不安全 Rust 要风险自担：若不当使用非安全代码，那么由内存不安全而导致的问题就会发生，比如空指针的解引用。

Rust 有着一个非安全的另外自我，an unsafe alter ego，的另一原因，便是所采行的计算机硬件本质上是不安全的。若 Rust 不允许咱们执行非安全操作，那么咱们就无法完成一些特定任务。Rust 需要允许咱们完成一些底层系统变成，诸如直接与操作系统交互，或甚至编写咱们自己的操作系统。而进行底层编程工作，是这门语言的目标之一。下面就来探讨，咱们可以使用非安全 Rust 做些什么，以及怎样使用非安全 Rust。


### 不安全的超级能力

**Unsafe Superpowers**

要切换到非安全 Rust，就要使用 `unsafe` 关键字，并于随后开启一个驻留着非安全代码的新代码块。在非安全 Rust 中，可以进行安全 Rust 所不能进行的五种行为，咱们把这些行为叫做 *不安全的超能力，unsafe superpowers*。这些超能力包括了实现下面这些的能力：

- 解引用某个原始指针，dereference a raw pointer;
- 调用某个非安全的函数或方法；
- 访问或修改某个可变静态变量；
- 实现某个非安全特质；
- 访问 `union` 类型的那些字段。


明白 `unsafe` 关键字，并不会关闭借用检查器或停用任何其他的 Rust 安全性检查，是重要的：当咱们在非安全代码中用到某个引用时，其仍将受检查。`unsafe` 关键字只给到咱们访问随后不受编译器内存检查的这五种特性访问。在非安全代码块内部，咱们仍将获得一定程度的安全性。

此外，`unsafe` 并不意味着其代码块内的代码就必然是危险的，或是明显将有着内存安全问题：其意图是作为编程者的咱们，将确保 `unsafe` 代码块内部的代码将以有效的方式访问内存。

人是容易犯错误的，而错误就会发生，但通过要求将这五种非安全操作，置于以 `unsafe` 做标记出的代码块中，咱们就将清楚，任何与内存安全相关的错误，都必须在某个 `unsafe` 代码块里。请保持那些 `unsafe` 代码块较小；当咱们在调查内存错误时，就会对这种做法感激不尽。

为尽量隔离非安全代码，最佳做法即把非安全代码，封闭在安全抽象里，而提供一个安全的 API，在本章检视到非安全函数及方法时，咱们将讨论这个问题，to isolate unsafe code as much as possible, it's best to enclose unsafe code within a safe abstraction and provide a safe API, which we'll discuss later in the chapter when we examing unsafe functions and methods。标准库的一些部分，即是作为已审核过的非安全代码的安全抽象，而实现的。由于运用安全抽象是安全的，因此将非安全代码封装在安全抽象中，就阻止了 `unsafe` 的运用，溢出到可能会用到以 `unsafe` 代码实现功能的全部处所。

下面就来依次看看，每个的这五种超能力。咱们还将看看一些提供了到非安全代码的安全接口的一些抽象。


### 解引用原始指针

**Dereferencing a Raw Pointer**

在第 4 章的 [悬空引用](Ch04_Understanding_Ownership.md#dangling-references) 小节，咱们曾提到编译器会确保引用始终有效。不安全的 Rust 则有着与引用类似的， 叫做 *原始指针，raw pointers* 的两种新类型。与引用一样，原始指针可以是不可变或可变的，并被相应地写作 `*const T` 及 `*mut T`。其中的星号 `*` 并非是解引用运算符；他是这种类型名字的一部分。在原始指针语境下，*不可变，immutable* 意指该指针在被解引用之后，不能被直接赋值。

与引用及灵巧指针不同，原始指针有着以下特征：

- 通过同一内存位置上的可变及不可变指针，或多个到内存同一位置上的可变指针，原始指针允许忽略借用规则；
- 原始指针不保证指向有效的内存；
- 原始指针允许为空 `null`；
- 原始指针不会实现任何的自动清理。


经由选择不让 Rust 强制执行这些保证，咱们就可以放弃（编译器）保证的安全性，而换得更佳的性能，或与其他语言或与硬件交互的能力，二者都是在 Rust 的保证中没有实现的。

下面清单 19-1 给出了怎样从引用创建出不可变与可变原始指针的方式：

```rust
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;


    println! ("{:?}, {:?}", r1, r2);
```

*清单 19-1：自引用创建原始指针*

> 运行结果如下：

```console
$ cargo run
   Compiling raw_pointers v0.1.0 (/home/lenny.peng/rust-lang/raw_pointers)
    Finished dev [unoptimized + debuginfo] target(s) in 0.59s
     Running `target/debug/raw_pointers`
0x7ffc2c28eb84, 0x7ffc2c28eb84
```

> 注：这里的两个内存地址一样，但每次运行会显示不同的内存地址。

请注意在此代码中，咱们并未包含 `unsafe` 关键字。咱们可在安全代码中，创建原始指针；只是咱们无法在非安全代码块外部，解引用原始指针，后面马上就将看到这一点。

为验证这一点，接下来咱们将创建咱们不能那么确定其有效性的一个原始指针。下面清单 19-2 给出了怎么创建到内存中任意位置的一个原始指针。尝试使用任意内存，属于不明确行为：在那个地址处可能有数据，或可能没有，编译器就可能优化该代码，如此就没有了内存访问，或是该程序可能以段错误，a segmentation fault，而出错。通常，像下面这样编写代码并无好的理由，但这样写是可能的。

```rust
    let address = 0x012345usize;
    let r = address as *const i32;

    println! ("{:?}", r);
```

*清单 19-2：创建到任意内存地址的一个原始指针*

回顾到咱们可在安全代码中创建原始指针，但咱们不能 *解引用，deference* 原始指针及读取所指向的数据。下面清单 19-3 中，咱们在要求 `unsafe` 代码块的一个原始指针上，使用了解引用运算符 `*`。

```rust
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println! ("r1 为：{}", *r1);
        println! ("r2 为：{}", *r2);
    }
```

*清单 19-3：在 `unsafe` 代码块里解引用原始指针*

> 运行结果如下：

```console
$ cargo run
   Compiling raw_pointers v0.1.0 (/home/lenny.peng/rust-lang/raw_pointers)
    Finished dev [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/raw_pointers`
r1 为：5
r2 为：5
```

创建指针没有什么害处；只有在咱们尝试访问其所指向的值可能遇到无效值时，才会造成危害。

还要注意在清单 19-1 与 19-3 中，咱们创建的 `*const i32` 与 `*mut i32` 两个原始指针，都指向了同一内存地址，及 `num` 所存储之处。相反若咱们尝试创建到这个 `num` 的一个不可变与可变的引用，那么由于 Rust 的所有权规则在有任何不可变引用的同时，允许可变引用，该代码就不会被编译。有了原始指针，咱们就可以创建到同一内存地址的可变指针与不可变指针，而经由那个可变指针修改数据，就会潜在的造成数据竞争。所以请当心！

在全部的这些危险之下，咱们为何还要使用原始指针呢？一个主要的原因就是在与 C 代码交互时，正如将在下一小节，[”调用非安全函数或方法“](#calling-an-unsafe-function-or-method)，中将看到的。另一中情况，便是在构建借用检查器不清楚的一些安全抽象时。咱们将介绍非安全函数，并在随后看看一个用到不安全代码的安全抽象。

### 调用不安全函数或方法

在非安全代码块中咱们所能进行的第二种操作，便是调用不安全函数了。不安全函数与方法看起来就像是常规函数与方法，但他们在其余定义之前，有个额外的 `unsafe` 关键字。由于 Rust （编译器）无法保证咱们在调用该函数时，业已满足一些要求，而因此这个 `unsafe` 关键字，就表明了其本身就有着这些要求。通过在 `unsafe` 代码块中调用某个不安全函数，就是说咱们为遵守该函数的合约，而已经阅读了这个函数的文档。

下面即为一个未在其函数体中实现任何东西的名为 `dangerous` 的不安全函数：

```rust
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }
```

咱们必须在一个单独的 `unsafe` 代码块里调用这个 `dangerous` 函数。若咱们尝试在那个 `unsafe` 代码块外部调用 `dangerous`，就将得到一个报错：

```console
$ cargo run
   Compiling unsafe_functions v0.1.0 (/home/lenny.peng/rust-lang/unsafe_functions)
error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
 --> src/main.rs:6:5
  |
6 |     dangerous();
  |     ^^^^^^^^^^^ call to unsafe function
  |
  = note: consult the function's documentation for information on how to avoid undefined behavior

For more information about this error, try `rustc --explain E0133`.
error: could not compile `unsafe_functions` due to previous error
```

而在 `unsafe` 代码块下，咱们便是在对 Rust 声称，咱们已经阅读了该函数的文档，明白如何恰当地使用他，以及咱们已经检查过咱们履行了这个函数合约。

不安全函数的函数体，都是有效的一些 `unsafe` 代码块，因此就可以在不安全函数里执行其他一些不安全操作，而无需添加别的 `unsafe` 代码块。


#### 创建非安全代码的安全抽象

**Creating a Safe Abstraction over Unsafe Code**

仅仅因为某个函数包含了不安全代码，并不意味着咱们就需要将这整个函数标记为 `unsafe`。事实上，将不安全代码封装在安全函数中，就是一种常见的抽象。作为一个示例，下面咱们就来研究一下标准库中的 `split_at_mut` 函数，其就需要一些不安全代码。咱们将探讨咱们该怎样实现他。这个安全方法是定义在可变切片上的：他会取得一个切片，并通过于作为参数给定的索引处分割这个切片，而将其构造为两个切片。下面清单 19-4 给出了使用 `split_at_mut` 函数的方式：

```rust
    let mut v = vec! [1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq! (a, &mut [1, 2, 3]);
    assert_eq! (b, &mut [4, 5, 6]);
```

*清单 19-4：使用安全的 `split_at_mut` 函数*

仅使用安全的 Rust，咱们是没法实现这个函数的。一种尝试可能看起来像清单 19-5 那样，其不会编译。为简化起见，咱们将把 `split_at_mut` 实现为一个函数而非方法，并只对 `i32` 的值而非泛型 `T` 实现。

```rust
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();

    assert! (mid <= len);

    (&mut values[..mid], &mut values[mid..])
}

```

*清单 19-5：仅使用安全的 Rust 的`split_at_mut` 的一种实现尝试*

这个函数首先得到的是那个切片的总长度。随后其通过检查作为参数所给到的索引小于等于这个总长度，而断言了该索引是在切片里的。这个断言意味着在咱们传入了大于要分割切片长度的一个索引时，该函数将在他尝试使用那个索引前终止运行。

随后咱们返回了在一个元组中的两个可变切片：一个来自原始切片开头到 `mid` 索引处，而另一个则是来自从 `mid` 处到那个切片的末尾。

当咱们尝试编译清单 19-5 中的代码时，就将得到一个报错：


```rust
$ cargo run
   Compiling safe_abstraction v0.1.0 (/home/lenny.peng/rust-lang/safe_abstraction)
error[E0499]: cannot borrow `*values` as mutable more than once at a time
 --> src/main.rs:8:31
  |
3 | fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
  |                         - let's call the lifetime of this reference `'1`
...
8 |     (&mut values[..mid], &mut values[mid..])
  |     --------------------------^^^^^^--------
  |     |     |                   |
  |     |     |                   second mutable borrow occurs here
  |     |     first mutable borrow occurs here
  |     returning this value requires that `*values` is borrowed for `'1`

For more information about this error, try `rustc --explain E0499`.
error: could not compile `safe_abstraction` due to previous error
```

Rust 的借用检查器无法搞清楚，咱们是在借用那个切片的不同部分；他只知道咱们借用了同一切片两次。由于借用切片的两个不同部分没有重叠，因此这样做从根本上讲是可以的，但 Rust 没有足够聪明到明白这点。在咱们清楚代码是没有问题的，而 Rust 并不清楚时，你们就是要用到不安全代码的时候了。

清单 19-6 给出了如何使用一个 `unsafe` 代码块、一个原始指针，以及一些到非安全函数的调用，来领到这个 `split_at_mut` 实现工作的方式。


```rust
use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert! (mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
```

*清单 19-6： 在 `split_at_mut` 函数实现中使用不安全代码*

回顾第 4 章中的 [“切片类型”](Ch04_Understanding_Ownership.md#the-slice-type) 小节，切片即为到一些数据的指针，与切片的长度。咱们使用了 `len` 方法，来获取切片的长度，并使用 `as_mut_ptr` 方法来访问切片的原始指针。在这个示例中，由于咱们有着一个到一些 `i32` 值的可变切片，`as_mut_prr` 就会返回类型 `*mut i32` 的原始指针，其已被咱们存储在变量 `ptr` 中。

咱们保留了那个 `mid` 索引是在切片里的断言。随后咱们就到了那不安全代码处：`slice::from_raw_parts_mut` 函数会取一个原始指针及长度，并创建出一个切片。咱们使用这个函数，来创建自 `ptr` 开始，且长度为 `mid` 的一个切片。随后咱们以 `mid` 作为参数，调用 `ptr` 上的 `add` 方法，来得到于 `mid` 处开始的一个原始指针，而咱们创建出使用那个指针，且以 `mid` 之后项目数量为长度的一个切片。

由于函数 `slice::from_raw_parts_mut` 取了一个原始指针，且必须相信这个指针是有效的，因此该函数是不安全的。由于原始指针上的 `add` 方法必须相信那个偏移地址亦为有效指针，故其也是不安全的。因此，咱们就不得不在这些到 `slice::from_raw_parts_mut` 及 `add` 的调用周围，放置一个 `unsafe` 代码块，从而才可以调用他们。通过查阅代码，及添加上 `mid` 务必小于等于 `len` 的断言，咱们就可以讲，在那个 `unsafe` 代码块里用到的全部原始指针，都将是到那个切片里数据的有效指针。这便是 `unsafe` 可接受及合理的使用。

请注意咱们无需将所得的 `split_at_mut` 函数标记为 `unsafe`，且咱们可以从安全的 Rust 调用这个函数。由于这个函数实现只会创建出其所访问数据的有效指针，因此他是以安全方式使用的 `unsafe` 代码，而咱们则以这个函数实现，就已经创建到非安全代码的安全抽象了。

作为对照，下面清单 19-7 中 `slice::from_raw_parts_mut` 的使用，于那个切片被用到时，大致就会崩溃。此代码取的是一个任意内存地址，并创建了有着 10,000 个条目长的切片。

```rust
    use std::slice;

    let address = 0x01234usize;
    let r = address as *mut i32;

    let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
```

*清单 19-7：自任意内存地址创建切片*


> *注*：上面的代码运行结果：


```console
$ cargo run
   Compiling safe_abstraction v0.1.0 (/home/lenny.peng/rust-lang/safe_abstraction)
    Finished dev [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/safe_abstraction`
```

> 可见并无报错，但若加上 `println! (":?", values);` 语句，运行结果将如下：


```console
$ cargo run
   Compiling safe_abstraction v0.1.0 (/home/lenny.peng/rust-lang/safe_abstraction)
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/safe_abstraction`
Segmentation fault (core dumped)
```

> 报出了段错误。

咱们并不拥有位于此任意地址处的内存，且没有此代码所创建出的切片，包含着一些有效 `i32` 值方面的保证。那么尝试将 `values` 当作其为有效切片使用，就会导致未定义行为，undefined behavior。


#### 运用 `extern` 函数来调用外部代码

**Using `extern` Functions to Call External Code**

有的时候，咱们的 Rust 代码可能需要跟以其他语言编写的代码交互。为这个目的，Rust 有着一个推动 *异种函数接口，Foreign Function Interface, FFI* 的创建与运用的 `extern` 关键字。所谓 FFI，是某门编程语言用于定义出一些函数，并实现一门别的（异种）编程语言来调用这些函数的方式。

下面清单 19-8 演示了怎样建立与来自 C 语言标准库 `abs` 函数的集成。从 Rust 代码调用 `extern` 代码块中声明的函数，总是不安全的。原因在于其他语言没有强制执行 Rust 的规则与保证，同时 Rust 无法对他们加以检查，因此确保安全性的责任，就落在编程者身上。

文件名：`src/main.rs`

```rust
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println! ("C 语言中 -3 的绝对值为：{}", abs(-3));
    }
}
```

*清单 19-8：声明并调用定义在别的语言中的 `extern` 函数*


> 上面代码运行结果为：

```console
$ cargo run
   Compiling extern_code v0.1.0 (/home/lenny.peng/rust-lang/extern_code)
    Finished dev [unoptimized + debuginfo] target(s) in 0.37s
     Running `target/debug/extern_code`
C 语言中 -3 的绝对值为：3
```

在那个 `extern "C"` 代码块里头，咱们列出了咱们打算调用的，来自另一语言的函数名字与签名。其中的 `"C"` 部分，定义了外部函数用到的何种 *应用二进制接口，application binary interface, ABI*：正是 ABI，定义了在汇编层面，at the assembly level，调用该函数的方式。而这个 `"C"` ABI，便是最常用的，且其遵循着 C 编程语言的 ABI。


> **自其他语言调用 Rust 的函数，calling Rust functions from other languages**
>
> 咱们还可以使用 `extern` 关键字，来创建允许其他语言调用 Rust 函数的接口。与创建出整个 `extern` 代码块不同，咱们只是要在相关函数的 `fn` 关键字前，添加 `extern` 关键字，并指定出要使用的 ABI。咱们还需添加一个 `#[no_mangle]` 注解，来告诉 Rust 编译器不要修饰这个函数的名字，mangle the name of this function。所谓 *名字修饰，Mangling*，是在编译器将咱们给到某个函数的名字，修改为别的包含了给到编译过程其他部分消费的更多信息，但对人类更难于阅读名字的做法。各种编程语言的编译器，对名字的修饰会略有不同，因此为了 Rust 函数可被其他语言命名，咱们就必须关闭 Rust 编译器的名字装饰。
>
> 在下面的示例中，咱们构造了一个其被编译到共享库，a shared library，并从 C 代码链接后，便可从 C 语言代码访问的 `call_from_c` 函数：

```rust
#[no_mangle]
pub extern "C" fn call_from_c() {
    println! ("刚从 C 调用了一个 Rust 函数！");
}
```

> `extern` 的这种用法，不需要 `unsafe` 关键字。

### 访问或修改可变静态变量

**Accessing or Modifying a Mutable Static Variable**

在本书中，咱们还不曾讲到过 *全局变量，global variables*，其不受 Rust 不支持而会与 Rust 的所有权规则发生问题。在两个线程都访问同一可变全局变量时，就会引起数据竞争。

在 Rust 中，全局变量被称为 *静态，static* 变量。下面清单 19-9 给出了有着字符串切片作为值的，一个静态变量的示例声明与运用。


文件名：`src/main.rs`

```rust
static HELLO_WORLD: &str = "你好，世界！";

fn main() {
    println! ("名字为：{}", HELLO_WORLD);
}
```

*清单 19-9：定义并使用不可变静态变量*

静态变量与咱们曾在第三章中 [“变量与常量区别”](Ch03_Common_Programming_Concepts.md#constants) 小节讨论过的常量类似。静态变量的名字，依约定都是 `SCREAMING_SNAKE_CASE` 形式。静态变量只能存储有着 `'static` 声明周期的引用，这意味着 Rust 编译器可以计算出声明周期，而不要求咱们显式地对其加以注解。访问不可变的静态变量是安全的。

常量与不可变静态变量的细微差别在于，静态变量里的值在内存中有着固定地址。用到该值就总是将访问同一数据。而另一方面的常量，则凡是在用到他们时，都是允许复制他们数据的。另一不同便是，静态变量可以是可变的。访问与修改可变静态变量是 *不安全的*。下面清单 19-10 给出了如何声明、访问及修改名为 `COUNT` 的可变静态变量方式。


文件名：`src/main.rs`

```rust
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);

    unsafe {
        println! ("COUNTER: {}", COUNTER);
    }
}
```

*清单 19-10：读取自或写入到可变静态变量均为不安全的*

与常规变量一样，咱们使用 `mut` 关键字指明可变性。任何读或写 `COUNTER` 的代码，都必须是在 `unsafe` 代码块里。由于这段代码是单线程的，因此其会如咱们预期的那样，编译并打印 `COUNTER: 3`。让多个线程访问 `COUNTER`，就可能会导致数据竞争。

在全局可访问的可变数据之下，就难于确保没有数据竞争，这就是 Rust 为何将可变静态变量视为不安全的原因。在可行条件下，就要首选运用并发技巧，以及在第 16 章中曾讨论过的线程安全的灵巧指针，从而编译器将就自不同线程访问数据以安全方式完成，而加以检查。


### 实现不安全的特质

**Implementing an Unsafe Trait**

咱们可以使用 `unsafe`，来实现不安全的特质。在至少有一个特质的方法有着编译器无法验证的一些定数，some invariant that the compiler can't verify，时，那么这个特质便是不安全的。通过在 `trait` 关键字前加上 `unsafe` 关键字，并将特质的实现也标记为 `unsafe`，咱们就把这个特质声明为了 `unsafe`，如下清单 19-11 中所示。


```rust
unsafe trait Foo {
    // 这里是些方法
}

unsafe impl Foo for i32 {
    // 方法实现在这里
}

fn main() {}
```

*清单 19-11：定义并实现不安全的特质*


通过使用 `unsafe impl`，咱们就承诺咱们将坚守那些编译器无法验证的定数，we'll uphold the invariants that the compiler can't verify。

作为示例，请回顾第 16 章中 [“`Sync` 与 `Send` 特质下的可扩展并发”](Ch16_Fearless_Concurrency.md#extensible-concurrency-with-the-sync-and-send-trait") 小节中，曾讨论过的 `Sync` 与 `Send` 两个标记性特质：在咱们的类型完全是由 `Send` 与 `Sync` 两种类型构成时，编译器就会自动实现这些特质。而在咱们实现某个包含了非 `Send` 或 `Sync` 的类型，比如原始指针，同时咱们打算将那个类型标记为 `Send` 或 `Sync` 时，咱们就必须使用 `unsafe`。Rust 无法验证咱们的类型坚守了其可被跨线程安全发送，或自多个线程安全访问的那些保证；因此，咱们就需要手动完成这些检查，并以 `unsafe` 照这样加以表明。

### 访问联合体的字段

**Accessing fields of a union**

使用 `unsafe` 的就只剩下最后的用法了，那便是访问 *联合体，union* 的字段。`union` 与 `struct` 类似，但一次只会用到特定实例中一个声明的字段。联合体主要用于与 C 语言代码中的联合体交互。由于 Rust 无法保证在联合体示例当前所存储的数据类型，因此访问联合体字段是不安全的。在 [Rust 参考手册](https://doc.rust-lang.org/reference/items/unions.html) 中，可了解更多有关联合体的知识。


### 何时使用不安全代码

**When to use unsafe code**

运用 `unsafe` 来采取上述五种做法（超能力）没有什么过错，或者不受欢迎。但由于编译器无法助力于保持内存安全，因此要让 `unsafe` 代码正确就更为棘手一些。在有使用 `unsafe` 代码的某种理由时，就可以这样做，而在问题出现时，显式的 `unsafe` 注解，就会令到排查问题原因更为容易。


## 高级特质

在第 10 章 [“特质：定义共用行为”](Ch10_Generic_Types_Traits_and_Lifetimes.md#trait-defining-shared-behavior) 小节中，咱们曾首先涉及到特质，但咱们不曾讨论更为高级的那些细节。现在咱们对 Rust 有了更多了解，咱们就可以深入本质，get into the nitty-gritty。


### 使用关联类型指定出特质定义中的一些占位性类型

**Specifying placeholder types in trait definitions with associated types**

*关联类型* 将类型占位符与特质加以结合，从而那些特质方法的定义，就可以在他们的签名中，使用这些占位符类型。特质的实现者，将为其特定实现，指明占位符类型所要使用的具体类型。如此一来，咱们便可以在特质被实现之前，无需准确获悉特质用到的类型下，定义出用到这些类型的特质。

在本章中，咱们已经介绍了绝大多数极少需要用到的高级特性。而关联类型则是位于这些高级特性中部的一种：其相较本书其余部分降到的那些特性，用得尤其少见，但相较这一章中讨论到的其他特性，则其要更常用一些。

带有关联类型特质的一个示例，便是标准库所提供的 `Iterator` 特质。其中的关联类型名为 `Item`，且代表着实现了这个 `Iterator` 特质的类型所迭代的那些值的类型。`Iterator` 特质的定义如下清单 19-12 中所示。


```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

*清单 19-12：有着关联类型 `Item` 的 `Iterator` 特质的定义*

其中的类型 `Item` 便是个占位符，而那个 `next` 方法的定义，则显示其将返回类型类型为 `Option<Self::Item>` 的值。`Iterator` 的实现者，将指明 `Item` 的具体类型，同时 `next` 方法将返回包含那个具体类型值的一个 `Option`。

从泛型允许咱们在不指明函数可处理何种类型下，而定义出某个函数上看，关联类型可能看起来是个与泛型类似类似的概念。为检视这两个概念的不同，咱们将看看在指定了 `Item` 类型为 `u32` 的一个名为 `Counter` 的类型上的 `Iterator` 实现：

```rust
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --跳过代码--
```

这种语法似乎可与泛型的那种语法相比。那么为何没有只使用泛型定义 `Iterator`，如下清单 19-13 中所示的那样呢？

```rust
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}
```

*清单 19-13：使用泛型的一种 `Iterator` 特质的定义假设*

不同之处在于，如同清单 19-13 中使用泛型时，咱们必须注解每个实现中的那些类型；由于咱们还可以实现 `Iterfator<String> for Counter` 或任何其他类型，咱们可以有对 `Counter` 的多个 `Iterator` 实现。也就是说，在特质有着泛型参数时，他就可以对某个类型被实现多次，每次都修改泛型参数的具体类型。当在 `Counter` 上使用 `next` 方法时，咱们就将不得不提供类型注解，来表明咱们想要使用哪个 `Iterator` 实现。

而在关联类型下，由于我们无法在一个类型上多次实现某个特质，因此就无需注解类型。在上面有着用到关联类型定义的清单 9-12 中，由于只能有一个 `impl Iterator for Counter`，所以咱们就只能就`Item` 为何选择一次。咱们不必在 `Counter` 上调用 `next` 的每个地方，指定咱们所要的是个 `u32` 值的迭代器。


关联类型还成了特质合约的一部分：特质的实现着必须提供一种类型，来顶替那个关联类型占位符。关联类型通常会有个描述该类型将被如何使用的名字，而在 API 文档中对关联类型编写文档，则是良好的做法。


### 默认泛型参数与运算符的重载

**Default Generic Type Parameters and Operator Overloading**

> **注**：请参考 [Difference Between Method Overloading and Method Overriding in Java](https://www.geeksforgeeks.org/difference-between-method-overloading-and-method-overriding-in-java/) 了解 Java 中的重载与重写区别。

在咱们用到泛型参数时，咱们可以给泛型指定默认具体类型。在所指定的默认类型就有效时，这样做消除了实现者指定具体类型的需求。在声明泛型时使用 `<PlaceholderType=ConcreteType>` 语法，指定出默认类型。


这种技巧有用处情形的一个了不起示例，便是 *运算符重载，operator overloading*，咱们可以其在某些情形下，定制某个运算符（比如 `+`）的行为。


Rust 不允许咱们创建自己的运算符，或重载任意运算符。但咱们可以通过实现与运算符相关的特质，而重载那些运算及于 `std::ops` 中所列出的相应特质。比如，在下面清单 19-14 中，咱们就将 `+` 运算符过载为把两个 `Point` 实例加在一起。咱们是通过在 `Point` 结构体上实现 `Add` 特质完成这一点的。


文件名：`src/main.rs`

```rust
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq! (
        Point { x: 1, y: 0 } + Point { x: 2, y: 3},
        Point { x: 3, y: 3 }
    );
}
```

*清单 19-14：实现 `Add` 特质来为 `Point` 实例过载 `+` 运算符*

这里的 `add` 方法，将两个 `Point` 实例的 `x` 值及两个实例的 `y` 值相加，而创建出一个新的 `Point`。这里的 `Add` 特质有着一个确定自其中的 `add` 方法返回类型，名为的 `Output` 关联类型。

此代码中的默认泛型，是在 `Add` 特质里。以下便是其定义：


```rust
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
```

此代码看起来应相当熟悉：有着一个方法与关联类型的特质。其中新的部分为 `Rhs=Self`：这种语法叫做 *默认类型参数，default type parameters*。其中的 `Rhs` 泛型参数（是 `right hand side` 的缩写），定义了 `add` 方法中 `rhs` 参数的类型，当咱们实现这个 `Add` 特质，而没有指明 `Rhs` 的类型时，`Rhs` 的类型将默认为 `Self`，其将是咱们在其上实现 `Add` 的类型。

当咱们为 `Point` 实现 `Add` 时，由于咱们打算把两个 `Point` 实例相加，因此而使用了 `Rhs` 的默认值。接下来看看，其中咱们打算定制那个 `Rhs` 而非使用其默认值的一个 `Add` 实现示例。


咱们有着两个结构体，`Millimeters` 与 `Meters`，保存着不同单位的一些值。这种将某个既有类型，封装在另一结构体的瘦封装，thin wrapping，就叫做 *新类型模式，newtype pattern*，在后面的 [“使用新型模式在外部类型上实现外部特质”](#using-the-newtype-pattern-to-implement-external-traits-on-external-types) 小节，咱们会对其进行更深入讨论。咱们打算把毫米值与以米计数的值相加，并要让 `Add` 的实现，正确完成单位转换。咱们可在将 `Meters` 作为 `Rhs` 下，对 `Millimeters` 实现 `Add`，如下清单 19-15 中所示。


```rust
#[derive(Debug, Copy, Clone, PartialEq)]
struct Millimeters(u32);

#[derive(Debug, Copy, Clone, PartialEq)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
```

*清单 19-15：在 `Millimeters` 上实现 `Add` 特质，以将 `Millimeters` 与 `Meters` 相加*

为了将 `Millimeters` 与 `Meters` 相加，咱们指明了 `impl Add<Meters>` 来设置那个 `Rhs` 类型参数，而非使用其默认的 `Self`。

咱们将以如下两种主要方式，使用默认的类型参数：

- 在不破坏既有代码之下，扩展某个类型；
- 为实现绝大多数不会需要的特定情形下的定制，to allow customization in specific cases most users won't need。


标准库的 `Add` 特质，便是第二种目的的一个示例：通常，咱们将把两个相似类型相加，但 `Add` 特质提供了定制超出那种情况的能力。在 `Add` 特质中使用默认类型，就意味着咱们不必在多数时候指定额外的参数。换句话说，并不需要一点点的实现样板，从而令到使用这个特质更为容易。

第一个目的与第二个类似，不过是反过来的：在咱们打算将类型参数添加到某个既有特质时，就可以给到其一个默认值，从而在不破坏既有那些实现代码下，实现该特质功能的扩展。


### 用于消除歧义的完全合格语法：以同一名字调用方法

**Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name**

Rust 中没有什么可以阻止某个特质有着，与另一特质的方法同样名字的方法，Rust 也不会阻止咱们在一个类型上实现这两种特质。至于直接在类型上，以来自不同特质方法的同样名字实现方法，也是可行的。

在以同一名字调用这些方法时，咱们将需要告诉 Rust 打算使用哪一个。设想下面清单 19-16 中，定义了两个特质，`Pilot` 与 `Wizard`，两个特质都有一个叫做 `fly` 的代码。咱们随后在已在其上实现了一个名为 `fly` 方法的类型 `Human` 上，实现了这两个特质。每个 `fly` 都完成不同的事情。

```rust
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println! ("机长在此发言。");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println! ("飞起来！");
    }
}

impl Human {
    fn fly(&self) {
        println! ("*愤怒地挥动双臂*");
    }
}
```

*清单 19-16：两个被定义作有 `fly` 方法的特质并都在 `Human` 类型上被实现，且在 `Human` 上直接实现了一个 `fly` 方法*

当咱们在 `Human` 实例上调用 `fly` 时，编译器默认为调用直接在该类型上实现的那个方法，如下清单 19-17 中所示。

```rust
fn main() {
    let person = Human;
    person.fly();
}
```

*清单 19-17：调用 `Human` 实例上的 `fly`*

运行此代码将打印出 `*愤怒地挥动双臂*`，显示 Rust 调用了直接在 `Human` 上实现的那个 `fly` 方法。

为了调用 `Pilot` 或 `Wizard` 特质上的 `fly` 方法，咱们需要使用更为显式的语法，来指明我们所指的是那个 `fly` 方法。下面清单 19-18 对此语法进行了演示。

文件名：`src/main.rs`

```rust
fn main() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}
```

*清单 19-18：指明咱们打算调用哪个特质的 `fly` 方法*


在方法名字前指明特质名字，就向 Rust 澄清了咱们打算调用 `fly` 的哪个实现。咱们本来也可以写下 `Human::fly(&person)`，这与咱们曾在清单 19-18 中所使用的 `person.fly()` 等级，但若咱们无需消除歧义，这样写起来就些许有些长了。


运行此代码会打印以下输出：


```console
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/disambiguation`
机长在此发言。
飞起来！
*愤怒地挥动双臂*
```

由于 `fly` 方法取了一个 `self` 参数，那么当咱们有着实现了一个 *特质* 的两个 *类型* 时，Rust 就可以根据 `self` 的类型，找出要使用特质的哪个实现。

然而，不是方法的那些关联函数，是没有 `self` 参数的。当存在以同样函数名字，定义了非方法函数的类型或特质时，除非咱们使用了 *完全合格语法，fully qualified syntax*，否则 Rust 就不会总是清楚咱们所指的是何种类型。比如，在下面清单 19-19 中，咱们创建了一个用于动物收容所的特质，其中打算将所有狗崽都命名为 `点点`。咱们构造了带有关联的非方法函数 `baby_name` 的一个 `Animal` 特质。对结构体 `Dog` 实现了这个 `Animal` 特质，在 `Dog` 上咱们还直接提供了一个关联的非方法函数 `baby_name`。

```rust
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("点点")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("Puppy")
    }
}

fn main() {
    println! ("狗崽叫做 {}", Dog::baby_name());
}
```

*清单 19-19：有着一个关联函数的特质以及一个有着同样函数名字关联函数、还实现了那个特质的类型*

咱们是在那个定义在 `Dog` 上的关联函数里，实现的将全部狗仔命名为点点的代码。`Dog` 类型还实现了特质 `Animal`，该特质描述了全部动物都有的特征。小狗都叫做狗崽，且这一点是在 `Dog` 上的 `Animal` 特质中，与 `Animal` 特质关联的 `baby_name` 函数中得以表达的。

在 `main` 函数中，咱们调用了那个 `Dog::baby_name` 函数，这就会调用直接定义在 `Dog` 上的那个关联函数。此代码会打印下面的输出：

```console
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/disambiguation`
狗崽叫做 点点
```

此输出不是咱们想要的。咱们想要调用作为咱们曾在 `Dog` 上实现过的 `Animal` 特质一部分的那个 `baby_name` 函数，从而代码会打印出 `小狗叫做 狗崽`。咱们曾在清单 19-18 中用到的指定特质名字的技巧，这里就不管用了；而若咱们将 `main` 修改为下面清单 19-20 中的代码，咱们就将收到一个编译报错。

```rust
fn main() {
    println! ("小狗叫做 {}", Animal::baby_name());
}
```

*清单 19-20：尝试调用 `Animal` 特质中的那个 `baby_name` 函数，但 Rust 不清楚要使用那个实现*

由于 `Animal::baby_name` 没有 `self` 参数，且这里可能有别的实现了 `Animal` 特质的类型，因此 Rust 就无法计算出咱们想要的那个 `Animal::baby_name` 实现。咱们将得到下面这个编译器错误：


```console
$ cargo run
   Compiling disambiguation v0.1.0 (/home/lenny.peng/rust-lang/disambiguation)
error[E0790]: cannot call associated function on trait without specifying the corresponding `impl` type
  --> src/main.rs:20:26
   |
2  |     fn baby_name() -> String;
   |     ------------------------- `Animal::baby_name` defined here
...
20 |     println! ("小狗叫做 {}", Animal::baby_name());
   |                              ^^^^^^^^^^^^^^^^^ cannot call associated function of trait
   |
help: use the fully-qualified path to the only available implementation
   |
20 |     println! ("小狗叫做 {}", <Dog as Animal>::baby_name());
   |                              +++++++       +

For more information about this error, try `rustc --explain E0790`.
error: could not compile `disambiguation` due to previous error
```

为消除歧义并告知 Rust 咱们打算使用 `Dog` 的那个 `Animal` 实现，而非某种其他类型的 `Animal` 实现，咱们需要使用完全合格语法。下面清单 19-21 演示了怎样使用完全合格语法。

文件名：`src/main.rs`

```rust
fn main() {
    println! ("小狗叫做 {}", <Dog as Animal>::baby_name());
}
```

*清单 19-21：使用完全合格语法来指明，咱们是要调用实现在 `Dog` 上的 `Animal` 特质中的那个 `baby_name` 函数*

通过讲出咱们希望将 `Dog` 类型，针对这个 `baby_name` 函数调用而作为 `Animal` 对待，从而表明咱们打算调用实现在 `Dog` 上的 `Animal` 特质中的 `baby_name` 方法，这样位处那尖括号中的类型注解，提供给 Rust。此代码现在将打印出咱们想要的输出：


```console
$ cargo run
   Compiling disambiguation v0.1.0 (/home/lenny.peng/rust-lang/disambiguation)
    Finished dev [unoptimized + debuginfo] target(s) in 0.18s
     Running `target/debug/disambiguation`
小狗叫做 狗崽
```

一般来讲，完全合格语法是像下面这样定义的：


```rust
<Type as Trait>::function(receiver_if_method, next_arg, ...);
```

对于那些不是方法的语法，此处就不会有 `receiver`：这里将只有其他参数的清单。在调用函数或方法的所有地方，咱们都可以使用完全合格语法。不过，在 Rust 能够从程序中另外的信息计算出（要调用哪个函数或方法）时，那么这种语法便是可以省略的。咱们只需在有着多个使用了同一名字的实现，且 Rust 需要帮助来识别出咱们打算调用哪个实现时，才需要使用这种更为冗长的语法。


### 在一个特质里运用超特质寻求另一特质的功能

**Using Supertraits to Require One Trait's Functionality Within Another Trait**


有的时候，咱们可能会编写依赖于另一特质的特质：对于要实现前一个特质的类型，咱们希望寻求那个类型也实现后一个特质。为了咱们的特质定义，可以利用后一个特质的那些关联项目，咱们就会实现这一点。咱们的特质所依赖的那个特质，被称为咱们特质的 *超特质，supertrait*。

比方说，咱们打算构造一个带有将所给的值格式化，从而其被星号框起来的 `outline_print` 方法，这样一个 `OutlinePrint` 特质。而那个所给的值则是，一个实现了标准库特质 `Display` 来得到 `(x, y)` 的 `Point` 结构体，即当咱们在有着 `x` 为 `1` `y` 为 `3` 的 `Point` 上调用 `outline_print` 时，其将打印以下输出：


```console
**********
*        *
* (1, 3) *
*        *
**********
```

在 `outline_print` 方法的实现中，咱们打算使用 `Display` 特质的功能。因此，咱们就需要指明，这个 `OutlinePrint` 特质将只对那些同时实现了 `Display` 生效，且提供了 `OutlinePrint` 所需的功能。咱们可以通过指明 `OutlinePrint: Display`，在该特质定义中实现那一点。这种技巧类似于给特质添加特质边界。下面清单 19-22 给出了这个 `OutlinePrint` 特质的一种实现。


```rust
use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();

        println! ("{}", "*".repeat(len + 4));
        println! ("*{}*", " ".repeat(len + 2));
        println! ("* {} *", output);
        println! ("*{}*", " ".repeat(len + 2));
        println! ("{}", "*".repeat(len + 4));
    }
}
```

*清单 19-22：需要 `Display` 中功能的 `OutlinePrint` 特质实现*

由于咱们已指明 `OutlinePrint` 需要 `Display` 特质，因此咱们就可以使用那个任何实现了 `Display` 类型上均已实现了的 `to_string` 函数。若咱们在没有于特质名字之后加上冒号并指明 `Display` 特质，便尝试使用 `to_string`，咱们就会得到一个声称当前作用域中的类型 `&Self` 下，未找到名为 `to_string` 的方法的报错。

下面来看看当咱们尝试在某个未实现 `Display` 的类型，比如 `Point` 结构体上，实现 `OutlinePrint` 时会发生什么：


```rust
struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point {}
```

咱们会得到一个声称要求 `Display` 当其未实现的报错：

```console
$ cargo run
   Compiling supertrait v0.1.0 (/home/lenny.peng/rust-lang/supertrait)
error[E0277]: `Point` doesn't implement `std::fmt::Display`
  --> src/main.rs:21:23
   |
21 | impl OutlinePrint for Point {}
   |                       ^^^^^ `Point` cannot be formatted with the default formatter
   |
   = help: the trait `std::fmt::Display` is not implemented for `Point`
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
note: required by a bound in `OutlinePrint`
  --> src/main.rs:3:21
   |
3  | trait OutlinePrint: fmt::Display {
   |                     ^^^^^^^^^^^^ required by this bound in `OutlinePrint`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `supertrait` due to previous error
```

为修复这个问题，咱们就要在 `Point` 上实现 `Display` 并满足 `OutlinePrint` 所需的约束，如下面这样：

```rust
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write! (f, "({}, {})", self.x, self.y)
    }
}
```

随后在 `Point` 上实现 `OutlinePrint` 就将成功编译，而咱们就可以在 `Point` 实例上调用 `outline_print` 来将其实现在星号轮廓里了。


### 使用新型模式在外层类型上实现外层的特质

**Using the Newtype Pattern to Implement External Traits on External Types**

第 10 章中的 [“在类型上实现特质”](Ch10_Generic_Types_Traits_and_Lifetimes.md#implmenting-a-trait-on-a-type) 小节，咱们曾提到，指明只有当特质或类型二者之一，属于代码本地的时，咱们才被允许在类型上实现特质的孤儿规则，the orphan rule。而使用涉及到在元组结构体中创建出一个新类型的 *新型模式，newtype pattern*，那么绕过这种限制便是可行的了。（咱们曾在第 5 章的 [“使用不带命名字段的元组结构体来创建不同类型”](Ch05_Using_Structs_to_Structure_Related_Data.md#using-tuple-structs-without-named-fields-to-create-different-types") 小节，谈到过元组结构体）这种元组结构体讲有一个字段，且将是围绕咱们要实现某个特质的类型的一个瘦封装，a thin wrapper。随后这个封装类型，便是咱们代码箱的本地类型了，而咱们就可以在这个封装上实现那个特质了。所谓 *新型，newtype*，是源自 Haskell 编程语言的一个术语。使用这种模式没有运行时性能代码，同时那个封装类型在编译时会被略去。

作为一个示例，就说咱们打算在 `Vec<T>` 上实现 `Display`，而由于 `Display` 特质与 `Vec<T>` 类型，均被定义在咱们代码箱外部，因此孤儿规则会阻止咱们直接这样做。咱们可以构造一个保存着 `Vec<T>` 类型实例的 `Wrapper`；随后咱们就可以在 `Wrapper` 上实现 `Display`，并使用那个 `Vec<T>` 值，如下清单 19-23 中所示。


文件名：`src/main.rs`


```rust
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write! (f, "[{}]", self.0.join(", "))
    }
}
fn main() {
    let w = Wrapper(vec! [String::from("你好"), String::from("世界")]);
    println! ("w = {}", w);
}
```

*清单 19-23：创建一个围绕 `Vec<String>` 的 `Wrapper` 类型来实现 `Display`*

由于 `Wrapper` 是个元组结构体，且 `Vec<T>` 是该元组中位于索引 `0` 处的项目，因此其中 `Display` 的实现，便使用了 `self.0` 来方法那个内部的 `Vec<T>`。随后咱们就可以在 `Wrapper` 上使用 `Display` 的功能了。

使用这种技巧的缺点，则是那个 `Wrapper` 是个新的类型，因此其没有他所保存值的那些方法。咱们讲必须直接在 `Wrapper` 上，实现 `Vec<T>` 的全部方法，即委托给 `self.0` 的那些方法，这就会允许咱们将 `Wrapper` 完全当作 `Vec<T>` 那样对待了。而若咱们想要这个新的类型，有着那个内部类型所有的全部方法，那么在 `Wrapper` 上实现 `Deref` 特质（曾在第 15 章的 [“运用 `Deref` 特质将灵巧指针像常规引用那样对待”](Ch15_Smart_Pointers.md#treating-smart-pointers-like-regular-references-with-deref-trait) 小节讨论过），来返回那个内部类型，将是一种办法。而若咱们不打算 `Wrapper` 类型有着内部类型的所有方法 -- 比如，为限制 `Wrapper` 的行为 -- 咱们就必须手动实现仅咱们想要的那些方法了。


即使不牵涉到特质，这种新型模式也是有用的。接下来就要转换一下视角，而看看与 Rust 的类型系统交互的一些高级方式。


## 高级类型

**Advanced Types**

Rust 的类型系统有着一些到目前为止咱们曾提到过但尚未讨论过的特性。咱们将以一般意义上检视新型模式作为类型为何有用，而讨论新型模式开始。随后咱们将移步到类型别名，一项与新型模式类似，不过有着些许不同语义的特性。咱们还将讨论 `!` 类型与动态大小的类型。


### 为类型安全与抽象而运用新型模式

**Using the Newtype Pattern for Type Safety and Abstraction**

> **注意**：此小节假定你已读过早先的 [“使用新型模式来再外层类型上实现外层的特质”](#using-the-newtype-pattern-to-implement-external-traits-on-external-types") 小节。

对于那些超出到目前为止咱们曾讨论过的任务，包括静态强制要求值绝不会混淆，以及表明某个值的单位等等，新型模式同样是有用的。在清单 19-15 中，咱们就曾看到一个使用新型，表明单位的一个示例：回顾到 `Millimeters` 与 `Meters` 两个结构体，都曾将 `u32` 值封装在新型中。而若咱们编写了带有一个类型 `Millimeters` 参数的函数，那么咱们就无法编译某个偶然尝试以类型 `Meters` 或普通 `u32` 的值，调用那个函数的程序。


咱们还可以使用新型模式，来抽象出某个类型的一些实现细节：新的类型可暴露处不同意私有内部类型 API 的一个公开 API。

新类型还可以隐藏内部实现。比如，咱们可提供一个 `People` 类型，来封装一个存储着某人与其名字关联的 ID 的 `HashMap<i32, String>`。使用 `People` 的代码，只需与咱们提供的公开 API，比如某个将名字字符串添加到 `People` 集合的方法交互；那些代码将不需要知悉咱们在内部分配了`i32` 的 ID 给那些名字。新型模式是达成，咱们曾在第 17 章讨论过的 [“隐藏实现细节的封装”](Ch17_Object_Oriented_Programming_Features_of_Rust.md#encapsulation-that-hides-implementation-details") 的一种轻量方式。


### 使用类型别名创建类型同义词

**Creating Type Synonyms with Type Aliases**

Rust 提供给到既有类型另一个名字的声明 *类型别名，type alias* 的能力。为此，咱们要使用 `type` 关键字。比如，咱们可以像下面这样，创建到 `i32` 的别名 `Kilometers`：

```rust
type Kilometers = i32;
```

现在，别名 `Kilometers` 便是 `i32` 的同义词了；与在清单 19-15 中咱们曾创建的 `Millimeters` 与 `Meters` 两个类型不同，`Kilometers` 不是个单独的、新类型。有着类型 `Kilometers` 的那些值，将与类型 `i32` 的那些值做同样对待：


```rust
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    assert_eq! (x, y);
```

由于 `Kilometers` 与 `i32` 为同样类型，因此咱们可将这两种类型的值相加，且咱们可将 `Kilometers` 值传递给取 `i32` 参数的那些函数。但是，在使用这种方法时，咱们不会获得咱们早先所讨论的新型模式中的类型检查的那些益处。换句话说，当咱们在一些地方混淆了 `Kilometers` 与 `i32` 时，编译器将不会给到咱们一个报错。

类型同义词的一种主要用例，是为减少重复。比如，咱们可能有下面这样一个冗长的类型：


```rust
Box<dyn Fn() + Send + 'static>
```


在函数签名中，以及在全部代码中作为类型注解编写这种冗长类型，就会令人疲倦而容易出错。设想有个全部是下面清单 19-24 中代码的项目：

```rust
    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println! ("嗨"));

    fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
        // --跳过代码--
    }

    fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
        // --跳过代码--
    }
```

*清单 19-24：在多处使用长类型*


类型别名通过降低重复，而令到这样的代码更为可管理。在下面清单 19-25 中，咱们为那个冗长类型，引入了一个名为 `Thunk` 的别名，从而便可以使用这个更简短的别名 `Thunk`，替换全部的该种类型。

```rust
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println! ("嗨"));

    fn takes_long_type(f: Thunk) {
        // --跳过代码--
    }

    fn returns_long_type() -> Thunk {
        // --跳过代码--
    }
```

*清单 19-25：引入类型别名 `Thunk` 来减少重复*

这样的代码，阅读和编写起来要容易得多！给类型别名选择有意义的名字，也可以有助于表达咱们的意图（ *形实替换，thunk* 是个表示会在稍后被计算执行，因此对于会被存储的闭包，其是个恰当的名字）。

类型别名，还普遍用于 `Result<T, E>` 下的消除重复。设想标准库中的 `std::io` 模组。I/O 操作经常会返回一个 `Result<T, E>`，以处理操作失效时的情况。这个库有个表示了所有可能 I/O 错误的 `std::io::Error` 结构。`std::io` 中的许多函数，都会在那个 `E` 为 `std::io::Error` 下，返回 `Result<T, E>`，比如 `Write` 特质中的这些函数：

```rust
use std::fmt;
use std::io::Error;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn flush(&mut self) -> Result<(), Error>;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
}
```

其中的 `Result<..., Error>` 就被重复了很多。由此，`std::io` 便有了下面这样的类型别名声明：

```rust
type Result<T> = std::result::Result<T, std::io::Error>;
```

由于这种声明是在 `std::io` 模组中，因此咱们就可以使用完全合格的别名 `std::io::Result<T>`；那即是，带有 `E` 被填充为 `std::io::Error` 的 `Result<T, E>`。那个 `Write` 特质的函数签名，最终看起来就像下面这样了：

```rust
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<();
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}
```

类型别名以这两种方式发挥作用：其令到代码更易于编写 *并* 在整个 `std::io` 层面给到咱们一个一致的接口。由于其为一个别名，因此他仅是另一个 `Result<T, E>`，这意味着咱们可以与其一道使用那些全部工作于 `Result<T, E>` 上的方法，以及诸如 `?` 运算符那样的特殊语法。


### 永不返回的永不类型

**The Never Type that Never Returns**

Rust 有着一种因其没有值，而因此在类型理论术语中，叫做 *空类型，empty type* 的名为 `!` 的类型。因为在某个函数绝不会返回值时，这个类型立于返回值类型处，所以咱们称其为 *永不类型，never type*。下面是个示例：

```rust
fn bar() -> ! {
    // --跳过代码--
}
```

此代码读作 “函数 `bar` 返回永不。” 返回永不的函数被称为 *发散函数，diverging functions*。咱们无法创建出类型 `!` 的值，因此 `bar` 就永不会有可能返回值。


然而一种咱们永不能创建出值的类型，到底有什么用处呢？回顾到清单 2-5 中，作为那个猜数游戏一部分的代码；咱们已在在下面清单 19-26 中，重现了他的一点点：

```rust
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
```

*清单 19-26：有着一个以 `continue` 结束支臂的 `match` 表达式*

那个时候，咱们跳过了此代码的一些细节。而在第 6 章中的 [“`match` 控制流运算符”](Ch06_Enums_and_Pattern_Matching.md#the-match-control-flow-construct) 小节，咱们曾讨论了 `match` 支臂必须全部返回同一类型。那么，比如说，下面的代码就不会工作：


```rust
    let guess = match guess.trim().parse() {
        Ok(_) => 5,
        Err(_) => "你好",
    }
```

此代码中的 `guess` 类型，将必须为整数与字符串，而 Rust 要求 `guess` 只有一种类型。那么 `continue` 到底返回的是什么呢？到底是怎样咱们才在清单 19-26 中，曾被允许从一个支臂返回一个 `u32`，并有着以 `continue` 结束另一个支臂的呢？

描述这种行为的正式方式，即类型 `!` 的表达式，可被强制转换为任何别的类型。由于 `continue` 不会返回值，因此咱们就被允许以 `continue` 结束这个 `match` 支臂；相反，这个 `match` 支臂将控制移回到该循环的顶部，因此在 `Err` 情形下，咱们就绝不会赋给 `guess` 一个值。

在 `panic!` 宏下，这个永不类型也是有用的。回顾到咱们在 `Option<T>` 值上调用 `unwrap` 函数来生成一个值，或在此定义下中止运行：


```rust
impl<T> Option<T> {
    pub fn unwrap(self) -> {
        match self {
            Some(val) => val,
            None => panic! ("在 `None` value 上调用了 `Option::unwrap()`"),
        }
    }
}
```

此代码中，与清单 19-26 中那个 `match` 同样的事情发生了：Rust 会发现那个 `val` 有着类型 `T`，且 `panic!` 有着类型 `!`，因此整个 `match` 表达式的结果便是 `T`。此代码之所以有效，是由于 `panic!` 不会产生值；他会终止这个程序。在 `None` 情形下，咱们不会从 `unwrap` 返回值，所以此代码是有效的。

最后一个有着类型 `!` 的表达式，则是一个 `loop`：

```rust
    print! ("永永 ");

    loop {
        print! ("远远 ");
    }
```

这里，那个循环永不会结束，因此 `!` 便是该表达式的值。但是，若咱们包含了一个 `break`，由于这个循环会在其到达 `break` 时终止，因此这就不再成立了。


### 动态大小的类型与 `Sized` 特质

**Dynamically Sized Types and the `Sized` Trait**


Rust 需要知道其类型的确切情况，比如给某种特定类型值分配多少的内存空间。在一开始这就给其类型系统的一个角落留下了一点混乱：那便是 *动态大小类型，dynamically sized types* 这个概念。此概念有时被称为 DSTs 或 *未知大小类型，unsized types*，这些类型让咱们编写出，使用了仅在运行时才知道其大小值的代码来。

下面来深入到名为 `str`，贯穿这本书咱们一直都在使用一个的动态大小类型细节。那正是 `str`，而非 `&str`，确实是个 DST。在运行时之前，咱们是无法掌握字符串有多长，就是说咱们无法创建出一个类型 `str` 的变量，也无法取类型 `str` 的参数。设想下面的这段无法工作的代码：

```rust
    let s1: str = "致以问候！";
    let s2: str = "最近过得怎么样？";
```

Rust 需要清楚，要给特定类型的任何值分配多少内存，且某种类型的所有值，都必须使用同样数量的内存。若 Rust 运行咱们编写此代码，那么这两个 `str` 值就将需要占据同样数量的内存空间。但他们有着不同长度：`s1` 需要 15 字节的存储，而 `s2` 需要 `24` 字节。这就是为何创建保存动态大小类型值的变量不可行的原因。

那么咱们要怎么做呢？在这种情况下，咱们就已经知道答案了：咱们要令到 `s1` 与 `s2` 的类型为 `&str` 而非 `str`。从第 4 章的 [“字符串切片”](Ch04_Understanding_Ownership.md#string-slices) 小节，回顾到切片数据结构，只会存储其开始位置和切片的长度。因此尽管 `&T` 是存储了 `T` 所处内存地址的单个值，而一个 `&str` 则是 *两个* 值：`str` 的地址与其长度。如此，咱们就知道某个 `&str` 在编译时的大小了：其为 `uszie` 长度的两倍。那便是，咱们总是清楚 `&str` 的大小，而不管他所指向的字符串有多长。一般来说，这就是 Rust 中动态大小类型被运用的方式：他们有着存储了动态信息大小的额外的一点元数据。动态大小类型的黄金法则，就是咱们必须始终把那些动态大小类型的值，放置某种指针之后。

咱们可将 `str` 与所有类别的指针结合：比如，`Box<str>` 或 `Rc<str>`。事实上，之前咱们就已经见到过这样的，只不过是在一种不同的动态大小类型下：那便是特质。每个特质都是咱们可以通过使用特质名字而加以引用的动态大小类型。在第 17 章中的 [“使用允许不同类型值的特质对象”](Ch17_Object_Oriented_Programming_Features_of_Rust.md#using-trait-objects-that-allow-for-values-of-different-types) 小节，咱们曾提到为了将特质用作特质对象，咱们就必须将其放在指针之后，比如 `&dyn Trait` 或 `Box<dyn Trait>` （`Rc<dyn Trait>` 也应生效）。

为处理 DSTs 相关问题，Rust 提供了 `Sized` 特质，来判断在编译时某个类型的大小是否已知。在运行时大小已知的全部物件，都已自动实现了这个特质。此外，Rust 会隐式地将 `Sized` 上的边界，添加到每个泛型函数。那就是说，像下面的一个泛型函数：

```rust
fn generic<T>(t: T) {
    // --跳过代码--
}
```

实际上会被如咱们像下面写的这样被对待：


```rust
fn generic<T: Sized>(t: T) {
    // --跳过代码--
}
```

默认情况下，泛型函数只将在那些编译时有着已知大小的类型上工作。但是，咱们可以使用下面的特殊语法来解除这种限制：


```rust
fn generic<T: ?Sized>(t: &T) {
    // --跳过代码--
}
```

在 `?Sized` 上的特质边界，表示 “`T` 可能是也可能不是 `Sized` 的”，而这样的注解就会重写泛型在编译时务必要有已知大小的默认限制。有着这种意义的 `?Trait` 语法，只对 `Sized` 可用，对其他任何特质都是不可用的。

还要注意咱们已将那个参数 `t` 的类型，从 `T` 更换为了 `&T`。由于这个类型可能不是 `Sized`，因此咱们就需要在某种指针之后使用他。在这种情况下，咱们选择了一个引用。

接下来，咱们将谈谈函数与闭包！


## 高级函数与闭包

**Advanced Functions and Closures**


这个小节会探讨一些与函数和闭包有关的高级特性，包括函数指针与作为返回值的闭包，function pointers and returning closures。


### 函数指针

**Function Pointers**


咱们已讲到了怎样把闭包传递给函数；咱们也可以把常规函数传递给函数！在咱们打算传递一个咱们已定义的函数，而非定义出一个新闭包时，这种技巧便是有用的。这些函数会强制转换到类型 `fn` （有着小写的 `f`），而不会与那个 `Fn` 闭包特质混淆。这个 `fn` 类型，被称为 *函数指针，funciton pointer*。使用函数指针的传递函数，将实现把函数作为其他函数参数而运用。

指明某个函数是个函数指针的语法，与参数是个闭包的语法类似，如下清单 19-27 中所示，其中咱们定义了一个往其参数加一的函数 `add_one`。函数 `do_twice` 则会取两个参数：到任何的取一个 `i32` 参数，并返回 `i32` 值函数的函数指针，以及一个 `i32` 值。这个 `do_twice` 函数会调用函数 `f` 两次，传递给他那个 `arg` 值，随后把这两次函数调用的结果相加在一起。`main` 函数使用了参数 `add_one` 与 `5` 调用 `do_twice`。


文件名：`src/main.rs`

```rust
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}


fn main() {
    let answer = do_twice(add_one, 5);

    println! ("答案为：{}", answer);
}
```


*清单 19-27：使用 `fn` 类型来以参数方式接收函数指针*


此代码会打印出 `答案为：12`。咱们指明了 `do_twice` 中的参数 `f` 是取一个类型 `i32` 参数，并返回一个 `i32` 的 `fn`。最后咱们便可以在 `do_twice` 函数体中调用 `f` 了。在 `main` 中，咱们可以将名为 `add_one` 的函数，作为首个参数传递给 `do_twice`。


与闭包不同，`fn` 是种类型而非一个特质，因此咱们将 `fn` 直接指定为参数类型，而非使用 `Fn` 特质之一，作为特质边界声明一个泛型参数。


函数指针实现了全部三个闭包特质（`Fn`、`FnMut` 与 `FnOnce`），意味着咱们可以一直将某个函数，作为期望得到一个闭包的函数的参数而加以传递。编写出使用了一个泛型及闭包特质之一的函数，是最佳做法，如此咱们的函数就既可以接收函数，也可以接收闭包了。

那就是说，一种咱们只想接收 `fn` 而不想接收闭包的情况，便是与并无闭包的外部代码相交互时：C 语言函数可以参数方式接收函数，但 C 语言是没有闭包的。


而作为既可以使用内联定义的闭包，又可以使用命名函数的一种情况，下面就来看看标准库中 `Iterator` 特质所提供的 `map` 函数的一种用法。要使用 `map` 函数来将某个一些数字构成的矢量值，转换为字符串的矢量，咱们可以使用一个闭包，如下面这样：

```rust
    let list_of_numbers = vec! [1, 2, 3];
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(|i| i.to_string()).collect();
```

或者咱们可以命名一个作为给 `map` 参数的函数，而非那个闭包，如下面这样：

```rust
    let list_of_numbers = vec! [1, 2, 3];
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(ToString::to_string).collect();
```

请注意由于有着多个可用的名为 `to_string` 函数，因此咱们就必须使用早先在 [“高级特质”](#advanced-traites) 小节中讲到的完全合格语法。这里咱们使用了那个标准库已对任何实现了 `Display` 类型，实现过了的 `ToString` 特质中的 `to_string` 函数。

自第 6 章 [“枚举取值”](Ch06_Enums_and_Pattern_Matching.md#enum-values) 小节，回顾咱们所定义的各个枚举变种名字，也会成为一个初始化函数。咱们可以将这些初始化函数，作为实现了那些闭包特质的函数指针使用，这就意味着咱们可以把这些初始化函数，指定为取闭包的方法的参数，像下面这样：

```rust
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
```

这里咱们运用了那些经由使用 `Status::Value` 的初始化函数，于其上调用了 `map` 的那个范围中各个 `u32` 值，而创建出了一些 `Status::Value` 的实例。有的人会首选这种方式，而别的人则首选闭包。他们会编译到同样的代码，因此请使用你认为更清晰的风格。


### 返回闭包

**Returning Closures**

闭包是由特质表示的，这就意味着咱们不能直接返回闭包。在多数咱们可能打算返回特质的情形中，咱们都可以转而使用实现了该特质的具体类型，作为函数的返回值。但是，由于闭包没有可返回的具体类型，因此对于闭包是不能这样做的；就好比咱们是不被允许将函数指针作为返回值类型。


下面的代码尝试直接返回一个闭包，但其不会编译：


```rust
fn returns_closure() -> dyn Fn(i32) -> i32 {
    |x| x + 1
}
```

编译器报错如下：

```console
$ cargo build
   Compiling returning_closure v0.1.0 (/home/lenny.peng/rust-lang/returning_closure)
error[E0746]: return type cannot have an unboxed trait object
 --> src/main.rs:1:25
  |
1 | fn returns_closure() -> dyn Fn(i32) -> i32 {
  |                         ^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
  |
  = note: for information on `impl Trait`, see <https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits>
help: use `impl Fn(i32) -> i32` as the return type, as all return paths are of type `[closure@src/main.rs:2:5: 2:8]`, which implements `Fn(i32) -> i32`
  |
1 | fn returns_closure() -> impl Fn(i32) -> i32 {
  |                         ~~~~~~~~~~~~~~~~~~~

For more information about this error, try `rustc --explain E0746`.
error: could not compile `returning_closure` due to previous error
```

这个报错再度指向了那个 `Sized` 特质！Rust 不清楚他将需要多少内存空间来存储这个闭包。早先咱们就已见到了对这个问题的解决办法了。咱们可以使用一个特质对象：

```rust
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
```

这段代码可以很好地编译。有关特质对象的更多内容，请参考第 17 章中的 [“使用特质对象实现不同类型值”](Ch17_Object_Oriented_Programming_Features_of_Rust.md#using-trait-objects-that-allow-for-values-of-different-types) 小节。


接下来，咱们就要看看宏了！


## 关于宏

**Macros**

贯穿这本书，咱们业已用到像是 `println!` 这样的宏，但咱们并未完整地探讨过何为宏，以及其工作原理。 *宏，macro* 这个术语，指的是 Rust 中的一个特性家族：有着 `macro_rules!` 的 *声明式，declarative* 宏，与如下三种 *程序性，procedural* 宏：

- 指明一些在结构体及枚举上以 `derive` 属性添加代码的 **定制 `#[derive]` 的宏**，custome `#[derive]` macros that specify code added with the `derive` attribute used on structs and enums；
- 定义出一些可在任何项目上使用的一些定制属性的 **类属性宏**，attribute-like macros that define custom attributes usable on any item；
- 看起来像函数调用，但是在一些指定为其参数的令牌上操作的 **类函数宏**，function-like macros that look like function calls but operate on the tokens specified as their argument。

咱们将逐个讲到这每个的宏，但首先来看看，为何在已有函数的情况下，咱们还需要宏？


> **注**：宏似乎与 Java 及 Python 等语言中的装饰器类似？

### 宏与函数的区别

根本上讲，宏是一种编写其他代码的代码编写方式，这种方式被称作 *元编程，metaprogramming*。在附录 C 中，咱们会讨论那个 `derive` 属性，其会为咱们生成各种特质的实现。遍布这本书，咱们也已用到了 `println!` 与 `vec!` 两个宏。全部这些宏，都会 *展开，expand* 来产生相比于咱们手写代码更多的代码。

对于降低咱们所必须编写与维护代码量，元编程是有用的，这也是函数的角色之一。但是，宏有着函数所没有的一些额外能力。

函数签名必须要声明该函数所有的参数个数与类型。而另一方面的宏，则可以取数目不定的参数：咱们可以一个参数调用 `println! ("你好")`，或以两个参数调用 `println! ("你好 {}", name)`。同时，宏是在编译器对代码的意义加以解译之前展开的，因此宏就可以，比如在给到他的类型上实现某个特质。由于函数是在运行时被调用的，而特质需要在编译时被实现，故函数没办法做到这点。

实现宏而非函数的缺点，就是因为咱们是在编写那些编写出 Rust 代码的代码，所以宏定义要比函数定义更为复杂。由于这种间接性，相比于函数定义，宏定义一般都更难阅读、理解及维护。

宏与函数的另一重要区别，便是咱们必须于某个文件中调用宏 *之前*，定义好他们或将他们带入到作用域中，这一点与可在任何地方定义并在任何地方调用的函数相反。


### 用于通用元编程的带有 `macro_rules!` 的声明式宏


**Declarative Macros with `macro_rules!` for General Metaprogramming**

Rust 中使用最广泛的宏形式，就是 **声明式宏，declarative macro**。这些宏有时也被指为 “示例性宏，macros by example”，“`macro_rules!` 宏”，或仅被指为 “宏，macros”。声明式宏的核心，便是实现编写出类似于 Rust `match` 表达式的一些东西来。正如在第 6 章中曾讨论过的，`match` 表达式是取一个表达式、将该表达式计算结果值与一些模式比较，而在随后返回与匹配模式相关联代码的一些控制结构。宏也会把某个值与一些与特定代码相关的模式比较：在这种情形下，那个值便是传被递给宏的字面 Rust 源代码；一些模式就与那源代码比较；而与各个模式关联的代码，在匹配上时，就会替换传递给该宏的代码。这全部都是在编译器期间发生的。

要定义宏，就要用到 `macro_rules!` 结构体下面就通过看看 `vec!` 宏是如何定义的，来探讨一下怎样使用这个 `macro_rules!`。第 8 张曾涉及到咱们可以如何使用 `vec!` 宏，来创建出有着一些特定值的新矢量。比如，下面的红会创建出一个包含三个整数的新矢量值：

```rust
let v: Vec<u32> = vec! [1, 2, 3];
```

咱们也可以使用 `vec!` 宏，构造出两个整数的矢量值，或是五个字符串的矢量值。由于咱们预先不会知道值数目和类型，因此是无法使用函数完成这同样事情的。

下面清单 19-28 给出了稍微简化后的 `vec!` 宏的定义。

文件名：`src/lib.rs`

```rust
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
```

*清单 19-28：`vec!` 宏定义的简化版本*

> 注意：标准库中 `vec!` 宏的具体定义，包含了预先分配正确数量内存的代码。在这里咱们为了令到这个示例更为简单，而并未包含那些属于优化的代码。


其中的 `#[macro_export]` 注解，表明当这个宏被定义的代码箱，被带入到作用域的时候，这个宏就应成为可用。若没有这个注解，那么该宏就无法被带入到作用域。

随后咱们以 `macro_rules!` 及 *不带* 感叹号的咱们正定义宏的名字，开始该宏的定义。在此示例总，名字即为 `vec`，其后跟着表示宏定义代码体，the body of the macro definition, 的一对花括号。

`vec!` 宏代码体中的结构，与 `match` 表达式的结构类似。在这里咱们有着一个带有模式 `( $( $x:expr ),* )`，跟着 `=>` 及与这个模式关联代码块的支臂。在该模式匹配是，那个关联代码块将被运行，be emitted。鉴于这是这个宏中的唯一支臂，那么就只有一种要匹配有效方式；任何其他模式都将导致报错。那些更为复杂的宏，则将有着多于一个的支臂。

由于宏的那些模式，始于 Rust 代码结构而非一些值相匹配的，因此宏定义中有效的模式语法，不同于第 18 章中所涉及的模式语法。咱们来看看，清单 19-28 中各个模式片段，分别表示什么；对于宏的完整模式语法，请参见 [Rust 参考手册](https://doc.rust-lang.org/reference/macros-by-example.html)。

首选，咱们使用了一对圆括号，把整个模式包括起来。咱们使用一个美元符号（`$`），来声明出在宏系统中的，一个将要包含与这个模式匹配的 Rust 代码的变量，we use a dollar sign(`$`) to declare a variable in the macro system that will contain the Rust code matching the pattern。这个美元符号明确了这是个宏变量，而非一个常规 Rust 变量。接下来是捕获用于替换代码中的，与圆括号中模式匹配的那些值的一对圆括号，next comes a set of parentheses that captures values that match the pattern within the parentheses for use in the replacement code。在 `$()` 里的，为 `$x:expr`，这会与任意 Rust 表达式匹配，并把那个表达式命名为 `$x`。

`$()` 之后的逗号，表明在匹配 `$()` 中代码的代码之后，可选择性地出现一个字面的逗号分隔符。那个 `*` 指出了该模式会与零个或更多的 `*` 之前的东西匹配。

当咱们以 `vec! [1, 2, 3];` 调用这个宏时，`$x` 就会分别与表达式 `1`、`2` 与 `3` 匹配三次。

现在来看看与这个支臂关联的代码体中的模式：对于匹配了模式中 `$()` 的各个部分，根据该模式匹配的次数，`$()*` 里的 `temp_vec.push()` 会被零次或更多次生成。其中的 `$x` 会被各个匹配的表达式替换。当咱们以 `vec! [1, 2, 3];` 调用这个宏时，所生成的替换这个宏的代码，将是下面这样：

```rust
{
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
}
```

咱们就已定义了可取任意数目、任意类型参数，并能生成创建出包含这些特定元素矢量的一个宏了。


要了解更多有关如何编写宏的知识，请参考在线文档或其他资源，比如由 Daniel Keep 起头，Lukas Wirth 续写的 [“Rust 宏小册子”](https://veykril.github.io/tlborm/)。


### 用于从属性生成代码的程序性宏

**Procedural Macros for Generating Code from Attributes**

宏的第二种形式，便是 *程序性宏，procedural macro*，其行事更像函数（而是程序的一种类型，a type of procedure）。程序性宏接收一些代码作为输入，在那些代码上加以操作，并产生作为输出的一些代码，而如同非声明式宏所做的那样，与一些模式匹配并以别的代码替换那些代码。程序性宏的三种类别分别是定制派生宏，custom derive、类属性宏，attribute-like 及类函数宏，function-like，且这三种类别的程序性宏，都以类似方式运作。

在创建程序性宏时，那些定义务必要位处有着特别代码箱名字的他们自己的代码箱中。这是由于咱们（Rust 开发团队）希望在今后消除的一些复杂技术原因。在下面清单 19-29 中，咱们给出了如何定义一个程序性宏的方式，其中 `some_attribute` 是为使用某个特定宏变种的一个占位符。

文件名：`src/lib.rs`

```rust
use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
}
```

*清单 19-29： 定义某个程序性宏的示例*


这个定义了某个宏的函数，会取一个 `TokenStream` 值作为输入，并产生出一个 `TokenStream` 作为输出。`TokenStream` 类型是由 Rust 所包含的 `proc_macro` 代码箱定义，且表示的是一个令牌序列，a sequence of tokens。这个宏的核心如此：该宏在其上操作的源代码，构成了那个输入的 `TokenStream`，而该宏产生的代码，便是那个输出的 `TokenStream`。该函数还有一个附加给他的属性，指出咱们正在创建的是何种的程序性宏。在同一代码箱中，咱们可以有着多种类别的程序性宏。

下面就来看看各种不同类别的程序性宏。咱们将以一个定制的派生宏开始，并于随后探讨令到其他那些宏形式有所区别的一些小差异。


### 怎样编写出定制的 `derive` 宏

**How to Write a Custom `derive` Macro**


咱们就来创建一个名为 `hello_macro` 的宏，这个宏定义了一个名为 `HelloMacro`，有着名为 `hello_macro` 的关联函数的特质。与让咱们的用户为他们的各个类型实现这个 `HelloMacro` 特质不同，咱们将提供一个程序性宏，如此用户就可以 `[derive(HelloMacro)]` 注解他们的类型，从而得到那个 `hello_macro` 函数的默认实现。默认实现将打印出 `你好，宏！我的名字是 TypeName!`，其中的 `TypeName` 是这个特质被定义所在类型的名字。也就是说，咱们将编写一些实现其他编程者编写如下清单 19-30 中用到咱们代码箱的代码。

文件名：`src/main.rs`

```rust
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}
```

当我们完成编写时，此代码将打印 `你好，宏！我的名字叫 Pancakes！`。第一步是要构造一个新的库代码箱，像下面这样：

```console
$ cargo new hello_macro --lib --vcs none
```

接下来，咱们将定义那个 `HelloMacro` 特质及其关联函数：

文件名：`src/lib.rs`

```rust
pub trait HelloMacro {
    fn hello_macro();
}
```

咱们就有了一个特质及其函数。到这里，咱们代码箱的用户就可以实现这个特质来达成所需功能，像下面这样：

```rust
use hello_macro::HelloMacro;

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println! ("你好，宏！我的名字叫 Pancakes！");
    }
}

fn main() {
    Pancakes::hello_macro();
}
```

不过，用户们将需要为各种打算使用 `hello_macro` 特质的类型，编写那个实现的代码块；而咱们原本是要他们免于必须完成这项工作的。

此外，咱们尚不能提供，有着将打印特质被实现在其上类型名字的`hello_macro` 函数默认实现：Rust 没有反射能力，reflection capabilities，因此他无法在运行时查找处那个类型的名字。咱们需要一个宏，从而在编译时生成代码。

下一步就是要定义这个程序性宏。在编写这个小节的时候，程序性宏是需要在他们自己的代码箱中的。最终这个限制可能会被消除。代码箱的结构组织与宏代码箱方面的约定如下：对于名为 `foo` 的代码箱，那么定制派生程序性宏代码箱就会叫做 `foo_derive`。下面就在咱们的 `hello_macro` 项目内，开启一个名为 `hello_macro_derive` 的新代码箱：

```console
$ cargo new hello_macro_derive --lib --vcs none
```

咱们的这两个代码箱是密切相关的，因此咱们是在咱们的 `hello_macro` 代码箱目录下，创建的这个程序性宏代码箱。而若咱们修改了 `hello_macro` 中的特质定义，咱们就将不得不也要修改 `hello_macro_derive` 中那个程序性宏。两个代码箱将需要单独发布，且使用这两个代码箱的程序员，将需要将二者都添加为依赖，并同时把他们都带入到作用域。相反，咱们可以让 `hello_macro` 代码箱，将 `hello_macro_derive` 作为依赖使用，并重导出这些程序性宏的代码。然而，咱们阻止结构该项目的这种方式，会让那些不想要 `derive` 功能的程序员，也可以使用 `hello_macro`。

咱们需要将 `hello_macro_derive` 代码箱，声明为程序性宏的代码箱。如同马上就会看到的那样，咱们还需要来自 `syn` 与 `quote` 代码箱的功能，，因此咱们就需要将他们添加为依赖。请将下面的配置，添加到 `hello_macro_derive` 的 `Cargo.toml` 文件：

```toml
[lib]
proc-macro = true

[dependencies]
syn = "1.0"
quote = "1.0"
```

要开始定义这个程序性宏，就要将下面清单 19-31 中的代码，放置于 `hello_macro_derive` 代码箱的 `src/lib.rs` 文件中。请注意在咱们添加了 `impl_hello_macro` 函数定义前，此代码不会编译。


文件名：`hello_macro_derive/src/lib.rs`

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // 以语法树形式，构建出咱们可操作 Rust 代码的表示
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // 构造出这个特质实现
    impl_hello_macro(&ast)
}
```

*清单 19-31：多数程序性宏为处理 Rust 代码而都需要的代码*

请注意咱们已经代码分解到 `hello_macro_derive` 函数中，由其负责解析那个 `TokenStream`，而其中的 `impl_hello_macro` 函数，则负责转换那个语法树：这样做令到编写程序性宏更为方便。对于几乎每个咱们所见到的或创建的程序性宏，外层函数（此示例中的 `hello_macro_derive`）中的代码将是一致的。而咱们在那个内层函数（此示例中的 `impl_hello_macro`）中指定的代码，将依据咱们程序性宏目的而有所不同。

咱们引入了三个新的代码箱：`proc_macro`、[`syn`](https://crates.io/crates/syn) 与 [`quote`](https://crates.io/crates/quote)。`proc_macro` 代码箱是 Rust 自带的，因此咱们无需将其添加到 `Cargo.toml` 的依赖。`proc_macro` 代码箱，是实现从咱们的代码读取及操作 Rust 代码的编译器 API。

`syn` 代码箱会从一个字符串将 Rust 代码解析为咱们可在其上执行操作的一种数据结构。而 `quote` 代码箱，则会将 `syn` 数据结构，转换回 Rust 代码。这些代码箱令到解析任何一种咱们打算处理的 Rust 代码更为容易：编写出 Rust 代码的完整解析器，并非易事。

这个 `hello_macro_derive` 函数，将在咱们的库用户，于某个类型上指明 `#[derive(HelloMacro)]` 时被调用。这样做之所以可行，是由于咱们已使用 `proc_macro_derive` 注解了这里的 `hello_macro_derive` 函数，并指定了于咱们的特质名字相符的名字 `HelloMacro`；而这正是多数程序性宏所遵循的约定。

这个 `hello_macro_derive` 函数首选会将那个 `input`，从一个 `TokenStream` 转换为咱们随后可以解读并于其上操作的一种数据结构。这正是 `syn` 发挥作用之处。`syn` 中的 `parse` 函数，会取一个 `TokenStream` 并返回一个表示解析出 Rust 代码的 `DeriveInput` 数据结构。下面清单 19-32 给出了咱们对 `struct Pancakes;` 字符串进行解析而得到的 `DeriveInput` 数据结构的有关部分：

```rust
DeriveInput {
    // --跳过代码--

    ident: Ident {
        ident: "Pancakes",
        span: #0 bytes(95..103)
    },
    data: Struct(
        DataStruct {
            struct_token: Struct,
            fields: Unit,
            semi_token: Some(
                Semi
            )
        }
    )
}
```

*清单 19-32：在对清单 19-30 中有着该宏属性的代码进行解析时咱们所得到的 `DeriveInput` 实例*


这个结构体的那些字段显示，咱们所解析的 Rust 是个有着 `Pancakes` 的 `ident`（标识符，意为名字）的一个单元结构体，a unit struct。此结构体上还有一些用于描述 Rust 各个方面的其他字段；请参阅 [有关 `DeriveInput` 的 `syn` 文档](https://docs.rs/syn/1.0/syn/struct.DeriveInput.html) 了解更多信息。

很快咱们就将实现那个 `impl_hello_macro` 函数，其中咱们将构建出咱们所打算包含的新 Rust 代码。但在咱们实现之前，请注意咱们的派生宏输出，同样是个 `TokenStream`。这个返回的 `TokenStream` 会添加到咱们代码箱用户编写的代码，因此当他们编译他们的代码箱时，他们将获得咱们在这个修改的 `TokenStream` 中所提供的额外功能。

咱们或许已经留意到，咱们调用了 `unwrap`，来在这里的到 `syn::parse` 函数调用失败时，造成那个 `hello_macro_derive` 函数终止运行。由于 `proc_macro_derive` 函数必须返回 `TokenStream`，而非 `Result` 来顺应程序性宏的 API，因此咱们的程序性宏就要在出错时终止运行。咱们已通过使用 `unwrap` 简化了这个示例；在生产代码中，咱们应通过运用 `panic!` 或 `expect`，提供有关那些东西出错的更具体的错误消息。

既然咱们有了将经注解的 Rust 代码，从一个 `TokenStream` 转换为一个 `DeriveInput` 实例的代码，那么就要生成在被注解类型上实现这个 `HelloMacro` 特质的代码，如下清单 19-33 中所示。

文件名：`hello_macro_derive/src/lib.rs`

```rust
fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println! ("你好，宏！我的名字叫 {}！", stringify! (#name));
            }
        }
    };
    gen.into()
}
```

*清单 19-33：是要解析出的 Rust 代码，实现这个 `HelloMacro` 特质*

通过使用 `ast.ident`，咱们得到了一个包含着受注解类型名字（标识符）的 `Ident` 结构体实例。清单 19-32 中的代码结构，显示当咱们在清单 19-30 中的代码上运行这个 `impl_hello_macro` 函数时，咱们得到的这个 `ident` 就将有着值为有一个 `"Pancakes"` 值的 `ident` 字段。因此，清单 19-33 中的 `name` 变量，就将包含一个在被打印出时，将为字符串 `"Pancakes"`，即清单 19-30 中那个结构体名字的 `Ident` 结构体。

其中的 `quote!` 宏，允许咱们定义出咱们打算返回的 Rust 代码。编译器会期望得到不同于这个 `quote!` 宏直接执行结果的东西，因此咱们就要将其转换为一个 `TokenStream`。咱们是通过调用的那个消费这个中间表示，并返回所需的 `TokenStream` 类型的一个值的 `into` 方法，完成这一点的。

`quote!` 宏还提供了一些非常酷的模板机制：咱们可以敲入 `#name`，而 `quote!` 就将使用变量 `name` 中的值，替换掉他。咱们甚至可以与宏工作类似方式，完成一些重复操作。请参考 [`quote` 代码箱文档](https://docs.rs/quote) 了解完整信息。

咱们是要这个程序性宏，在用户注解的类型上，生成咱们的 `HelloMacro` 特质实现，而咱们可通过使用 `#name` 做到这点。这个特质实现，有着一个名为 `hello_macro` 的函数，其函数体包含了咱们打算提供的功能：打印 `你好，宏！我的名字叫` 以及随后的那个受注解类型的名字。

这里用到的那个 `stringify!` 宏，是内建于 Rust 中的。他会取一个 Rust 表达式，比如 `1 + 2`，并在编译时将这个表达式转换为字符串字面值，比如 `"1 + 2"`。这与 `format!` 或 `println!` 这样的会执行表达式并随后将结果转换为一个 `String` 的宏不同。由于存在着那个 `#name` 输入，为一个要打印出字面值的表达式的可能，因此咱们便使用了 `stringify!`。使用 `stringify!` 还通过在编译时将 `#name` 转换为字符串字面值，而节省了一次内存分配。


到这里，在 `hello_macro` 与 `hello_macro_derive` 中，`cargo build` 都应完全成功。让我们来将这两个代码箱，连接到清单 19-30 中的代码，来看看行动中的程序性宏！在咱们的 `projects` 目录下，使用 `cargo new derive_macro_comsumer --vcs none` 创建一个新的二进制项目。咱们需要在这个 `derive_macro_comsumer` 代码箱的 `Cargo.toml` 中，把 `hello_macro` 及 `hello_macro_derive` 添加为依赖项。若咱们把咱们版本的 `hello_macro` 与 `hello_macro_derive` 发布在了 [crates.io](https://crates.io/)，那么他们将为一些常规依赖；而在没有发布时，咱们可以像下面这样，将他们指定为 `path` 的依赖：

```toml
hello_macro = { path = "../hello_macro" }
hello_macro_derive = { path = "./hello_macro/hello_macro_derive" }
```

请将清单 19-30 中的代码，放入到 `src/main.rs` 中，并运行 `cargo run`：其应打印出 `你好，宏！我的名字叫 Pancakes！` 在这个 `derive_macro_comsumer` 代码箱无需实现那个程序性宏中的 `HelloMacro` 特质下，该特质的实现就已被包含了；正是 `#[derive(HelloMacro)]` 添加了这个特质实现。

接下来，咱们要探讨其他类别的程序性宏，与定制派生宏有怎样的不同。


### 类属性宏

**Attribute-like macros**

类属性宏与定制派生宏类似，不过与生成 `derive` 属性的代码不同，他们允许咱们创建出新的属性。他们还更灵活：`derive` 只对结构体和枚举生效；而属性则同时可应用到其他项目，比如函数等。下面就是一个使用类属性宏的示例：比方说咱们在运用某个 web 应用框架时，就有一个对函数加以注解的名为 `route` 的属性：

```rust
#[route(GET, "/")]
fn index() {
```

这个 `#[route]` 就将是由那个框架，定义的一个程序性宏。那个宏定义函数的签名，将看起来像下面这样：

```rust
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenSteam {
```

这里，咱们有两个类型 `TokenStream` 的参数。头一个是属性的内容：即 `GET, "/"` 部分。而第二个，则是该属性被附加到的那个项目的函数体：在这个示例中，便是 `fn index() {}` 及该函数的函数体其余部分。

除此之外，类属性宏与定制派生宏以同样方式运作：咱们要创建出一个有着 `proc-macro` 代码箱类型的代码箱，并实现一个生成咱们想要代码的函数！


### 类函数宏

**Function-link macros**


类函数宏定义了看起来像函数调用的宏。与 `macro_rules!` 宏类似，他们比函数更为灵活；比如，他们就可取未知数目的参数。然而，`macro_rules!` 宏只能使用咱们早先在 [用于通用元编程的带有 `macro_rules!` 的声明式宏](#declarative-macros-with-macro_rules-for-general-metaprogramming") 小节，曾讨论过的 match-like 语法。而类函数宏，则会取一个 `TokenStream` 参数，而这些宏的定义，就会使用 Rust 代码，如同另外两种程序性宏所做的那样，对那个 `TokenStream` 加以操纵。作为类函数宏的一个例子，便是将如下面调用的一个 `sql!` 宏：

```rust
let sql = sql! (SELECT * FROM posts WHERE id=1);
```

这个宏会解析其内部的 SQL 语句，并就其语法方面的正确性加以检查，相比 `macro_rules!` 宏所能完成的处理，这就要复杂多了。这个 `sql!` 宏将像下面这样定义：

```rust
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
```

此定义与定制派生宏的签名类似：咱们会接收圆括号内部的那些令牌，并返回咱们所要生成的代码。


## 本章小节

咦！现在咱们在工具箱中，便有了大概率不会经常用到的一些 Rust 特性，不过咱们会明白，在一些极为特别的情况下他们会是可用的。咱们业已引入几个复杂的主题，因此在咱们于一些错误消息建议，或其他人的代码中遇到他们时，咱们就能识别出这些概念和语法。请将这一章，当作引导咱们得到解决办法的一个参考。


接下来，咱们将把这正本书中曾讨论过的所有内容，投入到实践中，而完成另一个项目！
