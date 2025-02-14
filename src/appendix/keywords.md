# 附录 A：关键字

以下清单包含了 Rust 语言当前或今后要用到的一些关键字。由此，他们便不能被用作标识符（除在 [“原始标识符”](#原始标识符) 小节中咱们将讨论的那些外）了。所谓标识符，是函数、变量、参数、结构体字段、模组、代码箱、常量、宏、静态值、属性、类型、特质或生命周期等的名字。


## 当前在用的关键字

**Keywords Currently in Use**


下面是当前在用关键字的清单，带有其作用描述。

- `as` - 执行原生强制转换，primitive casting，消除包含着某个项目的特定特质歧义，disambiguate the specific trait containing a item，或重命名 `use` 语句中的项目；
- `async` - 返回一个 `Future` 类型值，而非阻塞当前线程；
- `await` - 在某个 `Future` 值的结果准备好前，暂停程序执行;
- `break` - 立即退出某个循环；
- `const` - 定义出常量项目或常量原始指针；
- `continue` - 继续下一循环迭代；
- `crate` - 在模组路径中，指向代码箱根;
- `dyn` - 动态调遣到某个特质对象，参考 [特质对象执行动态调遣](Ch17_Object_Oriented_Programming_Features_of_Rust.md#特质对象执行动态调遣);
- `else` - `if` 的回退，及 `if let` 控制流的构件；
- `extern` - 链接外部函数或变量；
- `false` - 布尔值假的字面值；
- `fn` - 定义出某个函数或函数指针类型；
- `for` - 对某个迭代器的项目加以迭代、实现某个特质，或指明某个更高级别的生命周期，a higher-ranked lifetime;
- `if` - 基于某个条件表达式结果的分支；
- `impl` - 实现固有或特质功能，implement inherent or trait functionality;
- `in` - `for` 循环语法的一部分；
- `let` - 绑定某个变量；
- `loop` - 无条件地循环；
- `match` - 将某个值与模式匹配；
- `mod` - 定义出模组；
- `move` - 领导闭包取得其所有捕获值的所有权；
- `mut` - 注解出引用、原始指针或模式绑定等中的可变性；
- `pub` - 注解出结构体、`impl` 代码块或模组等中的公开可见性；
- `ref` - 按引用绑定；
- `return` - 自函数返回值；
- `Self` - 咱们正定义或实现中类型的类型别名；
- `self` - 方法主体，method subject，或当前模组；
- `static` - 在整个程序执行过程持续有效的全局变量或生命周期；
- `struct` - 定义出某个结构体；
- `super` - 当前模组的父模组；
- `trait` - 定义出某个特质；
- `true` - 布尔值真的字面值；
- `type` - 定义出某个类型别名或关联类型；
- `union` - 定义出某个 [联合体](https://doc.rust-lang.org/reference/items/unions.html)，是在联合体声明时用到的唯一关键字;
- `unsafe` - 注解非安全代码、函数、特质或一些实现；
- `use` - 将符号带入到作用域;
- `where` - 注解约束某个类型的子句；
- `while` - 基于某个表达式结果而有条件的循环。


## 为今后使用保留的关键字

**Keywords Reserved for Future Use**


以下关键字尚无任何功能，但被 Rust 为今后的潜在使用而保留。

- `abstract`
- `become`
- `box`
- `do`
- `final`
- `macro`
- `override`
- `priv`
- `try`
- `typeof`
- `unsized`
- `virtual`
- `yield`


## 原始标识符

**Raw Identifiers**


*原始标识符，raw identifiers* 属于允许实现使用一般不被允许关键字的语法。是通过在关键字前加上前缀 `r#`，使用原始标识符的。

比如，`match` 是个关键字。在咱们尝试编译下面这个使用 `match` 作其名字的函数时：

文件名：`src/main.rs`

```rust
fn match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}
```

咱们将得到这样的报错：

```console
error: expected identifier, found keyword `match`
 --> src/main.rs:1:4
  |
1 | fn match(needle: &str, haystack: &str) -> bool {
  |    ^^^^^ expected identifier, found keyword
```

该报错显示咱们无法将关键字 `match` 用作函数标识符。要将 `match` 用作函数名字，咱们就需要使用原始标识符语法，像下面这样：

文件名：`src/main.rs`

```rust
fn r#match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}

fn main() {
    assert! (r#match("foo", "foobar"));
}
```

此代码将不带任何错误地编译。请注意那个函数的定义中，与 `main` 中该函数被调用处其名字上的 `r#` 前缀。

原始标识符实现了将任何咱们所选的词语用作标识符，即使那个词语碰巧是个保留的关键字。这给到咱们更自由地选择标识符名字，以及实现与一些以其中这些词语不属于关键字的语言，所编写的程序集成。此外，原始标识符实现了，对那些以不同于咱们代码箱 Rust 版本编写库加以运用。比如，在 2015 版中 `try` 就不是个关键字，但在 2018 版本中却是。若咱们依赖于一个使用 2015 版本编写的库，而该库有一个 `try` 函数，那么咱们就将需要在这种情况下，使用原始标识符 `r#try`，来从咱们的 2018 版本的代码，调用那个函数。请参阅 [附录 E](#appendix-e) 了解更多有关版本的信息。


（End）


