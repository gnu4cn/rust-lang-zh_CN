# 使用生命周期验证引用

**Validating References with Lifetimes**


生命周期是另一种咱们前面已经用到的泛型。与确保类型有着期望行为的特质不同，生命周期确保的是引用在咱们需要他们有效期间，保持有效，lifetimes ensure that references are valid as long as we need them to be。

在第 4 章中 [引用与借用](Ch04_Understanding_Ownership.md#引用与借用references-and-borrowing) 小节，咱们未曾讨论的一个细节，即 Rust 中的每个引用，都有着 *生命周期，lifetime*，其便是引用有效的作用范围。多数时候，声明周期是隐式而被推导出来的，这正与多数时候类型是被推导出来的一样。咱们只须在可能有多个类型时注解类型。与此类似，在一些引用的生命周期，可能以几种方式存在关联时，咱们就必须注解出生命周期。为确保在运行时用到的具体类型显著有效，Rust 就会要求咱们使用泛型生命周期参数，注解出这些关系，in a similar way, we must annotate lifetimes when the lifetimes of references could be related in a few different ways. Rust requires us to annotate the relationships using generic lifetime parameters to ensure the actual references used at runtime will definitely be valid。

绝大多数别的编程语言，甚至都没有注解周期，annotating lifetimes, 这个概念，因此这会让人感到陌生。尽管在这一章中咱们不会涵盖生命周期的全部，咱们仍将讨论咱们可能遇到生命周期语法的一些常见方式，如此咱们就能适应这个概念。


## 使用生命周期，防止悬空引用

**Preventing Dangling References with Lifetimes**


生命周期的主要目的是防止 *悬空引用，dangling references*，其会导致程序引用并非其打算引用的数据。设想下面清单 10-16 中的程序，其有着一个外层作用范围与一个内层作用范围。


```rust
fn main() {
    let r;

    {
        let x = 5;
        r = &x;
    }

    println! {"r: {}", r};
}
```

*清单 10-16：使用了其值已超出作用域引用的尝试*

> 注意：清单 10-16、10-17 及 10-23 中的示例，均在不带变量初始值下，声明出了一些变量，而变量名存在于外层作用域中。乍一看，这样做似乎与 Rust 的无空值，Rust's having no `null` values，特性相抵触。不过，当咱们尝试于赋予变量值之前，使用某个变量，就会得到一个编译器时报错，这就表示 Rust 实际上是不允许空值的。


那个外层作用域声明了个名为 `r`、不带初始值的变量，而其中的内层作用域声明了个名为 `x`，带有初始值 `5` 的变量。在内层作用域里，咱们尝试将 `r` 的值，设置为到 `x` 的引用。随后那个内层作用域便结束了，同时咱们尝试打印 `r` 中的值。由于其中 `r` 所指向的值，在咱们尝试使用前已超出作用域，因此此代码不会编译。下面是错误消息：

```console
$ cargo run                                            lennyp@vm-manjaro
   Compiling lifetimes_demo v0.1.0 (/home/lennyp/rust-lang/lifetimes_demo)
error[E0597]: `x` does not live long enough
 --> src/main.rs:6:13
  |
6 |         r = &x;
  |             ^^ borrowed value does not live long enough
7 |     }
  |     - `x` dropped here while still borrowed
8 |
9 |     println! {"r: {}", r};
  |                        - borrow later used here

For more information about this error, try `rustc --explain E0597`.
error: could not compile `lifetimes_demo` due to previous error
```

变量 `x` 未 “存活足够长时间。” 原因是当内层作用域在第 7 行结束时，变量 `x` 将超出作用域。然而变量 `r` 对外层作用域仍有效；由于其作用域更大，咱们就说变量其 “存活得更长”。若 Rust 允许此代码工作，变量 `r` 就会引用变量 `x` 超出作用域时，已被解除分配的内存，且咱们尝试对变量 `x` 的任何操作，都将不会正确工作。那么 Rust 是怎样确定出此代码无效的呢？他使用了借用检查器，a borrow checker。


## 借用检查器

**The Borrow Checker**


Rust 编译器有着对作用域加以比较，而确定出全部借用是否有效的 *借用检查器，a borrow checker*。下面清单 10-17 给出了与清单 10-16 相同，而带有展示变量生命周期注解的代码。

```rust
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+
```

*清单 10-17：变量 `r` 与 `x` 生命周期的注解，各自取名为 `'a` 与 `'b`*

这里，咱们以 `'a` 注解出了 `r` 的声明周期，与 `'b` 注解出 `x` 的生命周期。正如咱们所能看到的，相比外层 `'a` 声明周期代码块，那个内层 `'b` 代码块要小得多。在编译时，Rust 会比较这两个生命周期的大小，而发现变量 `r` 有着 `'a` 的生命周期，但他却指向了个 `'b` 的生命周期。由于生命周期 `'b` 比 `'a` 要短，于是该程序就被拒绝：引用物，the subject of the reference，没有存活到引用那么长时间。

下面清单 10-18 修复了该代码，从而其就没有了悬空引用，并会不带任何错误地编译。


```rust
fn main() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}                         // ----------+
```

*清单 10-18：由于被数据有着长于引用的生命周期，因此这是一个有效的引用*


这里，`x` 有着生命周期 `'b`，在此示例中其是大于 `'a` 的。由于 Rust 清楚在变量 `r` 中的引用，在变量 `x` 有效期间将始终有效，这就意味着 `r` 可引用 `x`。

既然咱们清楚了引用的生命周期在何处，以及 Rust 怎样为确保引用始终有效，而分析生命周期，那么下面咱们就要探讨函数上下文中，参数与返回值的泛型生命周期了，generic lifetimes of parameters and return values in the context of functions。


### 函数中的泛型生命周期

**Generic Lifetimes in Functions**


咱们将编写一个返回两个字符串切片中较长者的函数。该函数将取两个字符串切片，并返回单个字符串切片。当咱们实现了 `longest` 函数后，下面清单 10-19 中的代码应打印 `最长的字符串为 abcd`。


文件名：`src/main.rs`


```rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println! ("最长的字符串为 {}", result);
}
```

*清单 10-19：调用 `longest` 函数来找出两个字符串切片中较长那个的 `main` 函数*


请注意由于咱们不想要这个 `longest` 函数，取得其参数的所有权，因此咱们是要该函数取两个均为引用的字符串切片，而非字符串。请参考第 4 章中 [作为函数参数的字符串切片](Ch04_Understanding_Ownership.md#字符串切片作为函数参数) 小节，了解更多为何咱们在清单 10-19 中用到的参数，即为咱们所想要参数的讨论。

当咱们如下面清单 10-20 中所示的那样，尝试实现 `longest` 函数时，其不会编译。


文件名：`src/main.rs`

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else { y }
}
```

*清单 10-20：返回两个字符串切片中较长者 `longest` 函数实现，但上不会编译*


咱们而是会得到以下谈及生命周期的错误：


```console
$ cargo run                                                                                  lennyp@vm-manjaro
   Compiling lifetimes_demo v0.1.0 (/home/lennyp/rust-lang/lifetimes_demo)
error[E0106]: missing lifetime specifier
 --> src/main.rs:1:33
  |
1 | fn longest(x: &str, y: &str) -> &str {
  |               ----     ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
help: consider introducing a named lifetime parameter
  |
1 | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  |           ++++     ++          ++          ++

For more information about this error, try `rustc --explain E0106`.
error: could not compile `lifetimes_demo` due to previous error
```

帮助文本揭示了由于 Rust 无法区分出正返回的引用，是指向 `x` 还是 `y`，因此返回值类型就需要其上的一个泛型生命周期参数，a generic lifetime parameter。事实上，由于在该函数的函数体中，`if` 代码块返回的是到参数 `x` 的引用，而 `else` 代码块返回的则是到 `y` 的引用，所以就连咱们也不清楚！

在咱们定义这个函数时，是不清楚将传入到该函数的那些具体值的，因此就不清楚究竟是`if` 情形，还是 `else` 情形会被执行。咱们也不清楚将传入引用的具体生命周期，进而就无法查看如清单 10-17 及 10-18 中所看到的作用域，以确定出返回的引用是否始终有效。由于借用检查器不清楚 `x` 与 `y` 的生命周期，与返回值的生命周期有怎样的关联，因此借用检查器也无法确定出这一点。要修复这个错误，咱们将添加定义出这些引用变量之间关系的泛型生命周期参数，进而借用检查器就可以完成他的分析。


## 生命周期注解语法

**Lifetime Annotation Syntax**

生命周期注解，不会改变任何引用的存活时长。而是，他们在不影响生命周期下，对多个引用变量的生命周期关系加以描述。正如函数签名指定了泛型参数时，函数便可接受任意类型一样，通过指定出泛型生命周期参数，函数就可以接受带有任意生命周期的引用了，just as functions can accept any type when the signatures specifies a generic type parameter, functions can accept with any lifetime by specifying a generic lifetime parameter。

生命周期注解有着些许不寻常的语法：生命周期参数名字，必须以撇号（单引号，`'`）开头，通常为全部小写字母，且像泛型一样非常短。多数人会用 `'a` 作为首个生命周期注解。咱们会将生命周期注解，放在引用的 `&` 之后，使用一个空格来将这种注解与该引用的类型分隔开。

下面是一些示例：到某个 `i32` 的不带生命周期参数的引用、到某个 `i32` 的有着名为 `'a` 的生命周期参数，以及到某个 `i32` 的同样有着生命周期 `'a` 的可变引用。

```rust
&i32        // 某个引用
&'a i32     // 某个带有显式生命周期的引用
&'a mut i32 // 某个有着显式生命周期的可变引用
```

由于注解的目的是告诉 Rust （编译器），多个引用的泛型生命周期参数相互之间如何相互关联，因此生命周期本身并没有什么意义。接下来咱们就要在那个 `largest` 函数上下文中，检视一下生命周期注解如何关联。


### 函数签名中的生命周期注解

**Lifetime Annotations in Function Signatures**


如同之前对通用 *类型，type* 参数所做的那样，要在函数签名中使用生命周期注解，咱们需在函数名字与参数清单间，于一对尖括号里，声明出通用 *生命周期，lifetime* 参数。

咱们是要那个函数签名表达出以下约束：返回的引用将与两个参数保持同样长的有效时间。这便是参数与返回值生命周期之间的关系。咱们将把这个生命周期命名为 `'a`，并在随后将其添加到各个引用，如下清单 10-21 中所示。

文件名：`src/main.rs`

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

*清单 10-21：指明签名中全部引用，都必须有着同一生命周期 `'a` 的 `longest` 函数定义*


此代码应会编译，并在清单 10-19 的 `main` 函数中使用他时，产生出咱们想要的结果。

这个函数签名现在告诉 Rust，针对某个生命周期 `'a`，该函数会取两个参数，他们都是存活时间至少为 `'a` 的字符串切片。该函数签名还告诉 Rust，从该函数返回的字符串切片，将存活至少生命周期 `'a` 那样长时间。实际上，这表示 `longest` 函数所返回引用的生命周期，与该函数参数引用值生命周期中较小的一致。这些关系，就是咱们想要 Rust 在分析此代码时，要用到的关系。

请记住，当咱们在这个函数签名中，指明那些生命周期进行时，咱们并未改变任何传入或返回值的生命周期。相反，咱们指明的是借用检查器应拒绝没有遵守这些约束的所有值。请注意 `longest` 函数不需要确切地掌握，`x` 与 `y` 将存活多久，而只要有可替代 `'a` 的某个作用域将满足此签名，note that the `longest` function doesn't need to know exactly how long `x` and `y` will live, only that some scope can be substituted for `'a` that will satisfy this signature。

当于函数中注解生命周期时，这些注解是在函数签名中，而非函数体中。生命周期注解，成为了该函数合约的一部分，这就很像是签名中的类型。令函数签名包含生命周期合约，the lifetime contract，就意味着 Rust 编译器执行的分析，会更简单。若函数被注解方式或被调用方式存在问题，那么编译器报错，就可以更精准地指向所编写代码或约束的某个部分。相反，若没有这些生命周期注解，那么相比于 Rust 编译器会作出更多有关咱们所预期的生命周期关系推断，编译器或许就只能够指出，在问题原因处许多步之外，咱们代码的某个使用，if, instead, the Rust compiler made more inferences about what we intended the relationships of the lifetimes to be, the compiler might only be able to point to a use of our code many steps away from the cause of the problem。

在咱们把具体引用传递给 `longest` 时，取代 `'a` 的具体生命周期的，便是 `x` 的作用域中，与 `y` 的作用域重叠的部分。也就是说，泛型生命周期 `'a` 将获得，与 `x` 与 `y` 的生命周期中较小者相等的具体生命周期。由于咱们已使用同一生命周期参数 `'a`，注解了返回的引用，因此返回的引用，就会在 `x` 与 `y` 的生命周期中，较小者的存活时长期间有效。

下面咱们来通过传入具有不同具体生命周期的引用，看一下生命周期注解，如何限制 `longest` 函数。下面清单 10-22 就是一个直观的示例。


文件名：`src/main.rs`

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("长字符串就是长");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println! ("最长的字符串为 {}", result);
    }
}
```

*清单 10-22：以到具有不同具体生命周期的 `String` 类型值的一些引用，使用 `longest` 函数*


在此示例中，`string1` 到外层作用域结束之前都有效，`string2` 到内层作用域结束之前有效，而 `result` 引用了在内层作用域结束之前有效的某个东西。运行此代码，咱们就会看到借用检查器予以了证实；此代码将编译并打印 `最长的字符串为 长字符串就是长`。

接下来，就要尝试一个展示 `result` 中引用的生命周期，必须为这两个参数生命周期中较小的那个的示例。这里将把那个 `result` 变量的声明，移到内层作用域外面而将到该 `result` 变量的赋值，仍然留在有着 `string2` 变量的作用域里头。随后将把那个用到 `result` 变量的 `println!` 语句，移出到内层作用域外面，在内层作用域结束之后。下面清单 10-23 中的代码将不会编译。


文件名：`src/main.rs`


```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("长字符串就是长");
    let result;

    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println! ("最长的字符串为 {}", result);
}
```

*清单 10-23：尝试在 `string2` 已超出作用域后使用 `result`*


在尝试编译此代码时，咱们会得到以下报错：


```console
$ cargo run                                                                        lennyp@vm-manjaro
   Compiling lifetimes_demo v0.1.0 (/home/lennyp/rust-lang/lifetimes_demo)
error[E0597]: `string2` does not live long enough
  --> src/main.rs:15:44
   |
15 |         result = longest(string1.as_str(), string2.as_str());
   |                                            ^^^^^^^^^^^^^^^^ borrowed value does not live long enough
16 |     }
   |     - `string2` dropped here while still borrowed
17 |     println! ("最长的字符串为 {}", result);
   |                                    ------ borrow later used here

For more information about this error, try `rustc --explain E0597`.
error: could not compile `lifetimes_demo` due to previous error
```

报错显示，要让 `result` 对那个 `println!` 语句有效，`string2` 将需要在外层作用域结束前一直有效。Rust （编译器）之所以清楚这点，是因为咱们使用同一生命周期参数 `'a`，注解了该函数的参数与返回值。

而咱们而作为人类，则可以看一下这段代码，并发现 `string1` 要长于 `string2`，而由此 `result` 将包含到 `string1` 的引用。由于 `string1` 尚未超出作用域，那么到 `string1` 的某个引用，对于 `println!` 语句仍将有效。然而编译器在此情形下，却无法看出该引用是有效的。咱们已告知 Rust，由 `longest` 函数所返回引用的生命周期，与所传入参数声的明周期中较小者相同。因此，借用检查器就会因代码中可能有着无效的引用，而不容许清单 10-23 中代码。


请尝试设计更多在传入 `longest` 函数的值与引用生命周期，及返回引用使用方式上各不相同的试验。在咱们编译前，要就这些试验是否会通过借用检查器的检查，做出一些假定；随后检查发现，咱们所做出的假定是否正确！


### 从生命周期角度思考

**Thinking in Terms of Lifetimes**


咱们需要以何种方式，来指明生命周期参数，取决于咱们的函数正在做什么。比如若咱们把 `longest` 函数实现，修改为始终返回第一个参数，而非最长的字符串切片，咱们就不需要在参数 `y` 上指定生命周期。以下代码将会编译：

文件名：`src/main.rs`

```rust
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```


咱们已为参数 `x` 与返回值类型，指定了生命周期参数 `'a`，而由于参数 `y` 的生命周期，与 `x` 或返回值的生命周期并无任何关系，故咱们并未将 `'a` 指定给参数 `y`。

当从函数返回引用时，返回值类型的生命周期参数，就需要匹配某个参数的生命周期参数。而当返回的引用 *不* 指向某个参数时，其就必定会指向函数内部创建出的某个值。然而，由于该值在函数结束处将超出作用域，因此这就会是个悬空引用。请设想下面这个不会编译的 `longest` 函数尝试实现：

文件名：`src/main.rs`

```rust
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("真正长的字符串");
    result.as_str()
}
```

这里，尽管咱们已为返回类型指定了生命周期参数 `'a`，但由于返回值生命周期与参数的生命周期毫无关系，故这个实现将编译失败。下面是咱们会得到的报错：

```console
$ cargo run                                                              lennyp@vm-manjaro
   Compiling lifetimes_demo v0.1.0 (/home/lennyp/rust-lang/lifetimes_demo)
error[E0515]: cannot return reference to local variable `result`
  --> src/main.rs:11:5
   |
11 |     result.as_str()
   |     ^^^^^^^^^^^^^^^ returns a reference to data owned by the current function

For more information about this error, try `rustc --explain E0515`.
warning: `lifetimes_demo` (bin "lifetimes_demo") generated 2 warnings
error: could not compile `lifetimes_demo` due to previous error; 2 warnings emitted
```

问题在于，那个 `result` 会在 `longest` 函数结束处超出作用域而被清理掉。而咱们还在尝试返回到该函数中 `result` 的引用。咱们没有办法指定出会纠正这个悬空引用的生命周期参数，而 Rust 也不会容许咱们创建出悬空引用。在这种情况下，最佳修复将是返回有着所有权的数据类型，而非某个引用（注：这样看来引用是没有所有权的），从而随后由调用函数，the calling function，负责清理该值。

最终，生命周期语法是关于把函数的不同参数与返回值的生命周期联系起来的。一旦他们联系起来，那么 Rust 就有了足够信息，来实现涉及内存安全的操作，并拦下会创建出悬空指针或危及内存安全的操作。


### 结构体定义中的生命周期注解

**Lifetime Annotations in Struct Definitions**


到目前为止，咱们曾定义的结构体，都保存着一些自有类型。咱们可定义出保存引用的结构体，但那样的话，咱们将需要在结构体定义中的每个引用上，添加生命周期注解。下面清单 10-24 有个名为 `ImportedExcerpt`，保存着一个字符串切片的结构体。

文件名：`src/main.rs`

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("请叫我伊萨梅尔。多年以前.....");
    let first_sentence = novel.split('。').next().expect("找不到一个 '。'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
```

*清单 10-24：保存着一个引用的结构体，因此就需要生命周期注解*

此结构体拥有保存着是个引用的字符串切片的单一字段 `part`。与通用数据类型（泛型），generic data types，下一样，咱们在结构他名字后的尖括号里，声明了通用声明周期参数，进而就可以在结构体定义代码体中，使用那个生命周期参数。这个注解表示，`ImportantExcerpt` 的实例，无法存活超过其在 `part` 字段中所保存的那个引用，this annotation means an instance of `ImportedExcerpt` can't outlive the reference it holds in its `part` field。

这里的 `main` 函数会创建出 `ImportantExcerpt` 结构体的，保存着到由变量 `novel` 拥有的 `String` 的第一个句子引用的一个示例。`novel` 中的数据在这个 `ImportantExcerpt` 实例被创建之前就存在了。此外，在这个 `ImportantExcerpt` 超出作用域之前，`novel` 不会超出作用域，因此这个 `ImportantExcerpt` 实例中的引用是有效的。


### 生命周期的省略

**Lifetime Elision**


咱们已经了解到每个引用都有生命周期，以及需要给使用了引用的函数与结构体，指明生命周期参数。不过，在第 4 章中的清单 4-9 中，咱们曾有一个不带生命周期注解也被编译了的函数，在下面清单 10-25 中再次予以展示。


文件名：`src/main.rs`

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
```

*清单 10-25：咱们曾在清单 4-9 中定义的一个即使其中的参数与返回值均为引用变量，而不带生命周期注解还仍编译了的函数*


这个函数不带生命周期注解仍会编译的原因，是历史遗留的：在 Rust 早期版本（`pre-1.0`）中，由于每个引用都需要显式生命周期，因此该代码就不会编译。那个时候，该函数签名会被写成下面这样：

```rust
fn first_word<'a>(s: &'a str) -> &'a str {
```

在编写许多 Rust 代码后，Rust 团队发现，Rust 程序员们在某些特定情形下，会一次又一次地敲入许多同样的生命周期注解。而这些特定情形，是可被预测的，并遵循了少数几种确定性模式，a few deterministic patterns。Rust 开发者们便把这些模式，编程进了编译器的代码，于是借用检查器就可以推断出这些情形下的生命周期，而无需显式的注解。

由于存在合并更多确定性模式，并将他们到编译器的可能，因此讲 Rust 的这段历史是有必要的。今后，或许就只要求更少甚至没有生命周期注解。

编程到 Rust 引用分析中的那些确定性模式，被称为 *生命周期省略规则，lifetime elision rules*。这些规则并非 Rust 程序员要遵循的；他们是编译器将考虑的一套特殊情形，并在咱们的代码符合这些情形时，咱们就不需要显式地写出生命周期。

这些省略规则并不提供完全的推断。在 Rust 明确地应用了这些规则，但仍存在引用具有什么样的生命周期方面的模糊性时，编译器将不会就其余引用变量应有什么样的生命周期，加以猜测。相比于猜测，编译器将给到咱们，可通过添加生命周期注解而解决的一个报错。

函数或方法参数上的生命周期，被称为 *输入生命周期，input lifetimes*，而在返回值上的生命周期，则被称为 *输出生命周期，output lifetimes*。

在没有显式注解时，编译器会运用三条规则，来计算出引用的生命周期。首条规则适用于输入生命周期，而第二及第三条规则，则都适用于输出生命周期。若编译器到了这三条规则的结束处，仍有其未能计算出生命周期的引用，那么编译器就会以报错而停止。这三条规则适用于 `fn` 定义及 `impl` 代码块都适用。

首条规则即为，编译器会指派生命周期参数给是引用的各个参数。也就是说，有着一个参数的函数，会获得一个生命周期参数：`fn foo<'a>(x: &'a i32)`；而有着两个参数的函数，就会得到两个单独生命周期参数：`fn foo<'a, 'b>(x: &'a i32, &'b i32)`；如此等等。

第二条规则，则是当确切地只有一个输入生命周期参数时，那个生命周期，就被指派给全部输出生命周期参数：`fn foo<'a>(x: &'a i32) -> &'a i32`。

第三条规则，当有多个输入生命周期参数，而由于这是个方法（这条规则是针对结构体上的方法），其中之一为 `&self` 或 `&mut self` 时，那么 `self` 的生命周期，便会被指派给全部输出生命周期参数。由于只有较少必要符号，因此这第三条规则，就会令到方法阅读与编写起来体验更佳。

下面咱们就来充当编译器。咱们将应用这些规则，来计算出清单 10-25 中，`first_word` 函数签名里各个引用的生命周期。函数签名以不带与其中引用关联的任何生命周期开始：

```rust
fn first_word(s: &str) -> &str {
```

随后编译器便应用首条规则，其指出了各个参数会获取到各自的生命周期。与平常一样，咱们将把该生命周期叫做 `'a`，那么现在函数签名就是这样的：

```rust
fn first_word<'a>(s: &'a str) -> &str {
```

由于这里只有一个输入生命周期，因此第二条规则便会适用。第二条规则指出，一个的输入参数生命周期，会被指派给输出生命周期，那么现在这个函数签名就是这样的：

```rust
fn first_word<'a>(s: &'a str) -> &'a str {
```

现在这个函数签名中的全部引用，都有了生命周期，进而编译器就可以在无需程序员注解函数签名中的生命周期的情况下，继续其分析了。

接下来就要看看另一个示例，这次要使用清单 10-20 中，一开始编写时没有生命周期参数的那个 `longest` 函数：

```rust
fn longest(x: &str, y: &str) -> &str {
```

首先来应用第一条规则：各个参数都得到自己的生命周期。这次不是一个而是两个参数，那么这里就有两个生命周期：

```rust
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
```

咱们可以看出，由于有多于一个的输入生命周期，因此第二条规则便不适用。因为 `longest` 是个函数而非方法，参数中没有一个是 `self`，因此第三条规则也不适用。在历经全部三条规则后，咱们仍未计算出返回值类型的生命周期为何。这就是咱们在尝试编译清单 10-20 中代码时，收到错误的原因：编译器历经这些生命周期省略规则，而仍未计算出函数签名中引用的全部生命周期。

由于第三条规则实际上只适用于方法签名，咱们接下来就要看看在方法上下文中的生命周期，以发现为何第三条规则，就意味着咱们不必经常注解方法签名中的生命周期。


### 方法定义中的生命周期注解

**Lifetime Annotations in Method Definitions**


当咱们在结构体上实现带有生命周期的方法时，咱们会使用与清单 10-11 中所展示的泛型参数同样语法。其中咱们会根据其是否与结构体字段，或方法参数及返回值相关，而声明出并用到生命周期参数。

由于结构体字段的生命周期是结构体类型的一部分，因此他们总是需要声明在 `impl` 关键字之后，且随后会在结构体名字之后用到。

在 `impl` 代码块里的方法签名中，一些引用可能与结构体字段中的引用生命周期相关，也可能他们是独立的。此外，生命周期省略规则通常会发挥作用，从而在方法签名中，生命周期注解就不是必要的。咱们来看看一些使用咱们曾于清单 10-24 中定义的名为 `ImportantExcerpt` 结构体的示例。

首先，咱们将使用一个名为 `level` 的方法，其唯一参数是个到 `self` 引用，且返回值为非到任何东西引用的 `i32`：

```rust
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
```


`impl` 后的生命周期参数声明，与类型名字后其使用，都是必须的，但由于第一条省略规则的原因，咱们未被要求注解其中到 `self` 引用的生命周期。

下面是个其中第三条生命周期省略规则适用的示例：

```rust
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println! ("请注意：{}", announcement);
        self.part
    }
}
```

这里有两个输入生命周期，那么 Rust 就会适用首条生命周期省略规则，而赋予 `&self` 与 `announcement` 其各自的生命周期。随后，由于其中一个参数是 `&self`，那么返回值类型就会得到 `&self` 的生命周期，进而全部生命周期都得到了计算。


### 静态生命周期

**The Static Lifetime**


咱们需要讨论的一种特殊生命周期是 `'static`，其表示受其影响的引用，*可以* 在程序整个持续时间内存活。所有字符串字面值，都有着 `'static` 的生命周期，咱们可将其注解为下面这样：

```rust
let s: &'static str = "我有静态的生命周期。";
```

此字符串的文本，被直接存储在该程序二进制数据中，而这是一直可用的。由此，所有字符串字面值的生命周期便是 `'static`。

在一些错误消息中，咱们或许会看到使用 `'static` 生命周期的建议。不过在给引用指定 `'static` 生命周期之前，请考虑一下手头的这个引用，是否会存活到整个程序的生命周期，以及咱们是否想要他存活到整个程序的生命周期。多数时候，建议 `'static` 生命周期的错误消息，都是由尝试创建悬空引用，或可用生命周期不匹配导致。在这些情况下，解决办法是修复这些问题，而非指定出 `'static` 生命周期。


## 泛型参数、特质边界与生命周期三位一体

**Generic Type Parameters, Trait Bounds, and Lifetimes Together**


咱们来简要地看看，在一个函数中，一起指定出泛型参数、特质边界与生命周期的语法！

```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println! ("通知！{}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

这便是清单 10-21 中，返回两个字符串切片中较长者的 `longest` 函数。不过现在他有了个泛型 `T` 名为 `ann` 的额外参数，泛型 `T` 可以实现了由 `where` 子句所指定的 `Display` 特质的任何类型填入。这个额外参数，将被使用 `{}` 打印出来，这便是为何 `Display` 特质为必要的原因。由于生命周期是泛型的一种，因此其中的生命周期参数 `'a` 与泛型参数 `T`，便处于函数名称后尖括号内的同一清单里。


# 本章小结

在这一章中，咱们谈到了很多东西！现在咱们清楚了泛型参数、特质与特质边界，与泛型生命周期参数，那么就准备好编写在许多不同情况下工作，不带有重复的代码了。泛型参数实现了将代码应用于不同类型。特质与特质边界确保即使类型是通用的，他们仍将有着代码所需的行为。咱们了解了如何使用生命周期注解，来保证这种灵活代码不会有任何悬空引用。且所有分析，都发生在编译时，其不会影响到运行时性能！

不论相信与否，关于咱们在本章讨论的这些话题，要掌握的东西远不止这些：第 17 章会讨论特质对象，trait objects，其是运用特质的另一种方式。还有更多咱们只会在极复杂场景下，才需要的涉及到更复杂场景的生命周期注解；要了解那些生命周期注解，咱们应阅读 [Rust 指南](https://doc.rust-lang.org/reference/index.html)。不过接下来，咱们将了解怎样编写 Rust 中的测试，从而就可以确保所咱们的代码，以其应有的方式工作。


（End）


