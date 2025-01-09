# 附录 B：运算符与符号

此附录包含了 Rust 语法的词汇表，包括运算符及别的一些，自己单独出现或出现于路径、泛型、特质边界、宏、属性、注释、元组及方括符等上下文中的符号。


## 运算符

**Operators**


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


## 非运算符的符号

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
| :-- | :-- |
| `path<...>` | 指明类型中的泛型参数（比如，`Vec<u8>`） |
| `path::<...>`, `method::<...>` | 指明表达式中泛型、函数或方法的参数；通常这被称作涡轮鱼语法，turbofish（比如，`"42".parse::<i32>()`，关于 Rust 的 turbofish 语法，请参考：[What is Rust's turbofish](https://techblog.tonsser.com/posts/what-is-rusts-turbofish)），[RUST 中的 turbofish 语法（一）](https://www.jianshu.com/p/9107685ece03) ... |
| `fn ident<...> ...` | 定义出泛型函数 |
| `struct ident<...> ...` | 定义出泛型结构体 |
| `enum ident<...> ...` | 定义出泛型枚举 |
| `impl<...> ...` | 定义出泛型实现 |
| `for<...> type` | 高阶声明周期边界，higher-ranked lifetime bounds |
| `type<ident=type>` | 其中一个或更多的关联类型有着指定赋值的某种泛型（a generic type where one or more associated types have specific assignments，比如，`Iterator<Item=T>`） |

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


（End）


