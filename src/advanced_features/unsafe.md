# 不安全的 Rust

**Unsafe Rust**

到目前为止，本书所讨论的全部代码，都曾在编译时，将 Rust 的内存安全保证进行了强制执行。然而，Rust 内部有着另一种不强制进行这些内存安全保证的语言：他被叫做 *不安全的 Rust，unsafe rust*，而其与常规 Rust 工作类似，只不过赋予了咱们额外的超能力。

不安全 Rust 之所以存在，是因为静态分析，static analysis 天生是保守的。在编译器尝试判断出代码是否维持了那些保证时，相比接受一些无效程序，则退回一些有效程序会更佳。尽管代码 *可能* 没有问题，在 Rust 编译器没有足够信息对代码有信心时，他就会退回该代码。在这些情况下，咱们就可以使用不安全代码特性，来告诉编译器，“请相信我，我明白我在做什么。”但请当心，使用不安全 Rust 要风险自担：若不当使用非安全代码，那么由内存不安全而导致的问题就会发生，比如空指针的解引用。

Rust 有着一个非安全的另外自我，an unsafe alter ego，的另一原因，便是所采行的计算机硬件本质上是不安全的。若 Rust 不允许咱们执行非安全操作，那么咱们就无法完成一些特定任务。Rust 需要允许咱们完成一些底层系统变成，诸如直接与操作系统交互，或甚至编写咱们自己的操作系统。而进行底层编程工作，是这门语言的目标之一。下面就来探讨，咱们可以使用非安全 Rust 做些什么，以及怎样使用非安全 Rust。


## 不安全的超能力

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


## 解引用原始指针

**Dereferencing a Raw Pointer**


在第 4 章的 [悬空引用](Ch04_Understanding_Ownership.md#悬空引用dangling-references) 小节，咱们曾提到编译器会确保引用始终有效。不安全的 Rust 则有着与引用类似的， 叫做 *原始指针，raw pointers* 的两种新类型。与引用一样，原始指针可以是不可变或可变的，并被相应地写作 `*const T` 及 `*mut T`。其中的星号 `*` 并非是解引用运算符；他是这种类型名字的一部分。在原始指针语境下，*不可变，immutable* 意指该指针在被解引用之后，不能被直接赋值。

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

在全部的这些危险之下，咱们为何还要使用原始指针呢？一个主要的原因就是在与 C 代码交互时，正如将在下一小节，[”调用非安全函数或方法“](#调用不安全函数或方法)，中将看到的。另一中情况，便是在构建借用检查器不清楚的一些安全抽象时。咱们将介绍非安全函数，并在随后看看一个用到不安全代码的安全抽象。


## 调用不安全函数或方法

**Calling an Unsafe Function or Method**


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


### 创建非安全代码的安全抽象

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

回顾第 4 章中的 [“切片类型”](Ch04_Understanding_Ownership.md#切片类型the-slice-type) 小节，切片即为到一些数据的指针，与切片的长度。咱们使用了 `len` 方法，来获取切片的长度，并使用 `as_mut_ptr` 方法来访问切片的原始指针。在这个示例中，由于咱们有着一个到一些 `i32` 值的可变切片，`as_mut_prr` 就会返回类型 `*mut i32` 的原始指针，其已被咱们存储在变量 `ptr` 中。

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


### 使用 `extern` 的函数调用外部代码

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


## 访问或修改可变静态变量

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

静态变量与咱们曾在第三章中 [“变量与常量区别”](Ch03_Common_Programming_Concepts.md#常量) 小节讨论过的常量类似。静态变量的名字，依约定都是 `SCREAMING_SNAKE_CASE` 形式。静态变量只能存储有着 `'static` 声明周期的引用，这意味着 Rust 编译器可以计算出声明周期，而不要求咱们显式地对其加以注解。访问不可变的静态变量是安全的。

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


## 实现不安全的特质

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

作为示例，请回顾第 16 章中 [“`Sync` 与 `Send` 特质下的可扩展并发”](Ch16_Fearless_Concurrency.md#sync-与-send-两个特质下的可扩展并发) 小节中，曾讨论过的 `Sync` 与 `Send` 两个标记性特质：在咱们的类型完全是由 `Send` 与 `Sync` 两种类型构成时，编译器就会自动实现这些特质。而在咱们实现某个包含了非 `Send` 或 `Sync` 的类型，比如原始指针，同时咱们打算将那个类型标记为 `Send` 或 `Sync` 时，咱们就必须使用 `unsafe`。Rust 无法验证咱们的类型坚守了其可被跨线程安全发送，或自多个线程安全访问的那些保证；因此，咱们就需要手动完成这些检查，并以 `unsafe` 照这样加以表明。


## 访问联合体的字段

**Accessing fields of a union**


使用 `unsafe` 的就只剩下最后的用法了，那便是访问 *联合体，union* 的字段。`union` 与 `struct` 类似，但一次只会用到特定实例中一个声明的字段。联合体主要用于与 C 语言代码中的联合体交互。由于 Rust 无法保证在联合体示例当前所存储的数据类型，因此访问联合体字段是不安全的。在 [Rust 参考手册](https://doc.rust-lang.org/reference/items/unions.html) 中，可了解更多有关联合体的知识。


## 何时使用不安全代码

**When to use unsafe code**


运用 `unsafe` 来采取上述五种做法（超能力）没有什么过错，或者不受欢迎。但由于编译器无法助力于保持内存安全，因此要让 `unsafe` 代码正确就更为棘手一些。在有使用 `unsafe` 代码的某种理由时，就可以这样做，而在问题出现时，显式的 `unsafe` 注解，就会令到排查问题原因更为容易。


（End）


