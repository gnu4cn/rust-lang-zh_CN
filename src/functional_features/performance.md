# 性能比较：循环与迭代器

**Comparing Performance: Loops vs. Iterators**


为确定是使用循环还是迭代器，咱们需要知道哪种实现更快：带有显式 `for` 循环的 `search` 函数版本，还带有迭代器的版本。

我们通过将阿瑟-柯南-道尔爵士的《福尔摩斯历险记》的全部内容加载到一个字符串中，并在内容中寻找单词 "福尔摩斯"，进行了一次基准测试。下面是使用 `for` 循环的 `search` 版本和使用迭代器的版本的基准测试结果：

```console
test bench_search_for  ... bench:  19,620,300 ns/iter (+/- 915,700)
test bench_search_iter ... bench:  19,234,900 ns/iter (+/- 657,200)
```

迭代器的版本稍微快了一些！由我们不会在这里解释基准代码，因为重点不是要证明这两个版本是等价的，而是要了解这两种实现在性能上的总体比较。

为了获得更全面的基准，你应该用各种大小的文本作为 `contents`，用不同的词和不同长度的词作为 `query`，以及其他各种变化来检查。重点是：迭代器虽然是个高级的抽象概念，但被编译成的代码与咱们自己编写的低级代码大致相同。迭代器是 Rust 的 *零成本抽象，zero-cost abstractions* 之一，我们的意思是使用该抽象不会带来额外的运行时开销。这类似于 C++ 最初的设计者和实现者 Bjarne Stroustrup 在《C++ 基础》（2012）中对零开销的定义：

> 一般来说，C++ 的实现遵循零开销原则：咱们不使用的东西，咱们不需要付出开销。再进一步：咱们使用的东西，咱们不可能手写出更良好的代码，the zero-overhead principle: What you don't use, you don't pay for. And further: What you do use, you couldn't hand code any better。

作为另一个示例，以下代码取自某个音频解码器。解码算法使用线性预测的数学运算，根据之前样本的线性函数来估计后面的数值。此代码使用迭代器链，an iterator chain，对作用域中三个变量执行一些数学计算：由一些数据构成 `buffer` 切片，由 12 个 `coeffecients` 构成的一个数组，以及保存着数据偏移量的 `qlp_shift`。咱们已在这个示例中声明了变量，但并未给他们任何值；尽管此代码在其上下文之外没有什么意义，但他仍不失为说明 Rust 如何将高级别的概念，转化为低级别代码的一个简练、真实的示例。

```rust
let buffer: &mut [i32];
let coefficients: [i64; 12];
let qlp_shift: i16;

for i in 12..buffer.len() {
    let prediction = coefficients.iter()
                                 .zip(&buffer[i - 12..i])
                                 .map(|&c, &s| c * s as i64)
                                 .sum::<i64>() >> qlp_shift;
    let delta = buffer[i];
    buffer[i] = prediction as i32 + delta;
}
```

为了计算预测值，此代码遍历 `coefficients` 中 12 个值中的每一个，并使用 `zip` 方法将系数值与 `buffer` 中的前 12 个值配对。随后对每个数值对，咱们将数值相乘，对所有结果求和，并将总和中的二进制位，向右偏移 `qlp_shift` 位。

像是音频解码器这样的应用中的计算，通常最优先考虑的是性能。在这里，我们正在创建一个迭代器，使用两个适配器，然后消费这个值。这段 Rust 代码会编译成什么汇编代码呢？好吧，在写这篇文章时，他可以编译成与咱们用手写的相同汇编代码。在系数的迭代过程中完全没有对应的循环： Rust 知道有 12 个迭代，所以他 “展开” 了这个循环。所谓 “展开，unrolling”，是消除循环控制代码方面的开销，而代之以生成循环历次迭代的重复代码的一种优化，*unrolling* is an optimization that removes the overhead of the loop controlling code and instead generates repetitive code for each iteration of the loop。

所有的系数都被存储在寄存器中，这意味着访问这些值的速度非常快。在运行时，对数组的访问没有边界检查。Rust 能够应用的所有这些优化使得所产生的代码非常高效。现在咱们知道了这些，咱们就可以毫无顾忌地使用迭代器和闭包了！他们使代码看起来更高级，但不会因为这样做而带来运行时的性能损失。



# 本章小结

闭包与迭代器，是 Rust 受函数式编程概念启发的两项特性。他们有助于 Rust 以底层性能清楚表达高级别概念的能力。闭包与迭代器的实现，不会影响到运行时性能。这正是 Rust 致力于提供零成本抽象目标的一部分。

现在我们已经改进了 I/O 项目的表达能力，让我们再来看看 `cargo` 的更多特性，这些特性将帮助我们与世界分享这个项目。
