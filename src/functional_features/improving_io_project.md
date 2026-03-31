# 改进我们的 I/O 项目

借助这些关于迭代器的新知识，我们可以通过使用迭代器改进第 12 章中的 I/O 项目，使代码中的各处更清楚与更简洁。我们来看看迭代器可以怎样改进 `Config::build` 函数和 `search` 函数的实现。


## 使用迭代器消除 `clone`

在 [清单 12-6](../io_project/refactoring.md#listing_12-6) 中，我们添加了取一个 `String` 值的切片，进而通过索引该切片并克隆值创建了个 `Config` 结构体的实例，从而允许 `Config` 结构体拥有这些值。在下面清单 13-17 中，我们重现了 [清单 12-23](../io_project/env_variables.md#listing_12-23) 中 `Config::build` 函数的实现：

<a name="listing_13-17"></a>
文件名：`src/main.rs`

```rust
impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("参数不足");
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

**清单 13-17**：重现清单 12-23 中的 `Config::build` 函数

当时，我们说过不要担心低效的 `clone` 调用，因为将在今后移除他们。嗯，就是现在了！

我们在这里需要 `clone`，因为我们在参数 `args` 中有个有着 `String` 元素的切片，而 `build` 函数并不拥有 `args`。为了返回 `Config` 实例的所有权，我们必须克隆 `Config` 结构体 `query` 与 `file_path` 字段中的值，以便 `Config` 实例可以拥有他的值。

凭借我们对迭代器的新知识，我们可以修改 `build` 函数为取得作为其参数的迭代器的所有权，而不是借用切片。我们将使用迭代器功能，而不是检查切片长度，然后索引到特定位置的代码。这将明确 `Config::build` 函数正在执行的操作，因为迭代器将访问这些值。

一旦 `Config::build` 取得迭代器的所有权，而停止使用借用的索引操作，我们就可以迁移迭代器中的 `String` 值到 `Config` 中，而不是调用 `clone` 并构造新的内存分配。


### 直接使用返回的迭代器

请打开咱们 I/O 项目的 `src/main.rs` 文件，其看起来应是下面这样：

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

我们将首先修改 [清单 12-24](../io_project/std_err.md#listing_12-24) 中的 `main` 函数开头，为下面清单 13-18 中的代码，这次使用迭代器。在我们更新 `Config::build` 前，这不会编译。

<a name="listing_13-18"></a>
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

**清单 13-18**：传递 `env::args` 的返回值给 `Config::build`

`env::args` 函数返回的是个迭代器！逾期收集迭代器的值到一个矢量值，然后传递一个切片到 `Config::build`，现在我们传递自 `env::args` 返回的迭代器的所有权给 `Config::build`。

接下来，我们需要更新 `Config::build` 的定义。我们来修改 `Config::build` 的定义为清单 13-19 那样。这仍将不编译，因为我们需要更新函数主体。

<a name="listing_13-19"></a>
文件名：`src/main.rs`

```rust
impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        // --跳过代码--
```

**清单 13-19**：更新 `Config::build` 的函数签名为期望迭代器

`env::args` 函数的标准库文档显示，他返回的迭代器类型为 `std::env::Args`，而这种类型实现了 `Iterator` 特质并会返回 `String` 值。

我们已更新 `Config::build` 函数的签名，从而参数 `args` 有个带有特质边界 `impl Iterator<Item = String>` 的泛型类型，而不是有个 `&[String]` 类型。我们在第 10 章的 [将特质用作参数](../generic_types_traits_and_lifetimes/traits.md#将特质用作参数) 小节中讨论过 `impl Trait` 语法的这种用法，表明 `args` 可以是任何实现 `Iterator` 类型并返回 `String` 项目的类型。

由于我们正在取得 `args` 的所有权，并且将通过遍历他来修改 `args`，因此我们可以添加 `mut` 关键字到 `args` 参数的说明中以使其可变。


### 使用 `Iterator` 特质方法

接下来，我们将修正 `Config::build` 的主体。由于 `args` 实现了 `Iterator` 特质，我们知道我们可以对其调用 `next` 方法！下面清单 13-20 更新 [清单 12-23](../io_project/env_variables.md#listing_12-23) 中的代码为使用 `next` 方法：

<a name="listing_13-20"></a>
文件名：`src/main.rs`

```rust
impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("未获取到查询字串"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("未获取到文件路径"),
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

**清单 13-20**：修改 `Config::build` 的主体为使用迭代器方法

请记住，`env::args` 返回值中的第一个值是程序的名字。我们希望忽略该值并获取下一个值，所以首先我们调用 `next` 并对返回值不执行任何操作。然后，我们调用 `next` 来获取我们打算放入 `Config` 的 `query` 字段中的值。当 `next` 返回 `Some` 时，我们使用 `match` 来提取该值。当其返回 `None` 时，就意味着没有给出足够参数，我们提前以 `Err` 值返回。我们对 `file_path` 的值执行相同的操作。


## 使用迭代器适配器清理代码

我们还可以在我们 I/O 项目中的 `search` 函数中利用迭代器，其重现于下面清单 13-21 中，就像其在 [清单 12-19](../io_project/test_driven_dev.md#listing_12-19) 中那样：

<a name="listing_13-21"></a>
文件名：`src/lib.rs`

```rust
pub fn search<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
```

**清单 13-21**：清单 12-19 中 `search` 函数的实现

我们可以使用迭代器适配器方法，以更简洁的方式编写这段代码。这样做还让我们可以避免有个可变的中间 `results` 矢量值。函数式编程风格倾向于尽量减少可变状态，以使代码更加清晰。移除这一可变状态可能带来一项今后的功能增强，使检索并行地进行，因为我们将不必管理对 `results` 这一矢量值的并发访问。下面清单 13-22 展示了这一修改：

<a name="listing_13-22"></a>
文件名：`src/lib.rs`

```rust
pub fn search<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect
}
```

**清单 13-22**：在 `search` 函数的实现中使用迭代器适配器方法

回顾一下，`search` 函数的目的是，返回 `contents` 中包含 `query` 的所有行。与 [清单 13-16](./iterators.md#listing_13-16) 中的 `filter` 示例类似，这段代码使用 `filter` 适配器来仅保留 `line.contains(query)` 返回 `true` 的行。然后我们以 `collect` 收集匹配的行到另一个矢量值中。这样简单多了！咱们也可以随意在 `search_case_insensitive` 函数中，进行同样的修改来使用迭代器的方法。

> **译注**：函数 `search_case_insenstitive` 修改后如下所示：
>
> ```rust
> pub fn search_insensitive<'a>(
>     query: &str,
>     contents: &'a str
> ) -> Vec<&'a str> {
>     let query = query.to_lowercase();
>
>     contents
>         .lines()
>         .filter(|line| line.to_lowercase().contains(&query))
>         .collect()
> }
> ```


## 在循环或迭代器之间做出选择

接下来的合理问题是，在咱们自己的代码中应该选择哪种风格，以及为什么：[清单 13-21](#listing_13-21) 中的最初实现，还是 [清单 13-22](#listing_13-22) 中使用迭代器的版本（假设我们在返回结果之前先收集所有结果，而不是返回迭代器）。大多数 Rust 程序员更倾向于使用迭代器风格。一开始掌握窍门有点困难，而一旦咱们熟悉了各种迭代器适配器及其作用，迭代器就会更容易理解。代码不再纠缠于各种循环细节和建立新的矢量值，而是专注于循环的高级目标。这种风格抽象掉部分常规代码，从而更容易看到这些代码特有的概念，比如迭代器中每个元素都必须通过的过滤条件。

但这两种实现方式真的等价吗？只觉可能是较低级别的循环会更快。咱们来谈谈性能问题。


（End）


