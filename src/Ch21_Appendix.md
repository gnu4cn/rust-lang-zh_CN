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


