# 附录 B：运算符与符号

这一附录包含 Rust 语法的词汇表，包括单独出现，或出现在路径、泛型、特质边界、宏、属性、注释、元组即方括号等上下文中的运算符和其他符号。


## 运算符

下表 B-1 包含 Rust 中的运算符、该运算符在上下文中出现方式的示例、简短说明，以及该运算符是否可重载。当运算符可重载时，则会列出用于重载该运算符的相关特质。

<a name="table_B-1"></a>
**表 B-1**：运算符

| 运算符 | 示例 | 说明 | 是否可重载？ |
| :- | :- | :- | :- |
| `!` | `ident! (...)`, `ident! {...}`, `ident! [...]` | 宏扩展 |   |
| `!` | `!expr` | 按位或，或逻辑求补 | 否 |
| `!=` | `expr != expr` | 不等的比较 | `PartialEq` |
| `%` | `expr % expr` | 算术求余 | `Rem` |
| `%=` | `var %= expr` | 算术求余并赋值 | `RemAssign` |
| `&` | `&expr`, `&mut expr` | 借用 |  |
| `&` | `&type`, `&mut type`, `&'a type`, `&'a mut type` | 借用指针类型 |  |
| `&` | `expr & expr` | 按位与 `AND` | `BitAnd` |
| `&=` | `var &= expr` | 按位与 `AND` 并赋值 | `BitAndAssign` |
| `&&` | `expr && expr` | 短路的逻辑与 `AND` 运算，short-circuit logical AND |  |
| `*` | `expr * expr` | 算术乘法 | `Mul` |
| `*=` | `var *= expr` | 算术乘法并赋值 | `MulAssign` |
| `*` | `*expr` | 解引用 | `Deref` |
| `*` | `*const type`, `*mut type` | 原始指针 |  |
| `+` | `trait + trait`, `'a + trait` | 复合类型的约束 |  |
| `+` | `expr + expr` | 算术加法 | `Add` |
| `+=` | `var += expr` | 算术加法并赋值 | `AddAssign` |
| `,` | `expr, expr` | 参数和元素的分隔符 |  |
| `-` | `- expr` | 算术取反运算 | `Neg` |
| `-` | `expr - expr` | 算术减法 | `Sub` |
| `-=` | `var -= expr` | 算术减法并赋值 | `SubAssign` |
| `->` | `fn(...) -> type`, <code>&vert;...&vert; -> type</code> | 函数与闭包的返回类型 |  |
| `.` | `expr.ident` | 字段访问 |  |
| `.` | `expr.ident(expr, ...)` | 方法调用 |  |
| `.` | `expr.0`, `expr.1` 等等 | 元组索引 |  |
| `..` | `..`, `expr..`, `..expr`, `expr..expr` | 右边界范围字面值 | `PartialOrd` |
| `..=` | `..=expr`, `expr..=expr` | 包含右侧的范围字面值 | `PartialOrd` |
| `..` | `..expr` | 结构体字面值更新语法 |  |
| `...` | `expr...expr` | （已弃用，请改用 `..=`）在模式中：包含范围模式 |  |
| `/` | `expr / expr` | 算术除法 | `Div` |
| `/=` | `var /= expr` | 算术除法并赋值 | `DivAssign` |
| `:` | `pat: type`, `ident: type` | 约束，限制条件 |  |
| `:` | `ident: expr` | 结构体字段初始化器 |  |
| `:` | `'a: loop {...}` | 循环标签 |  |
| `;` | `expr;` | 语句及项目的终止符 |  |
| `;` | `[..., len]` | 固定长度数组语法的一部分 |  |
| `<<` | `expr << expr` | 向左移位 | `Shl` |
| `<<=` | `var <<= expr` | 向左移位并赋值 | `ShlAssign` |
| `<` | `expr < expr` | 小于比较 | `PartialOrd` |
| `<=` | `expr <= expr` | 小于等于比较 | `PartialOrd` |
| `=` | `var = expr`, `ident = type` | 赋值/等价 |  |
| `==` | `expr == expr` | 相等性比较 | `PartialEq` |
| `=>` | `pat => expr` | 匹配支臂语法的一部分 |  |
| `>` | `expr > expr` | 大于比较 | `PartialOrd` |
| `>=` | `expr >= expr` | 大于等于比较 | `PartialOrd` |
| `>>` | `expr >> expr` | 向右移位 | `Shr` |
| `>>=` | `var >>= expr` | 向右移位并赋值 | `ShrAssign` |
| `@` | `ident @ pat` | 模式绑定 |  |
| `^` | `var ^ expr` | 按位异或 | `BitXor` |
| `^=` | `var ^= expr` | 按位异或并赋值 | `BitXorAssign` |
| <code>&vert;</code> | <code>pat &vert; pat</code> | 替代模式 | |
| <code>&vert;</code> | <code>expr &vert; expr</code> | 按位或 `OR`  | `BitOr` |
| <code>&vert;=</code> | <code>var &vert;= expr</code> | 按位或` OR` 并赋值 | `BitOrAssign` |
| <code>&vert;&vert;</code> | <code>expr &vert;&vert; expr</code> | 短路的逻辑或 `OR` | |
| `?` | `expr?` | 错误传播 |  |


## 非运算符的符号

以下表格包含所有不用作运算符的符号；也就是说，他们的行为不像函数或方法调用。

下表 B-2 展示了单独出现且在多种位置都有效的符号。

<a name="table_B-2"></a>
**表 B-2**：独立语法，Stand-Alone Syntax

| 符号 | 说明 |
| :--- | :--- |
| `'ident` | 命名的生命周期或循环标签 |
| 紧跟在数字后面的 `u8`、`i32`、`f64`、`usize` 等 | 指定类型的数字字面值 |
| `"..."` | 字符串字面值 |
| `r"..."`, `r#"..."#`, `r##"..."##` 等 | 原始字符串字面值，转义字符不会被处理 |
| `b"..."` | 字节字符串字面值；构造出一个字节数组而非字符串 |
| `br"..."`, `br#"..."`, `br##"..."##` 等 | 原始字节字符串字面值；原始字节字符串字面值与字节字符串字面值的组合 |
| `'...'` | 字符字面值 |
| `b'...'` | ASCII 的字节字面值 |
| <code>&vert;...&vert; expr</code> | 闭包 |
| `!` | 用于发散函数的常空底部类型，always empty bottom type for diverging functions |
| `_` | “忽略” 模式绑定；还用于使整数字面值可读。"Ignored" pattern binding; also used to make integer literals readable |


下表 B-3 展示了在通过模组层次结构到某个项目的路径的上下文中出现的符号。

<a name="table_B-3"></a>
**表 B-3**：路径相关的语法

| 符号 | 说明 |
| :--- | :--- |
| `ident::ident` | 命名空间路径 |
| `::path` | 相对于代码箱根的路径（即显式的绝对路径） |
| `self::path` | 相对于当前模组的路径（即显式的相对路径） |
| `super::path` | 相对于当前模组的父模组的路径 |
| `type::ident`, `<type as trait>::ident` | 关联的常量、函数及类型 |
| `<type>::...` | 无法直接命名的类型的关联项目（例如，`<&T>::...`, `<[T]>::...` 等） |
| `trait::method(...)` | 通过命名定义方法调用的特质来消除方法调用的歧义 |
| `type::method(...)` | 通过命名定义方法调用的类型来消除方法调用的歧义 |
| `<type as trait>::method(...)` | 通过命名特质和类型来消除方法调用的歧义 |

下表 B-4 展示了在使用泛型类型参数的上下文出现的符号。

<a name="table_B-4"></a>
**表 B-4**：泛型

| 符号 | 说明 |
| :-- | :-- |
| `path<...>` | 指定类型中泛型类型的参数（例如，`Vec<u8>`） |
| `path::<...>`, `method::<...>` | 指定表达式中泛型类型、函数或方法的参数；通常成为 *涡轮鱼语法，turbofish*（例如，`"42".parse::<i32>()`，关于 Rust 的 turbofish 语法，请参考：[What is Rust's turbofish](https://techblog.tonsser.com/posts/what-is-rusts-turbofish)，[RUST 中的 turbofish 语法（一）](https://www.jianshu.com/p/9107685ece03) ） |
| `fn ident<...> ...` | 定义泛型的函数 |
| `struct ident<...> ...` | 定义泛型的结构体 |
| `enum ident<...> ...` | 定义泛型的枚举 |
| `impl<...> ...` | 定义泛型的实现 |
| `for<...> type` | 更高阶的生命周期边界，higher-ranked lifetime bounds |
| `type<ident=type>` | 一种泛型类型，其中一个或多个关联类型有着特定的赋值（例如，`Iterator<Item=T>`） |

下表 B-5 展示了在通过特质边界约束泛型类型参数的上下文出现的符号。

<a name="table_B-5"></a>
**B-5**：特质边界约束

| 符号 | 说明 |
| :--- | :--- |
| `T: U` | 泛型参数 `T` 被约束为实现 `U` 的类型 |
| `T: 'a` | 泛型类型 `T` 的生命周期必须长于 `'a`（意味着该类型不能间接包含任何生命周期短于 `'a` 的引用） |
| `T: 'static` | 除 `'static` 的引用外，泛型类型 `T` 不能包含任何借用的引用 |
| `'b: 'a` | 泛型生命周期 `'b` 必须比 `'a` 的生命周期更长 |
| `T: ?Sized` | 允许泛型类型参数为动态大小的类型 |
| `'a + trait`, `trait + trait` | 复合类型约束 |

下表 B-6 展示了在调用或定义宏，以及对项目指定属性的上下文中出现的符号。

<a name="table_B-6"></a>
**B-6**：宏与属性

| 符号 | 说明 |
| :--- | :--- |
| `#[meta]` | 外层属性 |
| `#![meta]` | 内层熟悉 |
| `$ident` | 宏代换 |
| `$ident:kind` | 宏的元变量 |
| `$(...) ...` | 宏的重复 |
| `ident! (...)`, `ident! {...}`, `ident! [...]` | 宏调用 |

下表 B-7 展示了创建注释的符号。

<a name="table_B-7"></a>
**表 B-7**：注释

| 符号 | 说明 |
| :--- | :--- |
| `//` | 行注释 |
| `//!` | 内层行文档注释 |
| `///` | 外层行文档注释 |
| `/*...*/` | 注释块 |
| `/*!...*/` | 内层块文档注释 |
| `/**...*/` | 外层块文档注释 |


下表 B-8 展示使用括号的上下文。

<a name="table_B-8"></a>
**表 B-8**：括号

| 符号 | 说明 |
| :--- | :--- |
| `()` | 空元组（又叫单元值），包括字面值和类型 |
| `(expr)` | 带括号的表达式 |
| `(expr,)` | 单个元素的元组表达式 |
| `(type,)` | 单个元素的元组类型 |
| `(expr, ...)` | 元组表达式 |
| `(type, ...)` | 元组类型 |
| `expr(expr, ...)` | 函数调用表达式；还用于初始化元组的 `struct` 以及元组的 `enum` 变种 |


下表 B-9 展示了使用花括号的上下文。

<a name="table_B-9"></a>
**表 B-9**：花括号

| 符号 | 说明 |
| :--- | :--- |
| `{...}` | 代码块表达式 |
| `Type {...}` | 结构体字面值 |

下表 B-10 展示了使用方括号的上下文。

<a name="table_B-10"></a>
**表 B-10**：方括号

| 符号 | 说明 |
| :--- | :--- |
| `[...]` | 数组字面值 |
| `[expr; len]` | 包含 `len` 个 `expr` 副本的数组字面值 |
| `[type; len]` | 包含 `len` 个 `type` 实例的数组字面值 |
| `expr[expr]` | 集合索引。可重载 `(Index, IndexMut)` |
| `expr[..]`, `expr[a..]`, `expr[..b]`, `expr[a..b]` | 伪装成集合切片的集合索引，使用 `Range`、`RangeFrom`、`RangeTo` 或 `RangeFull` 作为 “索引” |


