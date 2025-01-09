# 改进咱们的 I/O 项目

**Improving Our I/O Project**


有了迭代器方面的新知识，咱们便可通过使用迭代器，改进第 12 章中的 I/O 项目，令到代码各处更清楚与简练。咱们来看看，迭代器可怎样改进其中 `Config::build` 与 `search` 函数的实现。


## 使用 `Iterator` 消除 `clone`

**Removing a `clone` Using an `Iterator`**


在清单 12-6 中，我们添加了一些代码，这些代码获取了一个 `String` 值的切片，并通过索引到该切片并克隆这些值，来创建一个 `Config` 体的实例，使 `Config` 结构体拥有这些值。下面清单 13-17 中，咱们重现了清单 12-23 中 `Config::build` 函数的实现：

文件名：`src/lib.rs`

```rust
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("参数数量不足");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
```

*清单 13-17：清单 12-23 中 `Config::build` 函数的重现*

当时，我们说不用担心低效的克隆调用，因为将来咱们会移除它们。好吧，现在是时候了！

咱们这里之所以需要 `clone` 方法，是由于在参数 `args` 中，咱们有一个 `String` 元素构成的切片，而 `build` 函数并不拥有 `args`。为返回 `Config` 实例的所有权，咱们不得不克隆 `Config` 结构体的 `query` 与 `filename` 字段，进而 `Config` 实例便可拥有他的值。

利用我们对迭代器的新知识，我们可以改变构建函数，使其拥有一个迭代器作为其参数，而不是借用一个切片。我们将使用迭代器的功能，而不是检查切片的长度并对特定位置进行索引的代码。这将明确 `Config::build` 函数正在做什么，因为迭代器将访问这些值。

一旦 `Config::build` 取得迭代器的所有权，而不再使用借用的索引操作，咱们就可以将 `String` 值从迭代器迁移到 `Config` 中，而不是调用 `clone` 方法并构造新的内存分配。


### 直接使用返回的迭代器

**Using the Returned Iterator Directly**


请打开咱们 I/O 项目的 `src/main.rs` 文件，其看起来应是这样的：

文件名：`src/main.rs`


```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln! ("解析参数时遇到问题：{err}");
        process::exit(1);
    });

    // --跳过代码--
}
```

咱们将首先把咱们在清单 12-24 中，有着的 `main` 函数开头，修改为下面清单 13-18 中，使用迭代器的代码。在咱们一并更新 `Config::build` 前，这不会编译。

文件名：`src/main.rs`

```rust
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln! ("解析参数时遇到问题：{err}");
        process::exit(1);
    });

    // --跳过代码--
}
```

*清单 13-18：把 `env::args` 的返回值传递给 `Config::build`*

`env::args` 函数会返回一个迭代器！相比于把迭代器值收集到矢量值，并随后把一个切片传递给 `Config::build`，现在咱们是在直接把由 `env::args` 返回的迭代器所有权，传递给 `Config::build`。

接下来，咱们需要更新 `Config::build` 的定义。在咱们 I/O 项目的 `src/data_structures.rs` 文件中，咱们就要像下面清单 13-19 中那样，修改 `Config::build` 的函数签名。由于咱们需要更新该函数的主体体，因此这仍不会编译。

文件名：`src/lib.rs`

```rust
impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        // --跳过代码--
```

*清单 13-19：将 `Config::build` 的函数签名，更新为期待得到一个迭代器*

`env::args` 函数的标准库文档显示，其返回的迭代器类型为 `std::env::Args`，且那种类型实现了 `Iterator` 特质，并会返回 `String` 值。

咱们已更新了 `Config::build` 函数的签名，那么参数 `args` 就会有一个有着特质边界 `impl Iterator<Item = String>` 的泛型，而不再是 `&[String]` 类型。咱们曾在第 10 章 [作为参数的特质](Ch10_Generic_Types_Traits_and_Lifetimes.md#作为参数的特质) 小节中，讨论过的 `impl Trait` 语法用法，表明 `args` 可以是任何实现了 `Iterator` 类型，且返回 `String` 条目的类型。

由于咱们正取得 `args` 的所有权，且咱们将通过对其迭代而修改 `args`，咱们便可把 `mut` 关键字，添加到 `args` 参数的说明中，以将其构造为可变。


### 使用 `Iterator` 特质的方法而非索引

**Using `Iterator` Trait Methods Instead of Indexing**


接下来，咱们将修正 `Config::build` 函数的主体。由于 `args` 实现了 `Iterator` 特质，咱们便清楚咱们可以调用他上面 `next` 方法！下面清单 13-20 将清单 12-23 中的代码，更新为了使用 `next` 方法：

文件名：`src/data_structures.rs`

```rust
impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("未曾获取到查询字串"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("未曾获取到文件路径"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
```

*清单 13-20：将 `Config::build` 函数的主体，修改为使用迭代器方法*

请记住 `env::args` 返回值中的第一个值，是程序的名字。咱们是要忽略那个值，而到下一值处，所以咱们首先调用 `next`，并对返回值不做任何处理。其次，咱们调用 `next` 来获取到咱们想要放入 `Config` 的 `query` 字段的值。若 `next` 返回一个 `Some`，咱们就使用 `match` 来提取该值。若其返回了 `None`，就意味着没有给出足够的参数，而咱们就及早地返回一个 `Err` 值。对于 `filename` 值，咱们进行了同样的处理。


## 使用迭代器适配器，令到代码更清晰

**Making Code Clearer with Iterator Adaptors**

咱们也可以在 I/O 项目的 `search` 函数中利用迭代器，取被转载于下面清单 13-21 中，如同其曾在清单 12-19 中那样：

文件名：`src/lib.rs`

```rust
pub fn search<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
```

*清单 13-21：清单 12-19 中 `search` 函数的实现*

咱们可使用迭代器适配器的方法，以更简练方式编写出此代码。这样做还可以让我们避免有一个可变的中间 `results` 矢量值。函数式编程风格，the functional programming style，倾向于最小化可变状态的数量以使代码更清晰。移除可变状态，就可能让令到搜索并行进行的今后功能增强可行，因为咱们将不必管理到 `results` 矢量的并发访问。下面清单 13-22 给出了这一修改：

文件名：`src/lib.rs`

```rust
pub fn search<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
```

*清单 13-22：在 `search` 函数实现中使用迭代器适配器方法*

回顾一下，`search` 函数的目的是要返回 `contents` 中包含 `query` 的所有行。与清单 13-16 中的 `filter` 示例类似，此代码使用 `filter` 适配器来只保留 `line.contains(query)` 返回 `true` 的行。咱们随后使用 `collect()`，把匹配行收集到另一矢量值中。这就简单多了！请随意做出同样的改变，在 `search_case_insensitive` 函数中使用迭代器方法。

> 函数 `search_case_insenstitive` 修改后如下所示：


```rust
pub fn search_insensitive<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}
```

### 在循环或迭代器之间做出选择

**Choosing Between Loops or Iterators**

下一个合乎逻辑的问题是，咱们应在自己的代码中选择哪种风格与为什么：清单 13-21 中原本的实现，或清单 13-22 中用到迭代器的版本。大多数 Rust 程序员喜欢使用迭代器风格。一开始他有点难掌握，但一旦咱们对各种迭代器适配器和他们的作用有了感觉，迭代器就会更容易理解。该代码没有拨弄循环的各个部分，与构建出新的矢量值，而是专注于循环的高级目标。这就把一些普通的代码抽象化了，所以更容易看到这段代码特有的概念，比如迭代器中每个元素必须通过的过滤条件。

但是，这两种实现方式真的等同吗？直观的假设可能是，更低级别的循环会更快。接下来咱们就会谈及性能问题。


（End）


