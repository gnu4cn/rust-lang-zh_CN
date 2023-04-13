# 附录

以下小节包含了在咱们的 Rust 路途中，会发现有用的一些参考资料。


## 附录 A：关键字

以下清单包含了 Rust 语言当前或今后要用到的一些关键字。由此，他们便不能被用作标识符（除在 [“原始标识符”](#原始标识符) 小节中咱们将讨论的那些外）了。所谓标识符，是函数、变量、参数、结构体字段、模组、代码箱、常量、宏、静态值、属性、类型、特质或生命周期等的名字。


### 当前在用的关键字

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

### 为今后使用保留的关键字

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

### 原始标识符

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


## 附录 B：运算符与符号

此附录包含了 Rust 语法的词汇表，包括运算符及别的一些，自己单独出现或出现于路径、泛型、特质边界、宏、属性、注释、元组及方括符等上下文中的符号。

### 运算符

表 B-1 包含了 Rust 中的符号、该符号将如何出现于上下文中的一个示例、简单的解释，以及该运算符是否可过载。若某个运算符可以过载，就会列出过载那个运算符要用到的相关特质。

**<small>表 B-1：运算符</small>**

| 运算符 | 示例 | 说明 | 是否可以过载 |
| :--- | :--- | :--- | :--- |
| `!` | `ident! (...)` <br /> `ident! {...}` <br /> `ident! [...]` | 宏扩展 |   |
| `!` | `!expr` | 按位或逻辑求补运算 | 否 |
| `!=` | `expr != expr` | 不等比较 | `PartialEq` |
| `%` | `expr % expr` | 算术求余运算 | `Rem` |
| `%=` | `var %= expr` | 算术求余并赋值 | `RemAssign` |
| `&` | `&expr`, `&mut expr` | 借用 |  |
| `&` | `&type`, `&mut type`, `&'a type`, `&'a mut type` | 借用指针类型 |  |
| `&` | `expr & expr` | 按位与（AND）运算 | `BitAnd` |
| `&=` | `var &= expr` | 按位与（AND）运算并赋值 | `BitAndAssign` |
| `&&` | `expr && expr` | 短路逻辑与（AND）运算，short-circuit logical AND |  |
| `*` | `expr * expr` | 算术乘法运算 | `Mul` |
| `*=` | `var *= expr` | 算术乘法运算并赋值 | `MulAssign` |
| `*` | `*expr` | 解引用运算 | `Deref` |
| `*` | `*const type`, `*mut type` | 原始指针运算 |  |
| `+` | `trait + trait`, `'a + trait` | 复合类型约束运算 |  |
| `+` | `expr + expr` | 算术加法运算 | `Add` |
| `+=` | `var += expr` | 算术加法运算并赋值 | `AddAssign` |
| `,` | `expr, expr` | 参数与元素分隔符 |  |
| `-` | `- expr` | 算术取反运算 | `Neg` |
| `-` | `expr - expr` | 算术减法运算 | `Sub` |
| `-=` | `var -= expr` | 算术减法运算并赋值 | `SubAssign` |
| `->` | `fn(...) -> type`, <code>&vert;...&vert; -> type</code> | 函数与闭包的返回值类型 |  |
| `.` | `expr.ident` | 成员访问 |  |
| `..` | `..`, `expr..`, `..expr`, `expr..expr` | 排除右侧的范围语法字面值 | `PartialOrd` |
| `..=` | `..=expr`, `expr..=expr` | 包含右侧范围语法字面值 | `PartialOrd` |
| `..` | `..expr` | 结构体更新语法 |  |
| `..` | `variant(x, ..)`, `struct_type { x, .. }` | “等等” 模式绑定，"And the rest" pattern binding |  |
| `...` | `expr...expr` | （已弃用，请使用 `..=` 代替）在模式中：包含式范围模式 |  |
| `/` | `expr / expr` | 算术除法运算 | `Div` |
| `/=` | `var /= expr` | 算术除法并赋值 | `DivAssign` |
| `:` | `pat: type`, `ident: type` | 约束 |  |
| `:` | `ident: expr` | 结构体字段初始化 |  |
| `:` | `'a: loop {...}` | 循环标签 |  |
| `;` | `expr;` | 语句及项目的终止符 |  |
| `;` | `[..., len]` | 固定大小数组语法的一部分 |  |
| `<<` | `expr << expr` | 向左移位运算 | `Shl` |
| `<<=` | `var <<= expr` | 向左移位运算并赋值 | `ShlAssign` |
| `<` | `expr < expr` | 小于比较 | `PartialOrd` |
| `<=` | `expr <= expr` | 小于等于比较 | `PartialOrd` |
| `=` | `var = expr`, `ident = type` | 赋值/等价，equivalence |  |
| `==` | `expr == expr` | 相等比较 | `PartialEq` |
| `=>` | `pat => expr` | 匹配支臂语法的一部分 |  |
| `>` | `expr > expr` | 大于比较 | `PartialOrd` |
| `>=` | `expr >= expr` | 大于等于比较 | `PartialOrd` |
| `>>` | `expr >> expr` | 向右位移运算 | `Shr` |
| `>>=` | `var >>= expr` | 向右位移运算并赋值 | `ShrAssign` |
| `@` | `ident @ pat` | 模式绑定 |  |
| `^` | `var ^ expr` | 按位异或运算 | `BitXor` |
| `^=` | `var ^= expr` | 按位异或运算并赋值 | `BitXorAssign` |
| <code>&vert;</code> | <code>pat &vert; pat</code> | 模式选择，pattern alternatives | |
| <code>&vert;</code> | <code>expr &vert; expr</code> | 按位或（OR）运算 | `BitOr` |
| <code>&vert;=</code> | <code>var &vert;= expr</code> | 按位或（OR）运算并赋值 | `BitOrAssign` |
| <code>&vert;&vert;</code> | <code>expr &vert;&vert; expr</code> | 短路逻辑或运算，Short-circuiting logical OR | |
| `?` | `expr?` | 错误传递 |  |

### 非运算符的符号

**Non-operator Symbols**

以下清单包含了不以运算符发挥作用的全部符号；那就是说，他们不会表现得像函数或方法调用。

表 B-2 给出了自己单独出现，并在多种场合有效的一些符号。

**<small>表 B-2：独立语法，Stand-Alone Syntax</small>**

| 符号 | 说明 |
| :--- | :--- |
| `'ident` | 命名的生命周期或循环标签 |
| `...u8`, `...i32`, `...f64`, `...usize` 等等 | 指定类型的数字字面值 |
| `"..."` | 字符串字面值 |
| `r"..."`, `r#"..."#`, `r##"..."##` 等等 | 原始字符串字面值，其中的转义字符不会被处理 |
| `b"..."` | 字节字符串字面值；构造出一个字节数组而非字符串 |
| `br"..."`, `br#"..."`, `br##"..."##` 等等 | 原始字节字符串字面值，是原始与字节字符串字面值的结合 |
| `'...'` | 字符字面值 |
| `b'...'` | ASCII 字节字面值 |
| <code>&vert;...&vert; expr</code> | 闭包 |
| `!` | 发散函数下总是空的底部类型，always empty bottom type for diverging functions |
| `_` | “忽略，ignored” 模式绑定；还用于令到整数字面值可读，also used to make integer literals readable |


表 B-3 展示了出现在模组层次结构中到某个项目路径上下文中的一些符号。

**<small>表 B-3：路径相关的语法</small>**

| 符号 | 说明 |
| :--- | :--- |
| `ident::ident` | 命名空间路径 |
| `::path` | 相对于代码箱根的路径（比如，某个显式绝对路径） |
| `self::path` | 相对于当前模组的路径（比如，某个显式相对路径） |
| `super::path` | 相对于当前模组父模组的路径 |
| `type::ident`, `<type as trait>::ident` | 关联的常量、函数及类型 |
| `<type>::...` | 无法直接命名的某个类型的关联项目（比如，`<&T>::...`, `<[T]>::...` 等等） |
| `trait::method(...)` | 通过命名出定义方法的类型，消除该方法调用的歧义 |
| `<type as trait>::method(...)` | 通过命名出特质与类型，消除方法调用的歧义 |

表 B-4 展示了出现在运用泛型参数上下文中的一些符号。

**<small>表 B-4：泛型</small>**

| 符号 | 说明 |
| ：--- | ：--- |
| `path<...>` | 指明类型中的泛型参数（比如，`Vec<u8>`） |
| `path::<...>`, `method::<...>` | 指明表达式中泛型、函数或方法的参数；通常这被称作涡轮鱼语法，turbofish（比如，`"42".parse::<i32>()`，关于 Rust 的 turbofish 语法，请参考：[What is Rust's turbofish](https://techblog.tonsser.com/posts/what-is-rusts-turbofish)），[RUST 中的 turbofish 语法（一）](https://www.jianshu.com/p/9107685ece03) ... |
| `fn ident<...> ...` | 定义出泛型函数 |
| `struct ident<...> ...` | 定义出泛型结构体 |
| `enum ident<...> ...` | 定义出泛型枚举 |
| `impl<...> ...` | 定义出泛型实现 |
| `for<...> type` | 高阶声明周期边界，higher-ranked lifetime bounds |
| `type<ident=type>` | 其中一个或更多的关联类型有着指定赋值的某种泛型（a generic type where one or more associated types have specific assignments，比如，`Iterator<Item=T>`）

下表 B-5 展示了出现在使用特质边界的约束性泛型参数上下文中的一些符号，table B-5 shows symbols that appear in the context of constraining generic type parameters with trait bounds。

**<small>B-5：特质边界约束，Trait Bound Constrains</small>**

| 符号 | 说明 |
| :--- | :--- |
| `T: U` | 泛型参数 `T` 受实现了 `U` 的类型约束 |
| `T: 'a` | 泛型 `T` 必须要比生命周期 `'a` 活得更久，generic type `T` must outlive lifetime `'a`（意思是该类型不能间接地包含任何生命周期短于 `'a` 的引用） |
| `T: 'static` | 泛型 `T` 不包含除 `'static` 的引用外的其他引用 |
| `'b: 'a` | 泛型生命周期 `'b` 必须要比 `'a` 存活得更久 |
| `T: ?Sized` | 允许泛型参数为动态大小类型 |
| `'a + trait`, `trait + trait` | 复合的类型约束 |

下表 B-6 展示了出现在宏调用或定义上下文中，并指明了某个项目上属性的一些符号。

**<small>B-6：宏与属性</small>**

| 符号 | 说明 |
| :--- | :--- |
| `#[meta]` | 外层属性 |
| `#![meta]` | 内层熟悉 |
| `$ident` | 宏代换，macro substitution |
| `$ident:kind` | 宏捕获 |
| `$(...) ...` | 宏重复，macro repetition |
| `ident! (...)`, `ident! {...}`, `ident! [...]` | 宏调用，macro invocation |

下表 B-7 展示了创建注释的一些符号。

**<small>表 B-7：注释</small>**

| 符号 | 说明 |
| :--- | :--- |
| `//` | 注释行 |
| `//!` | 内层行文档注释，inner line doc comment |
| `///` | 外层行文档注释，outter line doc comment |
| `/*...*/` | 注释块 |
| `/*!...*/` | 内层块文档注释，inner block doc comment |
| `/**...*/` | 外层块文档注释，outter block doc comment |

下表 B-8 展示了出现于用到元组上下文中的一些符号。

**<small>元组</small>**

| 符号 | 说明 |
| :--- | :--- |
| `()` | 空元组（又叫单元值），同时属于字面值与类型 |
| `(expr)` | 元括号括起来的表达式，parenthesized expression |
| `(expr,)` | 单一元素的元组表达式 |
| `(type,)` | 单一元素的元组类型，single-element tuple type |
| `(expr, ...)` | 元组表达式 |
| `(type, ...)` | 元组类型，tuple type |
| `expr(expr, ...)` | 函数调用表达式；还用于初始化一些元组的 `struct` 以及元组的 `enum` 变种，function call expression; also used to initialize tuple `struct`s and tuple `enum` vairants |
| `expr.0`, `expr.1` 等等 | 对元组进行索引 |

下表 B-9 展示了其中用到花括号上下文中的一些符号。

**<small>表 B-9：花括号</small>**

| 符号 | 说明 |
| :--- | :--- |
| `{...}` | 代码块表达式 |
| `Type {...}` | `struct` 的字面值 |

下表 B-10 展示了其中用到方括号上下文中的一些符号。

**<small>表 B-10：方括号</small>**

| 符号 | 说明 |
| :--- | :--- |
| `[...]` | 数组的字面值 |
| `[expr; len]` | 包含着 `expr` 的 `len` 拷贝数组的字面值 |
| `[type; len]` | 包含着 `len` 个 `type` 的实例数组的字面值 |
| `expr[expr]` | 对集合进行索引，collection indexing。是可过载的 `(Index, IndexMut)`，overloadable `(Index, IndexMut)` |
| `expr[..]`, `expr[a..]`, `expr[..b]`, `expr[a..b]` | 用到了 `Range`、`RangeFrom`、`RangeTo` 或 `RangeFull` 作为 “索引”的，带有集合切片集合索引，collection indexing pretending to be collection slicing, using `Range`, `RangeFrom`, `RangeTo`, or `RangeFull` as the "index" |


## 附录 C：派生特质

**Appendix C: Derivable Traits**

本书的多个不同地方，咱们都曾讨论过 `derive` 属性，咱们可将其应用到结构体或枚举定义。`derive` 属性会在咱们以 `derive` 语法注解的类型上，生成将以某个特质自身默认实现，而实现该特质的代码。

在这个附录中，咱们会提供到标准库中，咱们可以与 `derive` 一起使用的全部特质的参考。以下各个小节均会讲到：

- 此特质将启用那些操作符与方法；
- 由 `derive` 所提供到的该特质实现会做些什么；
- 实现该特质对那个类型意味着什么；
- 允许及不允许实现该特质的情况；
- 需要该特质操作的示例。

若咱们想要不同于由 `derive` 属性所提供的行为，请参考 [标准库文档](https://doc.rust-lang.org/std/index.html)，了解如何亲自实现各个特质的详细信息。

这里列出的这些特质，只是一些由标准库所提供的，可使用 `derive` 实现于咱们类型上的那些。定义在标准库中别的一些特质，则没有什么合理的默认行为，因此是否要以对于咱们正尝试完成的东西有意义的方式，实现他们就取决于咱们自己了。

不能派生的一个特质示例便是 `Display`，其为终端用户处理格式化。咱们应始终要考虑将某个类型显示给用户的恰当方式。终端用户应被允许看到该类型的哪些部分？他们会发现哪些部分是相关的？数据的何种形式才是与他们最为密切相关的？Rust 编译器并无这种见解，因此他就无法为咱们提供到恰当的默认行为。

这个附录中所提供到的派生特质清单并不详尽：库可以为他们自己的特质实现 `derive`，从而领导咱们可使用 `derive` 的特质清单为真正开放的。实现 `derive` 设计到使用程序性宏，这在第 19 章的 [“关于宏”](Ch19_Advanced_Features.md#关于宏) 小节讲到过。

### 输出给编程者的 `Debug`

**`Debug` for Programmer Output**

`Debug` 特质实现了格式字符串中的格式化，所谓格式字符串，即咱们通过在 `{}` 里添加 `:?` 所表示的。

`Debug` 特质允许咱们为调试目的打印某种类型的实例，如此咱们以及用到咱们类型的其他编程者，就可以在程序执行的某个特定时刻，就其某个实例加以探查。

在比如用到 `assert_eq!` 宏中等情况下，`Debug` 特质便是要求使用的。`assert_eq!` 这个宏在相等断言失败时，就会打印出作为参数所给到的两个实例值，如此编程者就可以看到为何这两个实例不相等。


### 用于相等比较的 `PartialEq` 与 `Eq`

`PartialEq` 特质允许咱们比较某种类型的两个实例，来检查他们是否相等，并实现 `==` 与 `!=` 运算符的应用。

对 `PartialEq` 进行派生，就会实现 `eq` 方法。当 `ParitalEq` 实在结构体上实现的时，只有在两个实例的 *全部* 字段都相等时，他们才是相等的，且在有任何字段不等时，两个实例便不相等。当在枚举上派生时，枚举的各个变种与自身相等，而不等于其他任何变种。

在使用需要能够比较某个类型的两个实例是否相等的 `assert_eq!` 宏时，就需要这个 `PartialEq` 特质。

而 `Eq` 特质则没有方法。他的目的是要表明，所注解的类型的每个值，其值都等于他自身。尽管并非所有实现 `PartialEq` 的类型都可以实现 `Eq`，但 `Eq` 特质却只可应用到那些同时实现了 `PartialEq` 的类型。这方面的一个示例，便是浮点数类型：浮点数的实现，就表明两个非数字（the not-a-number, `NaN`）的值，是各自不相等的。

要求 `Eq` 的一个示例，就是 `HashMap<K, V>` 中的那些键，如此 `HashMap<K, V>` 就可以区分出两个键是否一致。


### 用于排序比较的 `PartialOrd` 与 `Ord`

**`PartialOrd` and `Ord` for Ordering Comparisons**

`PartialOrd` 特质实现为排序目的，而比较某种类型的那些实例。实现了 `PartialOrd` 的类型，便可与 `<`、`>`、`<=` 及 `>=` 符号一起使用了。咱们只能对那些同时实现了 `PartialEq` 的类型，应用这个 `PartialOrd` 特质。

派生 `PartialOrd`，会实现 `partial_cmp` 方法，该方法会返回一个在所给的那些值不会产生出顺序时，将为 `None` 的一个 `Option<Ordering>`。至于即使那种类型的大多数值都可被比较，但仍不会产生出顺序的值的一个示例，便是非数字（`NaN`）浮点值。在任何浮点数和非数字浮点值下调用 `partial_cmp`，都会返回 `None`。

在于结构体上派生时，`PartialOrd` 会通过字段出现在结构体定义中的顺序，比较每个字段中的值，比较两个实例。而当于枚举上派生时，枚举定义中较早声明的枚举变种，被当作是小于后面所列出的那些变种的。

在比如会产生出由范围表达式所指定范围中一个随机数的， `rand` 代码箱的 `gen_range` 方法来说，`PartialOrd` 特质便是需要的。

`Ord` 特质实现对所注解类型的任何两个值，将存在有效顺序的掌握。`Ord` 特质会实现 `cmp` 方法，由于有效排序将始终可行，因此该方法返回的是 `Ordering` 而非 `Option<Ordering>`。咱们只可对那些同时实现了 `PartialOrd` 及 `Eq` (而 `Eq` 要求 `PartialEq`) 的类型，实现这个 `Ord` 特质。当于结构体及枚举上派生 `Ord` 时，`cmp` 就会以与 `PartialOrd` 下 `partial_cmp` 的派生实现同样方式行事。

要求 `Ord` 的一个示例，即为将一些值存储在 `BTreeSet<T>` 这种根据值的排序，而存储数据的数据结构中时。

### 用于复制值的 `Clone` 与 `Copy`

**`Clone` and `Copy` for Duplicating Values**

`Clone` 特质实现了显式创建值的深拷贝，而该复制过程则可能涉及运行一些任意代码，arbitary code，与拷贝内存堆数据。请参阅第 4 章中 [“变量与数据交互方式：克隆”](Ch04_Understanding_Ownership.md#变量与数据交互方式之二克隆) 小节，了解更多有关 `Clone` 的信息。

派生 `Clone` 会实现 `clone` 方法，当对整个类型实现了这个方法时，其就会在该类型的各个部分上调用 `clone`。这意味着类型要派生 `Clone` 其中的全部字段或值，都必须同时实现 `Clone`。

需要 `Clone` 特质的一个示例，便是在切片上调用 `to_vec` 方法时。切片不持有其包含的那些类型实例，但自 `to_vec` 所返回的那个矢量值，却将需要持有他的那些实例，从而 `to_vec` 会调用各个条目上的 `clone`。因此，存储在切片中的类型，就必须实现 `Clone`。

`Copy` 特质实现了只通过拷贝存储在栈上的二进制位，而复制某个值；任意代码并无必要。请参阅第 4 章中 [“唯栈数据：拷贝”](Ch04_Understanding_Ownership.md#唯栈数据拷贝stack-only-data-copy)，了解更多有关 `Copy` 的信息。

`Copy` 特质没有定义阻止编程者过载那些方法，及破坏不会有任意代码运行这个假设的任何方法。那样的话，所有编程者就都可以假定，拷贝值将会非常快。

咱们可在其组成部分都实现了 `Copy` 的任何类型上派生 `Copy` 特质。由于实现 `Copy` 的类型，都有着执行与 `Copy` 同样任务的一个 `Clone` 的简单实现，因此实现 `Copy` 的类型必须同时实现 `Clone`。

很少需要 `Copy` 特质；实现了 `Copy` 的类型，有着可供选择的优化方案，意味着咱们不必调用 `clone`，而调用 `clone` 会令到代码更简洁。

对于 `Copy` 下每种可能情况，咱们都可同时以 `Clone` 完成，除了代码可能更慢，或在一些地方不得不使用 `clone`。


### 用于将值映射到固定大小值的 `Hash`

**`Hash` for Mapping a Value to a Value of Fixed Size**


`Hash` 特质实现了取某种任意大小类型的实例，并通过使用散列函数，将那个实例映射到固定大小的值。派生 `Hash` 会实现 `hash` 方法。`hash` 放的派生实现，会将在该类型各个组成部分上调用 `hash` 的结果结合起来，这就意味着类型要派生 `Hash`，那么其全部字段，都必须同时实现 `Hash`。

要求 `Hash` 的一个示例，便是为了高效地存储数据，而在 `Hash<K, V>` 中存储那些键时。


### 用于默认值的 `Default`

**`Default` for Default Values**

`Default` 特质实现了为类型创建出一个默认值。派生 `Default` 会实现 `default` 函数。`default` 函数的派生实现，会在类型的各个部分上调用 `default` 函数，意味类型要派生 `Defualt`，其中的全部字段或值，都必须同时实现 `Default`。

`Default::default` 函数，通常是与第 5 章中 [“使用结构体更新语法从其他实例创建出实例”](Ch05_Using_Structs_to_Structure_Related_Data.md#使用结构体更新语法从其他实例创建出实例) 小节里曾讨论过的结构体更新语法结合使用的。咱们可以定制结构体的几个字段，并在随后通过使用 `..Default::default()`，为其余字段设置并使用默认值。

在 `Option<T>` 实例上使用 `unwrap_or_default` 方法时，便是需要 `Default` 特质的一个示例。当那个 `Option<T>` 为 `None` 时，方法 `unwrap_or_default` 就将返回存储在 `Option<T>` 中，那个类型 `T` 的 `Default::default` 结果。


## 附录 D：一些有用开发工具

在此附录中，咱们会讲到 Rust 项目所提供的一些有用的开发工具。咱们将看看自动格式化、应用警告修复的一些快速方法、一种代码静态分析工具，a linter，以及与多种 IDE 的集成。


### 使用 `rustfmt` 的自动格式化

**Automatic Formatting with `rustfmt`**

`rustfmt` 工具会依据社区编码风格，重新格式化咱们的代码。许多协作项目，都使用了 `rustfmt` 来防止有关编写 Rust 时使用何种风格方面的争论：每个人都使用这个工具来格式化他们的代码。

要安装 `rustfmt`，请键入下面的命令：

```console
$ rustup component add rustfmt
```

如同 Rust 会同时给到 `rustc` 与 `cargo` 一样，此命令会给到咱们 `rustfmt` 与 `cargo-fmt`。要格式化任何 Cargo 项目，请敲入下面的命令：

```console
$ cargo fmt
```

运行此命令，会重新格式化当前代码箱中全部的 Rust 代码。这只会改变编码风格，而不会改变代码语义。关于 `rustfmt` 的更多信息，请参阅 [其文档](https://github.com/rust-lang/rustfmt).


### 使用 `rustfix` 修复咱们的代码

**Fix Your Code with `rustfix`**

`rustfix` 工具已被 Rust 安装所包含，并可大致以咱们想要的方式，修复那些有着明确纠正问题方法的一些编译器告警。咱们之前大概率已经见到过编译器告警了。比如，设想有下面这段代码：

文件名：`src/main.rs`

```rust
fn do_something() {}

fn main() {
    for i in 0..100 {
        do_something();
    }
}
```

此处，咱们正调用 `do_something` 函数 100 次，但咱们在 `for` 循环的代码体中，从未用到那个变量 `i`。Rust 就会就此对咱们发出告警：

```console
$ cargo build
   Compiling rustfix_demo v0.1.0 (/home/lenny.peng/rust-lang-zh_CN/rustfix_demo)
warning: unused variable: `i`
 --> src/main.rs:4:9
  |
4 |     for i in 0..100 {
  |         ^ help: if this is intentional, prefix it with an underscore: `_i`
  |
  = note: `#[warn(unused_variables)]` on by default

warning: `rustfix_demo` (bin "rustfix_demo") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.29s
```

这个告警建议咱们要使用 `_i` 做名字：其中的下划线表示咱们有意不使用这个变量。通过运行 `cargo fix` 命令，咱们就可以使用 `rustfix`，自动应用那项建议：

```console
$ cargo fix --allow-no-vcs
    Checking rustfix_demo v0.1.0 (/home/lenny.peng/rust-lang-zh_CN/rustfix_demo)
       Fixed src/main.rs (1 fix)
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
```

当咱们再次看到 `src/main.rs`，就将发现 `cargo fix` 已修改了这段代码：

文件名：`src/main.rs`

```rust
fn do_something() {}

fn main() {
    for _i in 0..100 {
        do_something();
    }
}
```

那个 `for` 循环变量，现在就被命名为了 `_i`，同时那条告警也不再出现了。

咱们还可使用 `cargo fix` 命令，将咱们的代码在不同 Rust 版本之间转换。有关这些 Rust 版本，在附录 E 中有讲到。


### 使用 Clippy 获得更多的代码静态分析

**More Lints with Clippy**

Clippy 工具是用于分析咱们代码，从而咱们可以捕获到一些常见错误，而改进咱们 Rust 代码的一套代码静态分析集合。

要安装 Clippy，请输入以下命令：

```console
$ rustup component add Clippy
```

在任何 Cargo 项目上要运行 Clippy 的静态分析，请输入以下命令：

```console
$ cargo clippy
```

比如说咱们编写了像下面这个程序这样，用到某个数学常量近似值，好比说 `pi`，的一个程序：

文件名：`src/main.rs`

```rust
fn main() {
    let x = 3.1415;
    let r = 8.0;
    println!("圆的面积为 {}", x * r * r);
}
```

在这个项目上运行 `cargo clippy` 就会得到下面的报错：

```console
$ cargo clippy
    Checking clippy_demo v0.1.0 (/home/lenny.peng/rust-lang-zh_CN/clippy_demo)
error: approximate value of `f{32, 64}::consts::PI` found
 --> src/main.rs:2:13
  |
2 |     let x = 3.1415;
  |             ^^^^^^
  |
  = help: consider using the constant directly
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#approx_constant
  = note: `#[deny(clippy::approx_constant)]` on by default

error: could not compile `clippy_demo` due to previous error
```

此报错让咱们明白，Rust 已经定义了一个更精确的 `PI` 常量，且当咱们使用这个常量时，咱们的程序将更为正确。那么咱们随后就应修改咱们代码为使用这个 `PI` 常量。下面的代码就捕获导致 Clippy 的任何错误或告警：

文件名：`src/main.rs`

```rust
fn main() {
    let x = std::f64::consts::PI;
    let r = 8.0;
    println!("圆的面积为 {}", x * r * r);
}
```

有关 Clippy 的更多信息，请参阅 [其文档](https://github.com/rust-lang/rust-clippy)。

### 用到 `rust-analyzer` 的 IDE 集成

**IDE Integration Using `rust-analyzer`**

为帮助 IDE 集成，Rust 社区建议使用 [`rust-analyzer`](https://rust-analyzer.github.io/)。此工具是一套以编译器为中心，操 [语言服务器协议，Language Server Protocol](http://langserver.org/) 的实用工具；而所谓语言服务器协议，则是用于各种 IDEs 和编程语言，二者相互之间通信的一种规格。有多种不同客户端可使用 `rust-analyzer`，比如 [Visual Studio Code 的 Rust 分析器插件](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)。

请访问 `rust-analyzer` 项目 [主页](https://rust-analyzer.github.io/)，了解其安全说明，随后在咱们的特定 IDE 中安装该语言的服务器支持。咱们的 IDE 就能获得诸如自动补全、跳至定义及行内报错等能力。


## 附录 E：关于版本

**Appendix E - Editions**

在第一章中，咱们曾看到 `cargo new` 会把一点有关某个版的元数据，添加到咱们的 `Cargo.toml` 文件。此附录就会讲到那意味着什么！

Rust 语言及编译器有着六周的发布周期，意味着用户会得到源源不断的新功能。其他编程语言会不经常地发布较大变更；Rust 则会更频繁发布较小的更新。不久之后，全部这些小修改就会堆积起来。不过这一个个发布中，回头看看而讲到，“噢，从版本 1.10 到 1.31，Rust 改变了很多！”。则是不容易的。

每两三年，Rust 团队都会产生一个新的 Rust *版本，edition*。每个版本都会以完全更新的文档与工具，将那些业已落地到一个明确包中的特性放到一起。新版本会作为寻常的六周发布过程而交付。

这些版本服务了不同人群的不同目的：

- 对于活跃的 Rust 用户，新版本会把那些增量变更，一起放入到一个易于掌握的包中；
- 对于那些非用户，新版本释放了一些已落地的大进展信号，这会让 Rust 或许值得再看一看；
- 对于开发 Rust 的人们，新版本会提供这个项目作为整体的集结点。

在本书编写时，已有三个 Rust 版本可用：Rust 2015、Rust 2018 与 Rust 2021。本书是用 Rust 2021 版本的习惯用语编写的。

`Cargo.toml` 中的 `edition` 键，表示应对咱们的代码使用哪个版本的编译器。若该键不存在，Rust 就会以向后兼容原因，而使用 `2015` 作为版本值。

每个项目都可以选择一个不同于默认 2015 的版本。这些版本可能包含了不兼容的变更，比如包含了与代码中标识符冲突的新关键字。但是，除非咱们选到这些变更，那么即使咱们更新了所使用的 Rust 编译器，咱们的代码将继续编译。

全部 Rust 编译器版本，都会支持先于那个编译器发布而存在的任何版本，且他们可将任何受支持版本的代码箱连接起来。版本变更只会影响编译器于编译初期解析代码的方式。因此，当咱们正使用着 Rust 2015，而咱们的一项依赖使用了 Rust 2018 时，咱们的项目将编译，并能够使用那项依赖。与之相反，在咱们的项目使用 Rust 2018，而一项依赖使用了 Rust 2015 的情形下，也会工作。

要明确的是：绝大多数特性，在所有版本上都将可用。使用任何 Rust 版本的开发者，都将在新的稳定发布构造出来时，发现一些改进。但是，在一些情况下，主要是在新曾了关键字时，一些新特性就会只在稍后版本中可用了。若咱们打算利用上这些新特性，咱们将需要切换版本。

有关更多细节，[版本指南，Edition Guide](https://doc.rust-lang.org/stable/edition-guide/) 是本列举了不同版本间差异，并解释了怎样通过 `cargo fix`，而自动将咱们的代码更新到新版的一本完整的书。


## 附录 F - 本书的一些译本

<略>

## 附录 G - Rust 是怎样构造出来的与每日发布

**How Rust is Made and "Nightly Rust"**

此附录是有关 Rust 被怎样构造出来，及那会怎样作为一名 Rust 开发者的你。

### 强调稳定却并无止步不前

**Stability Without Stagnation**

作为一门语言，Rust 在注重咱们代码稳定性方面 *用心良苦*。咱们希望 Rust 成为你可以在其上构建软件的稳固基础，而若那些物件都一直变动，那将是不可能实现的。而与此同时，若咱们无法实验一些新特性，那么直到这些特性发布后咱们不能在修改一些东西时，咱们也不会发现一些重大缺陷。

对于这个问题，咱们（Rust 团队）的解决方案就是咱们称作 “强调稳定又不要止步不前”，而咱们的直到原则是这样的：你永不必害怕升级到稳定的Rust 新版本。每次升级都应是无痛的，而又应带给你一些新特性、更少的程序错误，以及更快的编译时间。


### 啾，啾！发布通道与搭上快车

**Choo, Choo! Release Channels and Riding the Trains**

Rust 的开发，是运作在 *火车时刻表，train schedule* 上的。那就是说，全部开发都是在 Rust 代码仓库的 `master` 分支上完成的。各个发布遵循了软件发布列车模型，a software release train model，该发布模型业已为 Cisco IOS 及其他软件项目所使用。Rust 有着以下三个 *发布通道，release channels*：

- 每日发布，nightly
- Beta 发布，beta
- 稳定发布，stable

多数 Rust 开发者主要使用稳定通道，而那些希望尝试实验性新特性的人们，则会使用每日发布或 beta 通道。

下面是个开发与发布流程运作方式的一个示例：咱们来假定 Rust 团队正工作于 Rust 1.5 的发布上。那个发布发生于 2015 年 11 月，但其将提供到我们实际版本数字。有个新特性被添加到 Rust：一次新提交落在了 `master` 分支。每天晚上，都有一个新的 Rust 每日版本被产生出来。每天都是个发布日，而这些发布是由咱们的发布基础设施自动创建的。因此随着时间流逝，咱们的发布看起来就像下面这样，每晚一次：

```text
nightly: * - - * - - *
```

每隔六周，便是要准备一个新发布的时候了！Rust 代码仓库的 `beta` 分支，便会从由每日发布所使用的 `master` 分支分叉开来。现在，就有了两个分支：

```text
nightly: * - - * - - *
                     |
beta:                *
```

多数 Rust 使用者不会积极使用这些 beta 发布，但会在他们的 CI 系统中就 beta 发布加以测试，以帮助 Rust 发现可能出现的倒退。与此同时，仍有着每晚的每日发布：

```text
nightly: * - - * - - * - - * - - *
                     |
beta:                *
```

在首个 beta 版创建出来六周后，就是稳定发布的时候了！`stable` 分支就被从 `beta` 分支创建出来：

```text
nightly: * - - * - - * - - * - - * - - * - * - *
                     |
beta:                * - - - - - - - - *
                                       |
stable:                                *
```

好！Rust 1.5 便完成了！不过，咱们忘了一件事：由于这六个星期以及过去，而咱们还需要 Rust *下一* 版本，1.6，的一个新的 beta 发布。因此在 `stale` 分支从 `beta` 分支分叉出来后，下一版本的 `beta` 又会从 `nightly` 再度分叉出来：

```text
nightly: * - - * - - * - - * - - * - - * - * - *
                     |                         |
beta:                * - - - - - - - - *       *
                                       |
stable:                                *
```

每六周就有一个发布 “离站”，但发布过程仍务必要在其抵达稳定发布前，经由这个 beta 通道行驶一段路程，由此这个过程便被称为 “列车模型”。

Rust 每六周发布，像时刻表一样。若咱们知道了一个 Rust 发布的日期，那么就能直到下一发布的日期：那便是六周后。每六周安排一次发布的一个好处，便是下一班列车很快就会到来。若某项特性刚好错过了某个特定发布，那么无需担心：另一发布将在不久后发生！这有助于减少在临近发布截止日期时，有可能未完善的功能偷偷潜入的压力。


归功于这个流程，咱们可以始终检出，check out，下一构建的 Rust，并自己验证到升级是容易的：若 beta 发布没有如预期那样工作，咱们就可以将其报告给 Rust 团队，并在下一稳定发布发生前修好他！beta 发布中的损坏相对较少，但 `rustc` 仍属于一个软件，而确实存在一些错误。


### 不稳定特性

**Unstable Features**

这种发布模型下，还有一个好处：不稳定特性。Rust 使用了一种名为 “特性标识，feature flags” 的技巧，来确定出给定发布中启用了哪些特性。若某项新特性处于活跃开发中，他就会落地在 `master` 分支上，而由此就会在每日发布中，但会有着一个 *特性标识*。而咱们，作为用户，希望尝试这个进展中的特性，the work-in-progress feature，时，咱们是可以尝试的，但必须使用 Rust 的每日发布，并使用恰当的标识来注解咱们的代码，来选用该特性。

若咱们使用着 beta 或稳定发布的 Rust，那么就不能使用任何特性标识。这是 Rust 团队在声明那些新特性永久稳定前，允许咱们实际用到他们的关键。希望选用最新特性的人们，便可这样做，而想要一种扎实体验的人，则可坚持使用稳定发布，而清楚他们的代码不会破坏。这便是稳定但并非止步不前。

由于那些工作中的特性仍在便会，且在本书写作时和他们在稳定构建中启用时，其间他们肯定将有所不同，因此本书只包含了那些稳定特性的信息。咱们可以在线上找到那些仅每日发布有的特性文档。

### Rustup 与 Rust 每日发布所扮演的角色

**Rustup and the Role of Rust Nightly**

Rust 令到易于在全局或每个项目基础上，从不同发布通道的 Rust 之间改变。默认情况下，咱们将安装稳定发布的 Rust。而比如要安装每日发布：

```console
$ rustup toolchain install nightly
```

咱们也可以使用 `rustup`，查看全部的 *工具链，toolchains* （Rust 的各个发布与关联组件）。下面就是本书一位作者的 Windows 计算机上的示例：

```powershell
> rustup toolchain list
stable-x86_64-pc-windows-msvc (default)
beta-x86_64-pc-windows-msvc
nightly-x86_64-pc-windows-msvc
```

> 在 Linux 系统上的输出如下：

```console
$ rustup toolchain list
stable-x86_64-unknown-linux-gnu (default)
```

可以看到，稳定发布的工具链是默认的。绝大多数 Rust 用户会在多数时候使用稳定发布。咱们可能想要在多数时候使用稳定发布，又因为咱们关心某项最新特性，而会在特定项目使用每日发布。要这样做，就可以在那个项目目录下，使用 `rustup override` 来将每日发布工具链，设置为当咱们位处那个目录中时，`rustup` 使用的那个工具链：

```console
$ cd ~/projects/needs-nightly
$ rustup override set nightly
```

现在，当咱们每次在 `~/projects/needs-nightly` 目录下调用 `rustc` 或 `cargo` 时，`rustup` 都会确保咱们在使用每日发布的 Rust，而非咱们默认的稳定发布 Rust 了。再有很多 Rust 项目时，这就会排上用场!

### 请求评议流程与各种团队

**The RFC Process and Teams**

那么咱们该怎么了解到这些新特性呢？Rust 的开发模型，遵循了 *请求评议流程，Request For Comments(RFC) process*。如你想要 Rust 的一项改进，那么就可以编写一个名为请求评议，RFC 的提议。

人人都可以编写请求评议来改进 Rust，同时这些提议会经过由许多议题子团队所组成的 Rust 团队审阅和讨论。[在 Rust 网站上](https://www.rust-lang.org/governance) 有这些团队的完整清单，其中包括了该项目各领域：语言设计、编译器实现、基础设施、文档及其他等的团队。恰当的团队会阅读提议与评论，撰写出他们自己的一些评论，并在最后，便有了接受或拒绝该特性的共识。

若该特性被接受了，就会在 Rust 代码仓库上开出一个 issue，同时某个人就可以实现他。将其实现得非常棒的那个人，可能不是最早提议这项特性的那人！在实现准备好时，其就会落地于 `master` 分支的特性门，a feature gate，之后，如同咱们曾在 [“不稳定特性”](#不稳定特性) 小节中曾讨论过的那样。

过了一段时间后，一旦那些用到每日发布的 Rust 开发者们，能够试用这项新特性，那么 Rust 团队成员将讨论这项特性，怎样将其编制到每日发布上，并决定其是否有那个被构造到稳定发布 Rust。而若决定是继续推进，那么特性门就会被移除，同时这项特性就被认为是稳定的了！他就会搭上列车，进到一个新的稳定发布 Rust  中。


## 附录 H - 有用笔记

此处记录学习及应用 Rust 编程软件过程中，觉得有用的一些东西。


### `cargo-binutils`

[这个项目](https://github.com/rust-embedded/cargo-binutils) 是 Embbeded-Rust 项目的，而不是 Rust 官方的，但提供了有用的功能。比如查看构建出的二进制程序文件的那些头部：


```console
$ cargo readobj --bin clippy_demo -- --file-headers
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
ELF Header:
  Magic:   7f 45 4c 46 02 01 01 00 00 00 00 00 00 00 00 00
  Class:                             ELF64
  Data:                              2's complement, little endian
  Version:                           1 (current)
  OS/ABI:                            UNIX - System V
  ABI Version:                       0
  Type:                              DYN (Shared object file)
  Machine:                           Advanced Micro Devices X86-64
  Version:                           0x1
  Entry point address:               0x86D0
  Start of program headers:          64 (bytes into file)
  Start of section headers:          4305200 (bytes into file)
  Flags:                             0x0
  Size of this header:               64 (bytes)
  Size of program headers:           56 (bytes)
  Number of program headers:         12
  Size of section headers:           64 (bytes)
  Number of section headers:         42
  Section header string table index: 41
```

使用前需要进行如下安装：

```console
$ cargo install cargo-binutils
$ rustup component add llvm-tools-preview
```

## 附录 I - 术语清单

- 单态化

所谓 *单态化，monomorphization*，是指即通过把在编译后用到的具体类型填入到泛型位置，而将通用代码转换为具体代码的过程。参考 [使用泛型代码的性能问题](Ch10_Generic_Types_Traits_and_Lifetimes.md#使用泛型参数代码的性能问题)。

- 内聚属性

a property called *coherence*，参见 [在类型上实现某个特质](Ch10_Generic_Types_Traits_and_Lifetimes.md#在类型上实现某个特质)。

- 孤儿规则

the orphan rule, 参见 [在类型上实现某个特质](Ch10_Generic_Types_Traits_and_Lifetimes.md#在类型上实现某个特质)。

