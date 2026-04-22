# 不安全的 Rust

到目前为止，我们讨论的所有代码都在编译时强制执行了 Rust 的内存安全保证。然而，Rust 内部隐藏着第二种语言，他不会强制执行这些内存安全保证：他被称为 *不安全 Rust*，其工作方式与 Rust 类似，但给予了我们额外的超能力。

不安全 Rust 之所以存在，是因为静态分析，static analysis 本质上是保守的。在编译器尝试确定代码是否支持保证时，相比接受一些无效的程序，拒绝一些有效的程序更好。尽管代码 *可能* 没有问题，当当 Rust 编译器缺乏足够的信息对来确信其正确性，他将拒绝该代码。在这些情况下，咱们可以使用不安全代码特性告诉编译器，“相信我，我知道我在做什么。” 但请注意，使用不安全 Rust 需要自担风险：若不当使用非安全代码，可能会因内存不安全而引发问题，比如空指针的解引用。

Rust 具有不安全的另一个自我，an unsafe alter ego，的另一原因是，底层的计算机硬件本质上是不安全的。若 Rust 不允许咱们执行不安全的操作，咱们就无法完成某些任务。Rust 需要允许咱们执行底层的系统编程，例如直接与操作系统交互，甚至编写自己的操作系统。进行底层编程工作正是这门语言的目标之一。我们来探讨以下，我们可以使用不安全的 Rust 做什么，以及怎样做。


## 发挥不安全的超能力

要切换到不安全的 Rust，就要使用 `unsafe` 关键字，然后开启一个包含不安全代码的新代码块。咱们可以在不安全的 Rust 中，执行五项在安全 Rust 下无法执行的操作，我们称之为 *不安全的超能力*。这些超能力包括以下能力：

1. 解引用原始指针;
2. 调用不安全的函数或方法；
3. 访问或修改可变的静态变量；
4. 实现不安全的特质；
5. 访问 `union` 类型的字段。

重要的是要明白， `unsafe` 不会关闭借用检查器，或禁用 Rust 的其他任何安全检查：当咱们在不安全代码中使用引用时，他仍将受检查。`unsafe` 关键字仅给予咱们对这五项特性的访问，他们随后不受编译器的内存安全检查。咱们在不安全代码块内部仍将获得一定程度的安全。

此外，`unsafe` 并不意味着代码块内的代码就必然危险，或是肯定存在内存安全问题：其初衷是，作为程序员的咱们将确保 `unsafe` 代码块内的代码，将以有效的方式访问内存。

人容易犯错，错误在所难免，但通过要求这五种不安全操作位于 `unsafe` 注解的代码块内，咱们就将知道，与内存安全相关的任何错误，都必须位于 `unsafe` 代码块内。请保持 `unsafe` 代码块较小；日后排查内存错误时，咱们会为此感到庆幸。

为了尽可能隔离不安全代码，最好将此类代码封装在安全抽象中，并提供安全的 API，我们将在本章后面的内容中探讨不安全的函数及方法时，进一步讨论这种做法。标准库的部分内容，便是作为在审计后的不安全代码之上的安全抽象实现的。将不安全代码封装在安全抽象中，可以防止 `unsafe` 的使用泄漏到咱们或咱们的用户可能打算使用以以 `unsafe` 代码实现的功能的所有地方，因为使用安全抽象是安全的。

我们来依次探讨这五种不安全的超能力。我们还将研究一些为不安全代码的提供安全接口的抽象。


## 解引用原始指针

在第 4 章的 [悬空引用](../ownership/references_and_borrowing.md#悬空引用) 小节中，我们提到编译器会确保引用始终有效。不安全 Rust 有两种名为 *原始指针* 的新类型，他们与引用类似。与引用一样，原始指针可以是不可变的或可变的，并分别写为 `*const T` 及 `*mut T`。其中星号 `*` 并非是解引用运算符；而是类型名字的一部分。在原始指针的语境下，*不可变* 意味着该指针在解引用后不能直接赋值。

不同于引用和及灵巧指针，原始指针有着以下特征：

- 允许通过同时存在指向同一内存位置的不可变和可变指针，或多个可变指针，来忽略借用规则；
- 不保证指向有效的内存；
- 允许为空；
- 不实现任何自动清理。

通过选择不让 Rust 强制执行这些保证，咱们可以放弃有保证的安全性，以换取更高的性能，或者获得与另一语言或与硬件交互的能力，其中 Rust 的保证均不适用。

下面清单 20-1 展示了怎样创建不可变和可变的原始指针。

<a name="listing_20-1"></a>
```rust
    let mut num = 5;

    let r1 = &raw const num;
    let r2 = &raw mut num;
```

**清单 20-1**：通过原始借用运算符创建原始指针

请注意，我们在这段代码中未包含 `unsafe` 关键字。我们可以在安全代码中创建原始指针；我们只是不能在不安全代码块外部解引用原始指针，稍后咱们就将看到。

我们通过使用原始借用运算符，创建了原始指针：`&raw const num` 创建了一个 `*const i32` 的不可变原始指针，而 `&raw mut num` 创建了一个 `*mut i32` 的可变原始指针。由于我们直接从局部变量创建了他们，因此我们知道这些特定的原始指针是有效的，但我们不能对任何原始指针都做出这样的假设。

为了演示这点，接下来我们将创建一个我们无法确定其有效性的原始指针，使用 `as` 关键字强制转换值，而不是使用原始借用运算符。下面清单 20-2 展示了怎样创建一个指向内存中任意位置的原始指针。尝试使用任意内存属于未定义行为：该地址处可能存在数据，也可能没有；编译器可能会优化代码，以便不存在内存访问，或者程序可能会因段错误而终止。通常，没有编写此类代码的充分理由，尤其是在咱们可以使用原始借用运算符替代的情况下，但这种做法确实有可能的。

<a name="listing_20-2"></a>
```rust
    let address = 0x012345usize;
    let r = address as *const i32;
```

**清单 20-2**：创建指向任意内存地址的原始指针

回顾一下，我们可以在安全代码中创建原始指针，但我们不能解引用原始指针并读取所指向的数据。在下面清单 20-3 中，我们对一个需要使用不安全代码块的原始指针，使用了解引用运算符 `*`。

<a name="listing_20-3"></a>
```rust
    let mut num = 5;

    let r1 = &raw const num;
    let r2 = &raw mut num;

    unsafe {
        println! ("r1 为：{}", *r1);
        println! ("r2 为：{}", *r2);
    }
```

**清单 20-3**：在 `unsafe` 代码块内解引用原始指针

创建指针本身并无害处；只有当我们试图访问他指向的值时，我们才最终会遇到无效值。

另请注意，在清单 20-1 与 20-3 中，我们创建了 `*const i32` 与 `*mut i32` 两个原始指针，都指向存储 `num` 的同一内存位置。反之若我们尝试创建到 `num` 的不可变和可变的引用，则代码将不会编译，因为 Rust 的所有权规不允许在存在不可变引用的同时存在可变引用。在原始指针下，我们可以创建到同一位置的可变指针与不可变指针，并通过可变指针修改数据，这潜在地会造成数据竞争。请务必当心！

面对所有这些危险，为什么咱们还要使用原始指针呢？一个主要用例是在与 C 代码交互时，咱们将在下一小节中看到这点。另一个用例是在构建借用检查器无法理解的安全抽象时。我们将先介绍不安全函数，然后通过一个使用不安全代码的安全抽象示例来说明。


## 调用不安全的函数或方法

咱们可以在不安全代码块中执行的第二种操作是调用不安全函数。不安全函数和方法看起来和常规函数和方法完全相同，但他们的定义开头有个额外的 `unsafe`。这种上下文中的 `unsafe` 关键字表明，该函数具有我们在调用该函数时需要遵守的要求，因为 Rust 无法保证我们满足这些要求。通过在 `unsafe` 代码块中调用不安全函数，我们表明我们已经阅读了该函数的文档，并承担遵守函数合约的责任。

以下是个名为 `dangerous` 的不安全函数，其函数体中没有执行任何操作：

```rust
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }
```

我们必须在单独的 `unsafe` 代码块内调用 `dangerous` 函数。若我们尝试在没有 `unsafe` 代码块的情况下调用 `dangerous`，我们将收到报错：

```console
$ cargo run
   Compiling unsafe_functions v0.1.0 (/home/hector/rust-lang-zh_CN/projects/unsafe_functions)
error[E0133]: call to unsafe function `dangerous` is unsafe and requires unsafe function or block
 --> src/main.rs:4:5
  |
4 |     dangerous();
  |     ^^^^^^^^^^^ call to unsafe function
  |
  = note: consult the function's documentation for information on how to avoid undefined behavior

For more information about this error, try `rustc --explain E0133`.
error: could not compile `unsafe_functions` (bin "unsafe_functions") due to 1 previous error
```

在 `unsafe` 代码块下，我们向 Rust 断言我们已经阅读了该函数的文档，了解如何正确使用他，并且已验证我们正在履行该函数的合约。

要在不安全函数的函数体内执行不安全的操作，咱们仍然需要使用 `unsafe` 代码块，就像在常规函数内一样，当咱们忘记时，编译器将警告咱们。这有助于咱们使不安全代码块尽可能小，因为整个函数体内坑能并不都需要不安全操作。


### 创建对不安全代码的安全抽象

仅仅因为函数包含不安全代码，并不意味着我们需要标记整个函数为 `unsafe`。事实上，封装不安全代码在安全函数中，属于一种常见的抽象。作为示例，我们来研究一下标准库中的 `split_at_mut` 函数，他需要一些不安全的代码。我们将探讨怎样实现他。这个安全方法定义在可变切片上：他取一个切片，并通过在作为参数给出的索引处分割这个切片，而将使其成为两个切片。下面清单 20-4 展示了如何使用 `split_at_mut`。

<a name="listing_20-4"></a>
```rust
    let mut v = vec! [1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq! (a, &mut [1, 2, 3]);
    assert_eq! (b, &mut [4, 5, 6]);
```

**清单 20-4**：使用安全的 `split_at_mut` 函数

我们无法仅使用安全的 Rust 实现这个函数。一种尝试可能类似于下面清单 20-5，但该代码将不编译。为简化起见，我们把 `split_at_mut` 实现为函数而非方法，并且只针对 `i32` 值的切片，而非泛型类型 `T`。

<a name="listing_20-5"></a>
```rust
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();

    assert! (mid <= len);

    (&mut values[..mid], &mut values[mid..])
}
```

**清单 20-5**：仅使用安全 Rust 对 `split_at_mut` 的一种实现尝试

这个函数首先获取切片的总长度。然后，通过检查作为参数给出的索引是否小于或等于该长度，断言该索引位于切片范围内。这一断言意味着，当我们传递的索引大于要分割切片的长度时，该函数在尝试使用该索引前会终止运行。

然后，我们以元组形式返回两个可变切片：一个从原始切片的开头到 `mid` 索引处，另一个从 `mid` 处到切片的末尾。

当我们尝试编译清单 20-5 中的代码时，将得到一个报错：

```rust
$ cargo run
   Compiling unsafe_example v0.1.0 (/home/hector/rust-lang-zh_CN/projects/unsafe_example)
error[E0499]: cannot borrow `*values` as mutable more than once at a time
 --> src/main.rs:6:31
  |
1 | fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
  |                         - let's call the lifetime of this reference `'1`
...
6 |     (&mut values[..mid], &mut values[mid..])
  |     --------------------------^^^^^^--------
  |     |     |                   |
  |     |     |                   second mutable borrow occurs here
  |     |     first mutable borrow occurs here
  |     returning this value requires that `*values` is borrowed for `'1`
  |
  = help: use `.split_at_mut(position)` to obtain two mutable non-overlapping sub-slices

For more information about this error, try `rustc --explain E0499`.
error: could not compile `unsafe_example` (bin "unsafe_example") due to 1 previous error
```

Rust 的借用检查器无法理解，我们正在借用切片的不同部分；他只知道我们从同一个切片借用了两次。借用切片的不同部分基本上是可行的，因为这两个切片并不重叠，但 Rust 还不够聪明，无法知道这点。当我们知道代码没有问题，而 Rust 不知道时，就该使用不安全代码了。

下面清单 20-6 展示了如何通过使用 `unsafe` 代码块、原始指针以及对不安全函数的一些调用，使 `split_at_mut` 的实现正常工作。

<a name="listing_20-6"></a>
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

**清单 20-6**： 在 `split_at_mut` 函数的实现中使用不安全代码

回顾第 4 章中 [切片类型](../ownership/the_slice_type.md) 小节，切片属于指向某些数据的指针和切片的长度。我们使用 `len` 方法来获取切片的长度，并使用 `as_mut_ptr` 方法访问切片的原始指针。在这一情形下，由于我们有个`i32` 值的可变切片，`as_mut_prr` 会返回一个类型 `*mut i32` 的原始指针，我们存储在变量 `ptr` 中。

我们保留 `mid` 索引位于切片范围内的断言。然后，我们来到不安全的代码：`slice::from_raw_parts_mut` 函数取一个原始指针和一个长度，并创建一个切片。我们使用这个函数创建一个从 `ptr` 开始、长度为 `mid` 个项目的切片。然后，我们以 `mid` 作为参数对 `ptr` 调用 `add` 方法，以获取一个从 `mid` 处开始的原始指针，并使用该指针和 `mid` 之后的剩余项目数作为长度创建一个切片。

函数 `slice::from_raw_parts_mut` 是不安全的，因为他取一个原始指针，并且必须信任该指针是有效的。原始指针上的 `add` 方法也是不安全的，因为他必须信任偏移量位置也是有效的指针。因此，我们必须在调用 `slice::from_raw_parts_mut` 及 `add` 放在 `unsafe` 代码块中，才可以调用他们。通过查看代码并添加 `mid` 必须小于或等于 `len` 的断言，我们可以确定，在 `unsafe` 代码块内使用的所有原始指针都将是指向切片中数据的有效指针。这是对 `unsafe` 合理且恰当的使用。

相比之下，下面清单 20-7 中对 `slice::from_raw_parts_mut` 的使用，可能会在使用切片时崩溃。这段代码会取一个任意的内存位置，并创建一个长度 10,000 个项目的切片。

<a name="listing_20-7"></a>
```rust
    use std::slice;

    let address = 0x01234usize;
    let r = address as *mut i32;

    let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
```

**清单 20-7**：从任意内存位置创建切片


> *注*：上面的代码运行结果：
>
> ```console
> $ cargo run
>    Compiling unsafe_example v0.1.0 (/home/hector/rust-lang-zh_CN/projects/unsafe_example)
>     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
>      Running `target/debug/unsafe_example`
> ```
>
> 可见并无报错，但若加上 `println! ("{values:?}");` 语句，运行结果将如下：
>
>
> ```console
> $ cargo run
>    Compiling unsafe_example v0.1.0 (/home/hector/rust-lang-zh_CN/projects/unsafe_example)
>     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
>      Running `target/debug/unsafe_example`
> zsh: segmentation fault (core dumped)  RUSTFLAGS="-A warnings" cargo run
> ```
>
> 报出了段错误。

我们并不拥有这一任意位置处的内存，并且无法保证这段代码创建的切片包含有效的 `i32` 值。尝试将 `values` 当作有效的切片使用，将导致未定义行为。


### 使用 `extern` 函数调用外部代码

有时，咱们的 Rust 代码可能需要与以另一种语言编写的代码交互。为此，Rust 提供了 `extern` 关键字，他有助于创建和使用 *外部函数接口，Foreign Function Interface, FFI*，这是用于编程语言定义函数，并使另一种（外部）语言能够调用这些函数的方式。

下面清单 20-8 展示了怎样建立与 C 语言标准库中的 `abs` 函数的集成。在 `extern` 代码块中声明的函数，在 Rust 代码中调用时通常是不安全的，因此 `extern` 代码块也必须标记为 `unsafe`。原因是其他语言不会强制执行 Rust 的规则与保证，并且 Rust 无法检查他们，因此确保安全性的责任落在程序员身上。

<a name="listing_20-8"></a>
文件名：`src/main.rs`

```rust
unsafe extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println! ("根据 C 语言，-3 的绝对值为：{}", abs(-3));
    }
}
```

**清单 20-8**：声明并调用在另一种语言中定义的 `extern` 函数


> 上面代码运行结果为：
>
> ```console
> $ cargo run
>    Compiling extern_code v0.1.0 (/home/hector/rust-lang-zh_CN/projects/extern_code)
>     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
>      Running `target/debug/extern_code`
> 根据 C 语言，-3 的绝对值为：3
> ```

在 `unsafe extern "C"` 代码块中，我们列出打算调用的另一种语言中的函数的名字与签名。其中 `"C"` 部分定义了外部函数使用的 *应用程序二进制接口，application binary interface, ABI*：ABI 定义在汇编层面，at the assembly level，如何调用该函数。`"C"` 的 ABI 最为常用的，遵循 C 编程语言的 ABI。有关 Rust 支持的所有 ABI 的信息，请参阅 [Rust 参考手册](https://doc.rust-lang.org/stable/reference/items/external-blocks.html#abi)。

在 `unsafe extern` 代码块内声明的每个项目都是隐式不安全的。但是，某些 FFI 函数 *是* 可以安全调用的。例如，C 标准库中的 `abs` 函数就没有任何内存安全顾虑，并且我们知道他可以任何 `i32` 值调用。在这种情况下，我们可以使用 safe 关键字来表示该特定函数可以安全调用，即使他位于 `unsafe extern` 代码块中。一旦进行这一修改，调用他就不再需要 `unsafe` 代码块，如下清单 20-9 中所示。

<a name="listing_20-9"></a>
文件名：`src/main.rs`

```rust
unsafe extern "C" {
    safe fn abs(input: i32) -> i32;
}

fn main() {
    println! ("根据 C 语言，-3 的绝对值为：{}", abs(-3));
}
```

**清单 20-9**：在 `unsafe extern` 代码块中显式地标记函数为 `safe`，并安全地调用他

标记外部函数为安全，并不意味着他本质上就是安全的！相反，这更像是咱们向 Rust 做出的承诺，即该函数是安全的。确保信守承诺仍然是咱们的责任！


### 从其他语言调用 Rust 函数

我们还可以使用 `extern` 创建接口，允许其他语言调用 Rust 的函数。无需创建整个 extern 代码块，我们只需在相关函数的 `fn` 关键字之前添加 `extern` 关键字，并指定要使用的 ABI 即可。我们还需要添加 `#[unsafe(no_mangle)]` 注解，以告知 Rust 编译器不要破坏该函数的名字。所谓 *名字重整*，是指编译器将我们给函数起的名字，更改为不同名字，该名字包含更多供编译过程的其他部分使用的信息，但可读性较低。每种编程语言的编译器对名字的重构方式略有不同，因此为了其他语言能够识别 Rust 函数，我们必须禁用 Rust 编译器的名字重构特性。这属于不安全的，因为在没有内置的名字重构下，不同库之间可能存在名字冲突，因此我们有责任确保我们选择的名字，在没有名字重构的情况下安全地导出。

在以下示例中，我们构造 `call_from_c` 函数为编译成共享库并从 C 中链接后，可从 C 代码访问：

```rust
#[unsafe(no_mangle)]
pub extern "C" fn call_from_c() {
    println!("刚刚从 C 调用了 Rust 函数！");
}
```

`extern` 的这种语法，仅需在属性中要求 `unsafe`，而无需在 `extern` 代码块上声明。


## 访问或修改可变静态变量

在本书中，我们还没有讨论全局变量，Rust 虽然支持全局变量，但可能会给 Rust 的所有权带来问题。当两个线程同时访问同一个可变全局变量时，可能会引发数据竞争。

在 Rust 中，全局变量被称为 *静态* 变量。下面清单 20-10 展示了以字符串切片作为值的静态变量的声明和使用。

<a name="listing_20-10"></a>
文件名：`src/main.rs`

```rust
static HELLO_WORLD: &str = "你好，世界！";

fn main() {
    println! ("名字为：{}", HELLO_WORLD);
}
```

**清单 20-10**：定义和使用不可变静态变量

静态变量与我们在第 3 章中 [声明常量](../programming_concepts/variables_and_mutability.md#声明常量) 小节中讨论过的常量类似。按照惯例，静态变量的名字采用 `SCREAMING_SNAKE_CASE` 个是。静态变量只能存储具有 `'static` 声明周期的引用，这意味着 Rust 编译器可以计算出声明周期，我们不需要显式注解他。访问不可变的静态变量是安全的。

常量与不可变静态变量之间的一个细微差别是，静态变量中的值在内存中有着固定的地址。使用该值将始终将访问同一数据。另一方面，常量则允许在每次使用时复制其数据。另一个区别是，静态变量可以是可变的。访问和修改可变静态变量是 *不安全的*。下面清单 20-11 展示了如何声明、访问和修改名为 `COUNT` 的可变静态变量。

<a name="listing_20-11"></a>
文件名：`src/main.rs`

```rust
static mut COUNTER: u32 = 0;

/// 安全提示：同时从多个线程调用此函数属于未定义行为，
/// 因此咱们 *必须* 确保每次仅从单个线程调用他。

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    unsafe {
        // 安全提示：此函数仅在 `main` 中的单个线程中调用。
        add_to_count(3);
        println! ("COUNTER: {}", *(&raw const COUNTER));
    }
}
```

**清单 20-11**：对可变静态变量的读取和写入时不安全的

与常规变量一样，我们使用 `mut` 关键字指定可变性。任何读写 `COUNTER` 的代码都必须位于 `unsafe` 代码块内。清单 20-11 中的代码会编译并如预期打印 `COUNT: 3` ，因为他是单线程的。让多个线程访问 `COUNTER` 可能会导致数据竞争，因此这属于为定义行为。因此，我们需要标记整个函数为 `unsafe` 并记录安全限制，以便任何调用该函数的人，都知道哪些是可以安全执行，哪些不可以安全执行。

每当我们编写不安全的函数时，惯例是在代码中添加以 `SAFETY` 开头的注释，并解释调用者需要采取哪些措施才能安全地调用该函数。同样，每当我们执行不安全的操作时，惯例也是添加以 `SAFETY` 开头的注释，说明安全规则是如何得到维护的。

此外，编译器会默认通过编译器 lint 规则阻止任何试图创建可变静态变量引用操作。您必须通过添加 #[allow(static_mut_refs)] 注解来显式禁用该 lint 规则的保护，或者通过使用原始借用运算符之一创建的原始指针来访问该可变静态变量。 这包括隐式创建引用的情况，例如本代码片段中在 `println!` 中的用法。要求通过原始指针创建对静态可变变量的引用，有助于使使用它们的安全要求更加明确。

对于全局可访问的可变数据，很难确保不存在数据竞争，这就是 Rust 为何将可变静态变量视为不安全的原因。在可能的情况下，最好使用我们在第 16 章中讨论的并发技术和线程安全的智能指针，以便编译器能够检查来自不同线程的数据访问是否安全。


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


