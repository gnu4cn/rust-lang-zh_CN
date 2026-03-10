# 控制流


根据某个条件是否为 `true` 运行某些代码，及在某个条件为 `true` 时重复运行某些代码的能力，是大多数编程语言中的基本构件。允许咱们控制 Rust 代码执行流程的最常见结构，为 `if` 表达式和循环。


## `if` 表达式

`if` 表达式允许咱们根据条件分支咱们的代码。咱们提供某个条件，然后声明：“当这一条件满足，则运行这个代码块。当该条件不满足，则不运行这个代码块"。

请在咱们的 `projects` 目录下创建一个名为 `branches` 的新项目来探讨 `if` 表达式。在 `src/main.rs` 文件中，输入以下内容：


文件名：`src/main.rs`

```rust
fn main() {
    let number = 3;

    if number < 5 {
        println! ("条件为真");
    } else {
        println! ("条件为假");
    }
}
```


所有 `if` 表达式都以关键字 `if` 开头，后跟某个条件。在本例中，条件会检查 `number` 这个是否有着小于 `5` 的值。我们把在该条件为 `true` 时要执行的代码块，在紧接着该条件之后，放在一对花括号内。与 `if` 表达式中条件相关的代码块，有时被称为 *支臂，arms*，就像我们在第 2 章的 [“比较猜数与秘密数”](../Ch02_Programming_a_Guessing_Game.md#比较猜数与秘密数) 小节中，曾讨论过的 `match` 表达式中的支臂一样。

可选地，我们还可以包含一个 `else` 表达式，这里我们就选择了这样做，以便在条件计算为 `false` 时为程序提供一个要执行的替代代码块。当我们不提供 `else` 表达式而条件为 `false` 时，程序将跳过这个 `if` 代码块并继续到下一段代码。

请尝试运行这段代码；咱们应看到以下输出：

```console
$ cargo run
   Compiling branches v0.1.0 (/home/hector/rust-lang-zh_CN/projects/branches)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/branches`
条件为真
```

我们来尝试将 `number` 的值修改为使该条件为 `false` 的值，看看会发生什么：

```rust
    let number = 7;
```


再次运行这个程序，并查看输出：


```console
$ cargo run
   Compiling branches v0.1.0 (/home/hector/rust-lang-zh_CN/projects/branches)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running `target/debug/branches`
条件为假
```

还值得注意的是，这段代码中的条件 *必须* 要个 `bool` 值。当条件不是个 `bool` 值时，我们将得到一个报错。例如，请尝试运行以下代码：


文件名：`src/main.rs`

```rust
fn main() {
    let number = 3;

    if number {
        println! ("数字是 3");
    }
}
```


这次 `if` 条件会计算为一个 `3` 的值，Rust 就会抛出一个错误：


```console
$ cargo run
   Compiling branches v0.1.0 (/home/hector/rust-lang-zh_CN/projects/branches)
error[E0308]: mismatched types
 --> src/main.rs:4:8
  |
4 |     if number {
  |        ^^^^^^ expected `bool`, found integer

For more information about this error, try `rustc --explain E0308`.
error: could not compile `branches` (bin "branches") due to 1 previous error
```


这个错误指出，Rust 期望的是一个 `bool` 值却得到了个整数。与诸如 Ruby 及 JavaScript 等语言不同，Rust 将不会自动尝试转换非布尔类型为布尔类型。咱们必须显式并始终提供布尔值给 `if` 作为其条件。例如，若我们希望 `if` 的代码块仅在某个数字不等于 `0` 时运行，那么我们可将这个 `if` 表达式改为以下这个：


文件名：`src/main.rs`

```rust
fn main() {
    let number = 3;

    if number != 0 {
        println! ("数字为非零数");
    }
}
```

运行此代码将打印 `数字为非零数`。


### 以 `else if` 处理多重条件


咱们可通过在 `else if` 表达式中组合 `if` 和 `else` 使用多重条件。例如：


文件名：`src/main.rs`

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println! ("数字可被 4 整除");
    } else if number % 3 == 0 {
        println! ("数字可被 3 整除");
    } else if number % 2 == 0 {
        println! ("数字可被 2 整除");
    } else {
        println! ("数字不可被 4、3 或 2 整除");
    }
}
```

这个程序有四条其可能采取的可能路径。运行该程序后，咱们应看到以下输出：


```console
$ cargo run
   Compiling branches v0.1.0 (/home/hector/rust-lang-zh_CN/projects/branches)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running `target/debug/branches`
数字可被 3 整除
```


在这个程序执行时，他会依次检查每个 `if` 表达式，并执行第一个求值为 `true` 的条件的主体。请注意，尽管 `6` 能被 `2` 整除，但我们没有看到输出 `数字可被 2 整除`，也没有看到 `else` 代码块中的 `数字不可被 4、3 或 2 整除` 文本。这是因为 Rust 只会执行第一个 `true` 条件的代码块，且他一旦找到一个，就不会检查其余条件。

使用过多 `else if` 表达式会使咱们的代码变得杂乱无章，因此当咱们有着多于一个的 `else if` 表达式时，咱们可能需要重构咱们的代码。第 6 章介绍了一种针对这些情况的，[名为 `match` 的强大 Rust 分支结构](../enums_and_pattern_matching/match_control_flow.md)。


### 在 `let` 语句中使用 `if`

因为 `if` 属于表达式，所以我们可以在 `let` 语句的右侧使用他，将结果赋值给某个变量，如下清单 3-2 中那样。


<a name="listing_3-2"></a>
文件名：`src/main.rs`


```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println! ("number 的值为：{number}");
}
```

**清单 3-2**：将 `if` 表达式的结果赋值给变量


其中的 `number` 变量将根据 `if` 表达式的结果绑定到某个值。请运行这段代码，看看会发生什么：


```console
$ cargo run
   Compiling branches v0.1.0 (/home/hector/rust-lang-zh_CN/projects/branches)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running `target/debug/branches`
number 的值为：5
```


请记住，代码块会求值为其中的最后一个表达式，而数字本身也属于表达式。在这种情况下，整个的 `if` 表达式的值就取决于哪个代码块会执行。这意味着有可能成为 `if` 表达式各个支臂结果的值，必须属于同一类型；在清单 3-2 中，`if` 支臂和 `else` 支臂的结果都是 `i32` 的整数。当类型不匹配，如以下示例中那样，我们将得到一个报错：


文件名：`src/main.rs`

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { "six" };

    println! ("number 的值为：{number}");
}
```


当我们尝试编译这段代码时，我们将得到一个报错。`if` 和 `else` 支臂有着不兼容的值类型，Rust 准确地指出了在这个程序中何处发现这个问题：


```console
$ cargo run
   Compiling branches v0.1.0 (/home/hector/rust-lang-zh_CN/projects/branches)
error[E0308]: `if` and `else` have incompatible types
 --> src/main.rs:3:44
  |
3 |     let number = if condition { 5 } else { "six" };
  |                                 -          ^^^^^ expected integer, found `&str`
  |                                 |
  |                                 expected because of this

For more information about this error, try `rustc --explain E0308`.
error: could not compile `branches` (bin "branches") due to 1 previous error
```


其中 `if` 代码块中的表达式会求值为一个整数，而 `else` 代码块中的表达式会求值为一个字符串。这行不通，因为变量都必须有着单一类型，而 Rust 需要在编译时明确知道 `number` 这个变量为何种类型。了解 `number` 的类型，会让编译器在咱们使用 `number` 的任何地方，都检查该类型是否有效。当 `number` 的类型只有在运行时才确定出来时，那么 Rust 将不能够做到这点；当编译器必须跟踪任何变量的多种假设类型时，那么编译器就会变得更加复杂，并对代码的保证也会减少。


## 循环下的重复

多次执行某个代码块通常很有用。出于这一任务，Rust 提供了数种 *循环，loops*，他们运行循环体内的代码直到结束，然后立即从头开始。为实验循环，咱们来构造一个名为 `loops` 的新项目。

Rust 有着三种循环类别：`loop`、`while` 及 `for`。咱们来一一尝试。


### 以 `loop` 关键字重复代码

`loop` 关键字会告诉 Rust 一遍又一遍地执行某个代码块，要么永远执行，要么直到咱们明确告诉他停止为止。

作为一个示例，修改咱们 `loops` 目录中的 `src/main.rs` 文件为下面这样：


文件名：`src/main.rs`

```rust
fn main() {
    loop {
        println! ("again!");
    }
}
```


当我们运行这个程序时，我们将看到 `again!` 被不断打印，直到我们手动停止程序为止。大多数终端都支持 `crtl-C` 中断某个陷入连续循环程序。尝试一下：


```console
$ cargo run
   Compiling branches v0.1.0 (/home/hector/rust-lang-zh_CN/projects/loops)
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/loops`
again!
again!
again!
again!
again!
^C
```

其中符号 `^C` 代表咱们按下 `ctrl-C` 的位置。

咱们可能会也可能不会在 `^C` 之后看到 `again!` 打印出来，取决于代码收到中断信号时其在循环中的位置。

> **译注**：在 Windows 平台上运行 MSYS2 的环境下，按下 `ctrl-c` 的输出如下所示：
>
> ```console
> again!
> again!
> again!
> again!
> again!
> again!
> error: process didn't exit successfully: `target\debug\loops.exe` (exit code: 0xc000013a, STATUS_CONTROL_C_EXIT)
>
> ```


幸运的是，Rust 也提供了使用代码跳出循环的方法。咱们可放置 `break` 关键字在循环中，告诉程序何时停止执行循环。回想一下，我们在第 2 章 的 [猜对后退出](../Ch02_Programming_a_Guessing_Game.md#猜对后的退出) 小节的猜数游戏中的猜数游戏中就这样做了，当用户猜对数字赢得游戏时退出程序。

在猜数游戏中我们还使用了 `continue`，其在循环中会告诉程序，跳过循环的本次迭代中任何剩余代码，而前往下一次迭代。


### 从循环返回值

`loop` 的用途之一，是重试咱们知道可能失败的某项操作，例如检查某个线程是否已完成其作业。咱们可能还需要从循环传递出该操作的结果，到咱们代码的其余部分。要完成这一目的，咱们可将咱们打算返回的值，添加在咱们用于停止循环的 `break` 表达式后；该值将从循环中返回出来，以便咱们使用他，如下所示：


```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println! ("结果为：{result}");
}
```

在其中的循环前，我们声明一个名为 `counter` 的变量并初始化其为 `0`。 然后，咱们声明一个名为 `result` 的变量保存从该循环返回的值。在该循环的每一次迭代时，我们都给 `counter` 变量加 `1`，然后检查 `counter` 是否等于 `10`。当等于 `10` 时，我们使用带有 `counter * 2` 值的 `break` 关键字。在这个循环之后，我们使用分号结束指派该值到 `result` 的语句。最后，我们打印 `result` 中的值，在本例中为 `20`。

我们还可在循环内 `return`。`break` 只会退出当前循环，而 `return` 则始终会推出当前函数。


> **译注**：这个示例还可写作如下。
>
>
> ```rust
> fn main() {
>     let mut counter = 0;
>
>     let result = loop {
>         counter += 1;
>
>         if counter == 10 {
>             break
>             counter * 2
>         }
>     };
>
>     println! ("结果为：{result}");
> }
> ```
>
> 这似乎与 `break` 后的循环体代码不再执行相矛盾。也可理解为 `counter * 2` 表达式为 `break` 表达式的参数？


### 以循环标签消除歧义


当咱们有着循环内的循环（译注：嵌套循环）时，`break` 和 `continue` 就会应用于该处的最内层循环。咱们可选择在某个循环上指定一个 *循环标签，loop lable*，随后咱们可与 `break` 或 `continue` 一起使用，指定这两个关键字会应用到某个带标签的循环，而不是最内层循环。循环标签必须以单引号开头。下面是两个嵌套循环下的一个示例：


```rust
fn main() {
    let mut count = 0;

    'counting_up: loop {
        println! ("计数 = {}", count);
        let mut remaining = 10;

        loop {
            println! ("剩余 = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println! ("最终计数 = {}", count);
}
```


其中外层循环有着标签 `'counting_up`，他将从 `0` 计数到 `2`。不带标签的内存循环会从 `10` 倒计数到 `9`。未指定标签的第一个 `break`，将只退出内层循环。那个 `break 'counting_up;` 语句将退出外层循环。这段代码会打印：


```console
$ cargo run
   Compiling loops v0.1.0 (/home/hector/rust-lang-zh_CN/projects/loops)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/loops`
计数 = 0
剩余 = 10
剩余 = 9
计数 = 1
剩余 = 10
剩余 = 9
计数 = 2
剩余 = 10
最终计数 = 2
```


### `while` 下的简化条件循环

程序将经常需要评估循环内的条件。当条件为 `true` 时，循环运行。当条件不再为 `true` 时，程序调用 `break`，停止循环。使用 `loop`、`if`、`else` 与 `break` 的组合实现像是这样的行为是可行的；若咱们愿意，咱们现在就可以在程序中尝试这种做法。然而，这种模式如此常见，以至于 Rust 为其提供了一种内置的语言结构，称为 `while` 循环。在下面清单 3-3 中，我们使用 `while` 循环该程序三次，每次都倒数，然后在循环之后打印一条消息并退出。


<a name="listing_3-3"></a>
文件名：`src/main.rs`

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println! ("{}!", number);

        number -= 1;
    }

    println! ("点火！！");
}
```

**清单 3-3**：使用 `while` 循环在条件求值为 `true` 时运行代码


这种结构消除了咱们在使用 `loop`、`if`、`else`、及 `break` 时所需的大量嵌套，且更为清晰。在条件求值为 `true` 时，代码运行；否则，他会退出循环。


###  使用 `for` 遍历集合

咱们可选择使用 `while` 结构遍历集合中的元素，比如数组。例如，下面清单 3-4 中的循环会打印数组 `a` 中的各个元素。


<a name="listing_3-4"></a>
文件名：`src/main.rs`

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < a.len() {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
```

**清单 3-4**：使用 `while` 循环遍历集合中的各个元素


此处，代码会对数组中的元素向上计数。他会于索引 `0` 处开始然后循环，直到他到达数组中的最终索引（即，当 `index < 5` 不再为 `true` 时）。运行这段代码将打印该数组中的每个元素：


```console
$ cargo run
   Compiling loops v0.1.0 (/home/hector/rust-lang-zh_CN/projects/loops)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/loops`
the value is: 10
the value is: 20
the value is: 30
the value is: 40
the value is: 50
```


正如预期那样，所有五个数组值都出现在终端中。即使 `index` 将在某一时刻达到 `5`，这个循环也会尝试获取该数组中第六个值前停止执行。

但是，这种方法很容易出错；当索引值或测试条件不正确时，我们就会导致程序终止运行。例如，若咱们将 `a` 这个数组的定义改为有着四个元素，却忘记将条件更新为 `while index < 4` 时，则这段代码将终止运行。他还很慢，因为编译器会添加一些运行时代码，在循环的每次迭代时，执行索引是否在数组边界内的条件检查。

作为更简洁的替代方法，咱们可使用 `for` 循环，并对集合中的每个项目执行一些代码。`for` 循环会看起来如下清单 3-5 中的代码。


<a name="listing_3-5"></a>
文件名：`src/main.rs`

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for el in a {
        println! ("the value is: {el}");
    }
}
```

**清单 3-5**：使用 `for` 循环遍历集合中的各个元素


当咱们运行这段代码时，我们将看到与清单 3-4 同样的输出。更重要的是，我们现在提高了代码的安全性，并消除了超出数组末尾，或因遍历不足远而遗漏某些项目等可能导致的 bug 的可能性。自 `for` 循环生成的机器码也会更加高效，因为索引不需要在每次迭代时都与数组长度比较。

使用 `for` 循环时，当咱们修改了数组中值的个数时，咱们将不需要向使用 [清单 3-4](#listing_3-4) 中用到的方法中那样，记住修改任何其他代码。

`for` 循环的安全性和简洁性，使其成为 Rust 中最常用到的循环结构。即使是在咱们打算运行某些代码一定次数的情形下，如清单 3-3 中使用 `while` 循环的倒计时示例中，大多数 Rustaceans 也会使用 `for` 循环。实现这点的方法，是使用由标准库提供的 `Range`，他会按顺序生成从一个数字开始到另一个数字前结束的所有数字。

下面是使用 `for` 循环和我们尚未讨论过的另一个反转范围的方法 `rev` 的倒计数，看起来的样子：


文件名：`src/main.rs`

```rust
fn main() {
    for number in (1..4).rev() {
        println! ("{}!", number);
    }

    println! ("LIFTOFF!!");
}
```

这段代码好一点，不是吗？


# 本章小结

咱们做到了！这是蔚为壮观的一章：咱们学习了变量、标量与复合数据类型、函数、注释、`if` 表达式和循环！要练习这一章中讨论的概念，请尝试构建程序完成以下操作：

- 转换华氏和摄氏之间的温度；
- 生成第 n 个斐波拉基数；
- 利用圣诞颂歌 “The Twelve Days of Christmas” 中的重复，而打印出其歌词；


当咱们准备好继续前进时，我们将讨论 Rust 中，一个在其他编程语言中 *不* 常见的概念：所有权。


# 练习答案


<details>
    <summary>“法式温度与摄氏温度的转换”</summary>

```rust
use std::io;
use std::process;

fn fah_to_cels(f: f32) -> f32 {
    return (f - 32.0) / 1.8;
}

fn cels_to_fah(c: f32) -> f32 {
    return c * 1.8 + 32.0;
}

fn main() {
    println! ("法式温度与摄氏温度之间的转换");

    loop {
        println! ("\n-----------------\n请选择：
            '1'-摄氏温度/'2'-法式温度/'Q'/\"quit\" 退出程序。
            '1'/'2'/'Q'/\"quit\"[1]：");

        let mut temp_type = String::new();

        io::stdin()
            .read_line(&mut temp_type)
            .expect("读取输入失败！");

        let temp_type = temp_type.trim();

        if temp_type.eq("Q") || temp_type.eq("quit") { process::exit(0); }

        if ! temp_type.eq("1") && ! temp_type.eq("2") && ! temp_type.eq("") {
            println! ("无效输入，请输入 '1'、'2'、'Q'、\"quit\"，或直接按下回车键");
            continue;
        }

        if temp_type.eq("1") || temp_type.eq("") {
            println! ("请输入要转换的摄氏温度：");
            let temp = get_temp_input();

            println! ("摄氏温度： {:.2}°C，约为法氏温度：{:.2}°F", temp, cels_to_fah(temp));
        }

        if temp_type.eq("2") {
            println! ("请输入要转换的法氏温度：");
            let temp = get_temp_input();

            println! ("法氏温度：{:.2}°F，约为摄氏温度：{:.2}°C", temp, fah_to_cels(temp));
        }
    }
}

fn get_temp_input() -> f32 {
    return loop {
        let mut tmp = String::new();

        io::stdin()
            .read_line(&mut tmp)
            .expect("读取输入失败");

        match tmp.trim().parse() {
            Ok(num) => { break num },
            Err(_) => {
                println! ("请输入一个浮点数，比如 -10.0, 15.6");
                continue
            }
        };
    };
}
```

</details>


<details>
    <summary>"生成第 n 个斐波拉基数"</summary>


```rust
use std::io;
use num_format::{Locale, ToFormattedString};
// use std::process;

fn nth_fibonacci(n: u64) -> u64 {

    if n == 0 || n == 1 {
        return n;
    } else {
        return nth_fibonacci(n - 1) + nth_fibonacci(n - 2);
    }
}

fn main() {
    println! ("找出第 n 个斐波拉基数");

    'main_loop: loop {
        println! ("请输入 n: （Q/quit 退出程序）");

        let n: u64 = loop {
            let mut tmp = String::new();

            io::stdin()
                .read_line(&mut tmp)
                .expect("读取输入失败！");

            let tmp = tmp.trim();

            if tmp.eq("Q") || tmp.eq("quit") {
                // process::exit(0);
                break 'main_loop;
            }

            match tmp.parse() {
                Ok(num) => { break num },
                Err(_) => {
                    println! ("请输入一个正整数！\n");
                    continue;
                },
            };
        };

        println! ("第 {} 个斐波拉基数为：{}",
            n,
            nth_fibonacci(n).to_formatted_string(&Locale::en));
    }
}
```

</details>


<details>
    <summary>"打印圣诞颂歌 ‘The Twelve Days of Christmas’ 歌词"</summary>

```rust
fn main() {
    let days = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "nineth",
        "tenth",
        "eleventh",
        "twelfth"
    ];
    let amounts = [
        "A",
        "Two",
        "Three",
        "Four",
        "Five",
        "Six",
        "Seven",
        "Eight",
        "Nine",
        "Ten",
        "Eleven",
        "Twelve"
    ];
    let things = [
        "partridge in a pear tree",
        "turtle doves",
        "French hens",
        "calling birds",
        "golden rings",
        "geese-a-laying",
        "swans-a-swimming",
        "maids-a-milking",
        "ladies dancing",
        "lords-a-leaping",
        "pipers piping",
        "drummers drumming",
    ];

    for num in 1..=12 {
        println! ("\nOn the {} day of Christmas,\nMy true love gave to me:",
            days[num-1]);
        for tmp in (0..num).rev() {
            if tmp == 0 && num == 1 {
                println! ("{} {}.", amounts[tmp], things[tmp]);
            }
            if tmp == 0 && num != 1 {
                println! ("And {} {}.", amounts[tmp].to_lowercase(), things[tmp]);
            }
            if tmp != 0 {
                println! ("{} {},", amounts[tmp], things[tmp]);
            }
        }
    }
}
```

</details>


（End）


